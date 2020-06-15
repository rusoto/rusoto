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
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelChangeSetRequest {
    /// <p>Required. The catalog related to the request. Fixed value: <code>AWSMarketplace</code>.</p>
    #[serde(rename = "Catalog")]
    pub catalog: String,
    /// <p>Required. The unique identifier of the <code>StartChangeSet</code> request that you want to cancel.</p>
    #[serde(rename = "ChangeSetId")]
    pub change_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelChangeSetResponse {
    /// <p>The ARN associated with the change set referenced in this request.</p>
    #[serde(rename = "ChangeSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_arn: Option<String>,
    /// <p>The unique identifier for the change set referenced in this request.</p>
    #[serde(rename = "ChangeSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
}

/// <p>An object that contains the <code>ChangeType</code>, <code>Details</code>, and <code>Entity</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Change {
    /// <p>Change types are single string values that describe your intention for the change. Each change type is unique for each <code>EntityType</code> provided in the change's scope.</p>
    #[serde(rename = "ChangeType")]
    pub change_type: String,
    /// <p>This object contains details specific to the change type of the requested change.</p>
    #[serde(rename = "Details")]
    pub details: String,
    /// <p>The entity to be changed.</p>
    #[serde(rename = "Entity")]
    pub entity: Entity,
}

/// <p>A summary of a change set returned in a list of change sets when the <code>ListChangeSets</code> action is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChangeSetSummaryListItem {
    /// <p>The ARN associated with the unique identifier for the change set referenced in this request.</p>
    #[serde(rename = "ChangeSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_arn: Option<String>,
    /// <p>The unique identifier for a change set.</p>
    #[serde(rename = "ChangeSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    /// <p>The non-unique name for the change set.</p>
    #[serde(rename = "ChangeSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    /// <p>The time, in ISO 8601 format (2018-02-27T13:45:22Z), when the change set was finished.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// <p>This object is a list of entity IDs (string) that are a part of a change set. The entity ID list is a maximum of 20 entities. It must contain at least one entity.</p>
    #[serde(rename = "EntityIdList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id_list: Option<Vec<String>>,
    /// <p>The time, in ISO 8601 format (2018-02-27T13:45:22Z), when the change set was started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>The current status of the change set.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>This object is a container for common summary information about the change. The summary doesn't contain the whole change structure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChangeSummary {
    /// <p>The type of the change.</p>
    #[serde(rename = "ChangeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    /// <p>The entity to be changed.</p>
    #[serde(rename = "Entity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
    /// <p>An array of <code>ErrorDetail</code> objects associated with the change.</p>
    #[serde(rename = "ErrorDetailList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail_list: Option<Vec<ErrorDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeChangeSetRequest {
    /// <p>Required. The catalog related to the request. Fixed value: <code>AWSMarketplace</code> </p>
    #[serde(rename = "Catalog")]
    pub catalog: String,
    /// <p>Required. The unique identifier for the <code>StartChangeSet</code> request that you want to describe the details for.</p>
    #[serde(rename = "ChangeSetId")]
    pub change_set_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeChangeSetResponse {
    /// <p>An array of <code>ChangeSummary</code> objects.</p>
    #[serde(rename = "ChangeSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set: Option<Vec<ChangeSummary>>,
    /// <p>The ARN associated with the unique identifier for the change set referenced in this request.</p>
    #[serde(rename = "ChangeSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_arn: Option<String>,
    /// <p>Required. The unique identifier for the change set referenced in this request.</p>
    #[serde(rename = "ChangeSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    /// <p>The optional name provided in the <code>StartChangeSet</code> request. If you do not provide a name, one is set by default.</p>
    #[serde(rename = "ChangeSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request transitioned to a terminal state. The change cannot transition to a different state. Null if the request is not in a terminal state. </p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// <p>Returned if there is a failure on the change set, but that failure is not related to any of the changes in the request.</p>
    #[serde(rename = "FailureDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<String>,
    /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request started. </p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>The status of the change request.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEntityRequest {
    /// <p>Required. The catalog related to the request. Fixed value: <code>AWSMarketplace</code> </p>
    #[serde(rename = "Catalog")]
    pub catalog: String,
    /// <p>Required. The unique ID of the entity to describe.</p>
    #[serde(rename = "EntityId")]
    pub entity_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEntityResponse {
    /// <p>This stringified JSON object includes the details of the entity.</p>
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The ARN associated to the unique identifier for the change set referenced in this request.</p>
    #[serde(rename = "EntityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_arn: Option<String>,
    /// <p>The identifier of the entity, in the format of <code>EntityId@RevisionId</code>.</p>
    #[serde(rename = "EntityIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_identifier: Option<String>,
    /// <p>The named type of the entity, in the format of <code>EntityType@Version</code>.</p>
    #[serde(rename = "EntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// <p>The last modified date of the entity, in ISO 8601 format (2018-02-27T13:45:22Z).</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
}

/// <p>A product entity contains data that describes your product, its supported features, and how it can be used or launched by your customer. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    /// <p>The identifier for the entity.</p>
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// <p>The type of entity.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>This object is a container for common summary information about the entity. The summary doesn't contain the whole entity structure, but it does contain information common across all entities.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntitySummary {
    /// <p>The ARN associated with the unique identifier for the entity.</p>
    #[serde(rename = "EntityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_arn: Option<String>,
    /// <p>The unique identifier for the entity.</p>
    #[serde(rename = "EntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// <p>The type of the entity.</p>
    #[serde(rename = "EntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// <p>The last time the entity was published, using ISO 8601 format (2018-02-27T13:45:22Z).</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// <p>The name for the entity. This value is not unique. It is defined by the provider.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The visibility status of the entity to subscribers. This value can be <code>Public</code> (everyone can view the entity), <code>Limited</code> (the entity is visible to limited accounts only), or <code>Restricted</code> (the entity was published and then unpublished and only existing subscribers can view it). </p>
    #[serde(rename = "Visibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// <p>Details about the error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ErrorDetail {
    /// <p>The error code that identifies the type of error.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The message for the error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// <p>A filter object, used to optionally filter results from calls to the <code>ListEntities</code> and <code>ListChangeSets</code> actions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>For <code>ListEntities</code>, the supported value for this is an <code>EntityId</code>.</p> <p>For <code>ListChangeSets</code>, the supported values are as follows:</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p> <code>ListEntities</code> - This is a list of unique <code>EntityId</code>s.</p> <p> <code>ListChangeSets</code> - The supported filter names and associated <code>ValueList</code>s is as follows:</p> <ul> <li> <p> <code>ChangeSetName</code> - The supported <code>ValueList</code> is a list of non-unique <code>ChangeSetName</code>s. These are defined when you call the <code>StartChangeSet</code> action.</p> </li> <li> <p> <code>Status</code> - The supported <code>ValueList</code> is a list of statuses for all change set requests.</p> </li> <li> <p> <code>EntityId</code> - The supported <code>ValueList</code> is a list of unique <code>EntityId</code>s.</p> </li> <li> <p> <code>BeforeStartTime</code> - The supported <code>ValueList</code> is a list of all change sets that started before the filter value.</p> </li> <li> <p> <code>AfterStartTime</code> - The supported <code>ValueList</code> is a list of all change sets that started after the filter value.</p> </li> <li> <p> <code>BeforeEndTime</code> - The supported <code>ValueList</code> is a list of all change sets that ended before the filter value.</p> </li> <li> <p> <code>AfterEndTime</code> - The supported <code>ValueList</code> is a list of all change sets that ended after the filter value.</p> </li> </ul></p>
    #[serde(rename = "ValueList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListChangeSetsRequest {
    /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code> </p>
    #[serde(rename = "Catalog")]
    pub catalog: String,
    /// <p>An array of filter objects.</p>
    #[serde(rename = "FilterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_list: Option<Vec<Filter>>,
    /// <p>The maximum number of results returned by a single call. This value must be provided in the next call to retrieve the next set of results. By default, this value is 20.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An object that contains two attributes, <code>sortBy</code> and <code>sortOrder</code>.</p>
    #[serde(rename = "Sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListChangeSetsResponse {
    /// <p> Array of <code>ChangeSetSummaryListItem</code> objects.</p>
    #[serde(rename = "ChangeSetSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_summary_list: Option<Vec<ChangeSetSummaryListItem>>,
    /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEntitiesRequest {
    /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code> </p>
    #[serde(rename = "Catalog")]
    pub catalog: String,
    /// <p>The type of entities to retrieve.</p>
    #[serde(rename = "EntityType")]
    pub entity_type: String,
    /// <p>An array of filter objects. Each filter object contains two attributes, <code>filterName</code> and <code>filterValues</code>.</p>
    #[serde(rename = "FilterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_list: Option<Vec<Filter>>,
    /// <p>Specifies the upper limit of the elements on a single page. If a value isn't provided, the default value is 20.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An object that contains two attributes, <code>sortBy</code> and <code>sortOrder</code>.</p>
    #[serde(rename = "Sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEntitiesResponse {
    /// <p> Array of <code>EntitySummary</code> object.</p>
    #[serde(rename = "EntitySummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_summary_list: Option<Vec<EntitySummary>>,
    /// <p>The value of the next token if it exists. Null if there is no more result.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>An object that contains two attributes, <code>sortBy</code> and <code>sortOrder</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Sort {
    /// <p>For <code>ListEntities</code>, supported attributes include <code>LastModifiedDate</code> (default), <code>Visibility</code>, <code>EntityId</code>, and <code>Name</code>.</p> <p>For <code>ListChangeSets</code>, supported attributes include <code>StartTime</code> and <code>EndTime</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// <p>The sorting order. Can be <code>ASCENDING</code> or <code>DESCENDING</code>. The default value is <code>DESCENDING</code>.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartChangeSetRequest {
    /// <p>The catalog related to the request. Fixed value: <code>AWSMarketplace</code> </p>
    #[serde(rename = "Catalog")]
    pub catalog: String,
    /// <p>Array of <code>change</code> object.</p>
    #[serde(rename = "ChangeSet")]
    pub change_set: Vec<Change>,
    /// <p>Optional case sensitive string of up to 100 ASCII characters. The change set name can be used to filter the list of change sets. </p>
    #[serde(rename = "ChangeSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    /// <p>A unique token to identify the request to ensure idempotency.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartChangeSetResponse {
    /// <p>The ARN associated to the unique identifier generated for the request.</p>
    #[serde(rename = "ChangeSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_arn: Option<String>,
    /// <p>Unique identifier generated for the request.</p>
    #[serde(rename = "ChangeSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
}

/// Errors returned by CancelChangeSet
#[derive(Debug, PartialEq)]
pub enum CancelChangeSetError {
    /// <p>Access is denied.</p>
    AccessDenied(String),
    /// <p>There was an internal service exception.</p>
    InternalService(String),
    /// <p>The resource is currently in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource wasn't found.</p>
    ResourceNotFound(String),
    /// <p>Too many requests.</p>
    Throttling(String),
}

impl CancelChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelChangeSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CancelChangeSetError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CancelChangeSetError::InternalService(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CancelChangeSetError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelChangeSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelChangeSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelChangeSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelChangeSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CancelChangeSetError::InternalService(ref cause) => write!(f, "{}", cause),
            CancelChangeSetError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CancelChangeSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CancelChangeSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelChangeSetError {}
/// Errors returned by DescribeChangeSet
#[derive(Debug, PartialEq)]
pub enum DescribeChangeSetError {
    /// <p>Access is denied.</p>
    AccessDenied(String),
    /// <p>There was an internal service exception.</p>
    InternalService(String),
    /// <p>The specified resource wasn't found.</p>
    ResourceNotFound(String),
    /// <p>Too many requests.</p>
    Throttling(String),
}

impl DescribeChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeChangeSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeChangeSetError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeChangeSetError::InternalService(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeChangeSetError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeChangeSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeChangeSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeChangeSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeChangeSetError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeChangeSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeChangeSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeChangeSetError {}
/// Errors returned by DescribeEntity
#[derive(Debug, PartialEq)]
pub enum DescribeEntityError {
    /// <p>Access is denied.</p>
    AccessDenied(String),
    /// <p>There was an internal service exception.</p>
    InternalService(String),
    /// <p>The specified resource wasn't found.</p>
    ResourceNotFound(String),
    /// <p>Currently, the specified resource is not supported.</p>
    ResourceNotSupported(String),
    /// <p>Too many requests.</p>
    Throttling(String),
}

impl DescribeEntityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEntityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeEntityError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeEntityError::InternalService(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEntityError::ResourceNotFound(err.msg))
                }
                "ResourceNotSupportedException" => {
                    return RusotoError::Service(DescribeEntityError::ResourceNotSupported(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeEntityError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEntityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEntityError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeEntityError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeEntityError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeEntityError::ResourceNotSupported(ref cause) => write!(f, "{}", cause),
            DescribeEntityError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEntityError {}
/// Errors returned by ListChangeSets
#[derive(Debug, PartialEq)]
pub enum ListChangeSetsError {
    /// <p>Access is denied.</p>
    AccessDenied(String),
    /// <p>There was an internal service exception.</p>
    InternalService(String),
    /// <p>Too many requests.</p>
    Throttling(String),
}

impl ListChangeSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChangeSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListChangeSetsError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListChangeSetsError::InternalService(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListChangeSetsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListChangeSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListChangeSetsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListChangeSetsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListChangeSetsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListChangeSetsError {}
/// Errors returned by ListEntities
#[derive(Debug, PartialEq)]
pub enum ListEntitiesError {
    /// <p>Access is denied.</p>
    AccessDenied(String),
    /// <p>There was an internal service exception.</p>
    InternalService(String),
    /// <p>The specified resource wasn't found.</p>
    ResourceNotFound(String),
    /// <p>Too many requests.</p>
    Throttling(String),
}

impl ListEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEntitiesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListEntitiesError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ListEntitiesError::InternalService(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListEntitiesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListEntitiesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEntitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEntitiesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListEntitiesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListEntitiesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListEntitiesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEntitiesError {}
/// Errors returned by StartChangeSet
#[derive(Debug, PartialEq)]
pub enum StartChangeSetError {
    /// <p>Access is denied.</p>
    AccessDenied(String),
    /// <p>There was an internal service exception.</p>
    InternalService(String),
    /// <p>The resource is currently in use.</p>
    ResourceInUse(String),
    /// <p>The specified resource wasn't found.</p>
    ResourceNotFound(String),
    /// <p>The maximum number of open requests per account has been exceeded.</p>
    ServiceQuotaExceeded(String),
    /// <p>Too many requests.</p>
    Throttling(String),
}

impl StartChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartChangeSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartChangeSetError::AccessDenied(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartChangeSetError::InternalService(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartChangeSetError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartChangeSetError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(StartChangeSetError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartChangeSetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartChangeSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartChangeSetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartChangeSetError::InternalService(ref cause) => write!(f, "{}", cause),
            StartChangeSetError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartChangeSetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartChangeSetError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            StartChangeSetError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartChangeSetError {}
/// Trait representing the capabilities of the AWS Marketplace Catalog API. AWS Marketplace Catalog clients implement this trait.
#[async_trait]
pub trait MarketplaceCatalog {
    /// <p>Used to cancel an open change request. Must be sent before the status of the request changes to <code>APPLYING</code>, the final stage of completing your change request. You can describe a change during the 60-day request history retention period for API calls.</p>
    async fn cancel_change_set(
        &self,
        input: CancelChangeSetRequest,
    ) -> Result<CancelChangeSetResponse, RusotoError<CancelChangeSetError>>;

    /// <p>Provides information about a given change set.</p>
    async fn describe_change_set(
        &self,
        input: DescribeChangeSetRequest,
    ) -> Result<DescribeChangeSetResponse, RusotoError<DescribeChangeSetError>>;

    /// <p>Returns the metadata and content of the entity.</p>
    async fn describe_entity(
        &self,
        input: DescribeEntityRequest,
    ) -> Result<DescribeEntityResponse, RusotoError<DescribeEntityError>>;

    /// <p>Returns the list of change sets owned by the account being used to make the call. You can filter this list by providing any combination of <code>entityId</code>, <code>ChangeSetName</code>, and status. If you provide more than one filter, the API operation applies a logical AND between the filters.</p> <p>You can describe a change during the 60-day request history retention period for API calls.</p>
    async fn list_change_sets(
        &self,
        input: ListChangeSetsRequest,
    ) -> Result<ListChangeSetsResponse, RusotoError<ListChangeSetsError>>;

    /// <p>Provides the list of entities of a given type.</p>
    async fn list_entities(
        &self,
        input: ListEntitiesRequest,
    ) -> Result<ListEntitiesResponse, RusotoError<ListEntitiesError>>;

    /// <p>This operation allows you to request changes in your entities.</p>
    async fn start_change_set(
        &self,
        input: StartChangeSetRequest,
    ) -> Result<StartChangeSetResponse, RusotoError<StartChangeSetError>>;
}
/// A client for the AWS Marketplace Catalog API.
#[derive(Clone)]
pub struct MarketplaceCatalogClient {
    client: Client,
    region: region::Region,
}

impl MarketplaceCatalogClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MarketplaceCatalogClient {
        MarketplaceCatalogClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MarketplaceCatalogClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MarketplaceCatalogClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MarketplaceCatalogClient {
        MarketplaceCatalogClient { client, region }
    }
}

#[async_trait]
impl MarketplaceCatalog for MarketplaceCatalogClient {
    /// <p>Used to cancel an open change request. Must be sent before the status of the request changes to <code>APPLYING</code>, the final stage of completing your change request. You can describe a change during the 60-day request history retention period for API calls.</p>
    #[allow(unused_mut)]
    async fn cancel_change_set(
        &self,
        input: CancelChangeSetRequest,
    ) -> Result<CancelChangeSetResponse, RusotoError<CancelChangeSetError>> {
        let request_uri = "/CancelChangeSet";

        let mut request =
            SignedRequest::new("PATCH", "aws-marketplace", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("catalog.marketplace".to_string());

        let mut params = Params::new();
        params.put("catalog", &input.catalog);
        params.put("changeSetId", &input.change_set_id);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelChangeSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelChangeSetError::from_response(response))
        }
    }

    /// <p>Provides information about a given change set.</p>
    #[allow(unused_mut)]
    async fn describe_change_set(
        &self,
        input: DescribeChangeSetRequest,
    ) -> Result<DescribeChangeSetResponse, RusotoError<DescribeChangeSetError>> {
        let request_uri = "/DescribeChangeSet";

        let mut request = SignedRequest::new("GET", "aws-marketplace", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("catalog.marketplace".to_string());

        let mut params = Params::new();
        params.put("catalog", &input.catalog);
        params.put("changeSetId", &input.change_set_id);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeChangeSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeChangeSetError::from_response(response))
        }
    }

    /// <p>Returns the metadata and content of the entity.</p>
    #[allow(unused_mut)]
    async fn describe_entity(
        &self,
        input: DescribeEntityRequest,
    ) -> Result<DescribeEntityResponse, RusotoError<DescribeEntityError>> {
        let request_uri = "/DescribeEntity";

        let mut request = SignedRequest::new("GET", "aws-marketplace", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("catalog.marketplace".to_string());

        let mut params = Params::new();
        params.put("catalog", &input.catalog);
        params.put("entityId", &input.entity_id);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeEntityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEntityError::from_response(response))
        }
    }

    /// <p>Returns the list of change sets owned by the account being used to make the call. You can filter this list by providing any combination of <code>entityId</code>, <code>ChangeSetName</code>, and status. If you provide more than one filter, the API operation applies a logical AND between the filters.</p> <p>You can describe a change during the 60-day request history retention period for API calls.</p>
    #[allow(unused_mut)]
    async fn list_change_sets(
        &self,
        input: ListChangeSetsRequest,
    ) -> Result<ListChangeSetsResponse, RusotoError<ListChangeSetsError>> {
        let request_uri = "/ListChangeSets";

        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("catalog.marketplace".to_string());
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
                .deserialize::<ListChangeSetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListChangeSetsError::from_response(response))
        }
    }

    /// <p>Provides the list of entities of a given type.</p>
    #[allow(unused_mut)]
    async fn list_entities(
        &self,
        input: ListEntitiesRequest,
    ) -> Result<ListEntitiesResponse, RusotoError<ListEntitiesError>> {
        let request_uri = "/ListEntities";

        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("catalog.marketplace".to_string());
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
                .deserialize::<ListEntitiesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEntitiesError::from_response(response))
        }
    }

    /// <p>This operation allows you to request changes in your entities.</p>
    #[allow(unused_mut)]
    async fn start_change_set(
        &self,
        input: StartChangeSetRequest,
    ) -> Result<StartChangeSetResponse, RusotoError<StartChangeSetError>> {
        let request_uri = "/StartChangeSet";

        let mut request = SignedRequest::new("POST", "aws-marketplace", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("catalog.marketplace".to_string());
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
                .deserialize::<StartChangeSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartChangeSetError::from_response(response))
        }
    }
}
