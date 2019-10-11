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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use futures::{FutureExt, TryFutureExt};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>The custom terminology applied to the input text by Amazon Translate for the translated text response. This is optional in the response and will only be present if you specified terminology input in the request. Currently, only one terminology can be applied per TranslateText request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AppliedTerminology {
    /// <p>The name of the custom terminology applied to the input text by Amazon Translate for the translated text response.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The specific terms of the custom terminology applied to the input text by Amazon Translate for the translated text response. A maximum of 250 terms will be returned, and the specific terms applied will be the first 250 terms in the source text. </p>
    #[serde(rename = "Terms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<Term>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTerminologyRequest {
    /// <p>The name of the custom terminology being deleted. </p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>The encryption key used to encrypt the custom terminologies used by Amazon Translate.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionKey {
    /// <p>The Amazon Resource Name (ARN) of the encryption key being used to encrypt the custom terminology.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The type of encryption key used by Amazon Translate to encrypt custom terminologies.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTerminologyRequest {
    /// <p>The name of the custom terminology being retrieved.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The data format of the custom terminology being retrieved, either CSV or TMX.</p>
    #[serde(rename = "TerminologyDataFormat")]
    pub terminology_data_format: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTerminologyResponse {
    /// <p>The data location of the custom terminology being retrieved. The custom terminology file is returned in a presigned url that has a 30 minute expiration.</p>
    #[serde(rename = "TerminologyDataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_data_location: Option<TerminologyDataLocation>,
    /// <p>The properties of the custom terminology being retrieved.</p>
    #[serde(rename = "TerminologyProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_properties: Option<TerminologyProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportTerminologyRequest {
    /// <p>The description of the custom terminology being imported.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The encryption key for the custom terminology being imported.</p>
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The merge strategy of the custom terminology being imported. Currently, only the OVERWRITE merge strategy is supported. In this case, the imported terminology will overwrite an existing terminology of the same name.</p>
    #[serde(rename = "MergeStrategy")]
    pub merge_strategy: String,
    /// <p>The name of the custom terminology being imported.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The terminology data for the custom terminology being imported.</p>
    #[serde(rename = "TerminologyData")]
    pub terminology_data: TerminologyData,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportTerminologyResponse {
    /// <p>The properties of the custom terminology being imported.</p>
    #[serde(rename = "TerminologyProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_properties: Option<TerminologyProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTerminologiesRequest {
    /// <p>The maximum number of custom terminologies returned per list request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the request to ListTerminologies was truncated, include the NextToken to fetch the next group of custom terminologies. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTerminologiesResponse {
    /// <p> If the response to the ListTerminologies was truncated, the NextToken fetches the next group of custom terminologies. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The properties list of the custom terminologies returned on the list request.</p>
    #[serde(rename = "TerminologyPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_properties_list: Option<Vec<TerminologyProperties>>,
}

/// <p>The term being translated by the custom terminology.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Term {
    /// <p>The source text of the term being translated by the custom terminology.</p>
    #[serde(rename = "SourceText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_text: Option<String>,
    /// <p>The target text of the term being translated by the custom terminology.</p>
    #[serde(rename = "TargetText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_text: Option<String>,
}

/// <p>The data associated with the custom terminology.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminologyData {
    /// <p>The file containing the custom terminology data.</p>
    #[serde(rename = "File")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub file: bytes::Bytes,
    /// <p>The data format of the custom terminology. Either CSV or TMX.</p>
    #[serde(rename = "Format")]
    pub format: String,
}

/// <p>The location of the custom terminology data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TerminologyDataLocation {
    /// <p>The location of the custom terminology data.</p>
    #[serde(rename = "Location")]
    pub location: String,
    /// <p>The repository type for the custom terminology data.</p>
    #[serde(rename = "RepositoryType")]
    pub repository_type: String,
}

/// <p>The properties of the custom terminology.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TerminologyProperties {
    /// <p> The Amazon Resource Name (ARN) of the custom terminology. </p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time at which the custom terminology was created, based on the timestamp.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the custom terminology properties.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The encryption key for the custom terminology.</p>
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The time at which the custom terminology was last update, based on the timestamp.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the custom terminology.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The size of the file used when importing a custom terminology.</p>
    #[serde(rename = "SizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// <p>The language code for the source text of the translation request for which the custom terminology is being used.</p>
    #[serde(rename = "SourceLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language_code: Option<String>,
    /// <p>The language codes for the target languages available with the custom terminology file. All possible target languages are returned in array.</p>
    #[serde(rename = "TargetLanguageCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language_codes: Option<Vec<String>>,
    /// <p>The number of terms included in the custom terminology.</p>
    #[serde(rename = "TermCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TranslateTextRequest {
    /// <p>The language code for the language of the source text. The language must be a language supported by Amazon Translate. </p> <p>To have Amazon Translate determine the source language of your text, you can specify <code>auto</code> in the <code>SourceLanguageCode</code> field. If you specify <code>auto</code>, Amazon Translate will call Amazon Comprehend to determine the source language.</p>
    #[serde(rename = "SourceLanguageCode")]
    pub source_language_code: String,
    /// <p>The language code requested for the language of the target text. The language must be a language supported by Amazon Translate.</p>
    #[serde(rename = "TargetLanguageCode")]
    pub target_language_code: String,
    /// <p>The TerminologyNames list that is taken as input to the TranslateText request. This has a minimum length of 0 and a maximum length of 1.</p>
    #[serde(rename = "TerminologyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_names: Option<Vec<String>>,
    /// <p>The text to translate. The text string can be a maximum of 5,000 bytes long. Depending on your character set, this may be fewer than 5,000 characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TranslateTextResponse {
    /// <p>The names of the custom terminologies applied to the input text by Amazon Translate for the translated text response.</p>
    #[serde(rename = "AppliedTerminologies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_terminologies: Option<Vec<AppliedTerminology>>,
    /// <p>The language code for the language of the source text. </p>
    #[serde(rename = "SourceLanguageCode")]
    pub source_language_code: String,
    /// <p>The language code for the language of the target text. </p>
    #[serde(rename = "TargetLanguageCode")]
    pub target_language_code: String,
    /// <p>The the translated text. The maximum length of this text is 5kb.</p>
    #[serde(rename = "TranslatedText")]
    pub translated_text: String,
}

/// Errors returned by DeleteTerminology
#[derive(Debug, PartialEq)]
pub enum DeleteTerminologyError {
    /// <p> An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request. .</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl DeleteTerminologyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTerminologyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteTerminologyError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTerminologyError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteTerminologyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTerminologyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTerminologyError {
    fn description(&self) -> &str {
        match *self {
            DeleteTerminologyError::InternalServer(ref cause) => cause,
            DeleteTerminologyError::ResourceNotFound(ref cause) => cause,
            DeleteTerminologyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTerminology
#[derive(Debug, PartialEq)]
pub enum GetTerminologyError {
    /// <p> An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request. .</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl GetTerminologyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTerminologyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetTerminologyError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetTerminologyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetTerminologyError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetTerminologyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTerminologyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTerminologyError {
    fn description(&self) -> &str {
        match *self {
            GetTerminologyError::InternalServer(ref cause) => cause,
            GetTerminologyError::InvalidParameterValue(ref cause) => cause,
            GetTerminologyError::ResourceNotFound(ref cause) => cause,
            GetTerminologyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportTerminology
#[derive(Debug, PartialEq)]
pub enum ImportTerminologyError {
    /// <p> An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p>The specified limit has been exceeded. Review your request and retry it with a quantity below the stated limit.</p>
    LimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl ImportTerminologyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportTerminologyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ImportTerminologyError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ImportTerminologyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ImportTerminologyError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ImportTerminologyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ImportTerminologyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportTerminologyError {
    fn description(&self) -> &str {
        match *self {
            ImportTerminologyError::InternalServer(ref cause) => cause,
            ImportTerminologyError::InvalidParameterValue(ref cause) => cause,
            ImportTerminologyError::LimitExceeded(ref cause) => cause,
            ImportTerminologyError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTerminologies
#[derive(Debug, PartialEq)]
pub enum ListTerminologiesError {
    /// <p> An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl ListTerminologiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTerminologiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTerminologiesError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTerminologiesError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTerminologiesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTerminologiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTerminologiesError {
    fn description(&self) -> &str {
        match *self {
            ListTerminologiesError::InternalServer(ref cause) => cause,
            ListTerminologiesError::InvalidParameterValue(ref cause) => cause,
            ListTerminologiesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by TranslateText
#[derive(Debug, PartialEq)]
pub enum TranslateTextError {
    /// <p>The confidence that Amazon Comprehend accurately detected the source language is low. If a low confidence level is acceptable for your application, you can use the language in the exception to call Amazon Translate again. For more information, see the <a href="https://docs.aws.amazon.com/comprehend/latest/dg/API_DetectDominantLanguage.html">DetectDominantLanguage</a> operation in the <i>Amazon Comprehend Developer Guide</i>. </p>
    DetectedLanguageLowConfidence(String),
    /// <p> An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request. </p>
    InvalidRequest(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request. .</p>
    ResourceNotFound(String),
    /// <p>The Amazon Translate service is temporarily unavailable. Please wait a bit and then retry your request.</p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
    /// <p>Amazon Translate does not support translation from the language of the source text into the requested target language. For more information, see <a>how-to-error-msg</a>. </p>
    UnsupportedLanguagePair(String),
}

impl TranslateTextError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TranslateTextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DetectedLanguageLowConfidenceException" => {
                    return RusotoError::Service(TranslateTextError::DetectedLanguageLowConfidence(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(TranslateTextError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TranslateTextError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TranslateTextError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TranslateTextError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(TranslateTextError::TextSizeLimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TranslateTextError::TooManyRequests(err.msg))
                }
                "UnsupportedLanguagePairException" => {
                    return RusotoError::Service(TranslateTextError::UnsupportedLanguagePair(
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
impl fmt::Display for TranslateTextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TranslateTextError {
    fn description(&self) -> &str {
        match *self {
            TranslateTextError::DetectedLanguageLowConfidence(ref cause) => cause,
            TranslateTextError::InternalServer(ref cause) => cause,
            TranslateTextError::InvalidRequest(ref cause) => cause,
            TranslateTextError::ResourceNotFound(ref cause) => cause,
            TranslateTextError::ServiceUnavailable(ref cause) => cause,
            TranslateTextError::TextSizeLimitExceeded(ref cause) => cause,
            TranslateTextError::TooManyRequests(ref cause) => cause,
            TranslateTextError::UnsupportedLanguagePair(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Translate API. Amazon Translate clients implement this trait.
pub trait Translate {
    /// <p>A synchronous action that deletes a custom terminology.</p>
    fn delete_terminology(
        &self,
        input: DeleteTerminologyRequest,
    ) -> RusotoFuture<(), DeleteTerminologyError>;

    /// <p>Retrieves a custom terminology.</p>
    fn get_terminology(
        &self,
        input: GetTerminologyRequest,
    ) -> RusotoFuture<GetTerminologyResponse, GetTerminologyError>;

    /// <p>Creates or updates a custom terminology, depending on whether or not one already exists for the given terminology name. Importing a terminology with the same name as an existing one will merge the terminologies based on the chosen merge strategy. Currently, the only supported merge strategy is OVERWRITE, and so the imported terminology will overwrite an existing terminology of the same name.</p> <p>If you import a terminology that overwrites an existing one, the new terminology take up to 10 minutes to fully propagate and be available for use in a translation due to cache policies with the DataPlane service that performs the translations.</p>
    fn import_terminology(
        &self,
        input: ImportTerminologyRequest,
    ) -> RusotoFuture<ImportTerminologyResponse, ImportTerminologyError>;

    /// <p>Provides a list of custom terminologies associated with your account.</p>
    fn list_terminologies(
        &self,
        input: ListTerminologiesRequest,
    ) -> RusotoFuture<ListTerminologiesResponse, ListTerminologiesError>;

    /// <p>Translates input text from the source language to the target language. It is not necessary to use English (en) as either the source or the target language but not all language combinations are supported by Amazon Translate. For more information, see <a href="http://docs.aws.amazon.com/translate/latest/dg/pairs.html">Supported Language Pairs</a>.</p> <ul> <li> <p>Arabic (ar)</p> </li> <li> <p>Chinese (Simplified) (zh)</p> </li> <li> <p>Chinese (Traditional) (zh-TW)</p> </li> <li> <p>Czech (cs)</p> </li> <li> <p>Danish (da)</p> </li> <li> <p>Dutch (nl)</p> </li> <li> <p>English (en)</p> </li> <li> <p>Finnish (fi)</p> </li> <li> <p>French (fr)</p> </li> <li> <p>German (de)</p> </li> <li> <p>Hebrew (he)</p> </li> <li> <p>Indonesian (id)</p> </li> <li> <p>Italian (it)</p> </li> <li> <p>Japanese (ja)</p> </li> <li> <p>Korean (ko)</p> </li> <li> <p>Polish (pl)</p> </li> <li> <p>Portuguese (pt)</p> </li> <li> <p>Russian (ru)</p> </li> <li> <p>Spanish (es)</p> </li> <li> <p>Swedish (sv)</p> </li> <li> <p>Turkish (tr)</p> </li> </ul> <p>To have Amazon Translate determine the source language of your text, you can specify <code>auto</code> in the <code>SourceLanguageCode</code> field. If you specify <code>auto</code>, Amazon Translate will call Amazon Comprehend to determine the source language.</p>
    fn translate_text(
        &self,
        input: TranslateTextRequest,
    ) -> RusotoFuture<TranslateTextResponse, TranslateTextError>;
}
/// A client for the Amazon Translate API.
#[derive(Clone)]
pub struct TranslateClient {
    client: Client,
    region: region::Region,
}

impl TranslateClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> TranslateClient {
        TranslateClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TranslateClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        TranslateClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl Translate for TranslateClient {
    /// <p>A synchronous action that deletes a custom terminology.</p>
    fn delete_terminology(
        &self,
        input: DeleteTerminologyRequest,
    ) -> RusotoFuture<(), DeleteTerminologyError> {
        let mut request = SignedRequest::new("POST", "translate", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.DeleteTerminology",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DeleteTerminologyError>
                            })
                            .and_then(|response| {
                                Err(DeleteTerminologyError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Retrieves a custom terminology.</p>
    fn get_terminology(
        &self,
        input: GetTerminologyRequest,
    ) -> RusotoFuture<GetTerminologyResponse, GetTerminologyError> {
        let mut request = SignedRequest::new("POST", "translate", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.GetTerminology",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| GetTerminologyError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<GetTerminologyError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<GetTerminologyResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<GetTerminologyError>
                            })
                            .and_then(|response| Err(GetTerminologyError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Creates or updates a custom terminology, depending on whether or not one already exists for the given terminology name. Importing a terminology with the same name as an existing one will merge the terminologies based on the chosen merge strategy. Currently, the only supported merge strategy is OVERWRITE, and so the imported terminology will overwrite an existing terminology of the same name.</p> <p>If you import a terminology that overwrites an existing one, the new terminology take up to 10 minutes to fully propagate and be available for use in a translation due to cache policies with the DataPlane service that performs the translations.</p>
    fn import_terminology(
        &self,
        input: ImportTerminologyRequest,
    ) -> RusotoFuture<ImportTerminologyResponse, ImportTerminologyError> {
        let mut request = SignedRequest::new("POST", "translate", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.ImportTerminology",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ImportTerminologyError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ImportTerminologyError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ImportTerminologyResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ImportTerminologyError>
                            })
                            .and_then(|response| {
                                Err(ImportTerminologyError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Provides a list of custom terminologies associated with your account.</p>
    fn list_terminologies(
        &self,
        input: ListTerminologiesRequest,
    ) -> RusotoFuture<ListTerminologiesResponse, ListTerminologiesError> {
        let mut request = SignedRequest::new("POST", "translate", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.ListTerminologies",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListTerminologiesError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListTerminologiesError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListTerminologiesResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListTerminologiesError>
                            })
                            .and_then(|response| {
                                Err(ListTerminologiesError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Translates input text from the source language to the target language. It is not necessary to use English (en) as either the source or the target language but not all language combinations are supported by Amazon Translate. For more information, see <a href="http://docs.aws.amazon.com/translate/latest/dg/pairs.html">Supported Language Pairs</a>.</p> <ul> <li> <p>Arabic (ar)</p> </li> <li> <p>Chinese (Simplified) (zh)</p> </li> <li> <p>Chinese (Traditional) (zh-TW)</p> </li> <li> <p>Czech (cs)</p> </li> <li> <p>Danish (da)</p> </li> <li> <p>Dutch (nl)</p> </li> <li> <p>English (en)</p> </li> <li> <p>Finnish (fi)</p> </li> <li> <p>French (fr)</p> </li> <li> <p>German (de)</p> </li> <li> <p>Hebrew (he)</p> </li> <li> <p>Indonesian (id)</p> </li> <li> <p>Italian (it)</p> </li> <li> <p>Japanese (ja)</p> </li> <li> <p>Korean (ko)</p> </li> <li> <p>Polish (pl)</p> </li> <li> <p>Portuguese (pt)</p> </li> <li> <p>Russian (ru)</p> </li> <li> <p>Spanish (es)</p> </li> <li> <p>Swedish (sv)</p> </li> <li> <p>Turkish (tr)</p> </li> </ul> <p>To have Amazon Translate determine the source language of your text, you can specify <code>auto</code> in the <code>SourceLanguageCode</code> field. If you specify <code>auto</code>, Amazon Translate will call Amazon Comprehend to determine the source language.</p>
    fn translate_text(
        &self,
        input: TranslateTextRequest,
    ) -> RusotoFuture<TranslateTextResponse, TranslateTextError> {
        let mut request = SignedRequest::new("POST", "translate", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.TranslateText",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| TranslateTextError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<TranslateTextError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<TranslateTextResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<TranslateTextError>
                            })
                            .and_then(|response| Err(TranslateTextError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }
}
