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

impl FraudDetectorClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "frauddetector", &self.region, request_uri);

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

use serde_json;
/// <p>Provides the error of the batch create variable API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FraudDetectorBatchCreateVariableError {
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchCreateVariableRequest {
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The list of variables for the batch create variable request.</p>
    #[serde(rename = "variableEntries")]
    pub variable_entries: Vec<VariableEntry>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchCreateVariableResult {
    /// <p>Provides the errors for the <code>BatchCreateVariable</code> request.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FraudDetectorBatchCreateVariableError>>,
}

/// <p>Provides the error of the batch get variable API.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FraudDetectorBatchGetVariableError {
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchGetVariableRequest {
    /// <p>The list of variable names to get.</p>
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchGetVariableResult {
    /// <p>The errors from the request.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FraudDetectorBatchGetVariableError>>,
    /// <p>The returned variables.</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<Variable>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>The rule execution mode for the rules included in the detector version.</p> <p>You can define and edit the rule mode at the detector version level, when it is in draft status.</p> <p>If you specify <code>FIRST_MATCHED</code>, Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p> <p>If you specifiy <code>ALL_MATCHED</code>, Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. </p> <p>The default behavior is <code>FIRST_MATCHED</code>.</p>
    #[serde(rename = "ruleExecutionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_execution_mode: Option<RuleExecutionMode>,
    /// <p>The rules to include in the detector version.</p>
    #[serde(rename = "rules")]
    pub rules: Vec<Rule>,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub status: Option<DetectorVersionStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateModelRequest {
    /// <p>The model description. </p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the event type.</p>
    #[serde(rename = "eventTypeName")]
    pub event_type_name: String,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type. </p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateModelResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateModelVersionRequest {
    /// <p>Details for the external events data used for model version training. Required if <code>trainingDataSource</code> is <code>EXTERNAL_EVENTS</code>.</p>
    #[serde(rename = "externalEventsDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_events_detail: Option<ExternalEventsDetail>,
    /// <p>The model ID. </p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The training data schema.</p>
    #[serde(rename = "trainingDataSchema")]
    pub training_data_schema: TrainingDataSchema,
    /// <p>The training data source location in Amazon S3. </p>
    #[serde(rename = "trainingDataSource")]
    pub training_data_source: TrainingDataSourceEnum,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateModelVersionResult {
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<ModelTypeEnum>,
    /// <p>The model version number of the model version created.</p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p>The model version status. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    pub language: Language,
    /// <p>The outcome or outcomes returned when the rule expression matches.</p>
    #[serde(rename = "outcomes")]
    pub outcomes: Vec<String>,
    /// <p>The rule ID.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRuleResult {
    /// <p>The created rule.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVariableRequest {
    /// <p>The source of the data.</p>
    #[serde(rename = "dataSource")]
    pub data_source: DataSource,
    /// <p>The data type.</p>
    #[serde(rename = "dataType")]
    pub data_type: DataType,
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
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The variable type. For more information see <a href="https://docs.aws.amazon.com/frauddetector/latest/ug/create-a-variable.html#variable-types">Variable types</a>. </p> <p>Valid Values: <code>AUTH_CODE | AVS | BILLING_ADDRESS_L1 | BILLING_ADDRESS_L2 | BILLING_CITY | BILLING_COUNTRY | BILLING_NAME | BILLING_PHONE | BILLING_STATE | BILLING_ZIP | CARD_BIN | CATEGORICAL | CURRENCY_CODE | EMAIL_ADDRESS | FINGERPRINT | FRAUD_LABEL | FREE_FORM_TEXT | IP_ADDRESS | NUMERIC | ORDER_ID | PAYMENT_TYPE | PHONE_NUMBER | PRICE | PRODUCT_CATEGORY | SHIPPING_ADDRESS_L1 | SHIPPING_ADDRESS_L2 | SHIPPING_CITY | SHIPPING_COUNTRY | SHIPPING_NAME | SHIPPING_PHONE | SHIPPING_STATE | SHIPPING_ZIP | USERAGENT</code> </p>
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVariableResult {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDataSource {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DataSource {
    Event,
    ExternalModelScore,
    ModelScore,
    #[doc(hidden)]
    UnknownVariant(UnknownDataSource),
}

impl Default for DataSource {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DataSource {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DataSource {
    fn into(self) -> String {
        match self {
            DataSource::Event => "EVENT".to_string(),
            DataSource::ExternalModelScore => "EXTERNAL_MODEL_SCORE".to_string(),
            DataSource::ModelScore => "MODEL_SCORE".to_string(),
            DataSource::UnknownVariant(UnknownDataSource { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DataSource {
    fn into(self) -> &'a str {
        match self {
            DataSource::Event => &"EVENT",
            DataSource::ExternalModelScore => &"EXTERNAL_MODEL_SCORE",
            DataSource::ModelScore => &"MODEL_SCORE",
            DataSource::UnknownVariant(UnknownDataSource { name: original }) => original,
        }
    }
}

impl From<&str> for DataSource {
    fn from(name: &str) -> Self {
        match name {
            "EVENT" => DataSource::Event,
            "EXTERNAL_MODEL_SCORE" => DataSource::ExternalModelScore,
            "MODEL_SCORE" => DataSource::ModelScore,
            _ => DataSource::UnknownVariant(UnknownDataSource {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DataSource {
    fn from(name: String) -> Self {
        match &*name {
            "EVENT" => DataSource::Event,
            "EXTERNAL_MODEL_SCORE" => DataSource::ExternalModelScore,
            "MODEL_SCORE" => DataSource::ModelScore,
            _ => DataSource::UnknownVariant(UnknownDataSource { name }),
        }
    }
}

impl ::std::str::FromStr for DataSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DataSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DataSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDataType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DataType {
    Boolean,
    Float,
    Integer,
    String,
    #[doc(hidden)]
    UnknownVariant(UnknownDataType),
}

impl Default for DataType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DataType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DataType {
    fn into(self) -> String {
        match self {
            DataType::Boolean => "BOOLEAN".to_string(),
            DataType::Float => "FLOAT".to_string(),
            DataType::Integer => "INTEGER".to_string(),
            DataType::String => "STRING".to_string(),
            DataType::UnknownVariant(UnknownDataType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DataType {
    fn into(self) -> &'a str {
        match self {
            DataType::Boolean => &"BOOLEAN",
            DataType::Float => &"FLOAT",
            DataType::Integer => &"INTEGER",
            DataType::String => &"STRING",
            DataType::UnknownVariant(UnknownDataType { name: original }) => original,
        }
    }
}

impl From<&str> for DataType {
    fn from(name: &str) -> Self {
        match name {
            "BOOLEAN" => DataType::Boolean,
            "FLOAT" => DataType::Float,
            "INTEGER" => DataType::Integer,
            "STRING" => DataType::String,
            _ => DataType::UnknownVariant(UnknownDataType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DataType {
    fn from(name: String) -> Self {
        match &*name {
            "BOOLEAN" => DataType::Boolean,
            "FLOAT" => DataType::Float,
            "INTEGER" => DataType::Integer,
            "STRING" => DataType::String,
            _ => DataType::UnknownVariant(UnknownDataType { name }),
        }
    }
}

impl ::std::str::FromStr for DataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The model training validation messages.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataValidationMetrics {
    /// <p>The field-specific model training validation messages.</p>
    #[serde(rename = "fieldLevelMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_messages: Option<Vec<FieldValidationMessage>>,
    /// <p>The file-specific model training validation messages.</p>
    #[serde(rename = "fileLevelMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_level_messages: Option<Vec<FileValidationMessage>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDetectorRequest {
    /// <p>The ID of the detector to delete.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDetectorResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDetectorVersionRequest {
    /// <p>The ID of the parent detector for the detector version to delete.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The ID of the detector version to delete.</p>
    #[serde(rename = "detectorVersionId")]
    pub detector_version_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDetectorVersionResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEntityTypeRequest {
    /// <p>The name of the entity type to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEntityTypeResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventRequest {
    /// <p>The ID of the event to delete.</p>
    #[serde(rename = "eventId")]
    pub event_id: String,
    /// <p>The name of the event type.</p>
    #[serde(rename = "eventTypeName")]
    pub event_type_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEventResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventTypeRequest {
    /// <p>The name of the event type to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEventTypeResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteExternalModelRequest {
    /// <p>The endpoint of the Amazon Sagemaker model to delete.</p>
    #[serde(rename = "modelEndpoint")]
    pub model_endpoint: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteExternalModelResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLabelRequest {
    /// <p>The name of the label to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLabelResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteModelRequest {
    /// <p>The model ID of the model to delete.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type of the model to delete.</p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteModelResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteModelVersionRequest {
    /// <p>The model ID of the model version to delete.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type of the model version to delete.</p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
    /// <p>The model version number of the model version to delete.</p>
    #[serde(rename = "modelVersionNumber")]
    pub model_version_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteModelVersionResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOutcomeRequest {
    /// <p>The name of the outcome to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteOutcomeResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRuleRequest {
    #[serde(rename = "rule")]
    pub rule: Rule,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRuleResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVariableRequest {
    /// <p>The name of the variable to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVariableResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDetectorResult {
    /// <p>The detector ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    pub model_type: Option<ModelTypeEnum>,
    /// <p>The model version number.</p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p>The next token from the previous results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Detector {
    /// <p>The detector ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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
    /// <p>The name of the event type.</p>
    #[serde(rename = "eventTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_name: Option<String>,
    /// <p>Timestamp of when the detector was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDetectorVersionStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DetectorVersionStatus {
    Active,
    Draft,
    Inactive,
    #[doc(hidden)]
    UnknownVariant(UnknownDetectorVersionStatus),
}

impl Default for DetectorVersionStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DetectorVersionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DetectorVersionStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DetectorVersionStatus {
    fn into(self) -> String {
        match self {
            DetectorVersionStatus::Active => "ACTIVE".to_string(),
            DetectorVersionStatus::Draft => "DRAFT".to_string(),
            DetectorVersionStatus::Inactive => "INACTIVE".to_string(),
            DetectorVersionStatus::UnknownVariant(UnknownDetectorVersionStatus {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DetectorVersionStatus {
    fn into(self) -> &'a str {
        match self {
            DetectorVersionStatus::Active => &"ACTIVE",
            DetectorVersionStatus::Draft => &"DRAFT",
            DetectorVersionStatus::Inactive => &"INACTIVE",
            DetectorVersionStatus::UnknownVariant(UnknownDetectorVersionStatus {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DetectorVersionStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => DetectorVersionStatus::Active,
            "DRAFT" => DetectorVersionStatus::Draft,
            "INACTIVE" => DetectorVersionStatus::Inactive,
            _ => DetectorVersionStatus::UnknownVariant(UnknownDetectorVersionStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DetectorVersionStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => DetectorVersionStatus::Active,
            "DRAFT" => DetectorVersionStatus::Draft,
            "INACTIVE" => DetectorVersionStatus::Inactive,
            _ => DetectorVersionStatus::UnknownVariant(UnknownDetectorVersionStatus { name }),
        }
    }
}

impl ::std::str::FromStr for DetectorVersionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DetectorVersionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DetectorVersionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The summary of the detector version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    pub status: Option<DetectorVersionStatus>,
}

/// <p>The entity details. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Entity {
    /// <p>The entity ID. If you do not know the <code>entityId</code>, you can pass <code>unknown</code>, which is areserved string literal.</p>
    #[serde(rename = "entityId")]
    pub entity_id: String,
    /// <p>The entity type.</p>
    #[serde(rename = "entityType")]
    pub entity_type: String,
}

/// <p>The entity type details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntityType {
    /// <p>The entity type ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Timestamp of when the entity type was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The entity type description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Timestamp of when the entity type was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The entity type name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The event type details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventType {
    /// <p>The entity type ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Timestamp of when the event type was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The event type description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The event type entity types.</p>
    #[serde(rename = "entityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<String>>,
    /// <p>The event type event variables.</p>
    #[serde(rename = "eventVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_variables: Option<Vec<String>>,
    /// <p>The event type labels.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>Timestamp of when the event type was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The event type name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Details for the external events data used for model version training.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExternalEventsDetail {
    /// <p>The ARN of the role that provides Amazon Fraud Detector access to the data location.</p>
    #[serde(rename = "dataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>The Amazon S3 bucket location for the data.</p>
    #[serde(rename = "dataLocation")]
    pub data_location: String,
}

/// <p>The Amazon SageMaker model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExternalModel {
    /// <p>The model ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Timestamp of when the model was last created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The input configuration.</p>
    #[serde(rename = "inputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_configuration: Option<ModelInputConfiguration>,
    /// <p>The role used to invoke the model. </p>
    #[serde(rename = "invokeModelEndpointRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_model_endpoint_role_arn: Option<String>,
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
    pub model_endpoint_status: Option<ModelEndpointStatus>,
    /// <p>The source of the model.</p>
    #[serde(rename = "modelSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_source: Option<ModelSource>,
    /// <p>The output configuration.</p>
    #[serde(rename = "outputConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_configuration: Option<ModelOutputConfiguration>,
}

/// <p>The message details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FieldValidationMessage {
    /// <p>The message content.</p>
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The field name.</p>
    #[serde(rename = "fieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// <p>The message ID.</p>
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// <p>The message title.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The message type.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The message details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileValidationMessage {
    /// <p>The message content.</p>
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>The message title.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The message type.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDetectorVersionRequest {
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The detector version ID.</p>
    #[serde(rename = "detectorVersionId")]
    pub detector_version_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDetectorVersionResult {
    /// <p>The detector version ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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
    /// <p>The execution mode of the rule in the dectector</p> <p> <code>FIRST_MATCHED</code> indicates that Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p> <p> <code>ALL_MATCHED</code> indicates that Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. You can define and edit the rule mode at the detector version level, when it is in draft status.</p>
    #[serde(rename = "ruleExecutionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_execution_mode: Option<RuleExecutionMode>,
    /// <p>The rules included in the detector version.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p>The status of the detector version.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DetectorVersionStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEntityTypesRequest {
    /// <p>The maximum number of objects to return for the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The next token for the subsequent request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEntityTypesResult {
    /// <p>An array of entity types.</p>
    #[serde(rename = "entityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<EntityType>>,
    /// <p>The next page token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEventPredictionRequest {
    /// <p>The detector ID.</p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The detector version ID.</p>
    #[serde(rename = "detectorVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_version_id: Option<String>,
    /// <p>The entity type (associated with the detector's event type) and specific entity ID representing who performed the event. If an entity id is not available, use "UNKNOWN."</p>
    #[serde(rename = "entities")]
    pub entities: Vec<Entity>,
    /// <p>The unique ID used to identify the event.</p>
    #[serde(rename = "eventId")]
    pub event_id: String,
    /// <p>Timestamp that defines when the event under evaluation occurred.</p>
    #[serde(rename = "eventTimestamp")]
    pub event_timestamp: String,
    /// <p>The event type associated with the detector specified for the prediction.</p>
    #[serde(rename = "eventTypeName")]
    pub event_type_name: String,
    /// <p>Names of the event type's variables you defined in Amazon Fraud Detector to represent data elements and their corresponding values for the event you are sending for evaluation.</p>
    #[serde(rename = "eventVariables")]
    pub event_variables: ::std::collections::HashMap<String, String>,
    /// <p>The Amazon SageMaker model endpoint input data blobs.</p>
    #[serde(rename = "externalModelEndpointDataBlobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_model_endpoint_data_blobs:
        Option<::std::collections::HashMap<String, ModelEndpointDataBlob>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEventPredictionResult {
    /// <p>The model scores. Amazon Fraud Detector generates model scores between 0 and 1000, where 0 is low fraud risk and 1000 is high fraud risk. Model scores are directly related to the false positive rate (FPR). For example, a score of 600 corresponds to an estimated 10% false positive rate whereas a score of 900 corresponds to an estimated 2% false positive rate.</p>
    #[serde(rename = "modelScores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_scores: Option<Vec<ModelScores>>,
    /// <p>The results.</p>
    #[serde(rename = "ruleResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_results: Option<Vec<RuleResult>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEventTypesRequest {
    /// <p>The maximum number of objects to return for the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The next token for the subsequent request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEventTypesResult {
    /// <p>An array of event types.</p>
    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventType>>,
    /// <p>The next page token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetKMSEncryptionKeyResult {
    /// <p>The KMS encryption key.</p>
    #[serde(rename = "kmsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<KMSKey>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLabelsRequest {
    /// <p>The maximum number of objects to return for the request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the label or labels to get.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The next token for the subsequent request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLabelsResult {
    /// <p>An array of labels.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    /// <p>The next page token.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetModelVersionRequest {
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
    /// <p>The model version number.</p>
    #[serde(rename = "modelVersionNumber")]
    pub model_version_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetModelVersionResult {
    /// <p>The model version ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The event details.</p>
    #[serde(rename = "externalEventsDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_events_detail: Option<ExternalEventsDetail>,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<ModelTypeEnum>,
    /// <p>The model version number.</p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p><p>The model version status.</p> <p>Possible values are:</p> <ul> <li> <p> <code>TRAINING<em>IN</em>PROGRESS</code> </p> </li> <li> <p> <code>TRAINING<em>COMPLETE</code> </p> </li> <li> <p> <code>ACTIVATE</em>REQUESTED</code> </p> </li> <li> <p> <code>ACTIVATE<em>IN</em>PROGRESS</code> </p> </li> <li> <p> <code>ACTIVE</code> </p> </li> <li> <p> <code>INACTIVATE<em>REQUESTED</code> </p> </li> <li> <p> <code>INACTIVATE</em>IN_PROGRESS</code> </p> </li> <li> <p> <code>INACTIVE</code> </p> </li> <li> <p> <code>ERROR</code> </p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The training data schema.</p>
    #[serde(rename = "trainingDataSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_schema: Option<TrainingDataSchema>,
    /// <p>The training data source.</p>
    #[serde(rename = "trainingDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_source: Option<TrainingDataSourceEnum>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetModelsRequest {
    /// <p>The maximum number of objects to return for the request. </p>
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
    pub model_type: Option<ModelTypeEnum>,
    /// <p>The next token for the subsequent request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetModelsResult {
    /// <p>The array of models.</p>
    #[serde(rename = "models")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<Model>>,
    /// <p>The next page token to be used in subsequent requests.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>The KMS key details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KMSKey {
    /// <p>The encryption key ARN.</p>
    #[serde(rename = "kmsEncryptionKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encryption_key_arn: Option<String>,
}

/// <p>The label details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Label {
    /// <p>The label ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Timestamp of when the event type was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The label description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Timestamp of when the label was last updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    /// <p>The label name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The label schema.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LabelSchema {
    /// <p>The label mapper maps the Amazon Fraud Detector supported model classification labels (<code>FRAUD</code>, <code>LEGIT</code>) to the appropriate event type labels. For example, if "<code>FRAUD</code>" and "<code>LEGIT</code>" are Amazon Fraud Detector supported labels, this mapper could be: <code>{"FRAUD" =&gt; ["0"]</code>, <code>"LEGIT" =&gt; ["1"]}</code> or <code>{"FRAUD" =&gt; ["false"]</code>, <code>"LEGIT" =&gt; ["true"]}</code> or <code>{"FRAUD" =&gt; ["fraud", "abuse"]</code>, <code>"LEGIT" =&gt; ["legit", "safe"]}</code>. The value part of the mapper is a list, because you may have multiple label variants from your event type for a single Amazon Fraud Detector label. </p>
    #[serde(rename = "labelMapper")]
    pub label_mapper: ::std::collections::HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownLanguage {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Language {
    Detectorpl,
    #[doc(hidden)]
    UnknownVariant(UnknownLanguage),
}

impl Default for Language {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Language {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Language {
    fn into(self) -> String {
        match self {
            Language::Detectorpl => "DETECTORPL".to_string(),
            Language::UnknownVariant(UnknownLanguage { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Language {
    fn into(self) -> &'a str {
        match self {
            Language::Detectorpl => &"DETECTORPL",
            Language::UnknownVariant(UnknownLanguage { name: original }) => original,
        }
    }
}

impl From<&str> for Language {
    fn from(name: &str) -> Self {
        match name {
            "DETECTORPL" => Language::Detectorpl,
            _ => Language::UnknownVariant(UnknownLanguage {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Language {
    fn from(name: String) -> Self {
        match &*name {
            "DETECTORPL" => Language::Detectorpl,
            _ => Language::UnknownVariant(UnknownLanguage { name }),
        }
    }
}

impl ::std::str::FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The maximum number of objects to return for the request. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The next token from the previous results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN that specifies the resource whose tags you want to list.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResult {
    /// <p>The next token for subsequent requests. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Model performance metrics data points.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MetricDataPoint {
    /// <p>The false positive rate. This is the percentage of total legitimate events that are incorrectly predicted as fraud.</p>
    #[serde(rename = "fpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpr: Option<f32>,
    /// <p>The percentage of fraud events correctly predicted as fraudulent as compared to all events predicted as fraudulent.</p>
    #[serde(rename = "precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f32>,
    /// <p>The model threshold that specifies an acceptable fraud capture rate. For example, a threshold of 500 means any model score 500 or above is labeled as fraud.</p>
    #[serde(rename = "threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f32>,
    /// <p>The true positive rate. This is the percentage of total fraud the model detects. Also known as capture rate.</p>
    #[serde(rename = "tpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpr: Option<f32>,
}

/// <p>The model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Model {
    /// <p>The ARN of the model.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Timestamp of when the model was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The model description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the event type.</p>
    #[serde(rename = "eventTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_name: Option<String>,
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
    pub model_type: Option<ModelTypeEnum>,
}

/// <p>A pre-formed Amazon SageMaker model input you can include if your detector version includes an imported Amazon SageMaker model endpoint with pass-through input configuration.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownModelEndpointStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ModelEndpointStatus {
    Associated,
    Dissociated,
    #[doc(hidden)]
    UnknownVariant(UnknownModelEndpointStatus),
}

impl Default for ModelEndpointStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ModelEndpointStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ModelEndpointStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ModelEndpointStatus {
    fn into(self) -> String {
        match self {
            ModelEndpointStatus::Associated => "ASSOCIATED".to_string(),
            ModelEndpointStatus::Dissociated => "DISSOCIATED".to_string(),
            ModelEndpointStatus::UnknownVariant(UnknownModelEndpointStatus { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ModelEndpointStatus {
    fn into(self) -> &'a str {
        match self {
            ModelEndpointStatus::Associated => &"ASSOCIATED",
            ModelEndpointStatus::Dissociated => &"DISSOCIATED",
            ModelEndpointStatus::UnknownVariant(UnknownModelEndpointStatus { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ModelEndpointStatus {
    fn from(name: &str) -> Self {
        match name {
            "ASSOCIATED" => ModelEndpointStatus::Associated,
            "DISSOCIATED" => ModelEndpointStatus::Dissociated,
            _ => ModelEndpointStatus::UnknownVariant(UnknownModelEndpointStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ModelEndpointStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ASSOCIATED" => ModelEndpointStatus::Associated,
            "DISSOCIATED" => ModelEndpointStatus::Dissociated,
            _ => ModelEndpointStatus::UnknownVariant(UnknownModelEndpointStatus { name }),
        }
    }
}

impl ::std::str::FromStr for ModelEndpointStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ModelEndpointStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ModelEndpointStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The Amazon SageMaker model input configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ModelInputConfiguration {
    /// <p> Template for constructing the CSV input-data sent to SageMaker. At event-evaluation, the placeholders for variable-names in the template will be replaced with the variable values before being sent to SageMaker. </p>
    #[serde(rename = "csvInputTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_input_template: Option<String>,
    /// <p>The event type name.</p>
    #[serde(rename = "eventTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_name: Option<String>,
    /// <p> The format of the model input configuration. The format differs depending on if it is passed through to SageMaker or constructed by Amazon Fraud Detector.</p>
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ModelInputDataFormat>,
    /// <p> Template for constructing the JSON input-data sent to SageMaker. At event-evaluation, the placeholders for variable names in the template will be replaced with the variable values before being sent to SageMaker. </p>
    #[serde(rename = "jsonInputTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_input_template: Option<String>,
    /// <p>The event variables.</p>
    #[serde(rename = "useEventVariables")]
    pub use_event_variables: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownModelInputDataFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ModelInputDataFormat {
    ApplicationJson,
    TextCsv,
    #[doc(hidden)]
    UnknownVariant(UnknownModelInputDataFormat),
}

impl Default for ModelInputDataFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ModelInputDataFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ModelInputDataFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ModelInputDataFormat {
    fn into(self) -> String {
        match self {
            ModelInputDataFormat::ApplicationJson => "APPLICATION_JSON".to_string(),
            ModelInputDataFormat::TextCsv => "TEXT_CSV".to_string(),
            ModelInputDataFormat::UnknownVariant(UnknownModelInputDataFormat {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ModelInputDataFormat {
    fn into(self) -> &'a str {
        match self {
            ModelInputDataFormat::ApplicationJson => &"APPLICATION_JSON",
            ModelInputDataFormat::TextCsv => &"TEXT_CSV",
            ModelInputDataFormat::UnknownVariant(UnknownModelInputDataFormat {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ModelInputDataFormat {
    fn from(name: &str) -> Self {
        match name {
            "APPLICATION_JSON" => ModelInputDataFormat::ApplicationJson,
            "TEXT_CSV" => ModelInputDataFormat::TextCsv,
            _ => ModelInputDataFormat::UnknownVariant(UnknownModelInputDataFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ModelInputDataFormat {
    fn from(name: String) -> Self {
        match &*name {
            "APPLICATION_JSON" => ModelInputDataFormat::ApplicationJson,
            "TEXT_CSV" => ModelInputDataFormat::TextCsv,
            _ => ModelInputDataFormat::UnknownVariant(UnknownModelInputDataFormat { name }),
        }
    }
}

impl ::std::str::FromStr for ModelInputDataFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ModelInputDataFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ModelInputDataFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides the Amazon Sagemaker model output configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ModelOutputConfiguration {
    /// <p>A map of CSV index values in the SageMaker response to the Amazon Fraud Detector variables. </p>
    #[serde(rename = "csvIndexToVariableMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_index_to_variable_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>The format of the model output configuration.</p>
    #[serde(rename = "format")]
    pub format: ModelOutputDataFormat,
    /// <p>A map of JSON keys in response from SageMaker to the Amazon Fraud Detector variables. </p>
    #[serde(rename = "jsonKeyToVariableMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_key_to_variable_map: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownModelOutputDataFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ModelOutputDataFormat {
    ApplicationJsonlines,
    TextCsv,
    #[doc(hidden)]
    UnknownVariant(UnknownModelOutputDataFormat),
}

impl Default for ModelOutputDataFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ModelOutputDataFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ModelOutputDataFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ModelOutputDataFormat {
    fn into(self) -> String {
        match self {
            ModelOutputDataFormat::ApplicationJsonlines => "APPLICATION_JSONLINES".to_string(),
            ModelOutputDataFormat::TextCsv => "TEXT_CSV".to_string(),
            ModelOutputDataFormat::UnknownVariant(UnknownModelOutputDataFormat {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ModelOutputDataFormat {
    fn into(self) -> &'a str {
        match self {
            ModelOutputDataFormat::ApplicationJsonlines => &"APPLICATION_JSONLINES",
            ModelOutputDataFormat::TextCsv => &"TEXT_CSV",
            ModelOutputDataFormat::UnknownVariant(UnknownModelOutputDataFormat {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ModelOutputDataFormat {
    fn from(name: &str) -> Self {
        match name {
            "APPLICATION_JSONLINES" => ModelOutputDataFormat::ApplicationJsonlines,
            "TEXT_CSV" => ModelOutputDataFormat::TextCsv,
            _ => ModelOutputDataFormat::UnknownVariant(UnknownModelOutputDataFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ModelOutputDataFormat {
    fn from(name: String) -> Self {
        match &*name {
            "APPLICATION_JSONLINES" => ModelOutputDataFormat::ApplicationJsonlines,
            "TEXT_CSV" => ModelOutputDataFormat::TextCsv,
            _ => ModelOutputDataFormat::UnknownVariant(UnknownModelOutputDataFormat { name }),
        }
    }
}

impl ::std::str::FromStr for ModelOutputDataFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ModelOutputDataFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ModelOutputDataFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The fraud prediction scores.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownModelSource {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ModelSource {
    Sagemaker,
    #[doc(hidden)]
    UnknownVariant(UnknownModelSource),
}

impl Default for ModelSource {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ModelSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ModelSource {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ModelSource {
    fn into(self) -> String {
        match self {
            ModelSource::Sagemaker => "SAGEMAKER".to_string(),
            ModelSource::UnknownVariant(UnknownModelSource { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ModelSource {
    fn into(self) -> &'a str {
        match self {
            ModelSource::Sagemaker => &"SAGEMAKER",
            ModelSource::UnknownVariant(UnknownModelSource { name: original }) => original,
        }
    }
}

impl From<&str> for ModelSource {
    fn from(name: &str) -> Self {
        match name {
            "SAGEMAKER" => ModelSource::Sagemaker,
            _ => ModelSource::UnknownVariant(UnknownModelSource {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ModelSource {
    fn from(name: String) -> Self {
        match &*name {
            "SAGEMAKER" => ModelSource::Sagemaker,
            _ => ModelSource::UnknownVariant(UnknownModelSource { name }),
        }
    }
}

impl ::std::str::FromStr for ModelSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ModelSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ModelSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownModelTypeEnum {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ModelTypeEnum {
    OnlineFraudInsights,
    #[doc(hidden)]
    UnknownVariant(UnknownModelTypeEnum),
}

impl Default for ModelTypeEnum {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ModelTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ModelTypeEnum {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ModelTypeEnum {
    fn into(self) -> String {
        match self {
            ModelTypeEnum::OnlineFraudInsights => "ONLINE_FRAUD_INSIGHTS".to_string(),
            ModelTypeEnum::UnknownVariant(UnknownModelTypeEnum { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ModelTypeEnum {
    fn into(self) -> &'a str {
        match self {
            ModelTypeEnum::OnlineFraudInsights => &"ONLINE_FRAUD_INSIGHTS",
            ModelTypeEnum::UnknownVariant(UnknownModelTypeEnum { name: original }) => original,
        }
    }
}

impl From<&str> for ModelTypeEnum {
    fn from(name: &str) -> Self {
        match name {
            "ONLINE_FRAUD_INSIGHTS" => ModelTypeEnum::OnlineFraudInsights,
            _ => ModelTypeEnum::UnknownVariant(UnknownModelTypeEnum {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ModelTypeEnum {
    fn from(name: String) -> Self {
        match &*name {
            "ONLINE_FRAUD_INSIGHTS" => ModelTypeEnum::OnlineFraudInsights,
            _ => ModelTypeEnum::UnknownVariant(UnknownModelTypeEnum { name }),
        }
    }
}

impl ::std::str::FromStr for ModelTypeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ModelTypeEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ModelTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The model version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ModelVersion {
    /// <p>The model version ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
    /// <p>The model version number.</p>
    #[serde(rename = "modelVersionNumber")]
    pub model_version_number: String,
}

/// <p>The details of the model version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModelVersionDetail {
    /// <p>The model version ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp when the model was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The event details.</p>
    #[serde(rename = "externalEventsDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_events_detail: Option<ExternalEventsDetail>,
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
    pub model_type: Option<ModelTypeEnum>,
    /// <p>The model version number.</p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p>The status of the model version.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The training data schema.</p>
    #[serde(rename = "trainingDataSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_schema: Option<TrainingDataSchema>,
    /// <p>The model version training data source.</p>
    #[serde(rename = "trainingDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_source: Option<TrainingDataSourceEnum>,
    /// <p>The training results.</p>
    #[serde(rename = "trainingResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_result: Option<TrainingResult>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownModelVersionStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ModelVersionStatus {
    Active,
    Inactive,
    #[doc(hidden)]
    UnknownVariant(UnknownModelVersionStatus),
}

impl Default for ModelVersionStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ModelVersionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ModelVersionStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ModelVersionStatus {
    fn into(self) -> String {
        match self {
            ModelVersionStatus::Active => "ACTIVE".to_string(),
            ModelVersionStatus::Inactive => "INACTIVE".to_string(),
            ModelVersionStatus::UnknownVariant(UnknownModelVersionStatus { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ModelVersionStatus {
    fn into(self) -> &'a str {
        match self {
            ModelVersionStatus::Active => &"ACTIVE",
            ModelVersionStatus::Inactive => &"INACTIVE",
            ModelVersionStatus::UnknownVariant(UnknownModelVersionStatus { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ModelVersionStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => ModelVersionStatus::Active,
            "INACTIVE" => ModelVersionStatus::Inactive,
            _ => ModelVersionStatus::UnknownVariant(UnknownModelVersionStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ModelVersionStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => ModelVersionStatus::Active,
            "INACTIVE" => ModelVersionStatus::Inactive,
            _ => ModelVersionStatus::UnknownVariant(UnknownModelVersionStatus { name }),
        }
    }
}

impl ::std::str::FromStr for ModelVersionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ModelVersionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for ModelVersionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The outcome.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Outcome {
    /// <p>The outcome ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDetectorRequest {
    /// <p>The description of the detector.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The detector ID. </p>
    #[serde(rename = "detectorId")]
    pub detector_id: String,
    /// <p>The name of the event type.</p>
    #[serde(rename = "eventTypeName")]
    pub event_type_name: String,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDetectorResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEntityTypeRequest {
    /// <p>The description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the entity type.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEntityTypeResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventTypeRequest {
    /// <p>The description of the event type.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The entity type for the event type. Example entity types: customer, merchant, account.</p>
    #[serde(rename = "entityTypes")]
    pub entity_types: Vec<String>,
    /// <p>The event type variables.</p>
    #[serde(rename = "eventVariables")]
    pub event_variables: Vec<String>,
    /// <p>The event type labels.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The name.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventTypeResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutExternalModelRequest {
    /// <p>The model endpoint input configuration.</p>
    #[serde(rename = "inputConfiguration")]
    pub input_configuration: ModelInputConfiguration,
    /// <p>The IAM role used to invoke the model endpoint.</p>
    #[serde(rename = "invokeModelEndpointRoleArn")]
    pub invoke_model_endpoint_role_arn: String,
    /// <p>The model endpoints name.</p>
    #[serde(rename = "modelEndpoint")]
    pub model_endpoint: String,
    /// <p>The model endpoint’s status in Amazon Fraud Detector.</p>
    #[serde(rename = "modelEndpointStatus")]
    pub model_endpoint_status: ModelEndpointStatus,
    /// <p>The source of the model.</p>
    #[serde(rename = "modelSource")]
    pub model_source: ModelSource,
    /// <p>The model endpoint output configuration.</p>
    #[serde(rename = "outputConfiguration")]
    pub output_configuration: ModelOutputConfiguration,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutExternalModelResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutKMSEncryptionKeyRequest {
    /// <p>The KMS encryption key ARN.</p>
    #[serde(rename = "kmsEncryptionKeyArn")]
    pub kms_encryption_key_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutKMSEncryptionKeyResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLabelRequest {
    /// <p>The label description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The label name.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p/></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutLabelResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutOutcomeRequest {
    /// <p>The outcome description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the outcome.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutOutcomeResult {}

/// <p>A rule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleDetail {
    /// <p>The rule ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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
    pub language: Option<Language>,
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRuleExecutionMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RuleExecutionMode {
    AllMatched,
    FirstMatched,
    #[doc(hidden)]
    UnknownVariant(UnknownRuleExecutionMode),
}

impl Default for RuleExecutionMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RuleExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RuleExecutionMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RuleExecutionMode {
    fn into(self) -> String {
        match self {
            RuleExecutionMode::AllMatched => "ALL_MATCHED".to_string(),
            RuleExecutionMode::FirstMatched => "FIRST_MATCHED".to_string(),
            RuleExecutionMode::UnknownVariant(UnknownRuleExecutionMode { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a RuleExecutionMode {
    fn into(self) -> &'a str {
        match self {
            RuleExecutionMode::AllMatched => &"ALL_MATCHED",
            RuleExecutionMode::FirstMatched => &"FIRST_MATCHED",
            RuleExecutionMode::UnknownVariant(UnknownRuleExecutionMode { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for RuleExecutionMode {
    fn from(name: &str) -> Self {
        match name {
            "ALL_MATCHED" => RuleExecutionMode::AllMatched,
            "FIRST_MATCHED" => RuleExecutionMode::FirstMatched,
            _ => RuleExecutionMode::UnknownVariant(UnknownRuleExecutionMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RuleExecutionMode {
    fn from(name: String) -> Self {
        match &*name {
            "ALL_MATCHED" => RuleExecutionMode::AllMatched,
            "FIRST_MATCHED" => RuleExecutionMode::FirstMatched,
            _ => RuleExecutionMode::UnknownVariant(UnknownRuleExecutionMode { name }),
        }
    }
}

impl ::std::str::FromStr for RuleExecutionMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for RuleExecutionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RuleExecutionMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The rule results.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleResult {
    /// <p>The outcomes of the matched rule, based on the rule execution mode.</p>
    #[serde(rename = "outcomes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Vec<String>>,
    /// <p>The rule ID that was matched, based on the rule execution mode.</p>
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// <p>A key and value pair. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>A tag key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>A value assigned to a tag key.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The resource ARN.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The tags to assign to the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResult {}

/// <p>The training data schema.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TrainingDataSchema {
    #[serde(rename = "labelSchema")]
    pub label_schema: LabelSchema,
    /// <p>The training data schema variables.</p>
    #[serde(rename = "modelVariables")]
    pub model_variables: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTrainingDataSourceEnum {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TrainingDataSourceEnum {
    ExternalEvents,
    #[doc(hidden)]
    UnknownVariant(UnknownTrainingDataSourceEnum),
}

impl Default for TrainingDataSourceEnum {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TrainingDataSourceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TrainingDataSourceEnum {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TrainingDataSourceEnum {
    fn into(self) -> String {
        match self {
            TrainingDataSourceEnum::ExternalEvents => "EXTERNAL_EVENTS".to_string(),
            TrainingDataSourceEnum::UnknownVariant(UnknownTrainingDataSourceEnum {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a TrainingDataSourceEnum {
    fn into(self) -> &'a str {
        match self {
            TrainingDataSourceEnum::ExternalEvents => &"EXTERNAL_EVENTS",
            TrainingDataSourceEnum::UnknownVariant(UnknownTrainingDataSourceEnum {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for TrainingDataSourceEnum {
    fn from(name: &str) -> Self {
        match name {
            "EXTERNAL_EVENTS" => TrainingDataSourceEnum::ExternalEvents,
            _ => TrainingDataSourceEnum::UnknownVariant(UnknownTrainingDataSourceEnum {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TrainingDataSourceEnum {
    fn from(name: String) -> Self {
        match &*name {
            "EXTERNAL_EVENTS" => TrainingDataSourceEnum::ExternalEvents,
            _ => TrainingDataSourceEnum::UnknownVariant(UnknownTrainingDataSourceEnum { name }),
        }
    }
}

impl ::std::str::FromStr for TrainingDataSourceEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for TrainingDataSourceEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TrainingDataSourceEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The training metric details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrainingMetrics {
    /// <p>The area under the curve. This summarizes true positive rate (TPR) and false positive rate (FPR) across all possible model score thresholds. A model with no predictive power has an AUC of 0.5, whereas a perfect model has a score of 1.0.</p>
    #[serde(rename = "auc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auc: Option<f32>,
    /// <p>The data points details.</p>
    #[serde(rename = "metricDataPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data_points: Option<Vec<MetricDataPoint>>,
}

/// <p>The training result details.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrainingResult {
    /// <p>The validation metrics.</p>
    #[serde(rename = "dataValidationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_validation_metrics: Option<DataValidationMetrics>,
    /// <p>The training metric details.</p>
    #[serde(rename = "trainingMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_metrics: Option<TrainingMetrics>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource from which to remove the tag.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The resource ARN.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorVersionMetadataResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>The rule execution mode to add to the detector.</p> <p>If you specify <code>FIRST_MATCHED</code>, Amazon Fraud Detector evaluates rules sequentially, first to last, stopping at the first matched rule. Amazon Fraud dectector then provides the outcomes for that single rule.</p> <p>If you specifiy <code>ALL_MATCHED</code>, Amazon Fraud Detector evaluates all rules and returns the outcomes for all matched rules. You can define and edit the rule mode at the detector version level, when it is in draft status.</p> <p>The default behavior is <code>FIRST_MATCHED</code>.</p>
    #[serde(rename = "ruleExecutionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_execution_mode: Option<RuleExecutionMode>,
    /// <p>The rules to include in the detector version.</p>
    #[serde(rename = "rules")]
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorVersionResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    pub status: DetectorVersionStatus,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDetectorVersionStatusResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateModelRequest {
    /// <p>The new model description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateModelResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateModelVersionRequest {
    /// <p>The event details.</p>
    #[serde(rename = "externalEventsDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_events_detail: Option<ExternalEventsDetail>,
    /// <p>The major version number.</p>
    #[serde(rename = "majorVersionNumber")]
    pub major_version_number: String,
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateModelVersionResult {
    /// <p>The model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<ModelTypeEnum>,
    /// <p>The model version number of the model version updated.</p>
    #[serde(rename = "modelVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version_number: Option<String>,
    /// <p>The status of the updated model version.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateModelVersionStatusRequest {
    /// <p>The model ID of the model version to update.</p>
    #[serde(rename = "modelId")]
    pub model_id: String,
    /// <p>The model type.</p>
    #[serde(rename = "modelType")]
    pub model_type: ModelTypeEnum,
    /// <p>The model version number.</p>
    #[serde(rename = "modelVersionNumber")]
    pub model_version_number: String,
    /// <p>The model version status.</p>
    #[serde(rename = "status")]
    pub status: ModelVersionStatus,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateModelVersionStatusResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRuleMetadataRequest {
    /// <p>The rule description.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The rule to update.</p>
    #[serde(rename = "rule")]
    pub rule: Rule,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRuleMetadataResult {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    pub language: Language,
    /// <p>The outcomes.</p>
    #[serde(rename = "outcomes")]
    pub outcomes: Vec<String>,
    /// <p>The rule to update.</p>
    #[serde(rename = "rule")]
    pub rule: Rule,
    /// <p>The tags to assign to the rule version.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRuleVersionResult {
    /// <p>The new rule version that was created.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p>The variable type. For more information see <a href="https://docs.aws.amazon.com/frauddetector/latest/ug/create-a-variable.html#variable-types">Variable types</a>.</p>
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVariableResult {}

/// <p>The variable.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Variable {
    /// <p>The ARN of the variable.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time when the variable was created.</p>
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// <p>The data source of the variable.</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    /// <p>The data type of the variable. For more information see <a href="https://docs.aws.amazon.com/frauddetector/latest/ug/create-a-variable.html#variable-types">Variable types</a>.</p>
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
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
    /// <p>The variable type of the variable.</p> <p>Valid Values: <code>AUTH_CODE | AVS | BILLING_ADDRESS_L1 | BILLING_ADDRESS_L2 | BILLING_CITY | BILLING_COUNTRY | BILLING_NAME | BILLING_PHONE | BILLING_STATE | BILLING_ZIP | CARD_BIN | CATEGORICAL | CURRENCY_CODE | EMAIL_ADDRESS | FINGERPRINT | FRAUD_LABEL | FREE_FORM_TEXT | IP_ADDRESS | NUMERIC | ORDER_ID | PAYMENT_TYPE | PHONE_NUMBER | PRICE | PRODUCT_CATEGORY | SHIPPING_ADDRESS_L1 | SHIPPING_ADDRESS_L2 | SHIPPING_CITY | SHIPPING_COUNTRY | SHIPPING_NAME | SHIPPING_PHONE | SHIPPING_STATE | SHIPPING_ZIP | USERAGENT </code> </p>
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}

/// <p>A variable in the list of variables for the batch create variable request.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VariableEntry {
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
    /// <p>The description of the variable.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of the variable. For more information see <a href="https://docs.aws.amazon.com/frauddetector/latest/ug/create-a-variable.html#variable-types">Variable types</a>.</p> <p>Valid Values: <code>AUTH_CODE | AVS | BILLING_ADDRESS_L1 | BILLING_ADDRESS_L2 | BILLING_CITY | BILLING_COUNTRY | BILLING_NAME | BILLING_PHONE | BILLING_STATE | BILLING_ZIP | CARD_BIN | CATEGORICAL | CURRENCY_CODE | EMAIL_ADDRESS | FINGERPRINT | FRAUD_LABEL | FREE_FORM_TEXT | IP_ADDRESS | NUMERIC | ORDER_ID | PAYMENT_TYPE | PHONE_NUMBER | PRICE | PRODUCT_CATEGORY | SHIPPING_ADDRESS_L1 | SHIPPING_ADDRESS_L2 | SHIPPING_CITY | SHIPPING_COUNTRY | SHIPPING_NAME | SHIPPING_PHONE | SHIPPING_STATE | SHIPPING_ZIP | USERAGENT </code> </p>
    #[serde(rename = "variableType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}

/// Errors returned by BatchCreateVariable
#[derive(Debug, PartialEq)]
pub enum BatchCreateVariableError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl BatchCreateVariableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchCreateVariableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(BatchCreateVariableError::AccessDenied(err.msg))
                }
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
            BatchCreateVariableError::AccessDenied(ref cause) => write!(f, "{}", cause),
            BatchCreateVariableError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchCreateVariableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchCreateVariableError {}
/// Errors returned by BatchGetVariable
#[derive(Debug, PartialEq)]
pub enum BatchGetVariableError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl BatchGetVariableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetVariableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(BatchGetVariableError::AccessDenied(err.msg))
                }
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
            BatchGetVariableError::AccessDenied(ref cause) => write!(f, "{}", cause),
            BatchGetVariableError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchGetVariableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchGetVariableError {}
/// Errors returned by CreateDetectorVersion
#[derive(Debug, PartialEq)]
pub enum CreateDetectorVersionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDetectorVersionError::AccessDenied(err.msg))
                }
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
            CreateDetectorVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDetectorVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDetectorVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateDetectorVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDetectorVersionError {}
/// Errors returned by CreateModel
#[derive(Debug, PartialEq)]
pub enum CreateModelError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl CreateModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateModelError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateModelError::InternalServer(err.msg))
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
            CreateModelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateModelError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateModelError {}
/// Errors returned by CreateModelVersion
#[derive(Debug, PartialEq)]
pub enum CreateModelVersionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateModelVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateModelVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateModelVersionError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateModelVersionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateModelVersionError::ResourceNotFound(err.msg))
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
            CreateModelVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateModelVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateModelVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateModelVersionError {}
/// Errors returned by CreateRule
#[derive(Debug, PartialEq)]
pub enum CreateRuleError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl CreateRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateRuleError::AccessDenied(err.msg))
                }
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
            CreateRuleError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateRuleError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRuleError {}
/// Errors returned by CreateVariable
#[derive(Debug, PartialEq)]
pub enum CreateVariableError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl CreateVariableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVariableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateVariableError::AccessDenied(err.msg))
                }
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
            CreateVariableError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateVariableError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateVariableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVariableError {}
/// Errors returned by DeleteDetector
#[derive(Debug, PartialEq)]
pub enum DeleteDetectorError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DeleteDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDetectorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDetectorError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteDetectorError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDetectorError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDetectorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDetectorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDetectorError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteDetectorError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDetectorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDetectorError {}
/// Errors returned by DeleteDetectorVersion
#[derive(Debug, PartialEq)]
pub enum DeleteDetectorVersionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDetectorVersionError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteDetectorVersionError::Conflict(err.msg))
                }
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
            DeleteDetectorVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDetectorVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteDetectorVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDetectorVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDetectorVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDetectorVersionError {}
/// Errors returned by DeleteEntityType
#[derive(Debug, PartialEq)]
pub enum DeleteEntityTypeError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl DeleteEntityTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEntityTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteEntityTypeError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteEntityTypeError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteEntityTypeError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEntityTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEntityTypeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteEntityTypeError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteEntityTypeError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEntityTypeError {}
/// Errors returned by DeleteEvent
#[derive(Debug, PartialEq)]
pub enum DeleteEventError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DeleteEventError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteEventError::AccessDenied(err.msg))
                }
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
            DeleteEventError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteEventError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteEventError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventError {}
/// Errors returned by DeleteEventType
#[derive(Debug, PartialEq)]
pub enum DeleteEventTypeError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl DeleteEventTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteEventTypeError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteEventTypeError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteEventTypeError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEventTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventTypeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteEventTypeError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteEventTypeError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventTypeError {}
/// Errors returned by DeleteExternalModel
#[derive(Debug, PartialEq)]
pub enum DeleteExternalModelError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DeleteExternalModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteExternalModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteExternalModelError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteExternalModelError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteExternalModelError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteExternalModelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteExternalModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteExternalModelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteExternalModelError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteExternalModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteExternalModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteExternalModelError {}
/// Errors returned by DeleteLabel
#[derive(Debug, PartialEq)]
pub enum DeleteLabelError {
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl DeleteLabelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLabelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteLabelError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteLabelError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLabelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLabelError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteLabelError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLabelError {}
/// Errors returned by DeleteModel
#[derive(Debug, PartialEq)]
pub enum DeleteModelError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl DeleteModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteModelError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteModelError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteModelError::InternalServer(err.msg))
                }
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
        match *self {
            DeleteModelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteModelError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteModelError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteModelError {}
/// Errors returned by DeleteModelVersion
#[derive(Debug, PartialEq)]
pub enum DeleteModelVersionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl DeleteModelVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteModelVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteModelVersionError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteModelVersionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteModelVersionError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteModelVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteModelVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteModelVersionError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteModelVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteModelVersionError {}
/// Errors returned by DeleteOutcome
#[derive(Debug, PartialEq)]
pub enum DeleteOutcomeError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DeleteOutcomeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOutcomeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteOutcomeError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteOutcomeError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteOutcomeError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteOutcomeError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteOutcomeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOutcomeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteOutcomeError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteOutcomeError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteOutcomeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteOutcomeError {}
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DeleteRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteRuleError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteRuleError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteRuleError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteRuleError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRuleError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRuleError {}
/// Errors returned by DeleteVariable
#[derive(Debug, PartialEq)]
pub enum DeleteVariableError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl DeleteVariableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVariableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteVariableError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteVariableError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteVariableError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteVariableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVariableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVariableError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteVariableError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteVariableError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteVariableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVariableError {}
/// Errors returned by DescribeDetector
#[derive(Debug, PartialEq)]
pub enum DescribeDetectorError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeDetectorError::AccessDenied(err.msg))
                }
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
            DescribeDetectorError::AccessDenied(ref cause) => write!(f, "{}", cause),
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
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeModelVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeModelVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeModelVersionsError::AccessDenied(err.msg))
                }
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
            DescribeModelVersionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeModelVersionsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeModelVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeModelVersionsError {}
/// Errors returned by GetDetectorVersion
#[derive(Debug, PartialEq)]
pub enum GetDetectorVersionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDetectorVersionError::AccessDenied(err.msg))
                }
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
            GetDetectorVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
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
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDetectorsError::AccessDenied(err.msg))
                }
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
            GetDetectorsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDetectorsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetDetectorsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetDetectorsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDetectorsError {}
/// Errors returned by GetEntityTypes
#[derive(Debug, PartialEq)]
pub enum GetEntityTypesError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetEntityTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEntityTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetEntityTypesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetEntityTypesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetEntityTypesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEntityTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEntityTypesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetEntityTypesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEntityTypesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEntityTypesError {}
/// Errors returned by GetEventPrediction
#[derive(Debug, PartialEq)]
pub enum GetEventPredictionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p><p>An exception indicating there was a conflict during a delete operation. The following delete operations can cause a conflict exception:</p> <ul> <li> <p>DeleteDetector: A conflict exception will occur if the detector has associated <code>Rules</code> or <code>DetectorVersions</code>. You can only delete a detector if it has no <code>Rules</code> or <code>DetectorVersions</code>.</p> </li> <li> <p>DeleteDetectorVersion: A conflict exception will occur if the <code>DetectorVersion</code> status is <code>ACTIVE</code>.</p> </li> <li> <p>DeleteRule: A conflict exception will occur if the <code>RuleVersion</code> is in use by an associated <code>ACTIVE</code> or <code>INACTIVE DetectorVersion</code>.</p> </li> </ul></p>
    Conflict(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl GetEventPredictionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEventPredictionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetEventPredictionError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetEventPredictionError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetEventPredictionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetEventPredictionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetEventPredictionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEventPredictionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEventPredictionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetEventPredictionError::Conflict(ref cause) => write!(f, "{}", cause),
            GetEventPredictionError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEventPredictionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetEventPredictionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEventPredictionError {}
/// Errors returned by GetEventTypes
#[derive(Debug, PartialEq)]
pub enum GetEventTypesError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetEventTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEventTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetEventTypesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetEventTypesError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetEventTypesError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEventTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEventTypesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetEventTypesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetEventTypesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEventTypesError {}
/// Errors returned by GetExternalModels
#[derive(Debug, PartialEq)]
pub enum GetExternalModelsError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(GetExternalModelsError::AccessDenied(err.msg))
                }
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
            GetExternalModelsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetExternalModelsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetExternalModelsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetExternalModelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetExternalModelsError {}
/// Errors returned by GetKMSEncryptionKey
#[derive(Debug, PartialEq)]
pub enum GetKMSEncryptionKeyError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetKMSEncryptionKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKMSEncryptionKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetKMSEncryptionKeyError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetKMSEncryptionKeyError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetKMSEncryptionKeyError::ResourceNotFound(
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
impl fmt::Display for GetKMSEncryptionKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKMSEncryptionKeyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetKMSEncryptionKeyError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetKMSEncryptionKeyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKMSEncryptionKeyError {}
/// Errors returned by GetLabels
#[derive(Debug, PartialEq)]
pub enum GetLabelsError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLabelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLabelsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetLabelsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLabelsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLabelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLabelsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLabelsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetLabelsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLabelsError {}
/// Errors returned by GetModelVersion
#[derive(Debug, PartialEq)]
pub enum GetModelVersionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetModelVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetModelVersionError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetModelVersionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetModelVersionError::ResourceNotFound(err.msg))
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
            GetModelVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetModelVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetModelVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetModelVersionError {}
/// Errors returned by GetModels
#[derive(Debug, PartialEq)]
pub enum GetModelsError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl GetModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetModelsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetModelsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetModelsError::ResourceNotFound(err.msg))
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
            GetModelsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetModelsError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetModelsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetModelsError {}
/// Errors returned by GetOutcomes
#[derive(Debug, PartialEq)]
pub enum GetOutcomesError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOutcomesError::AccessDenied(err.msg))
                }
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
            GetOutcomesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOutcomesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetOutcomesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetOutcomesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOutcomesError {}
/// Errors returned by GetRules
#[derive(Debug, PartialEq)]
pub enum GetRulesError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRulesError::AccessDenied(err.msg))
                }
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
            GetRulesError::AccessDenied(ref cause) => write!(f, "{}", cause),
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
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(GetVariablesError::AccessDenied(err.msg))
                }
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
            GetVariablesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetVariablesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetVariablesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetVariablesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVariablesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
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
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutDetector
#[derive(Debug, PartialEq)]
pub enum PutDetectorError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl PutDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDetectorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutDetectorError::AccessDenied(err.msg))
                }
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
            PutDetectorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutDetectorError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutDetectorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutDetectorError {}
/// Errors returned by PutEntityType
#[derive(Debug, PartialEq)]
pub enum PutEntityTypeError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl PutEntityTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEntityTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutEntityTypeError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(PutEntityTypeError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEntityTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEntityTypeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutEntityTypeError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEntityTypeError {}
/// Errors returned by PutEventType
#[derive(Debug, PartialEq)]
pub enum PutEventTypeError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl PutEventTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutEventTypeError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(PutEventTypeError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEventTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventTypeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutEventTypeError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventTypeError {}
/// Errors returned by PutExternalModel
#[derive(Debug, PartialEq)]
pub enum PutExternalModelError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl PutExternalModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutExternalModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutExternalModelError::AccessDenied(err.msg))
                }
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
            PutExternalModelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutExternalModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutExternalModelError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutExternalModelError {}
/// Errors returned by PutKMSEncryptionKey
#[derive(Debug, PartialEq)]
pub enum PutKMSEncryptionKeyError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl PutKMSEncryptionKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutKMSEncryptionKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutKMSEncryptionKeyError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(PutKMSEncryptionKeyError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutKMSEncryptionKeyError::ResourceNotFound(
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
impl fmt::Display for PutKMSEncryptionKeyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutKMSEncryptionKeyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutKMSEncryptionKeyError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutKMSEncryptionKeyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutKMSEncryptionKeyError {}
/// Errors returned by PutLabel
#[derive(Debug, PartialEq)]
pub enum PutLabelError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
}

impl PutLabelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLabelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutLabelError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(PutLabelError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutLabelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLabelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutLabelError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutLabelError {}
/// Errors returned by PutOutcome
#[derive(Debug, PartialEq)]
pub enum PutOutcomeError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating a throttling error.</p>
    Throttling(String),
}

impl PutOutcomeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutOutcomeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutOutcomeError::AccessDenied(err.msg))
                }
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
            PutOutcomeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutOutcomeError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutOutcomeError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutOutcomeError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
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
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
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
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDetectorVersion
#[derive(Debug, PartialEq)]
pub enum UpdateDetectorVersionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDetectorVersionError::AccessDenied(err.msg))
                }
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
            UpdateDetectorVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
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
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDetectorVersionMetadataError::AccessDenied(
                        err.msg,
                    ))
                }
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
            UpdateDetectorVersionMetadataError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionMetadataError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionMetadataError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDetectorVersionMetadataError {}
/// Errors returned by UpdateDetectorVersionStatus
#[derive(Debug, PartialEq)]
pub enum UpdateDetectorVersionStatusError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDetectorVersionStatusError::AccessDenied(
                        err.msg,
                    ))
                }
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
            UpdateDetectorVersionStatusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionStatusError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDetectorVersionStatusError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDetectorVersionStatusError {}
/// Errors returned by UpdateModel
#[derive(Debug, PartialEq)]
pub enum UpdateModelError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateModelError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateModelError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateModelError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateModelError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateModelError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateModelError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateModelError {}
/// Errors returned by UpdateModelVersion
#[derive(Debug, PartialEq)]
pub enum UpdateModelVersionError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateModelVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateModelVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateModelVersionError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateModelVersionError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateModelVersionError::ResourceNotFound(err.msg))
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
            UpdateModelVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateModelVersionError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateModelVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateModelVersionError {}
/// Errors returned by UpdateModelVersionStatus
#[derive(Debug, PartialEq)]
pub enum UpdateModelVersionStatusError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
    /// <p>An exception indicating an internal server error.</p>
    InternalServer(String),
    /// <p>An exception indicating the specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateModelVersionStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateModelVersionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateModelVersionStatusError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateModelVersionStatusError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateModelVersionStatusError::ResourceNotFound(
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
impl fmt::Display for UpdateModelVersionStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateModelVersionStatusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateModelVersionStatusError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateModelVersionStatusError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateModelVersionStatusError {}
/// Errors returned by UpdateRuleMetadata
#[derive(Debug, PartialEq)]
pub enum UpdateRuleMetadataError {
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateRuleMetadataError::AccessDenied(err.msg))
                }
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
            UpdateRuleMetadataError::AccessDenied(ref cause) => write!(f, "{}", cause),
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
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateRuleVersionError::AccessDenied(err.msg))
                }
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
            UpdateRuleVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
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
    /// <p>An exception indicating Amazon Fraud Detector does not have the needed permissions. This can occur if you submit a request, such as <code>PutExternalModel</code>, that specifies a role that is not in your account.</p>
    AccessDenied(String),
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
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateVariableError::AccessDenied(err.msg))
                }
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
            UpdateVariableError::AccessDenied(ref cause) => write!(f, "{}", cause),
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

    /// <p>Creates a model using the specified model type.</p>
    async fn create_model(
        &self,
        input: CreateModelRequest,
    ) -> Result<CreateModelResult, RusotoError<CreateModelError>>;

    /// <p>Creates a version of the model using the specified model type and model id. </p>
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

    /// <p>Deletes the detector. Before deleting a detector, you must first delete all detector versions and rule versions associated with the detector.</p> <p>When you delete a detector, Amazon Fraud Detector permanently deletes the detector and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_detector(
        &self,
        input: DeleteDetectorRequest,
    ) -> Result<DeleteDetectorResult, RusotoError<DeleteDetectorError>>;

    /// <p>Deletes the detector version. You cannot delete detector versions that are in <code>ACTIVE</code> status.</p> <p>When you delete a detector version, Amazon Fraud Detector permanently deletes the detector and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_detector_version(
        &self,
        input: DeleteDetectorVersionRequest,
    ) -> Result<DeleteDetectorVersionResult, RusotoError<DeleteDetectorVersionError>>;

    /// <p>Deletes an entity type.</p> <p>You cannot delete an entity type that is included in an event type.</p> <p>When you delete an entity type, Amazon Fraud Detector permanently deletes that entity type from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_entity_type(
        &self,
        input: DeleteEntityTypeRequest,
    ) -> Result<DeleteEntityTypeResult, RusotoError<DeleteEntityTypeError>>;

    /// <p>Deletes the specified event.</p> <p>When you delete an event, Amazon Fraud Detector permanently deletes that event from the evaluation history, and the event data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_event(
        &self,
        input: DeleteEventRequest,
    ) -> Result<DeleteEventResult, RusotoError<DeleteEventError>>;

    /// <p>Deletes an event type.</p> <p>You cannot delete an event type that is used in a detector or a model.</p> <p>When you delete an entity type, Amazon Fraud Detector permanently deletes that entity type from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_event_type(
        &self,
        input: DeleteEventTypeRequest,
    ) -> Result<DeleteEventTypeResult, RusotoError<DeleteEventTypeError>>;

    /// <p>Removes a SageMaker model from Amazon Fraud Detector.</p> <p>You can remove an Amazon SageMaker model if it is not associated with a detector version. Removing a SageMaker model disconnects it from Amazon Fraud Detector, but the model remains available in SageMaker.</p>
    async fn delete_external_model(
        &self,
        input: DeleteExternalModelRequest,
    ) -> Result<DeleteExternalModelResult, RusotoError<DeleteExternalModelError>>;

    /// <p>Deletes a label.</p> <p>You cannot delete labels that are included in an event type in Amazon Fraud Detector.</p> <p>You cannot delete a label assigned to an event ID. You must first delete the relevant event ID.</p> <p>When you delete a label, Amazon Fraud Detector permanently deletes that label from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_label(
        &self,
        input: DeleteLabelRequest,
    ) -> Result<DeleteLabelResult, RusotoError<DeleteLabelError>>;

    /// <p>Deletes a model.</p> <p>You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version.</p> <p> When you delete a model, Amazon Fraud Detector permanently deletes that model from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_model(
        &self,
        input: DeleteModelRequest,
    ) -> Result<DeleteModelResult, RusotoError<DeleteModelError>>;

    /// <p>Deletes a model version.</p> <p>You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version.</p> <p> When you delete a model version, Amazon Fraud Detector permanently deletes that model version from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_model_version(
        &self,
        input: DeleteModelVersionRequest,
    ) -> Result<DeleteModelVersionResult, RusotoError<DeleteModelVersionError>>;

    /// <p>Deletes an outcome.</p> <p>You cannot delete an outcome that is used in a rule version.</p> <p>When you delete an outcome, Amazon Fraud Detector permanently deletes that outcome from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_outcome(
        &self,
        input: DeleteOutcomeRequest,
    ) -> Result<DeleteOutcomeResult, RusotoError<DeleteOutcomeError>>;

    /// <p>Deletes the rule. You cannot delete a rule if it is used by an <code>ACTIVE</code> or <code>INACTIVE</code> detector version.</p> <p>When you delete a rule, Amazon Fraud Detector permanently deletes that rule from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> Result<DeleteRuleResult, RusotoError<DeleteRuleError>>;

    /// <p>Deletes a variable.</p> <p>You can't delete variables that are included in an event type in Amazon Fraud Detector.</p> <p>Amazon Fraud Detector automatically deletes model output variables and SageMaker model output variables when you delete the model. You can't delete these variables manually.</p> <p>When you delete a variable, Amazon Fraud Detector permanently deletes that variable from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_variable(
        &self,
        input: DeleteVariableRequest,
    ) -> Result<DeleteVariableResult, RusotoError<DeleteVariableError>>;

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

    /// <p>Gets all detectors or a single detector if a <code>detectorId</code> is specified. This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetDetectorsResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_detectors(
        &self,
        input: GetDetectorsRequest,
    ) -> Result<GetDetectorsResult, RusotoError<GetDetectorsError>>;

    /// <p>Gets all entity types or a specific entity type if a name is specified. This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetEntityTypesResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_entity_types(
        &self,
        input: GetEntityTypesRequest,
    ) -> Result<GetEntityTypesResult, RusotoError<GetEntityTypesError>>;

    /// <p>Evaluates an event against a detector version. If a version ID is not provided, the detector’s (<code>ACTIVE</code>) version is used.</p>
    async fn get_event_prediction(
        &self,
        input: GetEventPredictionRequest,
    ) -> Result<GetEventPredictionResult, RusotoError<GetEventPredictionError>>;

    /// <p>Gets all event types or a specific event type if name is provided. This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetEventTypesResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_event_types(
        &self,
        input: GetEventTypesRequest,
    ) -> Result<GetEventTypesResult, RusotoError<GetEventTypesError>>;

    /// <p>Gets the details for one or more Amazon SageMaker models that have been imported into the service. This is a paginated API. If you provide a null <code>maxResults</code>, this actions retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetExternalModelsResult</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_external_models(
        &self,
        input: GetExternalModelsRequest,
    ) -> Result<GetExternalModelsResult, RusotoError<GetExternalModelsError>>;

    /// <p>Gets the encryption key if a Key Management Service (KMS) customer master key (CMK) has been specified to be used to encrypt content in Amazon Fraud Detector.</p>
    async fn get_kms_encryption_key(
        &self,
    ) -> Result<GetKMSEncryptionKeyResult, RusotoError<GetKMSEncryptionKeyError>>;

    /// <p>Gets all labels or a specific label if name is provided. This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 50 records per page. If you provide a <code>maxResults</code>, the value must be between 10 and 50. To get the next page results, provide the pagination token from the <code>GetGetLabelsResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_labels(
        &self,
        input: GetLabelsRequest,
    ) -> Result<GetLabelsResult, RusotoError<GetLabelsError>>;

    /// <p>Gets the details of the specified model version.</p>
    async fn get_model_version(
        &self,
        input: GetModelVersionRequest,
    ) -> Result<GetModelVersionResult, RusotoError<GetModelVersionError>>;

    /// <p>Gets one or more models. Gets all models for the AWS account if no model type and no model id provided. Gets all models for the AWS account and model type, if the model type is specified but model id is not provided. Gets a specific model if (model type, model id) tuple is specified. </p> <p>This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 1 and 10. To get the next page results, provide the pagination token from the response as part of your request. A null pagination token fetches the records from the beginning.</p>
    async fn get_models(
        &self,
        input: GetModelsRequest,
    ) -> Result<GetModelsResult, RusotoError<GetModelsError>>;

    /// <p>Gets one or more outcomes. This is a paginated API. If you provide a null <code>maxResults</code>, this actions retrieves a maximum of 100 records per page. If you provide a <code>maxResults</code>, the value must be between 50 and 100. To get the next page results, provide the pagination token from the <code>GetOutcomesResult</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_outcomes(
        &self,
        input: GetOutcomesRequest,
    ) -> Result<GetOutcomesResult, RusotoError<GetOutcomesError>>;

    /// <p>Get all rules for a detector (paginated) if <code>ruleId</code> and <code>ruleVersion</code> are not specified. Gets all rules for the detector and the <code>ruleId</code> if present (paginated). Gets a specific rule if both the <code>ruleId</code> and the <code>ruleVersion</code> are specified.</p> <p>This is a paginated API. Providing null maxResults results in retrieving maximum of 100 records per page. If you provide maxResults the value must be between 50 and 100. To get the next page result, a provide a pagination token from GetRulesResult as part of your request. Null pagination token fetches the records from the beginning.</p>
    async fn get_rules(
        &self,
        input: GetRulesRequest,
    ) -> Result<GetRulesResult, RusotoError<GetRulesError>>;

    /// <p>Gets all of the variables or the specific variable. This is a paginated API. Providing null <code>maxSizePerPage</code> results in retrieving maximum of 100 records per page. If you provide <code>maxSizePerPage</code> the value must be between 50 and 100. To get the next page result, a provide a pagination token from <code>GetVariablesResult</code> as part of your request. Null pagination token fetches the records from the beginning. </p>
    async fn get_variables(
        &self,
        input: GetVariablesRequest,
    ) -> Result<GetVariablesResult, RusotoError<GetVariablesError>>;

    /// <p>Lists all tags associated with the resource. This is a paginated API. To get the next page results, provide the pagination token from the response as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>>;

    /// <p>Creates or updates a detector. </p>
    async fn put_detector(
        &self,
        input: PutDetectorRequest,
    ) -> Result<PutDetectorResult, RusotoError<PutDetectorError>>;

    /// <p>Creates or updates an entity type. An entity represents who is performing the event. As part of a fraud prediction, you pass the entity ID to indicate the specific entity who performed the event. An entity type classifies the entity. Example classifications include customer, merchant, or account.</p>
    async fn put_entity_type(
        &self,
        input: PutEntityTypeRequest,
    ) -> Result<PutEntityTypeResult, RusotoError<PutEntityTypeError>>;

    /// <p>Creates or updates an event type. An event is a business activity that is evaluated for fraud risk. With Amazon Fraud Detector, you generate fraud predictions for events. An event type defines the structure for an event sent to Amazon Fraud Detector. This includes the variables sent as part of the event, the entity performing the event (such as a customer), and the labels that classify the event. Example event types include online payment transactions, account registrations, and authentications.</p>
    async fn put_event_type(
        &self,
        input: PutEventTypeRequest,
    ) -> Result<PutEventTypeResult, RusotoError<PutEventTypeError>>;

    /// <p>Creates or updates an Amazon SageMaker model endpoint. You can also use this action to update the configuration of the model endpoint, including the IAM role and/or the mapped variables. </p>
    async fn put_external_model(
        &self,
        input: PutExternalModelRequest,
    ) -> Result<PutExternalModelResult, RusotoError<PutExternalModelError>>;

    /// <p>Specifies the Key Management Service (KMS) customer master key (CMK) to be used to encrypt content in Amazon Fraud Detector.</p>
    async fn put_kms_encryption_key(
        &self,
        input: PutKMSEncryptionKeyRequest,
    ) -> Result<PutKMSEncryptionKeyResult, RusotoError<PutKMSEncryptionKeyError>>;

    /// <p>Creates or updates label. A label classifies an event as fraudulent or legitimate. Labels are associated with event types and used to train supervised machine learning models in Amazon Fraud Detector. </p>
    async fn put_label(
        &self,
        input: PutLabelRequest,
    ) -> Result<PutLabelResult, RusotoError<PutLabelError>>;

    /// <p>Creates or updates an outcome. </p>
    async fn put_outcome(
        &self,
        input: PutOutcomeRequest,
    ) -> Result<PutOutcomeResult, RusotoError<PutOutcomeError>>;

    /// <p>Assigns tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResult, RusotoError<TagResourceError>>;

    /// <p>Removes tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResult, RusotoError<UntagResourceError>>;

    /// <p> Updates a detector version. The detector version attributes that you can update include models, external model endpoints, rules, rule execution mode, and description. You can only update a <code>DRAFT</code> detector version.</p>
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

    /// <p>Updates a model. You can update the description attribute using this action.</p>
    async fn update_model(
        &self,
        input: UpdateModelRequest,
    ) -> Result<UpdateModelResult, RusotoError<UpdateModelError>>;

    /// <p>Updates a model version. Updating a model version retrains an existing model version using updated training data and produces a new minor version of the model. You can update the training data set location and data access role attributes using this action. This action creates and trains a new minor version of the model, for example version 1.01, 1.02, 1.03.</p>
    async fn update_model_version(
        &self,
        input: UpdateModelVersionRequest,
    ) -> Result<UpdateModelVersionResult, RusotoError<UpdateModelVersionError>>;

    /// <p><p>Updates the status of a model version.</p> <p>You can perform the following status updates:</p> <ol> <li> <p>Change the <code>TRAINING_COMPLETE</code> status to <code>ACTIVE</code>.</p> </li> <li> <p>Change <code>ACTIVE</code>to <code>INACTIVE</code>.</p> </li> </ol></p>
    async fn update_model_version_status(
        &self,
        input: UpdateModelVersionStatusRequest,
    ) -> Result<UpdateModelVersionStatusResult, RusotoError<UpdateModelVersionStatusError>>;

    /// <p>Updates a rule's metadata. The description attribute can be updated.</p>
    async fn update_rule_metadata(
        &self,
        input: UpdateRuleMetadataRequest,
    ) -> Result<UpdateRuleMetadataResult, RusotoError<UpdateRuleMetadataError>>;

    /// <p>Updates a rule version resulting in a new rule version. Updates a rule version resulting in a new rule version (version 1, 2, 3 ...). </p>
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
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.BatchCreateVariable",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, BatchCreateVariableError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<BatchCreateVariableResult, _>()
    }

    /// <p>Gets a batch of variables.</p>
    async fn batch_get_variable(
        &self,
        input: BatchGetVariableRequest,
    ) -> Result<BatchGetVariableResult, RusotoError<BatchGetVariableError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.BatchGetVariable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, BatchGetVariableError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<BatchGetVariableResult, _>()
    }

    /// <p>Creates a detector version. The detector version starts in a <code>DRAFT</code> status.</p>
    async fn create_detector_version(
        &self,
        input: CreateDetectorVersionRequest,
    ) -> Result<CreateDetectorVersionResult, RusotoError<CreateDetectorVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.CreateDetectorVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDetectorVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDetectorVersionResult, _>()
    }

    /// <p>Creates a model using the specified model type.</p>
    async fn create_model(
        &self,
        input: CreateModelRequest,
    ) -> Result<CreateModelResult, RusotoError<CreateModelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.CreateModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateModelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateModelResult, _>()
    }

    /// <p>Creates a version of the model using the specified model type and model id. </p>
    async fn create_model_version(
        &self,
        input: CreateModelVersionRequest,
    ) -> Result<CreateModelVersionResult, RusotoError<CreateModelVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.CreateModelVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateModelVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateModelVersionResult, _>()
    }

    /// <p>Creates a rule for use with the specified detector. </p>
    async fn create_rule(
        &self,
        input: CreateRuleRequest,
    ) -> Result<CreateRuleResult, RusotoError<CreateRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.CreateRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateRuleResult, _>()
    }

    /// <p>Creates a variable.</p>
    async fn create_variable(
        &self,
        input: CreateVariableRequest,
    ) -> Result<CreateVariableResult, RusotoError<CreateVariableError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.CreateVariable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateVariableError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateVariableResult, _>()
    }

    /// <p>Deletes the detector. Before deleting a detector, you must first delete all detector versions and rule versions associated with the detector.</p> <p>When you delete a detector, Amazon Fraud Detector permanently deletes the detector and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_detector(
        &self,
        input: DeleteDetectorRequest,
    ) -> Result<DeleteDetectorResult, RusotoError<DeleteDetectorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteDetector");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDetectorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteDetectorResult, _>()
    }

    /// <p>Deletes the detector version. You cannot delete detector versions that are in <code>ACTIVE</code> status.</p> <p>When you delete a detector version, Amazon Fraud Detector permanently deletes the detector and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_detector_version(
        &self,
        input: DeleteDetectorVersionRequest,
    ) -> Result<DeleteDetectorVersionResult, RusotoError<DeleteDetectorVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.DeleteDetectorVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDetectorVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteDetectorVersionResult, _>()
    }

    /// <p>Deletes an entity type.</p> <p>You cannot delete an entity type that is included in an event type.</p> <p>When you delete an entity type, Amazon Fraud Detector permanently deletes that entity type from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_entity_type(
        &self,
        input: DeleteEntityTypeRequest,
    ) -> Result<DeleteEntityTypeResult, RusotoError<DeleteEntityTypeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteEntityType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEntityTypeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteEntityTypeResult, _>()
    }

    /// <p>Deletes the specified event.</p> <p>When you delete an event, Amazon Fraud Detector permanently deletes that event from the evaluation history, and the event data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_event(
        &self,
        input: DeleteEventRequest,
    ) -> Result<DeleteEventResult, RusotoError<DeleteEventError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteEvent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEventError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteEventResult, _>()
    }

    /// <p>Deletes an event type.</p> <p>You cannot delete an event type that is used in a detector or a model.</p> <p>When you delete an entity type, Amazon Fraud Detector permanently deletes that entity type from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_event_type(
        &self,
        input: DeleteEventTypeRequest,
    ) -> Result<DeleteEventTypeResult, RusotoError<DeleteEventTypeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteEventType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEventTypeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteEventTypeResult, _>()
    }

    /// <p>Removes a SageMaker model from Amazon Fraud Detector.</p> <p>You can remove an Amazon SageMaker model if it is not associated with a detector version. Removing a SageMaker model disconnects it from Amazon Fraud Detector, but the model remains available in SageMaker.</p>
    async fn delete_external_model(
        &self,
        input: DeleteExternalModelRequest,
    ) -> Result<DeleteExternalModelResult, RusotoError<DeleteExternalModelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.DeleteExternalModel",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteExternalModelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteExternalModelResult, _>()
    }

    /// <p>Deletes a label.</p> <p>You cannot delete labels that are included in an event type in Amazon Fraud Detector.</p> <p>You cannot delete a label assigned to an event ID. You must first delete the relevant event ID.</p> <p>When you delete a label, Amazon Fraud Detector permanently deletes that label from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_label(
        &self,
        input: DeleteLabelRequest,
    ) -> Result<DeleteLabelResult, RusotoError<DeleteLabelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteLabel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLabelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteLabelResult, _>()
    }

    /// <p>Deletes a model.</p> <p>You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version.</p> <p> When you delete a model, Amazon Fraud Detector permanently deletes that model from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_model(
        &self,
        input: DeleteModelRequest,
    ) -> Result<DeleteModelResult, RusotoError<DeleteModelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteModelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteModelResult, _>()
    }

    /// <p>Deletes a model version.</p> <p>You can delete models and model versions in Amazon Fraud Detector, provided that they are not associated with a detector version.</p> <p> When you delete a model version, Amazon Fraud Detector permanently deletes that model version from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_model_version(
        &self,
        input: DeleteModelVersionRequest,
    ) -> Result<DeleteModelVersionResult, RusotoError<DeleteModelVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.DeleteModelVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteModelVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteModelVersionResult, _>()
    }

    /// <p>Deletes an outcome.</p> <p>You cannot delete an outcome that is used in a rule version.</p> <p>When you delete an outcome, Amazon Fraud Detector permanently deletes that outcome from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_outcome(
        &self,
        input: DeleteOutcomeRequest,
    ) -> Result<DeleteOutcomeResult, RusotoError<DeleteOutcomeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteOutcome");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteOutcomeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteOutcomeResult, _>()
    }

    /// <p>Deletes the rule. You cannot delete a rule if it is used by an <code>ACTIVE</code> or <code>INACTIVE</code> detector version.</p> <p>When you delete a rule, Amazon Fraud Detector permanently deletes that rule from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> Result<DeleteRuleResult, RusotoError<DeleteRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteRuleResult, _>()
    }

    /// <p>Deletes a variable.</p> <p>You can't delete variables that are included in an event type in Amazon Fraud Detector.</p> <p>Amazon Fraud Detector automatically deletes model output variables and SageMaker model output variables when you delete the model. You can't delete these variables manually.</p> <p>When you delete a variable, Amazon Fraud Detector permanently deletes that variable from the evaluation history, and the data is no longer stored in Amazon Fraud Detector.</p>
    async fn delete_variable(
        &self,
        input: DeleteVariableRequest,
    ) -> Result<DeleteVariableResult, RusotoError<DeleteVariableError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DeleteVariable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteVariableError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteVariableResult, _>()
    }

    /// <p>Gets all versions for a specified detector.</p>
    async fn describe_detector(
        &self,
        input: DescribeDetectorRequest,
    ) -> Result<DescribeDetectorResult, RusotoError<DescribeDetectorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.DescribeDetector");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDetectorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeDetectorResult, _>()
    }

    /// <p>Gets all of the model versions for the specified model type or for the specified model type and model ID. You can also get details for a single, specified model version. </p>
    async fn describe_model_versions(
        &self,
        input: DescribeModelVersionsRequest,
    ) -> Result<DescribeModelVersionsResult, RusotoError<DescribeModelVersionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.DescribeModelVersions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeModelVersionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeModelVersionsResult, _>()
    }

    /// <p>Gets a particular detector version. </p>
    async fn get_detector_version(
        &self,
        input: GetDetectorVersionRequest,
    ) -> Result<GetDetectorVersionResult, RusotoError<GetDetectorVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.GetDetectorVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDetectorVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetDetectorVersionResult, _>()
    }

    /// <p>Gets all detectors or a single detector if a <code>detectorId</code> is specified. This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetDetectorsResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_detectors(
        &self,
        input: GetDetectorsRequest,
    ) -> Result<GetDetectorsResult, RusotoError<GetDetectorsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetDetectors");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetDetectorsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetDetectorsResult, _>()
    }

    /// <p>Gets all entity types or a specific entity type if a name is specified. This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetEntityTypesResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_entity_types(
        &self,
        input: GetEntityTypesRequest,
    ) -> Result<GetEntityTypesResult, RusotoError<GetEntityTypesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetEntityTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetEntityTypesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetEntityTypesResult, _>()
    }

    /// <p>Evaluates an event against a detector version. If a version ID is not provided, the detector’s (<code>ACTIVE</code>) version is used.</p>
    async fn get_event_prediction(
        &self,
        input: GetEventPredictionRequest,
    ) -> Result<GetEventPredictionResult, RusotoError<GetEventPredictionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.GetEventPrediction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetEventPredictionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetEventPredictionResult, _>()
    }

    /// <p>Gets all event types or a specific event type if name is provided. This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetEventTypesResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_event_types(
        &self,
        input: GetEventTypesRequest,
    ) -> Result<GetEventTypesResult, RusotoError<GetEventTypesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetEventTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetEventTypesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetEventTypesResult, _>()
    }

    /// <p>Gets the details for one or more Amazon SageMaker models that have been imported into the service. This is a paginated API. If you provide a null <code>maxResults</code>, this actions retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 5 and 10. To get the next page results, provide the pagination token from the <code>GetExternalModelsResult</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_external_models(
        &self,
        input: GetExternalModelsRequest,
    ) -> Result<GetExternalModelsResult, RusotoError<GetExternalModelsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.GetExternalModels",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetExternalModelsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetExternalModelsResult, _>()
    }

    /// <p>Gets the encryption key if a Key Management Service (KMS) customer master key (CMK) has been specified to be used to encrypt content in Amazon Fraud Detector.</p>
    async fn get_kms_encryption_key(
        &self,
    ) -> Result<GetKMSEncryptionKeyResult, RusotoError<GetKMSEncryptionKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.GetKMSEncryptionKey",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetKMSEncryptionKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetKMSEncryptionKeyResult, _>()
    }

    /// <p>Gets all labels or a specific label if name is provided. This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 50 records per page. If you provide a <code>maxResults</code>, the value must be between 10 and 50. To get the next page results, provide the pagination token from the <code>GetGetLabelsResponse</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_labels(
        &self,
        input: GetLabelsRequest,
    ) -> Result<GetLabelsResult, RusotoError<GetLabelsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetLabels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetLabelsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetLabelsResult, _>()
    }

    /// <p>Gets the details of the specified model version.</p>
    async fn get_model_version(
        &self,
        input: GetModelVersionRequest,
    ) -> Result<GetModelVersionResult, RusotoError<GetModelVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetModelVersion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetModelVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetModelVersionResult, _>()
    }

    /// <p>Gets one or more models. Gets all models for the AWS account if no model type and no model id provided. Gets all models for the AWS account and model type, if the model type is specified but model id is not provided. Gets a specific model if (model type, model id) tuple is specified. </p> <p>This is a paginated API. If you provide a null <code>maxResults</code>, this action retrieves a maximum of 10 records per page. If you provide a <code>maxResults</code>, the value must be between 1 and 10. To get the next page results, provide the pagination token from the response as part of your request. A null pagination token fetches the records from the beginning.</p>
    async fn get_models(
        &self,
        input: GetModelsRequest,
    ) -> Result<GetModelsResult, RusotoError<GetModelsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetModels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetModelsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetModelsResult, _>()
    }

    /// <p>Gets one or more outcomes. This is a paginated API. If you provide a null <code>maxResults</code>, this actions retrieves a maximum of 100 records per page. If you provide a <code>maxResults</code>, the value must be between 50 and 100. To get the next page results, provide the pagination token from the <code>GetOutcomesResult</code> as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn get_outcomes(
        &self,
        input: GetOutcomesRequest,
    ) -> Result<GetOutcomesResult, RusotoError<GetOutcomesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetOutcomes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetOutcomesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetOutcomesResult, _>()
    }

    /// <p>Get all rules for a detector (paginated) if <code>ruleId</code> and <code>ruleVersion</code> are not specified. Gets all rules for the detector and the <code>ruleId</code> if present (paginated). Gets a specific rule if both the <code>ruleId</code> and the <code>ruleVersion</code> are specified.</p> <p>This is a paginated API. Providing null maxResults results in retrieving maximum of 100 records per page. If you provide maxResults the value must be between 50 and 100. To get the next page result, a provide a pagination token from GetRulesResult as part of your request. Null pagination token fetches the records from the beginning.</p>
    async fn get_rules(
        &self,
        input: GetRulesRequest,
    ) -> Result<GetRulesResult, RusotoError<GetRulesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRulesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetRulesResult, _>()
    }

    /// <p>Gets all of the variables or the specific variable. This is a paginated API. Providing null <code>maxSizePerPage</code> results in retrieving maximum of 100 records per page. If you provide <code>maxSizePerPage</code> the value must be between 50 and 100. To get the next page result, a provide a pagination token from <code>GetVariablesResult</code> as part of your request. Null pagination token fetches the records from the beginning. </p>
    async fn get_variables(
        &self,
        input: GetVariablesRequest,
    ) -> Result<GetVariablesResult, RusotoError<GetVariablesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.GetVariables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetVariablesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetVariablesResult, _>()
    }

    /// <p>Lists all tags associated with the resource. This is a paginated API. To get the next page results, provide the pagination token from the response as part of your request. A null pagination token fetches the records from the beginning. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResult, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResult, _>()
    }

    /// <p>Creates or updates a detector. </p>
    async fn put_detector(
        &self,
        input: PutDetectorRequest,
    ) -> Result<PutDetectorResult, RusotoError<PutDetectorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutDetector");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutDetectorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutDetectorResult, _>()
    }

    /// <p>Creates or updates an entity type. An entity represents who is performing the event. As part of a fraud prediction, you pass the entity ID to indicate the specific entity who performed the event. An entity type classifies the entity. Example classifications include customer, merchant, or account.</p>
    async fn put_entity_type(
        &self,
        input: PutEntityTypeRequest,
    ) -> Result<PutEntityTypeResult, RusotoError<PutEntityTypeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutEntityType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutEntityTypeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutEntityTypeResult, _>()
    }

    /// <p>Creates or updates an event type. An event is a business activity that is evaluated for fraud risk. With Amazon Fraud Detector, you generate fraud predictions for events. An event type defines the structure for an event sent to Amazon Fraud Detector. This includes the variables sent as part of the event, the entity performing the event (such as a customer), and the labels that classify the event. Example event types include online payment transactions, account registrations, and authentications.</p>
    async fn put_event_type(
        &self,
        input: PutEventTypeRequest,
    ) -> Result<PutEventTypeResult, RusotoError<PutEventTypeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutEventType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutEventTypeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutEventTypeResult, _>()
    }

    /// <p>Creates or updates an Amazon SageMaker model endpoint. You can also use this action to update the configuration of the model endpoint, including the IAM role and/or the mapped variables. </p>
    async fn put_external_model(
        &self,
        input: PutExternalModelRequest,
    ) -> Result<PutExternalModelResult, RusotoError<PutExternalModelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutExternalModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutExternalModelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutExternalModelResult, _>()
    }

    /// <p>Specifies the Key Management Service (KMS) customer master key (CMK) to be used to encrypt content in Amazon Fraud Detector.</p>
    async fn put_kms_encryption_key(
        &self,
        input: PutKMSEncryptionKeyRequest,
    ) -> Result<PutKMSEncryptionKeyResult, RusotoError<PutKMSEncryptionKeyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.PutKMSEncryptionKey",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutKMSEncryptionKeyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutKMSEncryptionKeyResult, _>()
    }

    /// <p>Creates or updates label. A label classifies an event as fraudulent or legitimate. Labels are associated with event types and used to train supervised machine learning models in Amazon Fraud Detector. </p>
    async fn put_label(
        &self,
        input: PutLabelRequest,
    ) -> Result<PutLabelResult, RusotoError<PutLabelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutLabel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutLabelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutLabelResult, _>()
    }

    /// <p>Creates or updates an outcome. </p>
    async fn put_outcome(
        &self,
        input: PutOutcomeRequest,
    ) -> Result<PutOutcomeResult, RusotoError<PutOutcomeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.PutOutcome");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutOutcomeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutOutcomeResult, _>()
    }

    /// <p>Assigns tags to a resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResult, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResult, _>()
    }

    /// <p>Removes tags from a resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResult, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResult, _>()
    }

    /// <p> Updates a detector version. The detector version attributes that you can update include models, external model endpoints, rules, rule execution mode, and description. You can only update a <code>DRAFT</code> detector version.</p>
    async fn update_detector_version(
        &self,
        input: UpdateDetectorVersionRequest,
    ) -> Result<UpdateDetectorVersionResult, RusotoError<UpdateDetectorVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateDetectorVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDetectorVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateDetectorVersionResult, _>()
    }

    /// <p>Updates the detector version's description. You can update the metadata for any detector version (<code>DRAFT, ACTIVE,</code> or <code>INACTIVE</code>). </p>
    async fn update_detector_version_metadata(
        &self,
        input: UpdateDetectorVersionMetadataRequest,
    ) -> Result<UpdateDetectorVersionMetadataResult, RusotoError<UpdateDetectorVersionMetadataError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateDetectorVersionMetadata",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDetectorVersionMetadataError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateDetectorVersionMetadataResult, _>()
    }

    /// <p>Updates the detector version’s status. You can perform the following promotions or demotions using <code>UpdateDetectorVersionStatus</code>: <code>DRAFT</code> to <code>ACTIVE</code>, <code>ACTIVE</code> to <code>INACTIVE</code>, and <code>INACTIVE</code> to <code>ACTIVE</code>.</p>
    async fn update_detector_version_status(
        &self,
        input: UpdateDetectorVersionStatusRequest,
    ) -> Result<UpdateDetectorVersionStatusResult, RusotoError<UpdateDetectorVersionStatusError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateDetectorVersionStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDetectorVersionStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateDetectorVersionStatusResult, _>()
    }

    /// <p>Updates a model. You can update the description attribute using this action.</p>
    async fn update_model(
        &self,
        input: UpdateModelRequest,
    ) -> Result<UpdateModelResult, RusotoError<UpdateModelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.UpdateModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateModelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateModelResult, _>()
    }

    /// <p>Updates a model version. Updating a model version retrains an existing model version using updated training data and produces a new minor version of the model. You can update the training data set location and data access role attributes using this action. This action creates and trains a new minor version of the model, for example version 1.01, 1.02, 1.03.</p>
    async fn update_model_version(
        &self,
        input: UpdateModelVersionRequest,
    ) -> Result<UpdateModelVersionResult, RusotoError<UpdateModelVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateModelVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateModelVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateModelVersionResult, _>()
    }

    /// <p><p>Updates the status of a model version.</p> <p>You can perform the following status updates:</p> <ol> <li> <p>Change the <code>TRAINING_COMPLETE</code> status to <code>ACTIVE</code>.</p> </li> <li> <p>Change <code>ACTIVE</code>to <code>INACTIVE</code>.</p> </li> </ol></p>
    async fn update_model_version_status(
        &self,
        input: UpdateModelVersionStatusRequest,
    ) -> Result<UpdateModelVersionStatusResult, RusotoError<UpdateModelVersionStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateModelVersionStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateModelVersionStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateModelVersionStatusResult, _>()
    }

    /// <p>Updates a rule's metadata. The description attribute can be updated.</p>
    async fn update_rule_metadata(
        &self,
        input: UpdateRuleMetadataRequest,
    ) -> Result<UpdateRuleMetadataResult, RusotoError<UpdateRuleMetadataError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateRuleMetadata",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRuleMetadataError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateRuleMetadataResult, _>()
    }

    /// <p>Updates a rule version resulting in a new rule version. Updates a rule version resulting in a new rule version (version 1, 2, 3 ...). </p>
    async fn update_rule_version(
        &self,
        input: UpdateRuleVersionRequest,
    ) -> Result<UpdateRuleVersionResult, RusotoError<UpdateRuleVersionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHawksNestServiceFacade.UpdateRuleVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRuleVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateRuleVersionResult, _>()
    }

    /// <p>Updates a variable.</p>
    async fn update_variable(
        &self,
        input: UpdateVariableRequest,
    ) -> Result<UpdateVariableResult, RusotoError<UpdateVariableError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHawksNestServiceFacade.UpdateVariable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateVariableError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateVariableResult, _>()
    }
}
