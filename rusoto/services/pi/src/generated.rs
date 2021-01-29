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

impl PerformanceInsightsClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "pi", &self.region, request_uri);

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
/// <p>A timestamp, and a single numerical value, which together represent a measurement at a particular point in time.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DataPoint {
    /// <p>The time, in epoch format, associated with a particular <code>Value</code>.</p>
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
    /// <p>The actual value associated with a particular <code>Timestamp</code>.</p>
    #[serde(rename = "Value")]
    pub value: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDimensionKeysRequest {
    /// <p>The date and time specifying the end of the requested time series data. The value specified is <i>exclusive</i>, which means that data points less than (but not equal to) <code>EndTime</code> are returned.</p> <p>The value for <code>EndTime</code> must be later than the value for <code>StartTime</code>.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p><p>One or more filters to apply in the request. Restrictions:</p> <ul> <li> <p>Any number of filters by the same dimension, as specified in the <code>GroupBy</code> or <code>Partition</code> parameters.</p> </li> <li> <p>A single filter for any other dimension in this dimension group.</p> </li> </ul></p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<::std::collections::HashMap<String, String>>,
    /// <p>A specification for how to aggregate the data points from a query result. You must specify a valid dimension group. Performance Insights returns all dimensions within this group, unless you provide the names of specific dimensions within this group. You can also request that Performance Insights return a limited number of values for a dimension.</p>
    #[serde(rename = "GroupBy")]
    pub group_by: DimensionGroup,
    /// <p>An immutable, AWS Region-unique identifier for a data source. Performance Insights gathers metrics from this data source.</p> <p>To use an Amazon RDS instance as a data source, you specify its <code>DbiResourceId</code> value. For example, specify <code>db-FAIHNTYBKTGAUSUZQYPDS2GW4A</code> </p>
    #[serde(rename = "Identifier")]
    pub identifier: String,
    /// <p>The maximum number of items to return in the response. If more items exist than the specified <code>MaxRecords</code> value, a pagination token is included in the response so that the remaining results can be retrieved. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of a Performance Insights metric to be measured.</p> <p>Valid values for <code>Metric</code> are:</p> <ul> <li> <p> <code>db.load.avg</code> - a scaled representation of the number of active sessions for the database engine.</p> </li> <li> <p> <code>db.sampledload.avg</code> - the raw number of active sessions for the database engine.</p> </li> </ul> <p>If the number of active sessions is less than an internal Performance Insights threshold, <code>db.load.avg</code> and <code>db.sampledload.avg</code> are the same value. If the number of active sessions is greater than the internal threshold, Performance Insights samples the active sessions, with <code>db.load.avg</code> showing the scaled values, <code>db.sampledload.avg</code> showing the raw values, and <code>db.sampledload.avg</code> less than <code>db.load.avg</code>. For most use cases, you can query <code>db.load.avg</code> only. </p>
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
    /// <p>The granularity, in seconds, of the data points returned from Performance Insights. A period can be as short as one second, or as long as one day (86400 seconds). Valid values are:</p> <ul> <li> <p> <code>1</code> (one second)</p> </li> <li> <p> <code>60</code> (one minute)</p> </li> <li> <p> <code>300</code> (five minutes)</p> </li> <li> <p> <code>3600</code> (one hour)</p> </li> <li> <p> <code>86400</code> (twenty-four hours)</p> </li> </ul> <p>If you don't specify <code>PeriodInSeconds</code>, then Performance Insights chooses a value for you, with a goal of returning roughly 100-200 data points in the response.</p>
    #[serde(rename = "PeriodInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_in_seconds: Option<i64>,
    /// <p>The AWS service for which Performance Insights will return metrics. The only valid value for <i>ServiceType</i> is <code>RDS</code>.</p>
    #[serde(rename = "ServiceType")]
    pub service_type: String,
    /// <p>The date and time specifying the beginning of the requested time series data. You must specify a <code>StartTime</code> within the past 7 days. The value specified is <i>inclusive</i>, which means that data points equal to or greater than <code>StartTime</code> are returned.</p> <p>The value for <code>StartTime</code> must be earlier than the value for <code>EndTime</code>.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p><p>A logical grouping of Performance Insights metrics for a related subject area. For example, the <code>db.sql</code> dimension group consists of the following dimensions: <code>db.sql.id</code>, <code>db.sql.db<em>id</code>, <code>db.sql.statement</code>, and <code>db.sql.tokenized</em>id</code>.</p> <note> <p>Each response element returns a maximum of 500 bytes. For larger elements, such as SQL statements, only the first 500 bytes are returned.</p> </note></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DimensionGroup {
    /// <p><p>A list of specific dimensions from a dimension group. If this parameter is not present, then it signifies that all of the dimensions in the group were requested, or are present in the response.</p> <p>Valid values for elements in the <code>Dimensions</code> array are:</p> <ul> <li> <p> <code>db.application.name</code> - The name of the application that is connected to the database (only Aurora PostgreSQL and RDS PostgreSQL)</p> </li> <li> <p> <code>db.host.id</code> - The host ID of the connected client (all engines)</p> </li> <li> <p> <code>db.host.name</code> - The host name of the connected client (all engines)</p> </li> <li> <p> <code>db.name</code> - The name of the database to which the client is connected (only Aurora PostgreSQL, RDS PostgreSQL, Aurora MySQL, RDS MySQL, and MariaDB)</p> </li> <li> <p> <code>db.session<em>type.name</code> - The type of the current session (only Aurora PostgreSQL and RDS PostgreSQL)</p> </li> <li> <p> <code>db.sql.id</code> - The SQL ID generated by Performance Insights (all engines)</p> </li> <li> <p> <code>db.sql.db</em>id</code> - The SQL ID generated by the database (all engines)</p> </li> <li> <p> <code>db.sql.statement</code> - The SQL text that is being executed (all engines)</p> </li> <li> <p> <code>db.sql.tokenized<em>id</code> </p> </li> <li> <p> <code>db.sql</em>tokenized.id</code> - The SQL digest ID generated by Performance Insights (all engines)</p> </li> <li> <p> <code>db.sql<em>tokenized.db</em>id</code> - SQL digest ID generated by the database (all engines)</p> </li> <li> <p> <code>db.sql<em>tokenized.statement</code> - The SQL digest text (all engines)</p> </li> <li> <p> <code>db.user.id</code> - The ID of the user logged in to the database (all engines)</p> </li> <li> <p> <code>db.user.name</code> - The name of the user logged in to the database (all engines)</p> </li> <li> <p> <code>db.wait</em>event.name</code> - The event for which the backend is waiting (all engines)</p> </li> <li> <p> <code>db.wait<em>event.type</code> - The type of event for which the backend is waiting (all engines)</p> </li> <li> <p> <code>db.wait</em>event_type.name</code> - The name of the event type for which the backend is waiting (all engines)</p> </li> </ul></p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    /// <p><p>The name of the dimension group. Valid values are:</p> <ul> <li> <p> <code>db</code> - The name of the database to which the client is connected (only Aurora PostgreSQL, RDS PostgreSQL, Aurora MySQL, RDS MySQL, and MariaDB)</p> </li> <li> <p> <code>db.application</code> - The name of the application that is connected to the database (only Aurora PostgreSQL and RDS PostgreSQL)</p> </li> <li> <p> <code>db.host</code> - The host name of the connected client (all engines)</p> </li> <li> <p> <code>db.session<em>type</code> - The type of the current session (only Aurora PostgreSQL and RDS PostgreSQL)</p> </li> <li> <p> <code>db.sql</code> - The SQL that is currently executing (all engines)</p> </li> <li> <p> <code>db.sql</em>tokenized</code> - The SQL digest (all engines)</p> </li> <li> <p> <code>db.wait<em>event</code> - The event for which the database backend is waiting (all engines)</p> </li> <li> <p> <code>db.wait</em>event_type</code> - The type of event for which the database backend is waiting (all engines)</p> </li> <li> <p> <code>db.user</code> - The user logged in to the database (all engines)</p> </li> </ul></p>
    #[serde(rename = "Group")]
    pub group: String,
    /// <p>The maximum number of items to fetch for this dimension group.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// <p>An array of descriptions and aggregated values for each dimension within a dimension group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourceMetricsRequest {
    /// <p>The date and time specifying the end of the requested time series data. The value specified is <i>exclusive</i> - data points less than (but not equal to) <code>EndTime</code> will be returned.</p> <p>The value for <code>EndTime</code> must be later than the value for <code>StartTime</code>.</p>
    #[serde(rename = "EndTime")]
    pub end_time: f64,
    /// <p>An immutable, AWS Region-unique identifier for a data source. Performance Insights gathers metrics from this data source.</p> <p>To use a DB instance as a data source, specify its <code>DbiResourceId</code> value. For example, specify <code>db-FAIHNTYBKTGAUSUZQYPDS2GW4A</code>.</p>
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
    /// <p>The AWS service for which Performance Insights returns metrics. The only valid value for <i>ServiceType</i> is <code>RDS</code>.</p>
    #[serde(rename = "ServiceType")]
    pub service_type: String,
    /// <p>The date and time specifying the beginning of the requested time series data. You can't specify a <code>StartTime</code> that's earlier than 7 days ago. The value specified is <i>inclusive</i> - data points equal to or greater than <code>StartTime</code> will be returned.</p> <p>The value for <code>StartTime</code> must be earlier than the value for <code>EndTime</code>.</p>
    #[serde(rename = "StartTime")]
    pub start_time: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourceMetricsResponse {
    /// <p>The end time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedEndTime</code> will be greater than or equal to the value of the user-specified <code>Endtime</code>.</p>
    #[serde(rename = "AlignedEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_end_time: Option<f64>,
    /// <p>The start time for the returned metrics, after alignment to a granular boundary (as specified by <code>PeriodInSeconds</code>). <code>AlignedStartTime</code> will be less than or equal to the value of the user-specified <code>StartTime</code>.</p>
    #[serde(rename = "AlignedStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_start_time: Option<f64>,
    /// <p>An immutable, AWS Region-unique identifier for a data source. Performance Insights gathers metrics from this data source.</p> <p>To use a DB instance as a data source, you specify its <code>DbiResourceId</code> value - for example: <code>db-FAIHNTYBKTGAUSUZQYPDS2GW4A</code> </p>
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

/// <p>A time-ordered series of data points, corresponding to a dimension of a Performance Insights metric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MetricQuery {
    /// <p><p>One or more filters to apply in the request. Restrictions:</p> <ul> <li> <p>Any number of filters by the same dimension, as specified in the <code>GroupBy</code> parameter.</p> </li> <li> <p>A single filter for any other dimension in this dimension group.</p> </li> </ul></p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<::std::collections::HashMap<String, String>>,
    /// <p>A specification for how to aggregate the data points from a query result. You must specify a valid dimension group. Performance Insights will return all of the dimensions within that group, unless you provide the names of specific dimensions within that group. You can also request that Performance Insights return a limited number of values for a dimension.</p>
    #[serde(rename = "GroupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<DimensionGroup>,
    /// <p>The name of a Performance Insights metric to be measured.</p> <p>Valid values for <code>Metric</code> are:</p> <ul> <li> <p> <code>db.load.avg</code> - a scaled representation of the number of active sessions for the database engine.</p> </li> <li> <p> <code>db.sampledload.avg</code> - the raw number of active sessions for the database engine.</p> </li> </ul> <p>If the number of active sessions is less than an internal Performance Insights threshold, <code>db.load.avg</code> and <code>db.sampledload.avg</code> are the same value. If the number of active sessions is greater than the internal threshold, Performance Insights samples the active sessions, with <code>db.load.avg</code> showing the scaled values, <code>db.sampledload.avg</code> showing the raw values, and <code>db.sampledload.avg</code> less than <code>db.load.avg</code>. For most use cases, you can query <code>db.load.avg</code> only. </p>
    #[serde(rename = "Metric")]
    pub metric: String,
}

/// <p>If <code>PartitionBy</code> was specified in a <code>DescribeDimensionKeys</code> request, the dimensions are returned in an array. Each element in the array specifies one dimension. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResponsePartitionKey {
    /// <p>A dimension map that contains the dimension(s) for this partition.</p>
    #[serde(rename = "Dimensions")]
    pub dimensions: ::std::collections::HashMap<String, String>,
}

/// <p>An object describing a Performance Insights metric and one or more dimensions for that metric.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResponseResourceMetricKey {
    /// <p>The valid dimensions for the metric.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of a Performance Insights metric to be measured.</p> <p>Valid values for <code>Metric</code> are:</p> <ul> <li> <p> <code>db.load.avg</code> - a scaled representation of the number of active sessions for the database engine.</p> </li> <li> <p> <code>db.sampledload.avg</code> - the raw number of active sessions for the database engine.</p> </li> </ul> <p>If the number of active sessions is less than an internal Performance Insights threshold, <code>db.load.avg</code> and <code>db.sampledload.avg</code> are the same value. If the number of active sessions is greater than the internal threshold, Performance Insights samples the active sessions, with <code>db.load.avg</code> showing the scaled values, <code>db.sampledload.avg</code> showing the raw values, and <code>db.sampledload.avg</code> less than <code>db.load.avg</code>. For most use cases, you can query <code>db.load.avg</code> only. </p>
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
}

impl DescribeDimensionKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDimensionKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(DescribeDimensionKeysError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeDimensionKeysError::InvalidArgument(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeDimensionKeysError::NotAuthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeDimensionKeysError> {
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
impl fmt::Display for DescribeDimensionKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDimensionKeysError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            DescribeDimensionKeysError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeDimensionKeysError::NotAuthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDimensionKeysError {}
/// Errors returned by GetResourceMetrics
#[derive(Debug, PartialEq)]
pub enum GetResourceMetricsError {
    /// <p>The request failed due to an unknown error.</p>
    InternalServiceError(String),
    /// <p>One of the arguments provided is invalid for this request.</p>
    InvalidArgument(String),
    /// <p>The user is not authorized to perform this request.</p>
    NotAuthorized(String),
}

impl GetResourceMetricsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourceMetricsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceError" => {
                    return RusotoError::Service(GetResourceMetricsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(GetResourceMetricsError::InvalidArgument(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetResourceMetricsError::NotAuthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<GetResourceMetricsError> {
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
impl fmt::Display for GetResourceMetricsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourceMetricsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetResourceMetricsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            GetResourceMetricsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourceMetricsError {}
/// Trait representing the capabilities of the AWS PI API. AWS PI clients implement this trait.
#[async_trait]
pub trait PerformanceInsights {
    /// <p><p>For a specific time period, retrieve the top <code>N</code> dimension keys for a metric.</p> <note> <p>Each response element returns a maximum of 500 bytes. For larger elements, such as SQL statements, only the first 500 bytes are returned.</p> </note></p>
    async fn describe_dimension_keys(
        &self,
        input: DescribeDimensionKeysRequest,
    ) -> Result<DescribeDimensionKeysResponse, RusotoError<DescribeDimensionKeysError>>;

    /// <p><p>Retrieve Performance Insights metrics for a set of data sources, over a time period. You can provide specific dimension groups and dimensions, and provide aggregation and filtering criteria for each group.</p> <note> <p>Each response element returns a maximum of 500 bytes. For larger elements, such as SQL statements, only the first 500 bytes are returned.</p> </note></p>
    async fn get_resource_metrics(
        &self,
        input: GetResourceMetricsRequest,
    ) -> Result<GetResourceMetricsResponse, RusotoError<GetResourceMetricsError>>;
}
/// A client for the AWS PI API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PerformanceInsightsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PerformanceInsightsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PerformanceInsightsClient {
        PerformanceInsightsClient { client, region }
    }
}

#[async_trait]
impl PerformanceInsights for PerformanceInsightsClient {
    /// <p><p>For a specific time period, retrieve the top <code>N</code> dimension keys for a metric.</p> <note> <p>Each response element returns a maximum of 500 bytes. For larger elements, such as SQL statements, only the first 500 bytes are returned.</p> </note></p>
    async fn describe_dimension_keys(
        &self,
        input: DescribeDimensionKeysRequest,
    ) -> Result<DescribeDimensionKeysResponse, RusotoError<DescribeDimensionKeysError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PerformanceInsightsv20180227.DescribeDimensionKeys",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeDimensionKeysError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeDimensionKeysResponse, _>()
    }

    /// <p><p>Retrieve Performance Insights metrics for a set of data sources, over a time period. You can provide specific dimension groups and dimensions, and provide aggregation and filtering criteria for each group.</p> <note> <p>Each response element returns a maximum of 500 bytes. For larger elements, such as SQL statements, only the first 500 bytes are returned.</p> </note></p>
    async fn get_resource_metrics(
        &self,
        input: GetResourceMetricsRequest,
    ) -> Result<GetResourceMetricsResponse, RusotoError<GetResourceMetricsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "PerformanceInsightsv20180227.GetResourceMetrics",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetResourceMetricsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetResourceMetricsResponse, _>()
    }
}
