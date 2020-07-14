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
/// <p>Specifies a categorical hyperparameter and it's range of tunable values. This object is part of the <a>ParameterRanges</a> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoricalParameterRange {
    /// <p>The name of the categorical hyperparameter to tune.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A list of the tunable categories for the hyperparameter.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Specifies a continuous hyperparameter and it's range of tunable values. This object is part of the <a>ParameterRanges</a> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuousParameterRange {
    /// <p>The maximum tunable value of the hyperparameter.</p>
    #[serde(rename = "MaxValue")]
    pub max_value: f64,
    /// <p>The minimum tunable value of the hyperparameter.</p>
    #[serde(rename = "MinValue")]
    pub min_value: f64,
    /// <p>The name of the hyperparameter to tune.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The scale that hyperparameter tuning uses to search the hyperparameter range. Valid values:</p> <dl> <dt>Auto</dt> <dd> <p>Amazon Forecast hyperparameter tuning chooses the best scale for the hyperparameter.</p> </dd> <dt>Linear</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p> </dd> <dt>Logarithmic</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p> <p>Logarithmic scaling works only for ranges that have values greater than 0.</p> </dd> <dt>ReverseLogarithmic</dt> <dd> <p>hyperparameter tuning searches the values in the hyperparameter range by using a reverse logarithmic scale.</p> <p>Reverse logarithmic scaling works only for ranges that are entirely within the range 0 &lt;= x &lt; 1.0.</p> </dd> </dl> <p>For information about choosing a hyperparameter scale, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p>
    #[serde(rename = "ScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDatasetGroupRequest {
    /// <p>An array of Amazon Resource Names (ARNs) of the datasets that you want to include in the dataset group.</p>
    #[serde(rename = "DatasetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arns: Option<Vec<String>>,
    /// <p>A name for the dataset group.</p>
    #[serde(rename = "DatasetGroupName")]
    pub dataset_group_name: String,
    /// <p>The domain associated with the dataset group. When you add a dataset to a dataset group, this value and the value specified for the <code>Domain</code> parameter of the <a>CreateDataset</a> operation must match.</p> <p>The <code>Domain</code> and <code>DatasetType</code> that you choose determine the fields that must be present in training data that you import to a dataset. For example, if you choose the <code>RETAIL</code> domain and <code>TARGET_TIME_SERIES</code> as the <code>DatasetType</code>, Amazon Forecast requires that <code>item_id</code>, <code>timestamp</code>, and <code>demand</code> fields are present in your data. For more information, see <a>howitworks-datasets-groups</a>.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
    /// <p><p>The optional metadata that you apply to the dataset group to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50.</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDatasetGroupResponse {
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    #[serde(rename = "DatasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDatasetImportJobRequest {
    /// <p>The location of the training data to import and an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the data. The training data must be stored in an Amazon S3 bucket.</p> <p>If encryption is used, <code>DataSource</code> must include an AWS Key Management Service (KMS) key and the IAM role must allow Amazon Forecast permission to access the key. The KMS key and IAM role must match those specified in the <code>EncryptionConfig</code> parameter of the <a>CreateDataset</a> operation.</p>
    #[serde(rename = "DataSource")]
    pub data_source: DataSource,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Forecast dataset that you want to import data to.</p>
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: String,
    /// <p>The name for the dataset import job. We recommend including the current timestamp in the name, for example, <code>20190721DatasetImport</code>. This can help you avoid getting a <code>ResourceAlreadyExistsException</code> exception.</p>
    #[serde(rename = "DatasetImportJobName")]
    pub dataset_import_job_name: String,
    /// <p><p>The optional metadata that you apply to the dataset import job to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50.</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The format of timestamps in the dataset. The format that you specify depends on the <code>DataFrequency</code> specified when the dataset was created. The following formats are supported</p> <ul> <li> <p>"yyyy-MM-dd"</p> <p>For the following data frequencies: Y, M, W, and D</p> </li> <li> <p>"yyyy-MM-dd HH:mm:ss"</p> <p>For the following data frequencies: H, 30min, 15min, and 1min; and optionally, for: Y, M, W, and D</p> </li> </ul> <p>If the format isn't specified, Amazon Forecast expects the format to be "yyyy-MM-dd HH:mm:ss".</p>
    #[serde(rename = "TimestampFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDatasetImportJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the dataset import job.</p>
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDatasetRequest {
    /// <p>The frequency of data collection. This parameter is required for RELATED_TIME_SERIES datasets.</p> <p>Valid intervals are Y (Year), M (Month), W (Week), D (Day), H (Hour), 30min (30 minutes), 15min (15 minutes), 10min (10 minutes), 5min (5 minutes), and 1min (1 minute). For example, "D" indicates every day and "15min" indicates every 15 minutes.</p>
    #[serde(rename = "DataFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_frequency: Option<String>,
    /// <p>A name for the dataset.</p>
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// <p>The dataset type. Valid values depend on the chosen <code>Domain</code>.</p>
    #[serde(rename = "DatasetType")]
    pub dataset_type: String,
    /// <p>The domain associated with the dataset. When you add a dataset to a dataset group, this value and the value specified for the <code>Domain</code> parameter of the <a>CreateDatasetGroup</a> operation must match.</p> <p>The <code>Domain</code> and <code>DatasetType</code> that you choose determine the fields that must be present in the training data that you import to the dataset. For example, if you choose the <code>RETAIL</code> domain and <code>TARGET_TIME_SERIES</code> as the <code>DatasetType</code>, Amazon Forecast requires <code>item_id</code>, <code>timestamp</code>, and <code>demand</code> fields to be present in your data. For more information, see <a>howitworks-datasets-groups</a>.</p>
    #[serde(rename = "Domain")]
    pub domain: String,
    /// <p>An AWS Key Management Service (KMS) key and the AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the key.</p>
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    /// <p>The schema for the dataset. The schema attributes and their order must match the fields in your data. The dataset <code>Domain</code> and <code>DatasetType</code> that you choose determine the minimum required fields in your training data. For information about the required fields for a specific dataset domain and type, see <a>howitworks-domains-ds-types</a>.</p>
    #[serde(rename = "Schema")]
    pub schema: Schema,
    /// <p><p>The optional metadata that you apply to the dataset to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50.</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDatasetResponse {
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "DatasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateForecastExportJobRequest {
    /// <p>The location where you want to save the forecast and an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3 bucket.</p> <p>If encryption is used, <code>Destination</code> must include an AWS Key Management Service (KMS) key. The IAM role must allow Amazon Forecast permission to access the key.</p>
    #[serde(rename = "Destination")]
    pub destination: DataDestination,
    /// <p>The Amazon Resource Name (ARN) of the forecast that you want to export.</p>
    #[serde(rename = "ForecastArn")]
    pub forecast_arn: String,
    /// <p>The name for the forecast export job.</p>
    #[serde(rename = "ForecastExportJobName")]
    pub forecast_export_job_name: String,
    /// <p><p>The optional metadata that you apply to the forecast export job to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50.</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateForecastExportJobResponse {
    /// <p>The Amazon Resource Name (ARN) of the export job.</p>
    #[serde(rename = "ForecastExportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateForecastRequest {
    /// <p>A name for the forecast.</p>
    #[serde(rename = "ForecastName")]
    pub forecast_name: String,
    /// <p>The quantiles at which probabilistic forecasts are generated. <b>You can currently specify up to 5 quantiles per forecast</b>. Accepted values include <code>0.01 to 0.99</code> (increments of .01 only) and <code>mean</code>. The mean forecast is different from the median (0.50) when the distribution is not symmetric (for example, Beta and Negative Binomial). The default value is <code>["0.1", "0.5", "0.9"]</code>.</p>
    #[serde(rename = "ForecastTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the predictor to use to generate the forecast.</p>
    #[serde(rename = "PredictorArn")]
    pub predictor_arn: String,
    /// <p><p>The optional metadata that you apply to the forecast to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50.</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateForecastResponse {
    /// <p>The Amazon Resource Name (ARN) of the forecast.</p>
    #[serde(rename = "ForecastArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePredictorRequest {
    /// <p><p>The Amazon Resource Name (ARN) of the algorithm to use for model training. Required if <code>PerformAutoML</code> is not set to <code>true</code>.</p> <p class="title"> <b>Supported algorithms:</b> </p> <ul> <li> <p> <code>arn:aws:forecast:::algorithm/ARIMA</code> </p> </li> <li> <p> <code>arn:aws:forecast:::algorithm/Deep<em>AR</em>Plus</code> </p> <p>Supports hyperparameter optimization (HPO)</p> </li> <li> <p> <code>arn:aws:forecast:::algorithm/ETS</code> </p> </li> <li> <p> <code>arn:aws:forecast:::algorithm/NPTS</code> </p> </li> <li> <p> <code>arn:aws:forecast:::algorithm/Prophet</code> </p> </li> </ul></p>
    #[serde(rename = "AlgorithmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    /// <p>An AWS Key Management Service (KMS) key and the AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the key.</p>
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    /// <p>Used to override the default evaluation parameters of the specified algorithm. Amazon Forecast evaluates a predictor by splitting a dataset into training data and testing data. The evaluation parameters define how to perform the split and the number of iterations.</p>
    #[serde(rename = "EvaluationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_parameters: Option<EvaluationParameters>,
    /// <p>The featurization configuration.</p>
    #[serde(rename = "FeaturizationConfig")]
    pub featurization_config: FeaturizationConfig,
    /// <p>Specifies the number of time-steps that the model is trained to predict. The forecast horizon is also called the prediction length.</p> <p>For example, if you configure a dataset for daily data collection (using the <code>DataFrequency</code> parameter of the <a>CreateDataset</a> operation) and set the forecast horizon to 10, the model returns predictions for 10 days.</p> <p>The maximum forecast horizon is the lesser of 500 time-steps or 1/3 of the TARGET_TIME_SERIES dataset length.</p>
    #[serde(rename = "ForecastHorizon")]
    pub forecast_horizon: i64,
    /// <p>Provides hyperparameter override values for the algorithm. If you don't provide this parameter, Amazon Forecast uses default values. The individual algorithms specify which hyperparameters support hyperparameter optimization (HPO). For more information, see <a>aws-forecast-choosing-recipes</a>.</p> <p>If you included the <code>HPOConfig</code> object, you must set <code>PerformHPO</code> to true.</p>
    #[serde(rename = "HPOConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpo_config: Option<HyperParameterTuningJobConfig>,
    /// <p>Describes the dataset group that contains the data to use to train the predictor.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>Whether to perform AutoML. When Amazon Forecast performs AutoML, it evaluates the algorithms it provides and chooses the best algorithm and configuration for your training dataset.</p> <p>The default value is <code>false</code>. In this case, you are required to specify an algorithm.</p> <p>Set <code>PerformAutoML</code> to <code>true</code> to have Amazon Forecast perform AutoML. This is a good option if you aren't sure which algorithm is suitable for your training data. In this case, <code>PerformHPO</code> must be false.</p>
    #[serde(rename = "PerformAutoML")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_ml: Option<bool>,
    /// <p><p>Whether to perform hyperparameter optimization (HPO). HPO finds optimal hyperparameter values for your training data. The process of performing HPO is known as running a hyperparameter tuning job.</p> <p>The default value is <code>false</code>. In this case, Amazon Forecast uses default hyperparameter values from the chosen algorithm.</p> <p>To override the default values, set <code>PerformHPO</code> to <code>true</code> and, optionally, supply the <a>HyperParameterTuningJobConfig</a> object. The tuning job specifies a metric to optimize, which hyperparameters participate in tuning, and the valid range for each tunable hyperparameter. In this case, you are required to specify an algorithm and <code>PerformAutoML</code> must be false.</p> <p>The following algorithm supports HPO:</p> <ul> <li> <p>DeepAR+</p> </li> </ul></p>
    #[serde(rename = "PerformHPO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_hpo: Option<bool>,
    /// <p>A name for the predictor.</p>
    #[serde(rename = "PredictorName")]
    pub predictor_name: String,
    /// <p><p>The optional metadata that you apply to the predictor to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50.</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The hyperparameters to override for model training. The hyperparameters that you can override are listed in the individual algorithms. For the list of supported algorithms, see <a>aws-forecast-choosing-recipes</a>.</p>
    #[serde(rename = "TrainingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_parameters: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePredictorResponse {
    /// <p>The Amazon Resource Name (ARN) of the predictor.</p>
    #[serde(rename = "PredictorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
}

/// <p>The destination for an exported forecast, an AWS Identity and Access Management (IAM) role that allows Amazon Forecast to access the location and, optionally, an AWS Key Management Service (KMS) key. This object is submitted in the <a>CreateForecastExportJob</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataDestination {
    /// <p>The path to an Amazon Simple Storage Service (Amazon S3) bucket along with the credentials to access the bucket.</p>
    #[serde(rename = "S3Config")]
    pub s3_config: S3Config,
}

/// <p>The source of your training data, an AWS Identity and Access Management (IAM) role that allows Amazon Forecast to access the data and, optionally, an AWS Key Management Service (KMS) key. This object is submitted in the <a>CreateDatasetImportJob</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    /// <p>The path to the training data stored in an Amazon Simple Storage Service (Amazon S3) bucket along with the credentials to access the data.</p>
    #[serde(rename = "S3Config")]
    pub s3_config: S3Config,
}

/// <p>Provides a summary of the dataset group properties used in the <a>ListDatasetGroups</a> operation. To get the complete set of properties, call the <a>DescribeDatasetGroup</a> operation, and provide the <code>DatasetGroupArn</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetGroupSummary {
    /// <p>When the dataset group was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    #[serde(rename = "DatasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The name of the dataset group.</p>
    #[serde(rename = "DatasetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_name: Option<String>,
    /// <p>When the dataset group was created or last updated from a call to the <a>UpdateDatasetGroup</a> operation. While the dataset group is being updated, <code>LastModificationTime</code> is the current time of the <code>ListDatasetGroups</code> call.</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
}

/// <p>Provides a summary of the dataset import job properties used in the <a>ListDatasetImportJobs</a> operation. To get the complete set of properties, call the <a>DescribeDatasetImportJob</a> operation, and provide the <code>DatasetImportJobArn</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetImportJobSummary {
    /// <p>When the dataset import job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The location of the training data to import and an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the data. The training data must be stored in an Amazon S3 bucket.</p> <p>If encryption is used, <code>DataSource</code> includes an AWS Key Management Service (KMS) key.</p>
    #[serde(rename = "DataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    /// <p>The Amazon Resource Name (ARN) of the dataset import job.</p>
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    /// <p>The name of the dataset import job.</p>
    #[serde(rename = "DatasetImportJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_name: Option<String>,
    /// <p><p>The last time that the dataset was modified. The time depends on the status of the job, as follows:</p> <ul> <li> <p> <code>CREATE<em>PENDING</code> - The same time as <code>CreationTime</code>.</p> </li> <li> <p> <code>CREATE</em>IN<em>PROGRESS</code> - The current timestamp.</p> </li> <li> <p> <code>ACTIVE</code> or <code>CREATE</em>FAILED</code> - When the job finished or failed.</p> </li> </ul></p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>If an error occurred, an informational message about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The status of the dataset import job. The status is reflected in the status of the dataset. For example, when the import job status is <code>CREATE<em>IN</em>PROGRESS</code>, the status of the dataset is <code>UPDATE<em>IN</em>PROGRESS</code>. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides a summary of the dataset properties used in the <a>ListDatasets</a> operation. To get the complete set of properties, call the <a>DescribeDataset</a> operation, and provide the <code>DatasetArn</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DatasetSummary {
    /// <p>When the dataset was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "DatasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p>The name of the dataset.</p>
    #[serde(rename = "DatasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    /// <p>The dataset type.</p>
    #[serde(rename = "DatasetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    /// <p>The domain associated with the dataset.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>When you create a dataset, <code>LastModificationTime</code> is the same as <code>CreationTime</code>. While data is being imported to the dataset, <code>LastModificationTime</code> is the current time of the <code>ListDatasets</code> call. After a <a>CreateDatasetImportJob</a> operation has finished, <code>LastModificationTime</code> is when the import job completed or failed.</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDatasetGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset group to delete.</p>
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDatasetImportJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset import job to delete.</p>
    #[serde(rename = "DatasetImportJobArn")]
    pub dataset_import_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDatasetRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset to delete.</p>
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteForecastExportJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the forecast export job to delete.</p>
    #[serde(rename = "ForecastExportJobArn")]
    pub forecast_export_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteForecastRequest {
    /// <p>The Amazon Resource Name (ARN) of the forecast to delete.</p>
    #[serde(rename = "ForecastArn")]
    pub forecast_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePredictorRequest {
    /// <p>The Amazon Resource Name (ARN) of the predictor to delete.</p>
    #[serde(rename = "PredictorArn")]
    pub predictor_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDatasetGroupRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDatasetGroupResponse {
    /// <p>When the dataset group was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>An array of Amazon Resource Names (ARNs) of the datasets contained in the dataset group.</p>
    #[serde(rename = "DatasetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arns: Option<Vec<String>>,
    /// <p>The ARN of the dataset group.</p>
    #[serde(rename = "DatasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The name of the dataset group.</p>
    #[serde(rename = "DatasetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_name: Option<String>,
    /// <p>The domain associated with the dataset group.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>When the dataset group was created or last updated from a call to the <a>UpdateDatasetGroup</a> operation. While the dataset group is being updated, <code>LastModificationTime</code> is the current time of the <code>DescribeDatasetGroup</code> call.</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p><p>The status of the dataset group. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> <li> <p> <code>UPDATE<em>PENDING</code>, <code>UPDATE</em>IN<em>PROGRESS</code>, <code>UPDATE</em>FAILED</code> </p> </li> </ul> <p>The <code>UPDATE</code> states apply when you call the <a>UpdateDatasetGroup</a> operation.</p> <note> <p>The <code>Status</code> of the dataset group must be <code>ACTIVE</code> before you can use the dataset group to create a predictor.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDatasetImportJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset import job.</p>
    #[serde(rename = "DatasetImportJobArn")]
    pub dataset_import_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDatasetImportJobResponse {
    /// <p>When the dataset import job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The size of the dataset in gigabytes (GB) after the import job has finished.</p>
    #[serde(rename = "DataSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_size: Option<f64>,
    /// <p>The location of the training data to import and an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the data.</p> <p>If encryption is used, <code>DataSource</code> includes an AWS Key Management Service (KMS) key.</p>
    #[serde(rename = "DataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    /// <p>The Amazon Resource Name (ARN) of the dataset that the training data was imported to.</p>
    #[serde(rename = "DatasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p>The ARN of the dataset import job.</p>
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    /// <p>The name of the dataset import job.</p>
    #[serde(rename = "DatasetImportJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_name: Option<String>,
    /// <p>Statistical information about each field in the input data.</p>
    #[serde(rename = "FieldStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_statistics: Option<::std::collections::HashMap<String, Statistics>>,
    /// <p><p>The last time that the dataset was modified. The time depends on the status of the job, as follows:</p> <ul> <li> <p> <code>CREATE<em>PENDING</code> - The same time as <code>CreationTime</code>.</p> </li> <li> <p> <code>CREATE</em>IN<em>PROGRESS</code> - The current timestamp.</p> </li> <li> <p> <code>ACTIVE</code> or <code>CREATE</em>FAILED</code> - When the job finished or failed.</p> </li> </ul></p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>If an error occurred, an informational message about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The status of the dataset import job. The status is reflected in the status of the dataset. For example, when the import job status is <code>CREATE<em>IN</em>PROGRESS</code>, the status of the dataset is <code>UPDATE<em>IN</em>PROGRESS</code>. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The format of timestamps in the dataset. The format that you specify depends on the <code>DataFrequency</code> specified when the dataset was created. The following formats are supported</p> <ul> <li> <p>&quot;yyyy-MM-dd&quot;</p> <p>For the following data frequencies: Y, M, W, and D</p> </li> <li> <p>&quot;yyyy-MM-dd HH:mm:ss&quot;</p> <p>For the following data frequencies: H, 30min, 15min, and 1min; and optionally, for: Y, M, W, and D</p> </li> </ul></p>
    #[serde(rename = "TimestampFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDatasetRequest {
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDatasetResponse {
    /// <p>When the dataset was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The frequency of data collection.</p> <p>Valid intervals are Y (Year), M (Month), W (Week), D (Day), H (Hour), 30min (30 minutes), 15min (15 minutes), 10min (10 minutes), 5min (5 minutes), and 1min (1 minute). For example, "M" indicates every month and "30min" indicates every 30 minutes.</p>
    #[serde(rename = "DataFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_frequency: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the dataset.</p>
    #[serde(rename = "DatasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p>The name of the dataset.</p>
    #[serde(rename = "DatasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    /// <p>The dataset type.</p>
    #[serde(rename = "DatasetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    /// <p>The domain associated with the dataset.</p>
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>The AWS Key Management Service (KMS) key and the AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the key.</p>
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    /// <p>When you create a dataset, <code>LastModificationTime</code> is the same as <code>CreationTime</code>. While data is being imported to the dataset, <code>LastModificationTime</code> is the current time of the <code>DescribeDataset</code> call. After a <a>CreateDatasetImportJob</a> operation has finished, <code>LastModificationTime</code> is when the import job completed or failed.</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>An array of <code>SchemaAttribute</code> objects that specify the dataset fields. Each <code>SchemaAttribute</code> specifies the name and data type of a field.</p>
    #[serde(rename = "Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// <p><p>The status of the dataset. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> <li> <p> <code>UPDATE<em>PENDING</code>, <code>UPDATE</em>IN<em>PROGRESS</code>, <code>UPDATE</em>FAILED</code> </p> </li> </ul> <p>The <code>UPDATE</code> states apply while data is imported to the dataset from a call to the <a>CreateDatasetImportJob</a> operation and reflect the status of the dataset import job. For example, when the import job status is <code>CREATE<em>IN</em>PROGRESS</code>, the status of the dataset is <code>UPDATE<em>IN</em>PROGRESS</code>.</p> <note> <p>The <code>Status</code> of the dataset must be <code>ACTIVE</code> before you can import training data.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeForecastExportJobRequest {
    /// <p>The Amazon Resource Name (ARN) of the forecast export job.</p>
    #[serde(rename = "ForecastExportJobArn")]
    pub forecast_export_job_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeForecastExportJobResponse {
    /// <p>When the forecast export job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The path to the Amazon Simple Storage Service (Amazon S3) bucket where the forecast is exported.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    /// <p>The Amazon Resource Name (ARN) of the exported forecast.</p>
    #[serde(rename = "ForecastArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    /// <p>The ARN of the forecast export job.</p>
    #[serde(rename = "ForecastExportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_arn: Option<String>,
    /// <p>The name of the forecast export job.</p>
    #[serde(rename = "ForecastExportJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_name: Option<String>,
    /// <p>When the last successful export job finished.</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>If an error occurred, an informational message about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The status of the forecast export job. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> </ul> <note> <p>The <code>Status</code> of the forecast export job must be <code>ACTIVE</code> before you can access the forecast in your S3 bucket.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeForecastRequest {
    /// <p>The Amazon Resource Name (ARN) of the forecast.</p>
    #[serde(rename = "ForecastArn")]
    pub forecast_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeForecastResponse {
    /// <p>When the forecast creation task was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The ARN of the dataset group that provided the data used to train the predictor.</p>
    #[serde(rename = "DatasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The forecast ARN as specified in the request.</p>
    #[serde(rename = "ForecastArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    /// <p>The name of the forecast.</p>
    #[serde(rename = "ForecastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_name: Option<String>,
    /// <p>The quantiles at which probabilistic forecasts were generated.</p>
    #[serde(rename = "ForecastTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    /// <p>Initially, the same as <code>CreationTime</code> (status is <code>CREATE_PENDING</code>). Updated when inference (creating the forecast) starts (status changed to <code>CREATE_IN_PROGRESS</code>), and when inference is complete (status changed to <code>ACTIVE</code>) or fails (status changed to <code>CREATE_FAILED</code>).</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>If an error occurred, an informational message about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The ARN of the predictor used to generate the forecast.</p>
    #[serde(rename = "PredictorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    /// <p><p>The status of the forecast. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> </ul> <note> <p>The <code>Status</code> of the forecast must be <code>ACTIVE</code> before you can query or export the forecast.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePredictorRequest {
    /// <p>The Amazon Resource Name (ARN) of the predictor that you want information about.</p>
    #[serde(rename = "PredictorArn")]
    pub predictor_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePredictorResponse {
    /// <p>The Amazon Resource Name (ARN) of the algorithm used for model training.</p>
    #[serde(rename = "AlgorithmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    /// <p>When <code>PerformAutoML</code> is specified, the ARN of the chosen algorithm.</p>
    #[serde(rename = "AutoMLAlgorithmArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ml_algorithm_arns: Option<Vec<String>>,
    /// <p>When the model training task was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>An array of the ARNs of the dataset import jobs used to import training data for the predictor.</p>
    #[serde(rename = "DatasetImportJobArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arns: Option<Vec<String>>,
    /// <p>An AWS Key Management Service (KMS) key and the AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the key.</p>
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    /// <p>Used to override the default evaluation parameters of the specified algorithm. Amazon Forecast evaluates a predictor by splitting a dataset into training data and testing data. The evaluation parameters define how to perform the split and the number of iterations.</p>
    #[serde(rename = "EvaluationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_parameters: Option<EvaluationParameters>,
    /// <p>The featurization configuration.</p>
    #[serde(rename = "FeaturizationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featurization_config: Option<FeaturizationConfig>,
    /// <p>The number of time-steps of the forecast. The forecast horizon is also called the prediction length.</p>
    #[serde(rename = "ForecastHorizon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_horizon: Option<i64>,
    /// <p>The hyperparameter override values for the algorithm.</p>
    #[serde(rename = "HPOConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpo_config: Option<HyperParameterTuningJobConfig>,
    /// <p>Describes the dataset group that contains the data to use to train the predictor.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>Initially, the same as <code>CreationTime</code> (when the status is <code>CREATE_PENDING</code>). This value is updated when training starts (when the status changes to <code>CREATE_IN_PROGRESS</code>), and when training has completed (when the status changes to <code>ACTIVE</code>) or fails (when the status changes to <code>CREATE_FAILED</code>).</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>If an error occurred, an informational message about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Whether the predictor is set to perform AutoML.</p>
    #[serde(rename = "PerformAutoML")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_ml: Option<bool>,
    /// <p>Whether the predictor is set to perform hyperparameter optimization (HPO).</p>
    #[serde(rename = "PerformHPO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_hpo: Option<bool>,
    /// <p>The ARN of the predictor.</p>
    #[serde(rename = "PredictorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    /// <p>Details on the the status and results of the backtests performed to evaluate the accuracy of the predictor. You specify the number of backtests to perform when you call the operation.</p>
    #[serde(rename = "PredictorExecutionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_execution_details: Option<PredictorExecutionDetails>,
    /// <p>The name of the predictor.</p>
    #[serde(rename = "PredictorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_name: Option<String>,
    /// <p><p>The status of the predictor. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> <li> <p> <code>UPDATE<em>PENDING</code>, <code>UPDATE</em>IN<em>PROGRESS</code>, <code>UPDATE</em>FAILED</code> </p> </li> </ul> <note> <p>The <code>Status</code> of the predictor must be <code>ACTIVE</code> before you can use the predictor to create a forecast.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The default training parameters or overrides selected during model training. If using the AutoML algorithm or if HPO is turned on while using the DeepAR+ algorithms, the optimized values for the chosen hyperparameters are returned. For more information, see <a>aws-forecast-choosing-recipes</a>.</p>
    #[serde(rename = "TrainingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>An AWS Key Management Service (KMS) key and an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the key. You can specify this optional object in the <a>CreateDataset</a> and <a>CreatePredictor</a> requests.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// <p>The Amazon Resource Name (ARN) of the KMS key.</p>
    #[serde(rename = "KMSKeyArn")]
    pub kms_key_arn: String,
    /// <p>The ARN of the IAM role that Amazon Forecast can assume to access the AWS KMS key.</p> <p>Passing a role across AWS accounts is not allowed. If you pass a role that isn't in your account, you get an <code>InvalidInputException</code> error.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

/// <p>Parameters that define how to split a dataset into training data and testing data, and the number of iterations to perform. These parameters are specified in the predefined algorithms but you can override them in the <a>CreatePredictor</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluationParameters {
    /// <p>The point from the end of the dataset where you want to split the data for model training and testing (evaluation). Specify the value as the number of data points. The default is the value of the forecast horizon. <code>BackTestWindowOffset</code> can be used to mimic a past virtual forecast start date. This value must be greater than or equal to the forecast horizon and less than half of the TARGET_TIME_SERIES dataset length.</p> <p> <code>ForecastHorizon</code> &lt;= <code>BackTestWindowOffset</code> &lt; 1/2 * TARGET_TIME_SERIES dataset length</p>
    #[serde(rename = "BackTestWindowOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_test_window_offset: Option<i64>,
    /// <p>The number of times to split the input data. The default is 1. Valid values are 1 through 5.</p>
    #[serde(rename = "NumberOfBacktestWindows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_backtest_windows: Option<i64>,
}

/// <p>The results of evaluating an algorithm. Returned as part of the <a>GetAccuracyMetrics</a> response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EvaluationResult {
    /// <p>The Amazon Resource Name (ARN) of the algorithm that was evaluated.</p>
    #[serde(rename = "AlgorithmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    /// <p>The array of test windows used for evaluating the algorithm. The <code>NumberOfBacktestWindows</code> from the <a>EvaluationParameters</a> object determines the number of windows in the array.</p>
    #[serde(rename = "TestWindows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_windows: Option<Vec<WindowSummary>>,
}

/// <p>Provides featurization (transformation) information for a dataset field. This object is part of the <a>FeaturizationConfig</a> object.</p> <p>For example:</p> <p> <code>{</code> </p> <p> <code>"AttributeName": "demand",</code> </p> <p> <code>FeaturizationPipeline [ {</code> </p> <p> <code>"FeaturizationMethodName": "filling",</code> </p> <p> <code>"FeaturizationMethodParameters": {"aggregation": "avg", "backfill": "nan"}</code> </p> <p> <code>} ]</code> </p> <p> <code>}</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Featurization {
    /// <p>The name of the schema attribute that specifies the data field to be featurized. Amazon Forecast supports the target field of the <code>TARGET_TIME_SERIES</code> and the <code>RELATED_TIME_SERIES</code> datasets. For example, for the <code>RETAIL</code> domain, the target is <code>demand</code>, and for the <code>CUSTOM</code> domain, the target is <code>target_value</code>. For more information, see <a>howitworks-missing-values</a>.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>An array of one <code>FeaturizationMethod</code> object that specifies the feature transformation method.</p>
    #[serde(rename = "FeaturizationPipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featurization_pipeline: Option<Vec<FeaturizationMethod>>,
}

/// <p>In a <a>CreatePredictor</a> operation, the specified algorithm trains a model using the specified dataset group. You can optionally tell the operation to modify data fields prior to training a model. These modifications are referred to as <i>featurization</i>.</p> <p>You define featurization using the <code>FeaturizationConfig</code> object. You specify an array of transformations, one for each field that you want to featurize. You then include the <code>FeaturizationConfig</code> object in your <code>CreatePredictor</code> request. Amazon Forecast applies the featurization to the <code>TARGET_TIME_SERIES</code> and <code>RELATED_TIME_SERIES</code> datasets before model training.</p> <p>You can create multiple featurization configurations. For example, you might call the <code>CreatePredictor</code> operation twice by specifying different featurization configurations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturizationConfig {
    /// <p>An array of featurization (transformation) information for the fields of a dataset.</p>
    #[serde(rename = "Featurizations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featurizations: Option<Vec<Featurization>>,
    /// <p>An array of dimension (field) names that specify how to group the generated forecast.</p> <p>For example, suppose that you are generating a forecast for item sales across all of your stores, and your dataset contains a <code>store_id</code> field. If you want the sales forecast for each item by store, you would specify <code>store_id</code> as the dimension.</p> <p>All forecast dimensions specified in the <code>TARGET_TIME_SERIES</code> dataset don't need to be specified in the <code>CreatePredictor</code> request. All forecast dimensions specified in the <code>RELATED_TIME_SERIES</code> dataset must be specified in the <code>CreatePredictor</code> request.</p>
    #[serde(rename = "ForecastDimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_dimensions: Option<Vec<String>>,
    /// <p>The frequency of predictions in a forecast.</p> <p>Valid intervals are Y (Year), M (Month), W (Week), D (Day), H (Hour), 30min (30 minutes), 15min (15 minutes), 10min (10 minutes), 5min (5 minutes), and 1min (1 minute). For example, "Y" indicates every year and "5min" indicates every five minutes.</p> <p>The frequency must be greater than or equal to the TARGET_TIME_SERIES dataset frequency.</p> <p>When a RELATED_TIME_SERIES dataset is provided, the frequency must be equal to the RELATED_TIME_SERIES dataset frequency.</p>
    #[serde(rename = "ForecastFrequency")]
    pub forecast_frequency: String,
}

/// <p>Provides information about the method that featurizes (transforms) a dataset field. The method is part of the <code>FeaturizationPipeline</code> of the <a>Featurization</a> object. </p> <p>The following is an example of how you specify a <code>FeaturizationMethod</code> object.</p> <p> <code>{</code> </p> <p> <code>"FeaturizationMethodName": "filling",</code> </p> <p> <code>"FeaturizationMethodParameters": {"aggregation": "sum", "middlefill": "zero", "backfill": "zero"}</code> </p> <p> <code>}</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeaturizationMethod {
    /// <p>The name of the method. The "filling" method is the only supported method.</p>
    #[serde(rename = "FeaturizationMethodName")]
    pub featurization_method_name: String,
    /// <p><p>The method parameters (key-value pairs), which are a map of override parameters. Specify these parameters to override the default values. Related Time Series attributes do not accept aggregation parameters.</p> <p>The following list shows the parameters and their valid values for the &quot;filling&quot; featurization method for a <b>Target Time Series</b> dataset. Bold signifies the default value.</p> <ul> <li> <p> <code>aggregation</code>: <b>sum</b>, <code>avg</code>, <code>first</code>, <code>min</code>, <code>max</code> </p> </li> <li> <p> <code>frontfill</code>: <b>none</b> </p> </li> <li> <p> <code>middlefill</code>: <b>zero</b>, <code>nan</code> (not a number), <code>value</code>, <code>median</code>, <code>mean</code>, <code>min</code>, <code>max</code> </p> </li> <li> <p> <code>backfill</code>: <b>zero</b>, <code>nan</code>, <code>value</code>, <code>median</code>, <code>mean</code>, <code>min</code>, <code>max</code> </p> </li> </ul> <p>The following list shows the parameters and their valid values for a <b>Related Time Series</b> featurization method (there are no defaults):</p> <ul> <li> <p> <code>middlefill</code>: <code>zero</code>, <code>value</code>, <code>median</code>, <code>mean</code>, <code>min</code>, <code>max</code> </p> </li> <li> <p> <code>backfill</code>: <code>zero</code>, <code>value</code>, <code>median</code>, <code>mean</code>, <code>min</code>, <code>max</code> </p> </li> <li> <p> <code>futurefill</code>: <code>zero</code>, <code>value</code>, <code>median</code>, <code>mean</code>, <code>min</code>, <code>max</code> </p> </li> </ul></p>
    #[serde(rename = "FeaturizationMethodParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featurization_method_parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Describes a filter for choosing a subset of objects. Each filter consists of a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the objects that match the statement, respectively. The match statement consists of a key and a value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The condition to apply. To include the objects that match the statement, specify <code>IS</code>. To exclude matching objects, specify <code>IS_NOT</code>.</p>
    #[serde(rename = "Condition")]
    pub condition: String,
    /// <p>The name of the parameter to filter on.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value to match.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Provides a summary of the forecast export job properties used in the <a>ListForecastExportJobs</a> operation. To get the complete set of properties, call the <a>DescribeForecastExportJob</a> operation, and provide the listed <code>ForecastExportJobArn</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ForecastExportJobSummary {
    /// <p>When the forecast export job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The path to the Amazon Simple Storage Service (Amazon S3) bucket where the forecast is exported.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    /// <p>The Amazon Resource Name (ARN) of the forecast export job.</p>
    #[serde(rename = "ForecastExportJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_arn: Option<String>,
    /// <p>The name of the forecast export job.</p>
    #[serde(rename = "ForecastExportJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_name: Option<String>,
    /// <p>When the last successful export job finished.</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>If an error occurred, an informational message about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The status of the forecast export job. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> </ul> <note> <p>The <code>Status</code> of the forecast export job must be <code>ACTIVE</code> before you can access the forecast in your S3 bucket.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides a summary of the forecast properties used in the <a>ListForecasts</a> operation. To get the complete set of properties, call the <a>DescribeForecast</a> operation, and provide the <code>ForecastArn</code> that is listed in the summary.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ForecastSummary {
    /// <p>When the forecast creation task was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group that provided the data used to train the predictor.</p>
    #[serde(rename = "DatasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>The ARN of the forecast.</p>
    #[serde(rename = "ForecastArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    /// <p>The name of the forecast.</p>
    #[serde(rename = "ForecastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_name: Option<String>,
    /// <p>Initially, the same as <code>CreationTime</code> (status is <code>CREATE_PENDING</code>). Updated when inference (creating the forecast) starts (status changed to <code>CREATE_IN_PROGRESS</code>), and when inference is complete (status changed to <code>ACTIVE</code>) or fails (status changed to <code>CREATE_FAILED</code>).</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>If an error occurred, an informational message about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The ARN of the predictor used to generate the forecast.</p>
    #[serde(rename = "PredictorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    /// <p><p>The status of the forecast. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> </ul> <note> <p>The <code>Status</code> of the forecast must be <code>ACTIVE</code> before you can query or export the forecast.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccuracyMetricsRequest {
    /// <p>The Amazon Resource Name (ARN) of the predictor to get metrics for.</p>
    #[serde(rename = "PredictorArn")]
    pub predictor_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccuracyMetricsResponse {
    /// <p>An array of results from evaluating the predictor.</p>
    #[serde(rename = "PredictorEvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_evaluation_results: Option<Vec<EvaluationResult>>,
}

/// <p>Configuration information for a hyperparameter tuning job. You specify this object in the <a>CreatePredictor</a> request.</p> <p>A <i>hyperparameter</i> is a parameter that governs the model training process. You set hyperparameters before training starts, unlike model parameters, which are determined during training. The values of the hyperparameters effect which values are chosen for the model parameters.</p> <p>In a <i>hyperparameter tuning job</i>, Amazon Forecast chooses the set of hyperparameter values that optimize a specified metric. Forecast accomplishes this by running many training jobs over a range of hyperparameter values. The optimum set of values depends on the algorithm, the training data, and the specified metric objective.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperParameterTuningJobConfig {
    /// <p>Specifies the ranges of valid values for the hyperparameters.</p>
    #[serde(rename = "ParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_ranges: Option<ParameterRanges>,
}

/// <p>The data used to train a predictor. The data includes a dataset group and any supplementary features. You specify this object in the <a>CreatePredictor</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputDataConfig {
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: String,
    /// <p>An array of supplementary features. The only supported feature is a holiday calendar.</p>
    #[serde(rename = "SupplementaryFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_features: Option<Vec<SupplementaryFeature>>,
}

/// <p>Specifies an integer hyperparameter and it's range of tunable values. This object is part of the <a>ParameterRanges</a> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerParameterRange {
    /// <p>The maximum tunable value of the hyperparameter.</p>
    #[serde(rename = "MaxValue")]
    pub max_value: i64,
    /// <p>The minimum tunable value of the hyperparameter.</p>
    #[serde(rename = "MinValue")]
    pub min_value: i64,
    /// <p>The name of the hyperparameter to tune.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The scale that hyperparameter tuning uses to search the hyperparameter range. Valid values:</p> <dl> <dt>Auto</dt> <dd> <p>Amazon Forecast hyperparameter tuning chooses the best scale for the hyperparameter.</p> </dd> <dt>Linear</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a linear scale.</p> </dd> <dt>Logarithmic</dt> <dd> <p>Hyperparameter tuning searches the values in the hyperparameter range by using a logarithmic scale.</p> <p>Logarithmic scaling works only for ranges that have values greater than 0.</p> </dd> <dt>ReverseLogarithmic</dt> <dd> <p>Not supported for <code>IntegerParameterRange</code>.</p> <p>Reverse logarithmic scaling works only for ranges that are entirely within the range 0 &lt;= x &lt; 1.0.</p> </dd> </dl> <p>For information about choosing a hyperparameter scale, see <a href="http://docs.aws.amazon.com/sagemaker/latest/dg/automatic-model-tuning-define-ranges.html#scaling-type">Hyperparameter Scaling</a>. One of the following values:</p>
    #[serde(rename = "ScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDatasetGroupsRequest {
    /// <p>The number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDatasetGroupsResponse {
    /// <p>An array of objects that summarize each dataset group's properties.</p>
    #[serde(rename = "DatasetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_groups: Option<Vec<DatasetGroupSummary>>,
    /// <p>If the response is truncated, Amazon Forecast returns this token. To retrieve the next set of results, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDatasetImportJobsRequest {
    /// <p>An array of filters. For each filter, you provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the datasets that match the statement from the list, respectively. The match statement consists of a key and a value.</p> <p> <b>Filter properties</b> </p> <ul> <li> <p> <code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>. To include the datasets that match the statement, specify <code>IS</code>. To exclude matching datasets, specify <code>IS_NOT</code>.</p> </li> <li> <p> <code>Key</code> - The name of the parameter to filter on. Valid values are <code>DatasetArn</code> and <code>Status</code>.</p> </li> <li> <p> <code>Value</code> - The value to match.</p> </li> </ul> <p>For example, to list all dataset import jobs whose status is ACTIVE, you specify the following filter:</p> <p> <code>"Filters": [ { "Condition": "IS", "Key": "Status", "Value": "ACTIVE" } ]</code> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDatasetImportJobsResponse {
    /// <p>An array of objects that summarize each dataset import job's properties.</p>
    #[serde(rename = "DatasetImportJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_jobs: Option<Vec<DatasetImportJobSummary>>,
    /// <p>If the response is truncated, Amazon Forecast returns this token. To retrieve the next set of results, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDatasetsRequest {
    /// <p>The number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDatasetsResponse {
    /// <p>An array of objects that summarize each dataset's properties.</p>
    #[serde(rename = "Datasets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<DatasetSummary>>,
    /// <p>If the response is truncated, Amazon Forecast returns this token. To retrieve the next set of results, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListForecastExportJobsRequest {
    /// <p>An array of filters. For each filter, you provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the forecast export jobs that match the statement from the list, respectively. The match statement consists of a key and a value.</p> <p> <b>Filter properties</b> </p> <ul> <li> <p> <code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>. To include the forecast export jobs that match the statement, specify <code>IS</code>. To exclude matching forecast export jobs, specify <code>IS_NOT</code>.</p> </li> <li> <p> <code>Key</code> - The name of the parameter to filter on. Valid values are <code>ForecastArn</code> and <code>Status</code>.</p> </li> <li> <p> <code>Value</code> - The value to match.</p> </li> </ul> <p>For example, to list all jobs that export a forecast named <i>electricityforecast</i>, specify the following filter:</p> <p> <code>"Filters": [ { "Condition": "IS", "Key": "ForecastArn", "Value": "arn:aws:forecast:us-west-2:&lt;acct-id&gt;:forecast/electricityforecast" } ]</code> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListForecastExportJobsResponse {
    /// <p>An array of objects that summarize each export job's properties.</p>
    #[serde(rename = "ForecastExportJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_jobs: Option<Vec<ForecastExportJobSummary>>,
    /// <p>If the response is truncated, Amazon Forecast returns this token. To retrieve the next set of results, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListForecastsRequest {
    /// <p>An array of filters. For each filter, you provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the forecasts that match the statement from the list, respectively. The match statement consists of a key and a value.</p> <p> <b>Filter properties</b> </p> <ul> <li> <p> <code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>. To include the forecasts that match the statement, specify <code>IS</code>. To exclude matching forecasts, specify <code>IS_NOT</code>.</p> </li> <li> <p> <code>Key</code> - The name of the parameter to filter on. Valid values are <code>DatasetGroupArn</code>, <code>PredictorArn</code>, and <code>Status</code>.</p> </li> <li> <p> <code>Value</code> - The value to match.</p> </li> </ul> <p>For example, to list all forecasts whose status is not ACTIVE, you would specify:</p> <p> <code>"Filters": [ { "Condition": "IS_NOT", "Key": "Status", "Value": "ACTIVE" } ]</code> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListForecastsResponse {
    /// <p>An array of objects that summarize each forecast's properties.</p>
    #[serde(rename = "Forecasts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasts: Option<Vec<ForecastSummary>>,
    /// <p>If the response is truncated, Amazon Forecast returns this token. To retrieve the next set of results, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPredictorsRequest {
    /// <p>An array of filters. For each filter, you provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the predictors that match the statement from the list, respectively. The match statement consists of a key and a value.</p> <p> <b>Filter properties</b> </p> <ul> <li> <p> <code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>. To include the predictors that match the statement, specify <code>IS</code>. To exclude matching predictors, specify <code>IS_NOT</code>.</p> </li> <li> <p> <code>Key</code> - The name of the parameter to filter on. Valid values are <code>DatasetGroupArn</code> and <code>Status</code>.</p> </li> <li> <p> <code>Value</code> - The value to match.</p> </li> </ul> <p>For example, to list all predictors whose status is ACTIVE, you would specify:</p> <p> <code>"Filters": [ { "Condition": "IS", "Key": "Status", "Value": "ACTIVE" } ]</code> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The number of items to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPredictorsResponse {
    /// <p>If the response is truncated, Amazon Forecast returns this token. To retrieve the next set of results, use the token in the next request.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of objects that summarize each predictor's properties.</p>
    #[serde(rename = "Predictors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictors: Option<Vec<PredictorSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are Forecast dataset groups, datasets, dataset import jobs, predictors, forecasts, and forecast export jobs.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Provides metrics that are used to evaluate the performance of a predictor. This object is part of the <a>WindowSummary</a> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Metrics {
    /// <p>The root mean square error (RMSE).</p>
    #[serde(rename = "RMSE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmse: Option<f64>,
    /// <p>An array of weighted quantile losses. Quantiles divide a probability distribution into regions of equal probability. The distribution in this case is the loss function.</p>
    #[serde(rename = "WeightedQuantileLosses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_quantile_losses: Option<Vec<WeightedQuantileLoss>>,
}

/// <p>Specifies the categorical, continuous, and integer hyperparameters, and their ranges of tunable values. The range of tunable values determines which values that a hyperparameter tuning job can choose for the specified hyperparameter. This object is part of the <a>HyperParameterTuningJobConfig</a> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterRanges {
    /// <p>Specifies the tunable range for each categorical hyperparameter.</p>
    #[serde(rename = "CategoricalParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_parameter_ranges: Option<Vec<CategoricalParameterRange>>,
    /// <p>Specifies the tunable range for each continuous hyperparameter.</p>
    #[serde(rename = "ContinuousParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_parameter_ranges: Option<Vec<ContinuousParameterRange>>,
    /// <p>Specifies the tunable range for each integer hyperparameter.</p>
    #[serde(rename = "IntegerParameterRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_parameter_ranges: Option<Vec<IntegerParameterRange>>,
}

/// <p>The algorithm used to perform a backtest and the status of those tests.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PredictorExecution {
    /// <p>The ARN of the algorithm used to test the predictor.</p>
    #[serde(rename = "AlgorithmArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    /// <p>An array of test windows used to evaluate the algorithm. The <code>NumberOfBacktestWindows</code> from the object determines the number of windows in the array.</p>
    #[serde(rename = "TestWindows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_windows: Option<Vec<TestWindowSummary>>,
}

/// <p>Contains details on the backtests performed to evaluate the accuracy of the predictor. The tests are returned in descending order of accuracy, with the most accurate backtest appearing first. You specify the number of backtests to perform when you call the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PredictorExecutionDetails {
    /// <p>An array of the backtests performed to evaluate the accuracy of the predictor against a particular algorithm. The <code>NumberOfBacktestWindows</code> from the object determines the number of windows in the array.</p>
    #[serde(rename = "PredictorExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_executions: Option<Vec<PredictorExecution>>,
}

/// <p>Provides a summary of the predictor properties that are used in the <a>ListPredictors</a> operation. To get the complete set of properties, call the <a>DescribePredictor</a> operation, and provide the listed <code>PredictorArn</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PredictorSummary {
    /// <p>When the model training task was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the dataset group that contains the data used to train the predictor.</p>
    #[serde(rename = "DatasetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    /// <p>Initially, the same as <code>CreationTime</code> (status is <code>CREATE_PENDING</code>). Updated when training starts (status changed to <code>CREATE_IN_PROGRESS</code>), and when training is complete (status changed to <code>ACTIVE</code>) or fails (status changed to <code>CREATE_FAILED</code>).</p>
    #[serde(rename = "LastModificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    /// <p>If an error occurred, an informational message about the error.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The ARN of the predictor.</p>
    #[serde(rename = "PredictorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    /// <p>The name of the predictor.</p>
    #[serde(rename = "PredictorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_name: Option<String>,
    /// <p><p>The status of the predictor. States include:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>PENDING</code>, <code>CREATE</em>IN<em>PROGRESS</code>, <code>CREATE</em>FAILED</code> </p> </li> <li> <p> <code>DELETE<em>PENDING</code>, <code>DELETE</em>IN<em>PROGRESS</code>, <code>DELETE</em>FAILED</code> </p> </li> <li> <p> <code>UPDATE<em>PENDING</code>, <code>UPDATE</em>IN<em>PROGRESS</code>, <code>UPDATE</em>FAILED</code> </p> </li> </ul> <note> <p>The <code>Status</code> of the predictor must be <code>ACTIVE</code> before you can use the predictor to create a forecast.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The path to the file(s) in an Amazon Simple Storage Service (Amazon S3) bucket, and an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the file(s). Optionally, includes an AWS Key Management Service (KMS) key. This object is part of the <a>DataSource</a> object that is submitted in the <a>CreateDatasetImportJob</a> request, and part of the <a>DataDestination</a> object that is submitted in the <a>CreateForecastExportJob</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Config {
    /// <p>The Amazon Resource Name (ARN) of an AWS Key Management Service (KMS) key.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The path to an Amazon Simple Storage Service (Amazon S3) bucket or file(s) in an Amazon S3 bucket.</p>
    #[serde(rename = "Path")]
    pub path: String,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the Amazon S3 bucket or files. If you provide a value for the <code>KMSKeyArn</code> key, the role must allow access to the key.</p> <p>Passing a role across AWS accounts is not allowed. If you pass a role that isn't in your account, you get an <code>InvalidInputException</code> error.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

/// <p>Defines the fields of a dataset. You specify this object in the <a>CreateDataset</a> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    /// <p>An array of attributes specifying the name and type of each field in a dataset.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<SchemaAttribute>>,
}

/// <p>An attribute of a schema, which defines a dataset field. A schema attribute is required for every field in a dataset. The <a>Schema</a> object contains an array of <code>SchemaAttribute</code> objects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaAttribute {
    /// <p>The name of the dataset field.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The data type of the field.</p>
    #[serde(rename = "AttributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
}

/// <p>Provides statistics for each data field imported into to an Amazon Forecast dataset with the <a>CreateDatasetImportJob</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Statistics {
    /// <p>For a numeric field, the average value in the field.</p>
    #[serde(rename = "Avg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    /// <p>The number of values in the field.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The number of distinct values in the field.</p>
    #[serde(rename = "CountDistinct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_distinct: Option<i64>,
    /// <p>The number of NAN (not a number) values in the field.</p>
    #[serde(rename = "CountNan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_nan: Option<i64>,
    /// <p>The number of null values in the field.</p>
    #[serde(rename = "CountNull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_null: Option<i64>,
    /// <p>For a numeric field, the maximum value in the field.</p>
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    /// <p>For a numeric field, the minimum value in the field.</p>
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    /// <p>For a numeric field, the standard deviation.</p>
    #[serde(rename = "Stddev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stddev: Option<f64>,
}

/// <p><p>Describes a supplementary feature of a dataset group. This object is part of the <a>InputDataConfig</a> object.</p> <p>The only supported feature is a holiday calendar. If you use the calendar, all data in the datasets should belong to the same country as the calendar. For the holiday calendar data, see the <a href="http://jollyday.sourceforge.net/data.html">Jollyday</a> web site.</p> <p>India and Korea&#39;s holidays are not included in the Jollyday library, but both are supported by Amazon Forecast. Their holidays are:</p> <p> <b>&quot;IN&quot; - INDIA</b> </p> <ul> <li> <p> <code>JANUARY 26 - REPUBLIC DAY</code> </p> </li> <li> <p> <code>AUGUST 15 - INDEPENDENCE DAY</code> </p> </li> <li> <p> <code>OCTOBER 2 GANDHI&#39;S BIRTHDAY</code> </p> </li> </ul> <p> <b>&quot;KR&quot; - KOREA</b> </p> <ul> <li> <p> <code>JANUARY 1 - NEW YEAR</code> </p> </li> <li> <p> <code>MARCH 1 - INDEPENDENCE MOVEMENT DAY</code> </p> </li> <li> <p> <code>MAY 5 - CHILDREN&#39;S DAY</code> </p> </li> <li> <p> <code>JUNE 6 - MEMORIAL DAY</code> </p> </li> <li> <p> <code>AUGUST 15 - LIBERATION DAY</code> </p> </li> <li> <p> <code>OCTOBER 3 - NATIONAL FOUNDATION DAY</code> </p> </li> <li> <p> <code>OCTOBER 9 - HANGEUL DAY</code> </p> </li> <li> <p> <code>DECEMBER 25 - CHRISTMAS DAY</code> </p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplementaryFeature {
    /// <p>The name of the feature. This must be "holiday".</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>One of the following 2 letter country codes:</p> <ul> <li> <p>&quot;AR&quot; - ARGENTINA</p> </li> <li> <p>&quot;AT&quot; - AUSTRIA</p> </li> <li> <p>&quot;AU&quot; - AUSTRALIA</p> </li> <li> <p>&quot;BE&quot; - BELGIUM</p> </li> <li> <p>&quot;BR&quot; - BRAZIL</p> </li> <li> <p>&quot;CA&quot; - CANADA</p> </li> <li> <p>&quot;CN&quot; - CHINA</p> </li> <li> <p>&quot;CZ&quot; - CZECH REPUBLIC</p> </li> <li> <p>&quot;DK&quot; - DENMARK</p> </li> <li> <p>&quot;EC&quot; - ECUADOR</p> </li> <li> <p>&quot;FI&quot; - FINLAND</p> </li> <li> <p>&quot;FR&quot; - FRANCE</p> </li> <li> <p>&quot;DE&quot; - GERMANY</p> </li> <li> <p>&quot;HU&quot; - HUNGARY</p> </li> <li> <p>&quot;IE&quot; - IRELAND</p> </li> <li> <p>&quot;IN&quot; - INDIA</p> </li> <li> <p>&quot;IT&quot; - ITALY</p> </li> <li> <p>&quot;JP&quot; - JAPAN</p> </li> <li> <p>&quot;KR&quot; - KOREA</p> </li> <li> <p>&quot;LU&quot; - LUXEMBOURG</p> </li> <li> <p>&quot;MX&quot; - MEXICO</p> </li> <li> <p>&quot;NL&quot; - NETHERLANDS</p> </li> <li> <p>&quot;NO&quot; - NORWAY</p> </li> <li> <p>&quot;PL&quot; - POLAND</p> </li> <li> <p>&quot;PT&quot; - PORTUGAL</p> </li> <li> <p>&quot;RU&quot; - RUSSIA</p> </li> <li> <p>&quot;ZA&quot; - SOUTH AFRICA</p> </li> <li> <p>&quot;ES&quot; - SPAIN</p> </li> <li> <p>&quot;SE&quot; - SWEDEN</p> </li> <li> <p>&quot;CH&quot; - SWITZERLAND</p> </li> <li> <p>&quot;US&quot; - UNITED STATES</p> </li> <li> <p>&quot;UK&quot; - UNITED KINGDOM</p> </li> </ul></p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p><p>The optional metadata that you apply to a resource to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50.</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that makes up a tag. A <code>key</code> is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that makes up a tag. A <code>value</code> acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are Forecast dataset groups, datasets, dataset import jobs, predictors, forecasts, and forecast export jobs.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p><p>The tags to add to the resource. A tag is an array of key-value pairs.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per resource - 50.</p> </li> <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li> <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li> <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li> <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for AWS use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>The status, start time, and end time of a backtest, as well as a failure reason if applicable.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestWindowSummary {
    /// <p>If the test failed, the reason why it failed.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The status of the test. Possible status values are:</p> <ul> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>CREATE<em>IN</em>PROGRESS</code> </p> </li> <li> <p> <code>CREATE_FAILED</code> </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time at which the test ended.</p>
    #[serde(rename = "TestWindowEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_window_end: Option<f64>,
    /// <p>The time at which the test began.</p>
    #[serde(rename = "TestWindowStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_window_start: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are Forecast dataset groups, datasets, dataset import jobs, predictors, forecasts, and forecast exports.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDatasetGroupRequest {
    /// <p>An array of the Amazon Resource Names (ARNs) of the datasets to add to the dataset group.</p>
    #[serde(rename = "DatasetArns")]
    pub dataset_arns: Vec<String>,
    /// <p>The ARN of the dataset group.</p>
    #[serde(rename = "DatasetGroupArn")]
    pub dataset_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDatasetGroupResponse {}

/// <p>The weighted loss value for a quantile. This object is part of the <a>Metrics</a> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WeightedQuantileLoss {
    /// <p>The difference between the predicted value and the actual value over the quantile, weighted (normalized) by dividing by the sum over all quantiles.</p>
    #[serde(rename = "LossValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_value: Option<f64>,
    /// <p>The quantile. Quantiles divide a probability distribution into regions of equal probability. For example, if the distribution was divided into 5 regions of equal probability, the quantiles would be 0.2, 0.4, 0.6, and 0.8.</p>
    #[serde(rename = "Quantile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantile: Option<f64>,
}

/// <p>The metrics for a time range within the evaluation portion of a dataset. This object is part of the <a>EvaluationResult</a> object.</p> <p>The <code>TestWindowStart</code> and <code>TestWindowEnd</code> parameters are determined by the <code>BackTestWindowOffset</code> parameter of the <a>EvaluationParameters</a> object.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WindowSummary {
    /// <p><p>The type of evaluation.</p> <ul> <li> <p> <code>SUMMARY</code> - The average metrics across all windows.</p> </li> <li> <p> <code>COMPUTED</code> - The metrics for the specified window.</p> </li> </ul></p>
    #[serde(rename = "EvaluationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_type: Option<String>,
    /// <p>The number of data points within the window.</p>
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    /// <p>Provides metrics used to evaluate the performance of a predictor.</p>
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    /// <p>The timestamp that defines the end of the window.</p>
    #[serde(rename = "TestWindowEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_window_end: Option<f64>,
    /// <p>The timestamp that defines the start of the window.</p>
    #[serde(rename = "TestWindowStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_window_start: Option<f64>,
}

/// Errors returned by CreateDataset
#[derive(Debug, PartialEq)]
pub enum CreateDatasetError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The limit on the number of resources per account has been exceeded.</p>
    LimitExceeded(String),
    /// <p>There is already a resource with this name. Try again with a different name.</p>
    ResourceAlreadyExists(String),
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
        }
    }
}
impl Error for CreateDatasetError {}
/// Errors returned by CreateDatasetGroup
#[derive(Debug, PartialEq)]
pub enum CreateDatasetGroupError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The limit on the number of resources per account has been exceeded.</p>
    LimitExceeded(String),
    /// <p>There is already a resource with this name. Try again with a different name.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
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
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateDatasetGroupError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDatasetGroupError::ResourceNotFound(err.msg))
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
            CreateDatasetGroupError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateDatasetGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDatasetGroupError {}
/// Errors returned by CreateDatasetImportJob
#[derive(Debug, PartialEq)]
pub enum CreateDatasetImportJobError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The limit on the number of resources per account has been exceeded.</p>
    LimitExceeded(String),
    /// <p>There is already a resource with this name. Try again with a different name.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
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
/// Errors returned by CreateForecast
#[derive(Debug, PartialEq)]
pub enum CreateForecastError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The limit on the number of resources per account has been exceeded.</p>
    LimitExceeded(String),
    /// <p>There is already a resource with this name. Try again with a different name.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl CreateForecastError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateForecastError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateForecastError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateForecastError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateForecastError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateForecastError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateForecastError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateForecastError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateForecastError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateForecastError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateForecastError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateForecastError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateForecastError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateForecastError {}
/// Errors returned by CreateForecastExportJob
#[derive(Debug, PartialEq)]
pub enum CreateForecastExportJobError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The limit on the number of resources per account has been exceeded.</p>
    LimitExceeded(String),
    /// <p>There is already a resource with this name. Try again with a different name.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl CreateForecastExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateForecastExportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreateForecastExportJobError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateForecastExportJobError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateForecastExportJobError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateForecastExportJobError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateForecastExportJobError::ResourceNotFound(
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
impl fmt::Display for CreateForecastExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateForecastExportJobError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateForecastExportJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateForecastExportJobError::ResourceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateForecastExportJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateForecastExportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateForecastExportJobError {}
/// Errors returned by CreatePredictor
#[derive(Debug, PartialEq)]
pub enum CreatePredictorError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The limit on the number of resources per account has been exceeded.</p>
    LimitExceeded(String),
    /// <p>There is already a resource with this name. Try again with a different name.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl CreatePredictorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePredictorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(CreatePredictorError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePredictorError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreatePredictorError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreatePredictorError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreatePredictorError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePredictorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePredictorError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreatePredictorError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreatePredictorError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreatePredictorError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreatePredictorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePredictorError {}
/// Errors returned by DeleteDataset
#[derive(Debug, PartialEq)]
pub enum DeleteDatasetError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
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
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
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
/// Errors returned by DeleteDatasetImportJob
#[derive(Debug, PartialEq)]
pub enum DeleteDatasetImportJobError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl DeleteDatasetImportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatasetImportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDatasetImportJobError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteDatasetImportJobError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDatasetImportJobError::ResourceNotFound(
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
impl fmt::Display for DeleteDatasetImportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDatasetImportJobError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteDatasetImportJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteDatasetImportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDatasetImportJobError {}
/// Errors returned by DeleteForecast
#[derive(Debug, PartialEq)]
pub enum DeleteForecastError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl DeleteForecastError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteForecastError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteForecastError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteForecastError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteForecastError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteForecastError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteForecastError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteForecastError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteForecastError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteForecastError {}
/// Errors returned by DeleteForecastExportJob
#[derive(Debug, PartialEq)]
pub enum DeleteForecastExportJobError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl DeleteForecastExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteForecastExportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteForecastExportJobError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteForecastExportJobError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteForecastExportJobError::ResourceNotFound(
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
impl fmt::Display for DeleteForecastExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteForecastExportJobError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteForecastExportJobError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteForecastExportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteForecastExportJobError {}
/// Errors returned by DeletePredictor
#[derive(Debug, PartialEq)]
pub enum DeletePredictorError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl DeletePredictorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePredictorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DeletePredictorError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeletePredictorError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePredictorError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePredictorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePredictorError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeletePredictorError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeletePredictorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePredictorError {}
/// Errors returned by DescribeDataset
#[derive(Debug, PartialEq)]
pub enum DescribeDatasetError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
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
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
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
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
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
/// Errors returned by DescribeForecast
#[derive(Debug, PartialEq)]
pub enum DescribeForecastError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl DescribeForecastError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeForecastError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeForecastError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeForecastError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeForecastError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeForecastError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeForecastError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeForecastError {}
/// Errors returned by DescribeForecastExportJob
#[derive(Debug, PartialEq)]
pub enum DescribeForecastExportJobError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl DescribeForecastExportJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeForecastExportJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeForecastExportJobError::InvalidInput(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeForecastExportJobError::ResourceNotFound(
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
impl fmt::Display for DescribeForecastExportJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeForecastExportJobError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeForecastExportJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeForecastExportJobError {}
/// Errors returned by DescribePredictor
#[derive(Debug, PartialEq)]
pub enum DescribePredictorError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl DescribePredictorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePredictorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(DescribePredictorError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePredictorError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePredictorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePredictorError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribePredictorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePredictorError {}
/// Errors returned by GetAccuracyMetrics
#[derive(Debug, PartialEq)]
pub enum GetAccuracyMetricsError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl GetAccuracyMetricsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccuracyMetricsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(GetAccuracyMetricsError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(GetAccuracyMetricsError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAccuracyMetricsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAccuracyMetricsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccuracyMetricsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetAccuracyMetricsError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            GetAccuracyMetricsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccuracyMetricsError {}
/// Errors returned by ListDatasetGroups
#[derive(Debug, PartialEq)]
pub enum ListDatasetGroupsError {
    /// <p>The token is not valid. Tokens expire after 24 hours.</p>
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
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The token is not valid. Tokens expire after 24 hours.</p>
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
    /// <p>The token is not valid. Tokens expire after 24 hours.</p>
    InvalidNextToken(String),
}

impl ListDatasetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatasetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            ListDatasetsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDatasetsError {}
/// Errors returned by ListForecastExportJobs
#[derive(Debug, PartialEq)]
pub enum ListForecastExportJobsError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The token is not valid. Tokens expire after 24 hours.</p>
    InvalidNextToken(String),
}

impl ListForecastExportJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListForecastExportJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListForecastExportJobsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListForecastExportJobsError::InvalidNextToken(
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
impl fmt::Display for ListForecastExportJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListForecastExportJobsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListForecastExportJobsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListForecastExportJobsError {}
/// Errors returned by ListForecasts
#[derive(Debug, PartialEq)]
pub enum ListForecastsError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The token is not valid. Tokens expire after 24 hours.</p>
    InvalidNextToken(String),
}

impl ListForecastsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListForecastsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListForecastsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListForecastsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListForecastsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListForecastsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListForecastsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListForecastsError {}
/// Errors returned by ListPredictors
#[derive(Debug, PartialEq)]
pub enum ListPredictorsError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The token is not valid. Tokens expire after 24 hours.</p>
    InvalidNextToken(String),
}

impl ListPredictorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPredictorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListPredictorsError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListPredictorsError::InvalidNextToken(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPredictorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPredictorsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListPredictorsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPredictorsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
            ListTagsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The limit on the number of resources per account has been exceeded.</p>
    LimitExceeded(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
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
            TagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
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
            UntagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDatasetGroup
#[derive(Debug, PartialEq)]
pub enum UpdateDatasetGroupError {
    /// <p>We can't process the request because it includes an invalid value or a value that exceeds the valid range.</p>
    InvalidInput(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find a resource with that Amazon Resource Name (ARN). Check the ARN and try again.</p>
    ResourceNotFound(String),
}

impl UpdateDatasetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDatasetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDatasetGroupError::InvalidInput(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateDatasetGroupError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDatasetGroupError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDatasetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDatasetGroupError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateDatasetGroupError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateDatasetGroupError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDatasetGroupError {}
/// Trait representing the capabilities of the Amazon Forecast Service API. Amazon Forecast Service clients implement this trait.
#[async_trait]
pub trait Forecast {
    /// <p><p>Creates an Amazon Forecast dataset. The information about the dataset that you provide helps Forecast understand how to consume the data for model training. This includes the following:</p> <ul> <li> <p> <i> <code>DataFrequency</code> </i> - How frequently your historical time-series data is collected.</p> </li> <li> <p> <i> <code>Domain</code> </i> and <i> <code>DatasetType</code> </i> - Each dataset has an associated dataset domain and a type within the domain. Amazon Forecast provides a list of predefined domains and types within each domain. For each unique dataset domain and type within the domain, Amazon Forecast requires your data to include a minimum set of predefined fields.</p> </li> <li> <p> <i> <code>Schema</code> </i> - A schema specifies the fields in the dataset, including the field name and data type.</p> </li> </ul> <p>After creating a dataset, you import your training data into it and add the dataset to a dataset group. You use the dataset group to create a predictor. For more information, see <a>howitworks-datasets-groups</a>.</p> <p>To get a list of all your datasets, use the <a>ListDatasets</a> operation.</p> <p>For example Forecast datasets, see the <a href="https://github.com/aws-samples/amazon-forecast-samples">Amazon Forecast Sample GitHub repository</a>.</p> <note> <p>The <code>Status</code> of a dataset must be <code>ACTIVE</code> before you can import training data. Use the <a>DescribeDataset</a> operation to get the status.</p> </note></p>
    async fn create_dataset(
        &self,
        input: CreateDatasetRequest,
    ) -> Result<CreateDatasetResponse, RusotoError<CreateDatasetError>>;

    /// <p><p>Creates a dataset group, which holds a collection of related datasets. You can add datasets to the dataset group when you create the dataset group, or later by using the <a>UpdateDatasetGroup</a> operation.</p> <p>After creating a dataset group and adding datasets, you use the dataset group when you create a predictor. For more information, see <a>howitworks-datasets-groups</a>.</p> <p>To get a list of all your datasets groups, use the <a>ListDatasetGroups</a> operation.</p> <note> <p>The <code>Status</code> of a dataset group must be <code>ACTIVE</code> before you can create use the dataset group to create a predictor. To get the status, use the <a>DescribeDatasetGroup</a> operation.</p> </note></p>
    async fn create_dataset_group(
        &self,
        input: CreateDatasetGroupRequest,
    ) -> Result<CreateDatasetGroupResponse, RusotoError<CreateDatasetGroupError>>;

    /// <p>Imports your training data to an Amazon Forecast dataset. You provide the location of your training data in an Amazon Simple Storage Service (Amazon S3) bucket and the Amazon Resource Name (ARN) of the dataset that you want to import the data to.</p> <p>You must specify a <a>DataSource</a> object that includes an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the data, as Amazon Forecast makes a copy of your data and processes it in an internal AWS system. For more information, see <a>aws-forecast-iam-roles</a>.</p> <p>The training data must be in CSV format. The delimiter must be a comma (,).</p> <p>You can specify the path to a specific CSV file, the S3 bucket, or to a folder in the S3 bucket. For the latter two cases, Amazon Forecast imports all files up to the limit of 10,000 files.</p> <p>Because dataset imports are not aggregated, your most recent dataset import is the one that is used when training a predictor or generating a forecast. Make sure that your most recent dataset import contains all of the data you want to model off of, and not just the new data collected since the previous import.</p> <p>To get a list of all your dataset import jobs, filtered by specified criteria, use the <a>ListDatasetImportJobs</a> operation.</p>
    async fn create_dataset_import_job(
        &self,
        input: CreateDatasetImportJobRequest,
    ) -> Result<CreateDatasetImportJobResponse, RusotoError<CreateDatasetImportJobError>>;

    /// <p><p>Creates a forecast for each item in the <code>TARGET<em>TIME</em>SERIES</code> dataset that was used to train the predictor. This is known as inference. To retrieve the forecast for a single item at low latency, use the operation. To export the complete forecast into your Amazon Simple Storage Service (Amazon S3) bucket, use the <a>CreateForecastExportJob</a> operation.</p> <p>The range of the forecast is determined by the <code>ForecastHorizon</code> value, which you specify in the <a>CreatePredictor</a> request. When you query a forecast, you can request a specific date range within the forecast.</p> <p>To get a list of all your forecasts, use the <a>ListForecasts</a> operation.</p> <note> <p>The forecasts generated by Amazon Forecast are in the same time zone as the dataset that was used to create the predictor.</p> </note> <p>For more information, see <a>howitworks-forecast</a>.</p> <note> <p>The <code>Status</code> of the forecast must be <code>ACTIVE</code> before you can query or export the forecast. Use the <a>DescribeForecast</a> operation to get the status.</p> </note></p>
    async fn create_forecast(
        &self,
        input: CreateForecastRequest,
    ) -> Result<CreateForecastResponse, RusotoError<CreateForecastError>>;

    /// <p><p>Exports a forecast created by the <a>CreateForecast</a> operation to your Amazon Simple Storage Service (Amazon S3) bucket. The forecast file name will match the following conventions:</p> <p>&lt;ForecastExportJobName&gt;<em>&lt;ExportTimestamp&gt;</em>&lt;PartNumber&gt;</p> <p>where the &lt;ExportTimestamp&gt; component is in Java SimpleDateFormat (yyyy-MM-ddTHH-mm-ssZ).</p> <p>You must specify a <a>DataDestination</a> object that includes an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the Amazon S3 bucket. For more information, see <a>aws-forecast-iam-roles</a>.</p> <p>For more information, see <a>howitworks-forecast</a>.</p> <p>To get a list of all your forecast export jobs, use the <a>ListForecastExportJobs</a> operation.</p> <note> <p>The <code>Status</code> of the forecast export job must be <code>ACTIVE</code> before you can access the forecast in your Amazon S3 bucket. To get the status, use the <a>DescribeForecastExportJob</a> operation.</p> </note></p>
    async fn create_forecast_export_job(
        &self,
        input: CreateForecastExportJobRequest,
    ) -> Result<CreateForecastExportJobResponse, RusotoError<CreateForecastExportJobError>>;

    /// <p><p>Creates an Amazon Forecast predictor.</p> <p>In the request, you provide a dataset group and either specify an algorithm or let Amazon Forecast choose the algorithm for you using AutoML. If you specify an algorithm, you also can override algorithm-specific hyperparameters.</p> <p>Amazon Forecast uses the chosen algorithm to train a model using the latest version of the datasets in the specified dataset group. The result is called a predictor. You then generate a forecast using the <a>CreateForecast</a> operation.</p> <p>After training a model, the <code>CreatePredictor</code> operation also evaluates it. To see the evaluation metrics, use the <a>GetAccuracyMetrics</a> operation. Always review the evaluation metrics before deciding to use the predictor to generate a forecast.</p> <p>Optionally, you can specify a featurization configuration to fill and aggregate the data fields in the <code>TARGET<em>TIME</em>SERIES</code> dataset to improve model training. For more information, see <a>FeaturizationConfig</a>.</p> <p>For RELATED<em>TIME</em>SERIES datasets, <code>CreatePredictor</code> verifies that the <code>DataFrequency</code> specified when the dataset was created matches the <code>ForecastFrequency</code>. TARGET<em>TIME</em>SERIES datasets don&#39;t have this restriction. Amazon Forecast also verifies the delimiter and timestamp format. For more information, see <a>howitworks-datasets-groups</a>.</p> <p> <b>AutoML</b> </p> <p>If you want Amazon Forecast to evaluate each algorithm and choose the one that minimizes the <code>objective function</code>, set <code>PerformAutoML</code> to <code>true</code>. The <code>objective function</code> is defined as the mean of the weighted p10, p50, and p90 quantile losses. For more information, see <a>EvaluationResult</a>.</p> <p>When AutoML is enabled, the following properties are disallowed:</p> <ul> <li> <p> <code>AlgorithmArn</code> </p> </li> <li> <p> <code>HPOConfig</code> </p> </li> <li> <p> <code>PerformHPO</code> </p> </li> <li> <p> <code>TrainingParameters</code> </p> </li> </ul> <p>To get a list of all of your predictors, use the <a>ListPredictors</a> operation.</p> <note> <p>Before you can use the predictor to create a forecast, the <code>Status</code> of the predictor must be <code>ACTIVE</code>, signifying that training has completed. To get the status, use the <a>DescribePredictor</a> operation.</p> </note></p>
    async fn create_predictor(
        &self,
        input: CreatePredictorRequest,
    ) -> Result<CreatePredictorResponse, RusotoError<CreatePredictorError>>;

    /// <p><p>Deletes an Amazon Forecast dataset that was created using the <a>CreateDataset</a> operation. You can only delete datasets that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status use the <a>DescribeDataset</a> operation.</p> <note> <p>Forecast does not automatically update any dataset groups that contain the deleted dataset. In order to update the dataset group, use the operation, omitting the deleted dataset&#39;s ARN.</p> </note></p>
    async fn delete_dataset(
        &self,
        input: DeleteDatasetRequest,
    ) -> Result<(), RusotoError<DeleteDatasetError>>;

    /// <p>Deletes a dataset group created using the <a>CreateDatasetGroup</a> operation. You can only delete dataset groups that have a status of <code>ACTIVE</code>, <code>CREATE_FAILED</code>, or <code>UPDATE_FAILED</code>. To get the status, use the <a>DescribeDatasetGroup</a> operation.</p> <p>This operation deletes only the dataset group, not the datasets in the group.</p>
    async fn delete_dataset_group(
        &self,
        input: DeleteDatasetGroupRequest,
    ) -> Result<(), RusotoError<DeleteDatasetGroupError>>;

    /// <p>Deletes a dataset import job created using the <a>CreateDatasetImportJob</a> operation. You can delete only dataset import jobs that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status, use the <a>DescribeDatasetImportJob</a> operation.</p>
    async fn delete_dataset_import_job(
        &self,
        input: DeleteDatasetImportJobRequest,
    ) -> Result<(), RusotoError<DeleteDatasetImportJobError>>;

    /// <p>Deletes a forecast created using the <a>CreateForecast</a> operation. You can delete only forecasts that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status, use the <a>DescribeForecast</a> operation.</p> <p>You can't delete a forecast while it is being exported. After a forecast is deleted, you can no longer query the forecast.</p>
    async fn delete_forecast(
        &self,
        input: DeleteForecastRequest,
    ) -> Result<(), RusotoError<DeleteForecastError>>;

    /// <p>Deletes a forecast export job created using the <a>CreateForecastExportJob</a> operation. You can delete only export jobs that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status, use the <a>DescribeForecastExportJob</a> operation.</p>
    async fn delete_forecast_export_job(
        &self,
        input: DeleteForecastExportJobRequest,
    ) -> Result<(), RusotoError<DeleteForecastExportJobError>>;

    /// <p>Deletes a predictor created using the <a>CreatePredictor</a> operation. You can delete only predictor that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status, use the <a>DescribePredictor</a> operation.</p>
    async fn delete_predictor(
        &self,
        input: DeletePredictorRequest,
    ) -> Result<(), RusotoError<DeletePredictorError>>;

    /// <p><p>Describes an Amazon Forecast dataset created using the <a>CreateDataset</a> operation.</p> <p>In addition to listing the parameters specified in the <code>CreateDataset</code> request, this operation includes the following dataset properties:</p> <ul> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> </ul></p>
    async fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> Result<DescribeDatasetResponse, RusotoError<DescribeDatasetError>>;

    /// <p><p>Describes a dataset group created using the <a>CreateDatasetGroup</a> operation.</p> <p>In addition to listing the parameters provided in the <code>CreateDatasetGroup</code> request, this operation includes the following properties:</p> <ul> <li> <p> <code>DatasetArns</code> - The datasets belonging to the group.</p> </li> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> </ul></p>
    async fn describe_dataset_group(
        &self,
        input: DescribeDatasetGroupRequest,
    ) -> Result<DescribeDatasetGroupResponse, RusotoError<DescribeDatasetGroupError>>;

    /// <p><p>Describes a dataset import job created using the <a>CreateDatasetImportJob</a> operation.</p> <p>In addition to listing the parameters provided in the <code>CreateDatasetImportJob</code> request, this operation includes the following properties:</p> <ul> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>DataSize</code> </p> </li> <li> <p> <code>FieldStatistics</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>Message</code> - If an error occurred, information about the error.</p> </li> </ul></p>
    async fn describe_dataset_import_job(
        &self,
        input: DescribeDatasetImportJobRequest,
    ) -> Result<DescribeDatasetImportJobResponse, RusotoError<DescribeDatasetImportJobError>>;

    /// <p><p>Describes a forecast created using the <a>CreateForecast</a> operation.</p> <p>In addition to listing the properties provided in the <code>CreateForecast</code> request, this operation lists the following properties:</p> <ul> <li> <p> <code>DatasetGroupArn</code> - The dataset group that provided the training data.</p> </li> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>Message</code> - If an error occurred, information about the error.</p> </li> </ul></p>
    async fn describe_forecast(
        &self,
        input: DescribeForecastRequest,
    ) -> Result<DescribeForecastResponse, RusotoError<DescribeForecastError>>;

    /// <p><p>Describes a forecast export job created using the <a>CreateForecastExportJob</a> operation.</p> <p>In addition to listing the properties provided by the user in the <code>CreateForecastExportJob</code> request, this operation lists the following properties:</p> <ul> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>Message</code> - If an error occurred, information about the error.</p> </li> </ul></p>
    async fn describe_forecast_export_job(
        &self,
        input: DescribeForecastExportJobRequest,
    ) -> Result<DescribeForecastExportJobResponse, RusotoError<DescribeForecastExportJobError>>;

    /// <p><p>Describes a predictor created using the <a>CreatePredictor</a> operation.</p> <p>In addition to listing the properties provided in the <code>CreatePredictor</code> request, this operation lists the following properties:</p> <ul> <li> <p> <code>DatasetImportJobArns</code> - The dataset import jobs used to import training data.</p> </li> <li> <p> <code>AutoMLAlgorithmArns</code> - If AutoML is performed, the algorithms that were evaluated.</p> </li> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>Message</code> - If an error occurred, information about the error.</p> </li> </ul></p>
    async fn describe_predictor(
        &self,
        input: DescribePredictorRequest,
    ) -> Result<DescribePredictorResponse, RusotoError<DescribePredictorError>>;

    /// <p><p>Provides metrics on the accuracy of the models that were trained by the <a>CreatePredictor</a> operation. Use metrics to see how well the model performed and to decide whether to use the predictor to generate a forecast. For more information, see <a>metrics</a>.</p> <p>This operation generates metrics for each backtest window that was evaluated. The number of backtest windows (<code>NumberOfBacktestWindows</code>) is specified using the <a>EvaluationParameters</a> object, which is optionally included in the <code>CreatePredictor</code> request. If <code>NumberOfBacktestWindows</code> isn&#39;t specified, the number defaults to one.</p> <p>The parameters of the <code>filling</code> method determine which items contribute to the metrics. If you want all items to contribute, specify <code>zero</code>. If you want only those items that have complete data in the range being evaluated to contribute, specify <code>nan</code>. For more information, see <a>FeaturizationMethod</a>.</p> <note> <p>Before you can get accuracy metrics, the <code>Status</code> of the predictor must be <code>ACTIVE</code>, signifying that training has completed. To get the status, use the <a>DescribePredictor</a> operation.</p> </note></p>
    async fn get_accuracy_metrics(
        &self,
        input: GetAccuracyMetricsRequest,
    ) -> Result<GetAccuracyMetricsResponse, RusotoError<GetAccuracyMetricsError>>;

    /// <p>Returns a list of dataset groups created using the <a>CreateDatasetGroup</a> operation. For each dataset group, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). You can retrieve the complete set of properties by using the dataset group ARN with the <a>DescribeDatasetGroup</a> operation.</p>
    async fn list_dataset_groups(
        &self,
        input: ListDatasetGroupsRequest,
    ) -> Result<ListDatasetGroupsResponse, RusotoError<ListDatasetGroupsError>>;

    /// <p>Returns a list of dataset import jobs created using the <a>CreateDatasetImportJob</a> operation. For each import job, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). You can retrieve the complete set of properties by using the ARN with the <a>DescribeDatasetImportJob</a> operation. You can filter the list by providing an array of <a>Filter</a> objects.</p>
    async fn list_dataset_import_jobs(
        &self,
        input: ListDatasetImportJobsRequest,
    ) -> Result<ListDatasetImportJobsResponse, RusotoError<ListDatasetImportJobsError>>;

    /// <p>Returns a list of datasets created using the <a>CreateDataset</a> operation. For each dataset, a summary of its properties, including its Amazon Resource Name (ARN), is returned. To retrieve the complete set of properties, use the ARN with the <a>DescribeDataset</a> operation.</p>
    async fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> Result<ListDatasetsResponse, RusotoError<ListDatasetsError>>;

    /// <p>Returns a list of forecast export jobs created using the <a>CreateForecastExportJob</a> operation. For each forecast export job, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). To retrieve the complete set of properties, use the ARN with the <a>DescribeForecastExportJob</a> operation. You can filter the list using an array of <a>Filter</a> objects.</p>
    async fn list_forecast_export_jobs(
        &self,
        input: ListForecastExportJobsRequest,
    ) -> Result<ListForecastExportJobsResponse, RusotoError<ListForecastExportJobsError>>;

    /// <p>Returns a list of forecasts created using the <a>CreateForecast</a> operation. For each forecast, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). To retrieve the complete set of properties, specify the ARN with the <a>DescribeForecast</a> operation. You can filter the list using an array of <a>Filter</a> objects.</p>
    async fn list_forecasts(
        &self,
        input: ListForecastsRequest,
    ) -> Result<ListForecastsResponse, RusotoError<ListForecastsError>>;

    /// <p>Returns a list of predictors created using the <a>CreatePredictor</a> operation. For each predictor, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). You can retrieve the complete set of properties by using the ARN with the <a>DescribePredictor</a> operation. You can filter the list using an array of <a>Filter</a> objects.</p>
    async fn list_predictors(
        &self,
        input: ListPredictorsRequest,
    ) -> Result<ListPredictorsResponse, RusotoError<ListPredictorsError>>;

    /// <p>Lists the tags for an Amazon Forecast resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are also deleted.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Deletes the specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><p>Replaces the datasets in a dataset group with the specified datasets.</p> <note> <p>The <code>Status</code> of the dataset group must be <code>ACTIVE</code> before you can use the dataset group to create a predictor. Use the <a>DescribeDatasetGroup</a> operation to get the status.</p> </note></p>
    async fn update_dataset_group(
        &self,
        input: UpdateDatasetGroupRequest,
    ) -> Result<UpdateDatasetGroupResponse, RusotoError<UpdateDatasetGroupError>>;
}
/// A client for the Amazon Forecast Service API.
#[derive(Clone)]
pub struct ForecastClient {
    client: Client,
    region: region::Region,
}

impl ForecastClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ForecastClient {
        ForecastClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ForecastClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ForecastClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ForecastClient {
        ForecastClient { client, region }
    }
}

#[async_trait]
impl Forecast for ForecastClient {
    /// <p><p>Creates an Amazon Forecast dataset. The information about the dataset that you provide helps Forecast understand how to consume the data for model training. This includes the following:</p> <ul> <li> <p> <i> <code>DataFrequency</code> </i> - How frequently your historical time-series data is collected.</p> </li> <li> <p> <i> <code>Domain</code> </i> and <i> <code>DatasetType</code> </i> - Each dataset has an associated dataset domain and a type within the domain. Amazon Forecast provides a list of predefined domains and types within each domain. For each unique dataset domain and type within the domain, Amazon Forecast requires your data to include a minimum set of predefined fields.</p> </li> <li> <p> <i> <code>Schema</code> </i> - A schema specifies the fields in the dataset, including the field name and data type.</p> </li> </ul> <p>After creating a dataset, you import your training data into it and add the dataset to a dataset group. You use the dataset group to create a predictor. For more information, see <a>howitworks-datasets-groups</a>.</p> <p>To get a list of all your datasets, use the <a>ListDatasets</a> operation.</p> <p>For example Forecast datasets, see the <a href="https://github.com/aws-samples/amazon-forecast-samples">Amazon Forecast Sample GitHub repository</a>.</p> <note> <p>The <code>Status</code> of a dataset must be <code>ACTIVE</code> before you can import training data. Use the <a>DescribeDataset</a> operation to get the status.</p> </note></p>
    async fn create_dataset(
        &self,
        input: CreateDatasetRequest,
    ) -> Result<CreateDatasetResponse, RusotoError<CreateDatasetError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.CreateDataset");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateDatasetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDatasetError::from_response(response))
        }
    }

    /// <p><p>Creates a dataset group, which holds a collection of related datasets. You can add datasets to the dataset group when you create the dataset group, or later by using the <a>UpdateDatasetGroup</a> operation.</p> <p>After creating a dataset group and adding datasets, you use the dataset group when you create a predictor. For more information, see <a>howitworks-datasets-groups</a>.</p> <p>To get a list of all your datasets groups, use the <a>ListDatasetGroups</a> operation.</p> <note> <p>The <code>Status</code> of a dataset group must be <code>ACTIVE</code> before you can create use the dataset group to create a predictor. To get the status, use the <a>DescribeDatasetGroup</a> operation.</p> </note></p>
    async fn create_dataset_group(
        &self,
        input: CreateDatasetGroupRequest,
    ) -> Result<CreateDatasetGroupResponse, RusotoError<CreateDatasetGroupError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.CreateDatasetGroup");
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
                .deserialize::<CreateDatasetGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDatasetGroupError::from_response(response))
        }
    }

    /// <p>Imports your training data to an Amazon Forecast dataset. You provide the location of your training data in an Amazon Simple Storage Service (Amazon S3) bucket and the Amazon Resource Name (ARN) of the dataset that you want to import the data to.</p> <p>You must specify a <a>DataSource</a> object that includes an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the data, as Amazon Forecast makes a copy of your data and processes it in an internal AWS system. For more information, see <a>aws-forecast-iam-roles</a>.</p> <p>The training data must be in CSV format. The delimiter must be a comma (,).</p> <p>You can specify the path to a specific CSV file, the S3 bucket, or to a folder in the S3 bucket. For the latter two cases, Amazon Forecast imports all files up to the limit of 10,000 files.</p> <p>Because dataset imports are not aggregated, your most recent dataset import is the one that is used when training a predictor or generating a forecast. Make sure that your most recent dataset import contains all of the data you want to model off of, and not just the new data collected since the previous import.</p> <p>To get a list of all your dataset import jobs, filtered by specified criteria, use the <a>ListDatasetImportJobs</a> operation.</p>
    async fn create_dataset_import_job(
        &self,
        input: CreateDatasetImportJobRequest,
    ) -> Result<CreateDatasetImportJobResponse, RusotoError<CreateDatasetImportJobError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.CreateDatasetImportJob");
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
                .deserialize::<CreateDatasetImportJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDatasetImportJobError::from_response(response))
        }
    }

    /// <p><p>Creates a forecast for each item in the <code>TARGET<em>TIME</em>SERIES</code> dataset that was used to train the predictor. This is known as inference. To retrieve the forecast for a single item at low latency, use the operation. To export the complete forecast into your Amazon Simple Storage Service (Amazon S3) bucket, use the <a>CreateForecastExportJob</a> operation.</p> <p>The range of the forecast is determined by the <code>ForecastHorizon</code> value, which you specify in the <a>CreatePredictor</a> request. When you query a forecast, you can request a specific date range within the forecast.</p> <p>To get a list of all your forecasts, use the <a>ListForecasts</a> operation.</p> <note> <p>The forecasts generated by Amazon Forecast are in the same time zone as the dataset that was used to create the predictor.</p> </note> <p>For more information, see <a>howitworks-forecast</a>.</p> <note> <p>The <code>Status</code> of the forecast must be <code>ACTIVE</code> before you can query or export the forecast. Use the <a>DescribeForecast</a> operation to get the status.</p> </note></p>
    async fn create_forecast(
        &self,
        input: CreateForecastRequest,
    ) -> Result<CreateForecastResponse, RusotoError<CreateForecastError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.CreateForecast");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateForecastResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateForecastError::from_response(response))
        }
    }

    /// <p><p>Exports a forecast created by the <a>CreateForecast</a> operation to your Amazon Simple Storage Service (Amazon S3) bucket. The forecast file name will match the following conventions:</p> <p>&lt;ForecastExportJobName&gt;<em>&lt;ExportTimestamp&gt;</em>&lt;PartNumber&gt;</p> <p>where the &lt;ExportTimestamp&gt; component is in Java SimpleDateFormat (yyyy-MM-ddTHH-mm-ssZ).</p> <p>You must specify a <a>DataDestination</a> object that includes an AWS Identity and Access Management (IAM) role that Amazon Forecast can assume to access the Amazon S3 bucket. For more information, see <a>aws-forecast-iam-roles</a>.</p> <p>For more information, see <a>howitworks-forecast</a>.</p> <p>To get a list of all your forecast export jobs, use the <a>ListForecastExportJobs</a> operation.</p> <note> <p>The <code>Status</code> of the forecast export job must be <code>ACTIVE</code> before you can access the forecast in your Amazon S3 bucket. To get the status, use the <a>DescribeForecastExportJob</a> operation.</p> </note></p>
    async fn create_forecast_export_job(
        &self,
        input: CreateForecastExportJobRequest,
    ) -> Result<CreateForecastExportJobResponse, RusotoError<CreateForecastExportJobError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.CreateForecastExportJob");
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
                .deserialize::<CreateForecastExportJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateForecastExportJobError::from_response(response))
        }
    }

    /// <p><p>Creates an Amazon Forecast predictor.</p> <p>In the request, you provide a dataset group and either specify an algorithm or let Amazon Forecast choose the algorithm for you using AutoML. If you specify an algorithm, you also can override algorithm-specific hyperparameters.</p> <p>Amazon Forecast uses the chosen algorithm to train a model using the latest version of the datasets in the specified dataset group. The result is called a predictor. You then generate a forecast using the <a>CreateForecast</a> operation.</p> <p>After training a model, the <code>CreatePredictor</code> operation also evaluates it. To see the evaluation metrics, use the <a>GetAccuracyMetrics</a> operation. Always review the evaluation metrics before deciding to use the predictor to generate a forecast.</p> <p>Optionally, you can specify a featurization configuration to fill and aggregate the data fields in the <code>TARGET<em>TIME</em>SERIES</code> dataset to improve model training. For more information, see <a>FeaturizationConfig</a>.</p> <p>For RELATED<em>TIME</em>SERIES datasets, <code>CreatePredictor</code> verifies that the <code>DataFrequency</code> specified when the dataset was created matches the <code>ForecastFrequency</code>. TARGET<em>TIME</em>SERIES datasets don&#39;t have this restriction. Amazon Forecast also verifies the delimiter and timestamp format. For more information, see <a>howitworks-datasets-groups</a>.</p> <p> <b>AutoML</b> </p> <p>If you want Amazon Forecast to evaluate each algorithm and choose the one that minimizes the <code>objective function</code>, set <code>PerformAutoML</code> to <code>true</code>. The <code>objective function</code> is defined as the mean of the weighted p10, p50, and p90 quantile losses. For more information, see <a>EvaluationResult</a>.</p> <p>When AutoML is enabled, the following properties are disallowed:</p> <ul> <li> <p> <code>AlgorithmArn</code> </p> </li> <li> <p> <code>HPOConfig</code> </p> </li> <li> <p> <code>PerformHPO</code> </p> </li> <li> <p> <code>TrainingParameters</code> </p> </li> </ul> <p>To get a list of all of your predictors, use the <a>ListPredictors</a> operation.</p> <note> <p>Before you can use the predictor to create a forecast, the <code>Status</code> of the predictor must be <code>ACTIVE</code>, signifying that training has completed. To get the status, use the <a>DescribePredictor</a> operation.</p> </note></p>
    async fn create_predictor(
        &self,
        input: CreatePredictorRequest,
    ) -> Result<CreatePredictorResponse, RusotoError<CreatePredictorError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.CreatePredictor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreatePredictorResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePredictorError::from_response(response))
        }
    }

    /// <p><p>Deletes an Amazon Forecast dataset that was created using the <a>CreateDataset</a> operation. You can only delete datasets that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status use the <a>DescribeDataset</a> operation.</p> <note> <p>Forecast does not automatically update any dataset groups that contain the deleted dataset. In order to update the dataset group, use the operation, omitting the deleted dataset&#39;s ARN.</p> </note></p>
    async fn delete_dataset(
        &self,
        input: DeleteDatasetRequest,
    ) -> Result<(), RusotoError<DeleteDatasetError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DeleteDataset");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDatasetError::from_response(response))
        }
    }

    /// <p>Deletes a dataset group created using the <a>CreateDatasetGroup</a> operation. You can only delete dataset groups that have a status of <code>ACTIVE</code>, <code>CREATE_FAILED</code>, or <code>UPDATE_FAILED</code>. To get the status, use the <a>DescribeDatasetGroup</a> operation.</p> <p>This operation deletes only the dataset group, not the datasets in the group.</p>
    async fn delete_dataset_group(
        &self,
        input: DeleteDatasetGroupRequest,
    ) -> Result<(), RusotoError<DeleteDatasetGroupError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DeleteDatasetGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDatasetGroupError::from_response(response))
        }
    }

    /// <p>Deletes a dataset import job created using the <a>CreateDatasetImportJob</a> operation. You can delete only dataset import jobs that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status, use the <a>DescribeDatasetImportJob</a> operation.</p>
    async fn delete_dataset_import_job(
        &self,
        input: DeleteDatasetImportJobRequest,
    ) -> Result<(), RusotoError<DeleteDatasetImportJobError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DeleteDatasetImportJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDatasetImportJobError::from_response(response))
        }
    }

    /// <p>Deletes a forecast created using the <a>CreateForecast</a> operation. You can delete only forecasts that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status, use the <a>DescribeForecast</a> operation.</p> <p>You can't delete a forecast while it is being exported. After a forecast is deleted, you can no longer query the forecast.</p>
    async fn delete_forecast(
        &self,
        input: DeleteForecastRequest,
    ) -> Result<(), RusotoError<DeleteForecastError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DeleteForecast");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteForecastError::from_response(response))
        }
    }

    /// <p>Deletes a forecast export job created using the <a>CreateForecastExportJob</a> operation. You can delete only export jobs that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status, use the <a>DescribeForecastExportJob</a> operation.</p>
    async fn delete_forecast_export_job(
        &self,
        input: DeleteForecastExportJobRequest,
    ) -> Result<(), RusotoError<DeleteForecastExportJobError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DeleteForecastExportJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteForecastExportJobError::from_response(response))
        }
    }

    /// <p>Deletes a predictor created using the <a>CreatePredictor</a> operation. You can delete only predictor that have a status of <code>ACTIVE</code> or <code>CREATE_FAILED</code>. To get the status, use the <a>DescribePredictor</a> operation.</p>
    async fn delete_predictor(
        &self,
        input: DeletePredictorRequest,
    ) -> Result<(), RusotoError<DeletePredictorError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DeletePredictor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            std::mem::drop(response);
            Ok(())
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePredictorError::from_response(response))
        }
    }

    /// <p><p>Describes an Amazon Forecast dataset created using the <a>CreateDataset</a> operation.</p> <p>In addition to listing the parameters specified in the <code>CreateDataset</code> request, this operation includes the following dataset properties:</p> <ul> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> </ul></p>
    async fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> Result<DescribeDatasetResponse, RusotoError<DescribeDatasetError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DescribeDataset");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeDatasetResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDatasetError::from_response(response))
        }
    }

    /// <p><p>Describes a dataset group created using the <a>CreateDatasetGroup</a> operation.</p> <p>In addition to listing the parameters provided in the <code>CreateDatasetGroup</code> request, this operation includes the following properties:</p> <ul> <li> <p> <code>DatasetArns</code> - The datasets belonging to the group.</p> </li> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> </ul></p>
    async fn describe_dataset_group(
        &self,
        input: DescribeDatasetGroupRequest,
    ) -> Result<DescribeDatasetGroupResponse, RusotoError<DescribeDatasetGroupError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DescribeDatasetGroup");
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
                .deserialize::<DescribeDatasetGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDatasetGroupError::from_response(response))
        }
    }

    /// <p><p>Describes a dataset import job created using the <a>CreateDatasetImportJob</a> operation.</p> <p>In addition to listing the parameters provided in the <code>CreateDatasetImportJob</code> request, this operation includes the following properties:</p> <ul> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>DataSize</code> </p> </li> <li> <p> <code>FieldStatistics</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>Message</code> - If an error occurred, information about the error.</p> </li> </ul></p>
    async fn describe_dataset_import_job(
        &self,
        input: DescribeDatasetImportJobRequest,
    ) -> Result<DescribeDatasetImportJobResponse, RusotoError<DescribeDatasetImportJobError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DescribeDatasetImportJob");
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
                .deserialize::<DescribeDatasetImportJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDatasetImportJobError::from_response(response))
        }
    }

    /// <p><p>Describes a forecast created using the <a>CreateForecast</a> operation.</p> <p>In addition to listing the properties provided in the <code>CreateForecast</code> request, this operation lists the following properties:</p> <ul> <li> <p> <code>DatasetGroupArn</code> - The dataset group that provided the training data.</p> </li> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>Message</code> - If an error occurred, information about the error.</p> </li> </ul></p>
    async fn describe_forecast(
        &self,
        input: DescribeForecastRequest,
    ) -> Result<DescribeForecastResponse, RusotoError<DescribeForecastError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DescribeForecast");
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
                .deserialize::<DescribeForecastResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeForecastError::from_response(response))
        }
    }

    /// <p><p>Describes a forecast export job created using the <a>CreateForecastExportJob</a> operation.</p> <p>In addition to listing the properties provided by the user in the <code>CreateForecastExportJob</code> request, this operation lists the following properties:</p> <ul> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>Message</code> - If an error occurred, information about the error.</p> </li> </ul></p>
    async fn describe_forecast_export_job(
        &self,
        input: DescribeForecastExportJobRequest,
    ) -> Result<DescribeForecastExportJobResponse, RusotoError<DescribeForecastExportJobError>>
    {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DescribeForecastExportJob");
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
                .deserialize::<DescribeForecastExportJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeForecastExportJobError::from_response(response))
        }
    }

    /// <p><p>Describes a predictor created using the <a>CreatePredictor</a> operation.</p> <p>In addition to listing the properties provided in the <code>CreatePredictor</code> request, this operation lists the following properties:</p> <ul> <li> <p> <code>DatasetImportJobArns</code> - The dataset import jobs used to import training data.</p> </li> <li> <p> <code>AutoMLAlgorithmArns</code> - If AutoML is performed, the algorithms that were evaluated.</p> </li> <li> <p> <code>CreationTime</code> </p> </li> <li> <p> <code>LastModificationTime</code> </p> </li> <li> <p> <code>Status</code> </p> </li> <li> <p> <code>Message</code> - If an error occurred, information about the error.</p> </li> </ul></p>
    async fn describe_predictor(
        &self,
        input: DescribePredictorRequest,
    ) -> Result<DescribePredictorResponse, RusotoError<DescribePredictorError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.DescribePredictor");
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
                .deserialize::<DescribePredictorResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribePredictorError::from_response(response))
        }
    }

    /// <p><p>Provides metrics on the accuracy of the models that were trained by the <a>CreatePredictor</a> operation. Use metrics to see how well the model performed and to decide whether to use the predictor to generate a forecast. For more information, see <a>metrics</a>.</p> <p>This operation generates metrics for each backtest window that was evaluated. The number of backtest windows (<code>NumberOfBacktestWindows</code>) is specified using the <a>EvaluationParameters</a> object, which is optionally included in the <code>CreatePredictor</code> request. If <code>NumberOfBacktestWindows</code> isn&#39;t specified, the number defaults to one.</p> <p>The parameters of the <code>filling</code> method determine which items contribute to the metrics. If you want all items to contribute, specify <code>zero</code>. If you want only those items that have complete data in the range being evaluated to contribute, specify <code>nan</code>. For more information, see <a>FeaturizationMethod</a>.</p> <note> <p>Before you can get accuracy metrics, the <code>Status</code> of the predictor must be <code>ACTIVE</code>, signifying that training has completed. To get the status, use the <a>DescribePredictor</a> operation.</p> </note></p>
    async fn get_accuracy_metrics(
        &self,
        input: GetAccuracyMetricsRequest,
    ) -> Result<GetAccuracyMetricsResponse, RusotoError<GetAccuracyMetricsError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.GetAccuracyMetrics");
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
                .deserialize::<GetAccuracyMetricsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetAccuracyMetricsError::from_response(response))
        }
    }

    /// <p>Returns a list of dataset groups created using the <a>CreateDatasetGroup</a> operation. For each dataset group, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). You can retrieve the complete set of properties by using the dataset group ARN with the <a>DescribeDatasetGroup</a> operation.</p>
    async fn list_dataset_groups(
        &self,
        input: ListDatasetGroupsRequest,
    ) -> Result<ListDatasetGroupsResponse, RusotoError<ListDatasetGroupsError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.ListDatasetGroups");
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
                .deserialize::<ListDatasetGroupsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDatasetGroupsError::from_response(response))
        }
    }

    /// <p>Returns a list of dataset import jobs created using the <a>CreateDatasetImportJob</a> operation. For each import job, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). You can retrieve the complete set of properties by using the ARN with the <a>DescribeDatasetImportJob</a> operation. You can filter the list by providing an array of <a>Filter</a> objects.</p>
    async fn list_dataset_import_jobs(
        &self,
        input: ListDatasetImportJobsRequest,
    ) -> Result<ListDatasetImportJobsResponse, RusotoError<ListDatasetImportJobsError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.ListDatasetImportJobs");
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
                .deserialize::<ListDatasetImportJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDatasetImportJobsError::from_response(response))
        }
    }

    /// <p>Returns a list of datasets created using the <a>CreateDataset</a> operation. For each dataset, a summary of its properties, including its Amazon Resource Name (ARN), is returned. To retrieve the complete set of properties, use the ARN with the <a>DescribeDataset</a> operation.</p>
    async fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> Result<ListDatasetsResponse, RusotoError<ListDatasetsError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.ListDatasets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListDatasetsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDatasetsError::from_response(response))
        }
    }

    /// <p>Returns a list of forecast export jobs created using the <a>CreateForecastExportJob</a> operation. For each forecast export job, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). To retrieve the complete set of properties, use the ARN with the <a>DescribeForecastExportJob</a> operation. You can filter the list using an array of <a>Filter</a> objects.</p>
    async fn list_forecast_export_jobs(
        &self,
        input: ListForecastExportJobsRequest,
    ) -> Result<ListForecastExportJobsResponse, RusotoError<ListForecastExportJobsError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.ListForecastExportJobs");
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
                .deserialize::<ListForecastExportJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListForecastExportJobsError::from_response(response))
        }
    }

    /// <p>Returns a list of forecasts created using the <a>CreateForecast</a> operation. For each forecast, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). To retrieve the complete set of properties, specify the ARN with the <a>DescribeForecast</a> operation. You can filter the list using an array of <a>Filter</a> objects.</p>
    async fn list_forecasts(
        &self,
        input: ListForecastsRequest,
    ) -> Result<ListForecastsResponse, RusotoError<ListForecastsError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.ListForecasts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListForecastsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListForecastsError::from_response(response))
        }
    }

    /// <p>Returns a list of predictors created using the <a>CreatePredictor</a> operation. For each predictor, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). You can retrieve the complete set of properties by using the ARN with the <a>DescribePredictor</a> operation. You can filter the list using an array of <a>Filter</a> objects.</p>
    async fn list_predictors(
        &self,
        input: ListPredictorsRequest,
    ) -> Result<ListPredictorsResponse, RusotoError<ListPredictorsError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.ListPredictors");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListPredictorsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListPredictorsError::from_response(response))
        }
    }

    /// <p>Lists the tags for an Amazon Forecast resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are also deleted.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Deletes the specified tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p><p>Replaces the datasets in a dataset group with the specified datasets.</p> <note> <p>The <code>Status</code> of the dataset group must be <code>ACTIVE</code> before you can use the dataset group to create a predictor. Use the <a>DescribeDatasetGroup</a> operation to get the status.</p> </note></p>
    async fn update_dataset_group(
        &self,
        input: UpdateDatasetGroupRequest,
    ) -> Result<UpdateDatasetGroupResponse, RusotoError<UpdateDatasetGroupError>> {
        let mut request = SignedRequest::new("POST", "forecast", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonForecast.UpdateDatasetGroup");
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
                .deserialize::<UpdateDatasetGroupResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDatasetGroupError::from_response(response))
        }
    }
}
