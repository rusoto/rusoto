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
use rusoto_core::v2::{Dispatcher, Request, ServiceRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Column Metadata</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ColumnMetadata {
    /// <p>Homogenous array base SQL type from java.sql.Types.</p>
    #[serde(rename = "arrayBaseColumnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_base_column_type: Option<i64>,
    /// <p>Whether the designated column is automatically numbered</p>
    #[serde(rename = "isAutoIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_increment: Option<bool>,
    /// <p>Whether values in the designated column&#39;s case matters</p>
    #[serde(rename = "isCaseSensitive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_case_sensitive: Option<bool>,
    /// <p>Whether values in the designated column is a cash value</p>
    #[serde(rename = "isCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_currency: Option<bool>,
    /// <p>Whether values in the designated column are signed numbers</p>
    #[serde(rename = "isSigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_signed: Option<bool>,
    /// <p>Usually specified by the SQL AS. If not specified, return column name.</p>
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// <p>Name of the column.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Indicates the nullability of values in the designated column. One of columnNoNulls (0), columnNullable (1), columnNullableUnknown (2)</p>
    #[serde(rename = "nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<i64>,
    /// <p>Get the designated column&#39;s specified column size.For numeric data, this is the maximum precision.  For character data, this is the length in characters. For datetime datatypes, this is the length in characters of the String representation (assuming the maximum allowed precision of the fractional seconds component). For binary data, this is the length in bytes.  For the ROWID datatype, this is the length in bytes. 0 is returned for data types where the column size is not applicable.</p>
    #[serde(rename = "precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    /// <p>Designated column&#39;s number of digits to right of the decimal point. 0 is returned for data types where the scale is not applicable.</p>
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    /// <p>Designated column&#39;s table&#39;s schema</p>
    #[serde(rename = "schemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>Designated column&#39;s table name</p>
    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>SQL type from java.sql.Types.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    /// <p>Database-specific type name.</p>
    #[serde(rename = "typeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

/// <p>Execute SQL Request</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecuteSqlRequest {
    /// <p>ARN of the db credentials in AWS Secret Store or the friendly secret name</p>
    #[serde(rename = "awsSecretStoreArn")]
    pub aws_secret_store_arn: String,
    /// <p>Target DB name</p>
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// <p>ARN of the target db cluster or instance</p>
    #[serde(rename = "dbClusterOrInstanceArn")]
    pub db_cluster_or_instance_arn: String,
    /// <p>Target Schema name</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// <p>SQL statement(s) to be executed. Statements can be chained by using semicolons</p>
    #[serde(rename = "sqlStatements")]
    pub sql_statements: String,
}

/// <p>Execute SQL response</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExecuteSqlResponse {
    /// <p>Results returned by executing the sql statement(s)</p>
    #[serde(rename = "sqlStatementResults")]
    pub sql_statement_results: Vec<SqlStatementResult>,
}

/// <p>Row or Record</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Record {
    /// <p>Record</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Value>>,
}

/// <p>Result Frame</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResultFrame {
    /// <p>ResultSet Metadata.</p>
    #[serde(rename = "records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
    /// <p>ResultSet Metadata.</p>
    #[serde(rename = "resultSetMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set_metadata: Option<ResultSetMetadata>,
}

/// <p>List of columns and their types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResultSetMetadata {
    /// <p>Number of columns</p>
    #[serde(rename = "columnCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i64>,
    /// <p>List of columns and their types</p>
    #[serde(rename = "columnMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<ColumnMetadata>>,
}

/// <p>SQL statement execution result</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SqlStatementResult {
    /// <p>Number of rows updated.</p>
    #[serde(rename = "numberOfRecordsUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_records_updated: Option<i64>,
    /// <p>ResultFrame returned by executing the sql statement</p>
    #[serde(rename = "resultFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_frame: Option<ResultFrame>,
}

/// <p>User Defined Type</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StructValue {
    /// <p>Struct or UDT</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Value>>,
}

/// <p>Column value</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Value {
    /// <p>Arbitrarily nested arrays</p>
    #[serde(rename = "arrayValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_values: Option<Vec<Value>>,
    /// <p>Long value</p>
    #[serde(rename = "bigIntValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_int_value: Option<i64>,
    /// <p>Bit value</p>
    #[serde(rename = "bitValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_value: Option<bool>,
    /// <p>Blob value</p>
    #[serde(rename = "blobValue")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_value: Option<bytes::Bytes>,
    /// <p>Double value</p>
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    /// <p>Integer value</p>
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i64>,
    /// <p>Is column null</p>
    #[serde(rename = "isNull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_null: Option<bool>,
    /// <p>Float value</p>
    #[serde(rename = "realValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_value: Option<f32>,
    /// <p>String value</p>
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    /// <p>Struct or UDT</p>
    #[serde(rename = "structValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_value: Option<StructValue>,
}

/// Errors returned by ExecuteSql
#[derive(Debug, PartialEq)]
pub enum ExecuteSqlError {
    /// <p>Invalid Request exception</p>
    BadRequest(String),
    /// <p>Access denied exception</p>
    Forbidden(String),
    /// <p>Internal service error</p>
    InternalServerError(String),
    /// <p>Internal service unavailable error</p>
    ServiceUnavailableError(String),
}

impl ExecuteSqlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExecuteSqlError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ExecuteSqlError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ExecuteSqlError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ExecuteSqlError::InternalServerError(err.msg))
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(ExecuteSqlError::ServiceUnavailableError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ExecuteSqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExecuteSqlError {
    fn description(&self) -> &str {
        match *self {
            ExecuteSqlError::BadRequest(ref cause) => cause,
            ExecuteSqlError::Forbidden(ref cause) => cause,
            ExecuteSqlError::InternalServerError(ref cause) => cause,
            ExecuteSqlError::ServiceUnavailableError(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS RDS DataService API. AWS RDS DataService clients implement this trait.
pub trait RdsData {
    /// <p>Executes any SQL statement on the target database synchronously</p>
    fn execute_sql(&self, input: ExecuteSqlRequest) -> Request<ExecuteSqlRequest>;
}
/// A client for the AWS RDS DataService API.
#[derive(Clone)]
pub struct RdsDataClient {
    client: Client,
    region: region::Region,
}

impl RdsDataClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> RdsDataClient {
        RdsDataClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> RdsDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        RdsDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl RdsData for RdsDataClient {
    /// <p>Executes any SQL statement on the target database synchronously</p>
    fn execute_sql(&self, input: ExecuteSqlRequest) -> Request<ExecuteSqlRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for ExecuteSqlRequest {
    type Output = ExecuteSqlResponse;
    type Error = ExecuteSqlError;

    #[allow(unused_variables, warnings)]
    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let request_uri = "/ExecuteSql";

        let mut request = SignedRequest::new("POST", "rds-data", region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&self).unwrap());
        request.set_payload(encoded);

        dispatcher.dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ExecuteSqlResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ExecuteSqlError::from_response(response))),
                )
            }
        })
    }
}
