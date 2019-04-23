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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>An object that defines a message that contains text formatted using Amazon Pinpoint Voice Instructions markup.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CallInstructionsMessageType {
    /// <p>The language to use when delivering the message. For a complete list of supported languages, see the Amazon Polly Developer Guide.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// <p>An object that contains information about an event destination that sends data to Amazon CloudWatch Logs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchLogsDestination {
    /// <p>The Amazon Resource Name (ARN) of an Amazon Identity and Access Management (IAM) role that is able to write event data to an Amazon CloudWatch destination.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    /// <p>The name of the Amazon CloudWatch Log Group that you want to record events in.</p>
    #[serde(rename = "LogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

/// <p>Create a new event destination in a configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConfigurationSetEventDestinationRequest {
    /// <p>ConfigurationSetName</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    #[serde(rename = "EventDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destination: Option<EventDestinationDefinition>,
    /// <p>A name that identifies the event destination.</p>
    #[serde(rename = "EventDestinationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destination_name: Option<String>,
}

/// <p>An empty object that indicates that the event destination was created successfully.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateConfigurationSetEventDestinationResponse {}

/// <p>A request to create a new configuration set.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConfigurationSetRequest {
    /// <p>The name that you want to give the configuration set.</p>
    #[serde(rename = "ConfigurationSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
}

/// <p>An empty object that indicates that the configuration set was successfully created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateConfigurationSetResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConfigurationSetEventDestinationRequest {
    /// <p>ConfigurationSetName</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    /// <p>EventDestinationName</p>
    #[serde(rename = "EventDestinationName")]
    pub event_destination_name: String,
}

/// <p>An empty object that indicates that the event destination was deleted successfully.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteConfigurationSetEventDestinationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConfigurationSetRequest {
    /// <p>ConfigurationSetName</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
}

/// <p>An empty object that indicates that the configuration set was deleted successfully.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteConfigurationSetResponse {}

/// <p>An object that defines an event destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventDestination {
    #[serde(rename = "CloudWatchLogsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_destination: Option<CloudWatchLogsDestination>,
    /// <p>Indicates whether or not the event destination is enabled. If the event destination is enabled, then Amazon Pinpoint sends response data to the specified event destination.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "KinesisFirehoseDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    #[serde(rename = "MatchingEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_event_types: Option<Vec<String>>,
    /// <p>A name that identifies the event destination configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SnsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

/// <p>An object that defines a single event destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EventDestinationDefinition {
    #[serde(rename = "CloudWatchLogsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_destination: Option<CloudWatchLogsDestination>,
    /// <p>Indicates whether or not the event destination is enabled. If the event destination is enabled, then Amazon Pinpoint sends response data to the specified event destination.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "KinesisFirehoseDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    #[serde(rename = "MatchingEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_event_types: Option<Vec<String>>,
    #[serde(rename = "SnsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConfigurationSetEventDestinationsRequest {
    /// <p>ConfigurationSetName</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
}

/// <p>An object that contains information about an event destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetConfigurationSetEventDestinationsResponse {
    #[serde(rename = "EventDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destinations: Option<Vec<EventDestination>>,
}

/// <p>An object that contains information about an event destination that sends data to Amazon Kinesis Data Firehose.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisFirehoseDestination {
    /// <p>The Amazon Resource Name (ARN) of an IAM role that can write data to an Amazon Kinesis Data Firehose stream.</p>
    #[serde(rename = "DeliveryStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis Data Firehose destination that you want to use in the event destination.</p>
    #[serde(rename = "IamRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListConfigurationSetsRequest {
    /// <p>A token returned from a previous call to the API that indicates the position in the list of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Used to specify the number of items that should be returned in the response.</p>
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
}

/// <p>An object that contains information about the configuration sets for your account in the current region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListConfigurationSetsResponse {
    /// <p>An object that contains a list of configuration sets for your account in the current region.</p>
    #[serde(rename = "ConfigurationSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_sets: Option<Vec<String>>,
    /// <p>A token returned from a previous call to ListConfigurationSets to indicate the position in the list of configuration sets.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>An object that defines a message that contains unformatted text.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PlainTextMessageType {
    /// <p>The language to use when delivering the message. For a complete list of supported languages, see the Amazon Polly Developer Guide.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The plain (not SSML-formatted) text to deliver to the recipient.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>The name of the voice that you want to use to deliver the message. For a complete list of supported voices, see the Amazon Polly Developer Guide.</p>
    #[serde(rename = "VoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

/// <p>An object that defines a message that contains SSML-formatted text.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SSMLMessageType {
    /// <p>The language to use when delivering the message. For a complete list of supported languages, see the Amazon Polly Developer Guide.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The SSML-formatted text to deliver to the recipient.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>The name of the voice that you want to use to deliver the message. For a complete list of supported voices, see the Amazon Polly Developer Guide.</p>
    #[serde(rename = "VoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

/// <p>SendVoiceMessageRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendVoiceMessageRequest {
    /// <p>The phone number that appears on recipients&#39; devices when they receive the message.</p>
    #[serde(rename = "CallerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<String>,
    /// <p>The name of the configuration set that you want to use to send the message.</p>
    #[serde(rename = "ConfigurationSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<VoiceMessageContent>,
    /// <p>The phone number that you want to send the voice message to.</p>
    #[serde(rename = "DestinationPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_phone_number: Option<String>,
    /// <p>The phone number that Amazon Pinpoint should use to send the voice message. This isn&#39;t necessarily the phone number that appears on recipients&#39; devices when they receive the message, because you can specify a CallerId parameter in the request.</p>
    #[serde(rename = "OriginationPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_phone_number: Option<String>,
}

/// <p>An object that that contains the Message ID of a Voice message that was sent successfully.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SendVoiceMessageResponse {
    /// <p>A unique identifier for the voice message.</p>
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

/// <p>An object that contains information about an event destination that sends data to Amazon SNS.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnsDestination {
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic that you want to publish events to.</p>
    #[serde(rename = "TopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

/// <p>UpdateConfigurationSetEventDestinationRequest</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateConfigurationSetEventDestinationRequest {
    /// <p>ConfigurationSetName</p>
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: String,
    #[serde(rename = "EventDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destination: Option<EventDestinationDefinition>,
    /// <p>EventDestinationName</p>
    #[serde(rename = "EventDestinationName")]
    pub event_destination_name: String,
}

/// <p>An empty object that indicates that the event destination was updated successfully.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateConfigurationSetEventDestinationResponse {}

/// <p>An object that contains a voice message and information about the recipient that you want to send it to.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VoiceMessageContent {
    #[serde(rename = "CallInstructionsMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_instructions_message: Option<CallInstructionsMessageType>,
    #[serde(rename = "PlainTextMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text_message: Option<PlainTextMessageType>,
    #[serde(rename = "SSMLMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssml_message: Option<SSMLMessageType>,
}

/// Errors returned by CreateConfigurationSet
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The API encountered an unexpected error and couldn&#39;t complete the request. You might be able to successfully issue the request again in the future.</p>
    InternalServiceError(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>You&#39;ve issued too many requests to the resource. Wait a few minutes, and then try again.</p>
    TooManyRequests(String),
}

impl CreateConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConfigurationSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateConfigurationSetError::AlreadyExists(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateConfigurationSetError::BadRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(CreateConfigurationSetError::InternalServiceError(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateConfigurationSetError::LimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateConfigurationSetError::TooManyRequests(
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
impl fmt::Display for CreateConfigurationSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConfigurationSetError {
    fn description(&self) -> &str {
        match *self {
            CreateConfigurationSetError::AlreadyExists(ref cause) => cause,
            CreateConfigurationSetError::BadRequest(ref cause) => cause,
            CreateConfigurationSetError::InternalServiceError(ref cause) => cause,
            CreateConfigurationSetError::LimitExceeded(ref cause) => cause,
            CreateConfigurationSetError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetEventDestinationError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The API encountered an unexpected error and couldn&#39;t complete the request. You might be able to successfully issue the request again in the future.</p>
    InternalServiceError(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>You&#39;ve issued too many requests to the resource. Wait a few minutes, and then try again.</p>
    TooManyRequests(String),
}

impl CreateConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::AlreadyExists(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::InternalServiceError(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::LimitExceeded(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateConfigurationSetEventDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConfigurationSetEventDestinationError {
    fn description(&self) -> &str {
        match *self {
            CreateConfigurationSetEventDestinationError::AlreadyExists(ref cause) => cause,
            CreateConfigurationSetEventDestinationError::BadRequest(ref cause) => cause,
            CreateConfigurationSetEventDestinationError::InternalServiceError(ref cause) => cause,
            CreateConfigurationSetEventDestinationError::LimitExceeded(ref cause) => cause,
            CreateConfigurationSetEventDestinationError::NotFound(ref cause) => cause,
            CreateConfigurationSetEventDestinationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationSet
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The API encountered an unexpected error and couldn&#39;t complete the request. You might be able to successfully issue the request again in the future.</p>
    InternalServiceError(String),
    /// <p>The resource you attempted to access doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>You&#39;ve issued too many requests to the resource. Wait a few minutes, and then try again.</p>
    TooManyRequests(String),
}

impl DeleteConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConfigurationSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::BadRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::InternalServiceError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::TooManyRequests(
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
impl fmt::Display for DeleteConfigurationSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationSetError::BadRequest(ref cause) => cause,
            DeleteConfigurationSetError::InternalServiceError(ref cause) => cause,
            DeleteConfigurationSetError::NotFound(ref cause) => cause,
            DeleteConfigurationSetError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetEventDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The API encountered an unexpected error and couldn&#39;t complete the request. You might be able to successfully issue the request again in the future.</p>
    InternalServiceError(String),
    /// <p>The resource you attempted to access doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>You&#39;ve issued too many requests to the resource. Wait a few minutes, and then try again.</p>
    TooManyRequests(String),
}

impl DeleteConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::InternalServiceError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConfigurationSetEventDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationSetEventDestinationError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationSetEventDestinationError::BadRequest(ref cause) => cause,
            DeleteConfigurationSetEventDestinationError::InternalServiceError(ref cause) => cause,
            DeleteConfigurationSetEventDestinationError::NotFound(ref cause) => cause,
            DeleteConfigurationSetEventDestinationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConfigurationSetEventDestinations
#[derive(Debug, PartialEq)]
pub enum GetConfigurationSetEventDestinationsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The API encountered an unexpected error and couldn&#39;t complete the request. You might be able to successfully issue the request again in the future.</p>
    InternalServiceError(String),
    /// <p>The resource you attempted to access doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>You&#39;ve issued too many requests to the resource. Wait a few minutes, and then try again.</p>
    TooManyRequests(String),
}

impl GetConfigurationSetEventDestinationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetConfigurationSetEventDestinationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::BadRequest(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::InternalServiceError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConfigurationSetEventDestinationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConfigurationSetEventDestinationsError {
    fn description(&self) -> &str {
        match *self {
            GetConfigurationSetEventDestinationsError::BadRequest(ref cause) => cause,
            GetConfigurationSetEventDestinationsError::InternalServiceError(ref cause) => cause,
            GetConfigurationSetEventDestinationsError::NotFound(ref cause) => cause,
            GetConfigurationSetEventDestinationsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConfigurationSets
#[derive(Debug, PartialEq)]
pub enum ListConfigurationSetsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The API encountered an unexpected error and couldn&#39;t complete the request. You might be able to successfully issue the request again in the future.</p>
    InternalServiceError(String),
    /// <p>You&#39;ve issued too many requests to the resource. Wait a few minutes, and then try again.</p>
    TooManyRequests(String),
}

impl ListConfigurationSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConfigurationSetsError::BadRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(ListConfigurationSetsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListConfigurationSetsError::TooManyRequests(
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
impl fmt::Display for ListConfigurationSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConfigurationSetsError {
    fn description(&self) -> &str {
        match *self {
            ListConfigurationSetsError::BadRequest(ref cause) => cause,
            ListConfigurationSetsError::InternalServiceError(ref cause) => cause,
            ListConfigurationSetsError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by SendVoiceMessage
#[derive(Debug, PartialEq)]
pub enum SendVoiceMessageError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The API encountered an unexpected error and couldn&#39;t complete the request. You might be able to successfully issue the request again in the future.</p>
    InternalServiceError(String),
    /// <p>You&#39;ve issued too many requests to the resource. Wait a few minutes, and then try again.</p>
    TooManyRequests(String),
}

impl SendVoiceMessageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendVoiceMessageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(SendVoiceMessageError::BadRequest(err.msg))
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(SendVoiceMessageError::InternalServiceError(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SendVoiceMessageError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendVoiceMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendVoiceMessageError {
    fn description(&self) -> &str {
        match *self {
            SendVoiceMessageError::BadRequest(ref cause) => cause,
            SendVoiceMessageError::InternalServiceError(ref cause) => cause,
            SendVoiceMessageError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetEventDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The API encountered an unexpected error and couldn&#39;t complete the request. You might be able to successfully issue the request again in the future.</p>
    InternalServiceError(String),
    /// <p>The resource you attempted to access doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>You&#39;ve issued too many requests to the resource. Wait a few minutes, and then try again.</p>
    TooManyRequests(String),
}

impl UpdateConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "InternalServiceErrorException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::InternalServiceError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateConfigurationSetEventDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateConfigurationSetEventDestinationError {
    fn description(&self) -> &str {
        match *self {
            UpdateConfigurationSetEventDestinationError::BadRequest(ref cause) => cause,
            UpdateConfigurationSetEventDestinationError::InternalServiceError(ref cause) => cause,
            UpdateConfigurationSetEventDestinationError::NotFound(ref cause) => cause,
            UpdateConfigurationSetEventDestinationError::TooManyRequests(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Pinpoint SMS Voice API. Pinpoint SMS Voice clients implement this trait.
pub trait PinpointSmsVoice {
    /// <p>Create a new configuration set. After you create the configuration set, you can add one or more event destinations to it.</p>
    fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> RusotoFuture<CreateConfigurationSetResponse, CreateConfigurationSetError>;

    /// <p>Create a new event destination in a configuration set.</p>
    fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        CreateConfigurationSetEventDestinationResponse,
        CreateConfigurationSetEventDestinationError,
    >;

    /// <p>Deletes an existing configuration set.</p>
    fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> RusotoFuture<DeleteConfigurationSetResponse, DeleteConfigurationSetError>;

    /// <p>Deletes an event destination in a configuration set.</p>
    fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        DeleteConfigurationSetEventDestinationResponse,
        DeleteConfigurationSetEventDestinationError,
    >;

    /// <p>Obtain information about an event destination, including the types of events it reports, the Amazon Resource Name (ARN) of the destination, and the name of the event destination.</p>
    fn get_configuration_set_event_destinations(
        &self,
        input: GetConfigurationSetEventDestinationsRequest,
    ) -> RusotoFuture<
        GetConfigurationSetEventDestinationsResponse,
        GetConfigurationSetEventDestinationsError,
    >;

    /// <p>List all of the configuration sets associated with your Amazon Pinpoint account in the current region.</p>
    fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> RusotoFuture<ListConfigurationSetsResponse, ListConfigurationSetsError>;

    /// <p>Create a new voice message and send it to a recipient&#39;s phone number.</p>
    fn send_voice_message(
        &self,
        input: SendVoiceMessageRequest,
    ) -> RusotoFuture<SendVoiceMessageResponse, SendVoiceMessageError>;

    /// <p>Update an event destination in a configuration set. An event destination is a location that you publish information about your voice calls to. For example, you can log an event to an Amazon CloudWatch destination when a call fails.</p>
    fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        UpdateConfigurationSetEventDestinationResponse,
        UpdateConfigurationSetEventDestinationError,
    >;
}
/// A client for the Pinpoint SMS Voice API.
#[derive(Clone)]
pub struct PinpointSmsVoiceClient {
    client: Client,
    region: region::Region,
}

impl PinpointSmsVoiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PinpointSmsVoiceClient {
        PinpointSmsVoiceClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PinpointSmsVoiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        PinpointSmsVoiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl PinpointSmsVoice for PinpointSmsVoiceClient {
    /// <p>Create a new configuration set. After you create the configuration set, you can add one or more event destinations to it.</p>
    fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> RusotoFuture<CreateConfigurationSetResponse, CreateConfigurationSetError> {
        let request_uri = "/v1/sms-voice/configuration-sets";

        let mut request = SignedRequest::new("POST", "sms-voice", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("sms-voice.pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateConfigurationSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateConfigurationSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Create a new event destination in a configuration set.</p>
    fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        CreateConfigurationSetEventDestinationResponse,
        CreateConfigurationSetEventDestinationError,
    > {
        let request_uri = format!(
            "/v1/sms-voice/configuration-sets/{configuration_set_name}/event-destinations",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("POST", "sms-voice", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("sms-voice.pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<
                        CreateConfigurationSetEventDestinationResponse,
                    >(&body)
                    .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateConfigurationSetEventDestinationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes an existing configuration set.</p>
    fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> RusotoFuture<DeleteConfigurationSetResponse, DeleteConfigurationSetError> {
        let request_uri = format!(
            "/v1/sms-voice/configuration-sets/{configuration_set_name}",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("DELETE", "sms-voice", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("sms-voice.pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DeleteConfigurationSetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteConfigurationSetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an event destination in a configuration set.</p>
    fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        DeleteConfigurationSetEventDestinationResponse,
        DeleteConfigurationSetEventDestinationError,
    > {
        let request_uri = format!("/v1/sms-voice/configuration-sets/{configuration_set_name}/event-destinations/{event_destination_name}", configuration_set_name = input.configuration_set_name, event_destination_name = input.event_destination_name);

        let mut request = SignedRequest::new("DELETE", "sms-voice", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("sms-voice.pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<
                        DeleteConfigurationSetEventDestinationResponse,
                    >(&body)
                    .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationSetEventDestinationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Obtain information about an event destination, including the types of events it reports, the Amazon Resource Name (ARN) of the destination, and the name of the event destination.</p>
    fn get_configuration_set_event_destinations(
        &self,
        input: GetConfigurationSetEventDestinationsRequest,
    ) -> RusotoFuture<
        GetConfigurationSetEventDestinationsResponse,
        GetConfigurationSetEventDestinationsError,
    > {
        let request_uri = format!(
            "/v1/sms-voice/configuration-sets/{configuration_set_name}/event-destinations",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("GET", "sms-voice", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("sms-voice.pinpoint".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<
                        GetConfigurationSetEventDestinationsResponse,
                    >(&body)
                    .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetConfigurationSetEventDestinationsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>List all of the configuration sets associated with your Amazon Pinpoint account in the current region.</p>
    fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> RusotoFuture<ListConfigurationSetsResponse, ListConfigurationSetsError> {
        let request_uri = "/v1/sms-voice/configuration-sets";

        let mut request = SignedRequest::new("GET", "sms-voice", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("sms-voice.pinpoint".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListConfigurationSetsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListConfigurationSetsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Create a new voice message and send it to a recipient&#39;s phone number.</p>
    fn send_voice_message(
        &self,
        input: SendVoiceMessageRequest,
    ) -> RusotoFuture<SendVoiceMessageResponse, SendVoiceMessageError> {
        let request_uri = "/v1/sms-voice/voice/message";

        let mut request = SignedRequest::new("POST", "sms-voice", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("sms-voice.pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<SendVoiceMessageResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendVoiceMessageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Update an event destination in a configuration set. An event destination is a location that you publish information about your voice calls to. For example, you can log an event to an Amazon CloudWatch destination when a call fails.</p>
    fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> RusotoFuture<
        UpdateConfigurationSetEventDestinationResponse,
        UpdateConfigurationSetEventDestinationError,
    > {
        let request_uri = format!("/v1/sms-voice/configuration-sets/{configuration_set_name}/event-destinations/{event_destination_name}", configuration_set_name = input.configuration_set_name, event_destination_name = input.event_destination_name);

        let mut request = SignedRequest::new("PUT", "sms-voice", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("sms-voice.pinpoint".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<
                        UpdateConfigurationSetEventDestinationResponse,
                    >(&body)
                    .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateConfigurationSetEventDestinationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
