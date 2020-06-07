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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHomeRegionControlRequest {
    /// <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether the caller has permission to make the call.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The name of the home region of the calling account.</p>
    #[serde(rename = "HomeRegion")]
    pub home_region: String,
    /// <p>The account for which this command sets up a home region control. The <code>Target</code> is always of type <code>ACCOUNT</code>.</p>
    #[serde(rename = "Target")]
    pub target: Target,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHomeRegionControlResult {
    /// <p>This object is the <code>HomeRegionControl</code> object that's returned by a successful call to <code>CreateHomeRegionControl</code>.</p>
    #[serde(rename = "HomeRegionControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region_control: Option<HomeRegionControl>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHomeRegionControlsRequest {
    /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
    #[serde(rename = "ControlId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    /// <p>The name of the home region you'd like to view.</p>
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// <p>The maximum number of filtering results to display per page. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHomeRegionControlsResult {
    /// <p>An array that contains your <code>HomeRegionControl</code> objects.</p>
    #[serde(rename = "HomeRegionControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region_controls: Option<Vec<HomeRegionControl>>,
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetHomeRegionRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetHomeRegionResult {
    /// <p>The name of the home region of the calling account.</p>
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
}

/// <p>A home region control is an object that specifies the home region for an account, with some additional information. It contains a target (always of type <code>ACCOUNT</code>), an ID, and a time at which the home region was set.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HomeRegionControl {
    /// <p>A unique identifier that's generated for each home region control. It's always a string that begins with "hrc-" followed by 12 lowercase letters and numbers.</p>
    #[serde(rename = "ControlId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    /// <p>The AWS Region that's been set as home region. For example, "us-west-2" or "eu-central-1" are valid home regions.</p>
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// <p>A timestamp representing the time when the customer called <code>CreateHomeregionControl</code> and set the home region for the account.</p>
    #[serde(rename = "RequestedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_time: Option<f64>,
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always an <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}

/// <p>The target parameter specifies the identifier to which the home region is applied, which is always an <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Target {
    /// <p>The <code>TargetID</code> is a 12-character identifier of the <code>ACCOUNT</code> for which the control was created. (This must be the current account.) </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The target type is always an <code>ACCOUNT</code>.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// Errors returned by CreateHomeRegionControl
#[derive(Debug, PartialEq)]
pub enum CreateHomeRegionControlError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised to indicate that authorization of an action was successful, when the <code>DryRun</code> flag is set to true.</p>
    DryRunOperation(String),
    /// <p>Exception raised when an internal, configuration, or dependency error is encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when a request fails due to temporary unavailability of the service.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateHomeRegionControlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHomeRegionControlError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateHomeRegionControlError::AccessDenied(
                        err.msg,
                    ))
                }
                "DryRunOperation" => {
                    return RusotoError::Service(CreateHomeRegionControlError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateHomeRegionControlError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateHomeRegionControlError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateHomeRegionControlError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateHomeRegionControlError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateHomeRegionControlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHomeRegionControlError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateHomeRegionControlError::DryRunOperation(ref cause) => write!(f, "{}", cause),
            CreateHomeRegionControlError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateHomeRegionControlError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateHomeRegionControlError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateHomeRegionControlError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHomeRegionControlError {}
/// Errors returned by DescribeHomeRegionControls
#[derive(Debug, PartialEq)]
pub enum DescribeHomeRegionControlsError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised when an internal, configuration, or dependency error is encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when a request fails due to temporary unavailability of the service.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeHomeRegionControlsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeHomeRegionControlsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeHomeRegionControlsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeHomeRegionControlsError::InternalServerError(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeHomeRegionControlsError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        DescribeHomeRegionControlsError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeHomeRegionControlsError::Throttling(
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
impl fmt::Display for DescribeHomeRegionControlsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHomeRegionControlsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeHomeRegionControlsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeHomeRegionControlsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeHomeRegionControlsError::ServiceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeHomeRegionControlsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeHomeRegionControlsError {}
/// Errors returned by GetHomeRegion
#[derive(Debug, PartialEq)]
pub enum GetHomeRegionError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>Exception raised when an internal, configuration, or dependency error is encountered.</p>
    InternalServerError(String),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInput(String),
    /// <p>Exception raised when a request fails due to temporary unavailability of the service.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetHomeRegionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetHomeRegionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetHomeRegionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetHomeRegionError::InternalServerError(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetHomeRegionError::InvalidInput(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetHomeRegionError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetHomeRegionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetHomeRegionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetHomeRegionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetHomeRegionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetHomeRegionError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetHomeRegionError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            GetHomeRegionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetHomeRegionError {}
/// Trait representing the capabilities of the AWS Migration Hub Config API. AWS Migration Hub Config clients implement this trait.
#[async_trait]
pub trait MigrationHubConfig {
    /// <p>This API sets up the home region for the calling account only.</p>
    async fn create_home_region_control(
        &self,
        input: CreateHomeRegionControlRequest,
    ) -> Result<CreateHomeRegionControlResult, RusotoError<CreateHomeRegionControlError>>;

    /// <p>This API permits filtering on the <code>ControlId</code> and <code>HomeRegion</code> fields.</p>
    async fn describe_home_region_controls(
        &self,
        input: DescribeHomeRegionControlsRequest,
    ) -> Result<DescribeHomeRegionControlsResult, RusotoError<DescribeHomeRegionControlsError>>;

    /// <p>Returns the calling account’s home region, if configured. This API is used by other AWS services to determine the regional endpoint for calling AWS Application Discovery Service and Migration Hub. You must call <code>GetHomeRegion</code> at least once before you call any other AWS Application Discovery Service and AWS Migration Hub APIs, to obtain the account's Migration Hub home region.</p>
    async fn get_home_region(&self)
        -> Result<GetHomeRegionResult, RusotoError<GetHomeRegionError>>;
}
/// A client for the AWS Migration Hub Config API.
#[derive(Clone)]
pub struct MigrationHubConfigClient {
    client: Client,
    region: region::Region,
}

impl MigrationHubConfigClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MigrationHubConfigClient {
        MigrationHubConfigClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MigrationHubConfigClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MigrationHubConfigClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MigrationHubConfigClient {
        MigrationHubConfigClient { client, region }
    }
}

#[async_trait]
impl MigrationHubConfig for MigrationHubConfigClient {
    /// <p>This API sets up the home region for the calling account only.</p>
    async fn create_home_region_control(
        &self,
        input: CreateHomeRegionControlRequest,
    ) -> Result<CreateHomeRegionControlResult, RusotoError<CreateHomeRegionControlError>> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");
        request.set_endpoint_prefix("migrationhub-config".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSMigrationHubMultiAccountService.CreateHomeRegionControl",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateHomeRegionControlResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateHomeRegionControlError::from_response(response))
        }
    }

    /// <p>This API permits filtering on the <code>ControlId</code> and <code>HomeRegion</code> fields.</p>
    async fn describe_home_region_controls(
        &self,
        input: DescribeHomeRegionControlsRequest,
    ) -> Result<DescribeHomeRegionControlsResult, RusotoError<DescribeHomeRegionControlsError>>
    {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");
        request.set_endpoint_prefix("migrationhub-config".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSMigrationHubMultiAccountService.DescribeHomeRegionControls",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeHomeRegionControlsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHomeRegionControlsError::from_response(response))
        }
    }

    /// <p>Returns the calling account’s home region, if configured. This API is used by other AWS services to determine the regional endpoint for calling AWS Application Discovery Service and Migration Hub. You must call <code>GetHomeRegion</code> at least once before you call any other AWS Application Discovery Service and AWS Migration Hub APIs, to obtain the account's Migration Hub home region.</p>
    async fn get_home_region(
        &self,
    ) -> Result<GetHomeRegionResult, RusotoError<GetHomeRegionError>> {
        let mut request = SignedRequest::new("POST", "mgh", &self.region, "/");
        request.set_endpoint_prefix("migrationhub-config".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSMigrationHubMultiAccountService.GetHomeRegion",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetHomeRegionResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetHomeRegionError::from_response(response))
        }
    }
}
