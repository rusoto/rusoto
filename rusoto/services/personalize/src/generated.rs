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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl PersonalizeClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "personalize", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

/// <p>Describes a custom algorithm.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Algorithm {
    /// <p>The Amazon Resource Name (ARN) of the algorithm.</p>
    #[serde(rename = "algorithmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    /// <p>The URI of the Docker container for the algorithm image.</p>
    #[serde(rename = "algorithmImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_image: Option<AlgorithmImage>,
    /// <p>The date and time (in Unix time) that the algorithm was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>Specifies the default hyperparameters, their ranges, and whether they are tunable. A tunable hyperparameter can have its value determined during hyperparameter optimization (HPO).</p>
    #[serde(rename = "defaultHyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_hyper_parameter_ranges: Option<DefaultHyperParameterRanges>,
    /// <p>Specifies the default hyperparameters.</p>
    #[serde(rename = "defaultHyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the default maximum number of training jobs and parallel training jobs.</p>
    #[serde(rename = "defaultResourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_config: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date and time (in Unix time) that the algorithm was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the algorithm.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The training input mode.</p>
    #[serde(rename = "trainingInputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_input_mode: Option<String>,
}

/// <p>Describes an algorithm image.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AlgorithmImage {
    /// <p>The URI of the Docker container for the algorithm image.</p>
    #[serde(rename = "dockerURI")]
    pub docker_uri: String,
    /// <p>The name of the algorithm image.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>When the solution performs AutoML (<code>performAutoML</code> is true in <a>CreateSolution</a>), Amazon Personalize determines which recipe, from the specified list, optimizes the given metric. Amazon Personalize then uses that recipe for the solution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AutoMLConfig {
    /// <p>The metric to optimize.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The list of candidate recipes.</p>
    #[serde(rename = "recipeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_list: Option<Vec<String>>,
}

/// <p>When the solution performs AutoML (<code>performAutoML</code> is true in <a>CreateSolution</a>), specifies the recipe that best optimized the specified metric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutoMLResult {
    /// <p>The Amazon Resource Name (ARN) of the best recipe.</p>
    #[serde(rename = "bestRecipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_recipe_arn: Option<String>,
}

/// <p>Contains information on a batch inference job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchInferenceJob {
    /// <p>The Amazon Resource Name (ARN) of the batch inference job.</p>
    #[serde(rename = "batchInferenceJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_arn: Option<String>,
    /// <p>A string to string map of the configuration details of a batch inference job.</p>
    #[serde(rename = "batchInferenceJobConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_config: Option<BatchInferenceJobConfig>,
    /// <p>The time at which the batch inference job was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>If the batch inference job failed, the reason for the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The ARN of the filter used on the batch inference job.</p>
    #[serde(rename = "filterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    /// <p>The Amazon S3 path that leads to the input data used to generate the batch inference job.</p>
    #[serde(rename = "jobInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_input: Option<BatchInferenceJobInput>,
    /// <p>The name of the batch inference job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The Amazon S3 bucket that contains the output data generated by the batch inference job.</p>
    #[serde(rename = "jobOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_output: Option<BatchInferenceJobOutput>,
    /// <p>The time at which the batch inference job was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The number of recommendations generated by the batch inference job. This number includes the error messages generated for failed input records.</p>
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// <p>The ARN of the Amazon Identity and Access Management (IAM) role that requested the batch inference job.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the solution version from which the batch inference job was created.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    /// <p><p>The status of the batch inference job. The status is one of the following values:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>IN PROGRESS</p> </li> <li> <p>ACTIVE</p> </li> <li> <p>CREATE FAILED</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The configuration details of a batch inference job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BatchInferenceJobConfig {
    /// <p>A string to string map specifying the inference hyperparameters you wish to use for hyperparameter optimization. See <a>customizing-solution-config-hpo</a>.</p>
    #[serde(rename = "itemExplorationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_exploration_config: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The input configuration of a batch inference job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BatchInferenceJobInput {
    /// <p>The URI of the Amazon S3 location that contains your input data. The Amazon S3 bucket must be in the same region as the API endpoint you are calling.</p>
    #[serde(rename = "s3DataSource")]
    pub s_3_data_source: S3DataConfig,
}

/// <p>The output configuration parameters of a batch inference job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BatchInferenceJobOutput {
    /// <p>Information on the Amazon S3 bucket in which the batch inference job's output is stored.</p>
    #[serde(rename = "s3DataDestination")]
    pub s_3_data_destination: S3DataConfig,
}

/// <p>A truncated version of the <a>BatchInferenceJob</a> datatype. The <a>ListBatchInferenceJobs</a> operation returns a list of batch inference job summaries.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchInferenceJobSummary {
    /// <p>The Amazon Resource Name (ARN) of the batch inference job.</p>
    #[serde(rename = "batchInferenceJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_arn: Option<String>,
    /// <p>The time at which the batch inference job was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>If the batch inference job failed, the reason for the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The name of the batch inference job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The time at which the batch inference job was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The ARN of the solution version used by the batch inference job.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    /// <p><p>The status of the batch inference job. The status is one of the following values:</p> <ul> <li> <p>PENDING</p> </li> <li> <p>IN PROGRESS</p> </li> <li> <p>ACTIVE</p> </li> <li> <p>CREATE FAILED</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes a deployed solution version, otherwise known as a campaign. For more information on campaigns, see <a>CreateCampaign</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Campaign {
    /// <p>The Amazon Resource Name (ARN) of the campaign. </p>
    #[serde(rename = "campaignArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
    /// <p>The configuration details of a campaign.</p>
    #[serde(rename = "campaignConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_config: Option<CampaignConfig>,
    /// <p>The date and time (in Unix format) that the campaign was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>If a campaign fails, the reason behind the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The date and time (in Unix format) that the campaign was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "latestCampaignUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_campaign_update: Option<CampaignUpdateSummary>,
    /// <p>Specifies the requested minimum provisioned transactions (recommendations) per second.</p>
    #[serde(rename = "minProvisionedTPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_provisioned_tps: Option<i64>,
    /// <p>The name of the campaign.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of a specific version of the solution.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    /// <p><p>The status of the campaign.</p> <p>A campaign can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The configuration details of a campaign.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CampaignConfig {
    /// <p>A string to string map specifying the inference hyperparameters you wish to use for hyperparameter optimization. See <a>customizing-solution-config-hpo</a>.</p>
    #[serde(rename = "itemExplorationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_exploration_config: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides a summary of the properties of a campaign. For a complete listing, call the <a>DescribeCampaign</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CampaignSummary {
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    #[serde(rename = "campaignArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
    /// <p>The date and time (in Unix time) that the campaign was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>If a campaign fails, the reason behind the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The date and time (in Unix time) that the campaign was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the campaign.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the campaign.</p> <p>A campaign can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides a summary of the properties of a campaign update. For a complete listing, call the <a>DescribeCampaign</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CampaignUpdateSummary {
    #[serde(rename = "campaignConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_config: Option<CampaignConfig>,
    /// <p>The date and time (in Unix time) that the campaign update was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>If a campaign update fails, the reason behind the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The date and time (in Unix time) that the campaign update was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>Specifies the requested minimum provisioned transactions (recommendations) per second that Amazon Personalize will support.</p>
    #[serde(rename = "minProvisionedTPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_provisioned_tps: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the deployed solution version.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    /// <p><p>The status of the campaign update.</p> <p>A campaign update can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides the name and range of a categorical hyperparameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CategoricalHyperParameterRange {
    /// <p>The name of the hyperparameter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of the categories for the hyperparameter.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Provides the name and range of a continuous hyperparameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContinuousHyperParameterRange {
    /// <p>The maximum allowable value for the hyperparameter.</p>
    #[serde(rename = "maxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// <p>The minimum allowable value for the hyperparameter.</p>
    #[serde(rename = "minValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// <p>The name of the hyperparameter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBatchInferenceJobRequest {
    /// <p>The configuration details of a batch inference job.</p>
    #[serde(rename = "batchInferenceJobConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_config: Option<BatchInferenceJobConfig>,
    /// <p>The ARN of the filter to apply to the batch inference job. For more information on using filters, see Using Filters with Amazon Personalize.</p>
    #[serde(rename = "filterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    /// <p>The Amazon S3 path that leads to the input file to base your recommendations on. The input material must be in JSON format.</p>
    #[serde(rename = "jobInput")]
    pub job_input: BatchInferenceJobInput,
    /// <p>The name of the batch inference job to create.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The path to the Amazon S3 bucket where the job's output will be stored.</p>
    #[serde(rename = "jobOutput")]
    pub job_output: BatchInferenceJobOutput,
    /// <p>The number of recommendations to retreive.</p>
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// <p>The ARN of the Amazon Identity and Access Management role that has permissions to read and write to your input and out Amazon S3 buckets respectively.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the solution version that will be used to generate the batch inference recommendations.</p>
    #[serde(rename = "solutionVersionArn")]
    pub solution_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBatchInferenceJobResponse {
    /// <p>The ARN of the batch inference job.</p>
    #[serde(rename = "batchInferenceJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCampaignRequest {
    /// <p>The configuration details of a campaign.</p>
    #[serde(rename = "campaignConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_config: Option<CampaignConfig>,
    /// <p>Specifies the requested minimum provisioned transactions (recommendations) per second that Amazon Personalize will support.</p>
    #[serde(rename = "minProvisionedTPS")]
    pub min_provisioned_tps: i64,
    /// <p>A name for the new campaign. The campaign name must be unique within your account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the solution version to deploy.</p>
    #[serde(rename = "solutionVersionArn")]
    pub solution_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCampaignResponse {
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    #[serde(rename = "campaignArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDatasetGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of a KMS key used to encrypt the datasets.</p>
    #[serde(rename = "kmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The name for the new dataset group.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the IAM role that has permissions to access the KMS key. Supplying an IAM role is only valid when also specifying a KMS key.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDatasetGroupResponse {
    /// <p>The Amazon Resource Name (ARN) of the new dataset group.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDatasetImportJobRequest {
    /// <p>The Amazon S3 bucket that contains the training data to import.</p>
    #[serde(rename = "dataSource")]
    pub data_source: DataSource,
    /// <p>The ARN of the dataset that receives the imported data.</p>
    #[serde(rename = "datasetArn")]
    pub dataset_arn: String,
    /// <p>The name for the dataset import job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The ARN of the IAM role that has permissions to read from the Amazon S3 data source.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDatasetImportJobResponse {
    /// <p>The ARN of the dataset import job.</p>
    #[serde(rename = "datasetImportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDatasetRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset group to add the dataset to.</p>
    #[serde(rename = "datasetGroupArn")]
    pub dataset_group_arn: String,
    /// <p><p>The type of dataset.</p> <p>One of the following (case insensitive) values:</p> <ul> <li> <p>Interactions</p> </li> <li> <p>Items</p> </li> <li> <p>Users</p> </li> </ul></p>
    #[serde(rename = "datasetType")]
    pub dataset_type: String,
    /// <p>The name for the dataset.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the schema to associate with the dataset. The schema defines the dataset fields.</p>
    #[serde(rename = "schemaArn")]
    pub schema_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDatasetResponse {
    /// <p>The ARN of the dataset.</p>
    #[serde(rename = "datasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEventTrackerRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset group that receives the event data.</p>
    #[serde(rename = "datasetGroupArn")]
    pub dataset_group_arn: String,
    /// <p>The name for the event tracker.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEventTrackerResponse {
    /// <p>The ARN of the event tracker.</p>
    #[serde(rename = "eventTrackerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tracker_arn: Option<String>,
    /// <p>The ID of the event tracker. Include this ID in requests to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_UBS_PutEvents.html">PutEvents</a> API.</p>
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFilterRequest {
    /// <p>The ARN of the dataset group that the filter will belong to.</p>
    #[serde(rename = "datasetGroupArn")]
    pub dataset_group_arn: String,
    /// <p>The filter expression that designates the interaction types that the filter will filter out. A filter expression must follow the following format:</p> <p> <code>EXCLUDE itemId WHERE INTERACTIONS.event_type in ("EVENT_TYPE")</code> </p> <p>Where "EVENT_TYPE" is the type of event to filter out. To filter out all items with any interactions history, set <code>"*"</code> as the EVENT_TYPE. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filters.html">Using Filters with Amazon Personalize</a>.</p>
    #[serde(rename = "filterExpression")]
    pub filter_expression: String,
    /// <p>The name of the filter to create.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFilterResponse {
    /// <p>The ARN of the new filter.</p>
    #[serde(rename = "filterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSchemaRequest {
    /// <p>The name for the schema.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A schema in Avro JSON format.</p>
    #[serde(rename = "schema")]
    pub schema: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSchemaResponse {
    /// <p>The Amazon Resource Name (ARN) of the created schema.</p>
    #[serde(rename = "schemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSolutionRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset group that provides the training data.</p>
    #[serde(rename = "datasetGroupArn")]
    pub dataset_group_arn: String,
    /// <p>When your have multiple event types (using an <code>EVENT_TYPE</code> schema field), this parameter specifies which event type (for example, 'click' or 'like') is used for training the model.</p>
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// <p>The name for the solution.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Whether to perform automated machine learning (AutoML). The default is <code>false</code>. For this case, you must specify <code>recipeArn</code>.</p> <p>When set to <code>true</code>, Amazon Personalize analyzes your training data and selects the optimal USER_PERSONALIZATION recipe and hyperparameters. In this case, you must omit <code>recipeArn</code>. Amazon Personalize determines the optimal recipe by running tests with different values for the hyperparameters. AutoML lengthens the training process as compared to selecting a specific recipe.</p>
    #[serde(rename = "performAutoML")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_ml: Option<bool>,
    /// <p>Whether to perform hyperparameter optimization (HPO) on the specified or selected recipe. The default is <code>false</code>.</p> <p>When performing AutoML, this parameter is always <code>true</code> and you should not set it to <code>false</code>.</p>
    #[serde(rename = "performHPO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_hpo: Option<bool>,
    /// <p>The ARN of the recipe to use for model training. Only specified when <code>performAutoML</code> is false.</p>
    #[serde(rename = "recipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    /// <p>The configuration to use with the solution. When <code>performAutoML</code> is set to true, Amazon Personalize only evaluates the <code>autoMLConfig</code> section of the solution configuration.</p>
    #[serde(rename = "solutionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_config: Option<SolutionConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSolutionResponse {
    /// <p>The ARN of the solution.</p>
    #[serde(rename = "solutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSolutionVersionRequest {
    /// <p>The Amazon Resource Name (ARN) of the solution containing the training configuration information.</p>
    #[serde(rename = "solutionArn")]
    pub solution_arn: String,
    /// <p><p>The scope of training to be performed when creating the solution version. The <code>FULL</code> option trains the solution version based on the entirety of the input solution&#39;s training data, while the <code>UPDATE</code> option processes only the data that has changed in comparison to the input solution. Choose <code>UPDATE</code> when you want to incrementally update your solution version instead of creating an entirely new one.</p> <important> <p>The <code>UPDATE</code> option can only be used when you already have an active solution version created from the input solution using the <code>FULL</code> option and the input solution was trained with the <a>native-recipe-hrnn-coldstart</a> recipe.</p> </important></p>
    #[serde(rename = "trainingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_mode: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSolutionVersionResponse {
    /// <p>The ARN of the new solution version.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

/// <p>Describes the data source that contains the data to upload to a dataset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataSource {
    /// <p>The path to the Amazon S3 bucket where the data that you want to upload to your dataset is stored. For example: </p> <p> <code>s3://bucket-name/training-data.csv</code> </p>
    #[serde(rename = "dataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location: Option<String>,
}

/// <p>Provides metadata for a dataset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Dataset {
    /// <p>The creation date and time (in Unix time) of the dataset.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset that you want metadata for.</p>
    #[serde(rename = "datasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p><p>One of the following values:</p> <ul> <li> <p>Interactions</p> </li> <li> <p>Items</p> </li> <li> <p>Users</p> </li> </ul></p>
    #[serde(rename = "datasetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    /// <p>A time stamp that shows when the dataset was updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the dataset.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the associated schema.</p>
    #[serde(rename = "schemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    /// <p><p>The status of the dataset.</p> <p>A dataset can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>A dataset group is a collection of related datasets (Interactions, User, and Item). You create a dataset group by calling <a>CreateDatasetGroup</a>. You then create a dataset and add it to a dataset group by calling <a>CreateDataset</a>. The dataset group is used to create and train a solution by calling <a>CreateSolution</a>. A dataset group can contain only one of each type of dataset.</p> <p>You can specify an AWS Key Management Service (KMS) key to encrypt the datasets in the group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetGroup {
    /// <p>The creation date and time (in Unix time) of the dataset group.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>If creating a dataset group fails, provides the reason why.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the KMS key used to encrypt the datasets.</p>
    #[serde(rename = "kmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The last update date and time (in Unix time) of the dataset group.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the dataset group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the IAM role that has permissions to create the dataset group.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The current status of the dataset group.</p> <p>A dataset group can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides a summary of the properties of a dataset group. For a complete listing, call the <a>DescribeDatasetGroup</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetGroupSummary {
    /// <p>The date and time (in Unix time) that the dataset group was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>If creating a dataset group fails, the reason behind the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The date and time (in Unix time) that the dataset group was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the dataset group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the dataset group.</p> <p>A dataset group can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p><p>Describes a job that imports training data from a data source (Amazon S3 bucket) to an Amazon Personalize dataset. For more information, see <a>CreateDatasetImportJob</a>.</p> <p>A dataset import job can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetImportJob {
    /// <p>The creation date and time (in Unix time) of the dataset import job.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon S3 bucket that contains the training data to import.</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    /// <p>The Amazon Resource Name (ARN) of the dataset that receives the imported data.</p>
    #[serde(rename = "datasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p>The ARN of the dataset import job.</p>
    #[serde(rename = "datasetImportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    /// <p>If a dataset import job fails, provides the reason why.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The name of the import job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The date and time (in Unix time) the dataset was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that has permissions to read from the Amazon S3 data source.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The status of the dataset import job.</p> <p>A dataset import job can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides a summary of the properties of a dataset import job. For a complete listing, call the <a>DescribeDatasetImportJob</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetImportJobSummary {
    /// <p>The date and time (in Unix time) that the dataset import job was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset import job.</p>
    #[serde(rename = "datasetImportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    /// <p>If a dataset import job fails, the reason behind the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The name of the dataset import job.</p>
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The date and time (in Unix time) that the dataset was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p><p>The status of the dataset import job.</p> <p>A dataset import job can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes the schema for a dataset. For more information on schemas, see <a>CreateSchema</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetSchema {
    /// <p>The date and time (in Unix time) that the schema was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The date and time (in Unix time) that the schema was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the schema.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schema.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the schema.</p>
    #[serde(rename = "schemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

/// <p>Provides a summary of the properties of a dataset schema. For a complete listing, call the <a>DescribeSchema</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetSchemaSummary {
    /// <p>The date and time (in Unix time) that the schema was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The date and time (in Unix time) that the schema was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the schema.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the schema.</p>
    #[serde(rename = "schemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

/// <p>Provides a summary of the properties of a dataset. For a complete listing, call the <a>DescribeDataset</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetSummary {
    /// <p>The date and time (in Unix time) that the dataset was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "datasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p><p>The dataset type. One of the following values:</p> <ul> <li> <p>Interactions</p> </li> <li> <p>Items</p> </li> <li> <p>Users</p> </li> <li> <p>Event-Interactions</p> </li> </ul></p>
    #[serde(rename = "datasetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    /// <p>The date and time (in Unix time) that the dataset was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the dataset.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the dataset.</p> <p>A dataset can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides the name and default range of a categorical hyperparameter and whether the hyperparameter is tunable. A tunable hyperparameter can have its value determined during hyperparameter optimization (HPO).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DefaultCategoricalHyperParameterRange {
    /// <p>Whether the hyperparameter is tunable.</p>
    #[serde(rename = "isTunable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tunable: Option<bool>,
    /// <p>The name of the hyperparameter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of the categories for the hyperparameter.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>Provides the name and default range of a continuous hyperparameter and whether the hyperparameter is tunable. A tunable hyperparameter can have its value determined during hyperparameter optimization (HPO).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DefaultContinuousHyperParameterRange {
    /// <p>Whether the hyperparameter is tunable.</p>
    #[serde(rename = "isTunable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tunable: Option<bool>,
    /// <p>The maximum allowable value for the hyperparameter.</p>
    #[serde(rename = "maxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// <p>The minimum allowable value for the hyperparameter.</p>
    #[serde(rename = "minValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// <p>The name of the hyperparameter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Specifies the hyperparameters and their default ranges. Hyperparameters can be categorical, continuous, or integer-valued.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DefaultHyperParameterRanges {
    /// <p>The categorical hyperparameters and their default ranges.</p>
    #[serde(rename = "categoricalHyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_hyper_parameter_ranges: Option<Vec<DefaultCategoricalHyperParameterRange>>,
    /// <p>The continuous hyperparameters and their default ranges.</p>
    #[serde(rename = "continuousHyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_hyper_parameter_ranges: Option<Vec<DefaultContinuousHyperParameterRange>>,
    /// <p>The integer-valued hyperparameters and their default ranges.</p>
    #[serde(rename = "integerHyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_hyper_parameter_ranges: Option<Vec<DefaultIntegerHyperParameterRange>>,
}

/// <p>Provides the name and default range of a integer-valued hyperparameter and whether the hyperparameter is tunable. A tunable hyperparameter can have its value determined during hyperparameter optimization (HPO).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DefaultIntegerHyperParameterRange {
    /// <p>Indicates whether the hyperparameter is tunable.</p>
    #[serde(rename = "isTunable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tunable: Option<bool>,
    /// <p>The maximum allowable value for the hyperparameter.</p>
    #[serde(rename = "maxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,
    /// <p>The minimum allowable value for the hyperparameter.</p>
    #[serde(rename = "minValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i64>,
    /// <p>The name of the hyperparameter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCampaignRequest {
    /// <p>The Amazon Resource Name (ARN) of the campaign to delete.</p>
    #[serde(rename = "campaignArn")]
    pub campaign_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDatasetGroupRequest {
    /// <p>The ARN of the dataset group to delete.</p>
    #[serde(rename = "datasetGroupArn")]
    pub dataset_group_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDatasetRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset to delete.</p>
    #[serde(rename = "datasetArn")]
    pub dataset_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventTrackerRequest {
    /// <p>The Amazon Resource Name (ARN) of the event tracker to delete.</p>
    #[serde(rename = "eventTrackerArn")]
    pub event_tracker_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFilterRequest {
    /// <p>The ARN of the filter to delete.</p>
    #[serde(rename = "filterArn")]
    pub filter_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSchemaRequest {
    /// <p>The Amazon Resource Name (ARN) of the schema to delete.</p>
    #[serde(rename = "schemaArn")]
    pub schema_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSolutionRequest {
    /// <p>The ARN of the solution to delete.</p>
    #[serde(rename = "solutionArn")]
    pub solution_arn: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAlgorithmRequest {
    /// <p>The Amazon Resource Name (ARN) of the algorithm to describe.</p>
    #[serde(rename = "algorithmArn")]
    pub algorithm_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAlgorithmResponse {
    /// <p>A listing of the properties of the algorithm.</p>
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<Algorithm>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBatchInferenceJobRequest {
    /// <p>The ARN of the batch inference job to describe.</p>
    #[serde(rename = "batchInferenceJobArn")]
    pub batch_inference_job_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBatchInferenceJobResponse {
    /// <p>Information on the specified batch inference job.</p>
    #[serde(rename = "batchInferenceJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job: Option<BatchInferenceJob>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCampaignRequest {
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    #[serde(rename = "campaignArn")]
    pub campaign_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCampaignResponse {
    /// <p>The properties of the campaign.</p>
    #[serde(rename = "campaign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaign>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDatasetGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset group to describe.</p>
    #[serde(rename = "datasetGroupArn")]
    pub dataset_group_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDatasetGroupResponse {
    /// <p>A listing of the dataset group's properties.</p>
    #[serde(rename = "datasetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group: Option<DatasetGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDatasetImportJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset import job to describe.</p>
    #[serde(rename = "datasetImportJobArn")]
    pub dataset_import_job_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDatasetImportJobResponse {
    /// <p><p>Information about the dataset import job, including the status.</p> <p>The status is one of the following values:</p> <ul> <li> <p>CREATE PENDING</p> </li> <li> <p>CREATE IN_PROGRESS</p> </li> <li> <p>ACTIVE</p> </li> <li> <p>CREATE FAILED</p> </li> </ul></p>
    #[serde(rename = "datasetImportJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job: Option<DatasetImportJob>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDatasetRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset to describe.</p>
    #[serde(rename = "datasetArn")]
    pub dataset_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDatasetResponse {
    /// <p>A listing of the dataset's properties.</p>
    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventTrackerRequest {
    /// <p>The Amazon Resource Name (ARN) of the event tracker to describe.</p>
    #[serde(rename = "eventTrackerArn")]
    pub event_tracker_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventTrackerResponse {
    /// <p>An object that describes the event tracker.</p>
    #[serde(rename = "eventTracker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tracker: Option<EventTracker>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFeatureTransformationRequest {
    /// <p>The Amazon Resource Name (ARN) of the feature transformation to describe.</p>
    #[serde(rename = "featureTransformationArn")]
    pub feature_transformation_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFeatureTransformationResponse {
    /// <p>A listing of the FeatureTransformation properties.</p>
    #[serde(rename = "featureTransformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_transformation: Option<FeatureTransformation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFilterRequest {
    /// <p>The ARN of the filter to describe.</p>
    #[serde(rename = "filterArn")]
    pub filter_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFilterResponse {
    /// <p>The filter's details.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRecipeRequest {
    /// <p>The Amazon Resource Name (ARN) of the recipe to describe.</p>
    #[serde(rename = "recipeArn")]
    pub recipe_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRecipeResponse {
    /// <p>An object that describes the recipe.</p>
    #[serde(rename = "recipe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<Recipe>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSchemaRequest {
    /// <p>The Amazon Resource Name (ARN) of the schema to retrieve.</p>
    #[serde(rename = "schemaArn")]
    pub schema_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSchemaResponse {
    /// <p>The requested schema.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<DatasetSchema>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSolutionRequest {
    /// <p>The Amazon Resource Name (ARN) of the solution to describe.</p>
    #[serde(rename = "solutionArn")]
    pub solution_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSolutionResponse {
    /// <p>An object that describes the solution.</p>
    #[serde(rename = "solution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<Solution>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSolutionVersionRequest {
    /// <p>The Amazon Resource Name (ARN) of the solution version.</p>
    #[serde(rename = "solutionVersionArn")]
    pub solution_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSolutionVersionResponse {
    /// <p>The solution version.</p>
    #[serde(rename = "solutionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version: Option<SolutionVersion>,
}

/// <p>Provides information about an event tracker.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventTracker {
    /// <p>The Amazon AWS account that owns the event tracker.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The date and time (in Unix format) that the event tracker was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group that receives the event data.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The ARN of the event tracker.</p>
    #[serde(rename = "eventTrackerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tracker_arn: Option<String>,
    /// <p>The date and time (in Unix time) that the event tracker was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the event tracker.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the event tracker.</p> <p>An event tracker can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The ID of the event tracker. Include this ID in requests to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_UBS_PutEvents.html">PutEvents</a> API.</p>
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}

/// <p>Provides a summary of the properties of an event tracker. For a complete listing, call the <a>DescribeEventTracker</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventTrackerSummary {
    /// <p>The date and time (in Unix time) that the event tracker was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the event tracker.</p>
    #[serde(rename = "eventTrackerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tracker_arn: Option<String>,
    /// <p>The date and time (in Unix time) that the event tracker was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the event tracker.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the event tracker.</p> <p>An event tracker can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides feature transformation information. Feature transformation is the process of modifying raw input data into a form more suitable for model training.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FeatureTransformation {
    /// <p>The creation date and time (in Unix time) of the feature transformation.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>Provides the default parameters for feature transformation.</p>
    #[serde(rename = "defaultParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Amazon Resource Name (ARN) of the FeatureTransformation object.</p>
    #[serde(rename = "featureTransformationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_transformation_arn: Option<String>,
    /// <p>The last update date and time (in Unix time) of the feature transformation.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the feature transformation.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The status of the feature transformation.</p> <p>A feature transformation can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains information on a recommendation filter, including its ARN, status, and filter expression.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Filter {
    /// <p>The time at which the filter was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The ARN of the dataset group to which the filter belongs.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>If the filter failed, the reason for its failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The ARN of the filter.</p>
    #[serde(rename = "filterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    /// <p>Specifies the type of item interactions to filter out of recommendation results. The filter expression must follow the following format:</p> <p> <code>EXCLUDE itemId WHERE INTERACTIONS.event_type in ("EVENT_TYPE")</code> </p> <p>Where "EVENT_TYPE" is the type of event to filter out. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filters.html">Using Filters with Amazon Personalize</a>.</p>
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The time at which the filter was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the filter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the filter.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>A short summary of a filter's attributes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FilterSummary {
    /// <p>The time at which the filter was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The ARN of the dataset group to which the filter belongs.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>If the filter failed, the reason for the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The ARN of the filter.</p>
    #[serde(rename = "filterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    /// <p>The time at which the filter was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the filter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the filter.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSolutionMetricsRequest {
    /// <p>The Amazon Resource Name (ARN) of the solution version for which to get metrics.</p>
    #[serde(rename = "solutionVersionArn")]
    pub solution_version_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSolutionMetricsResponse {
    /// <p>The metrics for the solution version.</p>
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>The same solution version ARN as specified in the request.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

/// <p>Describes the properties for hyperparameter optimization (HPO). For use with the bring-your-own-recipe feature. Do not use for Amazon Personalize native recipes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HPOConfig {
    /// <p>The hyperparameters and their allowable ranges.</p>
    #[serde(rename = "algorithmHyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_hyper_parameter_ranges: Option<HyperParameterRanges>,
    /// <p>The metric to optimize during HPO.</p>
    #[serde(rename = "hpoObjective")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpo_objective: Option<HPOObjective>,
    /// <p>Describes the resource configuration for HPO.</p>
    #[serde(rename = "hpoResourceConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpo_resource_config: Option<HPOResourceConfig>,
}

/// <p>The metric to optimize during hyperparameter optimization (HPO).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HPOObjective {
    /// <p>The name of the metric.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>A regular expression for finding the metric in the training job logs.</p>
    #[serde(rename = "metricRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_regex: Option<String>,
    /// <p>The type of the metric. Valid values are <code>Maximize</code> and <code>Minimize</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes the resource configuration for hyperparameter optimization (HPO).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HPOResourceConfig {
    /// <p>The maximum number of training jobs when you create a solution version. The maximum value for <code>maxNumberOfTrainingJobs</code> is <code>40</code>.</p>
    #[serde(rename = "maxNumberOfTrainingJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_training_jobs: Option<String>,
    /// <p>The maximum number of parallel training jobs when you create a solution version. The maximum value for <code>maxParallelTrainingJobs</code> is <code>10</code>.</p>
    #[serde(rename = "maxParallelTrainingJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel_training_jobs: Option<String>,
}

/// <p>Specifies the hyperparameters and their ranges. Hyperparameters can be categorical, continuous, or integer-valued.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HyperParameterRanges {
    /// <p>The categorical hyperparameters and their ranges.</p>
    #[serde(rename = "categoricalHyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_hyper_parameter_ranges: Option<Vec<CategoricalHyperParameterRange>>,
    /// <p>The continuous hyperparameters and their ranges.</p>
    #[serde(rename = "continuousHyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_hyper_parameter_ranges: Option<Vec<ContinuousHyperParameterRange>>,
    /// <p>The integer-valued hyperparameters and their ranges.</p>
    #[serde(rename = "integerHyperParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_hyper_parameter_ranges: Option<Vec<IntegerHyperParameterRange>>,
}

/// <p>Provides the name and range of an integer-valued hyperparameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IntegerHyperParameterRange {
    /// <p>The maximum allowable value for the hyperparameter.</p>
    #[serde(rename = "maxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,
    /// <p>The minimum allowable value for the hyperparameter.</p>
    #[serde(rename = "minValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i64>,
    /// <p>The name of the hyperparameter.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBatchInferenceJobsRequest {
    /// <p>The maximum number of batch inference job results to return in each page. The default value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the solution version from which the batch inference jobs were created.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBatchInferenceJobsResponse {
    /// <p>A list containing information on each job that is returned.</p>
    #[serde(rename = "batchInferenceJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_jobs: Option<Vec<BatchInferenceJobSummary>>,
    /// <p>The token to use to retreive the next page of results. The value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCampaignsRequest {
    /// <p>The maximum number of campaigns to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListCampaigns</code> for getting the next set of campaigns (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the solution to list the campaigns for. When a solution is not specified, all the campaigns associated with the account are listed.</p>
    #[serde(rename = "solutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCampaignsResponse {
    /// <p>A list of the campaigns.</p>
    #[serde(rename = "campaigns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<Vec<CampaignSummary>>,
    /// <p>A token for getting the next set of campaigns (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDatasetGroupsRequest {
    /// <p>The maximum number of dataset groups to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListDatasetGroups</code> for getting the next set of dataset groups (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDatasetGroupsResponse {
    /// <p>The list of your dataset groups.</p>
    #[serde(rename = "datasetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_groups: Option<Vec<DatasetGroupSummary>>,
    /// <p>A token for getting the next set of dataset groups (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDatasetImportJobsRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset to list the dataset import jobs for.</p>
    #[serde(rename = "datasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p>The maximum number of dataset import jobs to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListDatasetImportJobs</code> for getting the next set of dataset import jobs (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDatasetImportJobsResponse {
    /// <p>The list of dataset import jobs.</p>
    #[serde(rename = "datasetImportJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_jobs: Option<Vec<DatasetImportJobSummary>>,
    /// <p>A token for getting the next set of dataset import jobs (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDatasetsRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset group that contains the datasets to list.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The maximum number of datasets to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListDatasetImportJobs</code> for getting the next set of dataset import jobs (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDatasetsResponse {
    /// <p>An array of <code>Dataset</code> objects. Each object provides metadata information.</p>
    #[serde(rename = "datasets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<DatasetSummary>>,
    /// <p>A token for getting the next set of datasets (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEventTrackersRequest {
    /// <p>The ARN of a dataset group used to filter the response.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The maximum number of event trackers to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListEventTrackers</code> for getting the next set of event trackers (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEventTrackersResponse {
    /// <p>A list of event trackers.</p>
    #[serde(rename = "eventTrackers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_trackers: Option<Vec<EventTrackerSummary>>,
    /// <p>A token for getting the next set of event trackers (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFiltersRequest {
    /// <p>The ARN of the dataset group that contains the filters.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The maximum number of filters to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListFilters</code> for getting the next set of filters (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFiltersResponse {
    /// <p>A list of returned filters.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<FilterSummary>>,
    /// <p>A token for getting the next set of filters (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRecipesRequest {
    /// <p>The maximum number of recipes to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListRecipes</code> for getting the next set of recipes (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The default is <code>SERVICE</code>.</p>
    #[serde(rename = "recipeProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRecipesResponse {
    /// <p>A token for getting the next set of recipes.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of available recipes.</p>
    #[serde(rename = "recipes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipes: Option<Vec<RecipeSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSchemasRequest {
    /// <p>The maximum number of schemas to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListSchemas</code> for getting the next set of schemas (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSchemasResponse {
    /// <p>A token used to get the next set of schemas (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of schemas.</p>
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<DatasetSchemaSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSolutionVersionsRequest {
    /// <p>The maximum number of solution versions to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListSolutionVersions</code> for getting the next set of solution versions (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the solution.</p>
    #[serde(rename = "solutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSolutionVersionsResponse {
    /// <p>A token for getting the next set of solution versions (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of solution versions describing the version properties.</p>
    #[serde(rename = "solutionVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_versions: Option<Vec<SolutionVersionSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSolutionsRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The maximum number of solutions to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token returned from the previous call to <code>ListSolutions</code> for getting the next set of solutions (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSolutionsResponse {
    /// <p>A token for getting the next set of solutions (if they exist).</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the current solutions.</p>
    #[serde(rename = "solutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solutions: Option<Vec<SolutionSummary>>,
}

/// <p>Provides information about a recipe. Each recipe provides an algorithm that Amazon Personalize uses in model training when you use the <a>CreateSolution</a> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Recipe {
    /// <p>The Amazon Resource Name (ARN) of the algorithm that Amazon Personalize uses to train the model.</p>
    #[serde(rename = "algorithmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    /// <p>The date and time (in Unix format) that the recipe was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The description of the recipe.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the FeatureTransformation object.</p>
    #[serde(rename = "featureTransformationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_transformation_arn: Option<String>,
    /// <p>The date and time (in Unix format) that the recipe was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the recipe.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the recipe.</p>
    #[serde(rename = "recipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    /// <p><p>One of the following values:</p> <ul> <li> <p>PERSONALIZED<em>RANKING</p> </li> <li> <p>RELATED</em>ITEMS</p> </li> <li> <p>USER_PERSONALIZATION</p> </li> </ul></p>
    #[serde(rename = "recipeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_type: Option<String>,
    /// <p>The status of the recipe.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides a summary of the properties of a recipe. For a complete listing, call the <a>DescribeRecipe</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecipeSummary {
    /// <p>The date and time (in Unix time) that the recipe was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The date and time (in Unix time) that the recipe was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the recipe.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the recipe.</p>
    #[serde(rename = "recipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    /// <p>The status of the recipe.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The configuration details of an Amazon S3 input or output bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3DataConfig {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Key Management Service (KMS) key that Amazon Personalize uses to encrypt or decrypt the input and output files of a batch inference job.</p>
    #[serde(rename = "kmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The file path of the Amazon S3 bucket.</p>
    #[serde(rename = "path")]
    pub path: String,
}

/// <p>An object that provides information about a solution. A solution is a trained model that can be deployed as a campaign.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Solution {
    /// <p>When <code>performAutoML</code> is true, specifies the best recipe found.</p>
    #[serde(rename = "autoMLResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_result: Option<AutoMLResult>,
    /// <p>The creation date and time (in Unix time) of the solution.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group that provides the training data.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The event type (for example, 'click' or 'like') that is used for training the model.</p>
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// <p>The date and time (in Unix time) that the solution was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>Describes the latest version of the solution, including the status and the ARN.</p>
    #[serde(rename = "latestSolutionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_solution_version: Option<SolutionVersionSummary>,
    /// <p>The name of the solution.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>When true, Amazon Personalize performs a search for the best USER_PERSONALIZATION recipe from the list specified in the solution configuration (<code>recipeArn</code> must not be specified). When false (the default), Amazon Personalize uses <code>recipeArn</code> for training.</p>
    #[serde(rename = "performAutoML")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_ml: Option<bool>,
    /// <p>Whether to perform hyperparameter optimization (HPO) on the chosen recipe. The default is <code>false</code>.</p>
    #[serde(rename = "performHPO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_hpo: Option<bool>,
    /// <p>The ARN of the recipe used to create the solution.</p>
    #[serde(rename = "recipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    /// <p>The ARN of the solution.</p>
    #[serde(rename = "solutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
    /// <p>Describes the configuration properties for the solution.</p>
    #[serde(rename = "solutionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_config: Option<SolutionConfig>,
    /// <p><p>The status of the solution.</p> <p>A solution can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes the configuration properties for the solution.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SolutionConfig {
    /// <p>Lists the hyperparameter names and ranges.</p>
    #[serde(rename = "algorithmHyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_hyper_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The <a>AutoMLConfig</a> object containing a list of recipes to search when AutoML is performed.</p>
    #[serde(rename = "autoMLConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_config: Option<AutoMLConfig>,
    /// <p>Only events with a value greater than or equal to this threshold are used for training a model.</p>
    #[serde(rename = "eventValueThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_value_threshold: Option<String>,
    /// <p>Lists the feature transformation parameters.</p>
    #[serde(rename = "featureTransformationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_transformation_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Describes the properties for hyperparameter optimization (HPO).</p>
    #[serde(rename = "hpoConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpo_config: Option<HPOConfig>,
}

/// <p>Provides a summary of the properties of a solution. For a complete listing, call the <a>DescribeSolution</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SolutionSummary {
    /// <p>The date and time (in Unix time) that the solution was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The date and time (in Unix time) that the solution was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The name of the solution.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the solution.</p>
    #[serde(rename = "solutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
    /// <p><p>The status of the solution.</p> <p>A solution can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>An object that provides information about a specific version of a <a>Solution</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SolutionVersion {
    /// <p>The date and time (in Unix time) that this version of the solution was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group providing the training data.</p>
    #[serde(rename = "datasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The event type (for example, 'click' or 'like') that is used for training the model.</p>
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// <p>If training a solution version fails, the reason for the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The date and time (in Unix time) that the solution was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>When true, Amazon Personalize searches for the most optimal recipe according to the solution configuration. When false (the default), Amazon Personalize uses <code>recipeArn</code>.</p>
    #[serde(rename = "performAutoML")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_ml: Option<bool>,
    /// <p>Whether to perform hyperparameter optimization (HPO) on the chosen recipe. The default is <code>false</code>.</p>
    #[serde(rename = "performHPO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_hpo: Option<bool>,
    /// <p>The ARN of the recipe used in the solution.</p>
    #[serde(rename = "recipeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    /// <p>The ARN of the solution.</p>
    #[serde(rename = "solutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
    /// <p>Describes the configuration properties for the solution.</p>
    #[serde(rename = "solutionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_config: Option<SolutionConfig>,
    /// <p>The ARN of the solution version.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    /// <p><p>The status of the solution version.</p> <p>A solution version can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING</p> </li> <li> <p>CREATE IN_PROGRESS</p> </li> <li> <p>ACTIVE</p> </li> <li> <p>CREATE FAILED</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time used to train the model. You are billed for the time it takes to train a model. This field is visible only after Amazon Personalize successfully trains a model.</p>
    #[serde(rename = "trainingHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_hours: Option<f64>,
    /// <p><p>The scope of training used to create the solution version. The <code>FULL</code> option trains the solution version based on the entirety of the input solution&#39;s training data, while the <code>UPDATE</code> option processes only the training data that has changed since the creation of the last solution version. Choose <code>UPDATE</code> when you want to start recommending items added to the dataset without retraining the model.</p> <important> <p>The <code>UPDATE</code> option can only be used after you&#39;ve created a solution version with the <code>FULL</code> option and the training solution uses the <a>native-recipe-hrnn-coldstart</a>.</p> </important></p>
    #[serde(rename = "trainingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_mode: Option<String>,
    /// <p>If hyperparameter optimization was performed, contains the hyperparameter values of the best performing model.</p>
    #[serde(rename = "tunedHPOParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuned_hpo_params: Option<TunedHPOParams>,
}

/// <p>Provides a summary of the properties of a solution version. For a complete listing, call the <a>DescribeSolutionVersion</a> API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SolutionVersionSummary {
    /// <p>The date and time (in Unix time) that this version of a solution was created.</p>
    #[serde(rename = "creationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>If a solution version fails, the reason behind the failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The date and time (in Unix time) that the solution version was last updated.</p>
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the solution version.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    /// <p><p>The status of the solution version.</p> <p>A solution version can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>If hyperparameter optimization (HPO) was performed, contains the hyperparameter values of the best performing model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TunedHPOParams {
    /// <p>A list of the hyperparameter values of the best performing model.</p>
    #[serde(rename = "algorithmHyperParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_hyper_parameters: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCampaignRequest {
    /// <p>The Amazon Resource Name (ARN) of the campaign.</p>
    #[serde(rename = "campaignArn")]
    pub campaign_arn: String,
    /// <p>The configuration details of a campaign.</p>
    #[serde(rename = "campaignConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_config: Option<CampaignConfig>,
    /// <p>Specifies the requested minimum provisioned transactions (recommendations) per second that Amazon Personalize will support.</p>
    #[serde(rename = "minProvisionedTPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_provisioned_tps: Option<i64>,
    /// <p>The ARN of a new solution version to deploy.</p>
    #[serde(rename = "solutionVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCampaignResponse {
    /// <p>The same campaign ARN as given in the request.</p>
    #[serde(rename = "campaignArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
}

/// Errors returned by CreateBatchInferenceJob
#[derive(Debug, PartialEq)]
pub enum CreateBatchInferenceJobError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl CreateBatchInferenceJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBatchInferenceJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateBatchInferenceJobError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateBatchInferenceJobError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateBatchInferenceJobError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateBatchInferenceJobError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateBatchInferenceJobError::ResourceNotFound(
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
impl fmt::Display for CreateBatchInferenceJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBatchInferenceJobError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateBatchInferenceJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateBatchInferenceJobError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateBatchInferenceJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateBatchInferenceJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBatchInferenceJobError {}
/// Errors returned by CreateCampaign
#[derive(Debug, PartialEq)]
pub enum CreateCampaignError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl CreateCampaignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCampaignError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateCampaignError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateCampaignError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateCampaignError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateCampaignError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateCampaignError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCampaignError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateCampaignError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCampaignError {}
/// Errors returned by CreateDataset
#[derive(Debug, PartialEq)]
pub enum CreateDatasetError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl CreateDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatasetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDatasetError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDatasetError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateDatasetError::ResourceAlreadyExists(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateDatasetError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDatasetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDatasetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDatasetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDatasetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDatasetError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateDatasetError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateDatasetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDatasetError {}
/// Errors returned by CreateDatasetGroup
#[derive(Debug, PartialEq)]
pub enum CreateDatasetGroupError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreateDatasetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatasetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDatasetGroupError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDatasetGroupError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateDatasetGroupError::ResourceAlreadyExists(
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
impl fmt::Display for CreateDatasetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDatasetGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDatasetGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDatasetGroupError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDatasetGroupError {}
/// Errors returned by CreateDatasetImportJob
#[derive(Debug, PartialEq)]
pub enum CreateDatasetImportJobError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl CreateDatasetImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatasetImportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDatasetImportJobError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDatasetImportJobError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateDatasetImportJobError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateDatasetImportJobError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDatasetImportJobError::ResourceNotFound(
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
impl fmt::Display for CreateDatasetImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDatasetImportJobError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateDatasetImportJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDatasetImportJobError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateDatasetImportJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateDatasetImportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDatasetImportJobError {}
/// Errors returned by CreateEventTracker
#[derive(Debug, PartialEq)]
pub enum CreateEventTrackerError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl CreateEventTrackerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEventTrackerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateEventTrackerError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEventTrackerError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateEventTrackerError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateEventTrackerError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateEventTrackerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEventTrackerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEventTrackerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateEventTrackerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEventTrackerError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateEventTrackerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateEventTrackerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEventTrackerError {}
/// Errors returned by CreateFilter
#[derive(Debug, PartialEq)]
pub enum CreateFilterError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl CreateFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateFilterError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateFilterError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateFilterError::ResourceAlreadyExists(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateFilterError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFilterError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFilterError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateFilterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFilterError {}
/// Errors returned by CreateSchema
#[derive(Debug, PartialEq)]
pub enum CreateSchemaError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
}

impl CreateSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSchemaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateSchemaError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSchemaError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateSchemaError::ResourceAlreadyExists(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSchemaError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSchemaError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSchemaError {}
/// Errors returned by CreateSolution
#[derive(Debug, PartialEq)]
pub enum CreateSolutionError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl CreateSolutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSolutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateSolutionError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSolutionError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateSolutionError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateSolutionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateSolutionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateSolutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSolutionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateSolutionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSolutionError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateSolutionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateSolutionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSolutionError {}
/// Errors returned by CreateSolutionVersion
#[derive(Debug, PartialEq)]
pub enum CreateSolutionVersionError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl CreateSolutionVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSolutionVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateSolutionVersionError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateSolutionVersionError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateSolutionVersionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateSolutionVersionError::ResourceNotFound(
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
impl fmt::Display for CreateSolutionVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSolutionVersionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateSolutionVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateSolutionVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateSolutionVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSolutionVersionError {}
/// Errors returned by DeleteCampaign
#[derive(Debug, PartialEq)]
pub enum DeleteCampaignError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteCampaignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCampaignError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteCampaignError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteCampaignError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteCampaignError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCampaignError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteCampaignError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteCampaignError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCampaignError {}
/// Errors returned by DeleteDataset
#[derive(Debug, PartialEq)]
pub enum DeleteDatasetError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatasetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDatasetError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteDatasetError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDatasetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDatasetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDatasetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteDatasetError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteDatasetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDatasetError {}
/// Errors returned by DeleteDatasetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteDatasetGroupError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteDatasetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatasetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDatasetGroupError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteDatasetGroupError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDatasetGroupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDatasetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDatasetGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteDatasetGroupError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteDatasetGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDatasetGroupError {}
/// Errors returned by DeleteEventTracker
#[derive(Debug, PartialEq)]
pub enum DeleteEventTrackerError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteEventTrackerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventTrackerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteEventTrackerError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteEventTrackerError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteEventTrackerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEventTrackerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventTrackerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteEventTrackerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteEventTrackerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventTrackerError {}
/// Errors returned by DeleteFilter
#[derive(Debug, PartialEq)]
pub enum DeleteFilterError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteFilterError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteFilterError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFilterError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFilterError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteFilterError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteFilterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFilterError {}
/// Errors returned by DeleteSchema
#[derive(Debug, PartialEq)]
pub enum DeleteSchemaError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSchemaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteSchemaError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteSchemaError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSchemaError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSchemaError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteSchemaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSchemaError {}
/// Errors returned by DeleteSolution
#[derive(Debug, PartialEq)]
pub enum DeleteSolutionError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DeleteSolutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSolutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteSolutionError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteSolutionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteSolutionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSolutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSolutionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteSolutionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteSolutionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSolutionError {}
/// Errors returned by DescribeAlgorithm
#[derive(Debug, PartialEq)]
pub enum DescribeAlgorithmError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeAlgorithmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAlgorithmError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeAlgorithmError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeAlgorithmError::ResourceNotFound(err.msg))
                }
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
        match *self {
            DescribeAlgorithmError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeAlgorithmError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAlgorithmError {}
/// Errors returned by DescribeBatchInferenceJob
#[derive(Debug, PartialEq)]
pub enum DescribeBatchInferenceJobError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeBatchInferenceJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBatchInferenceJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeBatchInferenceJobError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeBatchInferenceJobError::ResourceNotFound(
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
impl fmt::Display for DescribeBatchInferenceJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBatchInferenceJobError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeBatchInferenceJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBatchInferenceJobError {}
/// Errors returned by DescribeCampaign
#[derive(Debug, PartialEq)]
pub enum DescribeCampaignError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeCampaignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCampaignError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeCampaignError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCampaignError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCampaignError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeCampaignError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCampaignError {}
/// Errors returned by DescribeDataset
#[derive(Debug, PartialEq)]
pub enum DescribeDatasetError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDatasetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeDatasetError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDatasetError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDatasetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDatasetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeDatasetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDatasetError {}
/// Errors returned by DescribeDatasetGroup
#[derive(Debug, PartialEq)]
pub enum DescribeDatasetGroupError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeDatasetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDatasetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeDatasetGroupError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDatasetGroupError::ResourceNotFound(
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
impl fmt::Display for DescribeDatasetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDatasetGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeDatasetGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDatasetGroupError {}
/// Errors returned by DescribeDatasetImportJob
#[derive(Debug, PartialEq)]
pub enum DescribeDatasetImportJobError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeDatasetImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDatasetImportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeDatasetImportJobError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDatasetImportJobError::ResourceNotFound(
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
impl fmt::Display for DescribeDatasetImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDatasetImportJobError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeDatasetImportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDatasetImportJobError {}
/// Errors returned by DescribeEventTracker
#[derive(Debug, PartialEq)]
pub enum DescribeEventTrackerError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeEventTrackerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventTrackerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeEventTrackerError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEventTrackerError::ResourceNotFound(
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
impl fmt::Display for DescribeEventTrackerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventTrackerError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeEventTrackerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventTrackerError {}
/// Errors returned by DescribeFeatureTransformation
#[derive(Debug, PartialEq)]
pub enum DescribeFeatureTransformationError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeFeatureTransformationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeFeatureTransformationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeFeatureTransformationError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeFeatureTransformationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFeatureTransformationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFeatureTransformationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeFeatureTransformationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeFeatureTransformationError {}
/// Errors returned by DescribeFilter
#[derive(Debug, PartialEq)]
pub enum DescribeFilterError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeFilterError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeFilterError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFilterError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeFilterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFilterError {}
/// Errors returned by DescribeRecipe
#[derive(Debug, PartialEq)]
pub enum DescribeRecipeError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeRecipeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRecipeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeRecipeError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeRecipeError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRecipeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRecipeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeRecipeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRecipeError {}
/// Errors returned by DescribeSchema
#[derive(Debug, PartialEq)]
pub enum DescribeSchemaError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSchemaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeSchemaError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSchemaError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSchemaError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeSchemaError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSchemaError {}
/// Errors returned by DescribeSolution
#[derive(Debug, PartialEq)]
pub enum DescribeSolutionError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeSolutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSolutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeSolutionError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSolutionError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSolutionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSolutionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeSolutionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSolutionError {}
/// Errors returned by DescribeSolutionVersion
#[derive(Debug, PartialEq)]
pub enum DescribeSolutionVersionError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl DescribeSolutionVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSolutionVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeSolutionVersionError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeSolutionVersionError::ResourceNotFound(
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
impl fmt::Display for DescribeSolutionVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSolutionVersionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeSolutionVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSolutionVersionError {}
/// Errors returned by GetSolutionMetrics
#[derive(Debug, PartialEq)]
pub enum GetSolutionMetricsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl GetSolutionMetricsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSolutionMetricsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(GetSolutionMetricsError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(GetSolutionMetricsError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSolutionMetricsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSolutionMetricsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSolutionMetricsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetSolutionMetricsError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            GetSolutionMetricsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSolutionMetricsError {}
/// Errors returned by ListBatchInferenceJobs
#[derive(Debug, PartialEq)]
pub enum ListBatchInferenceJobsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListBatchInferenceJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBatchInferenceJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListBatchInferenceJobsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListBatchInferenceJobsError::InvalidNextToken(
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
impl fmt::Display for ListBatchInferenceJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBatchInferenceJobsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListBatchInferenceJobsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBatchInferenceJobsError {}
/// Errors returned by ListCampaigns
#[derive(Debug, PartialEq)]
pub enum ListCampaignsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListCampaignsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCampaignsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListCampaignsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListCampaignsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCampaignsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCampaignsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListCampaignsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCampaignsError {}
/// Errors returned by ListDatasetGroups
#[derive(Debug, PartialEq)]
pub enum ListDatasetGroupsError {
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListDatasetGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatasetGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDatasetGroupsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDatasetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDatasetGroupsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDatasetGroupsError {}
/// Errors returned by ListDatasetImportJobs
#[derive(Debug, PartialEq)]
pub enum ListDatasetImportJobsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListDatasetImportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatasetImportJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListDatasetImportJobsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDatasetImportJobsError::InvalidNextToken(
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
impl fmt::Display for ListDatasetImportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDatasetImportJobsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListDatasetImportJobsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDatasetImportJobsError {}
/// Errors returned by ListDatasets
#[derive(Debug, PartialEq)]
pub enum ListDatasetsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListDatasetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatasetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListDatasetsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDatasetsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDatasetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDatasetsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListDatasetsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDatasetsError {}
/// Errors returned by ListEventTrackers
#[derive(Debug, PartialEq)]
pub enum ListEventTrackersError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListEventTrackersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEventTrackersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListEventTrackersError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListEventTrackersError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEventTrackersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEventTrackersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListEventTrackersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEventTrackersError {}
/// Errors returned by ListFilters
#[derive(Debug, PartialEq)]
pub enum ListFiltersError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListFiltersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFiltersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListFiltersError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListFiltersError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFiltersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFiltersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListFiltersError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFiltersError {}
/// Errors returned by ListRecipes
#[derive(Debug, PartialEq)]
pub enum ListRecipesError {
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListRecipesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRecipesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListRecipesError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRecipesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRecipesError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRecipesError {}
/// Errors returned by ListSchemas
#[derive(Debug, PartialEq)]
pub enum ListSchemasError {
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListSchemasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSchemasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListSchemasError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSchemasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSchemasError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSchemasError {}
/// Errors returned by ListSolutionVersions
#[derive(Debug, PartialEq)]
pub enum ListSolutionVersionsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl ListSolutionVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSolutionVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListSolutionVersionsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListSolutionVersionsError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListSolutionVersionsError::ResourceNotFound(
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
impl fmt::Display for ListSolutionVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSolutionVersionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListSolutionVersionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            ListSolutionVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSolutionVersionsError {}
/// Errors returned by ListSolutions
#[derive(Debug, PartialEq)]
pub enum ListSolutionsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The token is not valid.</p>
    InvalidNextToken(String),
}

impl ListSolutionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSolutionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListSolutionsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListSolutionsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSolutionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSolutionsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListSolutionsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSolutionsError {}
/// Errors returned by UpdateCampaign
#[derive(Debug, PartialEq)]
pub enum UpdateCampaignError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl UpdateCampaignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCampaignError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateCampaignError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateCampaignError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateCampaignError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCampaignError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateCampaignError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateCampaignError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateCampaignError {}
/// Trait representing the capabilities of the Amazon Personalize API. Amazon Personalize clients implement this trait.
#[async_trait]
pub trait Personalize {
    /// <p>Creates a batch inference job. The operation can handle up to 50 million records and the input file must be in JSON format. For more information, see <a>recommendations-batch</a>.</p>
    async fn create_batch_inference_job(
        &self,
        input: CreateBatchInferenceJobRequest,
    ) -> Result<CreateBatchInferenceJobResponse, RusotoError<CreateBatchInferenceJobError>>;

    /// <p><p>Creates a campaign by deploying a solution version. When a client calls the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetRecommendations.html">GetRecommendations</a> and <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetPersonalizedRanking.html">GetPersonalizedRanking</a> APIs, a campaign is specified in the request.</p> <p> <b>Minimum Provisioned TPS and Auto-Scaling</b> </p> <p>A transaction is a single <code>GetRecommendations</code> or <code>GetPersonalizedRanking</code> call. Transactions per second (TPS) is the throughput and unit of billing for Amazon Personalize. The minimum provisioned TPS (<code>minProvisionedTPS</code>) specifies the baseline throughput provisioned by Amazon Personalize, and thus, the minimum billing charge. If your TPS increases beyond <code>minProvisionedTPS</code>, Amazon Personalize auto-scales the provisioned capacity up and down, but never below <code>minProvisionedTPS</code>, to maintain a 70% utilization. There&#39;s a short time delay while the capacity is increased that might cause loss of transactions. It&#39;s recommended to start with a low <code>minProvisionedTPS</code>, track your usage using Amazon CloudWatch metrics, and then increase the <code>minProvisionedTPS</code> as necessary.</p> <p> <b>Status</b> </p> <p>A campaign can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul> <p>To get the campaign status, call <a>DescribeCampaign</a>.</p> <note> <p>Wait until the <code>status</code> of the campaign is <code>ACTIVE</code> before asking the campaign for recommendations.</p> </note> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListCampaigns</a> </p> </li> <li> <p> <a>DescribeCampaign</a> </p> </li> <li> <p> <a>UpdateCampaign</a> </p> </li> <li> <p> <a>DeleteCampaign</a> </p> </li> </ul></p>
    async fn create_campaign(
        &self,
        input: CreateCampaignRequest,
    ) -> Result<CreateCampaignResponse, RusotoError<CreateCampaignError>>;

    /// <p><p>Creates an empty dataset and adds it to the specified dataset group. Use <a>CreateDatasetImportJob</a> to import your training data to a dataset.</p> <p>There are three types of datasets:</p> <ul> <li> <p>Interactions</p> </li> <li> <p>Items</p> </li> <li> <p>Users</p> </li> </ul> <p>Each dataset type has an associated schema with required field types. Only the <code>Interactions</code> dataset is required in order to train a model (also referred to as creating a solution).</p> <p>A dataset can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul> <p>To get the status of the dataset, call <a>DescribeDataset</a>.</p> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>CreateDatasetGroup</a> </p> </li> <li> <p> <a>ListDatasets</a> </p> </li> <li> <p> <a>DescribeDataset</a> </p> </li> <li> <p> <a>DeleteDataset</a> </p> </li> </ul></p>
    async fn create_dataset(
        &self,
        input: CreateDatasetRequest,
    ) -> Result<CreateDatasetResponse, RusotoError<CreateDatasetError>>;

    /// <p><p>Creates an empty dataset group. A dataset group contains related datasets that supply data for training a model. A dataset group can contain at most three datasets, one for each type of dataset:</p> <ul> <li> <p>Interactions</p> </li> <li> <p>Items</p> </li> <li> <p>Users</p> </li> </ul> <p>To train a model (create a solution), a dataset group that contains an <code>Interactions</code> dataset is required. Call <a>CreateDataset</a> to add a dataset to the group.</p> <p>A dataset group can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING</p> </li> </ul> <p>To get the status of the dataset group, call <a>DescribeDatasetGroup</a>. If the status shows as CREATE FAILED, the response includes a <code>failureReason</code> key, which describes why the creation failed.</p> <note> <p>You must wait until the <code>status</code> of the dataset group is <code>ACTIVE</code> before adding a dataset to the group.</p> </note> <p>You can specify an AWS Key Management Service (KMS) key to encrypt the datasets in the group. If you specify a KMS key, you must also include an AWS Identity and Access Management (IAM) role that has permission to access the key.</p> <p class="title"> <b>APIs that require a dataset group ARN in the request</b> </p> <ul> <li> <p> <a>CreateDataset</a> </p> </li> <li> <p> <a>CreateEventTracker</a> </p> </li> <li> <p> <a>CreateSolution</a> </p> </li> </ul> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListDatasetGroups</a> </p> </li> <li> <p> <a>DescribeDatasetGroup</a> </p> </li> <li> <p> <a>DeleteDatasetGroup</a> </p> </li> </ul></p>
    async fn create_dataset_group(
        &self,
        input: CreateDatasetGroupRequest,
    ) -> Result<CreateDatasetGroupResponse, RusotoError<CreateDatasetGroupError>>;

    /// <p><p>Creates a job that imports training data from your data source (an Amazon S3 bucket) to an Amazon Personalize dataset. To allow Amazon Personalize to import the training data, you must specify an AWS Identity and Access Management (IAM) role that has permission to read from the data source, as Amazon Personalize makes a copy of your data and processes it in an internal AWS system.</p> <important> <p>The dataset import job replaces any previous data in the dataset.</p> </important> <p> <b>Status</b> </p> <p>A dataset import job can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul> <p>To get the status of the import job, call <a>DescribeDatasetImportJob</a>, providing the Amazon Resource Name (ARN) of the dataset import job. The dataset import is complete when the status shows as ACTIVE. If the status shows as CREATE FAILED, the response includes a <code>failureReason</code> key, which describes why the job failed.</p> <note> <p>Importing takes time. You must wait until the status shows as ACTIVE before training a model using the dataset.</p> </note> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListDatasetImportJobs</a> </p> </li> <li> <p> <a>DescribeDatasetImportJob</a> </p> </li> </ul></p>
    async fn create_dataset_import_job(
        &self,
        input: CreateDatasetImportJobRequest,
    ) -> Result<CreateDatasetImportJobResponse, RusotoError<CreateDatasetImportJobError>>;

    /// <p><p>Creates an event tracker that you use when sending event data to the specified dataset group using the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_UBS_PutEvents.html">PutEvents</a> API.</p> <p>When Amazon Personalize creates an event tracker, it also creates an <i>event-interactions</i> dataset in the dataset group associated with the event tracker. The event-interactions dataset stores the event data from the <code>PutEvents</code> call. The contents of this dataset are not available to the user.</p> <note> <p>Only one event tracker can be associated with a dataset group. You will get an error if you call <code>CreateEventTracker</code> using the same dataset group as an existing event tracker.</p> </note> <p>When you send event data you include your tracking ID. The tracking ID identifies the customer and authorizes the customer to send the data.</p> <p>The event tracker can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul> <p>To get the status of the event tracker, call <a>DescribeEventTracker</a>.</p> <note> <p>The event tracker must be in the ACTIVE state before using the tracking ID.</p> </note> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListEventTrackers</a> </p> </li> <li> <p> <a>DescribeEventTracker</a> </p> </li> <li> <p> <a>DeleteEventTracker</a> </p> </li> </ul></p>
    async fn create_event_tracker(
        &self,
        input: CreateEventTrackerRequest,
    ) -> Result<CreateEventTrackerResponse, RusotoError<CreateEventTrackerError>>;

    /// <p>Creates a recommendation filter. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filters.html">Using Filters with Amazon Personalize</a>.</p>
    async fn create_filter(
        &self,
        input: CreateFilterRequest,
    ) -> Result<CreateFilterResponse, RusotoError<CreateFilterError>>;

    /// <p><p>Creates an Amazon Personalize schema from the specified schema string. The schema you create must be in Avro JSON format.</p> <p>Amazon Personalize recognizes three schema variants. Each schema is associated with a dataset type and has a set of required field and keywords. You specify a schema when you call <a>CreateDataset</a>.</p> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListSchemas</a> </p> </li> <li> <p> <a>DescribeSchema</a> </p> </li> <li> <p> <a>DeleteSchema</a> </p> </li> </ul></p>
    async fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> Result<CreateSchemaResponse, RusotoError<CreateSchemaError>>;

    /// <p><p>Creates the configuration for training a model. A trained model is known as a solution. After the configuration is created, you train the model (create a solution) by calling the <a>CreateSolutionVersion</a> operation. Every time you call <code>CreateSolutionVersion</code>, a new version of the solution is created.</p> <p>After creating a solution version, you check its accuracy by calling <a>GetSolutionMetrics</a>. When you are satisfied with the version, you deploy it using <a>CreateCampaign</a>. The campaign provides recommendations to a client through the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetRecommendations.html">GetRecommendations</a> API.</p> <p>To train a model, Amazon Personalize requires training data and a recipe. The training data comes from the dataset group that you provide in the request. A recipe specifies the training algorithm and a feature transformation. You can specify one of the predefined recipes provided by Amazon Personalize. Alternatively, you can specify <code>performAutoML</code> and Amazon Personalize will analyze your data and select the optimum USER<em>PERSONALIZATION recipe for you.</p> <p> <b>Status</b> </p> <p>A solution can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN</em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN_PROGRESS</p> </li> </ul> <p>To get the status of the solution, call <a>DescribeSolution</a>. Wait until the status shows as ACTIVE before calling <code>CreateSolutionVersion</code>.</p> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListSolutions</a> </p> </li> <li> <p> <a>CreateSolutionVersion</a> </p> </li> <li> <p> <a>DescribeSolution</a> </p> </li> <li> <p> <a>DeleteSolution</a> </p> </li> </ul> <ul> <li> <p> <a>ListSolutionVersions</a> </p> </li> <li> <p> <a>DescribeSolutionVersion</a> </p> </li> </ul></p>
    async fn create_solution(
        &self,
        input: CreateSolutionRequest,
    ) -> Result<CreateSolutionResponse, RusotoError<CreateSolutionError>>;

    /// <p><p>Trains or retrains an active solution. A solution is created using the <a>CreateSolution</a> operation and must be in the ACTIVE state before calling <code>CreateSolutionVersion</code>. A new version of the solution is created every time you call this operation.</p> <p> <b>Status</b> </p> <p>A solution version can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul> <p>To get the status of the version, call <a>DescribeSolutionVersion</a>. Wait until the status shows as ACTIVE before calling <code>CreateCampaign</code>.</p> <p>If the status shows as CREATE FAILED, the response includes a <code>failureReason</code> key, which describes why the job failed.</p> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListSolutionVersions</a> </p> </li> <li> <p> <a>DescribeSolutionVersion</a> </p> </li> </ul> <ul> <li> <p> <a>ListSolutions</a> </p> </li> <li> <p> <a>CreateSolution</a> </p> </li> <li> <p> <a>DescribeSolution</a> </p> </li> <li> <p> <a>DeleteSolution</a> </p> </li> </ul></p>
    async fn create_solution_version(
        &self,
        input: CreateSolutionVersionRequest,
    ) -> Result<CreateSolutionVersionResponse, RusotoError<CreateSolutionVersionError>>;

    /// <p>Removes a campaign by deleting the solution deployment. The solution that the campaign is based on is not deleted and can be redeployed when needed. A deleted campaign can no longer be specified in a <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetRecommendations.html">GetRecommendations</a> request. For more information on campaigns, see <a>CreateCampaign</a>.</p>
    async fn delete_campaign(
        &self,
        input: DeleteCampaignRequest,
    ) -> Result<(), RusotoError<DeleteCampaignError>>;

    /// <p>Deletes a dataset. You can't delete a dataset if an associated <code>DatasetImportJob</code> or <code>SolutionVersion</code> is in the CREATE PENDING or IN PROGRESS state. For more information on datasets, see <a>CreateDataset</a>.</p>
    async fn delete_dataset(
        &self,
        input: DeleteDatasetRequest,
    ) -> Result<(), RusotoError<DeleteDatasetError>>;

    /// <p><p>Deletes a dataset group. Before you delete a dataset group, you must delete the following:</p> <ul> <li> <p>All associated event trackers.</p> </li> <li> <p>All associated solutions.</p> </li> <li> <p>All datasets in the dataset group.</p> </li> </ul></p>
    async fn delete_dataset_group(
        &self,
        input: DeleteDatasetGroupRequest,
    ) -> Result<(), RusotoError<DeleteDatasetGroupError>>;

    /// <p>Deletes the event tracker. Does not delete the event-interactions dataset from the associated dataset group. For more information on event trackers, see <a>CreateEventTracker</a>.</p>
    async fn delete_event_tracker(
        &self,
        input: DeleteEventTrackerRequest,
    ) -> Result<(), RusotoError<DeleteEventTrackerError>>;

    /// <p>Deletes a filter.</p>
    async fn delete_filter(
        &self,
        input: DeleteFilterRequest,
    ) -> Result<(), RusotoError<DeleteFilterError>>;

    /// <p>Deletes a schema. Before deleting a schema, you must delete all datasets referencing the schema. For more information on schemas, see <a>CreateSchema</a>.</p>
    async fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> Result<(), RusotoError<DeleteSchemaError>>;

    /// <p>Deletes all versions of a solution and the <code>Solution</code> object itself. Before deleting a solution, you must delete all campaigns based on the solution. To determine what campaigns are using the solution, call <a>ListCampaigns</a> and supply the Amazon Resource Name (ARN) of the solution. You can't delete a solution if an associated <code>SolutionVersion</code> is in the CREATE PENDING or IN PROGRESS state. For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn delete_solution(
        &self,
        input: DeleteSolutionRequest,
    ) -> Result<(), RusotoError<DeleteSolutionError>>;

    /// <p>Describes the given algorithm.</p>
    async fn describe_algorithm(
        &self,
        input: DescribeAlgorithmRequest,
    ) -> Result<DescribeAlgorithmResponse, RusotoError<DescribeAlgorithmError>>;

    /// <p>Gets the properties of a batch inference job including name, Amazon Resource Name (ARN), status, input and output configurations, and the ARN of the solution version used to generate the recommendations.</p>
    async fn describe_batch_inference_job(
        &self,
        input: DescribeBatchInferenceJobRequest,
    ) -> Result<DescribeBatchInferenceJobResponse, RusotoError<DescribeBatchInferenceJobError>>;

    /// <p>Describes the given campaign, including its status.</p> <p>A campaign can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN_PROGRESS</p> </li> </ul> <p>When the <code>status</code> is <code>CREATE FAILED</code>, the response includes the <code>failureReason</code> key, which describes why.</p> <p>For more information on campaigns, see <a>CreateCampaign</a>.</p>
    async fn describe_campaign(
        &self,
        input: DescribeCampaignRequest,
    ) -> Result<DescribeCampaignResponse, RusotoError<DescribeCampaignError>>;

    /// <p>Describes the given dataset. For more information on datasets, see <a>CreateDataset</a>.</p>
    async fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> Result<DescribeDatasetResponse, RusotoError<DescribeDatasetError>>;

    /// <p>Describes the given dataset group. For more information on dataset groups, see <a>CreateDatasetGroup</a>.</p>
    async fn describe_dataset_group(
        &self,
        input: DescribeDatasetGroupRequest,
    ) -> Result<DescribeDatasetGroupResponse, RusotoError<DescribeDatasetGroupError>>;

    /// <p>Describes the dataset import job created by <a>CreateDatasetImportJob</a>, including the import job status.</p>
    async fn describe_dataset_import_job(
        &self,
        input: DescribeDatasetImportJobRequest,
    ) -> Result<DescribeDatasetImportJobResponse, RusotoError<DescribeDatasetImportJobError>>;

    /// <p>Describes an event tracker. The response includes the <code>trackingId</code> and <code>status</code> of the event tracker. For more information on event trackers, see <a>CreateEventTracker</a>.</p>
    async fn describe_event_tracker(
        &self,
        input: DescribeEventTrackerRequest,
    ) -> Result<DescribeEventTrackerResponse, RusotoError<DescribeEventTrackerError>>;

    /// <p>Describes the given feature transformation.</p>
    async fn describe_feature_transformation(
        &self,
        input: DescribeFeatureTransformationRequest,
    ) -> Result<
        DescribeFeatureTransformationResponse,
        RusotoError<DescribeFeatureTransformationError>,
    >;

    /// <p>Describes a filter's properties.</p>
    async fn describe_filter(
        &self,
        input: DescribeFilterRequest,
    ) -> Result<DescribeFilterResponse, RusotoError<DescribeFilterError>>;

    /// <p>Describes a recipe.</p> <p>A recipe contains three items:</p> <ul> <li> <p>An algorithm that trains a model.</p> </li> <li> <p>Hyperparameters that govern the training.</p> </li> <li> <p>Feature transformation information for modifying the input data before training.</p> </li> </ul> <p>Amazon Personalize provides a set of predefined recipes. You specify a recipe when you create a solution with the <a>CreateSolution</a> API. <code>CreateSolution</code> trains a model by using the algorithm in the specified recipe and a training dataset. The solution, when deployed as a campaign, can provide recommendations using the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetRecommendations.html">GetRecommendations</a> API.</p>
    async fn describe_recipe(
        &self,
        input: DescribeRecipeRequest,
    ) -> Result<DescribeRecipeResponse, RusotoError<DescribeRecipeError>>;

    /// <p>Describes a schema. For more information on schemas, see <a>CreateSchema</a>.</p>
    async fn describe_schema(
        &self,
        input: DescribeSchemaRequest,
    ) -> Result<DescribeSchemaResponse, RusotoError<DescribeSchemaError>>;

    /// <p>Describes a solution. For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn describe_solution(
        &self,
        input: DescribeSolutionRequest,
    ) -> Result<DescribeSolutionResponse, RusotoError<DescribeSolutionError>>;

    /// <p>Describes a specific version of a solution. For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn describe_solution_version(
        &self,
        input: DescribeSolutionVersionRequest,
    ) -> Result<DescribeSolutionVersionResponse, RusotoError<DescribeSolutionVersionError>>;

    /// <p>Gets the metrics for the specified solution version.</p>
    async fn get_solution_metrics(
        &self,
        input: GetSolutionMetricsRequest,
    ) -> Result<GetSolutionMetricsResponse, RusotoError<GetSolutionMetricsError>>;

    /// <p>Gets a list of the batch inference jobs that have been performed off of a solution version.</p>
    async fn list_batch_inference_jobs(
        &self,
        input: ListBatchInferenceJobsRequest,
    ) -> Result<ListBatchInferenceJobsResponse, RusotoError<ListBatchInferenceJobsError>>;

    /// <p>Returns a list of campaigns that use the given solution. When a solution is not specified, all the campaigns associated with the account are listed. The response provides the properties for each campaign, including the Amazon Resource Name (ARN). For more information on campaigns, see <a>CreateCampaign</a>.</p>
    async fn list_campaigns(
        &self,
        input: ListCampaignsRequest,
    ) -> Result<ListCampaignsResponse, RusotoError<ListCampaignsError>>;

    /// <p>Returns a list of dataset groups. The response provides the properties for each dataset group, including the Amazon Resource Name (ARN). For more information on dataset groups, see <a>CreateDatasetGroup</a>.</p>
    async fn list_dataset_groups(
        &self,
        input: ListDatasetGroupsRequest,
    ) -> Result<ListDatasetGroupsResponse, RusotoError<ListDatasetGroupsError>>;

    /// <p>Returns a list of dataset import jobs that use the given dataset. When a dataset is not specified, all the dataset import jobs associated with the account are listed. The response provides the properties for each dataset import job, including the Amazon Resource Name (ARN). For more information on dataset import jobs, see <a>CreateDatasetImportJob</a>. For more information on datasets, see <a>CreateDataset</a>.</p>
    async fn list_dataset_import_jobs(
        &self,
        input: ListDatasetImportJobsRequest,
    ) -> Result<ListDatasetImportJobsResponse, RusotoError<ListDatasetImportJobsError>>;

    /// <p>Returns the list of datasets contained in the given dataset group. The response provides the properties for each dataset, including the Amazon Resource Name (ARN). For more information on datasets, see <a>CreateDataset</a>.</p>
    async fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> Result<ListDatasetsResponse, RusotoError<ListDatasetsError>>;

    /// <p>Returns the list of event trackers associated with the account. The response provides the properties for each event tracker, including the Amazon Resource Name (ARN) and tracking ID. For more information on event trackers, see <a>CreateEventTracker</a>.</p>
    async fn list_event_trackers(
        &self,
        input: ListEventTrackersRequest,
    ) -> Result<ListEventTrackersResponse, RusotoError<ListEventTrackersError>>;

    /// <p>Lists all filters that belong to a given dataset group.</p>
    async fn list_filters(
        &self,
        input: ListFiltersRequest,
    ) -> Result<ListFiltersResponse, RusotoError<ListFiltersError>>;

    /// <p>Returns a list of available recipes. The response provides the properties for each recipe, including the recipe's Amazon Resource Name (ARN).</p>
    async fn list_recipes(
        &self,
        input: ListRecipesRequest,
    ) -> Result<ListRecipesResponse, RusotoError<ListRecipesError>>;

    /// <p>Returns the list of schemas associated with the account. The response provides the properties for each schema, including the Amazon Resource Name (ARN). For more information on schemas, see <a>CreateSchema</a>.</p>
    async fn list_schemas(
        &self,
        input: ListSchemasRequest,
    ) -> Result<ListSchemasResponse, RusotoError<ListSchemasError>>;

    /// <p>Returns a list of solution versions for the given solution. When a solution is not specified, all the solution versions associated with the account are listed. The response provides the properties for each solution version, including the Amazon Resource Name (ARN). For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn list_solution_versions(
        &self,
        input: ListSolutionVersionsRequest,
    ) -> Result<ListSolutionVersionsResponse, RusotoError<ListSolutionVersionsError>>;

    /// <p>Returns a list of solutions that use the given dataset group. When a dataset group is not specified, all the solutions associated with the account are listed. The response provides the properties for each solution, including the Amazon Resource Name (ARN). For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn list_solutions(
        &self,
        input: ListSolutionsRequest,
    ) -> Result<ListSolutionsResponse, RusotoError<ListSolutionsError>>;

    /// <p>Updates a campaign by either deploying a new solution or changing the value of the campaign's <code>minProvisionedTPS</code> parameter.</p> <p>To update a campaign, the campaign status must be ACTIVE or CREATE FAILED. Check the campaign status using the <a>DescribeCampaign</a> API.</p> <note> <p>You must wait until the <code>status</code> of the updated campaign is <code>ACTIVE</code> before asking the campaign for recommendations.</p> </note> <p>For more information on campaigns, see <a>CreateCampaign</a>.</p>
    async fn update_campaign(
        &self,
        input: UpdateCampaignRequest,
    ) -> Result<UpdateCampaignResponse, RusotoError<UpdateCampaignError>>;
}
/// A client for the Amazon Personalize API.
#[derive(Clone)]
pub struct PersonalizeClient {
    client: Client,
    region: region::Region,
}

impl PersonalizeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PersonalizeClient {
        PersonalizeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PersonalizeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PersonalizeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PersonalizeClient {
        PersonalizeClient { client, region }
    }
}

#[async_trait]
impl Personalize for PersonalizeClient {
    /// <p>Creates a batch inference job. The operation can handle up to 50 million records and the input file must be in JSON format. For more information, see <a>recommendations-batch</a>.</p>
    async fn create_batch_inference_job(
        &self,
        input: CreateBatchInferenceJobRequest,
    ) -> Result<CreateBatchInferenceJobResponse, RusotoError<CreateBatchInferenceJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateBatchInferenceJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateBatchInferenceJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateBatchInferenceJobResponse, _>()
    }

    /// <p><p>Creates a campaign by deploying a solution version. When a client calls the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetRecommendations.html">GetRecommendations</a> and <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetPersonalizedRanking.html">GetPersonalizedRanking</a> APIs, a campaign is specified in the request.</p> <p> <b>Minimum Provisioned TPS and Auto-Scaling</b> </p> <p>A transaction is a single <code>GetRecommendations</code> or <code>GetPersonalizedRanking</code> call. Transactions per second (TPS) is the throughput and unit of billing for Amazon Personalize. The minimum provisioned TPS (<code>minProvisionedTPS</code>) specifies the baseline throughput provisioned by Amazon Personalize, and thus, the minimum billing charge. If your TPS increases beyond <code>minProvisionedTPS</code>, Amazon Personalize auto-scales the provisioned capacity up and down, but never below <code>minProvisionedTPS</code>, to maintain a 70% utilization. There&#39;s a short time delay while the capacity is increased that might cause loss of transactions. It&#39;s recommended to start with a low <code>minProvisionedTPS</code>, track your usage using Amazon CloudWatch metrics, and then increase the <code>minProvisionedTPS</code> as necessary.</p> <p> <b>Status</b> </p> <p>A campaign can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul> <p>To get the campaign status, call <a>DescribeCampaign</a>.</p> <note> <p>Wait until the <code>status</code> of the campaign is <code>ACTIVE</code> before asking the campaign for recommendations.</p> </note> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListCampaigns</a> </p> </li> <li> <p> <a>DescribeCampaign</a> </p> </li> <li> <p> <a>UpdateCampaign</a> </p> </li> <li> <p> <a>DeleteCampaign</a> </p> </li> </ul></p>
    async fn create_campaign(
        &self,
        input: CreateCampaignRequest,
    ) -> Result<CreateCampaignResponse, RusotoError<CreateCampaignError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateCampaign");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateCampaignError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateCampaignResponse, _>()
    }

    /// <p><p>Creates an empty dataset and adds it to the specified dataset group. Use <a>CreateDatasetImportJob</a> to import your training data to a dataset.</p> <p>There are three types of datasets:</p> <ul> <li> <p>Interactions</p> </li> <li> <p>Items</p> </li> <li> <p>Users</p> </li> </ul> <p>Each dataset type has an associated schema with required field types. Only the <code>Interactions</code> dataset is required in order to train a model (also referred to as creating a solution).</p> <p>A dataset can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul> <p>To get the status of the dataset, call <a>DescribeDataset</a>.</p> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>CreateDatasetGroup</a> </p> </li> <li> <p> <a>ListDatasets</a> </p> </li> <li> <p> <a>DescribeDataset</a> </p> </li> <li> <p> <a>DeleteDataset</a> </p> </li> </ul></p>
    async fn create_dataset(
        &self,
        input: CreateDatasetRequest,
    ) -> Result<CreateDatasetResponse, RusotoError<CreateDatasetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateDataset");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDatasetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDatasetResponse, _>()
    }

    /// <p><p>Creates an empty dataset group. A dataset group contains related datasets that supply data for training a model. A dataset group can contain at most three datasets, one for each type of dataset:</p> <ul> <li> <p>Interactions</p> </li> <li> <p>Items</p> </li> <li> <p>Users</p> </li> </ul> <p>To train a model (create a solution), a dataset group that contains an <code>Interactions</code> dataset is required. Call <a>CreateDataset</a> to add a dataset to the group.</p> <p>A dataset group can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING</p> </li> </ul> <p>To get the status of the dataset group, call <a>DescribeDatasetGroup</a>. If the status shows as CREATE FAILED, the response includes a <code>failureReason</code> key, which describes why the creation failed.</p> <note> <p>You must wait until the <code>status</code> of the dataset group is <code>ACTIVE</code> before adding a dataset to the group.</p> </note> <p>You can specify an AWS Key Management Service (KMS) key to encrypt the datasets in the group. If you specify a KMS key, you must also include an AWS Identity and Access Management (IAM) role that has permission to access the key.</p> <p class="title"> <b>APIs that require a dataset group ARN in the request</b> </p> <ul> <li> <p> <a>CreateDataset</a> </p> </li> <li> <p> <a>CreateEventTracker</a> </p> </li> <li> <p> <a>CreateSolution</a> </p> </li> </ul> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListDatasetGroups</a> </p> </li> <li> <p> <a>DescribeDatasetGroup</a> </p> </li> <li> <p> <a>DeleteDatasetGroup</a> </p> </li> </ul></p>
    async fn create_dataset_group(
        &self,
        input: CreateDatasetGroupRequest,
    ) -> Result<CreateDatasetGroupResponse, RusotoError<CreateDatasetGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateDatasetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDatasetGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDatasetGroupResponse, _>()
    }

    /// <p><p>Creates a job that imports training data from your data source (an Amazon S3 bucket) to an Amazon Personalize dataset. To allow Amazon Personalize to import the training data, you must specify an AWS Identity and Access Management (IAM) role that has permission to read from the data source, as Amazon Personalize makes a copy of your data and processes it in an internal AWS system.</p> <important> <p>The dataset import job replaces any previous data in the dataset.</p> </important> <p> <b>Status</b> </p> <p>A dataset import job can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul> <p>To get the status of the import job, call <a>DescribeDatasetImportJob</a>, providing the Amazon Resource Name (ARN) of the dataset import job. The dataset import is complete when the status shows as ACTIVE. If the status shows as CREATE FAILED, the response includes a <code>failureReason</code> key, which describes why the job failed.</p> <note> <p>Importing takes time. You must wait until the status shows as ACTIVE before training a model using the dataset.</p> </note> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListDatasetImportJobs</a> </p> </li> <li> <p> <a>DescribeDatasetImportJob</a> </p> </li> </ul></p>
    async fn create_dataset_import_job(
        &self,
        input: CreateDatasetImportJobRequest,
    ) -> Result<CreateDatasetImportJobResponse, RusotoError<CreateDatasetImportJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateDatasetImportJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDatasetImportJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateDatasetImportJobResponse, _>()
    }

    /// <p><p>Creates an event tracker that you use when sending event data to the specified dataset group using the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_UBS_PutEvents.html">PutEvents</a> API.</p> <p>When Amazon Personalize creates an event tracker, it also creates an <i>event-interactions</i> dataset in the dataset group associated with the event tracker. The event-interactions dataset stores the event data from the <code>PutEvents</code> call. The contents of this dataset are not available to the user.</p> <note> <p>Only one event tracker can be associated with a dataset group. You will get an error if you call <code>CreateEventTracker</code> using the same dataset group as an existing event tracker.</p> </note> <p>When you send event data you include your tracking ID. The tracking ID identifies the customer and authorizes the customer to send the data.</p> <p>The event tracker can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN<em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN</em>PROGRESS</p> </li> </ul> <p>To get the status of the event tracker, call <a>DescribeEventTracker</a>.</p> <note> <p>The event tracker must be in the ACTIVE state before using the tracking ID.</p> </note> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListEventTrackers</a> </p> </li> <li> <p> <a>DescribeEventTracker</a> </p> </li> <li> <p> <a>DeleteEventTracker</a> </p> </li> </ul></p>
    async fn create_event_tracker(
        &self,
        input: CreateEventTrackerRequest,
    ) -> Result<CreateEventTrackerResponse, RusotoError<CreateEventTrackerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateEventTracker");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateEventTrackerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateEventTrackerResponse, _>()
    }

    /// <p>Creates a recommendation filter. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filters.html">Using Filters with Amazon Personalize</a>.</p>
    async fn create_filter(
        &self,
        input: CreateFilterRequest,
    ) -> Result<CreateFilterResponse, RusotoError<CreateFilterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateFilterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateFilterResponse, _>()
    }

    /// <p><p>Creates an Amazon Personalize schema from the specified schema string. The schema you create must be in Avro JSON format.</p> <p>Amazon Personalize recognizes three schema variants. Each schema is associated with a dataset type and has a set of required field and keywords. You specify a schema when you call <a>CreateDataset</a>.</p> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListSchemas</a> </p> </li> <li> <p> <a>DescribeSchema</a> </p> </li> <li> <p> <a>DeleteSchema</a> </p> </li> </ul></p>
    async fn create_schema(
        &self,
        input: CreateSchemaRequest,
    ) -> Result<CreateSchemaResponse, RusotoError<CreateSchemaError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateSchema");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSchemaError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateSchemaResponse, _>()
    }

    /// <p><p>Creates the configuration for training a model. A trained model is known as a solution. After the configuration is created, you train the model (create a solution) by calling the <a>CreateSolutionVersion</a> operation. Every time you call <code>CreateSolutionVersion</code>, a new version of the solution is created.</p> <p>After creating a solution version, you check its accuracy by calling <a>GetSolutionMetrics</a>. When you are satisfied with the version, you deploy it using <a>CreateCampaign</a>. The campaign provides recommendations to a client through the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetRecommendations.html">GetRecommendations</a> API.</p> <p>To train a model, Amazon Personalize requires training data and a recipe. The training data comes from the dataset group that you provide in the request. A recipe specifies the training algorithm and a feature transformation. You can specify one of the predefined recipes provided by Amazon Personalize. Alternatively, you can specify <code>performAutoML</code> and Amazon Personalize will analyze your data and select the optimum USER<em>PERSONALIZATION recipe for you.</p> <p> <b>Status</b> </p> <p>A solution can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN</em>PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN_PROGRESS</p> </li> </ul> <p>To get the status of the solution, call <a>DescribeSolution</a>. Wait until the status shows as ACTIVE before calling <code>CreateSolutionVersion</code>.</p> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListSolutions</a> </p> </li> <li> <p> <a>CreateSolutionVersion</a> </p> </li> <li> <p> <a>DescribeSolution</a> </p> </li> <li> <p> <a>DeleteSolution</a> </p> </li> </ul> <ul> <li> <p> <a>ListSolutionVersions</a> </p> </li> <li> <p> <a>DescribeSolutionVersion</a> </p> </li> </ul></p>
    async fn create_solution(
        &self,
        input: CreateSolutionRequest,
    ) -> Result<CreateSolutionResponse, RusotoError<CreateSolutionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateSolution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSolutionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateSolutionResponse, _>()
    }

    /// <p><p>Trains or retrains an active solution. A solution is created using the <a>CreateSolution</a> operation and must be in the ACTIVE state before calling <code>CreateSolutionVersion</code>. A new version of the solution is created every time you call this operation.</p> <p> <b>Status</b> </p> <p>A solution version can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> </ul> <p>To get the status of the version, call <a>DescribeSolutionVersion</a>. Wait until the status shows as ACTIVE before calling <code>CreateCampaign</code>.</p> <p>If the status shows as CREATE FAILED, the response includes a <code>failureReason</code> key, which describes why the job failed.</p> <p class="title"> <b>Related APIs</b> </p> <ul> <li> <p> <a>ListSolutionVersions</a> </p> </li> <li> <p> <a>DescribeSolutionVersion</a> </p> </li> </ul> <ul> <li> <p> <a>ListSolutions</a> </p> </li> <li> <p> <a>CreateSolution</a> </p> </li> <li> <p> <a>DescribeSolution</a> </p> </li> <li> <p> <a>DeleteSolution</a> </p> </li> </ul></p>
    async fn create_solution_version(
        &self,
        input: CreateSolutionVersionRequest,
    ) -> Result<CreateSolutionVersionResponse, RusotoError<CreateSolutionVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.CreateSolutionVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSolutionVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateSolutionVersionResponse, _>()
    }

    /// <p>Removes a campaign by deleting the solution deployment. The solution that the campaign is based on is not deleted and can be redeployed when needed. A deleted campaign can no longer be specified in a <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetRecommendations.html">GetRecommendations</a> request. For more information on campaigns, see <a>CreateCampaign</a>.</p>
    async fn delete_campaign(
        &self,
        input: DeleteCampaignRequest,
    ) -> Result<(), RusotoError<DeleteCampaignError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DeleteCampaign");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteCampaignError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a dataset. You can't delete a dataset if an associated <code>DatasetImportJob</code> or <code>SolutionVersion</code> is in the CREATE PENDING or IN PROGRESS state. For more information on datasets, see <a>CreateDataset</a>.</p>
    async fn delete_dataset(
        &self,
        input: DeleteDatasetRequest,
    ) -> Result<(), RusotoError<DeleteDatasetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DeleteDataset");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDatasetError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a dataset group. Before you delete a dataset group, you must delete the following:</p> <ul> <li> <p>All associated event trackers.</p> </li> <li> <p>All associated solutions.</p> </li> <li> <p>All datasets in the dataset group.</p> </li> </ul></p>
    async fn delete_dataset_group(
        &self,
        input: DeleteDatasetGroupRequest,
    ) -> Result<(), RusotoError<DeleteDatasetGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DeleteDatasetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDatasetGroupError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the event tracker. Does not delete the event-interactions dataset from the associated dataset group. For more information on event trackers, see <a>CreateEventTracker</a>.</p>
    async fn delete_event_tracker(
        &self,
        input: DeleteEventTrackerRequest,
    ) -> Result<(), RusotoError<DeleteEventTrackerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DeleteEventTracker");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEventTrackerError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a filter.</p>
    async fn delete_filter(
        &self,
        input: DeleteFilterRequest,
    ) -> Result<(), RusotoError<DeleteFilterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DeleteFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteFilterError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a schema. Before deleting a schema, you must delete all datasets referencing the schema. For more information on schemas, see <a>CreateSchema</a>.</p>
    async fn delete_schema(
        &self,
        input: DeleteSchemaRequest,
    ) -> Result<(), RusotoError<DeleteSchemaError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DeleteSchema");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSchemaError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes all versions of a solution and the <code>Solution</code> object itself. Before deleting a solution, you must delete all campaigns based on the solution. To determine what campaigns are using the solution, call <a>ListCampaigns</a> and supply the Amazon Resource Name (ARN) of the solution. You can't delete a solution if an associated <code>SolutionVersion</code> is in the CREATE PENDING or IN PROGRESS state. For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn delete_solution(
        &self,
        input: DeleteSolutionRequest,
    ) -> Result<(), RusotoError<DeleteSolutionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DeleteSolution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSolutionError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Describes the given algorithm.</p>
    async fn describe_algorithm(
        &self,
        input: DescribeAlgorithmRequest,
    ) -> Result<DescribeAlgorithmResponse, RusotoError<DescribeAlgorithmError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeAlgorithm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAlgorithmError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeAlgorithmResponse, _>()
    }

    /// <p>Gets the properties of a batch inference job including name, Amazon Resource Name (ARN), status, input and output configurations, and the ARN of the solution version used to generate the recommendations.</p>
    async fn describe_batch_inference_job(
        &self,
        input: DescribeBatchInferenceJobRequest,
    ) -> Result<DescribeBatchInferenceJobResponse, RusotoError<DescribeBatchInferenceJobError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonPersonalize.DescribeBatchInferenceJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeBatchInferenceJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeBatchInferenceJobResponse, _>()
    }

    /// <p>Describes the given campaign, including its status.</p> <p>A campaign can be in one of the following states:</p> <ul> <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li> <li> <p>DELETE PENDING &gt; DELETE IN_PROGRESS</p> </li> </ul> <p>When the <code>status</code> is <code>CREATE FAILED</code>, the response includes the <code>failureReason</code> key, which describes why.</p> <p>For more information on campaigns, see <a>CreateCampaign</a>.</p>
    async fn describe_campaign(
        &self,
        input: DescribeCampaignRequest,
    ) -> Result<DescribeCampaignResponse, RusotoError<DescribeCampaignError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeCampaign");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCampaignError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeCampaignResponse, _>()
    }

    /// <p>Describes the given dataset. For more information on datasets, see <a>CreateDataset</a>.</p>
    async fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> Result<DescribeDatasetResponse, RusotoError<DescribeDatasetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeDataset");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDatasetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeDatasetResponse, _>()
    }

    /// <p>Describes the given dataset group. For more information on dataset groups, see <a>CreateDatasetGroup</a>.</p>
    async fn describe_dataset_group(
        &self,
        input: DescribeDatasetGroupRequest,
    ) -> Result<DescribeDatasetGroupResponse, RusotoError<DescribeDatasetGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeDatasetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDatasetGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeDatasetGroupResponse, _>()
    }

    /// <p>Describes the dataset import job created by <a>CreateDatasetImportJob</a>, including the import job status.</p>
    async fn describe_dataset_import_job(
        &self,
        input: DescribeDatasetImportJobRequest,
    ) -> Result<DescribeDatasetImportJobResponse, RusotoError<DescribeDatasetImportJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeDatasetImportJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDatasetImportJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeDatasetImportJobResponse, _>()
    }

    /// <p>Describes an event tracker. The response includes the <code>trackingId</code> and <code>status</code> of the event tracker. For more information on event trackers, see <a>CreateEventTracker</a>.</p>
    async fn describe_event_tracker(
        &self,
        input: DescribeEventTrackerRequest,
    ) -> Result<DescribeEventTrackerResponse, RusotoError<DescribeEventTrackerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeEventTracker");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEventTrackerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEventTrackerResponse, _>()
    }

    /// <p>Describes the given feature transformation.</p>
    async fn describe_feature_transformation(
        &self,
        input: DescribeFeatureTransformationRequest,
    ) -> Result<
        DescribeFeatureTransformationResponse,
        RusotoError<DescribeFeatureTransformationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonPersonalize.DescribeFeatureTransformation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFeatureTransformationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeFeatureTransformationResponse, _>()
    }

    /// <p>Describes a filter's properties.</p>
    async fn describe_filter(
        &self,
        input: DescribeFilterRequest,
    ) -> Result<DescribeFilterResponse, RusotoError<DescribeFilterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFilterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeFilterResponse, _>()
    }

    /// <p>Describes a recipe.</p> <p>A recipe contains three items:</p> <ul> <li> <p>An algorithm that trains a model.</p> </li> <li> <p>Hyperparameters that govern the training.</p> </li> <li> <p>Feature transformation information for modifying the input data before training.</p> </li> </ul> <p>Amazon Personalize provides a set of predefined recipes. You specify a recipe when you create a solution with the <a>CreateSolution</a> API. <code>CreateSolution</code> trains a model by using the algorithm in the specified recipe and a training dataset. The solution, when deployed as a campaign, can provide recommendations using the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_RS_GetRecommendations.html">GetRecommendations</a> API.</p>
    async fn describe_recipe(
        &self,
        input: DescribeRecipeRequest,
    ) -> Result<DescribeRecipeResponse, RusotoError<DescribeRecipeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeRecipe");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeRecipeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeRecipeResponse, _>()
    }

    /// <p>Describes a schema. For more information on schemas, see <a>CreateSchema</a>.</p>
    async fn describe_schema(
        &self,
        input: DescribeSchemaRequest,
    ) -> Result<DescribeSchemaResponse, RusotoError<DescribeSchemaError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeSchema");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSchemaError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeSchemaResponse, _>()
    }

    /// <p>Describes a solution. For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn describe_solution(
        &self,
        input: DescribeSolutionRequest,
    ) -> Result<DescribeSolutionResponse, RusotoError<DescribeSolutionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeSolution");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSolutionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeSolutionResponse, _>()
    }

    /// <p>Describes a specific version of a solution. For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn describe_solution_version(
        &self,
        input: DescribeSolutionVersionRequest,
    ) -> Result<DescribeSolutionVersionResponse, RusotoError<DescribeSolutionVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.DescribeSolutionVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSolutionVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeSolutionVersionResponse, _>()
    }

    /// <p>Gets the metrics for the specified solution version.</p>
    async fn get_solution_metrics(
        &self,
        input: GetSolutionMetricsRequest,
    ) -> Result<GetSolutionMetricsResponse, RusotoError<GetSolutionMetricsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.GetSolutionMetrics");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSolutionMetricsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetSolutionMetricsResponse, _>()
    }

    /// <p>Gets a list of the batch inference jobs that have been performed off of a solution version.</p>
    async fn list_batch_inference_jobs(
        &self,
        input: ListBatchInferenceJobsRequest,
    ) -> Result<ListBatchInferenceJobsResponse, RusotoError<ListBatchInferenceJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListBatchInferenceJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListBatchInferenceJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListBatchInferenceJobsResponse, _>()
    }

    /// <p>Returns a list of campaigns that use the given solution. When a solution is not specified, all the campaigns associated with the account are listed. The response provides the properties for each campaign, including the Amazon Resource Name (ARN). For more information on campaigns, see <a>CreateCampaign</a>.</p>
    async fn list_campaigns(
        &self,
        input: ListCampaignsRequest,
    ) -> Result<ListCampaignsResponse, RusotoError<ListCampaignsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListCampaigns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListCampaignsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListCampaignsResponse, _>()
    }

    /// <p>Returns a list of dataset groups. The response provides the properties for each dataset group, including the Amazon Resource Name (ARN). For more information on dataset groups, see <a>CreateDatasetGroup</a>.</p>
    async fn list_dataset_groups(
        &self,
        input: ListDatasetGroupsRequest,
    ) -> Result<ListDatasetGroupsResponse, RusotoError<ListDatasetGroupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListDatasetGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDatasetGroupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListDatasetGroupsResponse, _>()
    }

    /// <p>Returns a list of dataset import jobs that use the given dataset. When a dataset is not specified, all the dataset import jobs associated with the account are listed. The response provides the properties for each dataset import job, including the Amazon Resource Name (ARN). For more information on dataset import jobs, see <a>CreateDatasetImportJob</a>. For more information on datasets, see <a>CreateDataset</a>.</p>
    async fn list_dataset_import_jobs(
        &self,
        input: ListDatasetImportJobsRequest,
    ) -> Result<ListDatasetImportJobsResponse, RusotoError<ListDatasetImportJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListDatasetImportJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDatasetImportJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListDatasetImportJobsResponse, _>()
    }

    /// <p>Returns the list of datasets contained in the given dataset group. The response provides the properties for each dataset, including the Amazon Resource Name (ARN). For more information on datasets, see <a>CreateDataset</a>.</p>
    async fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> Result<ListDatasetsResponse, RusotoError<ListDatasetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListDatasets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDatasetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListDatasetsResponse, _>()
    }

    /// <p>Returns the list of event trackers associated with the account. The response provides the properties for each event tracker, including the Amazon Resource Name (ARN) and tracking ID. For more information on event trackers, see <a>CreateEventTracker</a>.</p>
    async fn list_event_trackers(
        &self,
        input: ListEventTrackersRequest,
    ) -> Result<ListEventTrackersResponse, RusotoError<ListEventTrackersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListEventTrackers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListEventTrackersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListEventTrackersResponse, _>()
    }

    /// <p>Lists all filters that belong to a given dataset group.</p>
    async fn list_filters(
        &self,
        input: ListFiltersRequest,
    ) -> Result<ListFiltersResponse, RusotoError<ListFiltersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListFilters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListFiltersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListFiltersResponse, _>()
    }

    /// <p>Returns a list of available recipes. The response provides the properties for each recipe, including the recipe's Amazon Resource Name (ARN).</p>
    async fn list_recipes(
        &self,
        input: ListRecipesRequest,
    ) -> Result<ListRecipesResponse, RusotoError<ListRecipesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListRecipes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRecipesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRecipesResponse, _>()
    }

    /// <p>Returns the list of schemas associated with the account. The response provides the properties for each schema, including the Amazon Resource Name (ARN). For more information on schemas, see <a>CreateSchema</a>.</p>
    async fn list_schemas(
        &self,
        input: ListSchemasRequest,
    ) -> Result<ListSchemasResponse, RusotoError<ListSchemasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListSchemas");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSchemasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListSchemasResponse, _>()
    }

    /// <p>Returns a list of solution versions for the given solution. When a solution is not specified, all the solution versions associated with the account are listed. The response provides the properties for each solution version, including the Amazon Resource Name (ARN). For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn list_solution_versions(
        &self,
        input: ListSolutionVersionsRequest,
    ) -> Result<ListSolutionVersionsResponse, RusotoError<ListSolutionVersionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListSolutionVersions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSolutionVersionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListSolutionVersionsResponse, _>()
    }

    /// <p>Returns a list of solutions that use the given dataset group. When a dataset group is not specified, all the solutions associated with the account are listed. The response provides the properties for each solution, including the Amazon Resource Name (ARN). For more information on solutions, see <a>CreateSolution</a>.</p>
    async fn list_solutions(
        &self,
        input: ListSolutionsRequest,
    ) -> Result<ListSolutionsResponse, RusotoError<ListSolutionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.ListSolutions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSolutionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListSolutionsResponse, _>()
    }

    /// <p>Updates a campaign by either deploying a new solution or changing the value of the campaign's <code>minProvisionedTPS</code> parameter.</p> <p>To update a campaign, the campaign status must be ACTIVE or CREATE FAILED. Check the campaign status using the <a>DescribeCampaign</a> API.</p> <note> <p>You must wait until the <code>status</code> of the updated campaign is <code>ACTIVE</code> before asking the campaign for recommendations.</p> </note> <p>For more information on campaigns, see <a>CreateCampaign</a>.</p>
    async fn update_campaign(
        &self,
        input: UpdateCampaignRequest,
    ) -> Result<UpdateCampaignResponse, RusotoError<UpdateCampaignError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonPersonalize.UpdateCampaign");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateCampaignError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateCampaignResponse, _>()
    }
}
