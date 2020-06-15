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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPersonalizedRankingRequest {
    /// <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized ranking.</p>
    #[serde(rename = "campaignArn")]
    pub campaign_arn: String,
    /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of items (itemId's) to rank. If an item was not included in the training dataset, the item is appended to the end of the reranked list. The maximum is 500.</p>
    #[serde(rename = "inputList")]
    pub input_list: Vec<String>,
    /// <p>The user for which you want the campaign to provide a personalized ranking.</p>
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPersonalizedRankingResponse {
    /// <p>A list of items in order of most likely interest to the user. The maximum is 500.</p>
    #[serde(rename = "personalizedRanking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalized_ranking: Option<Vec<PredictedItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRecommendationsRequest {
    /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
    #[serde(rename = "campaignArn")]
    pub campaign_arn: String,
    /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    /// <p>The item ID to provide recommendations for.</p> <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// <p>The number of results to return. The default is 25. The maximum is 500.</p>
    #[serde(rename = "numResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// <p>The user ID to provide recommendations for.</p> <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRecommendationsResponse {
    /// <p>A list of recommendations sorted in ascending order by prediction score. There can be a maximum of 500 items in the list.</p>
    #[serde(rename = "itemList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_list: Option<Vec<PredictedItem>>,
}

/// <p>An object that identifies an item.</p> <p>The and APIs return a list of <code>PredictedItem</code>s.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PredictedItem {
    /// <p>The recommended item ID.</p>
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// <p>A numeric representation of the model's certainty in the item's suitability. For more information on scoring logic, see <a>how-scores-work</a>.</p>
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// Errors returned by GetPersonalizedRanking
#[derive(Debug, PartialEq)]
pub enum GetPersonalizedRankingError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl GetPersonalizedRankingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPersonalizedRankingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(GetPersonalizedRankingError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPersonalizedRankingError::ResourceNotFound(
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
impl fmt::Display for GetPersonalizedRankingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPersonalizedRankingError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetPersonalizedRankingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPersonalizedRankingError {}
/// Errors returned by GetRecommendations
#[derive(Debug, PartialEq)]
pub enum GetRecommendationsError {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInput(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl GetRecommendationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRecommendationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(GetRecommendationsError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRecommendationsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRecommendationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRecommendationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetRecommendationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRecommendationsError {}
/// Trait representing the capabilities of the Amazon Personalize Runtime API. Amazon Personalize Runtime clients implement this trait.
#[async_trait]
pub trait PersonalizeRuntime {
    /// <p><p>Re-ranks a list of recommended items for the given user. The first item in the list is deemed the most likely item to be of interest to the user.</p> <note> <p>The solution backing the campaign must have been created using a recipe of type PERSONALIZED_RANKING.</p> </note></p>
    async fn get_personalized_ranking(
        &self,
        input: GetPersonalizedRankingRequest,
    ) -> Result<GetPersonalizedRankingResponse, RusotoError<GetPersonalizedRankingError>>;

    /// <p><p>Returns a list of recommended items. The required input depends on the recipe type used to create the solution backing the campaign, as follows:</p> <ul> <li> <p>RELATED<em>ITEMS - <code>itemId</code> required, <code>userId</code> not used</p> </li> <li> <p>USER</em>PERSONALIZATION - <code>itemId</code> optional, <code>userId</code> required</p> </li> </ul> <note> <p>Campaigns that are backed by a solution created using a recipe of type PERSONALIZED_RANKING use the API.</p> </note></p>
    async fn get_recommendations(
        &self,
        input: GetRecommendationsRequest,
    ) -> Result<GetRecommendationsResponse, RusotoError<GetRecommendationsError>>;
}
/// A client for the Amazon Personalize Runtime API.
#[derive(Clone)]
pub struct PersonalizeRuntimeClient {
    client: Client,
    region: region::Region,
}

impl PersonalizeRuntimeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PersonalizeRuntimeClient {
        PersonalizeRuntimeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PersonalizeRuntimeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PersonalizeRuntimeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PersonalizeRuntimeClient {
        PersonalizeRuntimeClient { client, region }
    }
}

#[async_trait]
impl PersonalizeRuntime for PersonalizeRuntimeClient {
    /// <p><p>Re-ranks a list of recommended items for the given user. The first item in the list is deemed the most likely item to be of interest to the user.</p> <note> <p>The solution backing the campaign must have been created using a recipe of type PERSONALIZED_RANKING.</p> </note></p>
    #[allow(unused_mut)]
    async fn get_personalized_ranking(
        &self,
        input: GetPersonalizedRankingRequest,
    ) -> Result<GetPersonalizedRankingResponse, RusotoError<GetPersonalizedRankingError>> {
        let request_uri = "/personalize-ranking";

        let mut request = SignedRequest::new("POST", "personalize", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("personalize-runtime".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPersonalizedRankingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPersonalizedRankingError::from_response(response))
        }
    }

    /// <p><p>Returns a list of recommended items. The required input depends on the recipe type used to create the solution backing the campaign, as follows:</p> <ul> <li> <p>RELATED<em>ITEMS - <code>itemId</code> required, <code>userId</code> not used</p> </li> <li> <p>USER</em>PERSONALIZATION - <code>itemId</code> optional, <code>userId</code> required</p> </li> </ul> <note> <p>Campaigns that are backed by a solution created using a recipe of type PERSONALIZED_RANKING use the API.</p> </note></p>
    #[allow(unused_mut)]
    async fn get_recommendations(
        &self,
        input: GetRecommendationsRequest,
    ) -> Result<GetRecommendationsResponse, RusotoError<GetRecommendationsError>> {
        let request_uri = "/recommendations";

        let mut request = SignedRequest::new("POST", "personalize", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("personalize-runtime".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRecommendationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRecommendationsError::from_response(response))
        }
    }
}
