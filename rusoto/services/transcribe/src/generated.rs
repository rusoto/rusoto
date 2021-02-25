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

impl TranscribeClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "transcribe", &self.region, request_uri);

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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownBaseModelName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum BaseModelName {
    NarrowBand,
    WideBand,
    #[doc(hidden)]
    UnknownVariant(UnknownBaseModelName),
}

impl Default for BaseModelName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for BaseModelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for BaseModelName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for BaseModelName {
    fn into(self) -> String {
        match self {
            BaseModelName::NarrowBand => "NarrowBand".to_string(),
            BaseModelName::WideBand => "WideBand".to_string(),
            BaseModelName::UnknownVariant(UnknownBaseModelName { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a BaseModelName {
    fn into(self) -> &'a str {
        match self {
            BaseModelName::NarrowBand => &"NarrowBand",
            BaseModelName::WideBand => &"WideBand",
            BaseModelName::UnknownVariant(UnknownBaseModelName { name: original }) => original,
        }
    }
}

impl From<&str> for BaseModelName {
    fn from(name: &str) -> Self {
        match name {
            "NarrowBand" => BaseModelName::NarrowBand,
            "WideBand" => BaseModelName::WideBand,
            _ => BaseModelName::UnknownVariant(UnknownBaseModelName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for BaseModelName {
    fn from(name: String) -> Self {
        match &*name {
            "NarrowBand" => BaseModelName::NarrowBand,
            "WideBand" => BaseModelName::WideBand,
            _ => BaseModelName::UnknownVariant(UnknownBaseModelName { name }),
        }
    }
}

impl ::std::str::FromStr for BaseModelName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for BaseModelName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for BaseModelName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCLMLanguageCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CLMLanguageCode {
    EnUS,
    #[doc(hidden)]
    UnknownVariant(UnknownCLMLanguageCode),
}

impl Default for CLMLanguageCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CLMLanguageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CLMLanguageCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CLMLanguageCode {
    fn into(self) -> String {
        match self {
            CLMLanguageCode::EnUS => "en-US".to_string(),
            CLMLanguageCode::UnknownVariant(UnknownCLMLanguageCode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CLMLanguageCode {
    fn into(self) -> &'a str {
        match self {
            CLMLanguageCode::EnUS => &"en-US",
            CLMLanguageCode::UnknownVariant(UnknownCLMLanguageCode { name: original }) => original,
        }
    }
}

impl From<&str> for CLMLanguageCode {
    fn from(name: &str) -> Self {
        match name {
            "en-US" => CLMLanguageCode::EnUS,
            _ => CLMLanguageCode::UnknownVariant(UnknownCLMLanguageCode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CLMLanguageCode {
    fn from(name: String) -> Self {
        match &*name {
            "en-US" => CLMLanguageCode::EnUS,
            _ => CLMLanguageCode::UnknownVariant(UnknownCLMLanguageCode { name }),
        }
    }
}

impl ::std::str::FromStr for CLMLanguageCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CLMLanguageCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CLMLanguageCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Settings for content redaction within a transcription job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContentRedaction {
    /// <p>The output transcript file stored in either the default S3 bucket or in a bucket you specify.</p> <p>When you choose <code>redacted</code> Amazon Transcribe outputs only the redacted transcript.</p> <p>When you choose <code>redacted_and_unredacted</code> Amazon Transcribe outputs both the redacted and unredacted transcripts.</p>
    #[serde(rename = "RedactionOutput")]
    pub redaction_output: RedactionOutput,
    /// <p>Request parameter that defines the entities to be redacted. The only accepted value is <code>PII</code>.</p>
    #[serde(rename = "RedactionType")]
    pub redaction_type: RedactionType,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLanguageModelRequest {
    /// <p>The Amazon Transcribe standard language model, or base model used to create your custom language model.</p> <p>If you want to use your custom language model to transcribe audio with a sample rate of 16 kHz or greater, choose <code>Wideband</code>.</p> <p>If you want to use your custom language model to transcribe audio with a sample rate that is less than 16 kHz, choose <code>Narrowband</code>.</p>
    #[serde(rename = "BaseModelName")]
    pub base_model_name: BaseModelName,
    /// <p>Contains the data access role and the Amazon S3 prefixes to read the required input files to create a custom language model.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The language of the input text you're using to train your custom language model.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: CLMLanguageCode,
    /// <p>The name you choose for your custom language model when you create it.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLanguageModelResponse {
    /// <p>The Amazon Transcribe standard language model, or base model you've used to create a custom language model.</p>
    #[serde(rename = "BaseModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_name: Option<BaseModelName>,
    /// <p>The data access role and Amazon S3 prefixes you've chosen to create your custom language model.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The language code of the text you've used to create a custom language model.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<CLMLanguageCode>,
    /// <p>The name you've chosen for your custom language model.</p>
    #[serde(rename = "ModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// <p>The status of the custom language model. When the status is <code>COMPLETED</code> the model is ready to use.</p>
    #[serde(rename = "ModelStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_status: Option<ModelStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMedicalVocabularyRequest {
    /// <p>The language code for the language used for the entries in your custom vocabulary. The language code of your custom vocabulary must match the language code of your transcription job. US English (en-US) is the only language code available for Amazon Transcribe Medical.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: LanguageCode,
    /// <p>The location in Amazon S3 of the text file you use to define your custom vocabulary. The URI must be in the same AWS Region as the resource that you're calling. Enter information about your <code>VocabularyFileUri</code> in the following format:</p> <p> <code> https://s3.&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>The following is an example URI for a vocabulary file that is stored in Amazon S3:</p> <p> <code>https://s3.us-east-1.amazonaws.com/AWSDOC-EXAMPLE-BUCKET/vocab.txt</code> </p> <p>For more information about Amazon S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>For more information about custom vocabularies, see <a href="http://docs.aws.amazon.com/transcribe/latest/dg/how-it-works.html#how-vocabulary-med">Medical Custom Vocabularies</a>.</p>
    #[serde(rename = "VocabularyFileUri")]
    pub vocabulary_file_uri: String,
    /// <p>The name of the custom vocabulary. This case-sensitive name must be unique within an AWS account. If you try to create a vocabulary with the same name as a previous vocabulary, you get a <code>ConflictException</code> error.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMedicalVocabularyResponse {
    /// <p>If the <code>VocabularyState</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language code for the entries in your custom vocabulary. US English (en-US) is the only valid language code for Amazon Transcribe Medical.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that you created the vocabulary.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary. The name must be unique within an AWS account and is case sensitive.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of your custom vocabulary in Amazon Transcribe Medical. If the state is <code>READY</code>, you can use the vocabulary in a <code>StartMedicalTranscriptionJob</code> request.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<VocabularyState>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVocabularyFilterRequest {
    /// <p>The language code of the words in the vocabulary filter. All words in the filter must be in the same language. The vocabulary filter can only be used with transcription jobs in the specified language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: LanguageCode,
    /// <p>The Amazon S3 location of a text file used as input to create the vocabulary filter. Only use characters from the character set defined for custom vocabularies. For a list of character sets, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-vocabulary.html#charsets">Character Sets for Custom Vocabularies</a>.</p> <p>The specified file must be less than 50 KB of UTF-8 characters.</p> <p>If you provide the location of a list of words in the <code>VocabularyFilterFileUri</code> parameter, you can't use the <code>Words</code> parameter.</p>
    #[serde(rename = "VocabularyFilterFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_file_uri: Option<String>,
    /// <p>The vocabulary filter name. The name must be unique within the account that contains it. If you try to create a vocabulary filter with the same name as another vocabulary filter, you get a <code>ConflictException</code> error.</p>
    #[serde(rename = "VocabularyFilterName")]
    pub vocabulary_filter_name: String,
    /// <p>The words to use in the vocabulary filter. Only use characters from the character set defined for custom vocabularies. For a list of character sets, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-vocabulary.html#charsets">Character Sets for Custom Vocabularies</a>.</p> <p>If you provide a list of words in the <code>Words</code> parameter, you can't use the <code>VocabularyFilterFileUri</code> parameter.</p>
    #[serde(rename = "Words")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVocabularyFilterResponse {
    /// <p>The language code of the words in the collection.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary filter was modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary filter.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVocabularyRequest {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: LanguageCode,
    /// <p>An array of strings that contains the vocabulary entries. </p>
    #[serde(rename = "Phrases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
    /// <p>The S3 location of the text file that contains the definition of the custom vocabulary. The URI must be in the same region as the API endpoint that you are calling. The general form is </p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>For more information about custom vocabularies, see <a href="http://docs.aws.amazon.com/transcribe/latest/dg/how-it-works.html#how-vocabulary">Custom Vocabularies</a>.</p>
    #[serde(rename = "VocabularyFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    /// <p>The name of the vocabulary. The name must be unique within an AWS account. The name is case sensitive. If you try to create a vocabulary with the same name as a previous vocabulary you will receive a <code>ConflictException</code> error.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVocabularyResponse {
    /// <p>If the <code>VocabularyState</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary was created.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary. When the <code>VocabularyState</code> field contains <code>READY</code> the vocabulary is ready to be used in a <code>StartTranscriptionJob</code> request.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<VocabularyState>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLanguageModelRequest {
    /// <p>The name of the model you're choosing to delete.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMedicalTranscriptionJobRequest {
    /// <p>The name you provide to the <code>DeleteMedicalTranscriptionJob</code> object to delete a transcription job.</p>
    #[serde(rename = "MedicalTranscriptionJobName")]
    pub medical_transcription_job_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMedicalVocabularyRequest {
    /// <p>The name of the vocabulary that you want to delete.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTranscriptionJobRequest {
    /// <p>The name of the transcription job to be deleted.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVocabularyFilterRequest {
    /// <p>The name of the vocabulary filter to remove.</p>
    #[serde(rename = "VocabularyFilterName")]
    pub vocabulary_filter_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVocabularyRequest {
    /// <p>The name of the vocabulary to delete. </p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLanguageModelRequest {
    /// <p>The name of the custom language model you submit to get more information.</p>
    #[serde(rename = "ModelName")]
    pub model_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLanguageModelResponse {
    /// <p>The name of the custom language model you requested more information about.</p>
    #[serde(rename = "LanguageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model: Option<LanguageModel>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMedicalTranscriptionJobRequest {
    /// <p>The name of the medical transcription job.</p>
    #[serde(rename = "MedicalTranscriptionJobName")]
    pub medical_transcription_job_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMedicalTranscriptionJobResponse {
    /// <p>An object that contains the results of the medical transcription job.</p>
    #[serde(rename = "MedicalTranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job: Option<MedicalTranscriptionJob>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetMedicalVocabularyRequest {
    /// <p>The name of the vocabulary that you want information about. The value is case sensitive. </p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetMedicalVocabularyResponse {
    /// <p>The location in Amazon S3 where the vocabulary is stored. Use this URI to get the contents of the vocabulary. You can download your vocabulary from the URI for a limited time.</p>
    #[serde(rename = "DownloadUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    /// <p>If the <code>VocabularyState</code> is <code>FAILED</code>, this field contains information about why the job failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The valid language code for your vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary was last modified with a text file different from the one that was previously used.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary returned by Amazon Transcribe Medical.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary. If the <code>VocabularyState</code> is <code>READY</code> then you can use it in the <code>StartMedicalTranscriptionJob</code> operation. </p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<VocabularyState>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTranscriptionJobRequest {
    /// <p>The name of the job.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTranscriptionJobResponse {
    /// <p>An object that contains the results of the transcription job.</p>
    #[serde(rename = "TranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVocabularyFilterRequest {
    /// <p>The name of the vocabulary filter for which to return information.</p>
    #[serde(rename = "VocabularyFilterName")]
    pub vocabulary_filter_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVocabularyFilterResponse {
    /// <p>The URI of the list of words in the vocabulary filter. You can use this URI to get the list of words.</p>
    #[serde(rename = "DownloadUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    /// <p>The language code of the words in the vocabulary filter.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the contents of the vocabulary filter were updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary filter.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVocabularyRequest {
    /// <p>The name of the vocabulary to return information about. The name is case sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVocabularyResponse {
    /// <p>The S3 location where the vocabulary is stored. Use this URI to get the contents of the vocabulary. The URI is available for a limited time.</p>
    #[serde(rename = "DownloadUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    /// <p>If the <code>VocabularyState</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary to return.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<VocabularyState>,
}

/// <p>The object that contains the Amazon S3 object location and access role required to train and tune your custom language model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputDataConfig {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the permissions you've given Amazon Transcribe to access your Amazon S3 buckets containing your media files or text data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>The Amazon S3 prefix you specify to access the plain text files that you use to train your custom language model.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
    /// <p>The Amazon S3 prefix you specify to access the plain text files that you use to tune your custom language model.</p>
    #[serde(rename = "TuningDataS3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_data_s3_uri: Option<String>,
}

/// <p>Provides information about when a transcription job should be executed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JobExecutionSettings {
    /// <p>Indicates whether a job should be queued by Amazon Transcribe when the concurrent execution limit is exceeded. When the <code>AllowDeferredExecution</code> field is true, jobs are queued and executed when the number of executing jobs falls below the concurrent execution limit. If the field is false, Amazon Transcribe returns a <code>LimitExceededException</code> exception.</p> <p>If you specify the <code>AllowDeferredExecution</code> field, you must specify the <code>DataAccessRoleArn</code> field.</p>
    #[serde(rename = "AllowDeferredExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deferred_execution: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a role that has access to the S3 bucket that contains the input files. Amazon Transcribe assumes this role to read queued media files. If you have specified an output S3 bucket for the transcription results, this role should have access to the output bucket as well.</p> <p>If you specify the <code>AllowDeferredExecution</code> field, you must specify the <code>DataAccessRoleArn</code> field.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownLanguageCode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LanguageCode {
    AfZA,
    ArAE,
    ArSA,
    CyGB,
    DaDK,
    DeCH,
    DeDE,
    EnAB,
    EnAU,
    EnGB,
    EnIE,
    EnIN,
    EnUS,
    EnWL,
    EsES,
    EsUS,
    FaIR,
    FrCA,
    FrFR,
    GaIE,
    GdGB,
    HeIL,
    HiIN,
    IdID,
    ItIT,
    JaJP,
    KoKR,
    MsMY,
    NlNL,
    PtBR,
    PtPT,
    RuRU,
    TaIN,
    TeIN,
    TrTR,
    ZhCN,
    #[doc(hidden)]
    UnknownVariant(UnknownLanguageCode),
}

impl Default for LanguageCode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for LanguageCode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for LanguageCode {
    fn into(self) -> String {
        match self {
            LanguageCode::AfZA => "af-ZA".to_string(),
            LanguageCode::ArAE => "ar-AE".to_string(),
            LanguageCode::ArSA => "ar-SA".to_string(),
            LanguageCode::CyGB => "cy-GB".to_string(),
            LanguageCode::DaDK => "da-DK".to_string(),
            LanguageCode::DeCH => "de-CH".to_string(),
            LanguageCode::DeDE => "de-DE".to_string(),
            LanguageCode::EnAB => "en-AB".to_string(),
            LanguageCode::EnAU => "en-AU".to_string(),
            LanguageCode::EnGB => "en-GB".to_string(),
            LanguageCode::EnIE => "en-IE".to_string(),
            LanguageCode::EnIN => "en-IN".to_string(),
            LanguageCode::EnUS => "en-US".to_string(),
            LanguageCode::EnWL => "en-WL".to_string(),
            LanguageCode::EsES => "es-ES".to_string(),
            LanguageCode::EsUS => "es-US".to_string(),
            LanguageCode::FaIR => "fa-IR".to_string(),
            LanguageCode::FrCA => "fr-CA".to_string(),
            LanguageCode::FrFR => "fr-FR".to_string(),
            LanguageCode::GaIE => "ga-IE".to_string(),
            LanguageCode::GdGB => "gd-GB".to_string(),
            LanguageCode::HeIL => "he-IL".to_string(),
            LanguageCode::HiIN => "hi-IN".to_string(),
            LanguageCode::IdID => "id-ID".to_string(),
            LanguageCode::ItIT => "it-IT".to_string(),
            LanguageCode::JaJP => "ja-JP".to_string(),
            LanguageCode::KoKR => "ko-KR".to_string(),
            LanguageCode::MsMY => "ms-MY".to_string(),
            LanguageCode::NlNL => "nl-NL".to_string(),
            LanguageCode::PtBR => "pt-BR".to_string(),
            LanguageCode::PtPT => "pt-PT".to_string(),
            LanguageCode::RuRU => "ru-RU".to_string(),
            LanguageCode::TaIN => "ta-IN".to_string(),
            LanguageCode::TeIN => "te-IN".to_string(),
            LanguageCode::TrTR => "tr-TR".to_string(),
            LanguageCode::ZhCN => "zh-CN".to_string(),
            LanguageCode::UnknownVariant(UnknownLanguageCode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a LanguageCode {
    fn into(self) -> &'a str {
        match self {
            LanguageCode::AfZA => &"af-ZA",
            LanguageCode::ArAE => &"ar-AE",
            LanguageCode::ArSA => &"ar-SA",
            LanguageCode::CyGB => &"cy-GB",
            LanguageCode::DaDK => &"da-DK",
            LanguageCode::DeCH => &"de-CH",
            LanguageCode::DeDE => &"de-DE",
            LanguageCode::EnAB => &"en-AB",
            LanguageCode::EnAU => &"en-AU",
            LanguageCode::EnGB => &"en-GB",
            LanguageCode::EnIE => &"en-IE",
            LanguageCode::EnIN => &"en-IN",
            LanguageCode::EnUS => &"en-US",
            LanguageCode::EnWL => &"en-WL",
            LanguageCode::EsES => &"es-ES",
            LanguageCode::EsUS => &"es-US",
            LanguageCode::FaIR => &"fa-IR",
            LanguageCode::FrCA => &"fr-CA",
            LanguageCode::FrFR => &"fr-FR",
            LanguageCode::GaIE => &"ga-IE",
            LanguageCode::GdGB => &"gd-GB",
            LanguageCode::HeIL => &"he-IL",
            LanguageCode::HiIN => &"hi-IN",
            LanguageCode::IdID => &"id-ID",
            LanguageCode::ItIT => &"it-IT",
            LanguageCode::JaJP => &"ja-JP",
            LanguageCode::KoKR => &"ko-KR",
            LanguageCode::MsMY => &"ms-MY",
            LanguageCode::NlNL => &"nl-NL",
            LanguageCode::PtBR => &"pt-BR",
            LanguageCode::PtPT => &"pt-PT",
            LanguageCode::RuRU => &"ru-RU",
            LanguageCode::TaIN => &"ta-IN",
            LanguageCode::TeIN => &"te-IN",
            LanguageCode::TrTR => &"tr-TR",
            LanguageCode::ZhCN => &"zh-CN",
            LanguageCode::UnknownVariant(UnknownLanguageCode { name: original }) => original,
        }
    }
}

impl From<&str> for LanguageCode {
    fn from(name: &str) -> Self {
        match name {
            "af-ZA" => LanguageCode::AfZA,
            "ar-AE" => LanguageCode::ArAE,
            "ar-SA" => LanguageCode::ArSA,
            "cy-GB" => LanguageCode::CyGB,
            "da-DK" => LanguageCode::DaDK,
            "de-CH" => LanguageCode::DeCH,
            "de-DE" => LanguageCode::DeDE,
            "en-AB" => LanguageCode::EnAB,
            "en-AU" => LanguageCode::EnAU,
            "en-GB" => LanguageCode::EnGB,
            "en-IE" => LanguageCode::EnIE,
            "en-IN" => LanguageCode::EnIN,
            "en-US" => LanguageCode::EnUS,
            "en-WL" => LanguageCode::EnWL,
            "es-ES" => LanguageCode::EsES,
            "es-US" => LanguageCode::EsUS,
            "fa-IR" => LanguageCode::FaIR,
            "fr-CA" => LanguageCode::FrCA,
            "fr-FR" => LanguageCode::FrFR,
            "ga-IE" => LanguageCode::GaIE,
            "gd-GB" => LanguageCode::GdGB,
            "he-IL" => LanguageCode::HeIL,
            "hi-IN" => LanguageCode::HiIN,
            "id-ID" => LanguageCode::IdID,
            "it-IT" => LanguageCode::ItIT,
            "ja-JP" => LanguageCode::JaJP,
            "ko-KR" => LanguageCode::KoKR,
            "ms-MY" => LanguageCode::MsMY,
            "nl-NL" => LanguageCode::NlNL,
            "pt-BR" => LanguageCode::PtBR,
            "pt-PT" => LanguageCode::PtPT,
            "ru-RU" => LanguageCode::RuRU,
            "ta-IN" => LanguageCode::TaIN,
            "te-IN" => LanguageCode::TeIN,
            "tr-TR" => LanguageCode::TrTR,
            "zh-CN" => LanguageCode::ZhCN,
            _ => LanguageCode::UnknownVariant(UnknownLanguageCode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for LanguageCode {
    fn from(name: String) -> Self {
        match &*name {
            "af-ZA" => LanguageCode::AfZA,
            "ar-AE" => LanguageCode::ArAE,
            "ar-SA" => LanguageCode::ArSA,
            "cy-GB" => LanguageCode::CyGB,
            "da-DK" => LanguageCode::DaDK,
            "de-CH" => LanguageCode::DeCH,
            "de-DE" => LanguageCode::DeDE,
            "en-AB" => LanguageCode::EnAB,
            "en-AU" => LanguageCode::EnAU,
            "en-GB" => LanguageCode::EnGB,
            "en-IE" => LanguageCode::EnIE,
            "en-IN" => LanguageCode::EnIN,
            "en-US" => LanguageCode::EnUS,
            "en-WL" => LanguageCode::EnWL,
            "es-ES" => LanguageCode::EsES,
            "es-US" => LanguageCode::EsUS,
            "fa-IR" => LanguageCode::FaIR,
            "fr-CA" => LanguageCode::FrCA,
            "fr-FR" => LanguageCode::FrFR,
            "ga-IE" => LanguageCode::GaIE,
            "gd-GB" => LanguageCode::GdGB,
            "he-IL" => LanguageCode::HeIL,
            "hi-IN" => LanguageCode::HiIN,
            "id-ID" => LanguageCode::IdID,
            "it-IT" => LanguageCode::ItIT,
            "ja-JP" => LanguageCode::JaJP,
            "ko-KR" => LanguageCode::KoKR,
            "ms-MY" => LanguageCode::MsMY,
            "nl-NL" => LanguageCode::NlNL,
            "pt-BR" => LanguageCode::PtBR,
            "pt-PT" => LanguageCode::PtPT,
            "ru-RU" => LanguageCode::RuRU,
            "ta-IN" => LanguageCode::TaIN,
            "te-IN" => LanguageCode::TeIN,
            "tr-TR" => LanguageCode::TrTR,
            "zh-CN" => LanguageCode::ZhCN,
            _ => LanguageCode::UnknownVariant(UnknownLanguageCode { name }),
        }
    }
}

impl ::std::str::FromStr for LanguageCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for LanguageCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LanguageCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The structure used to describe a custom language model.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LanguageModel {
    /// <p>The Amazon Transcribe standard language model, or base model used to create the custom language model.</p>
    #[serde(rename = "BaseModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_name: Option<BaseModelName>,
    /// <p>The time the custom language model was created.</p>
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    /// <p>The reason why the custom language model couldn't be created.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The data access role and Amazon S3 prefixes for the input files used to train the custom language model.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The language code you used to create your custom language model.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<CLMLanguageCode>,
    /// <p>The most recent time the custom language model was modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the custom language model.</p>
    #[serde(rename = "ModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// <p>The creation status of a custom language model. When the status is <code>COMPLETED</code> the model is ready for use.</p>
    #[serde(rename = "ModelStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_status: Option<ModelStatus>,
    /// <p>Whether the base model used for the custom language model is up to date. If this field is <code>true</code> then you are running the most up-to-date version of the base model in your custom language model.</p>
    #[serde(rename = "UpgradeAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_availability: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLanguageModelsRequest {
    /// <p>The maximum number of language models to return in the response. If there are fewer results in the list, the response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When specified, the custom language model names returned contain the substring you've specified.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>When included, fetches the next set of jobs if the result of the previous request was truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When specified, returns only custom language models with the specified status. Language models are ordered by creation date, with the newest models first. If you don't specify a status, Amazon Transcribe returns all custom language models ordered by date.</p>
    #[serde(rename = "StatusEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<ModelStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLanguageModelsResponse {
    /// <p>A list of objects containing information about custom language models.</p>
    #[serde(rename = "Models")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<LanguageModel>>,
    /// <p>The operation returns a page of jobs at a time. The maximum size of the list is set by the MaxResults parameter. If there are more language models in the list than the page size, Amazon Transcribe returns the <code>NextPage</code> token. Include the token in the next request to the operation to return the next page of language models.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMedicalTranscriptionJobsRequest {
    /// <p>When specified, the jobs returned in the list are limited to jobs whose name contains the specified string.</p>
    #[serde(rename = "JobNameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name_contains: Option<String>,
    /// <p>The maximum number of medical transcription jobs to return in the response. IF there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If you a receive a truncated result in the previous request of <code>ListMedicalTranscriptionJobs</code>, include <code>NextToken</code> to fetch the next set of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When specified, returns only medical transcription jobs with the specified status. Jobs are ordered by creation date, with the newest jobs returned first. If you don't specify a status, Amazon Transcribe Medical returns all transcription jobs ordered by creation date.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TranscriptionJobStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMedicalTranscriptionJobsResponse {
    /// <p>A list of objects containing summary information for a transcription job.</p>
    #[serde(rename = "MedicalTranscriptionJobSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job_summaries: Option<Vec<MedicalTranscriptionJobSummary>>,
    /// <p>The <code>ListMedicalTranscriptionJobs</code> operation returns a page of jobs at a time. The maximum size of the page is set by the <code>MaxResults</code> parameter. If the number of jobs exceeds what can fit on a page, Amazon Transcribe Medical returns the <code>NextPage</code> token. Include the token in the next request to the <code>ListMedicalTranscriptionJobs</code> operation to return in the next page of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The requested status of the medical transcription jobs returned.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TranscriptionJobStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMedicalVocabulariesRequest {
    /// <p>The maximum number of vocabularies to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Returns vocabularies whose names contain the specified string. The search is not case sensitive. <code>ListMedicalVocabularies</code> returns both "<code>vocabularyname</code>" and "<code>VocabularyName</code>".</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of your previous request to <code>ListMedicalVocabularies</code> was truncated, include the <code>NextToken</code> to fetch the next set of vocabularies.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When specified, returns only vocabularies with the <code>VocabularyState</code> equal to the specified vocabulary state. Use this field to see which vocabularies are ready for your medical transcription jobs.</p>
    #[serde(rename = "StateEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_equals: Option<VocabularyState>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMedicalVocabulariesResponse {
    /// <p>The <code>ListMedicalVocabularies</code> operation returns a page of vocabularies at a time. You set the maximum number of vocabularies to return on a page with the <code>MaxResults</code> parameter. If there are more jobs in the list will fit on a page, Amazon Transcribe Medical returns the <code>NextPage</code> token. To return the next page of vocabularies, include the token in the next request to the <code>ListMedicalVocabularies</code> operation .</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The requested vocabulary state.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VocabularyState>,
    /// <p>A list of objects that describe the vocabularies that match your search criteria.</p>
    #[serde(rename = "Vocabularies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabularies: Option<Vec<VocabularyInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTranscriptionJobsRequest {
    /// <p>When specified, the jobs returned in the list are limited to jobs whose name contains the specified string.</p>
    #[serde(rename = "JobNameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name_contains: Option<String>,
    /// <p>The maximum number of jobs to return in the response. If there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request to <code>ListTranscriptionJobs</code> was truncated, include the <code>NextToken</code> to fetch the next set of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When specified, returns only transcription jobs with the specified status. Jobs are ordered by creation date, with the newest jobs returned first. If you don’t specify a status, Amazon Transcribe returns all transcription jobs ordered by creation date. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TranscriptionJobStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTranscriptionJobsResponse {
    /// <p>The <code>ListTranscriptionJobs</code> operation returns a page of jobs at a time. The maximum size of the page is set by the <code>MaxResults</code> parameter. If there are more jobs in the list than the page size, Amazon Transcribe returns the <code>NextPage</code> token. Include the token in the next request to the <code>ListTranscriptionJobs</code> operation to return in the next page of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The requested status of the jobs returned.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TranscriptionJobStatus>,
    /// <p>A list of objects containing summary information for a transcription job.</p>
    #[serde(rename = "TranscriptionJobSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_summaries: Option<Vec<TranscriptionJobSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVocabulariesRequest {
    /// <p>The maximum number of vocabularies to return in the response. If there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When specified, the vocabularies returned in the list are limited to vocabularies whose name contains the specified string. The search is not case sensitive, <code>ListVocabularies</code> returns both "vocabularyname" and "VocabularyName" in the response list.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous request to <code>ListVocabularies</code> was truncated, include the <code>NextToken</code> to fetch the next set of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When specified, only returns vocabularies with the <code>VocabularyState</code> field equal to the specified state.</p>
    #[serde(rename = "StateEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_equals: Option<VocabularyState>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVocabulariesResponse {
    /// <p>The <code>ListVocabularies</code> operation returns a page of vocabularies at a time. The maximum size of the page is set in the <code>MaxResults</code> parameter. If there are more jobs in the list than will fit on the page, Amazon Transcribe returns the <code>NextPage</code> token. To return in the next page of jobs, include the token in the next request to the <code>ListVocabularies</code> operation.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The requested vocabulary state.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VocabularyState>,
    /// <p>A list of objects that describe the vocabularies that match the search criteria in the request.</p>
    #[serde(rename = "Vocabularies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabularies: Option<Vec<VocabularyInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVocabularyFiltersRequest {
    /// <p>The maximum number of filters to return in the response. If there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Filters the response so that it only contains vocabulary filters whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous request to <code>ListVocabularyFilters</code> was truncated, include the <code>NextToken</code> to fetch the next set of collections.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVocabularyFiltersResponse {
    /// <p>The <code>ListVocabularyFilters</code> operation returns a page of collections at a time. The maximum size of the page is set by the <code>MaxResults</code> parameter. If there are more jobs in the list than the page size, Amazon Transcribe returns the <code>NextPage</code> token. Include the token in the next request to the <code>ListVocabularyFilters</code> operation to return in the next page of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of vocabulary filters. It contains at most <code>MaxResults</code> number of filters. If there are more filters, call the <code>ListVocabularyFilters</code> operation again with the <code>NextToken</code> parameter in the request set to the value of the <code>NextToken</code> field in the response.</p>
    #[serde(rename = "VocabularyFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filters: Option<Vec<VocabularyFilterInfo>>,
}

/// <p>Describes the input media file in a transcription request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Media {
    /// <p>The S3 object location of the input media file. The URI must be in the same region as the API endpoint that you are calling. The general form is:</p> <p>For example:</p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    #[serde(rename = "MediaFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_file_uri: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownMediaFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum MediaFormat {
    Amr,
    Flac,
    Mp3,
    Mp4,
    Ogg,
    Wav,
    Webm,
    #[doc(hidden)]
    UnknownVariant(UnknownMediaFormat),
}

impl Default for MediaFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for MediaFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for MediaFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for MediaFormat {
    fn into(self) -> String {
        match self {
            MediaFormat::Amr => "amr".to_string(),
            MediaFormat::Flac => "flac".to_string(),
            MediaFormat::Mp3 => "mp3".to_string(),
            MediaFormat::Mp4 => "mp4".to_string(),
            MediaFormat::Ogg => "ogg".to_string(),
            MediaFormat::Wav => "wav".to_string(),
            MediaFormat::Webm => "webm".to_string(),
            MediaFormat::UnknownVariant(UnknownMediaFormat { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a MediaFormat {
    fn into(self) -> &'a str {
        match self {
            MediaFormat::Amr => &"amr",
            MediaFormat::Flac => &"flac",
            MediaFormat::Mp3 => &"mp3",
            MediaFormat::Mp4 => &"mp4",
            MediaFormat::Ogg => &"ogg",
            MediaFormat::Wav => &"wav",
            MediaFormat::Webm => &"webm",
            MediaFormat::UnknownVariant(UnknownMediaFormat { name: original }) => original,
        }
    }
}

impl From<&str> for MediaFormat {
    fn from(name: &str) -> Self {
        match name {
            "amr" => MediaFormat::Amr,
            "flac" => MediaFormat::Flac,
            "mp3" => MediaFormat::Mp3,
            "mp4" => MediaFormat::Mp4,
            "ogg" => MediaFormat::Ogg,
            "wav" => MediaFormat::Wav,
            "webm" => MediaFormat::Webm,
            _ => MediaFormat::UnknownVariant(UnknownMediaFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for MediaFormat {
    fn from(name: String) -> Self {
        match &*name {
            "amr" => MediaFormat::Amr,
            "flac" => MediaFormat::Flac,
            "mp3" => MediaFormat::Mp3,
            "mp4" => MediaFormat::Mp4,
            "ogg" => MediaFormat::Ogg,
            "wav" => MediaFormat::Wav,
            "webm" => MediaFormat::Webm,
            _ => MediaFormat::UnknownVariant(UnknownMediaFormat { name }),
        }
    }
}

impl ::std::str::FromStr for MediaFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for MediaFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MediaFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Identifies the location of a medical transcript.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MedicalTranscript {
    /// <p>The S3 object location of the medical transcript.</p> <p>Use this URI to access the medical transcript. This URI points to the S3 bucket you created to store the medical transcript.</p>
    #[serde(rename = "TranscriptFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_file_uri: Option<String>,
}

/// <p>The data structure that contains the information for a medical transcription job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MedicalTranscriptionJob {
    /// <p>A timestamp that shows when the job was completed.</p>
    #[serde(rename = "CompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// <p>A timestamp that shows when the job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p><p>If the <code>TranscriptionJobStatus</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p> <p>The <code>FailureReason</code> field contains one of the following values:</p> <ul> <li> <p> <code>Unsupported media format</code>- The media format specified in the <code>MediaFormat</code> field of the request isn&#39;t valid. See the description of the <code>MediaFormat</code> field for a list of valid values.</p> </li> <li> <p> <code>The media format provided does not match the detected media format</code>- The media format of the audio file doesn&#39;t match the format specified in the <code>MediaFormat</code> field in the request. Check the media format of your media file and make sure the two values match.</p> </li> <li> <p> <code>Invalid sample rate for audio file</code>- The sample rate specified in the <code>MediaSampleRateHertz</code> of the request isn&#39;t valid. The sample rate must be between 8000 and 48000 Hertz.</p> </li> <li> <p> <code>The sample rate provided does not match the detected sample rate</code>- The sample rate in the audio file doesn&#39;t match the sample rate specified in the <code>MediaSampleRateHertz</code> field in the request. Check the sample rate of your media file and make sure that the two values match.</p> </li> <li> <p> <code>Invalid file size: file size too large</code>- The size of your audio file is larger than what Amazon Transcribe Medical can process. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/limits-guidelines.html#limits">Guidelines and Quotas</a> in the <i>Amazon Transcribe Medical Guide</i> </p> </li> <li> <p> <code>Invalid number of channels: number of channels too large</code>- Your audio contains more channels than Amazon Transcribe Medical is configured to process. To request additional channels, see <a href="https://docs.aws.amazon.com/general/latest/gr/transcribe-medical.html">Amazon Transcribe Medical Endpoints and Quotas</a> in the <i>Amazon Web Services General Reference</i> </p> </li> </ul></p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language code for the language spoken in the source audio file. US English (en-US) is the only supported language for medical transcriptions. Any other value you enter for language code results in a <code>BadRequestException</code> error.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    #[serde(rename = "Media")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    /// <p>The format of the input media file.</p>
    #[serde(rename = "MediaFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<MediaFormat>,
    /// <p>The sample rate, in Hertz, of the source audio containing medical information.</p> <p>If you don't specify the sample rate, Amazon Transcribe Medical determines it for you. If you choose to specify the sample rate, it must match the rate detected by Amazon Transcribe Medical. In most cases, you should leave the <code>MediaSampleHertz</code> blank and let Amazon Transcribe Medical determine the sample rate.</p>
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i64>,
    /// <p>The name for a given medical transcription job.</p>
    #[serde(rename = "MedicalTranscriptionJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job_name: Option<String>,
    /// <p>Object that contains object.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MedicalTranscriptionSetting>,
    /// <p><p>The medical specialty of any clinicians providing a dictation or having a conversation. <code>PRIMARYCARE</code> is the only available setting for this object. This specialty enables you to generate transcriptions for the following medical fields:</p> <ul> <li> <p>Family Medicine</p> </li> </ul></p>
    #[serde(rename = "Specialty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specialty: Option<Specialty>,
    /// <p>A timestamp that shows when the job started processing.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>An object that contains the <code>MedicalTranscript</code>. The <code>MedicalTranscript</code> contains the <code>TranscriptFileUri</code>.</p>
    #[serde(rename = "Transcript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<MedicalTranscript>,
    /// <p>The completion status of a medical transcription job.</p>
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<TranscriptionJobStatus>,
    /// <p>The type of speech in the transcription job. <code>CONVERSATION</code> is generally used for patient-physician dialogues. <code>DICTATION</code> is the setting for physicians speaking their notes after seeing a patient. For more information, see <a>how-it-works-med</a> </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

/// <p>Provides summary information about a transcription job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MedicalTranscriptionJobSummary {
    /// <p>A timestamp that shows when the job was completed.</p>
    #[serde(rename = "CompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// <p>A timestamp that shows when the medical transcription job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>If the <code>TranscriptionJobStatus</code> field is <code>FAILED</code>, a description of the error.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language of the transcript in the source audio file.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The name of a medical transcription job.</p>
    #[serde(rename = "MedicalTranscriptionJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job_name: Option<String>,
    /// <p>Indicates the location of the transcription job's output.</p> <p>The <code>CUSTOMER_BUCKET</code> is the S3 location provided in the <code>OutputBucketName</code> field when the </p>
    #[serde(rename = "OutputLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location_type: Option<OutputLocationType>,
    /// <p>The medical specialty of the transcription job. <code>Primary care</code> is the only valid value.</p>
    #[serde(rename = "Specialty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specialty: Option<Specialty>,
    /// <p>A timestamp that shows when the job began processing.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the medical transcription job.</p>
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<TranscriptionJobStatus>,
    /// <p>The speech of the clinician in the input audio.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

/// <p>Optional settings for the <a>StartMedicalTranscriptionJob</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MedicalTranscriptionSetting {
    /// <p>Instructs Amazon Transcribe Medical to process each audio channel separately and then merge the transcription output of each channel into a single transcription.</p> <p>Amazon Transcribe Medical also produces a transcription of each item detected on an audio channel, including the start time and end time of the item and alternative transcriptions of item. The alternative transcriptions also come with confidence scores provided by Amazon Transcribe Medical.</p> <p>You can't set both <code>ShowSpeakerLabels</code> and <code>ChannelIdentification</code> in the same request. If you set both, your request returns a <code>BadRequestException</code> </p>
    #[serde(rename = "ChannelIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_identification: Option<bool>,
    /// <p>The maximum number of alternatives that you tell the service to return. If you specify the <code>MaxAlternatives</code> field, you must set the <code>ShowAlternatives</code> field to true.</p>
    #[serde(rename = "MaxAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_alternatives: Option<i64>,
    /// <p>The maximum number of speakers to identify in the input audio. If there are more speakers in the audio than this number, multiple speakers are identified as a single speaker. If you specify the <code>MaxSpeakerLabels</code> field, you must set the <code>ShowSpeakerLabels</code> field to true.</p>
    #[serde(rename = "MaxSpeakerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speaker_labels: Option<i64>,
    /// <p>Determines whether alternative transcripts are generated along with the transcript that has the highest confidence. If you set <code>ShowAlternatives</code> field to true, you must also set the maximum number of alternatives to return in the <code>MaxAlternatives</code> field.</p>
    #[serde(rename = "ShowAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alternatives: Option<bool>,
    /// <p>Determines whether the transcription job uses speaker recognition to identify different speakers in the input audio. Speaker recognition labels individual speakers in the audio file. If you set the <code>ShowSpeakerLabels</code> field to true, you must also set the maximum number of speaker labels in the <code>MaxSpeakerLabels</code> field.</p> <p>You can't set both <code>ShowSpeakerLabels</code> and <code>ChannelIdentification</code> in the same request. If you set both, your request returns a <code>BadRequestException</code>.</p>
    #[serde(rename = "ShowSpeakerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_speaker_labels: Option<bool>,
    /// <p>The name of the vocabulary to use when processing a medical transcription job.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

/// <p>The object used to call your custom language model to your transcription job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ModelSettings {
    /// <p>The name of your custom language model.</p>
    #[serde(rename = "LanguageModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownModelStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ModelStatus {
    Completed,
    Failed,
    InProgress,
    #[doc(hidden)]
    UnknownVariant(UnknownModelStatus),
}

impl Default for ModelStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ModelStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ModelStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ModelStatus {
    fn into(self) -> String {
        match self {
            ModelStatus::Completed => "COMPLETED".to_string(),
            ModelStatus::Failed => "FAILED".to_string(),
            ModelStatus::InProgress => "IN_PROGRESS".to_string(),
            ModelStatus::UnknownVariant(UnknownModelStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ModelStatus {
    fn into(self) -> &'a str {
        match self {
            ModelStatus::Completed => &"COMPLETED",
            ModelStatus::Failed => &"FAILED",
            ModelStatus::InProgress => &"IN_PROGRESS",
            ModelStatus::UnknownVariant(UnknownModelStatus { name: original }) => original,
        }
    }
}

impl From<&str> for ModelStatus {
    fn from(name: &str) -> Self {
        match name {
            "COMPLETED" => ModelStatus::Completed,
            "FAILED" => ModelStatus::Failed,
            "IN_PROGRESS" => ModelStatus::InProgress,
            _ => ModelStatus::UnknownVariant(UnknownModelStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ModelStatus {
    fn from(name: String) -> Self {
        match &*name {
            "COMPLETED" => ModelStatus::Completed,
            "FAILED" => ModelStatus::Failed,
            "IN_PROGRESS" => ModelStatus::InProgress,
            _ => ModelStatus::UnknownVariant(UnknownModelStatus { name }),
        }
    }
}

impl ::std::str::FromStr for ModelStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ModelStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ModelStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOutputLocationType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum OutputLocationType {
    CustomerBucket,
    ServiceBucket,
    #[doc(hidden)]
    UnknownVariant(UnknownOutputLocationType),
}

impl Default for OutputLocationType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for OutputLocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for OutputLocationType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for OutputLocationType {
    fn into(self) -> String {
        match self {
            OutputLocationType::CustomerBucket => "CUSTOMER_BUCKET".to_string(),
            OutputLocationType::ServiceBucket => "SERVICE_BUCKET".to_string(),
            OutputLocationType::UnknownVariant(UnknownOutputLocationType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a OutputLocationType {
    fn into(self) -> &'a str {
        match self {
            OutputLocationType::CustomerBucket => &"CUSTOMER_BUCKET",
            OutputLocationType::ServiceBucket => &"SERVICE_BUCKET",
            OutputLocationType::UnknownVariant(UnknownOutputLocationType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for OutputLocationType {
    fn from(name: &str) -> Self {
        match name {
            "CUSTOMER_BUCKET" => OutputLocationType::CustomerBucket,
            "SERVICE_BUCKET" => OutputLocationType::ServiceBucket,
            _ => OutputLocationType::UnknownVariant(UnknownOutputLocationType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for OutputLocationType {
    fn from(name: String) -> Self {
        match &*name {
            "CUSTOMER_BUCKET" => OutputLocationType::CustomerBucket,
            "SERVICE_BUCKET" => OutputLocationType::ServiceBucket,
            _ => OutputLocationType::UnknownVariant(UnknownOutputLocationType { name }),
        }
    }
}

impl ::std::str::FromStr for OutputLocationType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for OutputLocationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OutputLocationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRedactionOutput {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RedactionOutput {
    Redacted,
    RedactedAndUnredacted,
    #[doc(hidden)]
    UnknownVariant(UnknownRedactionOutput),
}

impl Default for RedactionOutput {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RedactionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RedactionOutput {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RedactionOutput {
    fn into(self) -> String {
        match self {
            RedactionOutput::Redacted => "redacted".to_string(),
            RedactionOutput::RedactedAndUnredacted => "redacted_and_unredacted".to_string(),
            RedactionOutput::UnknownVariant(UnknownRedactionOutput { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RedactionOutput {
    fn into(self) -> &'a str {
        match self {
            RedactionOutput::Redacted => &"redacted",
            RedactionOutput::RedactedAndUnredacted => &"redacted_and_unredacted",
            RedactionOutput::UnknownVariant(UnknownRedactionOutput { name: original }) => original,
        }
    }
}

impl From<&str> for RedactionOutput {
    fn from(name: &str) -> Self {
        match name {
            "redacted" => RedactionOutput::Redacted,
            "redacted_and_unredacted" => RedactionOutput::RedactedAndUnredacted,
            _ => RedactionOutput::UnknownVariant(UnknownRedactionOutput {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RedactionOutput {
    fn from(name: String) -> Self {
        match &*name {
            "redacted" => RedactionOutput::Redacted,
            "redacted_and_unredacted" => RedactionOutput::RedactedAndUnredacted,
            _ => RedactionOutput::UnknownVariant(UnknownRedactionOutput { name }),
        }
    }
}

impl ::std::str::FromStr for RedactionOutput {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for RedactionOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RedactionOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRedactionType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RedactionType {
    Pii,
    #[doc(hidden)]
    UnknownVariant(UnknownRedactionType),
}

impl Default for RedactionType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RedactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RedactionType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RedactionType {
    fn into(self) -> String {
        match self {
            RedactionType::Pii => "PII".to_string(),
            RedactionType::UnknownVariant(UnknownRedactionType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RedactionType {
    fn into(self) -> &'a str {
        match self {
            RedactionType::Pii => &"PII",
            RedactionType::UnknownVariant(UnknownRedactionType { name: original }) => original,
        }
    }
}

impl From<&str> for RedactionType {
    fn from(name: &str) -> Self {
        match name {
            "PII" => RedactionType::Pii,
            _ => RedactionType::UnknownVariant(UnknownRedactionType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RedactionType {
    fn from(name: String) -> Self {
        match &*name {
            "PII" => RedactionType::Pii,
            _ => RedactionType::UnknownVariant(UnknownRedactionType { name }),
        }
    }
}

impl ::std::str::FromStr for RedactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for RedactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RedactionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides optional settings for the <code>StartTranscriptionJob</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    /// <p>Instructs Amazon Transcribe to process each audio channel separately and then merge the transcription output of each channel into a single transcription. </p> <p>Amazon Transcribe also produces a transcription of each item detected on an audio channel, including the start time and end time of the item and alternative transcriptions of the item including the confidence that Amazon Transcribe has in the transcription.</p> <p>You can't set both <code>ShowSpeakerLabels</code> and <code>ChannelIdentification</code> in the same request. If you set both, your request returns a <code>BadRequestException</code>.</p>
    #[serde(rename = "ChannelIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_identification: Option<bool>,
    /// <p>The number of alternative transcriptions that the service should return. If you specify the <code>MaxAlternatives</code> field, you must set the <code>ShowAlternatives</code> field to true.</p>
    #[serde(rename = "MaxAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_alternatives: Option<i64>,
    /// <p>The maximum number of speakers to identify in the input audio. If there are more speakers in the audio than this number, multiple speakers are identified as a single speaker. If you specify the <code>MaxSpeakerLabels</code> field, you must set the <code>ShowSpeakerLabels</code> field to true.</p>
    #[serde(rename = "MaxSpeakerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speaker_labels: Option<i64>,
    /// <p>Determines whether the transcription contains alternative transcriptions. If you set the <code>ShowAlternatives</code> field to true, you must also set the maximum number of alternatives to return in the <code>MaxAlternatives</code> field.</p>
    #[serde(rename = "ShowAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alternatives: Option<bool>,
    /// <p>Determines whether the transcription job uses speaker recognition to identify different speakers in the input audio. Speaker recognition labels individual speakers in the audio file. If you set the <code>ShowSpeakerLabels</code> field to true, you must also set the maximum number of speaker labels <code>MaxSpeakerLabels</code> field.</p> <p>You can't set both <code>ShowSpeakerLabels</code> and <code>ChannelIdentification</code> in the same request. If you set both, your request returns a <code>BadRequestException</code>.</p>
    #[serde(rename = "ShowSpeakerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_speaker_labels: Option<bool>,
    /// <p>Set to <code>mask</code> to remove filtered text from the transcript and replace it with three asterisks ("***") as placeholder text. Set to <code>remove</code> to remove filtered text from the transcript without using placeholder text.</p>
    #[serde(rename = "VocabularyFilterMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_method: Option<VocabularyFilterMethod>,
    /// <p>The name of the vocabulary filter to use when transcribing the audio. The filter that you specify must have the same language code as the transcription job.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
    /// <p>The name of a vocabulary to use when processing the transcription job.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSpecialty {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Specialty {
    Primarycare,
    #[doc(hidden)]
    UnknownVariant(UnknownSpecialty),
}

impl Default for Specialty {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Specialty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Specialty {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Specialty {
    fn into(self) -> String {
        match self {
            Specialty::Primarycare => "PRIMARYCARE".to_string(),
            Specialty::UnknownVariant(UnknownSpecialty { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Specialty {
    fn into(self) -> &'a str {
        match self {
            Specialty::Primarycare => &"PRIMARYCARE",
            Specialty::UnknownVariant(UnknownSpecialty { name: original }) => original,
        }
    }
}

impl From<&str> for Specialty {
    fn from(name: &str) -> Self {
        match name {
            "PRIMARYCARE" => Specialty::Primarycare,
            _ => Specialty::UnknownVariant(UnknownSpecialty {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Specialty {
    fn from(name: String) -> Self {
        match &*name {
            "PRIMARYCARE" => Specialty::Primarycare,
            _ => Specialty::UnknownVariant(UnknownSpecialty { name }),
        }
    }
}

impl ::std::str::FromStr for Specialty {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Specialty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Specialty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMedicalTranscriptionJobRequest {
    /// <p>The language code for the language spoken in the input media file. US English (en-US) is the valid value for medical transcription jobs. Any other value you enter for language code results in a <code>BadRequestException</code> error.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: LanguageCode,
    #[serde(rename = "Media")]
    pub media: Media,
    /// <p>The audio format of the input media file.</p>
    #[serde(rename = "MediaFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<MediaFormat>,
    /// <p>The sample rate, in Hertz, of the audio track in the input media file.</p> <p>If you do not specify the media sample rate, Amazon Transcribe Medical determines the sample rate. If you specify the sample rate, it must match the rate detected by Amazon Transcribe Medical. In most cases, you should leave the <code>MediaSampleRateHertz</code> field blank and let Amazon Transcribe Medical determine the sample rate.</p>
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i64>,
    /// <p>The name of the medical transcription job. You can't use the strings "<code>.</code>" or "<code>..</code>" by themselves as the job name. The name must also be unique within an AWS account. If you try to create a medical transcription job with the same name as a previous medical transcription job, you get a <code>ConflictException</code> error.</p>
    #[serde(rename = "MedicalTranscriptionJobName")]
    pub medical_transcription_job_name: String,
    /// <p>The Amazon S3 location where the transcription is stored.</p> <p>You must set <code>OutputBucketName</code> for Amazon Transcribe Medical to store the transcription results. Your transcript appears in the S3 location you specify. When you call the <a>GetMedicalTranscriptionJob</a>, the operation returns this location in the <code>TranscriptFileUri</code> field. The S3 bucket must have permissions that allow Amazon Transcribe Medical to put files in the bucket. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/security_iam_id-based-policy-examples.html#auth-role-iam-user">Permissions Required for IAM User Roles</a>.</p> <p>You can specify an AWS Key Management Service (KMS) key to encrypt the output of your transcription using the <code>OutputEncryptionKMSKeyId</code> parameter. If you don't specify a KMS key, Amazon Transcribe Medical uses the default Amazon S3 key for server-side encryption of transcripts that are placed in your S3 bucket.</p>
    #[serde(rename = "OutputBucketName")]
    pub output_bucket_name: String,
    /// <p>The Amazon Resource Name (ARN) of the AWS Key Management Service (KMS) key used to encrypt the output of the transcription job. The user calling the <a>StartMedicalTranscriptionJob</a> operation must have permission to use the specified KMS key.</p> <p>You use either of the following to identify a KMS key in the current account:</p> <ul> <li> <p>KMS Key ID: "1234abcd-12ab-34cd-56ef-1234567890ab"</p> </li> <li> <p>KMS Key Alias: "alias/ExampleAlias"</p> </li> </ul> <p>You can use either of the following to identify a KMS key in the current account or another account:</p> <ul> <li> <p>Amazon Resource Name (ARN) of a KMS key in the current account or another account: "arn:aws:kms:region:account ID:key/1234abcd-12ab-34cd-56ef-1234567890ab"</p> </li> <li> <p>ARN of a KMS Key Alias: "arn:aws:kms:region:account ID:alias/ExampleAlias"</p> </li> </ul> <p>If you don't specify an encryption key, the output of the medical transcription job is encrypted with the default Amazon S3 key (SSE-S3).</p> <p>If you specify a KMS key to encrypt your output, you must also specify an output location in the <code>OutputBucketName</code> parameter.</p>
    #[serde(rename = "OutputEncryptionKMSKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encryption_kms_key_id: Option<String>,
    /// <p>You can specify a location in an Amazon S3 bucket to store the output of your medical transcription job.</p> <p>If you don't specify an output key, Amazon Transcribe Medical stores the output of your transcription job in the Amazon S3 bucket you specified. By default, the object key is "your-transcription-job-name.json".</p> <p>You can use output keys to specify the Amazon S3 prefix and file name of the transcription output. For example, specifying the Amazon S3 prefix, "folder1/folder2/", as an output key would lead to the output being stored as "folder1/folder2/your-transcription-job-name.json". If you specify "my-other-job-name.json" as the output key, the object key is changed to "my-other-job-name.json". You can use an output key to change both the prefix and the file name, for example "folder/my-other-job-name.json".</p> <p>If you specify an output key, you must also specify an S3 bucket in the <code>OutputBucketName</code> parameter.</p>
    #[serde(rename = "OutputKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    /// <p>Optional settings for the medical transcription job.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MedicalTranscriptionSetting>,
    /// <p>The medical specialty of any clinician speaking in the input media.</p>
    #[serde(rename = "Specialty")]
    pub specialty: Specialty,
    /// <p>The type of speech in the input audio. <code>CONVERSATION</code> refers to conversations between two or more speakers, e.g., a conversations between doctors and patients. <code>DICTATION</code> refers to single-speaker dictated speech, e.g., for clinical notes.</p>
    #[serde(rename = "Type")]
    pub type_: Type,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMedicalTranscriptionJobResponse {
    /// <p>A batch job submitted to transcribe medical speech to text.</p>
    #[serde(rename = "MedicalTranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job: Option<MedicalTranscriptionJob>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartTranscriptionJobRequest {
    /// <p>An object that contains the request parameters for content redaction.</p>
    #[serde(rename = "ContentRedaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_redaction: Option<ContentRedaction>,
    /// <p>Set this field to <code>true</code> to enable automatic language identification. Automatic language identification is disabled by default. You receive a <code>BadRequestException</code> error if you enter a value for a <code>LanguageCode</code>.</p>
    #[serde(rename = "IdentifyLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_language: Option<bool>,
    /// <p>Provides information about how a transcription job is executed. Use this field to indicate that the job can be queued for deferred execution if the concurrency limit is reached and there are no slots available to immediately run the job.</p>
    #[serde(rename = "JobExecutionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_settings: Option<JobExecutionSettings>,
    /// <p>The language code for the language used in the input media file.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>An object containing a list of languages that might be present in your collection of audio files. Automatic language identification chooses a language that best matches the source audio from that list.</p>
    #[serde(rename = "LanguageOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_options: Option<Vec<LanguageCode>>,
    /// <p>An object that describes the input media for a transcription job.</p>
    #[serde(rename = "Media")]
    pub media: Media,
    /// <p>The format of the input media file.</p>
    #[serde(rename = "MediaFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<MediaFormat>,
    /// <p>The sample rate, in Hertz, of the audio track in the input media file. </p> <p>If you do not specify the media sample rate, Amazon Transcribe determines the sample rate. If you specify the sample rate, it must match the sample rate detected by Amazon Transcribe. In most cases, you should leave the <code>MediaSampleRateHertz</code> field blank and let Amazon Transcribe determine the sample rate.</p>
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i64>,
    /// <p>Choose the custom language model you use for your transcription job in this parameter.</p>
    #[serde(rename = "ModelSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<ModelSettings>,
    /// <p>The location where the transcription is stored.</p> <p>If you set the <code>OutputBucketName</code>, Amazon Transcribe puts the transcript in the specified S3 bucket. When you call the <a>GetTranscriptionJob</a> operation, the operation returns this location in the <code>TranscriptFileUri</code> field. If you enable content redaction, the redacted transcript appears in <code>RedactedTranscriptFileUri</code>. If you enable content redaction and choose to output an unredacted transcript, that transcript's location still appears in the <code>TranscriptFileUri</code>. The S3 bucket must have permissions that allow Amazon Transcribe to put files in the bucket. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/security_iam_id-based-policy-examples.html#auth-role-iam-user">Permissions Required for IAM User Roles</a>.</p> <p>You can specify an AWS Key Management Service (KMS) key to encrypt the output of your transcription using the <code>OutputEncryptionKMSKeyId</code> parameter. If you don't specify a KMS key, Amazon Transcribe uses the default Amazon S3 key for server-side encryption of transcripts that are placed in your S3 bucket.</p> <p>If you don't set the <code>OutputBucketName</code>, Amazon Transcribe generates a pre-signed URL, a shareable URL that provides secure access to your transcription, and returns it in the <code>TranscriptFileUri</code> field. Use this URL to download the transcription.</p>
    #[serde(rename = "OutputBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bucket_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Key Management Service (KMS) key used to encrypt the output of the transcription job. The user calling the <code>StartTranscriptionJob</code> operation must have permission to use the specified KMS key.</p> <p>You can use either of the following to identify a KMS key in the current account:</p> <ul> <li> <p>KMS Key ID: "1234abcd-12ab-34cd-56ef-1234567890ab"</p> </li> <li> <p>KMS Key Alias: "alias/ExampleAlias"</p> </li> </ul> <p>You can use either of the following to identify a KMS key in the current account or another account:</p> <ul> <li> <p>Amazon Resource Name (ARN) of a KMS Key: "arn:aws:kms:region:account ID:key/1234abcd-12ab-34cd-56ef-1234567890ab"</p> </li> <li> <p>ARN of a KMS Key Alias: "arn:aws:kms:region:account ID:alias/ExampleAlias"</p> </li> </ul> <p>If you don't specify an encryption key, the output of the transcription job is encrypted with the default Amazon S3 key (SSE-S3). </p> <p>If you specify a KMS key to encrypt your output, you must also specify an output location in the <code>OutputBucketName</code> parameter.</p>
    #[serde(rename = "OutputEncryptionKMSKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encryption_kms_key_id: Option<String>,
    /// <p>You can specify a location in an Amazon S3 bucket to store the output of your transcription job.</p> <p>If you don't specify an output key, Amazon Transcribe stores the output of your transcription job in the Amazon S3 bucket you specified. By default, the object key is "your-transcription-job-name.json".</p> <p>You can use output keys to specify the Amazon S3 prefix and file name of the transcription output. For example, specifying the Amazon S3 prefix, "folder1/folder2/", as an output key would lead to the output being stored as "folder1/folder2/your-transcription-job-name.json". If you specify "my-other-job-name.json" as the output key, the object key is changed to "my-other-job-name.json". You can use an output key to change both the prefix and the file name, for example "folder/my-other-job-name.json".</p> <p>If you specify an output key, you must also specify an S3 bucket in the <code>OutputBucketName</code> parameter.</p>
    #[serde(rename = "OutputKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    /// <p>A <code>Settings</code> object that provides optional settings for a transcription job.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    /// <p>The name of the job. You can't use the strings "<code>.</code>" or "<code>..</code>" by themselves as the job name. The name must also be unique within an AWS account. If you try to create a transcription job with the same name as a previous transcription job, you get a <code>ConflictException</code> error.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTranscriptionJobResponse {
    /// <p>An object containing details of the asynchronous transcription job.</p>
    #[serde(rename = "TranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

/// <p>Identifies the location of a transcription.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Transcript {
    /// <p>The S3 object location of the redacted transcript.</p> <p>Use this URI to access the redacted transcript. If you specified an S3 bucket in the <code>OutputBucketName</code> field when you created the job, this is the URI of that bucket. If you chose to store the transcript in Amazon Transcribe, this is a shareable URL that provides secure access to that location.</p>
    #[serde(rename = "RedactedTranscriptFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_transcript_file_uri: Option<String>,
    /// <p>The S3 object location of the transcript.</p> <p>Use this URI to access the transcript. If you specified an S3 bucket in the <code>OutputBucketName</code> field when you created the job, this is the URI of that bucket. If you chose to store the transcript in Amazon Transcribe, this is a shareable URL that provides secure access to that location.</p>
    #[serde(rename = "TranscriptFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_file_uri: Option<String>,
}

/// <p>Describes an asynchronous transcription job that was created with the <code>StartTranscriptionJob</code> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TranscriptionJob {
    /// <p>A timestamp that shows when the job was completed.</p>
    #[serde(rename = "CompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// <p>An object that describes content redaction settings for the transcription job.</p>
    #[serde(rename = "ContentRedaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_redaction: Option<ContentRedaction>,
    /// <p>A timestamp that shows when the job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p><p>If the <code>TranscriptionJobStatus</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p> <p>The <code>FailureReason</code> field can contain one of the following values:</p> <ul> <li> <p> <code>Unsupported media format</code> - The media format specified in the <code>MediaFormat</code> field of the request isn&#39;t valid. See the description of the <code>MediaFormat</code> field for a list of valid values.</p> </li> <li> <p> <code>The media format provided does not match the detected media format</code> - The media format of the audio file doesn&#39;t match the format specified in the <code>MediaFormat</code> field in the request. Check the media format of your media file and make sure that the two values match.</p> </li> <li> <p> <code>Invalid sample rate for audio file</code> - The sample rate specified in the <code>MediaSampleRateHertz</code> of the request isn&#39;t valid. The sample rate must be between 8000 and 48000 Hertz.</p> </li> <li> <p> <code>The sample rate provided does not match the detected sample rate</code> - The sample rate in the audio file doesn&#39;t match the sample rate specified in the <code>MediaSampleRateHertz</code> field in the request. Check the sample rate of your media file and make sure that the two values match.</p> </li> <li> <p> <code>Invalid file size: file size too large</code> - The size of your audio file is larger than Amazon Transcribe can process. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/limits-guidelines.html#limits">Limits</a> in the <i>Amazon Transcribe Developer Guide</i>.</p> </li> <li> <p> <code>Invalid number of channels: number of channels too large</code> - Your audio contains more channels than Amazon Transcribe is configured to process. To request additional channels, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits-amazon-transcribe">Amazon Transcribe Limits</a> in the <i>Amazon Web Services General Reference</i>.</p> </li> </ul></p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>A value between zero and one that Amazon Transcribe assigned to the language that it identified in the source audio. Larger values indicate that Amazon Transcribe has higher confidence in the language it identified.</p>
    #[serde(rename = "IdentifiedLanguageScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_language_score: Option<f32>,
    /// <p>A value that shows if automatic language identification was enabled for a transcription job.</p>
    #[serde(rename = "IdentifyLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_language: Option<bool>,
    /// <p>Provides information about how a transcription job is executed.</p>
    #[serde(rename = "JobExecutionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_settings: Option<JobExecutionSettings>,
    /// <p>The language code for the input speech.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>An object that shows the optional array of languages inputted for transcription jobs with automatic language identification enabled.</p>
    #[serde(rename = "LanguageOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_options: Option<Vec<LanguageCode>>,
    /// <p>An object that describes the input media for the transcription job.</p>
    #[serde(rename = "Media")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    /// <p>The format of the input media file.</p>
    #[serde(rename = "MediaFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<MediaFormat>,
    /// <p>The sample rate, in Hertz, of the audio track in the input media file. </p>
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i64>,
    /// <p>An object containing the details of your custom language model.</p>
    #[serde(rename = "ModelSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<ModelSettings>,
    /// <p>Optional settings for the transcription job. Use these settings to turn on speaker recognition, to set the maximum number of speakers that should be identified and to specify a custom vocabulary to use when processing the transcription job.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    /// <p>A timestamp that shows with the job was started processing.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>An object that describes the output of the transcription job.</p>
    #[serde(rename = "Transcript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Transcript>,
    /// <p>The name of the transcription job.</p>
    #[serde(rename = "TranscriptionJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_name: Option<String>,
    /// <p>The status of the transcription job.</p>
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<TranscriptionJobStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownTranscriptionJobStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum TranscriptionJobStatus {
    Completed,
    Failed,
    InProgress,
    Queued,
    #[doc(hidden)]
    UnknownVariant(UnknownTranscriptionJobStatus),
}

impl Default for TranscriptionJobStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for TranscriptionJobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for TranscriptionJobStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for TranscriptionJobStatus {
    fn into(self) -> String {
        match self {
            TranscriptionJobStatus::Completed => "COMPLETED".to_string(),
            TranscriptionJobStatus::Failed => "FAILED".to_string(),
            TranscriptionJobStatus::InProgress => "IN_PROGRESS".to_string(),
            TranscriptionJobStatus::Queued => "QUEUED".to_string(),
            TranscriptionJobStatus::UnknownVariant(UnknownTranscriptionJobStatus {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a TranscriptionJobStatus {
    fn into(self) -> &'a str {
        match self {
            TranscriptionJobStatus::Completed => &"COMPLETED",
            TranscriptionJobStatus::Failed => &"FAILED",
            TranscriptionJobStatus::InProgress => &"IN_PROGRESS",
            TranscriptionJobStatus::Queued => &"QUEUED",
            TranscriptionJobStatus::UnknownVariant(UnknownTranscriptionJobStatus {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for TranscriptionJobStatus {
    fn from(name: &str) -> Self {
        match name {
            "COMPLETED" => TranscriptionJobStatus::Completed,
            "FAILED" => TranscriptionJobStatus::Failed,
            "IN_PROGRESS" => TranscriptionJobStatus::InProgress,
            "QUEUED" => TranscriptionJobStatus::Queued,
            _ => TranscriptionJobStatus::UnknownVariant(UnknownTranscriptionJobStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for TranscriptionJobStatus {
    fn from(name: String) -> Self {
        match &*name {
            "COMPLETED" => TranscriptionJobStatus::Completed,
            "FAILED" => TranscriptionJobStatus::Failed,
            "IN_PROGRESS" => TranscriptionJobStatus::InProgress,
            "QUEUED" => TranscriptionJobStatus::Queued,
            _ => TranscriptionJobStatus::UnknownVariant(UnknownTranscriptionJobStatus { name }),
        }
    }
}

impl ::std::str::FromStr for TranscriptionJobStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for TranscriptionJobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TranscriptionJobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides a summary of information about a transcription job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TranscriptionJobSummary {
    /// <p>A timestamp that shows when the job was completed.</p>
    #[serde(rename = "CompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// <p>The content redaction settings of the transcription job.</p>
    #[serde(rename = "ContentRedaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_redaction: Option<ContentRedaction>,
    /// <p>A timestamp that shows when the job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>If the <code>TranscriptionJobStatus</code> field is <code>FAILED</code>, a description of the error.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>A value between zero and one that Amazon Transcribe assigned to the language it identified in the source audio. A higher score indicates that Amazon Transcribe is more confident in the language it identified.</p>
    #[serde(rename = "IdentifiedLanguageScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_language_score: Option<f32>,
    /// <p>Whether automatic language identification was enabled for a transcription job.</p>
    #[serde(rename = "IdentifyLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_language: Option<bool>,
    /// <p>The language code for the input speech.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    #[serde(rename = "ModelSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<ModelSettings>,
    /// <p>Indicates the location of the output of the transcription job.</p> <p>If the value is <code>CUSTOMER_BUCKET</code> then the location is the S3 bucket specified in the <code>outputBucketName</code> field when the transcription job was started with the <code>StartTranscriptionJob</code> operation.</p> <p>If the value is <code>SERVICE_BUCKET</code> then the output is stored by Amazon Transcribe and can be retrieved using the URI in the <code>GetTranscriptionJob</code> response's <code>TranscriptFileUri</code> field.</p>
    #[serde(rename = "OutputLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location_type: Option<OutputLocationType>,
    /// <p>A timestamp that shows when the job started processing.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The name of the transcription job.</p>
    #[serde(rename = "TranscriptionJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_name: Option<String>,
    /// <p>The status of the transcription job. When the status is <code>COMPLETED</code>, use the <code>GetTranscriptionJob</code> operation to get the results of the transcription.</p>
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<TranscriptionJobStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Type {
    Conversation,
    Dictation,
    #[doc(hidden)]
    UnknownVariant(UnknownType),
}

impl Default for Type {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for Type {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for Type {
    fn into(self) -> String {
        match self {
            Type::Conversation => "CONVERSATION".to_string(),
            Type::Dictation => "DICTATION".to_string(),
            Type::UnknownVariant(UnknownType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a Type {
    fn into(self) -> &'a str {
        match self {
            Type::Conversation => &"CONVERSATION",
            Type::Dictation => &"DICTATION",
            Type::UnknownVariant(UnknownType { name: original }) => original,
        }
    }
}

impl From<&str> for Type {
    fn from(name: &str) -> Self {
        match name {
            "CONVERSATION" => Type::Conversation,
            "DICTATION" => Type::Dictation,
            _ => Type::UnknownVariant(UnknownType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for Type {
    fn from(name: String) -> Self {
        match &*name {
            "CONVERSATION" => Type::Conversation,
            "DICTATION" => Type::Dictation,
            _ => Type::UnknownVariant(UnknownType { name }),
        }
    }
}

impl ::std::str::FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMedicalVocabularyRequest {
    /// <p>The language code of the language used for the entries in the updated vocabulary. US English (en-US) is the only valid language code in Amazon Transcribe Medical.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: LanguageCode,
    /// <p>The location in Amazon S3 of the text file that contains the you use for your custom vocabulary. The URI must be in the same AWS Region as the resource that you are calling. The following is the format for a URI:</p> <p> <code> https://s3.&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>For example:</p> <p> <code>https://s3.us-east-1.amazonaws.com/AWSDOC-EXAMPLE-BUCKET/vocab.txt</code> </p> <p>For more information about Amazon S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>For more information about custom vocabularies in Amazon Transcribe Medical, see <a href="http://docs.aws.amazon.com/transcribe/latest/dg/how-it-works.html#how-vocabulary">Medical Custom Vocabularies</a>.</p>
    #[serde(rename = "VocabularyFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    /// <p>The name of the vocabulary to update. The name is case sensitive. If you try to update a vocabulary with the same name as a vocabulary you've already made, you get a <code>ConflictException</code> error.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMedicalVocabularyResponse {
    /// <p>The language code for the language of the text file used to update the custom vocabulary. US English (en-US) is the only language supported in Amazon Transcribe Medical.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary was updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the updated vocabulary.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the update to the vocabulary. When the <code>VocabularyState</code> field is <code>READY</code>, the vocabulary is ready to be used in a <code>StartMedicalTranscriptionJob</code> request.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<VocabularyState>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVocabularyFilterRequest {
    /// <p>The Amazon S3 location of a text file used as input to create the vocabulary filter. Only use characters from the character set defined for custom vocabularies. For a list of character sets, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-vocabulary.html#charsets">Character Sets for Custom Vocabularies</a>.</p> <p>The specified file must be less than 50 KB of UTF-8 characters.</p> <p>If you provide the location of a list of words in the <code>VocabularyFilterFileUri</code> parameter, you can't use the <code>Words</code> parameter.</p>
    #[serde(rename = "VocabularyFilterFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_file_uri: Option<String>,
    /// <p>The name of the vocabulary filter to update. If you try to update a vocabulary filter with the same name as another vocabulary filter, you get a <code>ConflictException</code> error.</p>
    #[serde(rename = "VocabularyFilterName")]
    pub vocabulary_filter_name: String,
    /// <p>The words to use in the vocabulary filter. Only use characters from the character set defined for custom vocabularies. For a list of character sets, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-vocabulary.html#charsets">Character Sets for Custom Vocabularies</a>.</p> <p>If you provide a list of words in the <code>Words</code> parameter, you can't use the <code>VocabularyFilterFileUri</code> parameter.</p>
    #[serde(rename = "Words")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVocabularyFilterResponse {
    /// <p>The language code of the words in the vocabulary filter.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary filter was updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the updated vocabulary filter.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVocabularyRequest {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: LanguageCode,
    /// <p>An array of strings containing the vocabulary entries.</p>
    #[serde(rename = "Phrases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
    /// <p>The S3 location of the text file that contains the definition of the custom vocabulary. The URI must be in the same region as the API endpoint that you are calling. The general form is </p> <p>For example:</p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>For more information about custom vocabularies, see <a href="http://docs.aws.amazon.com/transcribe/latest/dg/how-it-works.html#how-vocabulary">Custom Vocabularies</a>.</p>
    #[serde(rename = "VocabularyFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    /// <p>The name of the vocabulary to update. The name is case sensitive. If you try to update a vocabulary with the same name as a previous vocabulary you will receive a <code>ConflictException</code> error.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVocabularyResponse {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary was updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary that was updated.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary. When the <code>VocabularyState</code> field contains <code>READY</code> the vocabulary is ready to be used in a <code>StartTranscriptionJob</code> request.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<VocabularyState>,
}

/// <p>Provides information about a vocabulary filter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VocabularyFilterInfo {
    /// <p>The language code of the words in the vocabulary filter.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary was last updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary filter. The name must be unique in the account that holds the filter.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownVocabularyFilterMethod {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum VocabularyFilterMethod {
    Mask,
    Remove,
    #[doc(hidden)]
    UnknownVariant(UnknownVocabularyFilterMethod),
}

impl Default for VocabularyFilterMethod {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for VocabularyFilterMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for VocabularyFilterMethod {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for VocabularyFilterMethod {
    fn into(self) -> String {
        match self {
            VocabularyFilterMethod::Mask => "mask".to_string(),
            VocabularyFilterMethod::Remove => "remove".to_string(),
            VocabularyFilterMethod::UnknownVariant(UnknownVocabularyFilterMethod {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a VocabularyFilterMethod {
    fn into(self) -> &'a str {
        match self {
            VocabularyFilterMethod::Mask => &"mask",
            VocabularyFilterMethod::Remove => &"remove",
            VocabularyFilterMethod::UnknownVariant(UnknownVocabularyFilterMethod {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for VocabularyFilterMethod {
    fn from(name: &str) -> Self {
        match name {
            "mask" => VocabularyFilterMethod::Mask,
            "remove" => VocabularyFilterMethod::Remove,
            _ => VocabularyFilterMethod::UnknownVariant(UnknownVocabularyFilterMethod {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for VocabularyFilterMethod {
    fn from(name: String) -> Self {
        match &*name {
            "mask" => VocabularyFilterMethod::Mask,
            "remove" => VocabularyFilterMethod::Remove,
            _ => VocabularyFilterMethod::UnknownVariant(UnknownVocabularyFilterMethod { name }),
        }
    }
}

impl ::std::str::FromStr for VocabularyFilterMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for VocabularyFilterMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for VocabularyFilterMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Provides information about a custom vocabulary. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VocabularyInfo {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,
    /// <p>The date and time that the vocabulary was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary. If the state is <code>READY</code> you can use the vocabulary in a <code>StartTranscriptionJob</code> request.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<VocabularyState>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownVocabularyState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum VocabularyState {
    Failed,
    Pending,
    Ready,
    #[doc(hidden)]
    UnknownVariant(UnknownVocabularyState),
}

impl Default for VocabularyState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for VocabularyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for VocabularyState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for VocabularyState {
    fn into(self) -> String {
        match self {
            VocabularyState::Failed => "FAILED".to_string(),
            VocabularyState::Pending => "PENDING".to_string(),
            VocabularyState::Ready => "READY".to_string(),
            VocabularyState::UnknownVariant(UnknownVocabularyState { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a VocabularyState {
    fn into(self) -> &'a str {
        match self {
            VocabularyState::Failed => &"FAILED",
            VocabularyState::Pending => &"PENDING",
            VocabularyState::Ready => &"READY",
            VocabularyState::UnknownVariant(UnknownVocabularyState { name: original }) => original,
        }
    }
}

impl From<&str> for VocabularyState {
    fn from(name: &str) -> Self {
        match name {
            "FAILED" => VocabularyState::Failed,
            "PENDING" => VocabularyState::Pending,
            "READY" => VocabularyState::Ready,
            _ => VocabularyState::UnknownVariant(UnknownVocabularyState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for VocabularyState {
    fn from(name: String) -> Self {
        match &*name {
            "FAILED" => VocabularyState::Failed,
            "PENDING" => VocabularyState::Pending,
            "READY" => VocabularyState::Ready,
            _ => VocabularyState::UnknownVariant(UnknownVocabularyState { name }),
        }
    }
}

impl ::std::str::FromStr for VocabularyState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for VocabularyState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for VocabularyState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// Errors returned by CreateLanguageModel
#[derive(Debug, PartialEq)]
pub enum CreateLanguageModelError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There is already a resource with that name.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl CreateLanguageModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLanguageModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateLanguageModelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateLanguageModelError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateLanguageModelError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateLanguageModelError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLanguageModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLanguageModelError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateLanguageModelError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateLanguageModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateLanguageModelError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLanguageModelError {}
/// Errors returned by CreateMedicalVocabulary
#[derive(Debug, PartialEq)]
pub enum CreateMedicalVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There is already a resource with that name.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl CreateMedicalVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMedicalVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateMedicalVocabularyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateMedicalVocabularyError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateMedicalVocabularyError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateMedicalVocabularyError::LimitExceeded(
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
impl fmt::Display for CreateMedicalVocabularyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMedicalVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateMedicalVocabularyError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateMedicalVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateMedicalVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMedicalVocabularyError {}
/// Errors returned by CreateVocabulary
#[derive(Debug, PartialEq)]
pub enum CreateVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There is already a resource with that name.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl CreateVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVocabularyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateVocabularyError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateVocabularyError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVocabularyError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateVocabularyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVocabularyError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVocabularyError {}
/// Errors returned by CreateVocabularyFilter
#[derive(Debug, PartialEq)]
pub enum CreateVocabularyFilterError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There is already a resource with that name.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl CreateVocabularyFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVocabularyFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVocabularyFilterError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateVocabularyFilterError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateVocabularyFilterError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVocabularyFilterError::LimitExceeded(
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
impl fmt::Display for CreateVocabularyFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVocabularyFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVocabularyFilterError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateVocabularyFilterError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateVocabularyFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVocabularyFilterError {}
/// Errors returned by DeleteLanguageModel
#[derive(Debug, PartialEq)]
pub enum DeleteLanguageModelError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl DeleteLanguageModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLanguageModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteLanguageModelError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteLanguageModelError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteLanguageModelError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLanguageModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLanguageModelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteLanguageModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteLanguageModelError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLanguageModelError {}
/// Errors returned by DeleteMedicalTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum DeleteMedicalTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl DeleteMedicalTranscriptionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteMedicalTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMedicalTranscriptionJobError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(
                        DeleteMedicalTranscriptionJobError::InternalFailure(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteMedicalTranscriptionJobError::LimitExceeded(
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
impl fmt::Display for DeleteMedicalTranscriptionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMedicalTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteMedicalTranscriptionJobError::InternalFailure(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteMedicalTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMedicalTranscriptionJobError {}
/// Errors returned by DeleteMedicalVocabulary
#[derive(Debug, PartialEq)]
pub enum DeleteMedicalVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl DeleteMedicalVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMedicalVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMedicalVocabularyError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteMedicalVocabularyError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteMedicalVocabularyError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMedicalVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMedicalVocabularyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMedicalVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteMedicalVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteMedicalVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteMedicalVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMedicalVocabularyError {}
/// Errors returned by DeleteTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum DeleteTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl DeleteTranscriptionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteTranscriptionJobError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteTranscriptionJobError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteTranscriptionJobError::LimitExceeded(
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
impl fmt::Display for DeleteTranscriptionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteTranscriptionJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTranscriptionJobError {}
/// Errors returned by DeleteVocabulary
#[derive(Debug, PartialEq)]
pub enum DeleteVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl DeleteVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVocabularyError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteVocabularyError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteVocabularyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVocabularyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVocabularyError {}
/// Errors returned by DeleteVocabularyFilter
#[derive(Debug, PartialEq)]
pub enum DeleteVocabularyFilterError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl DeleteVocabularyFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVocabularyFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVocabularyFilterError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteVocabularyFilterError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteVocabularyFilterError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVocabularyFilterError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVocabularyFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVocabularyFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyFilterError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyFilterError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVocabularyFilterError {}
/// Errors returned by DescribeLanguageModel
#[derive(Debug, PartialEq)]
pub enum DescribeLanguageModelError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl DescribeLanguageModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLanguageModelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeLanguageModelError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeLanguageModelError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DescribeLanguageModelError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeLanguageModelError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLanguageModelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLanguageModelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeLanguageModelError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeLanguageModelError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DescribeLanguageModelError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLanguageModelError {}
/// Errors returned by GetMedicalTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum GetMedicalTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl GetMedicalTranscriptionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetMedicalTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetMedicalTranscriptionJobError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetMedicalTranscriptionJobError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetMedicalTranscriptionJobError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetMedicalTranscriptionJobError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMedicalTranscriptionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMedicalTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetMedicalTranscriptionJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetMedicalTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetMedicalTranscriptionJobError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMedicalTranscriptionJobError {}
/// Errors returned by GetMedicalVocabulary
#[derive(Debug, PartialEq)]
pub enum GetMedicalVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl GetMedicalVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMedicalVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetMedicalVocabularyError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetMedicalVocabularyError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetMedicalVocabularyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetMedicalVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetMedicalVocabularyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetMedicalVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetMedicalVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetMedicalVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetMedicalVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetMedicalVocabularyError {}
/// Errors returned by GetTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum GetTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl GetTranscriptionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetTranscriptionJobError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetTranscriptionJobError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetTranscriptionJobError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTranscriptionJobError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTranscriptionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetTranscriptionJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetTranscriptionJobError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTranscriptionJobError {}
/// Errors returned by GetVocabulary
#[derive(Debug, PartialEq)]
pub enum GetVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl GetVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVocabularyError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetVocabularyError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetVocabularyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVocabularyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVocabularyError {}
/// Errors returned by GetVocabularyFilter
#[derive(Debug, PartialEq)]
pub enum GetVocabularyFilterError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl GetVocabularyFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVocabularyFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVocabularyFilterError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetVocabularyFilterError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetVocabularyFilterError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVocabularyFilterError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetVocabularyFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVocabularyFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVocabularyFilterError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetVocabularyFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetVocabularyFilterError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVocabularyFilterError {}
/// Errors returned by ListLanguageModels
#[derive(Debug, PartialEq)]
pub enum ListLanguageModelsError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListLanguageModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLanguageModelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListLanguageModelsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListLanguageModelsError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListLanguageModelsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLanguageModelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLanguageModelsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListLanguageModelsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListLanguageModelsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLanguageModelsError {}
/// Errors returned by ListMedicalTranscriptionJobs
#[derive(Debug, PartialEq)]
pub enum ListMedicalTranscriptionJobsError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListMedicalTranscriptionJobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListMedicalTranscriptionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListMedicalTranscriptionJobsError::BadRequest(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(
                        ListMedicalTranscriptionJobsError::InternalFailure(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListMedicalTranscriptionJobsError::LimitExceeded(
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
impl fmt::Display for ListMedicalTranscriptionJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMedicalTranscriptionJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListMedicalTranscriptionJobsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListMedicalTranscriptionJobsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMedicalTranscriptionJobsError {}
/// Errors returned by ListMedicalVocabularies
#[derive(Debug, PartialEq)]
pub enum ListMedicalVocabulariesError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListMedicalVocabulariesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMedicalVocabulariesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListMedicalVocabulariesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListMedicalVocabulariesError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListMedicalVocabulariesError::LimitExceeded(
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
impl fmt::Display for ListMedicalVocabulariesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMedicalVocabulariesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListMedicalVocabulariesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListMedicalVocabulariesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMedicalVocabulariesError {}
/// Errors returned by ListTranscriptionJobs
#[derive(Debug, PartialEq)]
pub enum ListTranscriptionJobsError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListTranscriptionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTranscriptionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTranscriptionJobsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListTranscriptionJobsError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTranscriptionJobsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTranscriptionJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTranscriptionJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTranscriptionJobsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTranscriptionJobsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTranscriptionJobsError {}
/// Errors returned by ListVocabularies
#[derive(Debug, PartialEq)]
pub enum ListVocabulariesError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListVocabulariesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVocabulariesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVocabulariesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListVocabulariesError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListVocabulariesError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVocabulariesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVocabulariesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVocabulariesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListVocabulariesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVocabulariesError {}
/// Errors returned by ListVocabularyFilters
#[derive(Debug, PartialEq)]
pub enum ListVocabularyFiltersError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListVocabularyFiltersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVocabularyFiltersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVocabularyFiltersError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListVocabularyFiltersError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListVocabularyFiltersError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVocabularyFiltersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVocabularyFiltersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVocabularyFiltersError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListVocabularyFiltersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVocabularyFiltersError {}
/// Errors returned by StartMedicalTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum StartMedicalTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There is already a resource with that name.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl StartMedicalTranscriptionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartMedicalTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartMedicalTranscriptionJobError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartMedicalTranscriptionJobError::Conflict(
                        err.msg,
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(
                        StartMedicalTranscriptionJobError::InternalFailure(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartMedicalTranscriptionJobError::LimitExceeded(
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
impl fmt::Display for StartMedicalTranscriptionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMedicalTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartMedicalTranscriptionJobError::Conflict(ref cause) => write!(f, "{}", cause),
            StartMedicalTranscriptionJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            StartMedicalTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMedicalTranscriptionJobError {}
/// Errors returned by StartTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum StartTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There is already a resource with that name.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl StartTranscriptionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartTranscriptionJobError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartTranscriptionJobError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(StartTranscriptionJobError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartTranscriptionJobError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartTranscriptionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartTranscriptionJobError::Conflict(ref cause) => write!(f, "{}", cause),
            StartTranscriptionJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            StartTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartTranscriptionJobError {}
/// Errors returned by UpdateMedicalVocabulary
#[derive(Debug, PartialEq)]
pub enum UpdateMedicalVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There is already a resource with that name.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl UpdateMedicalVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMedicalVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateMedicalVocabularyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateMedicalVocabularyError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateMedicalVocabularyError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateMedicalVocabularyError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateMedicalVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMedicalVocabularyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMedicalVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateMedicalVocabularyError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateMedicalVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateMedicalVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateMedicalVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMedicalVocabularyError {}
/// Errors returned by UpdateVocabulary
#[derive(Debug, PartialEq)]
pub enum UpdateVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There is already a resource with that name.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl UpdateVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVocabularyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateVocabularyError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateVocabularyError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateVocabularyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVocabularyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVocabularyError {}
/// Errors returned by UpdateVocabularyFilter
#[derive(Debug, PartialEq)]
pub enum UpdateVocabularyFilterError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the entity that you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl UpdateVocabularyFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVocabularyFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVocabularyFilterError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateVocabularyFilterError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateVocabularyFilterError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVocabularyFilterError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateVocabularyFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVocabularyFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyFilterError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyFilterError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVocabularyFilterError {}
/// Trait representing the capabilities of the Amazon Transcribe Service API. Amazon Transcribe Service clients implement this trait.
#[async_trait]
pub trait Transcribe {
    /// <p>Creates a new custom language model. Use Amazon S3 prefixes to provide the location of your input files. The time it takes to create your model depends on the size of your training data.</p>
    async fn create_language_model(
        &self,
        input: CreateLanguageModelRequest,
    ) -> Result<CreateLanguageModelResponse, RusotoError<CreateLanguageModelError>>;

    /// <p>Creates a new custom vocabulary that you can use to change how Amazon Transcribe Medical transcribes your audio file.</p>
    async fn create_medical_vocabulary(
        &self,
        input: CreateMedicalVocabularyRequest,
    ) -> Result<CreateMedicalVocabularyResponse, RusotoError<CreateMedicalVocabularyError>>;

    /// <p>Creates a new custom vocabulary that you can use to change the way Amazon Transcribe handles transcription of an audio file. </p>
    async fn create_vocabulary(
        &self,
        input: CreateVocabularyRequest,
    ) -> Result<CreateVocabularyResponse, RusotoError<CreateVocabularyError>>;

    /// <p>Creates a new vocabulary filter that you can use to filter words, such as profane words, from the output of a transcription job.</p>
    async fn create_vocabulary_filter(
        &self,
        input: CreateVocabularyFilterRequest,
    ) -> Result<CreateVocabularyFilterResponse, RusotoError<CreateVocabularyFilterError>>;

    /// <p>Deletes a custom language model using its name.</p>
    async fn delete_language_model(
        &self,
        input: DeleteLanguageModelRequest,
    ) -> Result<(), RusotoError<DeleteLanguageModelError>>;

    /// <p>Deletes a transcription job generated by Amazon Transcribe Medical and any related information.</p>
    async fn delete_medical_transcription_job(
        &self,
        input: DeleteMedicalTranscriptionJobRequest,
    ) -> Result<(), RusotoError<DeleteMedicalTranscriptionJobError>>;

    /// <p>Deletes a vocabulary from Amazon Transcribe Medical.</p>
    async fn delete_medical_vocabulary(
        &self,
        input: DeleteMedicalVocabularyRequest,
    ) -> Result<(), RusotoError<DeleteMedicalVocabularyError>>;

    /// <p>Deletes a previously submitted transcription job along with any other generated results such as the transcription, models, and so on.</p>
    async fn delete_transcription_job(
        &self,
        input: DeleteTranscriptionJobRequest,
    ) -> Result<(), RusotoError<DeleteTranscriptionJobError>>;

    /// <p>Deletes a vocabulary from Amazon Transcribe. </p>
    async fn delete_vocabulary(
        &self,
        input: DeleteVocabularyRequest,
    ) -> Result<(), RusotoError<DeleteVocabularyError>>;

    /// <p>Removes a vocabulary filter.</p>
    async fn delete_vocabulary_filter(
        &self,
        input: DeleteVocabularyFilterRequest,
    ) -> Result<(), RusotoError<DeleteVocabularyFilterError>>;

    /// <p>Gets information about a single custom language model. Use this information to see details about the language model in your AWS account. You can also see whether the base language model used to create your custom language model has been updated. If Amazon Transcribe has updated the base model, you can create a new custom language model using the updated base model. If the language model wasn't created, you can use this operation to understand why Amazon Transcribe couldn't create it. </p>
    async fn describe_language_model(
        &self,
        input: DescribeLanguageModelRequest,
    ) -> Result<DescribeLanguageModelResponse, RusotoError<DescribeLanguageModelError>>;

    /// <p>Returns information about a transcription job from Amazon Transcribe Medical. To see the status of the job, check the <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>, the job is finished. You find the results of the completed job in the <code>TranscriptFileUri</code> field.</p>
    async fn get_medical_transcription_job(
        &self,
        input: GetMedicalTranscriptionJobRequest,
    ) -> Result<GetMedicalTranscriptionJobResponse, RusotoError<GetMedicalTranscriptionJobError>>;

    /// <p>Retrieves information about a medical vocabulary.</p>
    async fn get_medical_vocabulary(
        &self,
        input: GetMedicalVocabularyRequest,
    ) -> Result<GetMedicalVocabularyResponse, RusotoError<GetMedicalVocabularyError>>;

    /// <p>Returns information about a transcription job. To see the status of the job, check the <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>, the job is finished and you can find the results at the location specified in the <code>TranscriptFileUri</code> field. If you enable content redaction, the redacted transcript appears in <code>RedactedTranscriptFileUri</code>.</p>
    async fn get_transcription_job(
        &self,
        input: GetTranscriptionJobRequest,
    ) -> Result<GetTranscriptionJobResponse, RusotoError<GetTranscriptionJobError>>;

    /// <p>Gets information about a vocabulary. </p>
    async fn get_vocabulary(
        &self,
        input: GetVocabularyRequest,
    ) -> Result<GetVocabularyResponse, RusotoError<GetVocabularyError>>;

    /// <p>Returns information about a vocabulary filter.</p>
    async fn get_vocabulary_filter(
        &self,
        input: GetVocabularyFilterRequest,
    ) -> Result<GetVocabularyFilterResponse, RusotoError<GetVocabularyFilterError>>;

    /// <p>Provides more information about the custom language models you've created. You can use the information in this list to find a specific custom language model. You can then use the operation to get more information about it.</p>
    async fn list_language_models(
        &self,
        input: ListLanguageModelsRequest,
    ) -> Result<ListLanguageModelsResponse, RusotoError<ListLanguageModelsError>>;

    /// <p>Lists medical transcription jobs with a specified status or substring that matches their names.</p>
    async fn list_medical_transcription_jobs(
        &self,
        input: ListMedicalTranscriptionJobsRequest,
    ) -> Result<ListMedicalTranscriptionJobsResponse, RusotoError<ListMedicalTranscriptionJobsError>>;

    /// <p>Returns a list of vocabularies that match the specified criteria. If you don't enter a value in any of the request parameters, returns the entire list of vocabularies.</p>
    async fn list_medical_vocabularies(
        &self,
        input: ListMedicalVocabulariesRequest,
    ) -> Result<ListMedicalVocabulariesResponse, RusotoError<ListMedicalVocabulariesError>>;

    /// <p>Lists transcription jobs with the specified status.</p>
    async fn list_transcription_jobs(
        &self,
        input: ListTranscriptionJobsRequest,
    ) -> Result<ListTranscriptionJobsResponse, RusotoError<ListTranscriptionJobsError>>;

    /// <p>Returns a list of vocabularies that match the specified criteria. If no criteria are specified, returns the entire list of vocabularies.</p>
    async fn list_vocabularies(
        &self,
        input: ListVocabulariesRequest,
    ) -> Result<ListVocabulariesResponse, RusotoError<ListVocabulariesError>>;

    /// <p>Gets information about vocabulary filters.</p>
    async fn list_vocabulary_filters(
        &self,
        input: ListVocabularyFiltersRequest,
    ) -> Result<ListVocabularyFiltersResponse, RusotoError<ListVocabularyFiltersError>>;

    /// <p>Starts a batch job to transcribe medical speech to text.</p>
    async fn start_medical_transcription_job(
        &self,
        input: StartMedicalTranscriptionJobRequest,
    ) -> Result<StartMedicalTranscriptionJobResponse, RusotoError<StartMedicalTranscriptionJobError>>;

    /// <p>Starts an asynchronous job to transcribe speech to text. </p>
    async fn start_transcription_job(
        &self,
        input: StartTranscriptionJobRequest,
    ) -> Result<StartTranscriptionJobResponse, RusotoError<StartTranscriptionJobError>>;

    /// <p>Updates a vocabulary with new values that you provide in a different text file from the one you used to create the vocabulary. The <code>UpdateMedicalVocabulary</code> operation overwrites all of the existing information with the values that you provide in the request.</p>
    async fn update_medical_vocabulary(
        &self,
        input: UpdateMedicalVocabularyRequest,
    ) -> Result<UpdateMedicalVocabularyResponse, RusotoError<UpdateMedicalVocabularyError>>;

    /// <p>Updates an existing vocabulary with new values. The <code>UpdateVocabulary</code> operation overwrites all of the existing information with the values that you provide in the request. </p>
    async fn update_vocabulary(
        &self,
        input: UpdateVocabularyRequest,
    ) -> Result<UpdateVocabularyResponse, RusotoError<UpdateVocabularyError>>;

    /// <p>Updates a vocabulary filter with a new list of filtered words.</p>
    async fn update_vocabulary_filter(
        &self,
        input: UpdateVocabularyFilterRequest,
    ) -> Result<UpdateVocabularyFilterResponse, RusotoError<UpdateVocabularyFilterError>>;
}
/// A client for the Amazon Transcribe Service API.
#[derive(Clone)]
pub struct TranscribeClient {
    client: Client,
    region: region::Region,
}

impl TranscribeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> TranscribeClient {
        TranscribeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TranscribeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        TranscribeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> TranscribeClient {
        TranscribeClient { client, region }
    }
}

#[async_trait]
impl Transcribe for TranscribeClient {
    /// <p>Creates a new custom language model. Use Amazon S3 prefixes to provide the location of your input files. The time it takes to create your model depends on the size of your training data.</p>
    async fn create_language_model(
        &self,
        input: CreateLanguageModelRequest,
    ) -> Result<CreateLanguageModelResponse, RusotoError<CreateLanguageModelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.CreateLanguageModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLanguageModelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLanguageModelResponse, _>()
    }

    /// <p>Creates a new custom vocabulary that you can use to change how Amazon Transcribe Medical transcribes your audio file.</p>
    async fn create_medical_vocabulary(
        &self,
        input: CreateMedicalVocabularyRequest,
    ) -> Result<CreateMedicalVocabularyResponse, RusotoError<CreateMedicalVocabularyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.CreateMedicalVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateMedicalVocabularyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateMedicalVocabularyResponse, _>()
    }

    /// <p>Creates a new custom vocabulary that you can use to change the way Amazon Transcribe handles transcription of an audio file. </p>
    async fn create_vocabulary(
        &self,
        input: CreateVocabularyRequest,
    ) -> Result<CreateVocabularyResponse, RusotoError<CreateVocabularyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.CreateVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateVocabularyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateVocabularyResponse, _>()
    }

    /// <p>Creates a new vocabulary filter that you can use to filter words, such as profane words, from the output of a transcription job.</p>
    async fn create_vocabulary_filter(
        &self,
        input: CreateVocabularyFilterRequest,
    ) -> Result<CreateVocabularyFilterResponse, RusotoError<CreateVocabularyFilterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.CreateVocabularyFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateVocabularyFilterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateVocabularyFilterResponse, _>()
    }

    /// <p>Deletes a custom language model using its name.</p>
    async fn delete_language_model(
        &self,
        input: DeleteLanguageModelRequest,
    ) -> Result<(), RusotoError<DeleteLanguageModelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.DeleteLanguageModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLanguageModelError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a transcription job generated by Amazon Transcribe Medical and any related information.</p>
    async fn delete_medical_transcription_job(
        &self,
        input: DeleteMedicalTranscriptionJobRequest,
    ) -> Result<(), RusotoError<DeleteMedicalTranscriptionJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.DeleteMedicalTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteMedicalTranscriptionJobError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a vocabulary from Amazon Transcribe Medical.</p>
    async fn delete_medical_vocabulary(
        &self,
        input: DeleteMedicalVocabularyRequest,
    ) -> Result<(), RusotoError<DeleteMedicalVocabularyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.DeleteMedicalVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteMedicalVocabularyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a previously submitted transcription job along with any other generated results such as the transcription, models, and so on.</p>
    async fn delete_transcription_job(
        &self,
        input: DeleteTranscriptionJobRequest,
    ) -> Result<(), RusotoError<DeleteTranscriptionJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.DeleteTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTranscriptionJobError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a vocabulary from Amazon Transcribe. </p>
    async fn delete_vocabulary(
        &self,
        input: DeleteVocabularyRequest,
    ) -> Result<(), RusotoError<DeleteVocabularyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.DeleteVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteVocabularyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Removes a vocabulary filter.</p>
    async fn delete_vocabulary_filter(
        &self,
        input: DeleteVocabularyFilterRequest,
    ) -> Result<(), RusotoError<DeleteVocabularyFilterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.DeleteVocabularyFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteVocabularyFilterError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Gets information about a single custom language model. Use this information to see details about the language model in your AWS account. You can also see whether the base language model used to create your custom language model has been updated. If Amazon Transcribe has updated the base model, you can create a new custom language model using the updated base model. If the language model wasn't created, you can use this operation to understand why Amazon Transcribe couldn't create it. </p>
    async fn describe_language_model(
        &self,
        input: DescribeLanguageModelRequest,
    ) -> Result<DescribeLanguageModelResponse, RusotoError<DescribeLanguageModelError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.DescribeLanguageModel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLanguageModelError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeLanguageModelResponse, _>()
    }

    /// <p>Returns information about a transcription job from Amazon Transcribe Medical. To see the status of the job, check the <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>, the job is finished. You find the results of the completed job in the <code>TranscriptFileUri</code> field.</p>
    async fn get_medical_transcription_job(
        &self,
        input: GetMedicalTranscriptionJobRequest,
    ) -> Result<GetMedicalTranscriptionJobResponse, RusotoError<GetMedicalTranscriptionJobError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.GetMedicalTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetMedicalTranscriptionJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetMedicalTranscriptionJobResponse, _>()
    }

    /// <p>Retrieves information about a medical vocabulary.</p>
    async fn get_medical_vocabulary(
        &self,
        input: GetMedicalVocabularyRequest,
    ) -> Result<GetMedicalVocabularyResponse, RusotoError<GetMedicalVocabularyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.GetMedicalVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetMedicalVocabularyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetMedicalVocabularyResponse, _>()
    }

    /// <p>Returns information about a transcription job. To see the status of the job, check the <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>, the job is finished and you can find the results at the location specified in the <code>TranscriptFileUri</code> field. If you enable content redaction, the redacted transcript appears in <code>RedactedTranscriptFileUri</code>.</p>
    async fn get_transcription_job(
        &self,
        input: GetTranscriptionJobRequest,
    ) -> Result<GetTranscriptionJobResponse, RusotoError<GetTranscriptionJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.GetTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetTranscriptionJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetTranscriptionJobResponse, _>()
    }

    /// <p>Gets information about a vocabulary. </p>
    async fn get_vocabulary(
        &self,
        input: GetVocabularyRequest,
    ) -> Result<GetVocabularyResponse, RusotoError<GetVocabularyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.GetVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetVocabularyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetVocabularyResponse, _>()
    }

    /// <p>Returns information about a vocabulary filter.</p>
    async fn get_vocabulary_filter(
        &self,
        input: GetVocabularyFilterRequest,
    ) -> Result<GetVocabularyFilterResponse, RusotoError<GetVocabularyFilterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.GetVocabularyFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetVocabularyFilterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetVocabularyFilterResponse, _>()
    }

    /// <p>Provides more information about the custom language models you've created. You can use the information in this list to find a specific custom language model. You can then use the operation to get more information about it.</p>
    async fn list_language_models(
        &self,
        input: ListLanguageModelsRequest,
    ) -> Result<ListLanguageModelsResponse, RusotoError<ListLanguageModelsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.ListLanguageModels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLanguageModelsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLanguageModelsResponse, _>()
    }

    /// <p>Lists medical transcription jobs with a specified status or substring that matches their names.</p>
    async fn list_medical_transcription_jobs(
        &self,
        input: ListMedicalTranscriptionJobsRequest,
    ) -> Result<ListMedicalTranscriptionJobsResponse, RusotoError<ListMedicalTranscriptionJobsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.ListMedicalTranscriptionJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListMedicalTranscriptionJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListMedicalTranscriptionJobsResponse, _>()
    }

    /// <p>Returns a list of vocabularies that match the specified criteria. If you don't enter a value in any of the request parameters, returns the entire list of vocabularies.</p>
    async fn list_medical_vocabularies(
        &self,
        input: ListMedicalVocabulariesRequest,
    ) -> Result<ListMedicalVocabulariesResponse, RusotoError<ListMedicalVocabulariesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.ListMedicalVocabularies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListMedicalVocabulariesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListMedicalVocabulariesResponse, _>()
    }

    /// <p>Lists transcription jobs with the specified status.</p>
    async fn list_transcription_jobs(
        &self,
        input: ListTranscriptionJobsRequest,
    ) -> Result<ListTranscriptionJobsResponse, RusotoError<ListTranscriptionJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.ListTranscriptionJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTranscriptionJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListTranscriptionJobsResponse, _>()
    }

    /// <p>Returns a list of vocabularies that match the specified criteria. If no criteria are specified, returns the entire list of vocabularies.</p>
    async fn list_vocabularies(
        &self,
        input: ListVocabulariesRequest,
    ) -> Result<ListVocabulariesResponse, RusotoError<ListVocabulariesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.ListVocabularies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListVocabulariesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListVocabulariesResponse, _>()
    }

    /// <p>Gets information about vocabulary filters.</p>
    async fn list_vocabulary_filters(
        &self,
        input: ListVocabularyFiltersRequest,
    ) -> Result<ListVocabularyFiltersResponse, RusotoError<ListVocabularyFiltersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.ListVocabularyFilters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListVocabularyFiltersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListVocabularyFiltersResponse, _>()
    }

    /// <p>Starts a batch job to transcribe medical speech to text.</p>
    async fn start_medical_transcription_job(
        &self,
        input: StartMedicalTranscriptionJobRequest,
    ) -> Result<StartMedicalTranscriptionJobResponse, RusotoError<StartMedicalTranscriptionJobError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.StartMedicalTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartMedicalTranscriptionJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartMedicalTranscriptionJobResponse, _>()
    }

    /// <p>Starts an asynchronous job to transcribe speech to text. </p>
    async fn start_transcription_job(
        &self,
        input: StartTranscriptionJobRequest,
    ) -> Result<StartTranscriptionJobResponse, RusotoError<StartTranscriptionJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.StartTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartTranscriptionJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartTranscriptionJobResponse, _>()
    }

    /// <p>Updates a vocabulary with new values that you provide in a different text file from the one you used to create the vocabulary. The <code>UpdateMedicalVocabulary</code> operation overwrites all of the existing information with the values that you provide in the request.</p>
    async fn update_medical_vocabulary(
        &self,
        input: UpdateMedicalVocabularyRequest,
    ) -> Result<UpdateMedicalVocabularyResponse, RusotoError<UpdateMedicalVocabularyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.UpdateMedicalVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateMedicalVocabularyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateMedicalVocabularyResponse, _>()
    }

    /// <p>Updates an existing vocabulary with new values. The <code>UpdateVocabulary</code> operation overwrites all of the existing information with the values that you provide in the request. </p>
    async fn update_vocabulary(
        &self,
        input: UpdateVocabularyRequest,
    ) -> Result<UpdateVocabularyResponse, RusotoError<UpdateVocabularyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.UpdateVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateVocabularyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateVocabularyResponse, _>()
    }

    /// <p>Updates a vocabulary filter with a new list of filtered words.</p>
    async fn update_vocabulary_filter(
        &self,
        input: UpdateVocabularyFilterRequest,
    ) -> Result<UpdateVocabularyFilterResponse, RusotoError<UpdateVocabularyFilterError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Transcribe.UpdateVocabularyFilter");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateVocabularyFilterError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateVocabularyFilterResponse, _>()
    }
}
