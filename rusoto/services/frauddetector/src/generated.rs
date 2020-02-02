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
/// <p>Provides the error of the batch create variable API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCreateVariableError {
    /// <p>The error code. </p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// <p>The error message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchCreateVariableRequest {
    /// <p>The list of variables for the batch create variable request.</p>
    #[serde(rename = "variableEntries")]
    pub variable_entries: Vec<VariableEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCreateVariableResult {
    /// <p>Provides the errors for the <code>BatchCreateVariable</code> request.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchCreateVariableError>>,
}

/// <p>Provides the error of the batch get variable API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetVariableError {
    /// <p>The error code. </p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// <p>The error message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The error name. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetVariableRequest {
    /// <p>The list of variable names to get.</p>
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetVariableResult {
    /// <p>The errors from the request.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchGetVariableError>>,
    /// <p>The returned variables.</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<Variable>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDetectorVersionRequest {
    /// <p>The description of the detector version.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the detector under which you want to create a new version.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The Amazon Sagemaker model endpoints to include in the detector version.</p>
    #[serde(rename = "externalModelEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_model_endpoints: Option<Vec<String>>,
    /// <p>The model versions to include in the detector version.</p>
    #[serde(rename = "modelVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_versions: Option<Vec<ModelVersion>>,
    /// <p>The rules to include in the detector version.</p>
    #[serde(rename = "rules")]
    pub rules: Vec<Rule>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDetectorVersionResult {
    /// <p>The ID for the created version's parent detector.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>The ID for the created detector. </p>
    #[serde(rename = "detectorVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_version_id: Option<String>,
    /// <p>The status of the detector version.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateModelVersionRequest {
    /// <p>The model version description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model ID. </p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateModelVersionResult {
    /// <p>The model ID. </p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    /// <p>The version of the model. </p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p>The model version status. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRuleRequest {
    /// <p>The rule description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The detector ID for the rule's parent detector.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The rule expression.</p>
    #[serde(rename = "expression")]
    pub expression: String,
    /// <p>The language of the rule.</p>
    #[serde(rename = "language")]
    pub language: String,
    /// <p>The outcome or outcomes returned when the rule expression matches.</p>
    #[serde(rename = "outcomes")]
    pub outcomes: Vec<String>,
    /// <p>The rule ID.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRuleResult {
    /// <p>The created rule.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVariableRequest {
    /// <p>The source of the data.</p>
    #[serde(rename = "dataSource")]
    pub data_source: String,
    /// <p>The data type.</p>
    #[serde(rename = "dataType")]
    pub data_type: String,
    /// <p>The default value for the variable when no value is received.</p>
    #[serde(rename = "defaultValue")]
    pub default_value: String,
    /// <p>The description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The variable type.</p>
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVariableResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDetectorVersionRequest {
    /// <p>The ID of the parent detector for the detector version to delete.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The ID of the detector version to delete.</p>
    #[serde(rename = "detectorVersionId")]
    pub detector_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDetectorVersionResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventRequest {
    /// <p>The ID of the event to delete.</p>
    #[serde(rename = "eventId")]
    pub event_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEventResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDetectorRequest {
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The maximum number of results to return for the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next token from the previous response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDetectorResult {
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>The status and description for each detector version.</p>
    #[serde(rename = "detectorVersionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_version_summaries: Option<Vec<DetectorVersionSummary>>,
    /// <p>The next token to be used for subsequent requests.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeModelVersionsRequest {
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    /// <p>The model version. </p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p>The next token from the previous results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeModelVersionsResult {
    /// <p>The model version details.</p>
    #[serde(rename = "modelVersionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_details: Option<Vec<ModelVersionDetail>>,
    /// <p>The next token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The detector.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Detector {
    /// <p>Timestamp of when the detector was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The detector description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>Timestamp of when the detector was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
}

/// <p>The summary of the detector version.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorVersionSummary {
    /// <p>The detector version description. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The detector version ID. </p>
    #[serde(rename = "detectorVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_version_id: Option<String>,
    /// <p>Timestamp of when the detector version was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The detector version status. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The Amazon SageMaker model.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExternalModel {
    /// <p>Timestamp of when the model was last created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The input configuration.</p>
    #[serde(rename = "inputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configuration: Option<ModelInputConfiguration>,
    /// <p>Timestamp of when the model was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The Amazon SageMaker model endpoints.</p>
    #[serde(rename = "modelEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_endpoint: Option<String>,
    /// <p>The Amazon Fraud Detector status for the external model endpoint</p>
    #[serde(rename = "modelEndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_endpoint_status: Option<String>,
    /// <p>The source of the model.</p>
    #[serde(rename = "modelSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_source: Option<String>,
    /// <p>The output configuration.</p>
    #[serde(rename = "outputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_configuration: Option<ModelOutputConfiguration>,
    /// <p>The role used to invoke the model. </p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDetectorVersionRequest {
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The detector version ID.</p>
    #[serde(rename = "detectorVersionId")]
    pub detector_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDetectorVersionResult {
    /// <p>The timestamp when the detector version was created. </p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The detector version description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>The detector version ID.</p>
    #[serde(rename = "detectorVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_version_id: Option<String>,
    /// <p>The Amazon SageMaker model endpoints included in the detector version.</p>
    #[serde(rename = "externalModelEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_model_endpoints: Option<Vec<String>>,
    /// <p>The timestamp when the detector version was last updated. </p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The model versions included in the detector version. </p>
    #[serde(rename = "modelVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_versions: Option<Vec<ModelVersion>>,
    /// <p>The rules included in the detector version.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p>The status of the detector version.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDetectorsRequest {
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>The maximum number of objects to return for the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next token for the subsequent request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDetectorsResult {
    /// <p>The detectors.</p>
    #[serde(rename = "detectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detectors: Option<Vec<Detector>>,
    /// <p>The next page token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetExternalModelsRequest {
    /// <p>The maximum number of objects to return for the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The Amazon SageMaker model endpoint.</p>
    #[serde(rename = "modelEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_endpoint: Option<String>,
    /// <p>The next page token for the request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetExternalModelsResult {
    /// <p>Gets the Amazon SageMaker models.</p>
    #[serde(rename = "externalModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_models: Option<Vec<ExternalModel>>,
    /// <p>The next page token to be used in subsequent requests.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetModelVersionRequest {
    /// <p>The model ID. </p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type. </p>
    #[serde(rename = "modelType")]
    pub model_type: String,
    /// <p>The model version. </p>
    #[serde(rename = "modelVersionNumber")]
    pub model_version_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetModelVersionResult {
    /// <p>The model version description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model ID. </p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type. </p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    /// <p>The model version. </p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p>The model version status. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetModelsRequest {
    /// <p>The maximum results to return for the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    /// <p>The next token for the request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetModelsResult {
    /// <p>The returned models. </p>
    #[serde(rename = "models")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<Model>>,
    /// <p>The next token for subsequent requests. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOutcomesRequest {
    /// <p>The maximum number of objects to return for the request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the outcome or outcomes to get.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The next page token for the request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOutcomesResult {
    /// <p>The next page token for subsequent requests.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The outcomes. </p>
    #[serde(rename = "outcomes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Vec<Outcome>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPredictionRequest {
    /// <p>The detector ID. </p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The detector version ID.</p>
    #[serde(rename = "detectorVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_version_id: Option<String>,
    /// <p>Names of variables you defined in Amazon Fraud Detector to represent event data elements and their corresponding values for the event you are sending for evaluation.</p>
    #[serde(rename = "eventAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The unique ID used to identify the event.</p>
    #[serde(rename = "eventId")]
    pub event_id: String,
    /// <p>The Amazon SageMaker model endpoint input data blobs.</p>
    #[serde(rename = "externalModelEndpointDataBlobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_model_endpoint_data_blobs:
        Option<::std::collections::HashMap<String, ModelEndpointDataBlob>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPredictionResult {
    /// <p>The model scores for models used in the detector version.</p>
    #[serde(rename = "modelScores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_scores: Option<Vec<ModelScores>>,
    /// <p>The prediction outcomes.</p>
    #[serde(rename = "outcomes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRulesRequest {
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The maximum number of rules to return for the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next page token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The rule ID.</p>
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// <p>The rule version.</p>
    #[serde(rename = "ruleVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRulesResult {
    /// <p>The next page token to be used in subsequent requests.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The details of the requested rule.</p>
    #[serde(rename = "ruleDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_details: Option<Vec<RuleDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVariablesRequest {
    /// <p>The max size per page determined for the get variable request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the variable. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The next page token of the get variable request. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVariablesResult {
    /// <p>The next page token to be used in subsequent requests. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The names of the variables returned. </p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<Variable>>,
}

/// <p>The label schema.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelSchema {
    /// <p>The label key.</p>
    #[serde(rename = "labelKey")]
    pub label_key: String,
    /// <p>The label mapper maps the Amazon Fraud Detector supported label to the appropriate source labels. For example, if <code>"FRAUD"</code> and <code>"LEGIT"</code> are Amazon Fraud Detector supported labels, this mapper could be: <code>{"FRAUD" =&gt; ["0"]</code>, "LEGIT" =&gt; ["1"]} or <code>{"FRAUD" =&gt; ["false"], "LEGIT" =&gt; ["true"]}</code> or <code>{"FRAUD" =&gt; ["fraud", "abuse"], "LEGIT" =&gt; ["legit", "safe"]}</code>. The value part of the mapper is a list, because you may have multiple variants for a single Amazon Fraud Detector label. </p>
    #[serde(rename = "labelMapper")]
    pub label_mapper: ::std::collections::HashMap<String, Vec<String>>,
}

/// <p>The model.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Model {
    /// <p>Timestamp of when the model was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The model description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model label schema.</p>
    #[serde(rename = "labelSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_schema: Option<LabelSchema>,
    /// <p>Timestamp of last time the model was updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    /// <p>The model input variables.</p>
    #[serde(rename = "modelVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_variables: Option<Vec<ModelVariable>>,
    /// <p>The model training data source in Amazon S3.</p>
    #[serde(rename = "trainingDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_source: Option<TrainingDataSource>,
}

/// <p>A pre-formed Amazon SageMaker model input you can include if your detector version includes an imported Amazon SageMaker model endpoint with pass-through input configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModelEndpointDataBlob {
    /// <p>The byte buffer of the Amazon SageMaker model endpoint input data blob.</p>
    #[serde(rename = "byteBuffer")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_buffer: Option<bytes::Bytes>,
    /// <p>The content type of the Amazon SageMaker model endpoint input data blob. </p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// <p>The model input configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelInputConfiguration {
    /// <p> Template for constructing the CSV input-data sent to SageMaker. At event-evaluation, the placeholders for variable-names in the template will be replaced with the variable values before being sent to SageMaker. </p>
    #[serde(rename = "csvInputTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_input_template: Option<String>,
    /// <p> The format of the model input configuration. The format differs depending on if it is passed through to SageMaker or constructed by Amazon Fraud Detector.</p>
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p> For an opaque-model, the input to the model will be a ByteBuffer blob provided in the getPrediction request, and will be passed to SageMaker as-is. For non-opaque models, the input will be constructed by Amazon Fraud Detector based on the model-configuration. </p>
    #[serde(rename = "isOpaque")]
    pub is_opaque: bool,
    /// <p> Template for constructing the JSON input-data sent to SageMaker. At event-evaluation, the placeholders for variable names in the template will be replaced with the variable values before being sent to SageMaker. </p>
    #[serde(rename = "jsonInputTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_input_template: Option<String>,
}

/// <p>Provides the model output configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelOutputConfiguration {
    /// <p>A map of CSV index values in the SageMaker response to the Amazon Fraud Detector variables. </p>
    #[serde(rename = "csvIndexToVariableMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_index_to_variable_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>The format of the model output configuration.</p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>A map of JSON keys in response from SageMaker to the Amazon Fraud Detector variables. </p>
    #[serde(rename = "jsonKeyToVariableMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_key_to_variable_map: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The fraud prediction scores.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModelScores {
    /// <p>The model version.</p>
    #[serde(rename = "modelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<ModelVersion>,
    /// <p>The model's fraud prediction scores.</p>
    #[serde(rename = "scores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores: Option<::std::collections::HashMap<String, f32>>,
}

/// <p>The model variable.&gt;</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelVariable {
    /// <p>The model variable's index.&gt;</p>
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The model variable's name.&gt;</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>The model version.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelVersion {
    /// <p>The parent model ID.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: String,
    /// <p>The model version.</p>
    #[serde(rename = "modelVersionNumber")]
    pub model_version_number: String,
}

/// <p>Provides the model version details. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModelVersionDetail {
    /// <p>The timestamp when the model was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The model description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model label schema.</p>
    #[serde(rename = "labelSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_schema: Option<LabelSchema>,
    /// <p>The timestamp when the model was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    /// <p>The model variables.</p>
    #[serde(rename = "modelVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_variables: Option<Vec<ModelVariable>>,
    /// <p>The model version.</p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p>The model status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The model training data source.</p>
    #[serde(rename = "trainingDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_source: Option<TrainingDataSource>,
    /// <p>The model training metrics.</p>
    #[serde(rename = "trainingMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_metrics: Option<::std::collections::HashMap<String, String>>,
    /// <p>The model validation metrics.</p>
    #[serde(rename = "validationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_metrics: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The outcome.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Outcome {
    /// <p>The timestamp when the outcome was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The outcome description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp when the outcome was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The outcome name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDetectorRequest {
    /// <p>The description of the detector.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The detector ID. </p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDetectorResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutExternalModelRequest {
    /// <p>The model endpoint input configuration.</p>
    #[serde(rename = "inputConfiguration")]
    pub input_configuration: ModelInputConfiguration,
    /// <p>The model endpoints name.</p>
    #[serde(rename = "modelEndpoint")]
    pub model_endpoint: String,
    /// <p>The model endpoint’s status in Amazon Fraud Detector.</p>
    #[serde(rename = "modelEndpointStatus")]
    pub model_endpoint_status: String,
    /// <p>The source of the model.</p>
    #[serde(rename = "modelSource")]
    pub model_source: String,
    /// <p>The model endpoint output configuration.</p>
    #[serde(rename = "outputConfiguration")]
    pub output_configuration: ModelOutputConfiguration,
    /// <p>The IAM role used to invoke the model endpoint.</p>
    #[serde(rename = "role")]
    pub role: Role,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutExternalModelResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutModelRequest {
    /// <p>The model description. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The label schema.</p>
    #[serde(rename = "labelSchema")]
    pub label_schema: LabelSchema,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type. </p>
    #[serde(rename = "modelType")]
    pub model_type: String,
    /// <p>The model input variables.</p>
    #[serde(rename = "modelVariables")]
    pub model_variables: Vec<ModelVariable>,
    /// <p>The training data source location in Amazon S3. </p>
    #[serde(rename = "trainingDataSource")]
    pub training_data_source: TrainingDataSource,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutModelResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutOutcomeRequest {
    /// <p>The outcome description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the outcome.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutOutcomeResult {}

/// <p>The role used to invoke external model endpoints.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    /// <p>The role ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The role name.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>A rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// <p>The detector for which the rule is associated.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The rule ID.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    /// <p>The rule version.</p>
    #[serde(rename = "ruleVersion")]
    pub rule_version: String,
}

/// <p>The details of the rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleDetail {
    /// <p>The timestamp of when the rule was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The rule description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The detector for which the rule is associated.</p>
    #[serde(rename = "detectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    /// <p>The rule expression.</p>
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The rule language.</p>
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// <p>Timestamp of the last time the rule was updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The rule outcomes.</p>
    #[serde(rename = "outcomes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Vec<String>>,
    /// <p>The rule ID.</p>
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// <p>The rule version.</p>
    #[serde(rename = "ruleVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_version: Option<String>,
}

/// <p>The training data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingDataSource {
    /// <p>The data access role ARN for the training data source.</p>
    #[serde(rename = "dataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>The data location of the training data source.</p>
    #[serde(rename = "dataLocation")]
    pub data_location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDetectorVersionMetadataRequest {
    /// <p>The description.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The detector version ID. </p>
    #[serde(rename = "detectorVersionId")]
    pub detector_version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorVersionMetadataResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDetectorVersionRequest {
    /// <p>The detector version description. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The parent detector ID for the detector version you want to update.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The detector version ID. </p>
    #[serde(rename = "detectorVersionId")]
    pub detector_version_id: String,
    /// <p>The Amazon SageMaker model endpoints to include in the detector version.</p>
    #[serde(rename = "externalModelEndpoints")]
    pub external_model_endpoints: Vec<String>,
    /// <p>The model versions to include in the detector version.</p>
    #[serde(rename = "modelVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_versions: Option<Vec<ModelVersion>>,
    /// <p>The rules to include in the detector version.</p>
    #[serde(rename = "rules")]
    pub rules: Vec<Rule>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorVersionResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDetectorVersionStatusRequest {
    /// <p>The detector ID. </p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The detector version ID. </p>
    #[serde(rename = "detectorVersionId")]
    pub detector_version_id: String,
    /// <p>The new status.</p>
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorVersionStatusResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateModelVersionRequest {
    /// <p>The model description.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: String,
    /// <p>The model version.</p>
    #[serde(rename = "modelVersionNumber")]
    pub model_version_number: String,
    /// <p>The new model status.</p>
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateModelVersionResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRuleMetadataRequest {
    /// <p>The rule description.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The rule to update.</p>
    #[serde(rename = "rule")]
    pub rule: Rule,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRuleMetadataResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRuleVersionRequest {
    /// <p>The description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The rule expression.</p>
    #[serde(rename = "expression")]
    pub expression: String,
    /// <p>The language.</p>
    #[serde(rename = "language")]
    pub language: String,
    /// <p>The outcomes.</p>
    #[serde(rename = "outcomes")]
    pub outcomes: Vec<String>,
    /// <p>The rule to update.</p>
    #[serde(rename = "rule")]
    pub rule: Rule,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRuleVersionResult {
    /// <p>The new rule version that was created.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVariableRequest {
    /// <p>The new default value of the variable.</p>
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>The new description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The variable type.</p>
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVariableResult {}

/// <p>The variable.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Variable {
    /// <p>The time when the variable was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The data source of the variable.</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    /// <p>The data type of the variable.</p>
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// <p>The default value of the variable.</p>
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>The description of the variable. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time when variable was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The variable type of the variable.</p>
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}

/// <p>The variable entry in a list.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VariableEntry {
    /// <p>The data source of the variable entry.</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    /// <p>The data type of the variable entry.</p>
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// <p>The default value of the variable entry.</p>
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>The description of the variable entry.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the variable entry.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of the variable entry.</p>
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}

/// Errors returned by BatchCreateVariable
#[derive(Debug, PartialEq)]
pub enum BatchCreateVariableError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl BatchCreateVariableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchCreateVariableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(BatchCreateVariableError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchCreateVariableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchCreateVariableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchCreateVariableError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchCreateVariableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchCreateVariableError {}
/// Errors returned by BatchGetVariable
#[derive(Debug, PartialEq)]
pub enum BatchGetVariableError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl BatchGetVariableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetVariableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(BatchGetVariableError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchGetVariableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchGetVariableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchGetVariableError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchGetVariableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetVariableError {}
/// Errors returned by CreateDetectorVersion
#[derive(Debug, PartialEq)]
pub enum CreateDetectorVersionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl CreateDetectorVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDetectorVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateDetectorVersionError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDetectorVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDetectorVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDetectorVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDetectorVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDetectorVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDetectorVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDetectorVersionError {}
/// Errors returned by CreateModelVersion
#[derive(Debug, PartialEq)]
pub enum CreateModelVersionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl CreateModelVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateModelVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateModelVersionError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateModelVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateModelVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateModelVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateModelVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateModelVersionError {}
/// Errors returned by CreateRule
#[derive(Debug, PartialEq)]
pub enum CreateRuleError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl CreateRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateRuleError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRuleError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRuleError {}
/// Errors returned by CreateVariable
#[derive(Debug, PartialEq)]
pub enum CreateVariableError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl CreateVariableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVariableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateVariableError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateVariableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVariableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVariableError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateVariableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVariableError {}
/// Errors returned by DeleteDetectorVersion
#[derive(Debug, PartialEq)]
pub enum DeleteDetectorVersionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DeleteDetectorVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDetectorVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDetectorVersionError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDetectorVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDetectorVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDetectorVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDetectorVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDetectorVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDetectorVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDetectorVersionError {}
/// Errors returned by DeleteEvent
#[derive(Debug, PartialEq)]
pub enum DeleteEventError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DeleteEventError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteEventError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteEventError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEventError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteEventError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventError {}
/// Errors returned by DescribeDetector
#[derive(Debug, PartialEq)]
pub enum DescribeDetectorError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DescribeDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDetectorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeDetectorError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDetectorError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDetectorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDetectorError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDetectorError {}
/// Errors returned by DescribeModelVersions
#[derive(Debug, PartialEq)]
pub enum DescribeModelVersionsError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DescribeModelVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeModelVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeModelVersionsError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeModelVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeModelVersionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeModelVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeModelVersionsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeModelVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeModelVersionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeModelVersionsError {}
/// Errors returned by GetDetectorVersion
#[derive(Debug, PartialEq)]
pub enum GetDetectorVersionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetDetectorVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDetectorVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetDetectorVersionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDetectorVersionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDetectorVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDetectorVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDetectorVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDetectorVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetDetectorVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDetectorVersionError {}
/// Errors returned by GetDetectors
#[derive(Debug, PartialEq)]
pub enum GetDetectorsError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetDetectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDetectorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetDetectorsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDetectorsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDetectorsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDetectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDetectorsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDetectorsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetDetectorsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDetectorsError {}
/// Errors returned by GetExternalModels
#[derive(Debug, PartialEq)]
pub enum GetExternalModelsError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetExternalModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExternalModelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetExternalModelsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetExternalModelsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetExternalModelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetExternalModelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetExternalModelsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetExternalModelsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetExternalModelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetExternalModelsError {}
/// Errors returned by GetModelVersion
#[derive(Debug, PartialEq)]
pub enum GetModelVersionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetModelVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetModelVersionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetModelVersionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetModelVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetModelVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetModelVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetModelVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetModelVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetModelVersionError {}
/// Errors returned by GetModels
#[derive(Debug, PartialEq)]
pub enum GetModelsError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetModelsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetModelsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetModelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetModelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetModelsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetModelsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetModelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetModelsError {}
/// Errors returned by GetOutcomes
#[derive(Debug, PartialEq)]
pub enum GetOutcomesError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetOutcomesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOutcomesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetOutcomesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetOutcomesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetOutcomesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOutcomesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOutcomesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetOutcomesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetOutcomesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOutcomesError {}
/// Errors returned by GetPrediction
#[derive(Debug, PartialEq)]
pub enum GetPredictionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetPredictionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPredictionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetPredictionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPredictionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetPredictionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPredictionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPredictionError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetPredictionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetPredictionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPredictionError {}
/// Errors returned by GetRules
#[derive(Debug, PartialEq)]
pub enum GetRulesError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetRulesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRulesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetRulesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRulesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetRulesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetRulesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRulesError {}
/// Errors returned by GetVariables
#[derive(Debug, PartialEq)]
pub enum GetVariablesError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetVariablesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVariablesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetVariablesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetVariablesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetVariablesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVariablesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVariablesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetVariablesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetVariablesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVariablesError {}
/// Errors returned by PutDetector
#[derive(Debug, PartialEq)]
pub enum PutDetectorError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl PutDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDetectorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(PutDetectorError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutDetectorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDetectorError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutDetectorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutDetectorError {}
/// Errors returned by PutExternalModel
#[derive(Debug, PartialEq)]
pub enum PutExternalModelError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl PutExternalModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutExternalModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(PutExternalModelError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutExternalModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutExternalModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutExternalModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutExternalModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutExternalModelError {}
/// Errors returned by PutModel
#[derive(Debug, PartialEq)]
pub enum PutModelError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl PutModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(PutModelError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutModelError {}
/// Errors returned by PutOutcome
#[derive(Debug, PartialEq)]
pub enum PutOutcomeError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl PutOutcomeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutOutcomeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(PutOutcomeError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutOutcomeError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutOutcomeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutOutcomeError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutOutcomeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutOutcomeError {}
/// Errors returned by UpdateDetectorVersion
#[derive(Debug, PartialEq)]
pub enum UpdateDetectorVersionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl UpdateDetectorVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDetectorVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDetectorVersionError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDetectorVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDetectorVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDetectorVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDetectorVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDetectorVersionError {}
/// Errors returned by UpdateDetectorVersionMetadata
#[derive(Debug, PartialEq)]
pub enum UpdateDetectorVersionMetadataError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl UpdateDetectorVersionMetadataError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDetectorVersionMetadataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        UpdateDetectorVersionMetadataError::InternalServer(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDetectorVersionMetadataError::Throttling(
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
impl fmt::Display for UpdateDetectorVersionMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDetectorVersionMetadataError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionMetadataError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDetectorVersionMetadataError {}
/// Errors returned by UpdateDetectorVersionStatus
#[derive(Debug, PartialEq)]
pub enum UpdateDetectorVersionStatusError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl UpdateDetectorVersionStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDetectorVersionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDetectorVersionStatusError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateDetectorVersionStatusError::ResourceNotFound(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDetectorVersionStatusError::Throttling(
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
impl fmt::Display for UpdateDetectorVersionStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDetectorVersionStatusError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionStatusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDetectorVersionStatusError {}
/// Errors returned by UpdateModelVersion
#[derive(Debug, PartialEq)]
pub enum UpdateModelVersionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl UpdateModelVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateModelVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateModelVersionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateModelVersionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateModelVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateModelVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateModelVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateModelVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateModelVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateModelVersionError {}
/// Errors returned by UpdateRuleMetadata
#[derive(Debug, PartialEq)]
pub enum UpdateRuleMetadataError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl UpdateRuleMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRuleMetadataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateRuleMetadataError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRuleMetadataError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateRuleMetadataError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRuleMetadataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRuleMetadataError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateRuleMetadataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateRuleMetadataError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRuleMetadataError {}
/// Errors returned by UpdateRuleVersion
#[derive(Debug, PartialEq)]
pub enum UpdateRuleVersionError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl UpdateRuleVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRuleVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateRuleVersionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRuleVersionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateRuleVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRuleVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRuleVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateRuleVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateRuleVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRuleVersionError {}
/// Errors returned by UpdateVariable
#[derive(Debug, PartialEq)]
pub enum UpdateVariableError {
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl UpdateVariableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVariableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateVariableError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateVariableError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateVariableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVariableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVariableError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateVariableError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateVariableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVariableError {}
/// Trait representing the capabilities of the Amazon Fraud Detector API. Amazon Fraud Detector clients implement this trait.
#[async_trait]
pub trait FraudDetector {
    /// <p>Creates a batch of variables.</p>
    async fn batch_create_variable(
        &self,
        input: BatchCreateVariableRequest,
    ) -> Result<BatchCreateVariableResult, RusotoError<BatchCreateVariableError>>;

    /// <p>Gets a batch of variables.</p>
    async fn batch_get_variable(
        &self,
        input: BatchGetVariableRequest,
    ) -> Result<BatchGetVariableResult, RusotoError<BatchGetVariableError>>;

    /// <p>Creates a detector version. The detector version starts in a <code>DRAFT</code> status.</p>
    async fn create_detector_version(
        &self,
        input: CreateDetectorVersionRequest,
    ) -> Result<CreateDetectorVersionResult, RusotoError<CreateDetectorVersionError>>;

    /// <p>Creates a version of the model using the specified model type. </p>
    async fn create_model_version(
        &self,
        input: CreateModelVersionRequest,
    ) -> Result<CreateModelVersionResult, RusotoError<CreateModelVersionError>>;

    /// <p>Creates a rule for use with the specified detector. </p>
    async fn create_rule(
        &self,
        input: CreateRuleRequest,
    ) -> Result<CreateRuleResult, RusotoError<CreateRuleError>>;

    /// <p>Creates a variable.</p>
    async fn create_variable(
        &self,
        input: CreateVariableRequest,
    ) -> Result<CreateVariableResult, RusotoError<CreateVariableError>>;

    /// <p>Deletes the detector version.</p>
    async fn delete_detector_version(
        &self,
        input: DeleteDetectorVersionRequest,
    ) -> Result<DeleteDetectorVersionResult, RusotoError<DeleteDetectorVersionError>>;

    /// <p>Deletes the specified event.</p>
    async fn delete_event(
        &self,
        input: DeleteEventRequest,
    ) -> Result<DeleteEventResult, RusotoError<DeleteEventError>>;

    /// <p>Gets all versions for a specified detector.</p>
    async fn describe_detector(
        &self,
        input: DescribeDetectorRequest,
    ) -> Result<DescribeDetectorResult, RusotoError<DescribeDetectorError>>;

    /// <p>Gets all of the model versions for the specified model type or for the specified model type and model ID. You can also get details for a single, specified model version. </p>
    async fn describe_model_versions(
        &self,
        input: DescribeModelVersionsRequest,
    ) -> Result<DescribeModelVersionsResult, RusotoError<DescribeModelVersionsError>>;

    /// <p>Gets a particular detector version. </p>
    async fn get_detector_version(
        &self,
        input: GetDetectorVersionRequest,
    ) -> Result<GetDetectorVersionResult, RusotoError<GetDetectorVersionError>>;

    /// <p>Gets all of detectors. This is a paginated API. If you provide a null <code>maxSizePerPage</code>, this actions retrieves a maximum of 10 records per page. If you provide a <code>maxSizePerPage</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetEventTypesResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_detectors(
        &self,
        input: GetDetectorsRequest,
    ) -> Result<GetDetectorsResult, RusotoError<GetDetectorsError>>;

    /// <p>Gets the details for one or more Amazon SageMaker models that have been imported into the service. This is a paginated API. If you provide a null <code>maxSizePerPage</code>, this actions retrieves a maximum of 10 records per page. If you provide a <code>maxSizePerPage</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetExternalModelsResult</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_external_models(
        &self,
        input: GetExternalModelsRequest,
    ) -> Result<GetExternalModelsResult, RusotoError<GetExternalModelsError>>;

    /// <p>Gets a model version. </p>
    async fn get_model_version(
        &self,
        input: GetModelVersionRequest,
    ) -> Result<GetModelVersionResult, RusotoError<GetModelVersionError>>;

    /// <p>Gets all of the models for the AWS account, or the specified model type, or gets a single model for the specified model type, model ID combination. </p>
    async fn get_models(
        &self,
        input: GetModelsRequest,
    ) -> Result<GetModelsResult, RusotoError<GetModelsError>>;

    /// <p>Gets one or more outcomes. This is a paginated API. If you provide a null <code>maxSizePerPage</code>, this actions retrieves a maximum of 10 records per page. If you provide a <code>maxSizePerPage</code>, the value must be between 50 and 100. To get the next page results, provide the pagination token from the <code>GetOutcomesResult</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_outcomes(
        &self,
        input: GetOutcomesRequest,
    ) -> Result<GetOutcomesResult, RusotoError<GetOutcomesError>>;

    /// <p>Evaluates an event against a detector version. If a version ID is not provided, the detector’s (<code>ACTIVE</code>) version is used. </p>
    async fn get_prediction(
        &self,
        input: GetPredictionRequest,
    ) -> Result<GetPredictionResult, RusotoError<GetPredictionError>>;

    /// <p>Gets all rules available for the specified detector.</p>
    async fn get_rules(
        &self,
        input: GetRulesRequest,
    ) -> Result<GetRulesResult, RusotoError<GetRulesError>>;

    /// <p>Gets all of the variables or the specific variable. This is a paginated API. Providing null <code>maxSizePerPage</code> results in retrieving maximum of 100 records per page. If you provide <code>maxSizePerPage</code> the value must be between 50 and 100. To get the next page result, a provide a pagination token from <code>GetVariablesResult</code> as part of your request. Null pagination token fetches the records from the beginning. </p>
    async fn get_variables(
        &self,
        input: GetVariablesRequest,
    ) -> Result<GetVariablesResult, RusotoError<GetVariablesError>>;

    /// <p>Creates or updates a detector. </p>
    async fn put_detector(
        &self,
        input: PutDetectorRequest,
    ) -> Result<PutDetectorResult, RusotoError<PutDetectorError>>;

    /// <p>Creates or updates an Amazon SageMaker model endpoint. You can also use this action to update the configuration of the model endpoint, including the IAM role and/or the mapped variables. </p>
    async fn put_external_model(
        &self,
        input: PutExternalModelRequest,
    ) -> Result<PutExternalModelResult, RusotoError<PutExternalModelError>>;

    /// <p>Creates or updates a model. </p>
    async fn put_model(
        &self,
        input: PutModelRequest,
    ) -> Result<PutModelResult, RusotoError<PutModelError>>;

    /// <p>Creates or updates an outcome. </p>
    async fn put_outcome(
        &self,
        input: PutOutcomeRequest,
    ) -> Result<PutOutcomeResult, RusotoError<PutOutcomeError>>;

    /// <p> Updates a detector version. The detector version attributes that you can update include models, external model endpoints, rules, and description. You can only update a <code>DRAFT</code> detector version.</p>
    async fn update_detector_version(
        &self,
        input: UpdateDetectorVersionRequest,
    ) -> Result<UpdateDetectorVersionResult, RusotoError<UpdateDetectorVersionError>>;

    /// <p>Updates the detector version's description. You can update the metadata for any detector version (<code>DRAFT, ACTIVE,</code> or <code>INACTIVE</code>). </p>
    async fn update_detector_version_metadata(
        &self,
        input: UpdateDetectorVersionMetadataRequest,
    ) -> Result<UpdateDetectorVersionMetadataResult, RusotoError<UpdateDetectorVersionMetadataError>>;

    /// <p>Updates the detector version’s status. You can perform the following promotions or demotions using <code>UpdateDetectorVersionStatus</code>: <code>DRAFT</code> to <code>ACTIVE</code>, <code>ACTIVE</code> to <code>INACTIVE</code>, and <code>INACTIVE</code> to <code>ACTIVE</code>.</p>
    async fn update_detector_version_status(
        &self,
        input: UpdateDetectorVersionStatusRequest,
    ) -> Result<UpdateDetectorVersionStatusResult, RusotoError<UpdateDetectorVersionStatusError>>;

    /// <p><p>Updates a model version. You can update the description and status attributes using this action. You can perform the following status updates: </p> <ol> <li> <p>Change the <code>TRAINING<em>COMPLETE</code> status to <code>ACTIVE</code> </p> </li> <li> <p>Change <code>ACTIVE</code> back to <code>TRAINING</em>COMPLETE</code> </p> </li> </ol></p>
    async fn update_model_version(
        &self,
        input: UpdateModelVersionRequest,
    ) -> Result<UpdateModelVersionResult, RusotoError<UpdateModelVersionError>>;

    /// <p>Updates a rule's metadata. </p>
    async fn update_rule_metadata(
        &self,
        input: UpdateRuleMetadataRequest,
    ) -> Result<UpdateRuleMetadataResult, RusotoError<UpdateRuleMetadataError>>;

    /// <p>Updates a rule version resulting in a new rule version. </p>
    async fn update_rule_version(
        &self,
        input: UpdateRuleVersionRequest,
    ) -> Result<UpdateRuleVersionResult, RusotoError<UpdateRuleVersionError>>;

    /// <p>Updates a variable.</p>
    async fn update_variable(
        &self,
        input: UpdateVariableRequest,
    ) -> Result<UpdateVariableResult, RusotoError<UpdateVariableError>>;
}
/// A client for the Amazon Fraud Detector API.
#[derive(Clone)]
pub struct FraudDetectorClient {
    client: Client,
    region: region::Region,
}

impl FraudDetectorClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> FraudDetectorClient {
        FraudDetectorClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> FraudDetectorClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        FraudDetectorClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> FraudDetectorClient {
        FraudDetectorClient { client, region }
    }
}

#[async_trait]
impl FraudDetector for FraudDetectorClient {
    /// <p>Creates a batch of variables.</p>
    async fn batch_create_variable(
        &self,
        input: BatchCreateVariableRequest,
    ) -> Result<BatchCreateVariableResult, RusotoError<BatchCreateVariableError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.BatchCreateVariable",
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
                .deserialize::<BatchCreateVariableResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchCreateVariableError::from_response(response))
        }
    }

    /// <p>Gets a batch of variables.</p>
    async fn batch_get_variable(
        &self,
        input: BatchGetVariableRequest,
    ) -> Result<BatchGetVariableResult, RusotoError<BatchGetVariableError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.BatchGetVariable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<BatchGetVariableResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchGetVariableError::from_response(response))
        }
    }

    /// <p>Creates a detector version. The detector version starts in a <code>DRAFT</code> status.</p>
    async fn create_detector_version(
        &self,
        input: CreateDetectorVersionRequest,
    ) -> Result<CreateDetectorVersionResult, RusotoError<CreateDetectorVersionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.CreateDetectorVersion",
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
                .deserialize::<CreateDetectorVersionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDetectorVersionError::from_response(response))
        }
    }

    /// <p>Creates a version of the model using the specified model type. </p>
    async fn create_model_version(
        &self,
        input: CreateModelVersionRequest,
    ) -> Result<CreateModelVersionResult, RusotoError<CreateModelVersionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.CreateModelVersion",
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
                .deserialize::<CreateModelVersionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateModelVersionError::from_response(response))
        }
    }

    /// <p>Creates a rule for use with the specified detector. </p>
    async fn create_rule(
        &self,
        input: CreateRuleRequest,
    ) -> Result<CreateRuleResult, RusotoError<CreateRuleError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.CreateRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateRuleResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateRuleError::from_response(response))
        }
    }

    /// <p>Creates a variable.</p>
    async fn create_variable(
        &self,
        input: CreateVariableRequest,
    ) -> Result<CreateVariableResult, RusotoError<CreateVariableError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.CreateVariable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateVariableResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVariableError::from_response(response))
        }
    }

    /// <p>Deletes the detector version.</p>
    async fn delete_detector_version(
        &self,
        input: DeleteDetectorVersionRequest,
    ) -> Result<DeleteDetectorVersionResult, RusotoError<DeleteDetectorVersionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.DeleteDetectorVersion",
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
                .deserialize::<DeleteDetectorVersionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDetectorVersionError::from_response(response))
        }
    }

    /// <p>Deletes the specified event.</p>
    async fn delete_event(
        &self,
        input: DeleteEventRequest,
    ) -> Result<DeleteEventResult, RusotoError<DeleteEventError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteEvent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteEventResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEventError::from_response(response))
        }
    }

    /// <p>Gets all versions for a specified detector.</p>
    async fn describe_detector(
        &self,
        input: DescribeDetectorRequest,
    ) -> Result<DescribeDetectorResult, RusotoError<DescribeDetectorError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DescribeDetector");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeDetectorResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDetectorError::from_response(response))
        }
    }

    /// <p>Gets all of the model versions for the specified model type or for the specified model type and model ID. You can also get details for a single, specified model version. </p>
    async fn describe_model_versions(
        &self,
        input: DescribeModelVersionsRequest,
    ) -> Result<DescribeModelVersionsResult, RusotoError<DescribeModelVersionsError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.DescribeModelVersions",
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
                .deserialize::<DescribeModelVersionsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeModelVersionsError::from_response(response))
        }
    }

    /// <p>Gets a particular detector version. </p>
    async fn get_detector_version(
        &self,
        input: GetDetectorVersionRequest,
    ) -> Result<GetDetectorVersionResult, RusotoError<GetDetectorVersionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.GetDetectorVersion",
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
                .deserialize::<GetDetectorVersionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDetectorVersionError::from_response(response))
        }
    }

    /// <p>Gets all of detectors. This is a paginated API. If you provide a null <code>maxSizePerPage</code>, this actions retrieves a maximum of 10 records per page. If you provide a <code>maxSizePerPage</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetEventTypesResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_detectors(
        &self,
        input: GetDetectorsRequest,
    ) -> Result<GetDetectorsResult, RusotoError<GetDetectorsError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetDetectors");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetDetectorsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDetectorsError::from_response(response))
        }
    }

    /// <p>Gets the details for one or more Amazon SageMaker models that have been imported into the service. This is a paginated API. If you provide a null <code>maxSizePerPage</code>, this actions retrieves a maximum of 10 records per page. If you provide a <code>maxSizePerPage</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetExternalModelsResult</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_external_models(
        &self,
        input: GetExternalModelsRequest,
    ) -> Result<GetExternalModelsResult, RusotoError<GetExternalModelsError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.GetExternalModels",
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
            proto::json::ResponsePayload::new(&response).deserialize::<GetExternalModelsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetExternalModelsError::from_response(response))
        }
    }

    /// <p>Gets a model version. </p>
    async fn get_model_version(
        &self,
        input: GetModelVersionRequest,
    ) -> Result<GetModelVersionResult, RusotoError<GetModelVersionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetModelVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetModelVersionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetModelVersionError::from_response(response))
        }
    }

    /// <p>Gets all of the models for the AWS account, or the specified model type, or gets a single model for the specified model type, model ID combination. </p>
    async fn get_models(
        &self,
        input: GetModelsRequest,
    ) -> Result<GetModelsResult, RusotoError<GetModelsError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetModels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetModelsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetModelsError::from_response(response))
        }
    }

    /// <p>Gets one or more outcomes. This is a paginated API. If you provide a null <code>maxSizePerPage</code>, this actions retrieves a maximum of 10 records per page. If you provide a <code>maxSizePerPage</code>, the value must be between 50 and 100. To get the next page results, provide the pagination token from the <code>GetOutcomesResult</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_outcomes(
        &self,
        input: GetOutcomesRequest,
    ) -> Result<GetOutcomesResult, RusotoError<GetOutcomesError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetOutcomes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetOutcomesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetOutcomesError::from_response(response))
        }
    }

    /// <p>Evaluates an event against a detector version. If a version ID is not provided, the detector’s (<code>ACTIVE</code>) version is used. </p>
    async fn get_prediction(
        &self,
        input: GetPredictionRequest,
    ) -> Result<GetPredictionResult, RusotoError<GetPredictionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetPrediction");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetPredictionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetPredictionError::from_response(response))
        }
    }

    /// <p>Gets all rules available for the specified detector.</p>
    async fn get_rules(
        &self,
        input: GetRulesRequest,
    ) -> Result<GetRulesResult, RusotoError<GetRulesError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetRulesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetRulesError::from_response(response))
        }
    }

    /// <p>Gets all of the variables or the specific variable. This is a paginated API. Providing null <code>maxSizePerPage</code> results in retrieving maximum of 100 records per page. If you provide <code>maxSizePerPage</code> the value must be between 50 and 100. To get the next page result, a provide a pagination token from <code>GetVariablesResult</code> as part of your request. Null pagination token fetches the records from the beginning. </p>
    async fn get_variables(
        &self,
        input: GetVariablesRequest,
    ) -> Result<GetVariablesResult, RusotoError<GetVariablesError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetVariables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetVariablesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetVariablesError::from_response(response))
        }
    }

    /// <p>Creates or updates a detector. </p>
    async fn put_detector(
        &self,
        input: PutDetectorRequest,
    ) -> Result<PutDetectorResult, RusotoError<PutDetectorError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutDetector");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutDetectorResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutDetectorError::from_response(response))
        }
    }

    /// <p>Creates or updates an Amazon SageMaker model endpoint. You can also use this action to update the configuration of the model endpoint, including the IAM role and/or the mapped variables. </p>
    async fn put_external_model(
        &self,
        input: PutExternalModelRequest,
    ) -> Result<PutExternalModelResult, RusotoError<PutExternalModelError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutExternalModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutExternalModelResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutExternalModelError::from_response(response))
        }
    }

    /// <p>Creates or updates a model. </p>
    async fn put_model(
        &self,
        input: PutModelRequest,
    ) -> Result<PutModelResult, RusotoError<PutModelError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutModelResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutModelError::from_response(response))
        }
    }

    /// <p>Creates or updates an outcome. </p>
    async fn put_outcome(
        &self,
        input: PutOutcomeRequest,
    ) -> Result<PutOutcomeResult, RusotoError<PutOutcomeError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutOutcome");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<PutOutcomeResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutOutcomeError::from_response(response))
        }
    }

    /// <p> Updates a detector version. The detector version attributes that you can update include models, external model endpoints, rules, and description. You can only update a <code>DRAFT</code> detector version.</p>
    async fn update_detector_version(
        &self,
        input: UpdateDetectorVersionRequest,
    ) -> Result<UpdateDetectorVersionResult, RusotoError<UpdateDetectorVersionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateDetectorVersion",
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
                .deserialize::<UpdateDetectorVersionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDetectorVersionError::from_response(response))
        }
    }

    /// <p>Updates the detector version's description. You can update the metadata for any detector version (<code>DRAFT, ACTIVE,</code> or <code>INACTIVE</code>). </p>
    async fn update_detector_version_metadata(
        &self,
        input: UpdateDetectorVersionMetadataRequest,
    ) -> Result<UpdateDetectorVersionMetadataResult, RusotoError<UpdateDetectorVersionMetadataError>>
    {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateDetectorVersionMetadata",
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
                .deserialize::<UpdateDetectorVersionMetadataResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDetectorVersionMetadataError::from_response(response))
        }
    }

    /// <p>Updates the detector version’s status. You can perform the following promotions or demotions using <code>UpdateDetectorVersionStatus</code>: <code>DRAFT</code> to <code>ACTIVE</code>, <code>ACTIVE</code> to <code>INACTIVE</code>, and <code>INACTIVE</code> to <code>ACTIVE</code>.</p>
    async fn update_detector_version_status(
        &self,
        input: UpdateDetectorVersionStatusRequest,
    ) -> Result<UpdateDetectorVersionStatusResult, RusotoError<UpdateDetectorVersionStatusError>>
    {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateDetectorVersionStatus",
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
                .deserialize::<UpdateDetectorVersionStatusResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateDetectorVersionStatusError::from_response(response))
        }
    }

    /// <p><p>Updates a model version. You can update the description and status attributes using this action. You can perform the following status updates: </p> <ol> <li> <p>Change the <code>TRAINING<em>COMPLETE</code> status to <code>ACTIVE</code> </p> </li> <li> <p>Change <code>ACTIVE</code> back to <code>TRAINING</em>COMPLETE</code> </p> </li> </ol></p>
    async fn update_model_version(
        &self,
        input: UpdateModelVersionRequest,
    ) -> Result<UpdateModelVersionResult, RusotoError<UpdateModelVersionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateModelVersion",
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
                .deserialize::<UpdateModelVersionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateModelVersionError::from_response(response))
        }
    }

    /// <p>Updates a rule's metadata. </p>
    async fn update_rule_metadata(
        &self,
        input: UpdateRuleMetadataRequest,
    ) -> Result<UpdateRuleMetadataResult, RusotoError<UpdateRuleMetadataError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateRuleMetadata",
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
                .deserialize::<UpdateRuleMetadataResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRuleMetadataError::from_response(response))
        }
    }

    /// <p>Updates a rule version resulting in a new rule version. </p>
    async fn update_rule_version(
        &self,
        input: UpdateRuleVersionRequest,
    ) -> Result<UpdateRuleVersionResult, RusotoError<UpdateRuleVersionError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateRuleVersion",
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
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateRuleVersionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRuleVersionError::from_response(response))
        }
    }

    /// <p>Updates a variable.</p>
    async fn update_variable(
        &self,
        input: UpdateVariableRequest,
    ) -> Result<UpdateVariableResult, RusotoError<UpdateVariableError>> {
        let mut request = SignedRequest::new("POST", "frauddetector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.UpdateVariable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateVariableResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVariableError::from_response(response))
        }
    }
}
