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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateOutpostInput {
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SiteId")]
    pub site_id: String,
    /// <p>The tags to apply to the Outpost.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOutpostOutput {
    #[serde(rename = "Outpost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost: Option<Outpost>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOutpostInput {
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteOutpostOutput {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSiteInput {
    #[serde(rename = "SiteId")]
    pub site_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSiteOutput {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOutpostInput {
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOutpostInstanceTypesInput {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostId")]
    pub outpost_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOutpostInstanceTypesOutput {
    #[serde(rename = "InstanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<InstanceTypeItem>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOutpostOutput {
    #[serde(rename = "Outpost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost: Option<Outpost>,
}

/// <p>Information about an instance type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceTypeItem {
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOutpostsInput {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOutpostsOutput {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Outposts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outposts: Option<Vec<Outpost>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSitesInput {
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSitesOutput {
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<Site>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The resource tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about an Outpost.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Outpost {
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LifeCycleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_status: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// <p>The Outpost tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Information about a site.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Site {
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// <p>The site tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by CreateOutpost
#[derive(Debug, PartialEq)]
pub enum CreateOutpostError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
    /// <p>You have exceeded a service quota.</p>
    ServiceQuotaExceeded(String),
}

impl CreateOutpostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOutpostError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateOutpostError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateOutpostError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateOutpostError::NotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateOutpostError::ServiceQuotaExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateOutpostError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOutpostError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateOutpostError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateOutpostError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateOutpostError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateOutpostError {}
/// Errors returned by DeleteOutpost
#[derive(Debug, PartialEq)]
pub enum DeleteOutpostError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl DeleteOutpostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOutpostError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteOutpostError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteOutpostError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteOutpostError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteOutpostError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOutpostError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteOutpostError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteOutpostError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteOutpostError {}
/// Errors returned by DeleteSite
#[derive(Debug, PartialEq)]
pub enum DeleteSiteError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl DeleteSiteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSiteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteSiteError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteSiteError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteSiteError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSiteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSiteError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteSiteError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteSiteError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSiteError {}
/// Errors returned by GetOutpost
#[derive(Debug, PartialEq)]
pub enum GetOutpostError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl GetOutpostError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOutpostError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOutpostError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetOutpostError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOutpostError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOutpostError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOutpostError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOutpostError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetOutpostError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOutpostError {}
/// Errors returned by GetOutpostInstanceTypes
#[derive(Debug, PartialEq)]
pub enum GetOutpostInstanceTypesError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl GetOutpostInstanceTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOutpostInstanceTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOutpostInstanceTypesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetOutpostInstanceTypesError::InternalServer(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOutpostInstanceTypesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOutpostInstanceTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOutpostInstanceTypesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetOutpostInstanceTypesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetOutpostInstanceTypesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOutpostInstanceTypesError {}
/// Errors returned by ListOutposts
#[derive(Debug, PartialEq)]
pub enum ListOutpostsError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
}

impl ListOutpostsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOutpostsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListOutpostsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListOutpostsError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOutpostsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOutpostsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListOutpostsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOutpostsError {}
/// Errors returned by ListSites
#[derive(Debug, PartialEq)]
pub enum ListSitesError {
    /// <p>You do not have permission to perform this operation.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
}

impl ListSitesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSitesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListSitesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListSitesError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSitesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSitesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListSitesError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSitesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The specified request is not valid.</p>
    NotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Outposts API. Outposts clients implement this trait.
#[async_trait]
pub trait Outposts {
    /// <p>Creates an Outpost.</p>
    async fn create_outpost(
        &self,
        input: CreateOutpostInput,
    ) -> Result<CreateOutpostOutput, RusotoError<CreateOutpostError>>;

    /// <p>Deletes the Outpost.</p>
    async fn delete_outpost(
        &self,
        input: DeleteOutpostInput,
    ) -> Result<DeleteOutpostOutput, RusotoError<DeleteOutpostError>>;

    /// <p>Deletes the site.</p>
    async fn delete_site(
        &self,
        input: DeleteSiteInput,
    ) -> Result<DeleteSiteOutput, RusotoError<DeleteSiteError>>;

    /// <p>Gets information about the specified Outpost.</p>
    async fn get_outpost(
        &self,
        input: GetOutpostInput,
    ) -> Result<GetOutpostOutput, RusotoError<GetOutpostError>>;

    /// <p>Lists the instance types for the specified Outpost.</p>
    async fn get_outpost_instance_types(
        &self,
        input: GetOutpostInstanceTypesInput,
    ) -> Result<GetOutpostInstanceTypesOutput, RusotoError<GetOutpostInstanceTypesError>>;

    /// <p>List the Outposts for your AWS account.</p>
    async fn list_outposts(
        &self,
        input: ListOutpostsInput,
    ) -> Result<ListOutpostsOutput, RusotoError<ListOutpostsError>>;

    /// <p>Lists the sites for the specified AWS account.</p>
    async fn list_sites(
        &self,
        input: ListSitesInput,
    ) -> Result<ListSitesOutput, RusotoError<ListSitesError>>;

    /// <p>Lists the tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds tags to the specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the Outposts API.
#[derive(Clone)]
pub struct OutpostsClient {
    client: Client,
    region: region::Region,
}

impl OutpostsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> OutpostsClient {
        OutpostsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> OutpostsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        OutpostsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> OutpostsClient {
        OutpostsClient { client, region }
    }
}

#[async_trait]
impl Outposts for OutpostsClient {
    /// <p>Creates an Outpost.</p>
    #[allow(unused_mut)]
    async fn create_outpost(
        &self,
        input: CreateOutpostInput,
    ) -> Result<CreateOutpostOutput, RusotoError<CreateOutpostError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/outposts";

        let mut request = SignedRequest::new("POST", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<CreateOutpostOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateOutpostError::from_response(response))
        }
    }

    /// <p>Deletes the Outpost.</p>
    #[allow(unused_mut)]
    async fn delete_outpost(
        &self,
        input: DeleteOutpostInput,
    ) -> Result<DeleteOutpostOutput, RusotoError<DeleteOutpostError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/outposts/{outpost_id}", outpost_id = input.outpost_id);

        let mut request = SignedRequest::new("DELETE", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteOutpostOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteOutpostError::from_response(response))
        }
    }

    /// <p>Deletes the site.</p>
    #[allow(unused_mut)]
    async fn delete_site(
        &self,
        input: DeleteSiteInput,
    ) -> Result<DeleteSiteOutput, RusotoError<DeleteSiteError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/sites/{site_id}", site_id = input.site_id);

        let mut request = SignedRequest::new("DELETE", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteSiteOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteSiteError::from_response(response))
        }
    }

    /// <p>Gets information about the specified Outpost.</p>
    #[allow(unused_mut)]
    async fn get_outpost(
        &self,
        input: GetOutpostInput,
    ) -> Result<GetOutpostOutput, RusotoError<GetOutpostError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/outposts/{outpost_id}", outpost_id = input.outpost_id);

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetOutpostOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetOutpostError::from_response(response))
        }
    }

    /// <p>Lists the instance types for the specified Outpost.</p>
    #[allow(unused_mut)]
    async fn get_outpost_instance_types(
        &self,
        input: GetOutpostInstanceTypesInput,
    ) -> Result<GetOutpostInstanceTypesOutput, RusotoError<GetOutpostInstanceTypesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/outposts/{outpost_id}/instanceTypes",
            outpost_id = input.outpost_id
        );

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
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
                .deserialize::<GetOutpostInstanceTypesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetOutpostInstanceTypesError::from_response(response))
        }
    }

    /// <p>List the Outposts for your AWS account.</p>
    #[allow(unused_mut)]
    async fn list_outposts(
        &self,
        input: ListOutpostsInput,
    ) -> Result<ListOutpostsOutput, RusotoError<ListOutpostsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/outposts";

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
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
                .deserialize::<ListOutpostsOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListOutpostsError::from_response(response))
        }
    }

    /// <p>Lists the sites for the specified AWS account.</p>
    #[allow(unused_mut)]
    async fn list_sites(
        &self,
        input: ListSitesInput,
    ) -> Result<ListSitesOutput, RusotoError<ListSitesError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/sites";

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<ListSitesOutput, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSitesError::from_response(response))
        }
    }

    /// <p>Lists the tags for the specified resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Adds tags to the specified resource.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

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
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from the specified resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "outposts", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
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
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }
}
