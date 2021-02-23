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
#[allow(unused_imports)]
use rusoto_core::pagination::{aws_stream, Paged, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};
#[allow(unused_imports)]
use std::borrow::Cow;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Represents user interaction event information sent using the <code>PutEvents</code> API.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Event {
    /// <p>An ID associated with the event. If an event ID is not provided, Amazon Personalize generates a unique ID for the event. An event ID is not used as an input to the model. Amazon Personalize uses the event ID to distinquish unique events. Any subsequent events after the first with the same event ID are not used in model training.</p>
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p>The type of event, such as click or download. This property corresponds to the <code>EVENT_TYPE</code> field of your Interactions schema and depends on the types of events you are tracking.</p>
    #[serde(rename = "eventType")]
    pub event_type: String,
    /// <p>The event value that corresponds to the <code>EVENT_VALUE</code> field of the Interactions schema.</p>
    #[serde(rename = "eventValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_value: Option<f32>,
    /// <p>A list of item IDs that represents the sequence of items you have shown the user. For example, <code>["itemId1", "itemId2", "itemId3"]</code>.</p>
    #[serde(rename = "impression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression: Option<Vec<String>>,
    /// <p>The item ID key that corresponds to the <code>ITEM_ID</code> field of the Interactions schema.</p>
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// <p>A string map of event-specific data that you might choose to record. For example, if a user rates a movie on your site, other than movie ID (<code>itemId</code>) and rating (<code>eventValue</code>) , you might also send the number of movie ratings made by the user.</p> <p>Each item in the map consists of a key-value pair. For example,</p> <p> <code>{"numberOfRatings": "12"}</code> </p> <p>The keys use camel case names that match the fields in the Interactions schema. In the above example, the <code>numberOfRatings</code> would match the 'NUMBER_OF_RATINGS' field defined in the Interactions schema.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    /// <p>The ID of the recommendation.</p>
    #[serde(rename = "recommendationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    /// <p>The timestamp (in Unix time) on the client side when the event occurred.</p>
    #[serde(rename = "sentAt")]
    pub sent_at: f64,
}

/// <p>Represents item metadata added to an Items dataset using the <code>PutItems</code> API.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Item {
    /// <p>The ID associated with the item.</p>
    #[serde(rename = "itemId")]
    pub item_id: String,
    /// <p>A string map of item-specific metadata. Each element in the map consists of a key-value pair. For example, </p> <p> <code>{"numberOfRatings": "12"}</code> </p> <p>The keys use camel case names that match the fields in the Items schema. In the above example, the <code>numberOfRatings</code> would match the 'NUMBER_OF_RATINGS' field defined in the Items schema.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}

/// see [PersonalizeEvents::put_events]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventsRequest {
    /// <p>A list of event data from the session.</p>
    #[serde(rename = "eventList")]
    pub event_list: Vec<Event>,
    /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information see <a>event-record-api</a>.</p>
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
    /// <p>The user associated with the event.</p>
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// see [PersonalizeEvents::put_items]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutItemsRequest {
    /// <p>The Amazon Resource Number (ARN) of the Items dataset you are adding the item or items to.</p>
    #[serde(rename = "datasetArn")]
    pub dataset_arn: String,
    /// <p>A list of item data.</p>
    #[serde(rename = "items")]
    pub items: Vec<Item>,
}

/// see [PersonalizeEvents::put_users]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutUsersRequest {
    /// <p>The Amazon Resource Number (ARN) of the Users dataset you are adding the user or users to.</p>
    #[serde(rename = "datasetArn")]
    pub dataset_arn: String,
    /// <p>A list of user data.</p>
    #[serde(rename = "users")]
    pub users: Vec<User>,
}

/// <p>Represents user metadata added to a Users dataset using the <code>PutUsers</code> API.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct User {
    /// <p>A string map of user-specific metadata. Each element in the map consists of a key-value pair. For example, </p> <p> <code>{"numberOfVideosWatched": "45"}</code> </p> <p>The keys use camel case names that match the fields in the Users schema. In the above example, the <code>numberOfVideosWatched</code> would match the 'NUMBER_OF_VIDEOS_WATCHED' field defined in the Users schema.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    /// <p>The ID associated with the user.</p>
    #[serde(rename = "userId")]
    pub user_id: String,
}

/// Errors returned by PutEvents
#[derive(Debug, PartialEq)]
pub enum PutEventsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
}

impl PutEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(PutEventsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventsError {}
/// Errors returned by PutItems
#[derive(Debug, PartialEq)]
pub enum PutItemsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl PutItemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutItemsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(PutItemsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutItemsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutItemsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutItemsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PutItemsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutItemsError {}
/// Errors returned by PutUsers
#[derive(Debug, PartialEq)]
pub enum PutUsersError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>Could not find the specified resource.</p>
    ResourceNotFound(String),
}

impl PutUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutUsersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(PutUsersError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutUsersError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutUsersError::InvalidInput(ref cause) => write!(f, "{}", cause),
            PutUsersError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutUsersError {}
/// Trait representing the capabilities of the Amazon Personalize Events API. Amazon Personalize Events clients implement this trait.
#[async_trait]
pub trait PersonalizeEvents: Clone + Sync + Send + 'static {
    /// <p>Records user interaction event data. For more information see <a>event-record-api</a>.</p>
    async fn put_events(&self, input: PutEventsRequest) -> Result<(), RusotoError<PutEventsError>>;

    /// <p>Adds one or more items to an Items dataset. For more information see <a>importing-items</a>.</p>
    async fn put_items(&self, input: PutItemsRequest) -> Result<(), RusotoError<PutItemsError>>;

    /// <p>Adds one or more users to a Users dataset. For more information see <a>importing-users</a>.</p>
    async fn put_users(&self, input: PutUsersRequest) -> Result<(), RusotoError<PutUsersError>>;
}
/// A client for the Amazon Personalize Events API.
#[derive(Clone)]
pub struct PersonalizeEventsClient {
    client: Client,
    region: region::Region,
}

impl PersonalizeEventsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PersonalizeEventsClient {
        PersonalizeEventsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PersonalizeEventsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PersonalizeEventsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PersonalizeEventsClient {
        PersonalizeEventsClient { client, region }
    }
}

#[async_trait]
impl PersonalizeEvents for PersonalizeEventsClient {
    /// <p>Records user interaction event data. For more information see <a>event-record-api</a>.</p>
    #[allow(unused_mut)]
    async fn put_events(&self, input: PutEventsRequest) -> Result<(), RusotoError<PutEventsError>> {
        let request_uri = "/events";

        let mut request = SignedRequest::new("POST", "personalize", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("personalize-events".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEventsError::from_response(response))
        }
    }

    /// <p>Adds one or more items to an Items dataset. For more information see <a>importing-items</a>.</p>
    #[allow(unused_mut)]
    async fn put_items(&self, input: PutItemsRequest) -> Result<(), RusotoError<PutItemsError>> {
        let request_uri = "/items";

        let mut request = SignedRequest::new("POST", "personalize", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("personalize-events".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutItemsError::from_response(response))
        }
    }

    /// <p>Adds one or more users to a Users dataset. For more information see <a>importing-users</a>.</p>
    #[allow(unused_mut)]
    async fn put_users(&self, input: PutUsersRequest) -> Result<(), RusotoError<PutUsersError>> {
        let request_uri = "/users";

        let mut request = SignedRequest::new("POST", "personalize", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("personalize-events".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutUsersError::from_response(response))
        }
    }
}
