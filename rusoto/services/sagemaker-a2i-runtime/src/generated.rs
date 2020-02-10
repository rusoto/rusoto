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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use futures::prelude::*;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteHumanLoopRequest {
    /// <p>The name of the human loop you want to delete.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteHumanLoopResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHumanLoopRequest {
    /// <p>The name of the human loop.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHumanLoopResponse {
    /// <p>The timestamp when Amazon Augmented AI created the human loop.</p>
    #[serde(rename = "CreationTimestamp")]
    pub creation_timestamp: f64,
    /// <p>A failure code denoting a specific type of failure.</p>
    #[serde(rename = "FailureCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// <p>The reason why a human loop has failed. The failure reason is returned when the human loop status is <code>Failed</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    #[serde(rename = "HumanLoopArn")]
    pub human_loop_arn: String,
    /// <p>An object containing information about the human loop input.</p>
    #[serde(rename = "HumanLoopInput")]
    pub human_loop_input: HumanLoopInputContent,
    /// <p>The name of the human loop.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
    /// <p>An object containing information about the output of the human loop.</p>
    #[serde(rename = "HumanLoopOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_output: Option<HumanLoopOutputContent>,
    /// <p>The status of the human loop. Valid values:</p>
    #[serde(rename = "HumanLoopStatus")]
    pub human_loop_status: String,
}

/// <p>Contains information about why a human loop was triggered. If at least one activation reason is evaluated to be true, the human loop is activated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HumanLoopActivationReason {
    /// <p>True if the specified conditions were matched to trigger the human loop.</p>
    #[serde(rename = "ConditionsMatched")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions_matched: Option<bool>,
}

/// <p>Information about the corresponding flow definition's human loop activation condition evaluation. Null if <code>StartHumanLoop</code> was invoked directly.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HumanLoopActivationResults {
    /// <p>A copy of the human loop activation conditions of the flow definition, augmented with the results of evaluating those conditions on the input provided to the <code>StartHumanLoop</code> operation.</p>
    #[serde(rename = "HumanLoopActivationConditionsEvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_conditions_evaluation_results: Option<String>,
    /// <p>An object containing information about why a human loop was triggered.</p>
    #[serde(rename = "HumanLoopActivationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_reason: Option<HumanLoopActivationReason>,
}

/// <p>An object containing the input.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanLoopInputContent {
    /// <p>Serialized input from the human loop.</p>
    #[serde(rename = "InputContent")]
    pub input_content: String,
}

/// <p>Information about where the human output will be stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HumanLoopOutputContent {
    /// <p>The location of the Amazon S3 object where Amazon Augmented AI stores your human loop output. The output is stored at the following location: <code>s3://S3OutputPath/HumanLoopName/CreationTime/output.json</code>.</p>
    #[serde(rename = "OutputS3Uri")]
    pub output_s3_uri: String,
}

/// <p>Summary information about the human loop.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HumanLoopSummary {
    /// <p>When Amazon Augmented AI created the human loop.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The reason why the human loop failed. A failure reason is returned only when the status of the human loop is <code>Failed</code>.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    #[serde(rename = "FlowDefinitionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_definition_arn: Option<String>,
    /// <p>The name of the human loop.</p>
    #[serde(rename = "HumanLoopName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_name: Option<String>,
    /// <p>The status of the human loop. Valid values:</p>
    #[serde(rename = "HumanLoopStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_status: Option<String>,
}

/// <p>Attributes of the data specified by the customer. Use these to describe the data to be labeled.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HumanReviewDataAttributes {
    /// <p>Declares that your content is free of personally identifiable information or adult content. Amazon SageMaker may restrict the Amazon Mechanical Turk workers that can view your task based on this information.</p>
    #[serde(rename = "ContentClassifiers")]
    pub content_classifiers: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHumanLoopsRequest {
    /// <p>(Optional) The timestamp of the date when you want the human loops to begin. For example, <code>1551000000</code>.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>(Optional) The timestamp of the date before which you want the human loops to begin. For example, <code>1550000000</code>.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The total number of items to return. If the total number of available items is more than the value specified in <code>MaxResults</code>, then a <code>NextToken</code> will be provided in the output that you can use to resume pagination.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to resume pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional value that specifies whether you want the results sorted in <code>Ascending</code> or <code>Descending</code> order.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHumanLoopsResponse {
    /// <p>An array of objects containing information about the human loops.</p>
    #[serde(rename = "HumanLoopSummaries")]
    pub human_loop_summaries: Vec<HumanLoopSummary>,
    /// <p>A token to resume pagination.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartHumanLoopRequest {
    /// <p>Attributes of the data specified by the customer.</p>
    #[serde(rename = "DataAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HumanReviewDataAttributes>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
    /// <p>An object containing information about the human loop.</p>
    #[serde(rename = "HumanLoopInput")]
    pub human_loop_input: HumanLoopInputContent,
    /// <p>The name of the human loop.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartHumanLoopResponse {
    /// <p>An object containing information about the human loop activation.</p>
    #[serde(rename = "HumanLoopActivationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_results: Option<HumanLoopActivationResults>,
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    #[serde(rename = "HumanLoopArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopHumanLoopRequest {
    /// <p>The name of the human loop you want to stop.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopHumanLoopResponse {}

/// Errors returned by DeleteHumanLoop
#[derive(Debug, PartialEq)]
pub enum DeleteHumanLoopError {
    /// <p>Your request could not be processed.</p>
    InternalServer(String),
    /// <p>We were unable to find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>Your request has exceeded the allowed amount of requests.</p>
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
    /// <p>Your request could not be processed.</p>
    InternalServer(String),
    /// <p>We were unable to find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>Your request has exceeded the allowed amount of requests.</p>
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
    /// <p>Your request could not be processed.</p>
    InternalServer(String),
    /// <p>Your request has exceeded the allowed amount of requests.</p>
    Throttling(String),
}

impl ListHumanLoopsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHumanLoopsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListHumanLoopsError::InternalServer(err.msg))
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
            ListHumanLoopsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHumanLoopsError {}
/// Errors returned by StartHumanLoop
#[derive(Debug, PartialEq)]
pub enum StartHumanLoopError {
    /// <p>Your request could not be processed.</p>
    InternalServer(String),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or request a service quota increase.</p>
    ServiceQuotaExceeded(String),
    /// <p>Your request has exceeded the allowed amount of requests.</p>
    Throttling(String),
}

impl StartHumanLoopError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartHumanLoopError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
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
    /// <p>Your request could not be processed.</p>
    InternalServer(String),
    /// <p>We were unable to find the requested resource.</p>
    ResourceNotFound(String),
    /// <p>Your request has exceeded the allowed amount of requests.</p>
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
pub trait SagemakerA2iRuntime {
    /// <p>Deletes the specified human loop for a flow definition.</p>
    fn delete_human_loop(
        &self,
        input: DeleteHumanLoopRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteHumanLoopResponse, RusotoError<DeleteHumanLoopError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Returns information about the specified human loop.</p>
    fn describe_human_loop(
        &self,
        input: DescribeHumanLoopRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DescribeHumanLoopResponse, RusotoError<DescribeHumanLoopError>>,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Returns information about human loops, given the specified parameters.</p>
    fn list_human_loops(
        &self,
        input: ListHumanLoopsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListHumanLoopsResponse, RusotoError<ListHumanLoopsError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Starts a human loop, provided that at least one activation condition is met.</p>
    fn start_human_loop(
        &self,
        input: StartHumanLoopRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<StartHumanLoopResponse, RusotoError<StartHumanLoopError>>>
                + Send
                + 'static,
        >,
    >;

    /// <p>Stops the specified human loop.</p>
    fn stop_human_loop(
        &self,
        input: StopHumanLoopRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<StopHumanLoopResponse, RusotoError<StopHumanLoopError>>>
                + Send
                + 'static,
        >,
    >;
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

impl SagemakerA2iRuntime for SagemakerA2iRuntimeClient {
    /// <p>Deletes the specified human loop for a flow definition.</p>
    fn delete_human_loop(
        &self,
        input: DeleteHumanLoopRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DeleteHumanLoopResponse, RusotoError<DeleteHumanLoopError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/human-loops/{human_loop_name}",
            human_loop_name = input.human_loop_name
        );

        let mut request = SignedRequest::new("DELETE", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DeleteHumanLoopResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DeleteHumanLoopError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns information about the specified human loop.</p>
    fn describe_human_loop(
        &self,
        input: DescribeHumanLoopRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<DescribeHumanLoopResponse, RusotoError<DescribeHumanLoopError>>,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/human-loops/{human_loop_name}",
            human_loop_name = input.human_loop_name
        );

        let mut request = SignedRequest::new("GET", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeHumanLoopResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeHumanLoopError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Returns information about human loops, given the specified parameters.</p>
    fn list_human_loops(
        &self,
        input: ListHumanLoopsRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<ListHumanLoopsResponse, RusotoError<ListHumanLoopsError>>>
                + Send
                + 'static,
        >,
    > {
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

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListHumanLoopsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListHumanLoopsError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Starts a human loop, provided that at least one activation condition is met.</p>
    fn start_human_loop(
        &self,
        input: StartHumanLoopRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<StartHumanLoopResponse, RusotoError<StartHumanLoopError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/human-loops";

        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<StartHumanLoopResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(StartHumanLoopError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Stops the specified human loop.</p>
    fn stop_human_loop(
        &self,
        input: StopHumanLoopRequest,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<StopHumanLoopResponse, RusotoError<StopHumanLoopError>>>
                + Send
                + 'static,
        >,
    > {
        let request_uri = "/human-loops/stop";

        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("a2i-runtime.sagemaker".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<StopHumanLoopResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(StopHumanLoopError::from_response(response))
            }
        }
        .boxed()
    }
}
