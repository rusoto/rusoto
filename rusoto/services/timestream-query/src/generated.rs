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

impl TimestreamQueryClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "timestream", &self.region, request_uri);
        request.set_endpoint_prefix("query.timestream".to_string());

        request.set_content_type("application/x-amz-json-1.0".to_owned());

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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelQueryRequest {
    /// <p> The id of the query that needs to be cancelled. <code>QueryID</code> is returned as part of QueryResult. </p>
    #[serde(rename = "QueryId")]
    pub query_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelQueryResponse {
    /// <p> A <code>CancellationMessage</code> is returned when a <code>CancelQuery</code> request for the query specified by <code>QueryId</code> has already been issued. </p>
    #[serde(rename = "CancellationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_message: Option<String>,
}

/// <p> Contains the meta data for query results such as the column names, data types, and other attributes. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ColumnInfo {
    /// <p> The name of the result set column. The name of the result set is available for columns of all data types except for arrays. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The data type of the result set column. The data type can be a scalar or complex. Scalar data types are integers, strings, doubles, booleans, and others. Complex data types are types such as arrays, rows, and others. </p>
    #[serde(rename = "Type")]
    pub type_: Box<Type>,
}

/// <p> Datum represents a single data point in a query result. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Datum {
    /// <p> Indicates if the data point is an array. </p>
    #[serde(rename = "ArrayValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_value: Option<Vec<Datum>>,
    /// <p> Indicates if the data point is null. </p>
    #[serde(rename = "NullValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value: Option<bool>,
    /// <p> Indicates if the data point is a row. </p>
    #[serde(rename = "RowValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_value: Option<Row>,
    /// <p> Indicates if the data point is a scalar value such as integer, string, double, or boolean. </p>
    #[serde(rename = "ScalarValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_value: Option<String>,
    /// <p> Indicates if the data point is of timeseries data type. </p>
    #[serde(rename = "TimeSeriesValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_value: Option<Vec<TimeSeriesDataPoint>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointsRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointsResponse {
    /// <p>An <code>Endpoints</code> object is returned when a <code>DescribeEndpoints</code> request is made.</p>
    #[serde(rename = "Endpoints")]
    pub endpoints: Vec<Endpoint>,
}

/// <p>Represents an available endpoint against which to make API calls agaisnt, as well as the TTL for that endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Endpoint {
    /// <p>An endpoint address.</p>
    #[serde(rename = "Address")]
    pub address: String,
    /// <p>The TTL for the endpoint, in minutes.</p>
    #[serde(rename = "CachePeriodInMinutes")]
    pub cache_period_in_minutes: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct QueryRequest {
    /// <p> Unique, case-sensitive string of up to 64 ASCII characters that you specify when you make a Query request. Providing a <code>ClientToken</code> makes the call to <code>Query</code> idempotent, meaning that multiple identical calls have the same effect as one single call. </p> <p>Your query request will fail in the following cases:</p> <ul> <li> <p> If you submit a request with the same client token outside the 5-minute idepotency window. </p> </li> <li> <p> If you submit a request with the same client token but a change in other parameters within the 5-minute idempotency window. </p> </li> </ul> <p> After 4 hours, any request with the same client token is treated as a new request. </p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p> The total number of rows to return in the output. If the total number of rows available is more than the value specified, a NextToken is provided in the command's output. To resume pagination, provide the NextToken value in the starting-token argument of a subsequent command. </p>
    #[serde(rename = "MaxRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rows: Option<i64>,
    /// <p> A pagination token passed to get a set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> The query to be executed by Timestream. </p>
    #[serde(rename = "QueryString")]
    pub query_string: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryResponse {
    /// <p> The column data types of the returned result set. </p>
    #[serde(rename = "ColumnInfo")]
    pub column_info: Vec<ColumnInfo>,
    /// <p> A pagination token that can be used again on a <code>Query</code> call to get the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p> A unique ID for the given query. </p>
    #[serde(rename = "QueryId")]
    pub query_id: String,
    /// <p>Information about the status of the query, including progress and bytes scannned.</p>
    #[serde(rename = "QueryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_status: Option<QueryStatus>,
    /// <p> The result set rows returned by the query. </p>
    #[serde(rename = "Rows")]
    pub rows: Vec<Row>,
}

/// <p>Information about the status of the query, including progress and bytes scannned.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueryStatus {
    /// <p>The amount of data scanned by the query in bytes that you will be charged for. This is a cumulative sum and represents the total amount of data that you will be charged for since the query was started. The charge is applied only once and is either applied when the query completes execution or when the query is cancelled. </p>
    #[serde(rename = "CumulativeBytesMetered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative_bytes_metered: Option<i64>,
    /// <p>The amount of data scanned by the query in bytes. This is a cumulative sum and represents the total amount of bytes scanned since the query was started. </p>
    #[serde(rename = "CumulativeBytesScanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative_bytes_scanned: Option<i64>,
    /// <p>The progress of the query, expressed as a percentage.</p>
    #[serde(rename = "ProgressPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<f64>,
}

/// <p>Represents a single row in the query results.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Row {
    /// <p>List of data points in a single row of the result set.</p>
    #[serde(rename = "Data")]
    pub data: Vec<Datum>,
}

/// <p>The timeseries datatype represents the values of a measure over time. A time series is an array of rows of timestamps and measure values, with rows sorted in ascending order of time. A TimeSeriesDataPoint is a single data point in the timeseries. It represents a tuple of (time, measure value) in a timeseries. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TimeSeriesDataPoint {
    /// <p>The timestamp when the measure value was collected.</p>
    #[serde(rename = "Time")]
    pub time: String,
    /// <p>The measure value for the data point.</p>
    #[serde(rename = "Value")]
    pub value: Datum,
}

/// <p>Contains the data type of a column in a query result set. The data type can be scalar or complex. The supported scalar data types are integers, boolean, string, double, timestamp, date, time, and intervals. The supported complex data types are arrays, rows, and timeseries.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Type {
    /// <p>Indicates if the column is an array.</p>
    #[serde(rename = "ArrayColumnInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_column_info: Option<ColumnInfo>,
    /// <p>Indicates if the column is a row.</p>
    #[serde(rename = "RowColumnInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_column_info: Option<Vec<ColumnInfo>>,
    /// <p>Indicates if the column is of type string, integer, boolean, double, timestamp, date, time. </p>
    #[serde(rename = "ScalarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_type: Option<String>,
    /// <p>Indicates if the column is a timeseries data type.</p>
    #[serde(rename = "TimeSeriesMeasureValueColumnInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_measure_value_column_info: Option<ColumnInfo>,
}

/// Errors returned by CancelQuery
#[derive(Debug, PartialEq)]
pub enum CancelQueryError {
    /// <p> You are not authorized to perform this action. </p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error. </p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CancelQueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelQueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CancelQueryError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CancelQueryError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(CancelQueryError::InvalidEndpoint(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelQueryError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelQueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelQueryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CancelQueryError::InternalServer(ref cause) => write!(f, "{}", cause),
            CancelQueryError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            CancelQueryError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelQueryError {}
/// Errors returned by DescribeEndpoints
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointsError {
    /// <p> Timestream was unable to fully process this request because of an internal server error. </p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeEndpointsError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeEndpointsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEndpointsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEndpointsError {}
/// Errors returned by Query
#[derive(Debug, PartialEq)]
pub enum QueryError {
    /// <p> You are not authorized to perform this action. </p>
    AccessDenied(String),
    /// <p> Unable to poll results for a cancelled query. </p>
    Conflict(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error. </p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p> Timestream was unable to run the query successfully. </p>
    QueryExecution(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl QueryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<QueryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(QueryError::AccessDenied(err.msg))
                }
                "ConflictException" => return RusotoError::Service(QueryError::Conflict(err.msg)),
                "InternalServerException" => {
                    return RusotoError::Service(QueryError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(QueryError::InvalidEndpoint(err.msg))
                }
                "QueryExecutionException" => {
                    return RusotoError::Service(QueryError::QueryExecution(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(QueryError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for QueryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            QueryError::Conflict(ref cause) => write!(f, "{}", cause),
            QueryError::InternalServer(ref cause) => write!(f, "{}", cause),
            QueryError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            QueryError::QueryExecution(ref cause) => write!(f, "{}", cause),
            QueryError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for QueryError {}
/// Trait representing the capabilities of the Timestream Query API. Timestream Query clients implement this trait.
#[async_trait]
pub trait TimestreamQuery {
    /// <p> Cancels a query that has been issued. Cancellation is guaranteed only if the query has not completed execution before the cancellation request was issued. Because cancellation is an idempotent operation, subsequent cancellation requests will return a <code>CancellationMessage</code>, indicating that the query has already been canceled. </p>
    async fn cancel_query(
        &self,
        input: CancelQueryRequest,
    ) -> Result<CancelQueryResponse, RusotoError<CancelQueryError>>;

    /// <p>DescribeEndpoints returns a list of available endpoints to make Timestream API calls against. This API is available through both Write and Query.</p> <p>Because Timestream’s SDKs are designed to transparently work with the service’s architecture, including the management and mapping of the service endpoints, <i>it is not recommended that you use this API unless</i>:</p> <ul> <li> <p>Your application uses a programming language that does not yet have SDK support</p> </li> <li> <p>You require better control over the client-side implementation</p> </li> </ul> <p>For detailed information on how to use DescribeEndpoints, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/Using-API.endpoint-discovery.html">The Endpoint Discovery Pattern and REST APIs</a>.</p>
    async fn describe_endpoints(
        &self,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>>;

    /// <p> Query is a synchronous operation that enables you to execute a query. Query will timeout after 60 seconds. You must update the default timeout in the SDK to support a timeout of 60 seconds. The result set will be truncated to 1MB. Service quotas apply. For more information, see Quotas in the Timestream Developer Guide. </p>
    async fn query(&self, input: QueryRequest) -> Result<QueryResponse, RusotoError<QueryError>>;
}
/// A client for the Timestream Query API.
#[derive(Clone)]
pub struct TimestreamQueryClient {
    client: Client,
    region: region::Region,
}

impl TimestreamQueryClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> TimestreamQueryClient {
        TimestreamQueryClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TimestreamQueryClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        TimestreamQueryClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> TimestreamQueryClient {
        TimestreamQueryClient { client, region }
    }
}

#[async_trait]
impl TimestreamQuery for TimestreamQueryClient {
    /// <p> Cancels a query that has been issued. Cancellation is guaranteed only if the query has not completed execution before the cancellation request was issued. Because cancellation is an idempotent operation, subsequent cancellation requests will return a <code>CancellationMessage</code>, indicating that the query has already been canceled. </p>
    async fn cancel_query(
        &self,
        input: CancelQueryRequest,
    ) -> Result<CancelQueryResponse, RusotoError<CancelQueryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.CancelQuery");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelQueryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CancelQueryResponse, _>()
    }

    /// <p>DescribeEndpoints returns a list of available endpoints to make Timestream API calls against. This API is available through both Write and Query.</p> <p>Because Timestream’s SDKs are designed to transparently work with the service’s architecture, including the management and mapping of the service endpoints, <i>it is not recommended that you use this API unless</i>:</p> <ul> <li> <p>Your application uses a programming language that does not yet have SDK support</p> </li> <li> <p>You require better control over the client-side implementation</p> </li> </ul> <p>For detailed information on how to use DescribeEndpoints, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/Using-API.endpoint-discovery.html">The Endpoint Discovery Pattern and REST APIs</a>.</p>
    async fn describe_endpoints(
        &self,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.DescribeEndpoints");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DescribeEndpointsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEndpointsResponse, _>()
    }

    /// <p> Query is a synchronous operation that enables you to execute a query. Query will timeout after 60 seconds. You must update the default timeout in the SDK to support a timeout of 60 seconds. The result set will be truncated to 1MB. Service quotas apply. For more information, see Quotas in the Timestream Developer Guide. </p>
    async fn query(&self, input: QueryRequest) -> Result<QueryResponse, RusotoError<QueryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.Query");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, QueryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<QueryResponse, _>()
    }
}
