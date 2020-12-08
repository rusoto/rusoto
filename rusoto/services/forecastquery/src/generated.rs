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

impl ForecastQueryClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "forecast", &self.region, request_uri);
        request.set_endpoint_prefix("forecastquery".to_string());

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch(
        &self,
        request: SignedRequest,
    ) -> Result<HttpResponse, RusotoError<std::convert::Infallible>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await?;
            return Err(RusotoError::Unknown(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>The forecast value for a specific date. Part of the <a>Forecast</a> object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataPoint {
    /// <p>The timestamp of the specific forecast.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// <p>The forecast value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>Provides information about a forecast. Returned as part of the <a>QueryForecast</a> response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Forecast {
    /// <p><p>The forecast.</p> <p>The <i>string</i> of the string-to-array map is one of the following values:</p> <ul> <li> <p>p10</p> </li> <li> <p>p50</p> </li> <li> <p>p90</p> </li> </ul></p>
    #[serde(rename = "Predictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictions: Option<::std::collections::HashMap<String, Vec<DataPoint>>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryForecastRequest {
    /// <p>The end date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p> <p> <code>{"item_id" : "client_21"}</code> </p> <p>To get the full forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateForecastExportJob.html">CreateForecastExportJob</a> operation.</p>
    #[serde(rename = "Filters")]
    pub filters: ::std::collections::HashMap<String, String>,
    /// <p>The Amazon Resource Name (ARN) of the forecast to query.</p>
    #[serde(rename = "ForecastArn")]
    pub forecast_arn: String,
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The start date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryForecastResponse {
    /// <p>The forecast.</p>
    #[serde(rename = "Forecast")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<Forecast>,
}

/// Errors returned by QueryForecast
#[derive(Debug, PartialEq)]
pub enum QueryForecastError {
    /// <p>The value is invalid or is too long.</p>
    InvalidInput(String),
    /// <p>The token is not valid. Tokens expire after 24 hours.</p>
    InvalidNextToken(String),
    /// <p>The limit on the number of requests per second has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified resource is in use.</p>
    ResourceInUse(String),
    /// <p>We can't find that resource. Check the information that you've provided and try again.</p>
    ResourceNotFound(String),
}

impl QueryForecastError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<QueryForecastError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInputException" => {
                    return RusotoError::Service(QueryForecastError::InvalidInput(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(QueryForecastError::InvalidNextToken(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(QueryForecastError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(QueryForecastError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(QueryForecastError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<QueryForecastError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for QueryForecastError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryForecastError::InvalidInput(ref cause) => write!(f, "{}", cause),
            QueryForecastError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            QueryForecastError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            QueryForecastError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            QueryForecastError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for QueryForecastError {}
/// Trait representing the capabilities of the Amazon Forecast Query Service API. Amazon Forecast Query Service clients implement this trait.
#[async_trait]
pub trait ForecastQuery {
    /// <p><p>Retrieves a forecast for a single item, filtered by the supplied criteria.</p> <p>The criteria is a key-value pair. The key is either <code>item<em>id</code> (or the equivalent non-timestamp, non-target field) from the <code>TARGET</em>TIME<em>SERIES</code> dataset, or one of the forecast dimensions specified as part of the <code>FeaturizationConfig</code> object.</p> <p>By default, <code>QueryForecast</code> returns the complete date range for the filtered forecast. You can request a specific date range.</p> <p>To get the full forecast, use the &lt;a href=&quot;https://docs.aws.amazon.com/en</em>us/forecast/latest/dg/API_CreateForecastExportJob.html&quot;&gt;CreateForecastExportJob</a> operation.</p> <note> <p>The forecasts generated by Amazon Forecast are in the same timezone as the dataset that was used to create the predictor.</p> </note></p>
    async fn query_forecast(
        &self,
        input: QueryForecastRequest,
    ) -> Result<QueryForecastResponse, RusotoError<QueryForecastError>>;
}
/// A client for the Amazon Forecast Query Service API.
#[derive(Clone)]
pub struct ForecastQueryClient {
    client: Client,
    region: region::Region,
}

impl ForecastQueryClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ForecastQueryClient {
        ForecastQueryClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ForecastQueryClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ForecastQueryClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ForecastQueryClient {
        ForecastQueryClient { client, region }
    }
}

#[async_trait]
impl ForecastQuery for ForecastQueryClient {
    /// <p><p>Retrieves a forecast for a single item, filtered by the supplied criteria.</p> <p>The criteria is a key-value pair. The key is either <code>item<em>id</code> (or the equivalent non-timestamp, non-target field) from the <code>TARGET</em>TIME<em>SERIES</code> dataset, or one of the forecast dimensions specified as part of the <code>FeaturizationConfig</code> object.</p> <p>By default, <code>QueryForecast</code> returns the complete date range for the filtered forecast. You can request a specific date range.</p> <p>To get the full forecast, use the &lt;a href=&quot;https://docs.aws.amazon.com/en</em>us/forecast/latest/dg/API_CreateForecastExportJob.html&quot;&gt;CreateForecastExportJob</a> operation.</p> <note> <p>The forecasts generated by Amazon Forecast are in the same timezone as the dataset that was used to create the predictor.</p> </note></p>
    async fn query_forecast(
        &self,
        input: QueryForecastRequest,
    ) -> Result<QueryForecastResponse, RusotoError<QueryForecastError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonForecastRuntime.QueryForecast");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(QueryForecastError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<QueryForecastResponse, _>()
    }
}
