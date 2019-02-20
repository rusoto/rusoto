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

#[allow(unused_imports)]
use rusoto_core::signature::decode_uri;

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Attributes {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ClaimDevicesByClaimCodeRequest {
    /// <p>The claim code, starting with "C-", as provided by the device manufacturer.</p>
    #[serde(rename = "ClaimCode")]
    pub claim_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ClaimDevicesByClaimCodeResponse {
    /// <p>The claim code provided by the device manufacturer.</p>
    #[serde(rename = "ClaimCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_code: Option<String>,
    /// <p>The total number of devices associated with the claim code that has been processed
    /// in the claim request.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDeviceRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDeviceResponse {
    /// <p>Device details.</p>
    #[serde(rename = "DeviceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_description: Option<DeviceDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Device {
    /// <p>The user specified attributes associated with the device for an event.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>The device type, such as "button".</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceClaimResponse {
    /// <p>The device's final claim state.</p>
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeviceDescription {
    /// <p>An array of zero or more elements of DeviceAttribute objects
    /// providing user specified device attributes.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>A Boolean value indicating whether or not the device is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>A value between 0 and 1 inclusive, representing the fraction of life remaining for
    /// the device.</p>
    #[serde(rename = "RemainingLife")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_life: Option<f64>,
    /// <p>The type of the device, such as "button".</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeviceEvent {
    /// <p>An object representing the device associated with the event.</p>
    #[serde(rename = "Device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>A serialized JSON object representing the device-type specific event.</p>
    #[serde(rename = "StdEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_event: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeviceEventsResponse {
    /// <p>An array of zero or more elements describing the event(s) associated with the
    /// device.</p>
    pub events: Option<Vec<DeviceEvent>>,
    /// <p>The token to retrieve the next set of results.</p>
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceMethod {
    /// <p>The type of the device, such as "button".</p>
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// <p>The name of the method applicable to the deviceType.</p>
    #[serde(rename = "MethodName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_name: Option<String>,
}

/// <p>On success, an empty object is returned.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Empty {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FinalizeDeviceClaimRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FinalizeDeviceClaimResponse {
    /// <p>The device's final claim state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceMethodsRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDeviceMethodsResponse {
    /// <p>List of available device APIs.</p>
    #[serde(rename = "DeviceMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_methods: Option<Vec<DeviceMethod>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InitiateDeviceClaimRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InitiateDeviceClaimResponse {
    /// <p>The device's final claim state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InvokeDeviceMethodRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The device method to invoke.</p>
    #[serde(rename = "DeviceMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_method: Option<DeviceMethod>,
    /// <p>A JSON encoded string containing the device method request parameters.</p>
    #[serde(rename = "DeviceMethodParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_method_parameters: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InvokeDeviceMethodResponse {
    /// <p>A JSON encoded string containing the device method response.</p>
    #[serde(rename = "DeviceMethodResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_method_response: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeviceEventsRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>The start date for the device event query, in ISO8061 format. For example,
    /// 2018-03-28T15:45:12.880Z
    /// </p>
    #[serde(rename = "FromTimeStamp")]
    pub from_time_stamp: f64,
    /// <p>The maximum number of results to return per request. If not set, a default value
    /// of 100 is used.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The end date for the device event query, in ISO8061 format. For example,
    /// 2018-03-28T15:45:12.880Z
    /// </p>
    #[serde(rename = "ToTimeStamp")]
    pub to_time_stamp: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDeviceEventsResponse {
    /// <p>An array of zero or more elements describing the event(s) associated with the
    /// device.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DeviceEvent>>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDevicesRequest {
    /// <p>The type of the device, such as "button".</p>
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// <p>The maximum number of results to return per request. If not set, a default value
    /// of 100 is used.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDevicesResponse {
    /// <p>A list of devices.</p>
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceDescription>>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnclaimDeviceRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnclaimDeviceResponse {
    /// <p>The device's final claim state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeviceStateRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>If true, the device is enabled. If false, the device is
    /// disabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDeviceStateResponse {}

/// Errors returned by ClaimDevicesByClaimCode
#[derive(Debug, PartialEq)]
pub enum ClaimDevicesByClaimCodeError {
    Forbidden(String),

    InternalFailure(String),

    InvalidRequest(String),
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

impl ClaimDevicesByClaimCodeError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ClaimDevicesByClaimCodeError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ForbiddenException" => {
                    return ClaimDevicesByClaimCodeError::Forbidden(String::from(error_message));
                }
                "InternalFailureException" => {
                    return ClaimDevicesByClaimCodeError::InternalFailure(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return ClaimDevicesByClaimCodeError::InvalidRequest(String::from(error_message));
                }
                "ValidationException" => {
                    return ClaimDevicesByClaimCodeError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ClaimDevicesByClaimCodeError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ClaimDevicesByClaimCodeError {
    fn from(err: serde_json::error::Error) -> ClaimDevicesByClaimCodeError {
        ClaimDevicesByClaimCodeError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ClaimDevicesByClaimCodeError {
    fn from(err: CredentialsError) -> ClaimDevicesByClaimCodeError {
        ClaimDevicesByClaimCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ClaimDevicesByClaimCodeError {
    fn from(err: HttpDispatchError) -> ClaimDevicesByClaimCodeError {
        ClaimDevicesByClaimCodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for ClaimDevicesByClaimCodeError {
    fn from(err: io::Error) -> ClaimDevicesByClaimCodeError {
        ClaimDevicesByClaimCodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ClaimDevicesByClaimCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ClaimDevicesByClaimCodeError {
    fn description(&self) -> &str {
        match *self {
            ClaimDevicesByClaimCodeError::Forbidden(ref cause) => cause,
            ClaimDevicesByClaimCodeError::InternalFailure(ref cause) => cause,
            ClaimDevicesByClaimCodeError::InvalidRequest(ref cause) => cause,
            ClaimDevicesByClaimCodeError::Validation(ref cause) => cause,
            ClaimDevicesByClaimCodeError::Credentials(ref err) => err.description(),
            ClaimDevicesByClaimCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ClaimDevicesByClaimCodeError::ParseError(ref cause) => cause,
            ClaimDevicesByClaimCodeError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDevice
#[derive(Debug, PartialEq)]
pub enum DescribeDeviceError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
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

impl DescribeDeviceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDeviceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return DescribeDeviceError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DescribeDeviceError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DescribeDeviceError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeDeviceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeDeviceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDeviceError {
    fn from(err: serde_json::error::Error) -> DescribeDeviceError {
        DescribeDeviceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDeviceError {
    fn from(err: CredentialsError) -> DescribeDeviceError {
        DescribeDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDeviceError {
    fn from(err: HttpDispatchError) -> DescribeDeviceError {
        DescribeDeviceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDeviceError {
    fn from(err: io::Error) -> DescribeDeviceError {
        DescribeDeviceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDeviceError {
    fn description(&self) -> &str {
        match *self {
            DescribeDeviceError::InternalFailure(ref cause) => cause,
            DescribeDeviceError::InvalidRequest(ref cause) => cause,
            DescribeDeviceError::ResourceNotFound(ref cause) => cause,
            DescribeDeviceError::Validation(ref cause) => cause,
            DescribeDeviceError::Credentials(ref err) => err.description(),
            DescribeDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeDeviceError::ParseError(ref cause) => cause,
            DescribeDeviceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by FinalizeDeviceClaim
#[derive(Debug, PartialEq)]
pub enum FinalizeDeviceClaimError {
    InternalFailure(String),

    InvalidRequest(String),

    PreconditionFailed(String),

    ResourceConflict(String),

    ResourceNotFound(String),
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

impl FinalizeDeviceClaimError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> FinalizeDeviceClaimError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return FinalizeDeviceClaimError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return FinalizeDeviceClaimError::InvalidRequest(String::from(error_message));
                }
                "PreconditionFailedException" => {
                    return FinalizeDeviceClaimError::PreconditionFailed(String::from(error_message));
                }
                "ResourceConflictException" => {
                    return FinalizeDeviceClaimError::ResourceConflict(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return FinalizeDeviceClaimError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return FinalizeDeviceClaimError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return FinalizeDeviceClaimError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for FinalizeDeviceClaimError {
    fn from(err: serde_json::error::Error) -> FinalizeDeviceClaimError {
        FinalizeDeviceClaimError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for FinalizeDeviceClaimError {
    fn from(err: CredentialsError) -> FinalizeDeviceClaimError {
        FinalizeDeviceClaimError::Credentials(err)
    }
}
impl From<HttpDispatchError> for FinalizeDeviceClaimError {
    fn from(err: HttpDispatchError) -> FinalizeDeviceClaimError {
        FinalizeDeviceClaimError::HttpDispatch(err)
    }
}
impl From<io::Error> for FinalizeDeviceClaimError {
    fn from(err: io::Error) -> FinalizeDeviceClaimError {
        FinalizeDeviceClaimError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for FinalizeDeviceClaimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for FinalizeDeviceClaimError {
    fn description(&self) -> &str {
        match *self {
            FinalizeDeviceClaimError::InternalFailure(ref cause) => cause,
            FinalizeDeviceClaimError::InvalidRequest(ref cause) => cause,
            FinalizeDeviceClaimError::PreconditionFailed(ref cause) => cause,
            FinalizeDeviceClaimError::ResourceConflict(ref cause) => cause,
            FinalizeDeviceClaimError::ResourceNotFound(ref cause) => cause,
            FinalizeDeviceClaimError::Validation(ref cause) => cause,
            FinalizeDeviceClaimError::Credentials(ref err) => err.description(),
            FinalizeDeviceClaimError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            FinalizeDeviceClaimError::ParseError(ref cause) => cause,
            FinalizeDeviceClaimError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetDeviceMethods
#[derive(Debug, PartialEq)]
pub enum GetDeviceMethodsError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
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

impl GetDeviceMethodsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetDeviceMethodsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return GetDeviceMethodsError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return GetDeviceMethodsError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetDeviceMethodsError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return GetDeviceMethodsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetDeviceMethodsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDeviceMethodsError {
    fn from(err: serde_json::error::Error) -> GetDeviceMethodsError {
        GetDeviceMethodsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeviceMethodsError {
    fn from(err: CredentialsError) -> GetDeviceMethodsError {
        GetDeviceMethodsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeviceMethodsError {
    fn from(err: HttpDispatchError) -> GetDeviceMethodsError {
        GetDeviceMethodsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeviceMethodsError {
    fn from(err: io::Error) -> GetDeviceMethodsError {
        GetDeviceMethodsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeviceMethodsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceMethodsError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceMethodsError::InternalFailure(ref cause) => cause,
            GetDeviceMethodsError::InvalidRequest(ref cause) => cause,
            GetDeviceMethodsError::ResourceNotFound(ref cause) => cause,
            GetDeviceMethodsError::Validation(ref cause) => cause,
            GetDeviceMethodsError::Credentials(ref err) => err.description(),
            GetDeviceMethodsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDeviceMethodsError::ParseError(ref cause) => cause,
            GetDeviceMethodsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by InitiateDeviceClaim
#[derive(Debug, PartialEq)]
pub enum InitiateDeviceClaimError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceConflict(String),

    ResourceNotFound(String),
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

impl InitiateDeviceClaimError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> InitiateDeviceClaimError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return InitiateDeviceClaimError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return InitiateDeviceClaimError::InvalidRequest(String::from(error_message));
                }
                "ResourceConflictException" => {
                    return InitiateDeviceClaimError::ResourceConflict(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return InitiateDeviceClaimError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return InitiateDeviceClaimError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return InitiateDeviceClaimError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for InitiateDeviceClaimError {
    fn from(err: serde_json::error::Error) -> InitiateDeviceClaimError {
        InitiateDeviceClaimError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for InitiateDeviceClaimError {
    fn from(err: CredentialsError) -> InitiateDeviceClaimError {
        InitiateDeviceClaimError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InitiateDeviceClaimError {
    fn from(err: HttpDispatchError) -> InitiateDeviceClaimError {
        InitiateDeviceClaimError::HttpDispatch(err)
    }
}
impl From<io::Error> for InitiateDeviceClaimError {
    fn from(err: io::Error) -> InitiateDeviceClaimError {
        InitiateDeviceClaimError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InitiateDeviceClaimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitiateDeviceClaimError {
    fn description(&self) -> &str {
        match *self {
            InitiateDeviceClaimError::InternalFailure(ref cause) => cause,
            InitiateDeviceClaimError::InvalidRequest(ref cause) => cause,
            InitiateDeviceClaimError::ResourceConflict(ref cause) => cause,
            InitiateDeviceClaimError::ResourceNotFound(ref cause) => cause,
            InitiateDeviceClaimError::Validation(ref cause) => cause,
            InitiateDeviceClaimError::Credentials(ref err) => err.description(),
            InitiateDeviceClaimError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InitiateDeviceClaimError::ParseError(ref cause) => cause,
            InitiateDeviceClaimError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by InvokeDeviceMethod
#[derive(Debug, PartialEq)]
pub enum InvokeDeviceMethodError {
    InternalFailure(String),

    InvalidRequest(String),

    PreconditionFailed(String),

    RangeNotSatisfiable(String),

    ResourceConflict(String),

    ResourceNotFound(String),
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

impl InvokeDeviceMethodError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> InvokeDeviceMethodError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return InvokeDeviceMethodError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return InvokeDeviceMethodError::InvalidRequest(String::from(error_message));
                }
                "PreconditionFailedException" => {
                    return InvokeDeviceMethodError::PreconditionFailed(String::from(error_message));
                }
                "RangeNotSatisfiableException" => {
                    return InvokeDeviceMethodError::RangeNotSatisfiable(String::from(error_message));
                }
                "ResourceConflictException" => {
                    return InvokeDeviceMethodError::ResourceConflict(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return InvokeDeviceMethodError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return InvokeDeviceMethodError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return InvokeDeviceMethodError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for InvokeDeviceMethodError {
    fn from(err: serde_json::error::Error) -> InvokeDeviceMethodError {
        InvokeDeviceMethodError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for InvokeDeviceMethodError {
    fn from(err: CredentialsError) -> InvokeDeviceMethodError {
        InvokeDeviceMethodError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InvokeDeviceMethodError {
    fn from(err: HttpDispatchError) -> InvokeDeviceMethodError {
        InvokeDeviceMethodError::HttpDispatch(err)
    }
}
impl From<io::Error> for InvokeDeviceMethodError {
    fn from(err: io::Error) -> InvokeDeviceMethodError {
        InvokeDeviceMethodError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InvokeDeviceMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InvokeDeviceMethodError {
    fn description(&self) -> &str {
        match *self {
            InvokeDeviceMethodError::InternalFailure(ref cause) => cause,
            InvokeDeviceMethodError::InvalidRequest(ref cause) => cause,
            InvokeDeviceMethodError::PreconditionFailed(ref cause) => cause,
            InvokeDeviceMethodError::RangeNotSatisfiable(ref cause) => cause,
            InvokeDeviceMethodError::ResourceConflict(ref cause) => cause,
            InvokeDeviceMethodError::ResourceNotFound(ref cause) => cause,
            InvokeDeviceMethodError::Validation(ref cause) => cause,
            InvokeDeviceMethodError::Credentials(ref err) => err.description(),
            InvokeDeviceMethodError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InvokeDeviceMethodError::ParseError(ref cause) => cause,
            InvokeDeviceMethodError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDeviceEvents
#[derive(Debug, PartialEq)]
pub enum ListDeviceEventsError {
    InternalFailure(String),

    InvalidRequest(String),

    RangeNotSatisfiable(String),

    ResourceNotFound(String),
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

impl ListDeviceEventsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListDeviceEventsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return ListDeviceEventsError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListDeviceEventsError::InvalidRequest(String::from(error_message));
                }
                "RangeNotSatisfiableException" => {
                    return ListDeviceEventsError::RangeNotSatisfiable(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return ListDeviceEventsError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return ListDeviceEventsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListDeviceEventsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDeviceEventsError {
    fn from(err: serde_json::error::Error) -> ListDeviceEventsError {
        ListDeviceEventsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeviceEventsError {
    fn from(err: CredentialsError) -> ListDeviceEventsError {
        ListDeviceEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeviceEventsError {
    fn from(err: HttpDispatchError) -> ListDeviceEventsError {
        ListDeviceEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeviceEventsError {
    fn from(err: io::Error) -> ListDeviceEventsError {
        ListDeviceEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeviceEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeviceEventsError {
    fn description(&self) -> &str {
        match *self {
            ListDeviceEventsError::InternalFailure(ref cause) => cause,
            ListDeviceEventsError::InvalidRequest(ref cause) => cause,
            ListDeviceEventsError::RangeNotSatisfiable(ref cause) => cause,
            ListDeviceEventsError::ResourceNotFound(ref cause) => cause,
            ListDeviceEventsError::Validation(ref cause) => cause,
            ListDeviceEventsError::Credentials(ref err) => err.description(),
            ListDeviceEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDeviceEventsError::ParseError(ref cause) => cause,
            ListDeviceEventsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDevices
#[derive(Debug, PartialEq)]
pub enum ListDevicesError {
    InternalFailure(String),

    InvalidRequest(String),

    RangeNotSatisfiable(String),
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

impl ListDevicesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListDevicesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return ListDevicesError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListDevicesError::InvalidRequest(String::from(error_message));
                }
                "RangeNotSatisfiableException" => {
                    return ListDevicesError::RangeNotSatisfiable(String::from(error_message));
                }
                "ValidationException" => {
                    return ListDevicesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListDevicesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDevicesError {
    fn from(err: serde_json::error::Error) -> ListDevicesError {
        ListDevicesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDevicesError {
    fn from(err: CredentialsError) -> ListDevicesError {
        ListDevicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDevicesError {
    fn from(err: HttpDispatchError) -> ListDevicesError {
        ListDevicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDevicesError {
    fn from(err: io::Error) -> ListDevicesError {
        ListDevicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDevicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDevicesError {
    fn description(&self) -> &str {
        match *self {
            ListDevicesError::InternalFailure(ref cause) => cause,
            ListDevicesError::InvalidRequest(ref cause) => cause,
            ListDevicesError::RangeNotSatisfiable(ref cause) => cause,
            ListDevicesError::Validation(ref cause) => cause,
            ListDevicesError::Credentials(ref err) => err.description(),
            ListDevicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDevicesError::ParseError(ref cause) => cause,
            ListDevicesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UnclaimDevice
#[derive(Debug, PartialEq)]
pub enum UnclaimDeviceError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
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

impl UnclaimDeviceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UnclaimDeviceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return UnclaimDeviceError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return UnclaimDeviceError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UnclaimDeviceError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UnclaimDeviceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UnclaimDeviceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UnclaimDeviceError {
    fn from(err: serde_json::error::Error) -> UnclaimDeviceError {
        UnclaimDeviceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UnclaimDeviceError {
    fn from(err: CredentialsError) -> UnclaimDeviceError {
        UnclaimDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UnclaimDeviceError {
    fn from(err: HttpDispatchError) -> UnclaimDeviceError {
        UnclaimDeviceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UnclaimDeviceError {
    fn from(err: io::Error) -> UnclaimDeviceError {
        UnclaimDeviceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UnclaimDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnclaimDeviceError {
    fn description(&self) -> &str {
        match *self {
            UnclaimDeviceError::InternalFailure(ref cause) => cause,
            UnclaimDeviceError::InvalidRequest(ref cause) => cause,
            UnclaimDeviceError::ResourceNotFound(ref cause) => cause,
            UnclaimDeviceError::Validation(ref cause) => cause,
            UnclaimDeviceError::Credentials(ref err) => err.description(),
            UnclaimDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UnclaimDeviceError::ParseError(ref cause) => cause,
            UnclaimDeviceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateDeviceState
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceStateError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
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

impl UpdateDeviceStateError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDeviceStateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InternalFailureException" => {
                    return UpdateDeviceStateError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return UpdateDeviceStateError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateDeviceStateError::ResourceNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateDeviceStateError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateDeviceStateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDeviceStateError {
    fn from(err: serde_json::error::Error) -> UpdateDeviceStateError {
        UpdateDeviceStateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDeviceStateError {
    fn from(err: CredentialsError) -> UpdateDeviceStateError {
        UpdateDeviceStateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDeviceStateError {
    fn from(err: HttpDispatchError) -> UpdateDeviceStateError {
        UpdateDeviceStateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDeviceStateError {
    fn from(err: io::Error) -> UpdateDeviceStateError {
        UpdateDeviceStateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDeviceStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeviceStateError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeviceStateError::InternalFailure(ref cause) => cause,
            UpdateDeviceStateError::InvalidRequest(ref cause) => cause,
            UpdateDeviceStateError::ResourceNotFound(ref cause) => cause,
            UpdateDeviceStateError::Validation(ref cause) => cause,
            UpdateDeviceStateError::Credentials(ref err) => err.description(),
            UpdateDeviceStateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDeviceStateError::ParseError(ref cause) => cause,
            UpdateDeviceStateError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS IoT 1-Click Devices Service API. AWS IoT 1-Click Devices Service clients implement this trait.
pub trait Iot1ClickDevices {
    /// <p>Adds device(s) to your account (i.e., claim one or more devices) if and only if
    /// you received a claim code with the device(s).</p>
    fn claim_devices_by_claim_code(
        &self,
        input: ClaimDevicesByClaimCodeRequest,
    ) -> RusotoFuture<ClaimDevicesByClaimCodeResponse, ClaimDevicesByClaimCodeError>;

    /// <p>Given a device ID, returns a DescribeDeviceResponse object describing
    /// the details of the device.</p>
    fn describe_device(
        &self,
        input: DescribeDeviceRequest,
    ) -> RusotoFuture<DescribeDeviceResponse, DescribeDeviceError>;

    /// <p>Given a device ID, finalizes the claim request for the associated device.</p><note>
    /// <p>Claiming a device consists of initiating a claim, then publishing a device
    /// event, and finalizing the claim. For a device of type button, a
    /// device event can be published by simply clicking the device.</p>
    ///
    /// <p></note></p>
    fn finalize_device_claim(
        &self,
        input: FinalizeDeviceClaimRequest,
    ) -> RusotoFuture<FinalizeDeviceClaimResponse, FinalizeDeviceClaimError>;

    /// <p>Given a device ID, returns the invokable methods associated with the
    /// device.</p>
    fn get_device_methods(
        &self,
        input: GetDeviceMethodsRequest,
    ) -> RusotoFuture<GetDeviceMethodsResponse, GetDeviceMethodsError>;

    /// <p>Given a device ID, initiates a claim request for the associated device.</p><note>
    /// <p>Claiming a device consists of initiating a claim, then publishing a device
    /// event, and finalizing the claim. For a device of type button, a
    /// device event can be published by simply clicking the device.</p>
    ///
    /// <p></note></p>
    fn initiate_device_claim(
        &self,
        input: InitiateDeviceClaimRequest,
    ) -> RusotoFuture<InitiateDeviceClaimResponse, InitiateDeviceClaimError>;

    /// <p>Given a device ID, issues a request to invoke a named device method (with possible
    /// parameters). See the "Example POST" code snippet below.</p>
    fn invoke_device_method(
        &self,
        input: InvokeDeviceMethodRequest,
    ) -> RusotoFuture<InvokeDeviceMethodResponse, InvokeDeviceMethodError>;

    /// <p>Using a device ID, returns a DeviceEventsResponse object containing
    /// an array of events for the device.</p>
    fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> RusotoFuture<ListDeviceEventsResponse, ListDeviceEventsError>;

    /// <p>Lists the 1-Click compatible devices associated with your AWS account.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResponse, ListDevicesError>;

    /// <p>Disassociates a device from your AWS account using its device ID.</p>
    fn unclaim_device(
        &self,
        input: UnclaimDeviceRequest,
    ) -> RusotoFuture<UnclaimDeviceResponse, UnclaimDeviceError>;

    /// <p>Using a Boolean value (true or false), this operation
    /// enables or disables the device given a device ID.</p>
    fn update_device_state(
        &self,
        input: UpdateDeviceStateRequest,
    ) -> RusotoFuture<UpdateDeviceStateResponse, UpdateDeviceStateError>;
}
/// A client for the AWS IoT 1-Click Devices Service API.
#[derive(Clone)]
pub struct Iot1ClickDevicesClient {
    client: Client,
    region: region::Region,
}

impl Iot1ClickDevicesClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Iot1ClickDevicesClient {
        Iot1ClickDevicesClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Iot1ClickDevicesClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Iot1ClickDevicesClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Iot1ClickDevices for Iot1ClickDevicesClient {
    /// <p>Adds device(s) to your account (i.e., claim one or more devices) if and only if
    /// you received a claim code with the device(s).</p>
    fn claim_devices_by_claim_code(
        &self,
        input: ClaimDevicesByClaimCodeRequest,
    ) -> RusotoFuture<ClaimDevicesByClaimCodeResponse, ClaimDevicesByClaimCodeError> {
        let request_uri = format!("/claims/{claim_code}", claim_code = input.claim_code);

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ClaimDevicesByClaimCodeResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ClaimDevicesByClaimCodeError::from_response(response))
                }))
            }
        })
    }

    /// <p>Given a device ID, returns a DescribeDeviceResponse object describing
    /// the details of the device.</p>
    fn describe_device(
        &self,
        input: DescribeDeviceRequest,
    ) -> RusotoFuture<DescribeDeviceResponse, DescribeDeviceError> {
        let request_uri = format!("/devices/{device_id}", device_id = input.device_id);

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeDeviceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Given a device ID, finalizes the claim request for the associated device.</p><note>
    /// <p>Claiming a device consists of initiating a claim, then publishing a device
    /// event, and finalizing the claim. For a device of type button, a
    /// device event can be published by simply clicking the device.</p>
    ///
    /// <p></note></p>
    fn finalize_device_claim(
        &self,
        input: FinalizeDeviceClaimRequest,
    ) -> RusotoFuture<FinalizeDeviceClaimResponse, FinalizeDeviceClaimError> {
        let request_uri = format!(
            "/devices/{device_id}/finalize-claim",
            device_id = input.device_id
        );

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<FinalizeDeviceClaimResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(FinalizeDeviceClaimError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Given a device ID, returns the invokable methods associated with the
    /// device.</p>
    fn get_device_methods(
        &self,
        input: GetDeviceMethodsRequest,
    ) -> RusotoFuture<GetDeviceMethodsResponse, GetDeviceMethodsError> {
        let request_uri = format!("/devices/{device_id}/methods", device_id = input.device_id);

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetDeviceMethodsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDeviceMethodsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Given a device ID, initiates a claim request for the associated device.</p><note>
    /// <p>Claiming a device consists of initiating a claim, then publishing a device
    /// event, and finalizing the claim. For a device of type button, a
    /// device event can be published by simply clicking the device.</p>
    ///
    /// <p></note></p>
    fn initiate_device_claim(
        &self,
        input: InitiateDeviceClaimRequest,
    ) -> RusotoFuture<InitiateDeviceClaimResponse, InitiateDeviceClaimError> {
        let request_uri = format!(
            "/devices/{device_id}/initiate-claim",
            device_id = input.device_id
        );

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<InitiateDeviceClaimResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(InitiateDeviceClaimError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Given a device ID, issues a request to invoke a named device method (with possible
    /// parameters). See the "Example POST" code snippet below.</p>
    fn invoke_device_method(
        &self,
        input: InvokeDeviceMethodRequest,
    ) -> RusotoFuture<InvokeDeviceMethodResponse, InvokeDeviceMethodError> {
        let request_uri = format!("/devices/{device_id}/methods", device_id = input.device_id);

        let mut request = SignedRequest::new("POST", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<InvokeDeviceMethodResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(InvokeDeviceMethodError::from_response(response))),
                )
            }
        })
    }

    /// <p>Using a device ID, returns a DeviceEventsResponse object containing
    /// an array of events for the device.</p>
    fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> RusotoFuture<ListDeviceEventsResponse, ListDeviceEventsError> {
        let request_uri = format!("/devices/{device_id}/events", device_id = input.device_id);

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        let mut params = Params::new();
        params.put("fromTimeStamp", &input.from_time_stamp);
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        params.put("toTimeStamp", &input.to_time_stamp);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListDeviceEventsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDeviceEventsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the 1-Click compatible devices associated with your AWS account.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResponse, ListDevicesError> {
        let request_uri = "/devices";

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.device_type {
            params.put("deviceType", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListDevicesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDevicesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disassociates a device from your AWS account using its device ID.</p>
    fn unclaim_device(
        &self,
        input: UnclaimDeviceRequest,
    ) -> RusotoFuture<UnclaimDeviceResponse, UnclaimDeviceError> {
        let request_uri = format!("/devices/{device_id}/unclaim", device_id = input.device_id);

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UnclaimDeviceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UnclaimDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Using a Boolean value (true or false), this operation
    /// enables or disables the device given a device ID.</p>
    fn update_device_state(
        &self,
        input: UpdateDeviceStateRequest,
    ) -> RusotoFuture<UpdateDeviceStateResponse, UpdateDeviceStateError> {
        let request_uri = format!("/devices/{device_id}/state", device_id = input.device_id);

        let mut request = SignedRequest::new("PUT", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateDeviceStateResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDeviceStateError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
