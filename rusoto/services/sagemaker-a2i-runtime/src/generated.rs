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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// see [SagemakerA2iRuntime::delete_human_loop]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteHumanLoopRequest {
    /// <p>The name of the human loop that you want to delete.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

/// see [SagemakerA2iRuntime::delete_human_loop]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteHumanLoopResponse {}

/// see [SagemakerA2iRuntime::describe_human_loop]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHumanLoopRequest {
    /// <p>The name of the human loop that you want information about.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

/// see [SagemakerA2iRuntime::describe_human_loop]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHumanLoopResponse {
    /// <p>The creation time when Amazon Augmented AI created the human loop.</p>
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    /// <p>A failure code that identifies the type of failure.</p>
    #[serde(rename = "FailureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason why a human loop failed. The failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    #[serde(rename = "HumanLoopArn")]
    pub human_loop_arn: String,
    /// <p>The name of the human loop. The name must be lowercase, unique within the Region in your account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
    /// <p>An object that contains information about the output of the human loop.</p>
    #[serde(rename = "HumanLoopOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_output: Option<HumanLoopOutput>,
    /// <p>The status of the human loop. </p>
    #[serde(rename = "HumanLoopStatus")]
    pub human_loop_status: String,
}

/// <p>Attributes of the data specified by the customer. Use these to describe the data to be labeled.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HumanLoopDataAttributes {
    /// <p>Declares that your content is free of personally identifiable information or adult content.</p> <p>Amazon SageMaker can restrict the Amazon Mechanical Turk workers who can view your task based on this information.</p>
    #[serde(rename = "ContentClassifiers")]
    pub content_classifiers: Vec<String>,
}

/// <p>An object containing the human loop input in JSON format.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HumanLoopInput {
    /// <p>Serialized input from the human loop. The input must be a string representation of a file in JSON format.</p>
    #[serde(rename = "InputContent")]
    pub input_content: String,
}

/// <p>Information about where the human output will be stored.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HumanLoopOutput {
    /// <p>The location of the Amazon S3 object where Amazon Augmented AI stores your human loop output.</p>
    #[serde(rename = "OutputS3Uri")]
    pub output_s3_uri: String,
}

/// <p>Summary information about the human loop.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HumanLoopSummary {
    /// <p>When Amazon Augmented AI created the human loop.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The reason why the human loop failed. A failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition used to configure the human loop.</p>
    #[serde(rename = "FlowDefinitionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_definition_arn: Option<String>,
    /// <p>The name of the human loop.</p>
    #[serde(rename = "HumanLoopName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_name: Option<String>,
    /// <p>The status of the human loop. </p>
    #[serde(rename = "HumanLoopStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_status: Option<String>,
}

/// see [SagemakerA2iRuntime::list_human_loops]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHumanLoopsRequest {
    /// <p>(Optional) The timestamp of the date when you want the human loops to begin in ISO 8601 format. For example, <code>2020-02-24</code>.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>(Optional) The timestamp of the date before which you want the human loops to begin in ISO 8601 format. For example, <code>2020-02-24</code>.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of a flow definition.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
    /// <p>The total number of items to return. If the total number of available items is more than the value specified in <code>MaxResults</code>, then a <code>NextToken</code> is returned in the output. You can use this token to display the next page of results. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to display the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional. The order for displaying results. Valid values: <code>Ascending</code> and <code>Descending</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl Paged for ListHumanLoopsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_token)
    }
}

impl PagedRequest for ListHumanLoopsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.next_token = key;
    }
}

/// see [SagemakerA2iRuntime::list_human_loops]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHumanLoopsResponse {
    /// <p>An array of objects that contain information about the human loops.</p>
    #[serde(rename = "HumanLoopSummaries")]
    pub human_loop_summaries: Vec<HumanLoopSummary>,
    /// <p>A token to display the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl Paged for ListHumanLoopsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_token.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_token)
    }
}

impl PagedOutput for ListHumanLoopsResponse {
    type Item = HumanLoopSummary;

    fn into_pagination_page(self) -> Vec<HumanLoopSummary> {
        self.human_loop_summaries
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [SagemakerA2iRuntime::start_human_loop]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartHumanLoopRequest {
    /// <p>Attributes of the specified data. Use <code>DataAttributes</code> to specify if your data is free of personally identifiable information and/or free of adult content.</p>
    #[serde(rename = "DataAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HumanLoopDataAttributes>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition associated with this human loop.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
    /// <p>An object that contains information about the human loop.</p>
    #[serde(rename = "HumanLoopInput")]
    pub human_loop_input: HumanLoopInput,
    /// <p>The name of the human loop.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

/// see [SagemakerA2iRuntime::start_human_loop]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartHumanLoopResponse {
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    #[serde(rename = "HumanLoopArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_arn: Option<String>,
}

/// see [SagemakerA2iRuntime::stop_human_loop]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopHumanLoopRequest {
    /// <p>The name of the human loop that you want to stop.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

/// see [SagemakerA2iRuntime::stop_human_loop]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopHumanLoopResponse {}

/// Errors returned by DeleteHumanLoop
#[derive(Debug, PartialEq)]
pub enum DeleteHumanLoopError {
    /// <p>We couldn't process your request because of an issue with the server. Try again later.</p>
    InternalServer(String),
    /// <p>We couldn't find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>You exceeded the maximum number of requests.</p>
    Throttling(String),
}

impl DeleteHumanLoopError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHumanLoopError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteHumanLoopError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteHumanLoopError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteHumanLoopError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteHumanLoopError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteHumanLoopError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteHumanLoopError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteHumanLoopError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteHumanLoopError {}
/// Errors returned by DescribeHumanLoop
#[derive(Debug, PartialEq)]
pub enum DescribeHumanLoopError {
    /// <p>We couldn't process your request because of an issue with the server. Try again later.</p>
    InternalServer(String),
    /// <p>We couldn't find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>You exceeded the maximum number of requests.</p>
    Throttling(String),
}

impl DescribeHumanLoopError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHumanLoopError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeHumanLoopError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeHumanLoopError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeHumanLoopError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeHumanLoopError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHumanLoopError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeHumanLoopError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeHumanLoopError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeHumanLoopError {}
/// Errors returned by ListHumanLoops
#[derive(Debug, PartialEq)]
pub enum ListHumanLoopsError {
    /// <p>We couldn't process your request because of an issue with the server. Try again later.</p>
    InternalServer(String),
    /// <p>We couldn't find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>You exceeded the maximum number of requests.</p>
    Throttling(String),
}

impl ListHumanLoopsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHumanLoopsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListHumanLoopsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListHumanLoopsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListHumanLoopsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListHumanLoopsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHumanLoopsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListHumanLoopsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListHumanLoopsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHumanLoopsError {}
/// Errors returned by StartHumanLoop
#[derive(Debug, PartialEq)]
pub enum StartHumanLoopError {
    /// <p>Your request has the same name as another active human loop but has different input data. You cannot start two human loops with the same name and different input data.</p>
    Conflict(String),
    /// <p>We couldn't process your request because of an issue with the server. Try again later.</p>
    InternalServer(String),
    /// <p>You exceeded your service quota. Delete some resources or request an increase in your service quota.</p>
    ServiceQuotaExceeded(String),
    /// <p>You exceeded the maximum number of requests.</p>
    Throttling(String),
}

impl StartHumanLoopError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartHumanLoopError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(StartHumanLoopError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartHumanLoopError::InternalServer(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(StartHumanLoopError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartHumanLoopError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartHumanLoopError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartHumanLoopError::Conflict(ref cause) => write!(f, "{}", cause),
            StartHumanLoopError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartHumanLoopError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            StartHumanLoopError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartHumanLoopError {}
/// Errors returned by StopHumanLoop
#[derive(Debug, PartialEq)]
pub enum StopHumanLoopError {
    /// <p>We couldn't process your request because of an issue with the server. Try again later.</p>
    InternalServer(String),
    /// <p>We couldn't find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>You exceeded the maximum number of requests.</p>
    Throttling(String),
}

impl StopHumanLoopError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopHumanLoopError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopHumanLoopError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopHumanLoopError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StopHumanLoopError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopHumanLoopError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopHumanLoopError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopHumanLoopError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopHumanLoopError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopHumanLoopError {}
/// Trait representing the capabilities of the Amazon Augmented AI Runtime API. Amazon Augmented AI Runtime clients implement this trait.
#[async_trait]
pub trait SagemakerA2iRuntime: Clone + Sync + Send + 'static {
    /// <p>Deletes the specified human loop for a flow definition.</p>
    async fn delete_human_loop(
        &self,
        input: DeleteHumanLoopRequest,
    ) -> Result<DeleteHumanLoopResponse, RusotoError<DeleteHumanLoopError>>;

    /// <p>Returns information about the specified human loop.</p>
    async fn describe_human_loop(
        &self,
        input: DescribeHumanLoopRequest,
    ) -> Result<DescribeHumanLoopResponse, RusotoError<DescribeHumanLoopError>>;

    /// <p>Returns information about human loops, given the specified parameters. If a human loop was deleted, it will not be included.</p>
    async fn list_human_loops(
        &self,
        input: ListHumanLoopsRequest,
    ) -> Result<ListHumanLoopsResponse, RusotoError<ListHumanLoopsError>>;

    /// Auto-paginating version of `list_human_loops`
    fn list_human_loops_pages<'a>(
        &'a self,
        mut input: ListHumanLoopsRequest,
    ) -> RusotoStream<'a, HumanLoopSummary, ListHumanLoopsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_human_loops(input.clone())
        }))
    }

    /// <p>Starts a human loop, provided that at least one activation condition is met.</p>
    async fn start_human_loop(
        &self,
        input: StartHumanLoopRequest,
    ) -> Result<StartHumanLoopResponse, RusotoError<StartHumanLoopError>>;

    /// <p>Stops the specified human loop.</p>
    async fn stop_human_loop(
        &self,
        input: StopHumanLoopRequest,
    ) -> Result<StopHumanLoopResponse, RusotoError<StopHumanLoopError>>;
}
/// A client for the Amazon Augmented AI Runtime API.
#[derive(Clone)]
pub struct SagemakerA2iRuntimeClient {
    client: Client,
    region: region::Region,
}

impl SagemakerA2iRuntimeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SagemakerA2iRuntimeClient {
        SagemakerA2iRuntimeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SagemakerA2iRuntimeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SagemakerA2iRuntimeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SagemakerA2iRuntimeClient {
        SagemakerA2iRuntimeClient { client, region }
    }
}

#[async_trait]
impl SagemakerA2iRuntime for SagemakerA2iRuntimeClient {
    /// <p>Deletes the specified human loop for a flow definition.</p>
    #[allow(unused_mut)]
    async fn delete_human_loop(
        &self,
        input: DeleteHumanLoopRequest,
    ) -> Result<DeleteHumanLoopResponse, RusotoError<DeleteHumanLoopError>> {
        let request_uri = format!(
            "/human-loops/{human_loop_name}",
            human_loop_name = input.human_loop_name
        );

        let mut request = SignedRequest::new("DELETE", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteHumanLoopResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteHumanLoopError::from_response(response))
        }
    }

    /// <p>Returns information about the specified human loop.</p>
    #[allow(unused_mut)]
    async fn describe_human_loop(
        &self,
        input: DescribeHumanLoopRequest,
    ) -> Result<DescribeHumanLoopResponse, RusotoError<DescribeHumanLoopError>> {
        let request_uri = format!(
            "/human-loops/{human_loop_name}",
            human_loop_name = input.human_loop_name
        );

        let mut request = SignedRequest::new("GET", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeHumanLoopResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHumanLoopError::from_response(response))
        }
    }

    /// <p>Returns information about human loops, given the specified parameters. If a human loop was deleted, it will not be included.</p>
    #[allow(unused_mut)]
    async fn list_human_loops(
        &self,
        input: ListHumanLoopsRequest,
    ) -> Result<ListHumanLoopsResponse, RusotoError<ListHumanLoopsError>> {
        let request_uri = "/human-loops";

        let mut request = SignedRequest::new("GET", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.creation_time_after {
            params.put("CreationTimeAfter", x);
        }
        if let Some(ref x) = input.creation_time_before {
            params.put("CreationTimeBefore", x);
        }
        params.put("FlowDefinitionArn", &input.flow_definition_arn);
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.sort_order {
            params.put("SortOrder", x);
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
                .deserialize::<ListHumanLoopsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListHumanLoopsError::from_response(response))
        }
    }

    /// <p>Starts a human loop, provided that at least one activation condition is met.</p>
    #[allow(unused_mut)]
    async fn start_human_loop(
        &self,
        input: StartHumanLoopRequest,
    ) -> Result<StartHumanLoopResponse, RusotoError<StartHumanLoopError>> {
        let request_uri = "/human-loops";

        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());
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
                .deserialize::<StartHumanLoopResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartHumanLoopError::from_response(response))
        }
    }

    /// <p>Stops the specified human loop.</p>
    #[allow(unused_mut)]
    async fn stop_human_loop(
        &self,
        input: StopHumanLoopRequest,
    ) -> Result<StopHumanLoopResponse, RusotoError<StopHumanLoopError>> {
        let request_uri = "/human-loops/stop";

        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());
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
                .deserialize::<StopHumanLoopResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopHumanLoopError::from_response(response))
        }
    }
}
