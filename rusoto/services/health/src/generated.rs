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
/// <p>Information about an entity that is affected by a Health event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AffectedEntity {
    /// <p>The 12-digit AWS account number that contains the affected entity.</p>
    #[serde(rename = "awsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The unique identifier for the entity. Format: <code>arn:aws:health:<i>entity-region</i>:<i>aws-account</i>:entity/<i>entity-id</i> </code>. Example: <code>arn:aws:health:us-east-1:111222333444:entity/AVh5GGT7ul1arKr1sE1K</code> </p>
    #[serde(rename = "entityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_arn: Option<String>,
    /// <p>The URL of the affected entity.</p>
    #[serde(rename = "entityUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_url: Option<String>,
    /// <p>The ID of the affected entity.</p>
    #[serde(rename = "entityValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_value: Option<String>,
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arn: Option<String>,
    /// <p>The most recent time that the entity was updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The most recent status of the entity affected by the event. The possible values are <code>IMPAIRED</code>, <code>UNIMPAIRED</code>, and <code>UNKNOWN</code>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// <p>A map of entity tags attached to the affected entity.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A range of dates and times that is used by the <a>EventFilter</a> and <a>EntityFilter</a> objects. If <code>from</code> is set and <code>to</code> is set: match items where the timestamp (<code>startTime</code>, <code>endTime</code>, or <code>lastUpdatedTime</code>) is between <code>from</code> and <code>to</code> inclusive. If <code>from</code> is set and <code>to</code> is not set: match items where the timestamp value is equal to or after <code>from</code>. If <code>from</code> is not set and <code>to</code> is set: match items where the timestamp value is equal to or before <code>to</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DateTimeRange {
    /// <p>The starting date and time of a time range.</p>
    #[serde(rename = "from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    /// <p>The ending date and time of a time range.</p>
    #[serde(rename = "to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAffectedAccountsForOrganizationRequest {
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    pub event_arn: String,
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAffectedAccountsForOrganizationResponse {
    /// <p>A JSON set of elements of the affected accounts.</p>
    #[serde(rename = "affectedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_accounts: Option<Vec<String>>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAffectedEntitiesForOrganizationRequest {
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A JSON set of elements including the <code>awsAccountId</code> and the <code>eventArn</code>.</p>
    #[serde(rename = "organizationEntityFilters")]
    pub organization_entity_filters: Vec<EventAccountFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAffectedEntitiesForOrganizationResponse {
    /// <p>A JSON set of elements including the <code>awsAccountId</code> and its <code>entityArn</code>, <code>entityValue</code> and its <code>entityArn</code>, <code>lastUpdatedTime</code>, <code>statusCode</code>, and <code>tags</code>.</p>
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<AffectedEntity>>,
    /// <p>A JSON set of elements of the failed response, including the <code>awsAccountId</code>, <code>errorMessage</code>, <code>errorName</code>, and <code>eventArn</code>.</p>
    #[serde(rename = "failedSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_set: Option<Vec<OrganizationAffectedEntitiesErrorItem>>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAffectedEntitiesRequest {
    /// <p>Values to narrow the results returned. At least one event ARN is required.</p>
    #[serde(rename = "filter")]
    pub filter: EntityFilter,
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAffectedEntitiesResponse {
    /// <p>The entities that match the filter criteria.</p>
    #[serde(rename = "entities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<AffectedEntity>>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEntityAggregatesRequest {
    /// <p>A list of event ARNs (unique identifiers). For example: <code>"arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-CDE456", "arn:aws:health:us-west-1::event/EBS/AWS_EBS_LOST_VOLUME/AWS_EBS_LOST_VOLUME_CHI789_JKL101"</code> </p>
    #[serde(rename = "eventArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEntityAggregatesResponse {
    /// <p>The number of entities that are affected by each of the specified events.</p>
    #[serde(rename = "entityAggregates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_aggregates: Option<Vec<EntityAggregate>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventAggregatesRequest {
    /// <p>The only currently supported value is <code>eventTypeCategory</code>.</p>
    #[serde(rename = "aggregateField")]
    pub aggregate_field: String,
    /// <p>Values to narrow the results returned.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventFilter>,
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventAggregatesResponse {
    /// <p>The number of events in each category that meet the optional filter criteria.</p>
    #[serde(rename = "eventAggregates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_aggregates: Option<Vec<EventAggregate>>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventDetailsForOrganizationRequest {
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>A set of JSON elements that includes the <code>awsAccountId</code> and the <code>eventArn</code>.</p>
    #[serde(rename = "organizationEventDetailFilters")]
    pub organization_event_detail_filters: Vec<EventAccountFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventDetailsForOrganizationResponse {
    /// <p>Error messages for any events that could not be retrieved.</p>
    #[serde(rename = "failedSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_set: Option<Vec<OrganizationEventDetailsErrorItem>>,
    /// <p>Information about the events that could be retrieved.</p>
    #[serde(rename = "successfulSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_set: Option<Vec<OrganizationEventDetails>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventDetailsRequest {
    /// <p>A list of event ARNs (unique identifiers). For example: <code>"arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-CDE456", "arn:aws:health:us-west-1::event/EBS/AWS_EBS_LOST_VOLUME/AWS_EBS_LOST_VOLUME_CHI789_JKL101"</code> </p>
    #[serde(rename = "eventArns")]
    pub event_arns: Vec<String>,
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventDetailsResponse {
    /// <p>Error messages for any events that could not be retrieved.</p>
    #[serde(rename = "failedSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_set: Option<Vec<EventDetailsErrorItem>>,
    /// <p>Information about the events that could be retrieved.</p>
    #[serde(rename = "successfulSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_set: Option<Vec<EventDetails>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventTypesRequest {
    /// <p>Values to narrow the results returned.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventTypeFilter>,
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventTypesResponse {
    /// <p>A list of event types that match the filter criteria. Event types have a category (<code>issue</code>, <code>accountNotification</code>, or <code>scheduledChange</code>), a service (for example, <code>EC2</code>, <code>RDS</code>, <code>DATAPIPELINE</code>, <code>BILLING</code>), and a code (in the format <code>AWS_<i>SERVICE</i>_<i>DESCRIPTION</i> </code>; for example, <code>AWS_EC2_SYSTEM_MAINTENANCE_EVENT</code>).</p>
    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventsForOrganizationRequest {
    /// <p>Values to narrow the results returned.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<OrganizationEventFilter>,
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventsForOrganizationResponse {
    /// <p>The events that match the specified filter criteria.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<OrganizationEvent>>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventsRequest {
    /// <p>Values to narrow the results returned.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventFilter>,
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventsResponse {
    /// <p>The events that match the specified filter criteria.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHealthServiceStatusForOrganizationResponse {
    /// <p>Information about the status of enabling or disabling AWS Health Organizational View in your organization.</p> <p>Valid values are <code>ENABLED | DISABLED | PENDING</code>. </p>
    #[serde(rename = "healthServiceAccessStatusForOrganization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_service_access_status_for_organization: Option<String>,
}

/// <p>The number of entities that are affected by one or more events. Returned by the <a>DescribeEntityAggregates</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntityAggregate {
    /// <p>The number entities that match the criteria for the specified events.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arn: Option<String>,
}

/// <p>The values to use to filter results from the <a>DescribeAffectedEntities</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EntityFilter {
    /// <p>A list of entity ARNs (unique identifiers).</p>
    #[serde(rename = "entityArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_arns: Option<Vec<String>>,
    /// <p>A list of IDs for affected entities.</p>
    #[serde(rename = "entityValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_values: Option<Vec<String>>,
    /// <p>A list of event ARNs (unique identifiers). For example: <code>"arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-CDE456", "arn:aws:health:us-west-1::event/EBS/AWS_EBS_LOST_VOLUME/AWS_EBS_LOST_VOLUME_CHI789_JKL101"</code> </p>
    #[serde(rename = "eventArns")]
    pub event_arns: Vec<String>,
    /// <p>A list of the most recent dates and times that the entity was updated.</p>
    #[serde(rename = "lastUpdatedTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_times: Option<Vec<DateTimeRange>>,
    /// <p>A list of entity status codes (<code>IMPAIRED</code>, <code>UNIMPAIRED</code>, or <code>UNKNOWN</code>).</p>
    #[serde(rename = "statusCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_codes: Option<Vec<String>>,
    /// <p>A map of entity tags attached to the affected entity.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<::std::collections::HashMap<String, String>>>,
}

/// <p>Summary information about an AWS Health event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Event {
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The AWS Availability Zone of the event. For example, us-east-1a.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The date and time that the event ended.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The category of the event. Possible values are <code>issue</code>, <code>scheduledChange</code>, and <code>accountNotification</code>.</p>
    #[serde(rename = "eventTypeCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_category: Option<String>,
    /// <p>The unique identifier for the event type. The format is <code>AWS_<i>SERVICE</i>_<i>DESCRIPTION</i> </code>; for example, <code>AWS_EC2_SYSTEM_MAINTENANCE_EVENT</code>.</p>
    #[serde(rename = "eventTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_code: Option<String>,
    /// <p>The most recent date and time that the event was updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The AWS region name of the event.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The AWS service that is affected by the event. For example, <code>EC2</code>, <code>RDS</code>.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// <p>The date and time that the event began.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The most recent status of the event. Possible values are <code>open</code>, <code>closed</code>, and <code>upcoming</code>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p>The values used to filter results from the <a>DescribeEventDetailsForOrganization</a> and <a>DescribeAffectedEntitiesForOrganization</a> operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventAccountFilter {
    /// <p>The 12-digit AWS account numbers that contains the affected entities.</p>
    #[serde(rename = "awsAccountId")]
    pub aws_account_id: String,
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    pub event_arn: String,
}

/// <p>The number of events of each issue type. Returned by the <a>DescribeEventAggregates</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventAggregate {
    /// <p>The issue type for the associated count.</p>
    #[serde(rename = "aggregateValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_value: Option<String>,
    /// <p>The number of events of the associated issue type.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// <p>Detailed information about an event. A combination of an <a>Event</a> object, an <a>EventDescription</a> object, and additional metadata about the event. Returned by the <a>DescribeEventDetails</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventDetails {
    /// <p>Summary information about the event.</p>
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    /// <p>The most recent description of the event.</p>
    #[serde(rename = "eventDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
    /// <p>Additional metadata about the event.</p>
    #[serde(rename = "eventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Error information returned when a <a>DescribeEventDetails</a> operation cannot find a specified event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventDetailsErrorItem {
    /// <p>A message that describes the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The name of the error.</p>
    #[serde(rename = "errorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arn: Option<String>,
}

/// <p>The values to use to filter results from the <a>DescribeEvents</a> and <a>DescribeEventAggregates</a> operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventFilter {
    /// <p>A list of AWS availability zones.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>A list of dates and times that the event ended.</p>
    #[serde(rename = "endTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_times: Option<Vec<DateTimeRange>>,
    /// <p>A list of entity ARNs (unique identifiers).</p>
    #[serde(rename = "entityArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_arns: Option<Vec<String>>,
    /// <p>A list of entity identifiers, such as EC2 instance IDs (<code>i-34ab692e</code>) or EBS volumes (<code>vol-426ab23e</code>).</p>
    #[serde(rename = "entityValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_values: Option<Vec<String>>,
    /// <p>A list of event ARNs (unique identifiers). For example: <code>"arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-CDE456", "arn:aws:health:us-west-1::event/EBS/AWS_EBS_LOST_VOLUME/AWS_EBS_LOST_VOLUME_CHI789_JKL101"</code> </p>
    #[serde(rename = "eventArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arns: Option<Vec<String>>,
    /// <p>A list of event status codes.</p>
    #[serde(rename = "eventStatusCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status_codes: Option<Vec<String>>,
    /// <p>A list of event type category codes (<code>issue</code>, <code>scheduledChange</code>, or <code>accountNotification</code>).</p>
    #[serde(rename = "eventTypeCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_categories: Option<Vec<String>>,
    /// <p>A list of unique identifiers for event types. For example, <code>"AWS_EC2_SYSTEM_MAINTENANCE_EVENT","AWS_RDS_MAINTENANCE_SCHEDULED".</code> </p>
    #[serde(rename = "eventTypeCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_codes: Option<Vec<String>>,
    /// <p>A list of dates and times that the event was last updated.</p>
    #[serde(rename = "lastUpdatedTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_times: Option<Vec<DateTimeRange>>,
    /// <p>A list of AWS regions.</p>
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    /// <p>The AWS services associated with the event. For example, <code>EC2</code>, <code>RDS</code>.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// <p>A list of dates and times that the event began.</p>
    #[serde(rename = "startTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_times: Option<Vec<DateTimeRange>>,
    /// <p>A map of entity tags attached to the affected entity.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<::std::collections::HashMap<String, String>>>,
}

/// <p>The values to use to filter results from the <a>DescribeEventTypes</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventTypeFilter {
    /// <p>A list of event type category codes (<code>issue</code>, <code>scheduledChange</code>, or <code>accountNotification</code>).</p>
    #[serde(rename = "eventTypeCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_categories: Option<Vec<String>>,
    /// <p>A list of event type codes.</p>
    #[serde(rename = "eventTypeCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_codes: Option<Vec<String>>,
    /// <p>The AWS services associated with the event. For example, <code>EC2</code>, <code>RDS</code>.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

/// <p>Error information returned when a <a>DescribeAffectedEntitiesForOrganization</a> operation cannot find or process a specific entity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationAffectedEntitiesErrorItem {
    /// <p>The 12-digit AWS account numbers that contains the affected entities.</p>
    #[serde(rename = "awsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The unique identifier for the event type. The format is <code>AWS_SERVICE_DESCRIPTION</code>. For example, <code>AWS_EC2_SYSTEM_MAINTENANCE_EVENT</code>.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The name of the error.</p>
    #[serde(rename = "errorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arn: Option<String>,
}

/// <p>Summary information about an event, returned by the <a>DescribeEventsForOrganization</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationEvent {
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the event ended.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The category of the event type.</p>
    #[serde(rename = "eventTypeCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_category: Option<String>,
    /// <p>The unique identifier for the event type. The format is <code>AWS_SERVICE_DESCRIPTION</code>. For example, <code>AWS_EC2_SYSTEM_MAINTENANCE_EVENT</code>.</p>
    #[serde(rename = "eventTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_code: Option<String>,
    /// <p>The most recent date and time that the event was updated.</p>
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The AWS Region name of the event.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The AWS service that is affected by the event. For example, EC2, RDS.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// <p>The date and time that the event began.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The most recent status of the event. Possible values are <code>open</code>, <code>closed</code>, and <code>upcoming</code>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p>Detailed information about an event. A combination of an <a>Event</a> object, an <a>EventDescription</a> object, and additional metadata about the event. Returned by the <a>DescribeEventDetailsForOrganization</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationEventDetails {
    /// <p>The 12-digit AWS account numbers that contains the affected entities.</p>
    #[serde(rename = "awsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    #[serde(rename = "eventDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
    /// <p>Additional metadata about the event.</p>
    #[serde(rename = "eventMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Error information returned when a <a>DescribeEventDetailsForOrganization</a> operation cannot find a specified event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationEventDetailsErrorItem {
    /// <p>Error information returned when a <a>DescribeEventDetailsForOrganization</a> operation cannot find a specified event.</p>
    #[serde(rename = "awsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>A message that describes the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The name of the error.</p>
    #[serde(rename = "errorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arn: Option<String>,
}

/// <p>The values to filter results from the <a>DescribeEventsForOrganization</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OrganizationEventFilter {
    /// <p>A list of 12-digit AWS account numbers that contains the affected entities.</p>
    #[serde(rename = "awsAccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_ids: Option<Vec<String>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTimeRange>,
    /// <p>REPLACEME</p>
    #[serde(rename = "entityArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_arns: Option<Vec<String>>,
    /// <p>A list of entity identifiers, such as EC2 instance IDs (i-34ab692e) or EBS volumes (vol-426ab23e).</p>
    #[serde(rename = "entityValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_values: Option<Vec<String>>,
    /// <p>A list of event status codes.</p>
    #[serde(rename = "eventStatusCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status_codes: Option<Vec<String>>,
    /// <p>REPLACEME</p>
    #[serde(rename = "eventTypeCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_categories: Option<Vec<String>>,
    /// <p>A list of unique identifiers for event types. For example, <code>"AWS_EC2_SYSTEM_MAINTENANCE_EVENT","AWS_RDS_MAINTENANCE_SCHEDULED".</code> </p>
    #[serde(rename = "eventTypeCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_codes: Option<Vec<String>>,
    #[serde(rename = "lastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<DateTimeRange>,
    /// <p>A list of AWS Regions.</p>
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    /// <p>The AWS services associated with the event. For example, <code>EC2</code>, <code>RDS</code>.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTimeRange>,
}

/// Errors returned by DescribeAffectedAccountsForOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeAffectedAccountsForOrganizationError {
    /// <p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
}

impl DescribeAffectedAccountsForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAffectedAccountsForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidPaginationToken" => {
                    return RusotoError::Service(
                        DescribeAffectedAccountsForOrganizationError::InvalidPaginationToken(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAffectedAccountsForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAffectedAccountsForOrganizationError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAffectedAccountsForOrganizationError {}
/// Errors returned by DescribeAffectedEntities
#[derive(Debug, PartialEq)]
pub enum DescribeAffectedEntitiesError {
    /// <p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
}

impl DescribeAffectedEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAffectedEntitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidPaginationToken" => {
                    return RusotoError::Service(
                        DescribeAffectedEntitiesError::InvalidPaginationToken(err.msg),
                    )
                }
                "UnsupportedLocale" => {
                    return RusotoError::Service(DescribeAffectedEntitiesError::UnsupportedLocale(
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
impl fmt::Display for DescribeAffectedEntitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAffectedEntitiesError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAffectedEntitiesError::UnsupportedLocale(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAffectedEntitiesError {}
/// Errors returned by DescribeAffectedEntitiesForOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeAffectedEntitiesForOrganizationError {
    /// <p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
}

impl DescribeAffectedEntitiesForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAffectedEntitiesForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidPaginationToken" => {
                    return RusotoError::Service(
                        DescribeAffectedEntitiesForOrganizationError::InvalidPaginationToken(
                            err.msg,
                        ),
                    )
                }
                "UnsupportedLocale" => {
                    return RusotoError::Service(
                        DescribeAffectedEntitiesForOrganizationError::UnsupportedLocale(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAffectedEntitiesForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAffectedEntitiesForOrganizationError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAffectedEntitiesForOrganizationError::UnsupportedLocale(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAffectedEntitiesForOrganizationError {}
/// Errors returned by DescribeEntityAggregates
#[derive(Debug, PartialEq)]
pub enum DescribeEntityAggregatesError {}

impl DescribeEntityAggregatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEntityAggregatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEntityAggregatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEntityAggregatesError {}
/// Errors returned by DescribeEventAggregates
#[derive(Debug, PartialEq)]
pub enum DescribeEventAggregatesError {
    /// <p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
}

impl DescribeEventAggregatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventAggregatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidPaginationToken" => {
                    return RusotoError::Service(
                        DescribeEventAggregatesError::InvalidPaginationToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventAggregatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventAggregatesError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEventAggregatesError {}
/// Errors returned by DescribeEventDetails
#[derive(Debug, PartialEq)]
pub enum DescribeEventDetailsError {
    /// <p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
}

impl DescribeEventDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "UnsupportedLocale" => {
                    return RusotoError::Service(DescribeEventDetailsError::UnsupportedLocale(
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
impl fmt::Display for DescribeEventDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventDetailsError::UnsupportedLocale(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventDetailsError {}
/// Errors returned by DescribeEventDetailsForOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeEventDetailsForOrganizationError {
    /// <p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
}

impl DescribeEventDetailsForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEventDetailsForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "UnsupportedLocale" => {
                    return RusotoError::Service(
                        DescribeEventDetailsForOrganizationError::UnsupportedLocale(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventDetailsForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventDetailsForOrganizationError::UnsupportedLocale(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEventDetailsForOrganizationError {}
/// Errors returned by DescribeEventTypes
#[derive(Debug, PartialEq)]
pub enum DescribeEventTypesError {
    /// <p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
}

impl DescribeEventTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidPaginationToken" => {
                    return RusotoError::Service(DescribeEventTypesError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "UnsupportedLocale" => {
                    return RusotoError::Service(DescribeEventTypesError::UnsupportedLocale(
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
impl fmt::Display for DescribeEventTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventTypesError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            DescribeEventTypesError::UnsupportedLocale(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventTypesError {}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    /// <p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
}

impl DescribeEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidPaginationToken" => {
                    return RusotoError::Service(DescribeEventsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "UnsupportedLocale" => {
                    return RusotoError::Service(DescribeEventsError::UnsupportedLocale(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventsError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            DescribeEventsError::UnsupportedLocale(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventsError {}
/// Errors returned by DescribeEventsForOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeEventsForOrganizationError {
    /// <p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
}

impl DescribeEventsForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEventsForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidPaginationToken" => {
                    return RusotoError::Service(
                        DescribeEventsForOrganizationError::InvalidPaginationToken(err.msg),
                    )
                }
                "UnsupportedLocale" => {
                    return RusotoError::Service(
                        DescribeEventsForOrganizationError::UnsupportedLocale(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventsForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventsForOrganizationError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEventsForOrganizationError::UnsupportedLocale(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEventsForOrganizationError {}
/// Errors returned by DescribeHealthServiceStatusForOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeHealthServiceStatusForOrganizationError {}

impl DescribeHealthServiceStatusForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeHealthServiceStatusForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeHealthServiceStatusForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeHealthServiceStatusForOrganizationError {}
/// Errors returned by DisableHealthServiceAccessForOrganization
#[derive(Debug, PartialEq)]
pub enum DisableHealthServiceAccessForOrganizationError {
    /// <p> <a>EnableHealthServiceAccessForOrganization</a> is already in progress. Wait for the action to complete before trying again. To get the current status, use the <a>DescribeHealthServiceStatusForOrganization</a> operation.</p>
    ConcurrentModification(String),
}

impl DisableHealthServiceAccessForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisableHealthServiceAccessForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DisableHealthServiceAccessForOrganizationError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableHealthServiceAccessForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableHealthServiceAccessForOrganizationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableHealthServiceAccessForOrganizationError {}
/// Errors returned by EnableHealthServiceAccessForOrganization
#[derive(Debug, PartialEq)]
pub enum EnableHealthServiceAccessForOrganizationError {
    /// <p> <a>EnableHealthServiceAccessForOrganization</a> is already in progress. Wait for the action to complete before trying again. To get the current status, use the <a>DescribeHealthServiceStatusForOrganization</a> operation.</p>
    ConcurrentModification(String),
}

impl EnableHealthServiceAccessForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<EnableHealthServiceAccessForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        EnableHealthServiceAccessForOrganizationError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableHealthServiceAccessForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableHealthServiceAccessForOrganizationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableHealthServiceAccessForOrganizationError {}
/// Trait representing the capabilities of the AWSHealth API. AWSHealth clients implement this trait.
#[async_trait]
pub trait AWSHealth {
    /// <p>Returns a list of accounts in the organization from AWS Organizations that are affected by the provided event.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a>EnableHealthServiceAccessForOrganization</a> operation from your organization's master account.</p>
    async fn describe_affected_accounts_for_organization(
        &self,
        input: DescribeAffectedAccountsForOrganizationRequest,
    ) -> Result<
        DescribeAffectedAccountsForOrganizationResponse,
        RusotoError<DescribeAffectedAccountsForOrganizationError>,
    >;

    /// <p>Returns a list of entities that have been affected by the specified events, based on the specified filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service. Events that have impact beyond that of the affected entities, or where the extent of impact is unknown, include at least one entity indicating this.</p> <p>At least one event ARN is required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p>
    async fn describe_affected_entities(
        &self,
        input: DescribeAffectedEntitiesRequest,
    ) -> Result<DescribeAffectedEntitiesResponse, RusotoError<DescribeAffectedEntitiesError>>;

    /// <p>Returns a list of entities that have been affected by one or more events for one or more accounts in your organization in AWS Organizations, based on the filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service.</p> <p>At least one event ARN and account ID are required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a>EnableHealthServiceAccessForOrganization</a> operation from your organization's master account. </p>
    async fn describe_affected_entities_for_organization(
        &self,
        input: DescribeAffectedEntitiesForOrganizationRequest,
    ) -> Result<
        DescribeAffectedEntitiesForOrganizationResponse,
        RusotoError<DescribeAffectedEntitiesForOrganizationError>,
    >;

    /// <p>Returns the number of entities that are affected by each of the specified events. If no events are specified, the counts of all affected entities are returned.</p>
    async fn describe_entity_aggregates(
        &self,
        input: DescribeEntityAggregatesRequest,
    ) -> Result<DescribeEntityAggregatesResponse, RusotoError<DescribeEntityAggregatesError>>;

    /// <p>Returns the number of events of each event type (issue, scheduled change, and account notification). If no filter is specified, the counts of all events in each category are returned.</p>
    async fn describe_event_aggregates(
        &self,
        input: DescribeEventAggregatesRequest,
    ) -> Result<DescribeEventAggregatesResponse, RusotoError<DescribeEventAggregatesError>>;

    /// <p>Returns detailed information about one or more specified events. Information includes standard event data (region, service, and so on, as returned by <a>DescribeEvents</a>), a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included; to retrieve those, use the <a>DescribeAffectedEntities</a> operation.</p> <p>If a specified event cannot be retrieved, an error message is returned for that event.</p>
    async fn describe_event_details(
        &self,
        input: DescribeEventDetailsRequest,
    ) -> Result<DescribeEventDetailsResponse, RusotoError<DescribeEventDetailsError>>;

    /// <p>Returns detailed information about one or more specified events for one or more accounts in your organization. Information includes standard event data (Region, service, and so on, as returned by <a>DescribeEventsForOrganization</a>, a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included; to retrieve those, use the <a>DescribeAffectedEntitiesForOrganization</a> operation.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a>EnableHealthServiceAccessForOrganization</a> operation from your organization's master account.</p>
    async fn describe_event_details_for_organization(
        &self,
        input: DescribeEventDetailsForOrganizationRequest,
    ) -> Result<
        DescribeEventDetailsForOrganizationResponse,
        RusotoError<DescribeEventDetailsForOrganizationError>,
    >;

    /// <p>Returns the event types that meet the specified filter criteria. If no filter criteria are specified, all event types are returned, in no particular order.</p>
    async fn describe_event_types(
        &self,
        input: DescribeEventTypesRequest,
    ) -> Result<DescribeEventTypesResponse, RusotoError<DescribeEventTypesError>>;

    /// <p>Returns information about events that meet the specified filter criteria. Events are returned in a summary form and do not include the detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the <a>DescribeEventDetails</a> and <a>DescribeAffectedEntities</a> operations.</p> <p>If no filter criteria are specified, all events are returned. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent.</p>
    async fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> Result<DescribeEventsResponse, RusotoError<DescribeEventsError>>;

    /// <p>Returns information about events across your organization in AWS Organizations, meeting the specified filter criteria. Events are returned in a summary form and do not include the accounts impacted, detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the <a>DescribeAffectedAccountsForOrganization</a>, <a>DescribeEventDetailsForOrganization</a>, and <a>DescribeAffectedEntitiesForOrganization</a> operations.</p> <p>If no filter criteria are specified, all events across your organization are returned. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent.</p> <p>Before you can call this operation, you must first enable Health to work with AWS Organizations. To do this, call the <a>EnableHealthServiceAccessForOrganization</a> operation from your organization's master account.</p>
    async fn describe_events_for_organization(
        &self,
        input: DescribeEventsForOrganizationRequest,
    ) -> Result<
        DescribeEventsForOrganizationResponse,
        RusotoError<DescribeEventsForOrganizationError>,
    >;

    /// <p>This operation provides status information on enabling or disabling AWS Health to work with your organization. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p>
    async fn describe_health_service_status_for_organization(
        &self,
    ) -> Result<
        DescribeHealthServiceStatusForOrganizationResponse,
        RusotoError<DescribeHealthServiceStatusForOrganizationError>,
    >;

    /// <p>Calling this operation disables Health from working with AWS Organizations. This does not remove the Service Linked Role (SLR) from the the master account in your organization. Use the IAM console, API, or AWS CLI to remove the SLR if desired. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p>
    async fn disable_health_service_access_for_organization(
        &self,
    ) -> Result<(), RusotoError<DisableHealthServiceAccessForOrganizationError>>;

    /// <p>Calling this operation enables AWS Health to work with AWS Organizations. This applies a Service Linked Role (SLR) to the master account in the organization. To learn more about the steps in this process, visit enabling service access for AWS Health in AWS Organizations. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p>
    async fn enable_health_service_access_for_organization(
        &self,
    ) -> Result<(), RusotoError<EnableHealthServiceAccessForOrganizationError>>;
}
/// A client for the AWSHealth API.
#[derive(Clone)]
pub struct AWSHealthClient {
    client: Client,
    region: region::Region,
}

impl AWSHealthClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AWSHealthClient {
        AWSHealthClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AWSHealthClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        AWSHealthClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> AWSHealthClient {
        AWSHealthClient { client, region }
    }
}

#[async_trait]
impl AWSHealth for AWSHealthClient {
    /// <p>Returns a list of accounts in the organization from AWS Organizations that are affected by the provided event.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a>EnableHealthServiceAccessForOrganization</a> operation from your organization's master account.</p>
    async fn describe_affected_accounts_for_organization(
        &self,
        input: DescribeAffectedAccountsForOrganizationRequest,
    ) -> Result<
        DescribeAffectedAccountsForOrganizationResponse,
        RusotoError<DescribeAffectedAccountsForOrganizationError>,
    > {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeAffectedAccountsForOrganization",
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
                .deserialize::<DescribeAffectedAccountsForOrganizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAffectedAccountsForOrganizationError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a list of entities that have been affected by the specified events, based on the specified filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service. Events that have impact beyond that of the affected entities, or where the extent of impact is unknown, include at least one entity indicating this.</p> <p>At least one event ARN is required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p>
    async fn describe_affected_entities(
        &self,
        input: DescribeAffectedEntitiesRequest,
    ) -> Result<DescribeAffectedEntitiesResponse, RusotoError<DescribeAffectedEntitiesError>> {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeAffectedEntities",
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
                .deserialize::<DescribeAffectedEntitiesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAffectedEntitiesError::from_response(response))
        }
    }

    /// <p>Returns a list of entities that have been affected by one or more events for one or more accounts in your organization in AWS Organizations, based on the filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service.</p> <p>At least one event ARN and account ID are required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a>EnableHealthServiceAccessForOrganization</a> operation from your organization's master account. </p>
    async fn describe_affected_entities_for_organization(
        &self,
        input: DescribeAffectedEntitiesForOrganizationRequest,
    ) -> Result<
        DescribeAffectedEntitiesForOrganizationResponse,
        RusotoError<DescribeAffectedEntitiesForOrganizationError>,
    > {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeAffectedEntitiesForOrganization",
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
                .deserialize::<DescribeAffectedEntitiesForOrganizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAffectedEntitiesForOrganizationError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the number of entities that are affected by each of the specified events. If no events are specified, the counts of all affected entities are returned.</p>
    async fn describe_entity_aggregates(
        &self,
        input: DescribeEntityAggregatesRequest,
    ) -> Result<DescribeEntityAggregatesResponse, RusotoError<DescribeEntityAggregatesError>> {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeEntityAggregates",
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
                .deserialize::<DescribeEntityAggregatesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEntityAggregatesError::from_response(response))
        }
    }

    /// <p>Returns the number of events of each event type (issue, scheduled change, and account notification). If no filter is specified, the counts of all events in each category are returned.</p>
    async fn describe_event_aggregates(
        &self,
        input: DescribeEventAggregatesRequest,
    ) -> Result<DescribeEventAggregatesResponse, RusotoError<DescribeEventAggregatesError>> {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventAggregates");
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
                .deserialize::<DescribeEventAggregatesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventAggregatesError::from_response(response))
        }
    }

    /// <p>Returns detailed information about one or more specified events. Information includes standard event data (region, service, and so on, as returned by <a>DescribeEvents</a>), a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included; to retrieve those, use the <a>DescribeAffectedEntities</a> operation.</p> <p>If a specified event cannot be retrieved, an error message is returned for that event.</p>
    async fn describe_event_details(
        &self,
        input: DescribeEventDetailsRequest,
    ) -> Result<DescribeEventDetailsResponse, RusotoError<DescribeEventDetailsError>> {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventDetails");
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
                .deserialize::<DescribeEventDetailsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventDetailsError::from_response(response))
        }
    }

    /// <p>Returns detailed information about one or more specified events for one or more accounts in your organization. Information includes standard event data (Region, service, and so on, as returned by <a>DescribeEventsForOrganization</a>, a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included; to retrieve those, use the <a>DescribeAffectedEntitiesForOrganization</a> operation.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a>EnableHealthServiceAccessForOrganization</a> operation from your organization's master account.</p>
    async fn describe_event_details_for_organization(
        &self,
        input: DescribeEventDetailsForOrganizationRequest,
    ) -> Result<
        DescribeEventDetailsForOrganizationResponse,
        RusotoError<DescribeEventDetailsForOrganizationError>,
    > {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeEventDetailsForOrganization",
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
                .deserialize::<DescribeEventDetailsForOrganizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventDetailsForOrganizationError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns the event types that meet the specified filter criteria. If no filter criteria are specified, all event types are returned, in no particular order.</p>
    async fn describe_event_types(
        &self,
        input: DescribeEventTypesRequest,
    ) -> Result<DescribeEventTypesResponse, RusotoError<DescribeEventTypesError>> {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventTypes");
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
                .deserialize::<DescribeEventTypesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventTypesError::from_response(response))
        }
    }

    /// <p>Returns information about events that meet the specified filter criteria. Events are returned in a summary form and do not include the detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the <a>DescribeEventDetails</a> and <a>DescribeAffectedEntities</a> operations.</p> <p>If no filter criteria are specified, all events are returned. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent.</p>
    async fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> Result<DescribeEventsResponse, RusotoError<DescribeEventsError>> {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventsError::from_response(response))
        }
    }

    /// <p>Returns information about events across your organization in AWS Organizations, meeting the specified filter criteria. Events are returned in a summary form and do not include the accounts impacted, detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the <a>DescribeAffectedAccountsForOrganization</a>, <a>DescribeEventDetailsForOrganization</a>, and <a>DescribeAffectedEntitiesForOrganization</a> operations.</p> <p>If no filter criteria are specified, all events across your organization are returned. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent.</p> <p>Before you can call this operation, you must first enable Health to work with AWS Organizations. To do this, call the <a>EnableHealthServiceAccessForOrganization</a> operation from your organization's master account.</p>
    async fn describe_events_for_organization(
        &self,
        input: DescribeEventsForOrganizationRequest,
    ) -> Result<
        DescribeEventsForOrganizationResponse,
        RusotoError<DescribeEventsForOrganizationError>,
    > {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeEventsForOrganization",
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
                .deserialize::<DescribeEventsForOrganizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEventsForOrganizationError::from_response(response))
        }
    }

    /// <p>This operation provides status information on enabling or disabling AWS Health to work with your organization. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p>
    async fn describe_health_service_status_for_organization(
        &self,
    ) -> Result<
        DescribeHealthServiceStatusForOrganizationResponse,
        RusotoError<DescribeHealthServiceStatusForOrganizationError>,
    > {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeHealthServiceStatusForOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeHealthServiceStatusForOrganizationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeHealthServiceStatusForOrganizationError::from_response(response))
        }
    }

    /// <p>Calling this operation disables Health from working with AWS Organizations. This does not remove the Service Linked Role (SLR) from the the master account in your organization. Use the IAM console, API, or AWS CLI to remove the SLR if desired. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p>
    async fn disable_health_service_access_for_organization(
        &self,
    ) -> Result<(), RusotoError<DisableHealthServiceAccessForOrganizationError>> {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DisableHealthServiceAccessForOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisableHealthServiceAccessForOrganizationError::from_response(response))
        }
    }

    /// <p>Calling this operation enables AWS Health to work with AWS Organizations. This applies a Service Linked Role (SLR) to the master account in the organization. To learn more about the steps in this process, visit enabling service access for AWS Health in AWS Organizations. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p>
    async fn enable_health_service_access_for_organization(
        &self,
    ) -> Result<(), RusotoError<EnableHealthServiceAccessForOrganizationError>> {
        let mut request = SignedRequest::new("POST", "health", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.EnableHealthServiceAccessForOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(EnableHealthServiceAccessForOrganizationError::from_response(response))
        }
    }
}
