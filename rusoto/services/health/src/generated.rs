#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
pub type AccountId = String;
#[doc="<p>Information about an entity that is affected by a Health event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AffectedEntity {
    #[doc="<p>The 12-digit AWS account number that contains the affected entity.</p>"]
    #[serde(rename="awsAccountId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aws_account_id: Option<AccountId>,
    #[doc="<p>The unique identifier for the entity. Format: <code>arn:aws:health:<i>entity-region</i>:<i>aws-account</i>:entity/<i>entity-id</i> </code>. Example: <code>arn:aws:health:us-east-1:111222333444:entity/AVh5GGT7ul1arKr1sE1K</code> </p>"]
    #[serde(rename="entityArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entity_arn: Option<EntityArn>,
    #[doc="<p>The ID of the affected entity.</p>"]
    #[serde(rename="entityValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entity_value: Option<EntityValue>,
    #[doc="<p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>arn:aws:health:us-east-1::event/AWS_EC2_MAINTENANCE_5331</code> </p>"]
    #[serde(rename="eventArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_arn: Option<EventArn>,
    #[doc="<p>The most recent time that the entity was updated.</p>"]
    #[serde(rename="lastUpdatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_time: Option<Timestamp>,
    #[doc="<p>The most recent status of the entity affected by the event. The possible values are <code>IMPAIRED</code>, <code>UNIMPAIRED</code>, and <code>UNKNOWN</code>.</p>"]
    #[serde(rename="statusCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_code: Option<EntityStatusCode>,
    #[doc="<p>A map of entity tags attached to the affected entity.</p>"]
    #[serde(rename="tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<TagSet>,
}

pub type AggregateValue = String;
pub type AvailabilityZone = String;
pub type AvailabilityZones = Vec<AvailabilityZone>;
pub type Count = i64;
#[doc="<p>A range of dates and times that is used by the <a>EventFilter</a> and <a>EntityFilter</a> objects. If <code>from</code> is set and <code>to</code> is set: match items where the timestamp (<code>startTime</code>, <code>endTime</code>, or <code>lastUpdatedTime</code>) is between <code>from</code> and <code>to</code> inclusive. If <code>from</code> is set and <code>to</code> is not set: match items where the timestamp value is equal to or after <code>from</code>. If <code>from</code> is not set and <code>to</code> is set: match items where the timestamp value is equal to or before <code>to</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DateTimeRange {
    #[doc="<p>The starting date and time of a time range.</p>"]
    #[serde(rename="from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<Timestamp>,
    #[doc="<p>The ending date and time of a time range.</p>"]
    #[serde(rename="to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<Timestamp>,
}

pub type DateTimeRangeList = Vec<DateTimeRange>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeAffectedEntitiesRequest {
    #[doc="<p>Values to narrow the results returned. At least one event ARN is required. </p>"]
    #[serde(rename="filter")]
    pub filter: EntityFilter,
    #[doc="<p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>"]
    #[serde(rename="locale")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locale: Option<Locale>,
    #[doc="<p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<MaxResults>,
    #[doc="<p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeAffectedEntitiesResponse {
    #[doc="<p>The entities that match the filter criteria.</p>"]
    #[serde(rename="entities")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entities: Option<EntityList>,
    #[doc="<p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEntityAggregatesRequest {
    #[doc="<p>A list of event ARNs (unique identifiers). For example: <code>\"arn:aws:health:us-east-1::event/AWS_EC2_MAINTENANCE_5331\", \"arn:aws:health:us-west-1::event/AWS_EBS_LOST_VOLUME_xyz\"</code> </p>"]
    #[serde(rename="eventArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_arns: Option<EventArnsList>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEntityAggregatesResponse {
    #[doc="<p>The number of entities that are affected by each of the specified events.</p>"]
    #[serde(rename="entityAggregates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entity_aggregates: Option<EntityAggregateList>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventAggregatesRequest {
    #[doc="<p>The only currently supported value is <code>eventTypeCategory</code>.</p>"]
    #[serde(rename="aggregateField")]
    pub aggregate_field: EventAggregateField,
    #[doc="<p>Values to narrow the results returned.</p>"]
    #[serde(rename="filter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<EventFilter>,
    #[doc="<p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<MaxResults>,
    #[doc="<p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventAggregatesResponse {
    #[doc="<p>The number of events in each category that meet the optional filter criteria.</p>"]
    #[serde(rename="eventAggregates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_aggregates: Option<EventAggregateList>,
    #[doc="<p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

pub type DescribeEventDetailsFailedSet = Vec<EventDetailsErrorItem>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventDetailsRequest {
    #[doc="<p>A list of event ARNs (unique identifiers). For example: <code>\"arn:aws:health:us-east-1::event/AWS_EC2_MAINTENANCE_5331\", \"arn:aws:health:us-west-1::event/AWS_EBS_LOST_VOLUME_xyz\"</code> </p>"]
    #[serde(rename="eventArns")]
    pub event_arns: EventArnList,
    #[doc="<p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>"]
    #[serde(rename="locale")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locale: Option<Locale>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventDetailsResponse {
    #[doc="<p>Error messages for any events that could not be retrieved.</p>"]
    #[serde(rename="failedSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_set: Option<DescribeEventDetailsFailedSet>,
    #[doc="<p>Information about the events that could be retrieved.</p>"]
    #[serde(rename="successfulSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub successful_set: Option<DescribeEventDetailsSuccessfulSet>,
}

pub type DescribeEventDetailsSuccessfulSet = Vec<EventDetails>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventTypesRequest {
    #[doc="<p>Values to narrow the results returned.</p>"]
    #[serde(rename="filter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<EventTypeFilter>,
    #[doc="<p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>"]
    #[serde(rename="locale")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locale: Option<Locale>,
    #[doc="<p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<MaxResults>,
    #[doc="<p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventTypesResponse {
    #[doc="<p>A list of event types that match the filter criteria. Event types have a category (<code>issue</code>, <code>accountNotification</code>, or <code>scheduledChange</code>), a service (for example, <code>EC2</code>, <code>RDS</code>, <code>DATAPIPELINE</code>, <code>BILLING</code>), and a code (in the format <code>AWS_<i>SERVICE</i>_<i>DESCRIPTION</i> </code>; for example, <code>AWS_EC2_SYSTEM_MAINTENANCE_EVENT</code>).</p>"]
    #[serde(rename="eventTypes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_types: Option<EventTypeList>,
    #[doc="<p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventsRequest {
    #[doc="<p>Values to narrow the results returned.</p>"]
    #[serde(rename="filter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<EventFilter>,
    #[doc="<p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>"]
    #[serde(rename="locale")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locale: Option<Locale>,
    #[doc="<p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<MaxResults>,
    #[doc="<p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventsResponse {
    #[doc="<p>The events that match the specified filter criteria.</p>"]
    #[serde(rename="events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<EventList>,
    #[doc="<p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[doc="<p>The number of entities that are affected by one or more events. Returned by the <a>DescribeEntityAggregates</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EntityAggregate {
    #[doc="<p>The number entities that match the criteria for the specified events.</p>"]
    #[serde(rename="count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<Count>,
    #[doc="<p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>arn:aws:health:us-east-1::event/AWS_EC2_MAINTENANCE_5331</code> </p>"]
    #[serde(rename="eventArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_arn: Option<EventArn>,
}

pub type EntityAggregateList = Vec<EntityAggregate>;
pub type EntityArn = String;
pub type EntityArnList = Vec<EntityArn>;
#[doc="<p>The values to use to filter results from the <a>DescribeAffectedEntities</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EntityFilter {
    #[doc="<p>A list of entity ARNs (unique identifiers).</p>"]
    #[serde(rename="entityArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entity_arns: Option<EntityArnList>,
    #[doc="<p>A list of IDs for affected entities.</p>"]
    #[serde(rename="entityValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entity_values: Option<EntityValueList>,
    #[doc="<p>A list of event ARNs (unique identifiers). For example: <code>\"arn:aws:health:us-east-1::event/AWS_EC2_MAINTENANCE_5331\", \"arn:aws:health:us-west-1::event/AWS_EBS_LOST_VOLUME_xyz\"</code> </p>"]
    #[serde(rename="eventArns")]
    pub event_arns: EventArnList,
    #[doc="<p>A list of the most recent dates and times that the entity was updated.</p>"]
    #[serde(rename="lastUpdatedTimes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_times: Option<DateTimeRangeList>,
    #[doc="<p>A list of entity status codes (<code>IMPAIRED</code>, <code>UNIMPAIRED</code>, or <code>UNKNOWN</code>).</p>"]
    #[serde(rename="statusCodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_codes: Option<EntityStatusCodeList>,
    #[doc="<p>A map of entity tags attached to the affected entity.</p>"]
    #[serde(rename="tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<TagFilter>,
}

pub type EntityList = Vec<AffectedEntity>;
pub type EntityStatusCode = String;
pub type EntityStatusCodeList = Vec<EntityStatusCode>;
pub type EntityValue = String;
pub type EntityValueList = Vec<EntityValue>;
#[doc="<p>Summary information about an event, returned by the <a>DescribeEvents</a> operation. The <a>DescribeEventDetails</a> operation also returns this information, as well as the <a>EventDescription</a> and additional event metadata.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Event {
    #[doc="<p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>arn:aws:health:us-east-1::event/AWS_EC2_MAINTENANCE_5331</code> </p>"]
    #[serde(rename="arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<EventArn>,
    #[doc="<p>The AWS Availability Zone of the event. For example, us-east-1a.</p>"]
    #[serde(rename="availabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<AvailabilityZone>,
    #[doc="<p>The date and time that the event ended.</p>"]
    #[serde(rename="endTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_time: Option<Timestamp>,
    #[doc="<p>The </p>"]
    #[serde(rename="eventTypeCategory")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_type_category: Option<EventTypeCategory>,
    #[doc="<p>The unique identifier for the event type. The format is <code>AWS_<i>SERVICE</i>_<i>DESCRIPTION</i> </code>; for example, <code>AWS_EC2_SYSTEM_MAINTENANCE_EVENT</code>.</p>"]
    #[serde(rename="eventTypeCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_type_code: Option<EventTypeCode>,
    #[doc="<p>The most recent date and time that the event was updated.</p>"]
    #[serde(rename="lastUpdatedTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_time: Option<Timestamp>,
    #[doc="<p>The AWS region name of the event.</p>"]
    #[serde(rename="region")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub region: Option<Region>,
    #[doc="<p>The AWS service that is affected by the event. For example, <code>EC2</code>, <code>RDS</code>.</p>"]
    #[serde(rename="service")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service: Option<Service>,
    #[doc="<p>The date and time that the event began.</p>"]
    #[serde(rename="startTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<Timestamp>,
    #[doc="<p>The most recent status of the event. Possible values are <code>open</code>, <code>closed</code>, and <code>upcoming</code>.</p>"]
    #[serde(rename="statusCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_code: Option<EventStatusCode>,
}

#[doc="<p>The number of events of each issue type. Returned by the <a>DescribeEventAggregates</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EventAggregate {
    #[doc="<p>The issue type for the associated count.</p>"]
    #[serde(rename="aggregateValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aggregate_value: Option<AggregateValue>,
    #[doc="<p>The number of events of the associated issue type.</p>"]
    #[serde(rename="count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<Count>,
}

pub type EventAggregateField = String;
pub type EventAggregateList = Vec<EventAggregate>;
pub type EventArn = String;
pub type EventArnList = Vec<EventArn>;
pub type EventArnsList = Vec<EventArn>;
pub type EventDescription = String;
#[doc="<p>Detailed information about an event. A combination of an <a>Event</a> object, an <a>EventDescription</a> object, and additional metadata about the event. Returned by the <a>DescribeEventDetails</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EventDetails {
    #[doc="<p>Summary information about the event.</p>"]
    #[serde(rename="event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<Event>,
    #[doc="<p>The most recent description of the event.</p>"]
    #[serde(rename="eventDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_description: Option<EventDescription>,
    #[doc="<p>Additional metadata about the event.</p>"]
    #[serde(rename="eventMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_metadata: Option<EventMetadata>,
}

#[doc="<p>Error information returned when a <a>DescribeEventDetails</a> operation cannot find a specified event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EventDetailsErrorItem {
    #[doc="<p>A message that describes the error.</p>"]
    #[serde(rename="errorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
    #[doc="<p>The name of the error.</p>"]
    #[serde(rename="errorName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_name: Option<String>,
    #[doc="<p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>arn:aws:health:us-east-1::event/AWS_EC2_MAINTENANCE_5331</code> </p>"]
    #[serde(rename="eventArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_arn: Option<EventArn>,
}

#[doc="<p>The values to use to filter results from the <a>DescribeEvents</a> and <a>DescribeEventAggregates</a> operations.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EventFilter {
    #[doc="<p>A list of AWS availability zones.</p>"]
    #[serde(rename="availabilityZones")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[doc="<p>A list of dates and times that the event ended.</p>"]
    #[serde(rename="endTimes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_times: Option<DateTimeRangeList>,
    #[doc="<p>A list of entity ARNs (unique identifiers).</p>"]
    #[serde(rename="entityArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entity_arns: Option<EntityArnList>,
    #[doc="<p>A list of entity identifiers, such as EC2 instance IDs (<code>i-34ab692e</code>) or EBS volumes (<code>vol-426ab23e</code>).</p>"]
    #[serde(rename="entityValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entity_values: Option<EntityValueList>,
    #[doc="<p>A list of event ARNs (unique identifiers). For example: <code>\"arn:aws:health:us-east-1::event/AWS_EC2_MAINTENANCE_5331\", \"arn:aws:health:us-west-1::event/AWS_EBS_LOST_VOLUME_xyz\"</code> </p>"]
    #[serde(rename="eventArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_arns: Option<EventArnList>,
    #[doc="<p>A list of event status codes.</p>"]
    #[serde(rename="eventStatusCodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_status_codes: Option<EventStatusCodeList>,
    #[doc="<p>A list of event type category codes (<code>issue</code>, <code>scheduledChange</code>, or <code>accountNotification</code>).</p>"]
    #[serde(rename="eventTypeCategories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_type_categories: Option<EventTypeCategoryList>,
    #[doc="<p>A list of unique identifiers for event types. For example, <code>\"AWS_EC2_SYSTEM_MAINTENANCE_EVENT\",\"AWS_RDS_MAINTENANCE_SCHEDULED\"</code> </p>"]
    #[serde(rename="eventTypeCodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_type_codes: Option<EventTypeList>,
    #[doc="<p>A list of dates and times that the event was last updated.</p>"]
    #[serde(rename="lastUpdatedTimes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_times: Option<DateTimeRangeList>,
    #[doc="<p>A list of AWS regions.</p>"]
    #[serde(rename="regions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub regions: Option<RegionList>,
    #[doc="<p>The AWS services associated with the event. For example, <code>EC2</code>, <code>RDS</code>.</p>"]
    #[serde(rename="services")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub services: Option<ServiceList>,
    #[doc="<p>A list of dates and times that the event began.</p>"]
    #[serde(rename="startTimes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_times: Option<DateTimeRangeList>,
    #[doc="<p>A map of entity tags attached to the affected entity.</p>"]
    #[serde(rename="tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<TagFilter>,
}

pub type EventList = Vec<Event>;
pub type EventMetadata = ::std::collections::HashMap<MetadataKey, MetadataValue>;
pub type EventStatusCode = String;
pub type EventStatusCodeList = Vec<EventStatusCode>;
pub type EventType = String;
pub type EventTypeCategory = String;
pub type EventTypeCategoryList = Vec<EventTypeCategory>;
pub type EventTypeCode = String;
pub type EventTypeCodeList = Vec<EventTypeCode>;
#[doc="<p>The values to use to filter results from the <a>DescribeEventTypes</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EventTypeFilter {
    #[doc="<p>A list of event type category codes (<code>issue</code>, <code>scheduledChange</code>, or <code>accountNotification</code>).</p>"]
    #[serde(rename="eventTypeCategories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_type_categories: Option<EventTypeCategoryList>,
    #[doc="<p>A list of event type codes.</p>"]
    #[serde(rename="eventTypeCodes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_type_codes: Option<EventTypeCodeList>,
    #[doc="<p>The AWS services associated with the event. For example, <code>EC2</code>, <code>RDS</code>.</p>"]
    #[serde(rename="services")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub services: Option<ServiceList>,
}

pub type EventTypeList = Vec<EventType>;
pub type Locale = String;
pub type MaxResults = i64;
pub type MetadataKey = String;
pub type MetadataValue = String;
pub type NextToken = String;
pub type Region = String;
pub type RegionList = Vec<Region>;
pub type Service = String;
pub type ServiceList = Vec<Service>;
pub type TagFilter = Vec<TagSet>;
pub type TagKey = String;
pub type TagSet = ::std::collections::HashMap<TagKey, TagValue>;
pub type TagValue = String;
pub type Timestamp = f64;
/// Errors returned by DescribeAffectedEntities
#[derive(Debug, PartialEq)]
pub enum DescribeAffectedEntitiesError {
    ///<p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    ///<p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeAffectedEntitiesError {
    pub fn from_body(body: &str) -> DescribeAffectedEntitiesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidPaginationToken" => DescribeAffectedEntitiesError::InvalidPaginationToken(String::from(error_message)),
                    "UnsupportedLocale" => DescribeAffectedEntitiesError::UnsupportedLocale(String::from(error_message)),
                    "ValidationException" => {
                        DescribeAffectedEntitiesError::Validation(error_message.to_string())
                    }
                    _ => DescribeAffectedEntitiesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAffectedEntitiesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAffectedEntitiesError {
    fn from(err: serde_json::error::Error) -> DescribeAffectedEntitiesError {
        DescribeAffectedEntitiesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAffectedEntitiesError {
    fn from(err: CredentialsError) -> DescribeAffectedEntitiesError {
        DescribeAffectedEntitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAffectedEntitiesError {
    fn from(err: HttpDispatchError) -> DescribeAffectedEntitiesError {
        DescribeAffectedEntitiesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeAffectedEntitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAffectedEntitiesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAffectedEntitiesError::InvalidPaginationToken(ref cause) => cause,
            DescribeAffectedEntitiesError::UnsupportedLocale(ref cause) => cause,
            DescribeAffectedEntitiesError::Validation(ref cause) => cause,
            DescribeAffectedEntitiesError::Credentials(ref err) => err.description(),
            DescribeAffectedEntitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAffectedEntitiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEntityAggregates
#[derive(Debug, PartialEq)]
pub enum DescribeEntityAggregatesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEntityAggregatesError {
    pub fn from_body(body: &str) -> DescribeEntityAggregatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DescribeEntityAggregatesError::Validation(error_message.to_string())
                    }
                    _ => DescribeEntityAggregatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEntityAggregatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEntityAggregatesError {
    fn from(err: serde_json::error::Error) -> DescribeEntityAggregatesError {
        DescribeEntityAggregatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEntityAggregatesError {
    fn from(err: CredentialsError) -> DescribeEntityAggregatesError {
        DescribeEntityAggregatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEntityAggregatesError {
    fn from(err: HttpDispatchError) -> DescribeEntityAggregatesError {
        DescribeEntityAggregatesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEntityAggregatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEntityAggregatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeEntityAggregatesError::Validation(ref cause) => cause,
            DescribeEntityAggregatesError::Credentials(ref err) => err.description(),
            DescribeEntityAggregatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEntityAggregatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventAggregates
#[derive(Debug, PartialEq)]
pub enum DescribeEventAggregatesError {
    ///<p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEventAggregatesError {
    pub fn from_body(body: &str) -> DescribeEventAggregatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidPaginationToken" => DescribeEventAggregatesError::InvalidPaginationToken(String::from(error_message)),
                    "ValidationException" => {
                        DescribeEventAggregatesError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventAggregatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventAggregatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventAggregatesError {
    fn from(err: serde_json::error::Error) -> DescribeEventAggregatesError {
        DescribeEventAggregatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventAggregatesError {
    fn from(err: CredentialsError) -> DescribeEventAggregatesError {
        DescribeEventAggregatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventAggregatesError {
    fn from(err: HttpDispatchError) -> DescribeEventAggregatesError {
        DescribeEventAggregatesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEventAggregatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventAggregatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventAggregatesError::InvalidPaginationToken(ref cause) => cause,
            DescribeEventAggregatesError::Validation(ref cause) => cause,
            DescribeEventAggregatesError::Credentials(ref err) => err.description(),
            DescribeEventAggregatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventAggregatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventDetails
#[derive(Debug, PartialEq)]
pub enum DescribeEventDetailsError {
    ///<p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEventDetailsError {
    pub fn from_body(body: &str) -> DescribeEventDetailsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "UnsupportedLocale" => {
                        DescribeEventDetailsError::UnsupportedLocale(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEventDetailsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventDetailsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventDetailsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventDetailsError {
    fn from(err: serde_json::error::Error) -> DescribeEventDetailsError {
        DescribeEventDetailsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventDetailsError {
    fn from(err: CredentialsError) -> DescribeEventDetailsError {
        DescribeEventDetailsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventDetailsError {
    fn from(err: HttpDispatchError) -> DescribeEventDetailsError {
        DescribeEventDetailsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEventDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventDetailsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventDetailsError::UnsupportedLocale(ref cause) => cause,
            DescribeEventDetailsError::Validation(ref cause) => cause,
            DescribeEventDetailsError::Credentials(ref err) => err.description(),
            DescribeEventDetailsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventDetailsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventTypes
#[derive(Debug, PartialEq)]
pub enum DescribeEventTypesError {
    ///<p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    ///<p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEventTypesError {
    pub fn from_body(body: &str) -> DescribeEventTypesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidPaginationToken" => {
                        DescribeEventTypesError::InvalidPaginationToken(String::from(error_message))
                    }
                    "UnsupportedLocale" => {
                        DescribeEventTypesError::UnsupportedLocale(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEventTypesError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventTypesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventTypesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventTypesError {
    fn from(err: serde_json::error::Error) -> DescribeEventTypesError {
        DescribeEventTypesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventTypesError {
    fn from(err: CredentialsError) -> DescribeEventTypesError {
        DescribeEventTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventTypesError {
    fn from(err: HttpDispatchError) -> DescribeEventTypesError {
        DescribeEventTypesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEventTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventTypesError::InvalidPaginationToken(ref cause) => cause,
            DescribeEventTypesError::UnsupportedLocale(ref cause) => cause,
            DescribeEventTypesError::Validation(ref cause) => cause,
            DescribeEventTypesError::Credentials(ref err) => err.description(),
            DescribeEventTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    ///<p>The specified pagination token (<code>nextToken</code>) is not valid.</p>
    InvalidPaginationToken(String),
    ///<p>The specified locale is not supported.</p>
    UnsupportedLocale(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEventsError {
    pub fn from_body(body: &str) -> DescribeEventsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidPaginationToken" => {
                        DescribeEventsError::InvalidPaginationToken(String::from(error_message))
                    }
                    "UnsupportedLocale" => {
                        DescribeEventsError::UnsupportedLocale(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEventsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventsError {
    fn from(err: serde_json::error::Error) -> DescribeEventsError {
        DescribeEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventsError {
    fn from(err: CredentialsError) -> DescribeEventsError {
        DescribeEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventsError {
    fn from(err: HttpDispatchError) -> DescribeEventsError {
        DescribeEventsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventsError::InvalidPaginationToken(ref cause) => cause,
            DescribeEventsError::UnsupportedLocale(ref cause) => cause,
            DescribeEventsError::Validation(ref cause) => cause,
            DescribeEventsError::Credentials(ref err) => err.description(),
            DescribeEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSHealth API. AWSHealth clients implement this trait.
pub trait AWSHealth {
    #[doc="<p>Returns a list of entities that have been affected by the specified events, based on the specified filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service. Events that have impact beyond that of the affected entities, or where the extent of impact is unknown, include at least one entity indicating this.</p> <p>At least one event ARN is required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p>"]
    fn describe_affected_entities
        (&self,
         input: &DescribeAffectedEntitiesRequest)
         -> Result<DescribeAffectedEntitiesResponse, DescribeAffectedEntitiesError>;


    #[doc="<p>Returns the number of entities that are affected by each of the specified events. If no events are specified, the counts of all affected entities are returned.</p>"]
    fn describe_entity_aggregates
        (&self,
         input: &DescribeEntityAggregatesRequest)
         -> Result<DescribeEntityAggregatesResponse, DescribeEntityAggregatesError>;


    #[doc="<p>Returns the number of events of each event type (issue, scheduled change, and account notification). If no filter is specified, the counts of all events in each category are returned.</p>"]
    fn describe_event_aggregates
        (&self,
         input: &DescribeEventAggregatesRequest)
         -> Result<DescribeEventAggregatesResponse, DescribeEventAggregatesError>;


    #[doc="<p>Returns detailed information about one or more specified events. Information includes standard event data (region, service, etc., as returned by <a>DescribeEvents</a>), a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included; to retrieve those, use the <a>DescribeAffectedEntities</a> operation.</p> <p>If a specified event cannot be retrieved, an error message is returned for that event.</p>"]
    fn describe_event_details
        (&self,
         input: &DescribeEventDetailsRequest)
         -> Result<DescribeEventDetailsResponse, DescribeEventDetailsError>;


    #[doc="<p>Returns the event types that meet the specified filter criteria. If no filter criteria are specified, all event types are returned, in no particular order.</p>"]
    fn describe_event_types(&self,
                            input: &DescribeEventTypesRequest)
                            -> Result<DescribeEventTypesResponse, DescribeEventTypesError>;


    #[doc="<p>Returns information about events that meet the specified filter criteria. Events are returned in a summary form and do not include the detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the <a>DescribeEventDetails</a> and <a>DescribeAffectedEntities</a> operations.</p> <p>If no filter criteria are specified, all events are returned. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent.</p>"]
    fn describe_events(&self,
                       input: &DescribeEventsRequest)
                       -> Result<DescribeEventsResponse, DescribeEventsError>;
}
/// A client for the AWSHealth API.
pub struct AWSHealthClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> AWSHealthClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        AWSHealthClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> AWSHealth for AWSHealthClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Returns a list of entities that have been affected by the specified events, based on the specified filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service. Events that have impact beyond that of the affected entities, or where the extent of impact is unknown, include at least one entity indicating this.</p> <p>At least one event ARN is required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p>"]
    fn describe_affected_entities
        (&self,
         input: &DescribeAffectedEntitiesRequest)
         -> Result<DescribeAffectedEntitiesResponse, DescribeAffectedEntitiesError> {
        let mut request = SignedRequest::new("POST", "health", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSHealth_20160804.DescribeAffectedEntities");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeAffectedEntitiesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeAffectedEntitiesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the number of entities that are affected by each of the specified events. If no events are specified, the counts of all affected entities are returned.</p>"]
    fn describe_entity_aggregates
        (&self,
         input: &DescribeEntityAggregatesRequest)
         -> Result<DescribeEntityAggregatesResponse, DescribeEntityAggregatesError> {
        let mut request = SignedRequest::new("POST", "health", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSHealth_20160804.DescribeEntityAggregates");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEntityAggregatesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeEntityAggregatesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the number of events of each event type (issue, scheduled change, and account notification). If no filter is specified, the counts of all events in each category are returned.</p>"]
    fn describe_event_aggregates
        (&self,
         input: &DescribeEventAggregatesRequest)
         -> Result<DescribeEventAggregatesResponse, DescribeEventAggregatesError> {
        let mut request = SignedRequest::new("POST", "health", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventAggregates");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEventAggregatesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeEventAggregatesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns detailed information about one or more specified events. Information includes standard event data (region, service, etc., as returned by <a>DescribeEvents</a>), a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included; to retrieve those, use the <a>DescribeAffectedEntities</a> operation.</p> <p>If a specified event cannot be retrieved, an error message is returned for that event.</p>"]
    fn describe_event_details
        (&self,
         input: &DescribeEventDetailsRequest)
         -> Result<DescribeEventDetailsResponse, DescribeEventDetailsError> {
        let mut request = SignedRequest::new("POST", "health", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventDetails");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEventDetailsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeEventDetailsError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the event types that meet the specified filter criteria. If no filter criteria are specified, all event types are returned, in no particular order.</p>"]
    fn describe_event_types(&self,
                            input: &DescribeEventTypesRequest)
                            -> Result<DescribeEventTypesResponse, DescribeEventTypesError> {
        let mut request = SignedRequest::new("POST", "health", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventTypes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEventTypesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeEventTypesError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about events that meet the specified filter criteria. Events are returned in a summary form and do not include the detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the <a>DescribeEventDetails</a> and <a>DescribeAffectedEntities</a> operations.</p> <p>If no filter criteria are specified, all events are returned. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent.</p>"]
    fn describe_events(&self,
                       input: &DescribeEventsRequest)
                       -> Result<DescribeEventsResponse, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "health", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEvents");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEventsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeEventsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
