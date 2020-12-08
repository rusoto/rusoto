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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl AWSHealthClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "health", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch(
        &self,
        request: SignedRequest,
    ) -> Result<HttpResponse, RusotoError<std::convert::Infallible>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await?;
            return Err(RusotoError::Unknown(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>Information about an entity that is affected by a Health event.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p><p>A map of entity tags attached to the affected entity.</p> <note> <p>Currently, the <code>tags</code> property isn&#39;t supported.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A range of dates and times that is used by the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EventFilter.html">EventFilter</a> and <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EntityFilter.html">EntityFilter</a> objects. If <code>from</code> is set and <code>to</code> is set: match items where the timestamp (<code>startTime</code>, <code>endTime</code>, or <code>lastUpdatedTime</code>) is between <code>from</code> and <code>to</code> inclusive. If <code>from</code> is set and <code>to</code> is not set: match items where the timestamp value is equal to or after <code>from</code>. If <code>from</code> is not set and <code>to</code> is set: match items where the timestamp value is equal to or before <code>to</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAffectedAccountsForOrganizationResponse {
    /// <p>A JSON set of elements of the affected accounts.</p>
    #[serde(rename = "affectedAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_accounts: Option<Vec<String>>,
    /// <p><p>This parameter specifies if the AWS Health event is a public AWS service event or an account-specific event.</p> <ul> <li> <p>If the <code>eventScopeCode</code> value is <code>PUBLIC</code>, then the <code>affectedAccounts</code> value is always empty.</p> </li> <li> <p>If the <code>eventScopeCode</code> value is <code>ACCOUNT_SPECIFIC</code>, then the <code>affectedAccounts</code> value lists the affected AWS accounts in your organization. For example, if an event affects a service such as Amazon Elastic Compute Cloud and you have AWS accounts that use that service, those account IDs appear in the response.</p> </li> <li> <p>If the <code>eventScopeCode</code> value is <code>NONE</code>, then the <code>eventArn</code> that you specified in the request is invalid or doesn&#39;t exist.</p> </li> </ul></p>
    #[serde(rename = "eventScopeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_scope_code: Option<String>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAffectedEntitiesForOrganizationResponse {
    /// <p>A JSON set of elements including the <code>awsAccountId</code> and its <code>entityArn</code>, <code>entityValue</code> and its <code>entityArn</code>, <code>lastUpdatedTime</code>, and <code>statusCode</code>.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEntityAggregatesRequest {
    /// <p>A list of event ARNs (unique identifiers). For example: <code>"arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-CDE456", "arn:aws:health:us-west-1::event/EBS/AWS_EBS_LOST_VOLUME/AWS_EBS_LOST_VOLUME_CHI789_JKL101"</code> </p>
    #[serde(rename = "eventArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEntityAggregatesResponse {
    /// <p>The number of entities that are affected by each of the specified events.</p>
    #[serde(rename = "entityAggregates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_aggregates: Option<Vec<EntityAggregate>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHealthServiceStatusForOrganizationResponse {
    /// <p>Information about the status of enabling or disabling AWS Health Organizational View in your organization.</p> <p>Valid values are <code>ENABLED | DISABLED | PENDING</code>. </p>
    #[serde(rename = "healthServiceAccessStatusForOrganization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_service_access_status_for_organization: Option<String>,
}

/// <p>The number of entities that are affected by one or more events. Returned by the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEntityAggregates.html">DescribeEntityAggregates</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntityAggregate {
    /// <p>The number of entities that match the criteria for the specified events.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arn: Option<String>,
}

/// <p>The values to use to filter results from the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EntityFilter.html">EntityFilter</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p><p>A map of entity tags attached to the affected entity.</p> <note> <p>Currently, the <code>tags</code> property isn&#39;t supported.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<::std::collections::HashMap<String, String>>>,
}

/// <p>Summary information about an AWS Health event.</p> <p>AWS Health events can be public or account-specific:</p> <ul> <li> <p> <i>Public events</i> might be service events that are not specific to an AWS account. For example, if there is an issue with an AWS Region, AWS Health provides information about the event, even if you don't use services or resources in that Region.</p> </li> <li> <p> <i>Account-specific</i> events are specific to either your AWS account or an account in your organization. For example, if there's an issue with Amazon Elastic Compute Cloud in a Region that you use, AWS Health provides information about the event and the affected resources in the account.</p> </li> </ul> <p>You can determine if an event is public or account-specific by using the <code>eventScopeCode</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html#AWSHealth-Type-Event-eventScopeCode">eventScopeCode</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p><p>This parameter specifies if the AWS Health event is a public AWS service event or an account-specific event.</p> <ul> <li> <p>If the <code>eventScopeCode</code> value is <code>PUBLIC</code>, then the <code>affectedAccounts</code> value is always empty.</p> </li> <li> <p>If the <code>eventScopeCode</code> value is <code>ACCOUNT_SPECIFIC</code>, then the <code>affectedAccounts</code> value lists the affected AWS accounts in your organization. For example, if an event affects a service such as Amazon Elastic Compute Cloud and you have AWS accounts that use that service, those account IDs appear in the response.</p> </li> <li> <p>If the <code>eventScopeCode</code> value is <code>NONE</code>, then the <code>eventArn</code> that you specified in the request is invalid or doesn&#39;t exist.</p> </li> </ul></p>
    #[serde(rename = "eventScopeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_scope_code: Option<String>,
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

/// <p>The values used to filter results from the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetailsForOrganization.html">DescribeEventDetailsForOrganization</a> and <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a> operations.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventAccountFilter {
    /// <p>The 12-digit AWS account numbers that contains the affected entities.</p>
    #[serde(rename = "awsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The unique identifier for the event. Format: <code>arn:aws:health:<i>event-region</i>::event/<i>SERVICE</i>/<i>EVENT_TYPE_CODE</i>/<i>EVENT_TYPE_PLUS_ID</i> </code>. Example: <code>Example: arn:aws:health:us-east-1::event/EC2/EC2_INSTANCE_RETIREMENT_SCHEDULED/EC2_INSTANCE_RETIREMENT_SCHEDULED_ABC123-DEF456</code> </p>
    #[serde(rename = "eventArn")]
    pub event_arn: String,
}

/// <p>The number of events of each issue type. Returned by the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventAggregates.html">DescribeEventAggregates</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>Detailed information about an event. A combination of an <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a> object, an <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EventDescription.html">EventDescription</a> object, and additional metadata about the event. Returned by the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetails.html">DescribeEventDetails</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>Error information returned when a <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetails.html">DescribeEventDetails</a> operation cannot find a specified event.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>The values to use to filter results from the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEvents.html">DescribeEvents</a> and <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventAggregates.html">DescribeEventAggregates</a> operations.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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
    /// <p><p>A map of entity tags attached to the affected entity.</p> <note> <p>Currently, the <code>tags</code> property isn&#39;t supported.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<::std::collections::HashMap<String, String>>>,
}

/// <p>The values to use to filter results from the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventTypes.html">DescribeEventTypes</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

/// <p>Error information returned when a <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a> operation cannot find or process a specific entity.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>Summary information about an event, returned by the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventsForOrganization.html">DescribeEventsForOrganization</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p><p>This parameter specifies if the AWS Health event is a public AWS service event or an account-specific event.</p> <ul> <li> <p>If the <code>eventScopeCode</code> value is <code>PUBLIC</code>, then the <code>affectedAccounts</code> value is always empty.</p> </li> <li> <p>If the <code>eventScopeCode</code> value is <code>ACCOUNT_SPECIFIC</code>, then the <code>affectedAccounts</code> value lists the affected AWS accounts in your organization. For example, if an event affects a service such as Amazon Elastic Compute Cloud and you have AWS accounts that use that service, those account IDs appear in the response.</p> </li> <li> <p>If the <code>eventScopeCode</code> value is <code>NONE</code>, then the <code>eventArn</code> that you specified in the request is invalid or doesn&#39;t exist.</p> </li> </ul></p>
    #[serde(rename = "eventScopeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_scope_code: Option<String>,
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

/// <p>Detailed information about an event. A combination of an <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a> object, an <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EventDescription.html">EventDescription</a> object, and additional metadata about the event. Returned by the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetailsForOrganization.html">DescribeEventDetailsForOrganization</a> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>Error information returned when a <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetailsForOrganization.html">DescribeEventDetailsForOrganization</a> operation cannot find a specified event.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationEventDetailsErrorItem {
    /// <p>Error information returned when a <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetailsForOrganization.html">DescribeEventDetailsForOrganization</a> operation cannot find a specified event.</p>
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

/// <p>The values to filter results from the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventsForOrganization.html">DescribeEventsForOrganization</a> operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OrganizationEventFilter {
    /// <p>A list of 12-digit AWS account numbers that contains the affected entities.</p>
    #[serde(rename = "awsAccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_ids: Option<Vec<String>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTimeRange>,
    /// <p>A list of entity ARNs (unique identifiers).</p>
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
    /// <p>A list of event type category codes (issue, scheduledChange, or accountNotification).</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeAffectedAccountsForOrganizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeAffectedEntitiesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeAffectedEntitiesForOrganizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeEntityAggregatesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeEventAggregatesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeEventDetailsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeEventDetailsForOrganizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeEventTypesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DescribeEventsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeEventsForOrganizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeHealthServiceStatusForOrganizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p> <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> is already in progress. Wait for the action to complete before trying again. To get the current status, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeHealthServiceStatusForOrganization.html">DescribeHealthServiceStatusForOrganization</a> operation.</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DisableHealthServiceAccessForOrganizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p> <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> is already in progress. Wait for the action to complete before trying again. To get the current status, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeHealthServiceStatusForOrganization.html">DescribeHealthServiceStatusForOrganization</a> operation.</p>
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

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<EnableHealthServiceAccessForOrganizationError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
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
    /// <p><p>Returns a list of accounts in the organization from AWS Organizations that are affected by the provided event. For more information about the different types of AWS Health events, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>. </p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization&#39;s master account.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_affected_accounts_for_organization(
        &self,
        input: DescribeAffectedAccountsForOrganizationRequest,
    ) -> Result<
        DescribeAffectedAccountsForOrganizationResponse,
        RusotoError<DescribeAffectedAccountsForOrganizationError>,
    >;

    /// <p><p>Returns a list of entities that have been affected by the specified events, based on the specified filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service. Events that have impact beyond that of the affected entities, or where the extent of impact is unknown, include at least one entity indicating this.</p> <p>At least one event ARN is required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_affected_entities(
        &self,
        input: DescribeAffectedEntitiesRequest,
    ) -> Result<DescribeAffectedEntitiesResponse, RusotoError<DescribeAffectedEntitiesError>>;

    /// <p><p>Returns a list of entities that have been affected by one or more events for one or more accounts in your organization in AWS Organizations, based on the filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service.</p> <p>At least one event Amazon Resource Name (ARN) and account ID are required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization&#39;s master account. </p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
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

    /// <p><p>Returns the number of events of each event type (issue, scheduled change, and account notification). If no filter is specified, the counts of all events in each category are returned.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_event_aggregates(
        &self,
        input: DescribeEventAggregatesRequest,
    ) -> Result<DescribeEventAggregatesResponse, RusotoError<DescribeEventAggregatesError>>;

    /// <p>Returns detailed information about one or more specified events. Information includes standard event data (Region, service, and so on, as returned by <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEvents.html">DescribeEvents</a>), a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included. To retrieve those, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntities.html">DescribeAffectedEntities</a> operation.</p> <p>If a specified event cannot be retrieved, an error message is returned for that event.</p>
    async fn describe_event_details(
        &self,
        input: DescribeEventDetailsRequest,
    ) -> Result<DescribeEventDetailsResponse, RusotoError<DescribeEventDetailsError>>;

    /// <p>Returns detailed information about one or more specified events for one or more accounts in your organization. Information includes standard event data (Region, service, and so on, as returned by <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventsForOrganization.html">DescribeEventsForOrganization</a>), a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included; to retrieve those, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a> operation.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization's master account.</p> <p>When you call the <code>DescribeEventDetailsForOrganization</code> operation, you specify the <code>organizationEventDetailFilters</code> object in the request. Depending on the AWS Health event type, note the following differences:</p> <ul> <li> <p>If the event is public, the <code>awsAccountId</code> parameter must be empty. If you specify an account ID for a public event, then an error message is returned. That's because the event might apply to all AWS accounts and isn't specific to an account in your organization.</p> </li> <li> <p>If the event is specific to an account, then you must specify the <code>awsAccountId</code> parameter in the request. If you don't specify an account ID, an error message returns because the event is specific to an AWS account in your organization. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p>
    async fn describe_event_details_for_organization(
        &self,
        input: DescribeEventDetailsForOrganizationRequest,
    ) -> Result<
        DescribeEventDetailsForOrganizationResponse,
        RusotoError<DescribeEventDetailsForOrganizationError>,
    >;

    /// <p><p>Returns the event types that meet the specified filter criteria. If no filter criteria are specified, all event types are returned, in no particular order.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_event_types(
        &self,
        input: DescribeEventTypesRequest,
    ) -> Result<DescribeEventTypesResponse, RusotoError<DescribeEventTypesError>>;

    /// <p><p> Returns information about events that meet the specified filter criteria. Events are returned in a summary form and do not include the detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetails.html">DescribeEventDetails</a> and <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntities.html">DescribeAffectedEntities</a> operations.</p> <p>If no filter criteria are specified, all events are returned. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent event.</p> <note> <ul> <li> <p>When you call the <code>DescribeEvents</code> operation and specify an entity for the <code>entityValues</code> parameter, AWS Health might return public events that aren&#39;t specific to that resource. For example, if you call <code>DescribeEvents</code> and specify an ID for an Amazon Elastic Compute Cloud (Amazon EC2) instance, AWS Health might return events that aren&#39;t specific to that resource or service. To get events that are specific to a service, use the <code>services</code> parameter in the <code>filter</code> object. For more information, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p> </li> <li> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </li> </ul> </note></p>
    async fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> Result<DescribeEventsResponse, RusotoError<DescribeEventsError>>;

    /// <p><p>Returns information about events across your organization in AWS Organizations. You can use the<code>filters</code> parameter to specify the events that you want to return. Events are returned in a summary form and don&#39;t include the affected accounts, detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the following operations:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedAccountsForOrganization.html">DescribeAffectedAccountsForOrganization</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetailsForOrganization.html">DescribeEventDetailsForOrganization</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a> </p> </li> </ul> <p>If you don&#39;t specify a <code>filter</code>, the <code>DescribeEventsForOrganizations</code> returns all events across your organization. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent event. </p> <p>For more information about the different types of AWS Health events, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization&#39;s master AWS account.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
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

    /// <p><p>Disables AWS Health from working with AWS Organizations. To call this operation, you must sign in as an AWS Identity and Access Management (IAM) user, assume an IAM role, or sign in as the root user (not recommended) in the organization&#39;s master AWS account. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating AWS Health events</a> in the <i>AWS Health User Guide</i>.</p> <p>This operation doesn&#39;t remove the service-linked role (SLR) from the AWS master account in your organization. You must use the IAM console, API, or AWS Command Line Interface (AWS CLI) to remove the SLR. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html#delete-service-linked-role">Deleting a Service-Linked Role</a> in the <i>IAM User Guide</i>.</p> <note> <p>You can also disable the organizational feature by using the Organizations <a href="https://docs.aws.amazon.com/organizations/latest/APIReference/API_DisableAWSServiceAccess.html">DisableAWSServiceAccess</a> API operation. After you call this operation, AWS Health stops aggregating events for all other AWS accounts in your organization. If you call the AWS Health API operations for organizational view, AWS Health returns an error. AWS Health continues to aggregate health events for your AWS account.</p> </note></p>
    async fn disable_health_service_access_for_organization(
        &self,
    ) -> Result<(), RusotoError<DisableHealthServiceAccessForOrganizationError>>;

    /// <p>Calling this operation enables AWS Health to work with AWS Organizations. This applies a service-linked role (SLR) to the master account in the organization. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating AWS Health events</a> in the <i>AWS Health User Guide</i>.</p>
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
    /// <p><p>Returns a list of accounts in the organization from AWS Organizations that are affected by the provided event. For more information about the different types of AWS Health events, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>. </p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization&#39;s master account.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_affected_accounts_for_organization(
        &self,
        input: DescribeAffectedAccountsForOrganizationRequest,
    ) -> Result<
        DescribeAffectedAccountsForOrganizationResponse,
        RusotoError<DescribeAffectedAccountsForOrganizationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeAffectedAccountsForOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeAffectedAccountsForOrganizationError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeAffectedAccountsForOrganizationResponse, _>()
    }

    /// <p><p>Returns a list of entities that have been affected by the specified events, based on the specified filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service. Events that have impact beyond that of the affected entities, or where the extent of impact is unknown, include at least one entity indicating this.</p> <p>At least one event ARN is required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_affected_entities(
        &self,
        input: DescribeAffectedEntitiesRequest,
    ) -> Result<DescribeAffectedEntitiesResponse, RusotoError<DescribeAffectedEntitiesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeAffectedEntities",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeAffectedEntitiesError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeAffectedEntitiesResponse, _>()
    }

    /// <p><p>Returns a list of entities that have been affected by one or more events for one or more accounts in your organization in AWS Organizations, based on the filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the AWS service.</p> <p>At least one event Amazon Resource Name (ARN) and account ID are required. Results are sorted by the <code>lastUpdatedTime</code> of the entity, starting with the most recent.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization&#39;s master account. </p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_affected_entities_for_organization(
        &self,
        input: DescribeAffectedEntitiesForOrganizationRequest,
    ) -> Result<
        DescribeAffectedEntitiesForOrganizationResponse,
        RusotoError<DescribeAffectedEntitiesForOrganizationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeAffectedEntitiesForOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeAffectedEntitiesForOrganizationError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeAffectedEntitiesForOrganizationResponse, _>()
    }

    /// <p>Returns the number of entities that are affected by each of the specified events. If no events are specified, the counts of all affected entities are returned.</p>
    async fn describe_entity_aggregates(
        &self,
        input: DescribeEntityAggregatesRequest,
    ) -> Result<DescribeEntityAggregatesResponse, RusotoError<DescribeEntityAggregatesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeEntityAggregates",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeEntityAggregatesError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEntityAggregatesResponse, _>()
    }

    /// <p><p>Returns the number of events of each event type (issue, scheduled change, and account notification). If no filter is specified, the counts of all events in each category are returned.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_event_aggregates(
        &self,
        input: DescribeEventAggregatesRequest,
    ) -> Result<DescribeEventAggregatesResponse, RusotoError<DescribeEventAggregatesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventAggregates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeEventAggregatesError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEventAggregatesResponse, _>()
    }

    /// <p>Returns detailed information about one or more specified events. Information includes standard event data (Region, service, and so on, as returned by <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEvents.html">DescribeEvents</a>), a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included. To retrieve those, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntities.html">DescribeAffectedEntities</a> operation.</p> <p>If a specified event cannot be retrieved, an error message is returned for that event.</p>
    async fn describe_event_details(
        &self,
        input: DescribeEventDetailsRequest,
    ) -> Result<DescribeEventDetailsResponse, RusotoError<DescribeEventDetailsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventDetails");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeEventDetailsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEventDetailsResponse, _>()
    }

    /// <p>Returns detailed information about one or more specified events for one or more accounts in your organization. Information includes standard event data (Region, service, and so on, as returned by <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventsForOrganization.html">DescribeEventsForOrganization</a>), a detailed event description, and possible additional metadata that depends upon the nature of the event. Affected entities are not included; to retrieve those, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a> operation.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization's master account.</p> <p>When you call the <code>DescribeEventDetailsForOrganization</code> operation, you specify the <code>organizationEventDetailFilters</code> object in the request. Depending on the AWS Health event type, note the following differences:</p> <ul> <li> <p>If the event is public, the <code>awsAccountId</code> parameter must be empty. If you specify an account ID for a public event, then an error message is returned. That's because the event might apply to all AWS accounts and isn't specific to an account in your organization.</p> </li> <li> <p>If the event is specific to an account, then you must specify the <code>awsAccountId</code> parameter in the request. If you don't specify an account ID, an error message returns because the event is specific to an AWS account in your organization. </p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p>
    async fn describe_event_details_for_organization(
        &self,
        input: DescribeEventDetailsForOrganizationRequest,
    ) -> Result<
        DescribeEventDetailsForOrganizationResponse,
        RusotoError<DescribeEventDetailsForOrganizationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeEventDetailsForOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeEventDetailsForOrganizationError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEventDetailsForOrganizationResponse, _>()
    }

    /// <p><p>Returns the event types that meet the specified filter criteria. If no filter criteria are specified, all event types are returned, in no particular order.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_event_types(
        &self,
        input: DescribeEventTypesRequest,
    ) -> Result<DescribeEventTypesResponse, RusotoError<DescribeEventTypesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEventTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeEventTypesError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEventTypesResponse, _>()
    }

    /// <p><p> Returns information about events that meet the specified filter criteria. Events are returned in a summary form and do not include the detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetails.html">DescribeEventDetails</a> and <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntities.html">DescribeAffectedEntities</a> operations.</p> <p>If no filter criteria are specified, all events are returned. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent event.</p> <note> <ul> <li> <p>When you call the <code>DescribeEvents</code> operation and specify an entity for the <code>entityValues</code> parameter, AWS Health might return public events that aren&#39;t specific to that resource. For example, if you call <code>DescribeEvents</code> and specify an ID for an Amazon Elastic Compute Cloud (Amazon EC2) instance, AWS Health might return events that aren&#39;t specific to that resource or service. To get events that are specific to a service, use the <code>services</code> parameter in the <code>filter</code> object. For more information, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p> </li> <li> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </li> </ul> </note></p>
    async fn describe_events(
        &self,
        input: DescribeEventsRequest,
    ) -> Result<DescribeEventsResponse, RusotoError<DescribeEventsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSHealth_20160804.DescribeEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeEventsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEventsResponse, _>()
    }

    /// <p><p>Returns information about events across your organization in AWS Organizations. You can use the<code>filters</code> parameter to specify the events that you want to return. Events are returned in a summary form and don&#39;t include the affected accounts, detailed description, any additional metadata that depends on the event type, or any affected resources. To retrieve that information, use the following operations:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedAccountsForOrganization.html">DescribeAffectedAccountsForOrganization</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeEventDetailsForOrganization.html">DescribeEventDetailsForOrganization</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a> </p> </li> </ul> <p>If you don&#39;t specify a <code>filter</code>, the <code>DescribeEventsForOrganizations</code> returns all events across your organization. Results are sorted by <code>lastModifiedTime</code>, starting with the most recent event. </p> <p>For more information about the different types of AWS Health events, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p> <p>Before you can call this operation, you must first enable AWS Health to work with AWS Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization&#39;s master AWS account.</p> <note> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </note></p>
    async fn describe_events_for_organization(
        &self,
        input: DescribeEventsForOrganizationRequest,
    ) -> Result<
        DescribeEventsForOrganizationResponse,
        RusotoError<DescribeEventsForOrganizationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeEventsForOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeEventsForOrganizationError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEventsForOrganizationResponse, _>()
    }

    /// <p>This operation provides status information on enabling or disabling AWS Health to work with your organization. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p>
    async fn describe_health_service_status_for_organization(
        &self,
    ) -> Result<
        DescribeHealthServiceStatusForOrganizationResponse,
        RusotoError<DescribeHealthServiceStatusForOrganizationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DescribeHealthServiceStatusForOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeHealthServiceStatusForOrganizationError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeHealthServiceStatusForOrganizationResponse, _>()
    }

    /// <p><p>Disables AWS Health from working with AWS Organizations. To call this operation, you must sign in as an AWS Identity and Access Management (IAM) user, assume an IAM role, or sign in as the root user (not recommended) in the organization&#39;s master AWS account. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating AWS Health events</a> in the <i>AWS Health User Guide</i>.</p> <p>This operation doesn&#39;t remove the service-linked role (SLR) from the AWS master account in your organization. You must use the IAM console, API, or AWS Command Line Interface (AWS CLI) to remove the SLR. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html#delete-service-linked-role">Deleting a Service-Linked Role</a> in the <i>IAM User Guide</i>.</p> <note> <p>You can also disable the organizational feature by using the Organizations <a href="https://docs.aws.amazon.com/organizations/latest/APIReference/API_DisableAWSServiceAccess.html">DisableAWSServiceAccess</a> API operation. After you call this operation, AWS Health stops aggregating events for all other AWS accounts in your organization. If you call the AWS Health API operations for organizational view, AWS Health returns an error. AWS Health continues to aggregate health events for your AWS account.</p> </note></p>
    async fn disable_health_service_access_for_organization(
        &self,
    ) -> Result<(), RusotoError<DisableHealthServiceAccessForOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.DisableHealthServiceAccessForOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DisableHealthServiceAccessForOrganizationError::refine)?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Calling this operation enables AWS Health to work with AWS Organizations. This applies a service-linked role (SLR) to the master account in the organization. To call this operation, you must sign in as an IAM user, assume an IAM role, or sign in as the root user (not recommended) in the organization's master account.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating AWS Health events</a> in the <i>AWS Health User Guide</i>.</p>
    async fn enable_health_service_access_for_organization(
        &self,
    ) -> Result<(), RusotoError<EnableHealthServiceAccessForOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSHealth_20160804.EnableHealthServiceAccessForOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(EnableHealthServiceAccessForOrganizationError::refine)?;
        std::mem::drop(response);
        Ok(())
    }
}
