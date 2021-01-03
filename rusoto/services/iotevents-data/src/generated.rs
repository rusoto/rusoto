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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// <p>Contains information about the errors encountered.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPutMessageErrorEntry {
    /// <p>The code associated with the error.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>More information about the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the message that caused the error. (See the value corresponding to the <code>"messageId"</code> key in the <code>"message"</code> object.)</p>
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchPutMessageRequest {
    /// <p>The list of messages to send. Each message has the following format: <code>'{ "messageId": "string", "inputName": "string", "payload": "string"}'</code> </p>
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchPutMessageResponse {
    /// <p>A list of any errors encountered when sending the messages.</p>
    #[serde(rename = "BatchPutMessageErrorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_put_message_error_entries: Option<Vec<BatchPutMessageErrorEntry>>,
}

/// <p>Information about the error that occured when attempting to update a detector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateDetectorErrorEntry {
    /// <p>The code of the error.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A message describing the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The <code>"messageId"</code> of the update request that caused the error. (The value of the <code>"messageId"</code> in the update request <code>"Detector"</code> object.)</p>
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchUpdateDetectorRequest {
    /// <p>The list of detectors (instances) to update, along with the values to update.</p>
    #[serde(rename = "detectors")]
    pub detectors: Vec<UpdateDetectorRequest>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateDetectorResponse {
    /// <p>A list of those detector updates that resulted in errors. (If an error is listed here, the specific update did not occur.)</p>
    #[serde(rename = "batchUpdateDetectorErrorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_update_detector_error_entries: Option<Vec<BatchUpdateDetectorErrorEntry>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDetectorRequest {
    /// <p>The name of the detector model whose detectors (instances) you want information about.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>A filter used to limit results to detectors (instances) created because of the given key ID.</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDetectorResponse {
    /// <p>Information about the detector (instance).</p>
    #[serde(rename = "detector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector: Option<Detector>,
}

/// <p>Information about the detector (instance).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Detector {
    /// <p>The time the detector (instance) was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the detector model that created this detector (instance).</p>
    #[serde(rename = "detectorModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_name: Option<String>,
    /// <p>The version of the detector model that created this detector (instance).</p>
    #[serde(rename = "detectorModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version: Option<String>,
    /// <p>The value of the key (identifying the device or system) that caused the creation of this detector (instance).</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The time the detector (instance) was last updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The current state of the detector (instance).</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<DetectorState>,
}

/// <p>Information about the current state of the detector instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorState {
    /// <p>The name of the state.</p>
    #[serde(rename = "stateName")]
    pub state_name: String,
    /// <p>The current state of the detector's timers.</p>
    #[serde(rename = "timers")]
    pub timers: Vec<Timer>,
    /// <p>The current values of the detector's variables.</p>
    #[serde(rename = "variables")]
    pub variables: Vec<Variable>,
}

/// <p>The new state, variable values, and timer settings of the detector (instance).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectorStateDefinition {
    /// <p>The name of the new state of the detector (instance).</p>
    #[serde(rename = "stateName")]
    pub state_name: String,
    /// <p>The new values of the detector's timers. Any timer whose value isn't specified is cleared, and its timeout event won't occur.</p>
    #[serde(rename = "timers")]
    pub timers: Vec<TimerDefinition>,
    /// <p>The new values of the detector's variables. Any variable whose value isn't specified is cleared.</p>
    #[serde(rename = "variables")]
    pub variables: Vec<VariableDefinition>,
}

/// <p>Information about the detector state.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorStateSummary {
    /// <p>The name of the state.</p>
    #[serde(rename = "stateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
}

/// <p>Information about the detector (instance).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectorSummary {
    /// <p>The time the detector (instance) was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the detector model that created this detector (instance).</p>
    #[serde(rename = "detectorModelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_name: Option<String>,
    /// <p>The version of the detector model that created this detector (instance).</p>
    #[serde(rename = "detectorModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_model_version: Option<String>,
    /// <p>The value of the key (identifying the device or system) that caused the creation of this detector (instance).</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The time the detector (instance) was last updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The current state of the detector (instance).</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<DetectorStateSummary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDetectorsRequest {
    /// <p>The name of the detector model whose detectors (instances) are listed.</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A filter that limits results to those detectors (instances) in the given state.</p>
    #[serde(rename = "stateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDetectorsResponse {
    /// <p>A list of summary information about the detectors (instances).</p>
    #[serde(rename = "detectorSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_summaries: Option<Vec<DetectorSummary>>,
    /// <p>A token to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Information about a message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Message {
    /// <p>The name of the input into which the message payload is transformed.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
    /// <p>The ID to assign to the message. Within each batch sent, each <code>"messageId"</code> must be unique.</p>
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// <p>The payload of the message. This can be a JSON string or a Base-64-encoded string representing binary data (in which case you must decode it).</p>
    #[serde(rename = "payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub payload: bytes::Bytes,
}

/// <p>The current state of a timer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Timer {
    /// <p>The name of the timer.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The number of seconds which have elapsed on the timer.</p>
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}

/// <p>The new setting of a timer.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TimerDefinition {
    /// <p>The name of the timer.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The new setting of the timer (the number of seconds before the timer elapses).</p>
    #[serde(rename = "seconds")]
    pub seconds: i64,
}

/// <p>Information used to update the detector (instance).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDetectorRequest {
    /// <p>The name of the detector model that created the detectors (instances).</p>
    #[serde(rename = "detectorModelName")]
    pub detector_model_name: String,
    /// <p>The value of the input key attribute (identifying the device or system) that caused the creation of this detector (instance).</p>
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    /// <p>The ID to assign to the detector update <code>"message"</code>. Each <code>"messageId"</code> must be unique within each batch sent.</p>
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// <p>The new state, variable values, and timer settings of the detector (instance).</p>
    #[serde(rename = "state")]
    pub state: DetectorStateDefinition,
}

/// <p>The current state of the variable.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Variable {
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The current value of the variable.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>The new value of the variable.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VariableDefinition {
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The new value of the variable.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// Errors returned by BatchPutMessage
#[derive(Debug, PartialEq)]
pub enum BatchPutMessageError {
    /// <p>An internal failure occured.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchPutMessageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchPutMessageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchPutMessageError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchPutMessageError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchPutMessageError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchPutMessageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchPutMessageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchPutMessageError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchPutMessageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchPutMessageError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchPutMessageError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchPutMessageError {}
/// Errors returned by BatchUpdateDetector
#[derive(Debug, PartialEq)]
pub enum BatchUpdateDetectorError {
    /// <p>An internal failure occured.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl BatchUpdateDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchUpdateDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchUpdateDetectorError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchUpdateDetectorError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchUpdateDetectorError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchUpdateDetectorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchUpdateDetectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchUpdateDetectorError::InternalFailure(ref cause) => write!(f, "{}", cause),
            BatchUpdateDetectorError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchUpdateDetectorError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            BatchUpdateDetectorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchUpdateDetectorError {}
/// Errors returned by DescribeDetector
#[derive(Debug, PartialEq)]
pub enum DescribeDetectorError {
    /// <p>An internal failure occured.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl DescribeDetectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDetectorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDetectorError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDetectorError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDetectorError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeDetectorError::ServiceUnavailable(err.msg))
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
            DescribeDetectorError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DescribeDetectorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDetectorError {}
/// Errors returned by ListDetectors
#[derive(Debug, PartialEq)]
pub enum ListDetectorsError {
    /// <p>An internal failure occured.</p>
    InternalFailure(String),
    /// <p>The request was invalid.</p>
    InvalidRequest(String),
    /// <p>The resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request could not be completed due to throttling.</p>
    Throttling(String),
}

impl ListDetectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDetectorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDetectorsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDetectorsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDetectorsError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListDetectorsError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDetectorsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDetectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDetectorsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            ListDetectorsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDetectorsError {}
/// Trait representing the capabilities of the AWS IoT Events Data API. AWS IoT Events Data clients implement this trait.
#[async_trait]
pub trait IotEventsData {
    /// <p>Sends a set of messages to the AWS IoT Events system. Each message payload is transformed into the input you specify (<code>"inputName"</code>) and ingested into any detectors that monitor that input. If multiple messages are sent, the order in which the messages are processed isn't guaranteed. To guarantee ordering, you must send messages one at a time and wait for a successful response.</p>
    async fn batch_put_message(
        &self,
        input: BatchPutMessageRequest,
    ) -> Result<BatchPutMessageResponse, RusotoError<BatchPutMessageError>>;

    /// <p>Updates the state, variable values, and timer settings of one or more detectors (instances) of a specified detector model.</p>
    async fn batch_update_detector(
        &self,
        input: BatchUpdateDetectorRequest,
    ) -> Result<BatchUpdateDetectorResponse, RusotoError<BatchUpdateDetectorError>>;

    /// <p>Returns information about the specified detector (instance).</p>
    async fn describe_detector(
        &self,
        input: DescribeDetectorRequest,
    ) -> Result<DescribeDetectorResponse, RusotoError<DescribeDetectorError>>;

    /// <p>Lists detectors (the instances of a detector model).</p>
    async fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> Result<ListDetectorsResponse, RusotoError<ListDetectorsError>>;
}
/// A client for the AWS IoT Events Data API.
#[derive(Clone)]
pub struct IotEventsDataClient {
    client: Client,
    region: region::Region,
}

impl IotEventsDataClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IotEventsDataClient {
        IotEventsDataClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IotEventsDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        IotEventsDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> IotEventsDataClient {
        IotEventsDataClient { client, region }
    }
}

#[async_trait]
impl IotEventsData for IotEventsDataClient {
    /// <p>Sends a set of messages to the AWS IoT Events system. Each message payload is transformed into the input you specify (<code>"inputName"</code>) and ingested into any detectors that monitor that input. If multiple messages are sent, the order in which the messages are processed isn't guaranteed. To guarantee ordering, you must send messages one at a time and wait for a successful response.</p>
    #[allow(unused_mut)]
    async fn batch_put_message(
        &self,
        input: BatchPutMessageRequest,
    ) -> Result<BatchPutMessageResponse, RusotoError<BatchPutMessageError>> {
        let request_uri = "/inputs/messages";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchPutMessageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchPutMessageError::from_response(response))
        }
    }

    /// <p>Updates the state, variable values, and timer settings of one or more detectors (instances) of a specified detector model.</p>
    #[allow(unused_mut)]
    async fn batch_update_detector(
        &self,
        input: BatchUpdateDetectorRequest,
    ) -> Result<BatchUpdateDetectorResponse, RusotoError<BatchUpdateDetectorError>> {
        let request_uri = "/detectors";

        let mut request = SignedRequest::new("POST", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchUpdateDetectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchUpdateDetectorError::from_response(response))
        }
    }

    /// <p>Returns information about the specified detector (instance).</p>
    #[allow(unused_mut)]
    async fn describe_detector(
        &self,
        input: DescribeDetectorRequest,
    ) -> Result<DescribeDetectorResponse, RusotoError<DescribeDetectorError>> {
        let request_uri = format!(
            "/detectors/{detector_model_name}/keyValues/",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("GET", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.key_value {
            params.put("keyValue", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDetectorResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDetectorError::from_response(response))
        }
    }

    /// <p>Lists detectors (the instances of a detector model).</p>
    #[allow(unused_mut)]
    async fn list_detectors(
        &self,
        input: ListDetectorsRequest,
    ) -> Result<ListDetectorsResponse, RusotoError<ListDetectorsError>> {
        let request_uri = format!(
            "/detectors/{detector_model_name}",
            detector_model_name = input.detector_model_name
        );

        let mut request = SignedRequest::new("GET", "ioteventsdata", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.iotevents".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.state_name {
            params.put("stateName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDetectorsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDetectorsError::from_response(response))
        }
    }
}
