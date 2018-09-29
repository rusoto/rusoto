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
/// <p>A timestamp, and a single numerical value, which together represent a measurement at a particular point in time.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DataPoint {
    /// <p>The time, in epoch format, associated with a particular <code>Value</code>.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
    /// <p>The actual value associated with a particular <code>Timestamp</code>.</p>
    #[serde(rename = "Value")]
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDimensionKeysRequest {
    /// <p>The date and time specifying the end of the requested time series data. The value specified is <i>exclusive</i> - data points less than (but not equal to) <code>EndTime</code> will be returned.</p> <p>The value for <code>EndTime</code> must be later than the value for <code>StartTime</code>.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p><p>One or more filters to apply in the request. Restrictions:</p> <ul> <li> <p>Any number of filters by the same dimension, as specified in the <code>GroupBy</code> or <code>Partition</code> parameters.</p> </li> <li> <p>A single filter for any other dimension in this dimension group.</p> </li> </ul></p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<::std::collections::HashMap<String, String>>,
    /// <p>A specification for how to aggregate the data points from a query result. You must specify a valid dimension group. Performance Insights will return all of the dimensions within that group, unless you provide the names of specific dimensions within that group. You can also request that Performance Insights return a limited number of values for a dimension.</p>
    #[serde(rename = "GroupBy")]
    pub group_by: DimensionGroup,
    /// <p>An immutable, AWS Region-unique identifier for a data source. Performance Insights gathers metrics from this data source.</p> <p>To use an Amazon RDS instance as a data source, you specify its <code>DbiResourceId</code> value - for example: <code>db-FAIHNTYBKTGAUSUZQYPDS2GW4A</code> </p>
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// <p>The maximum number of items to return in the response. If more items exist than the specified <code>MaxRecords</code> value, a pagination token is included in the response so that the remaining results can be retrieved. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The name of a Performance Insights metric to be measured.</p> <p>Valid values for <code>Metric</code> are:</p> <ul> <li> <p> <code>db.load.avg</code> - a scaled representation of the number of active sessions for the database engine.</p> </li> <li> <p> <code>db.sampledload.avg</code> - the raw number of active sessions for the database engine.</p> </li> </ul></p>
    #[serde(rename = "Metric")]
    pub metric: String,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>For each dimension specified in <code>GroupBy</code>, specify a secondary dimension to further subdivide the partition keys in the response.</p>
    #[serde(rename = "PartitionBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_by: Option<DimensionGroup>,
    /// <p>The granularity, in seconds, of the data points returned from Performance Insights. A period can be as short as one second, or as long as one day (86400 seconds). Valid values are:</p> <ul> <li> <p> <code>1</code> (one second)</p> </li> <li> <p> <code>60</code> (one minute)</p> </li> <li> <p> <code>300</code> (five minutes)</p> </li> <li> <p> <code>3600</code> (one hour)</p> </li> <li> <p> <code>86400</code> (twenty-four hours)</p> </li> </ul> <p>If you don't specify <code>PeriodInSeconds</code>, then Performance Insights will choose a value for you, with a goal of returning roughly 100-200 data points in the response.</p>
    #[serde(rename = "PeriodInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_in_seconds: Option<i64>,
    /// <p>The AWS service for which Performance Insights will return metrics. The only valid value for <i>ServiceType</i> is: <code>RDS</code> </p>
    #[serde(rename = "ServiceType")]
    pub service_type: String,
    /// <p>The date and time specifying the beginning of the requested time series data. You can't specify a <code>StartTime</code> that's earlier than 7 days ago. The value specified is <i>inclusive</i> - data points equal to or greater than <code>StartTime</code> will be returned.</p> <p>The value for <code>StartTime</code> must be earlier than the value for <code>EndTime</code>.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDimensionKeysResponse {
    /// <p>The end time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>.</p>
    #[serde(rename = "AlignedEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_end_time: Option<f64>,
    /// <p>The start time for the returned dimension keys, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>.</p>
    #[serde(rename = "AlignedStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_start_time: Option<f64>,
    /// <p>The dimension keys that were requested.</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<DimensionKeyDescription>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If <code>PartitionBy</code> was present in the request, <code>PartitionKeys</code> contains the breakdown of dimension keys by the specified partitions.</p>
    #[serde(rename = "PartitionKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<ResponsePartitionKey>>,
}

/// <p>A logical grouping of Performance Insights metrics for a related subject area. For example, the <code>db.sql</code> dimension group consists of the following dimensions: <code>db.sql.id</code>, <code>db.sql.db_id</code>, <code>db.sql.statement</code>, and <code>db.sql.tokenized_id</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DimensionGroup {
    /// <p><p>A list of specific dimensions from a dimension group. If this parameter is not present, then it signifies that all of the dimensions in the group were requested, or are present in the response.</p> <p>Valid values for elements in the <code>Dimensions</code> array are:</p> <ul> <li> <p>db.user.id</p> </li> <li> <p>db.user.name</p> </li> <li> <p>db.host.id</p> </li> <li> <p>db.host.name</p> </li> <li> <p>db.sql.id</p> </li> <li> <p>db.sql.db<em>id</p> </li> <li> <p>db.sql.statement</p> </li> <li> <p>db.sql.tokenized</em>id</p> </li> <li> <p>db.sql<em>tokenized.id</p> </li> <li> <p>db.sql</em>tokenized.db<em>id</p> </li> <li> <p>db.sql</em>tokenized.statement</p> </li> <li> <p>db.wait<em>event.name</p> </li> <li> <p>db.wait</em>event.type</p> </li> <li> <p>db.wait<em>event</em>type.name</p> </li> </ul></p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    /// <p><p>The name of the dimension group. Valid values are:</p> <ul> <li> <p> <code>db.user</code> </p> </li> <li> <p> <code>db.host</code> </p> </li> <li> <p> <code>db.sql</code> </p> </li> <li> <p> <code>db.sql<em>tokenized</code> </p> </li> <li> <p> <code>db.wait</em>event</code> </p> </li> <li> <p> <code>db.wait<em>event</em>type</code> </p> </li> </ul></p>
    #[serde(rename = "Group")]
    pub group: String,
    /// <p>The maximum number of items to fetch for this dimension group.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// <p>An array of descriptions and aggregated values for each dimension within a dimension group.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DimensionKeyDescription {
    /// <p>A map of name-value pairs for the dimensions in the group.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<::std::collections::HashMap<String, String>>,
    /// <p>If <code>PartitionBy</code> was specified, <code>PartitionKeys</code> contains the dimensions that were.</p>
    #[serde(rename = "Partitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<f64>>,
    /// <p>The aggregated metric value for the dimension(s), over the requested time range.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceMetricsRequest {
    /// <p>The date and time specifiying the end of the requested time series data. The value specified is <i>exclusive</i> - data points less than (but not equal to) <code>EndTime</code> will be returned.</p> <p>The value for <code>EndTime</code> must be later than the value for <code>StartTime</code>.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>An immutable, AWS Region-unique identifier for a data source. Performance Insights gathers metrics from this data source.</p> <p>To use an Amazon RDS instance as a data source, you specify its <code>DbiResourceId</code> value - for example: <code>db-FAIHNTYBKTGAUSUZQYPDS2GW4A</code> </p>
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// <p>The maximum number of items to return in the response. If more items exist than the specified <code>MaxRecords</code> value, a pagination token is included in the response so that the remaining results can be retrieved. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An array of one or more queries to perform. Each query must specify a Performance Insights metric, and can optionally specify aggregation and filtering criteria.</p>
    #[serde(rename = "MetricQueries")]
    pub metric_queries: Vec<MetricQuery>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The granularity, in seconds, of the data points returned from Performance Insights. A period can be as short as one second, or as long as one day (86400 seconds). Valid values are:</p> <ul> <li> <p> <code>1</code> (one second)</p> </li> <li> <p> <code>60</code> (one minute)</p> </li> <li> <p> <code>300</code> (five minutes)</p> </li> <li> <p> <code>3600</code> (one hour)</p> </li> <li> <p> <code>86400</code> (twenty-four hours)</p> </li> </ul> <p>If you don't specify <code>PeriodInSeconds</code>, then Performance Insights will choose a value for you, with a goal of returning roughly 100-200 data points in the response.</p>
    #[serde(rename = "PeriodInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_in_seconds: Option<i64>,
    /// <p>The AWS service for which Performance Insights will return metrics. The only valid value for <i>ServiceType</i> is: <code>RDS</code> </p>
    #[serde(rename = "ServiceType")]
    pub service_type: String,
    /// <p>The date and time specifying the beginning of the requested time series data. You can't specify a <code>StartTime</code> that's earlier than 7 days ago. The value specified is <i>inclusive</i> - data points equal to or greater than <code>StartTime</code> will be returned.</p> <p>The value for <code>StartTime</code> must be earlier than the value for <code>EndTime</code>.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetResourceMetricsResponse {
    /// <p>The end time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>.</p>
    #[serde(rename = "AlignedEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_end_time: Option<f64>,
    /// <p>The start time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>.</p>
    #[serde(rename = "AlignedStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_start_time: Option<f64>,
    /// <p>An immutable, AWS Region-unique identifier for a data source. Performance Insights gathers metrics from this data source.</p> <p>To use an Amazon RDS instance as a data source, you specify its <code>DbiResourceId</code> value - for example: <code>db-FAIHNTYBKTGAUSUZQYPDS2GW4A</code> </p>
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// <p>An array of metric results,, where each array element contains all of the data points for a particular dimension.</p>
    #[serde(rename = "MetricList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_list: Option<Vec<MetricKeyDataPoints>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A time-ordered series of data points, correpsonding to a dimension of a Performance Insights metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MetricKeyDataPoints {
    /// <p>An array of timestamp-value pairs, representing measurements over a period of time.</p>
    #[serde(rename = "DataPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Vec<DataPoint>>,
    /// <p>The dimension(s) to which the data points apply.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<ResponseResourceMetricKey>,
}

/// <p>A single query to be processed. You must provide the metric to query. If no other parameters are specified, Performance Insights returns all of the data points for that metric. You can optionally request that the data points be aggregated by dimension group ( <code>GroupBy</code>), and return only those data points that match your criteria (<code>Filter</code>).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MetricQuery {
    /// <p><p>One or more filters to apply in the request. Restrictions:</p> <ul> <li> <p>Any number of filters by the same dimension, as specified in the <code>GroupBy</code> parameter.</p> </li> <li> <p>A single filter for any other dimension in this dimension group.</p> </li> </ul></p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<::std::collections::HashMap<String, String>>,
    /// <p>A specification for how to aggregate the data points from a query result. You must specify a valid dimension group. Performance Insights will return all of the dimensions within that group, unless you provide the names of specific dimensions within that group. You can also request that Performance Insights return a limited number of values for a dimension.</p>
    #[serde(rename = "GroupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<DimensionGroup>,
    /// <p><p>The name of a Performance Insights metric to be measured.</p> <p>Valid values for <code>Metric</code> are:</p> <ul> <li> <p> <code>db.load.avg</code> - a scaled representation of the number of active sessions for the database engine.</p> </li> <li> <p> <code>db.sampledload.avg</code> - the raw number of active sessions for the database engine.</p> </li> </ul></p>
    #[serde(rename = "Metric")]
    pub metric: String,
}

/// <p>If <code>PartitionBy</code> was specified in a <code>DescribeDimensionKeys</code> request, the dimensions are returned in an array. Each element in the array specifies one dimension. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResponsePartitionKey {
    /// <p>A dimension map that contains the dimension(s) for this partition.</p>
    #[serde(rename = "Dimensions")]
    pub dimensions: ::std::collections::HashMap<String, String>,
}

/// <p>An object describing a Performance Insights metric and one or more dimensions for that metric.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResponseResourceMetricKey {
    /// <p>The valid dimensions for the metric.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>The name of a Performance Insights metric to be measured.</p> <p>Valid values for <code>Metric</code> are:</p> <ul> <li> <p> <code>db.load.avg</code> - a scaled representation of the number of active sessions for the database engine.</p> </li> <li> <p> <code>db.sampledload.avg</code> - the raw number of active sessions for the database engine.</p> </li> </ul></p>
    #[serde(rename = "Metric")]
    pub metric: String,
}

/// Errors returned by DescribeDimensionKeys
#[derive(Debug, PartialEq)]
pub enum DescribeDimensionKeysError {
    /// <p>The request failed due to an unknown error.</p>
    InternalServiceError(String),
    /// <p>One of the arguments provided is invalid for this request.</p>
    InvalidArgument(String),
    /// <p>The user is not authorized to perform this request.</p>
    NotAuthorized(String),
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

impl DescribeDimensionKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDimensionKeysError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServiceError" => {
                    return DescribeDimensionKeysError::InternalServiceError(String::from(
                        error_message,
                    ))
                }
                "InvalidArgumentException" => {
                    return DescribeDimensionKeysError::InvalidArgument(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return DescribeDimensionKeysError::NotAuthorized(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeDimensionKeysError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeDimensionKeysError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDimensionKeysError {
    fn from(err: serde_json::error::Error) -> DescribeDimensionKeysError {
        DescribeDimensionKeysError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDimensionKeysError {
    fn from(err: CredentialsError) -> DescribeDimensionKeysError {
        DescribeDimensionKeysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDimensionKeysError {
    fn from(err: HttpDispatchError) -> DescribeDimensionKeysError {
        DescribeDimensionKeysError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDimensionKeysError {
    fn from(err: io::Error) -> DescribeDimensionKeysError {
        DescribeDimensionKeysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDimensionKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDimensionKeysError {
    fn description(&self) -> &str {
        match *self {
            DescribeDimensionKeysError::InternalServiceError(ref cause) => cause,
            DescribeDimensionKeysError::InvalidArgument(ref cause) => cause,
            DescribeDimensionKeysError::NotAuthorized(ref cause) => cause,
            DescribeDimensionKeysError::Validation(ref cause) => cause,
            DescribeDimensionKeysError::Credentials(ref err) => err.description(),
            DescribeDimensionKeysError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDimensionKeysError::ParseError(ref cause) => cause,
            DescribeDimensionKeysError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetResourceMetrics
#[derive(Debug, PartialEq)]
pub enum GetResourceMetricsError {
    /// <p>The request failed due to an unknown error.</p>
    InternalServiceError(String),
    /// <p>One of the arguments provided is invalid for this request.</p>
    InvalidArgument(String),
    /// <p>The user is not authorized to perform this request.</p>
    NotAuthorized(String),
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

impl GetResourceMetricsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetResourceMetricsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServiceError" => {
                    return GetResourceMetricsError::InternalServiceError(String::from(
                        error_message,
                    ))
                }
                "InvalidArgumentException" => {
                    return GetResourceMetricsError::InvalidArgument(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return GetResourceMetricsError::NotAuthorized(String::from(error_message))
                }
                "ValidationException" => {
                    return GetResourceMetricsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetResourceMetricsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetResourceMetricsError {
    fn from(err: serde_json::error::Error) -> GetResourceMetricsError {
        GetResourceMetricsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetResourceMetricsError {
    fn from(err: CredentialsError) -> GetResourceMetricsError {
        GetResourceMetricsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetResourceMetricsError {
    fn from(err: HttpDispatchError) -> GetResourceMetricsError {
        GetResourceMetricsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetResourceMetricsError {
    fn from(err: io::Error) -> GetResourceMetricsError {
        GetResourceMetricsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetResourceMetricsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceMetricsError {
    fn description(&self) -> &str {
        match *self {
            GetResourceMetricsError::InternalServiceError(ref cause) => cause,
            GetResourceMetricsError::InvalidArgument(ref cause) => cause,
            GetResourceMetricsError::NotAuthorized(ref cause) => cause,
            GetResourceMetricsError::Validation(ref cause) => cause,
            GetResourceMetricsError::Credentials(ref err) => err.description(),
            GetResourceMetricsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetResourceMetricsError::ParseError(ref cause) => cause,
            GetResourceMetricsError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS PI API. AWS PI clients implement this trait.
pub trait PerformanceInsights {
    /// <p>For a specific time period, retrieve the top <code>N</code> dimension keys for a metric.</p>
    fn describe_dimension_keys(
        &self,
        input: DescribeDimensionKeysRequest,
    ) -> RusotoFuture<DescribeDimensionKeysResponse, DescribeDimensionKeysError>;

    /// <p>Retrieve Performance Insights metrics for a set of data sources, over a time period. You can provide specific dimension groups and dimensions, and provide aggregation and filtering criteria for each group.</p>
    fn get_resource_metrics(
        &self,
        input: GetResourceMetricsRequest,
    ) -> RusotoFuture<GetResourceMetricsResponse, GetResourceMetricsError>;
}
/// A client for the AWS PI API.
pub struct PerformanceInsightsClient {
    client: Client,
    region: region::Region,
}

impl PerformanceInsightsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PerformanceInsightsClient {
        PerformanceInsightsClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PerformanceInsightsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        PerformanceInsightsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl PerformanceInsights for PerformanceInsightsClient {
    /// <p>For a specific time period, retrieve the top <code>N</code> dimension keys for a metric.</p>
    fn describe_dimension_keys(
        &self,
        input: DescribeDimensionKeysRequest,
    ) -> RusotoFuture<DescribeDimensionKeysResponse, DescribeDimensionKeysError> {
        let mut request = SignedRequest::new("POST", "pi", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PerformanceInsightsv20180227.DescribeDimensionKeys",
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

                    serde_json::from_str::<DescribeDimensionKeysResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeDimensionKeysError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieve Performance Insights metrics for a set of data sources, over a time period. You can provide specific dimension groups and dimensions, and provide aggregation and filtering criteria for each group.</p>
    fn get_resource_metrics(
        &self,
        input: GetResourceMetricsRequest,
    ) -> RusotoFuture<GetResourceMetricsResponse, GetResourceMetricsError> {
        let mut request = SignedRequest::new("POST", "pi", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "PerformanceInsightsv20180227.GetResourceMetrics",
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

                    serde_json::from_str::<GetResourceMetricsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetResourceMetricsError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
