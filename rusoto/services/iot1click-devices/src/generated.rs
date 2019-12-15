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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Attributes {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ClaimDevicesByClaimCodeRequest {
    /// <p>The claim code, starting with "C-", as provided by the device manufacturer.</p>
    #[serde(rename = "ClaimCode")]
    pub claim_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClaimDevicesByClaimCodeResponse {
    /// <p>The claim code provided by the device manufacturer.</p>
    #[serde(rename = "ClaimCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_code: Option<String>,
    /// <p>The total number of devices associated with the claim code that has been processed in
    /// the claim request.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDeviceResponse {
    /// <p>Device details.</p>
    #[serde(rename = "DeviceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_description: Option<DeviceDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceDescription {
    /// <p>The ARN of the device.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An array of zero or more elements of DeviceAttribute objects providing
    /// user specified device attributes.</p>
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
    /// <p>A value between 0 and 1 inclusive, representing the fraction of life remaining for the
    /// device.</p>
    #[serde(rename = "RemainingLife")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_life: Option<f64>,
    /// <p>The tags currently associated with the AWS IoT 1-Click device.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the device, such as "button".</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FinalizeDeviceClaimRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>A collection of key/value pairs defining the resource tags. For example, {
    /// "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS
    /// Tagging Strategies</a>.</p><p>
    ///
    /// </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The maximum number of results to return per request. If not set, a default value of
    /// 100 is used.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The maximum number of results to return per request. If not set, a default value of
    /// 100 is used.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A collection of key/value pairs defining the resource tags. For example, {
    /// "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS
    /// Tagging Strategies</a>.</p><p>
    ///
    /// </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A collection of key/value pairs defining the resource tags. For example, {
    /// "tags": {"key1": "value1", "key2": "value2"} }. For more information, see <a href="https://aws.amazon.com/answers/account-management/aws-tagging-strategies/">AWS
    /// Tagging Strategies</a>.</p><p>
    ///
    /// </p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnclaimDeviceRequest {
    /// <p>The unique identifier of the device.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnclaimDeviceResponse {
    /// <p>The device's final claim state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A collections of tag keys. For example, {"key1","key2"}</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDeviceStateResponse {}

/// Errors returned by ClaimDevicesByClaimCode
#[derive(Debug, PartialEq)]
pub enum ClaimDevicesByClaimCodeError {
    Forbidden(String),

    InternalFailure(String),

    InvalidRequest(String),
}

impl ClaimDevicesByClaimCodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ClaimDevicesByClaimCodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ForbiddenException" => {
                    return RusotoError::Service(ClaimDevicesByClaimCodeError::Forbidden(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ClaimDevicesByClaimCodeError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ClaimDevicesByClaimCodeError::InvalidRequest(
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
        }
    }
}
/// Errors returned by DescribeDevice
#[derive(Debug, PartialEq)]
pub enum DescribeDeviceError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
}

impl DescribeDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDeviceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDeviceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDeviceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl FinalizeDeviceClaimError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<FinalizeDeviceClaimError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(FinalizeDeviceClaimError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(FinalizeDeviceClaimError::InvalidRequest(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(FinalizeDeviceClaimError::PreconditionFailed(
                        err.msg,
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(FinalizeDeviceClaimError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(FinalizeDeviceClaimError::ResourceNotFound(
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
        }
    }
}
/// Errors returned by GetDeviceMethods
#[derive(Debug, PartialEq)]
pub enum GetDeviceMethodsError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
}

impl GetDeviceMethodsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeviceMethodsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetDeviceMethodsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetDeviceMethodsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDeviceMethodsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl InitiateDeviceClaimError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InitiateDeviceClaimError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(InitiateDeviceClaimError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(InitiateDeviceClaimError::InvalidRequest(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(InitiateDeviceClaimError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InitiateDeviceClaimError::ResourceNotFound(
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
}

impl InvokeDeviceMethodError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InvokeDeviceMethodError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(InvokeDeviceMethodError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(InvokeDeviceMethodError::InvalidRequest(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(InvokeDeviceMethodError::PreconditionFailed(
                        err.msg,
                    ))
                }
                "RangeNotSatisfiableException" => {
                    return RusotoError::Service(InvokeDeviceMethodError::RangeNotSatisfiable(
                        err.msg,
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(InvokeDeviceMethodError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InvokeDeviceMethodError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListDeviceEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeviceEventsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDeviceEventsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDeviceEventsError::InvalidRequest(err.msg))
                }
                "RangeNotSatisfiableException" => {
                    return RusotoError::Service(ListDeviceEventsError::RangeNotSatisfiable(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDeviceEventsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListDevices
#[derive(Debug, PartialEq)]
pub enum ListDevicesError {
    InternalFailure(String),

    InvalidRequest(String),

    RangeNotSatisfiable(String),
}

impl ListDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDevicesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDevicesError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDevicesError::InvalidRequest(err.msg))
                }
                "RangeNotSatisfiableException" => {
                    return RusotoError::Service(ListDevicesError::RangeNotSatisfiable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    InternalFailure(String),

    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalFailure(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::InternalFailure(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(TagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::InternalFailure(ref cause) => cause,
            TagResourceError::InvalidRequest(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UnclaimDevice
#[derive(Debug, PartialEq)]
pub enum UnclaimDeviceError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
}

impl UnclaimDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnclaimDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UnclaimDeviceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UnclaimDeviceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UnclaimDeviceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::InternalFailure(ref cause) => cause,
            UntagResourceError::InvalidRequest(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeviceState
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceStateError {
    InternalFailure(String),

    InvalidRequest(String),

    ResourceNotFound(String),
}

impl UpdateDeviceStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeviceStateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDeviceStateError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateDeviceStateError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDeviceStateError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Trait representing the capabilities of the AWS IoT 1-Click Devices Service API. AWS IoT 1-Click Devices Service clients implement this trait.
pub trait Iot1ClickDevices: region::GetRegion {
    /// <p>Adds device(s) to your account (i.e., claim one or more devices) if and only if you
    /// received a claim code with the device(s).</p>
    fn claim_devices_by_claim_code(
        &self,
        input: ClaimDevicesByClaimCodeRequest,
    ) -> RusotoFuture<ClaimDevicesByClaimCodeResponse, ClaimDevicesByClaimCodeError>;

    /// <p>Given a device ID, returns a DescribeDeviceResponse object describing the
    /// details of the device.</p>
    fn describe_device(
        &self,
        input: DescribeDeviceRequest,
    ) -> RusotoFuture<DescribeDeviceResponse, DescribeDeviceError>;

    /// <p>Given a device ID, finalizes the claim request for the associated device.</p><note>
    /// <p>Claiming a device consists of initiating a claim, then publishing a device event,
    /// and finalizing the claim. For a device of type button, a device event can
    /// be published by simply clicking the device.</p>
    ///
    /// <p></note></p>
    fn finalize_device_claim(
        &self,
        input: FinalizeDeviceClaimRequest,
    ) -> RusotoFuture<FinalizeDeviceClaimResponse, FinalizeDeviceClaimError>;

    /// <p>Given a device ID, returns the invokable methods associated with the device.</p>
    fn get_device_methods(
        &self,
        input: GetDeviceMethodsRequest,
    ) -> RusotoFuture<GetDeviceMethodsResponse, GetDeviceMethodsError>;

    /// <p>Given a device ID, initiates a claim request for the associated device.</p><note>
    /// <p>Claiming a device consists of initiating a claim, then publishing a device event,
    /// and finalizing the claim. For a device of type button, a device event can
    /// be published by simply clicking the device.</p>
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

    /// <p>Using a device ID, returns a DeviceEventsResponse object containing an
    /// array of events for the device.</p>
    fn list_device_events(
        &self,
        input: ListDeviceEventsRequest,
    ) -> RusotoFuture<ListDeviceEventsResponse, ListDeviceEventsError>;

    /// <p>Lists the 1-Click compatible devices associated with your AWS account.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResponse, ListDevicesError>;

    /// <p>Lists the tags associated with the specified resource ARN.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Adds or updates the tags associated with the resource ARN. See <a href="https://docs.aws.amazon.com/iot-1-click/latest/developerguide/1click-appendix.html#1click-limits">AWS IoT 1-Click Service Limits</a> for the maximum number of tags allowed per
    /// resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Disassociates a device from your AWS account using its device ID.</p>
    fn unclaim_device(
        &self,
        input: UnclaimDeviceRequest,
    ) -> RusotoFuture<UnclaimDeviceResponse, UnclaimDeviceError>;

    /// <p>Using tag keys, deletes the tags (key/value pairs) associated with the specified
    /// resource ARN.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

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
        Self::new_with_client(Client::shared(), region)
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
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> Iot1ClickDevicesClient {
        Iot1ClickDevicesClient { client, region }
    }
}

impl fmt::Debug for Iot1ClickDevicesClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iot1ClickDevicesClient")
            .field("region", &self.region)
            .finish()
    }
}

impl region::GetRegion for Iot1ClickDevicesClient {
    fn region(&self) -> &region::Region {
        &self.region
    }
}

impl Iot1ClickDevices for Iot1ClickDevicesClient {
    /// <p>Adds device(s) to your account (i.e., claim one or more devices) if and only if you
    /// received a claim code with the device(s).</p>
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ClaimDevicesByClaimCodeResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ClaimDevicesByClaimCodeError::from_response(response))
                }))
            }
        })
    }

    /// <p>Given a device ID, returns a DescribeDeviceResponse object describing the
    /// details of the device.</p>
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDeviceResponse, _>()?;

                    Ok(result)
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
    /// <p>Claiming a device consists of initiating a claim, then publishing a device event,
    /// and finalizing the claim. For a device of type button, a device event can
    /// be published by simply clicking the device.</p>
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
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<FinalizeDeviceClaimResponse, _>()?;

                    Ok(result)
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

    /// <p>Given a device ID, returns the invokable methods associated with the device.</p>
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDeviceMethodsResponse, _>()?;

                    Ok(result)
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
    /// <p>Claiming a device consists of initiating a claim, then publishing a device event,
    /// and finalizing the claim. For a device of type button, a device event can
    /// be published by simply clicking the device.</p>
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<InitiateDeviceClaimResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<InvokeDeviceMethodResponse, _>()?;

                    Ok(result)
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

    /// <p>Using a device ID, returns a DeviceEventsResponse object containing an
    /// array of events for the device.</p>
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDeviceEventsResponse, _>()?;

                    Ok(result)
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDevicesResponse, _>()?;

                    Ok(result)
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

    /// <p>Lists the tags associated with the specified resource ARN.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds or updates the tags associated with the resource ARN. See <a href="https://docs.aws.amazon.com/iot-1-click/latest/developerguide/1click-appendix.html#1click-limits">AWS IoT 1-Click Service Limits</a> for the maximum number of tags allowed per
    /// resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UnclaimDeviceResponse, _>()?;

                    Ok(result)
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

    /// <p>Using tag keys, deletes the tags (key/value pairs) associated with the specified
    /// resource ARN.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "iot1click", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("devices.iot1click".to_string());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
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
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDeviceStateResponse, _>()?;

                    Ok(result)
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
