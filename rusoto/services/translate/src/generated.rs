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
pub struct TranslateTextRequest {
    /// <p>One of the supported language codes for the source text. If the <code>TargetLanguageCode</code> is not "en", the <code>SourceLanguageCode</code> must be "en".</p> <p>To have Amazon Translate determine the source language of your text, you can specify <code>auto</code> in the <code>SourceLanguageCode</code> field. If you specify <code>auto</code>, Amazon Translate will call Amazon Comprehend to determine the source language.</p>
    #[serde(rename = "SourceLanguageCode")]
    pub source_language_code: String,
    /// <p>One of the supported language codes for the target text. If the <code>SourceLanguageCode</code> is not "en", the <code>TargetLanguageCode</code> must be "en".</p>
    #[serde(rename = "TargetLanguageCode")]
    pub target_language_code: String,
    /// <p>The text to translate.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TranslateTextResponse {
    /// <p>The language code for the language of the input text. </p>
    #[serde(rename = "SourceLanguageCode")]
    pub source_language_code: String,
    /// <p>The language code for the language of the translated text. </p>
    #[serde(rename = "TargetLanguageCode")]
    pub target_language_code: String,
    /// <p>The text translated into the target language.</p>
    #[serde(rename = "TranslatedText")]
    pub translated_text: String,
}

/// Errors returned by TranslateText
#[derive(Debug, PartialEq)]
pub enum TranslateTextError {
    /// <p>The confidence that Amazon Comprehend accurately detected the source language is low. If a low confidence level is acceptable for your application, you can use the language in the exception to call Amazon Translate again. For more information, see the <a href="https://docs.aws.amazon.com/comprehend/latest/dg/API_DetectDominantLanguage.html">DetectDominantLanguage</a> operation in the <i>Amazon Comprehend Developer Guide</i>.</p>
    DetectedLanguageLowConfidence(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>Amazon Translate is unavailable. Retry your request later.</p>
    ServiceUnavailable(String),
    /// <p>The size of the input text exceeds the length constraint for the <code>Text</code> field. Try again with a shorter text. </p>
    TextSizeLimitExceeded(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
    /// <p>Amazon Translate cannot translate input text in the source language into this target language. For more information, see <a>how-to-error-msg</a>. </p>
    UnsupportedLanguagePair(String),
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

impl TranslateTextError {
    pub fn from_response(res: BufferedHttpResponse) -> TranslateTextError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DetectedLanguageLowConfidenceException" => {
                    return TranslateTextError::DetectedLanguageLowConfidence(String::from(
                        error_message,
                    ))
                }
                "InternalServerException" => {
                    return TranslateTextError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return TranslateTextError::InvalidRequest(String::from(error_message))
                }
                "ServiceUnavailableException" => {
                    return TranslateTextError::ServiceUnavailable(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return TranslateTextError::TextSizeLimitExceeded(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return TranslateTextError::TooManyRequests(String::from(error_message))
                }
                "UnsupportedLanguagePairException" => {
                    return TranslateTextError::UnsupportedLanguagePair(String::from(error_message))
                }
                "ValidationException" => {
                    return TranslateTextError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return TranslateTextError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TranslateTextError {
    fn from(err: serde_json::error::Error) -> TranslateTextError {
        TranslateTextError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TranslateTextError {
    fn from(err: CredentialsError) -> TranslateTextError {
        TranslateTextError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TranslateTextError {
    fn from(err: HttpDispatchError) -> TranslateTextError {
        TranslateTextError::HttpDispatch(err)
    }
}
impl From<io::Error> for TranslateTextError {
    fn from(err: io::Error) -> TranslateTextError {
        TranslateTextError::HttpDispatch(HttpDispatchError::from(err))
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
            TranslateTextError::ServiceUnavailable(ref cause) => cause,
            TranslateTextError::TextSizeLimitExceeded(ref cause) => cause,
            TranslateTextError::TooManyRequests(ref cause) => cause,
            TranslateTextError::UnsupportedLanguagePair(ref cause) => cause,
            TranslateTextError::Validation(ref cause) => cause,
            TranslateTextError::Credentials(ref err) => err.description(),
            TranslateTextError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TranslateTextError::ParseError(ref cause) => cause,
            TranslateTextError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Translate API. Amazon Translate clients implement this trait.
pub trait Translate {
    /// <p>Translates input text from the source language to the target language. You can translate between English (en) and one of the following languages, or between one of the following languages and English.</p> <ul> <li> <p>Arabic (ar)</p> </li> <li> <p>Chinese (Simplified) (zh)</p> </li> <li> <p>French (fr)</p> </li> <li> <p>German (de)</p> </li> <li> <p>Portuguese (pt)</p> </li> <li> <p>Spanish (es)</p> </li> </ul> <p>To have Amazon Translate determine the source language of your text, you can specify <code>auto</code> in the <code>SourceLanguageCode</code> field. If you specify <code>auto</code>, Amazon Translate will call Amazon Comprehend to determine the source language.</p>
    fn translate_text(
        &self,
        input: TranslateTextRequest,
    ) -> RusotoFuture<TranslateTextResponse, TranslateTextError>;
}
/// A client for the Amazon Translate API.
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
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TranslateClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        TranslateClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Translate for TranslateClient {
    /// <p>Translates input text from the source language to the target language. You can translate between English (en) and one of the following languages, or between one of the following languages and English.</p> <ul> <li> <p>Arabic (ar)</p> </li> <li> <p>Chinese (Simplified) (zh)</p> </li> <li> <p>French (fr)</p> </li> <li> <p>German (de)</p> </li> <li> <p>Portuguese (pt)</p> </li> <li> <p>Spanish (es)</p> </li> </ul> <p>To have Amazon Translate determine the source language of your text, you can specify <code>auto</code> in the <code>SourceLanguageCode</code> field. If you specify <code>auto</code>, Amazon Translate will call Amazon Comprehend to determine the source language.</p>
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
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TranslateTextResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TranslateTextError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
