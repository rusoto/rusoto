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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p> An extracted segment of the text that is an attribute of an entity, or otherwise related to an entity, such as the dosage of a medication taken. It contains information about the attribute such as id, begin and end offset within the input text, and the segment of the input text. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Attribute {
    /// <p> The 0-based character offset in the input text that shows where the attribute begins. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "BeginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p> The 0-based character offset in the input text that shows where the attribute ends. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "EndOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p> The numeric identifier for this attribute. This is a monotonically increasing id unique within this response rather than a global unique identifier. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p> The level of confidence that Comprehend Medical has that this attribute is correctly related to this entity. </p>
    #[serde(rename = "RelationshipScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_score: Option<f32>,
    /// <p> The level of confidence that Comprehend Medical has that the segment of text is correctly recognized as an attribute. </p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p> The segment of input text extracted as this attribute.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p> Contextual information for this attribute. </p>
    #[serde(rename = "Traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<Trait>>,
    /// <p> The type of attribute. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectEntitiesRequest {
    /// <p> A UTF-8 text string containing the clinical content being examined for entities. Each string must contain fewer than 20,000 bytes of characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectEntitiesResponse {
    /// <p> The collection of medical entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence that Comprehend Medical has in the detection and analysis. Attributes and traits of the entity are also returned.</p>
    #[serde(rename = "Entities")]
    pub entities: Vec<Entity>,
    /// <p> If the result of the previous request to DetectEntities was truncated, include the Paginationtoken to fetch the next page of entities.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p> Attributes extracted from the input text that we were unable to relate to an entity.</p>
    #[serde(rename = "UnmappedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmapped_attributes: Option<Vec<UnmappedAttribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectPHIRequest {
    /// <p> A UTF-8 text string containing the clinical content being examined for PHI entities. Each string must contain fewer than 20,000 bytes of characters. </p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectPHIResponse {
    /// <p> The collection of PHI entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence that Comprehend Medical has in its detection. </p>
    #[serde(rename = "Entities")]
    pub entities: Vec<Entity>,
    /// <p> If the result of the previous request to DetectPHI was truncated, include the Paginationtoken to fetch the next page of PHI entities. </p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

/// <p> Provides information about an extracted medical entity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Entity {
    /// <p> The extracted attributes that relate to this entity.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p> The 0-based character offset in the input text that shows where the entity begins. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "BeginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p> The category of the entity.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p> The 0-based character offset in the input text that shows where the entity ends. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "EndOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p> The numeric identifier for the entity. This is a monotonically increasing id unique within this response rather than a global unique identifier. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The level of confidence that Comprehend Medical has in the accuracy of the detection.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p> The segment of input text extracted as this entity.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>Contextual information for the entity</p>
    #[serde(rename = "Traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<Trait>>,
    /// <p> Describes the specific type of entity with category of entities. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p> Provides contextual information about the extracted entity. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Trait {
    /// <p> Provides a name or contextual description about the trait. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The level of confidence that Comprehend Medical has in the accuracy of this trait.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

/// <p> An attribute that we extracted, but were unable to relate to an entity. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnmappedAttribute {
    /// <p> The specific attribute that has been extracted but not mapped to an entity. </p>
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<Attribute>,
    /// <p> The type of the attribute, could be one of the following values: "MEDICATION", "MEDICAL_CONDITION", "ANATOMY", "TEST_AND_TREATMENT_PROCEDURE" or "PERSONAL_HEALTH_INFORMATION". </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Errors returned by DetectEntities
#[derive(Debug, PartialEq)]
pub enum DetectEntitiesError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DetectEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectEntitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectEntitiesError::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(DetectEntitiesError::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectEntitiesError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DetectEntitiesError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectEntitiesError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetectEntitiesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectEntitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectEntitiesError {
    fn description(&self) -> &str {
        match *self {
            DetectEntitiesError::InternalServer(ref cause) => cause,
            DetectEntitiesError::InvalidEncoding(ref cause) => cause,
            DetectEntitiesError::InvalidRequest(ref cause) => cause,
            DetectEntitiesError::ServiceUnavailable(ref cause) => cause,
            DetectEntitiesError::TextSizeLimitExceeded(ref cause) => cause,
            DetectEntitiesError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DetectPHI
#[derive(Debug, PartialEq)]
pub enum DetectPHIError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DetectPHIError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectPHIError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectPHIError::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(DetectPHIError::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectPHIError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DetectPHIError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectPHIError::TextSizeLimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetectPHIError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectPHIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectPHIError {
    fn description(&self) -> &str {
        match *self {
            DetectPHIError::InternalServer(ref cause) => cause,
            DetectPHIError::InvalidEncoding(ref cause) => cause,
            DetectPHIError::InvalidRequest(ref cause) => cause,
            DetectPHIError::ServiceUnavailable(ref cause) => cause,
            DetectPHIError::TextSizeLimitExceeded(ref cause) => cause,
            DetectPHIError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the ComprehendMedical API. ComprehendMedical clients implement this trait.
pub trait ComprehendMedical {
    /// <p> Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information .</p>
    fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> RusotoFuture<DetectEntitiesResponse, DetectEntitiesError>;

    /// <p> Inspects the clinical text for personal health information (PHI) entities and entity category, location, and confidence score on that information.</p>
    fn detect_phi(
        &self,
        input: DetectPHIRequest,
    ) -> RusotoFuture<DetectPHIResponse, DetectPHIError>;
}
/// A client for the ComprehendMedical API.
#[derive(Clone)]
pub struct ComprehendMedicalClient {
    client: Client,
    region: region::Region,
}

impl ComprehendMedicalClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ComprehendMedicalClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ComprehendMedicalClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ComprehendMedicalClient {
        ComprehendMedicalClient { client, region }
    }
}

impl ComprehendMedical for ComprehendMedicalClient {
    /// <p> Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information .</p>
    fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> RusotoFuture<DetectEntitiesResponse, DetectEntitiesError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ComprehendMedical_20181030.DetectEntities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectEntitiesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectEntitiesError::from_response(response))),
                )
            }
        })
    }

    /// <p> Inspects the clinical text for personal health information (PHI) entities and entity category, location, and confidence score on that information.</p>
    fn detect_phi(
        &self,
        input: DetectPHIRequest,
    ) -> RusotoFuture<DetectPHIResponse, DetectPHIError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ComprehendMedical_20181030.DetectPHI");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectPHIResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectPHIError::from_response(response))),
                )
            }
        })
    }
}
