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

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Request of DeleteReportDefinition</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReportDefinitionRequest {
    #[serde(rename = "ReportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
}

/// <p>Response of DeleteReportDefinition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteReportDefinitionResponse {
    #[serde(rename = "ResponseMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message: Option<String>,
}

/// <p>Request of DescribeReportDefinitions</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeReportDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Response of DescribeReportDefinitions</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeReportDefinitionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReportDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_definitions: Option<Vec<ReportDefinition>>,
}

/// <p>Request of PutReportDefinition</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutReportDefinitionRequest {
    #[serde(rename = "ReportDefinition")]
    pub report_definition: ReportDefinition,
}

/// <p>Response of PutReportDefinition</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutReportDefinitionResponse {}

/// <p>The definition of AWS Cost and Usage Report. Customer can specify the report name, time unit, report format, compression format, S3 bucket and additional artifacts and schema elements in the definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportDefinition {
    #[serde(rename = "AdditionalArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_artifacts: Option<Vec<String>>,
    #[serde(rename = "AdditionalSchemaElements")]
    pub additional_schema_elements: Vec<String>,
    #[serde(rename = "Compression")]
    pub compression: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "ReportName")]
    pub report_name: String,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: String,
    #[serde(rename = "S3Region")]
    pub s3_region: String,
    #[serde(rename = "TimeUnit")]
    pub time_unit: String,
}

/// Errors returned by DeleteReportDefinition
#[derive(Debug, PartialEq)]
pub enum DeleteReportDefinitionError {
    /// <p>This exception is thrown on a known dependency failure.</p>
    InternalError(String),
}

impl DeleteReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReportDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteReportDefinitionError::InternalError(
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
impl fmt::Display for DeleteReportDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReportDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeleteReportDefinitionError::InternalError(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReportDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeReportDefinitionsError {
    /// <p>This exception is thrown on a known dependency failure.</p>
    InternalError(String),
}

impl DescribeReportDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReportDefinitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeReportDefinitionsError::InternalError(
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
impl fmt::Display for DescribeReportDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReportDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReportDefinitionsError::InternalError(ref cause) => cause,
        }
    }
}
/// Errors returned by PutReportDefinition
#[derive(Debug, PartialEq)]
pub enum PutReportDefinitionError {
    /// <p>This exception is thrown when putting a report preference with a name that already exists.</p>
    DuplicateReportName(String),
    /// <p>This exception is thrown on a known dependency failure.</p>
    InternalError(String),
    /// <p>This exception is thrown when the number of report preference reaches max limit. The max number is 5.</p>
    ReportLimitReached(String),
}

impl PutReportDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutReportDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateReportNameException" => {
                    return RusotoError::Service(PutReportDefinitionError::DuplicateReportName(
                        err.msg,
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(PutReportDefinitionError::InternalError(err.msg))
                }
                "ReportLimitReachedException" => {
                    return RusotoError::Service(PutReportDefinitionError::ReportLimitReached(
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
impl fmt::Display for PutReportDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutReportDefinitionError {
    fn description(&self) -> &str {
        match *self {
            PutReportDefinitionError::DuplicateReportName(ref cause) => cause,
            PutReportDefinitionError::InternalError(ref cause) => cause,
            PutReportDefinitionError::ReportLimitReached(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Cost and Usage Report Service API. AWS Cost and Usage Report Service clients implement this trait.
pub trait CostAndUsageReport {
    /// <p>Delete a specified report definition</p>
    fn delete_report_definition(
        &self,
        input: DeleteReportDefinitionRequest,
    ) -> RusotoFuture<DeleteReportDefinitionResponse, DeleteReportDefinitionError>;

    /// <p>Describe a list of report definitions owned by the account</p>
    fn describe_report_definitions(
        &self,
        input: DescribeReportDefinitionsRequest,
    ) -> RusotoFuture<DescribeReportDefinitionsResponse, DescribeReportDefinitionsError>;

    /// <p>Create a new report definition</p>
    fn put_report_definition(
        &self,
        input: PutReportDefinitionRequest,
    ) -> RusotoFuture<PutReportDefinitionResponse, PutReportDefinitionError>;
}
/// A client for the AWS Cost and Usage Report Service API.
#[derive(Clone)]
pub struct CostAndUsageReportClient {
    client: Client,
    region: region::Region,
}

impl CostAndUsageReportClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CostAndUsageReportClient {
        CostAndUsageReportClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CostAndUsageReportClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CostAndUsageReportClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CostAndUsageReport for CostAndUsageReportClient {
    /// <p>Delete a specified report definition</p>
    fn delete_report_definition(
        &self,
        input: DeleteReportDefinitionRequest,
    ) -> RusotoFuture<DeleteReportDefinitionResponse, DeleteReportDefinitionError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.DeleteReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteReportDefinitionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteReportDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describe a list of report definitions owned by the account</p>
    fn describe_report_definitions(
        &self,
        input: DescribeReportDefinitionsRequest,
    ) -> RusotoFuture<DescribeReportDefinitionsResponse, DescribeReportDefinitionsError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.DescribeReportDefinitions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeReportDefinitionsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReportDefinitionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Create a new report definition</p>
    fn put_report_definition(
        &self,
        input: PutReportDefinitionRequest,
    ) -> RusotoFuture<PutReportDefinitionResponse, PutReportDefinitionError> {
        let mut request = SignedRequest::new("POST", "cur", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSOrigamiServiceGatewayService.PutReportDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutReportDefinitionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutReportDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }
}
