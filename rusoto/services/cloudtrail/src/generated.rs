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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Specifies the tags to add to a trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsRequest {
    /// <p>Specifies the ARN of the trail to which one or more tags will be added. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Contains a list of CloudTrail tags, up to a limit of 50</p>
    #[serde(rename = "TagsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddTagsResponse {}

/// <p>Specifies the settings for each trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTrailRequest {
    /// <p>Specifies a log group name using an Amazon Resource Name (ARN), a unique identifier that represents the log group to which CloudTrail logs will be delivered. Not required unless you specify CloudWatchLogsRoleArn.</p>
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// <p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// <p><p>Specifies whether log file integrity validation is enabled. The default is false.</p> <note> <p>When you disable log file integrity validation, the chain of digest files is broken after one hour. CloudTrail will not create digest files for log files that were delivered during a period in which log file integrity validation was disabled. For example, if you enable log file integrity validation at noon on January 1, disable it at noon on January 2, and re-enable it at noon on January 10, digest files will not be created for the log files delivered from noon on January 2 to noon on January 10. The same applies whenever you stop CloudTrail logging or delete a trail.</p> </note></p>
    #[serde(rename = "EnableLogFileValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_log_file_validation: Option<bool>,
    /// <p>Specifies whether the trail is publishing events from global services such as IAM to the log files.</p>
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    /// <p>Specifies whether the trail is created in the current region or in all regions. The default is false, which creates a trail only in the region where you are signed in. As a best practice, consider creating trails that log events in all regions.</p>
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    /// <p>Specifies whether the trail is created for all accounts in an organization in AWS Organizations, or only for the current AWS account. The default is false, and cannot be true unless the call is made on behalf of an AWS account that is the master account for an organization in AWS Organizations.</p>
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    /// <p><p>Specifies the KMS key ID to use to encrypt the logs delivered by CloudTrail. The value can be an alias name prefixed by &quot;alias/&quot;, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <p>Examples:</p> <ul> <li> <p>alias/MyAliasName</p> </li> <li> <p>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</p> </li> <li> <p>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>12345678-1234-1234-1234-123456789012</p> </li> </ul></p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p><p>Specifies the name of the trail. The name must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies the name of the Amazon S3 bucket designated for publishing log files. See <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/create_trail_naming_policy.html">Amazon S3 Bucket Naming Requirements</a>.</p>
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
    /// <p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html">Finding Your CloudTrail Log Files</a>. The maximum length is 200 characters.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>Specifies the name of the Amazon SNS topic defined for notification of log file delivery. The maximum length is 256 characters.</p>
    #[serde(rename = "SnsTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
    #[serde(rename = "TagsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTrailResponse {
    /// <p>Specifies the Amazon Resource Name (ARN) of the log group to which CloudTrail logs will be delivered.</p>
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// <p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// <p>Specifies whether the trail is publishing events from global services such as IAM to the log files.</p>
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    /// <p>Specifies whether the trail exists in one region or in all regions.</p>
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    /// <p>Specifies whether the trail is an organization trail.</p>
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    /// <p>Specifies the KMS key ID that encrypts the logs delivered by CloudTrail. The value is a fully specified ARN to a KMS key in the format:</p> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Specifies whether log file integrity validation is enabled.</p>
    #[serde(rename = "LogFileValidationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,
    /// <p>Specifies the name of the trail.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies the name of the Amazon S3 bucket designated for publishing log files.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html">Finding Your CloudTrail Log Files</a>.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>Specifies the ARN of the Amazon SNS topic that CloudTrail uses to send notifications when log files are delivered. The format of a topic ARN is:</p> <p> <code>arn:aws:sns:us-east-2:123456789012:MyTopic</code> </p>
    #[serde(rename = "SnsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>Specifies the ARN of the trail that was created. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// <p><p>The Amazon S3 buckets or AWS Lambda functions that you specify in your event selectors for your trail to log data events. Data events provide information about the resource operations performed on or within a resource itself. These are also known as data plane operations. You can specify up to 250 data resources for a trail.</p> <note> <p>The total number of allowed data resources is 250. This number can be distributed between 1 and 5 event selectors, but the total cannot exceed 250 across all selectors.</p> </note> <p>The following example demonstrates how logging works when you configure logging of all data events for an S3 bucket named <code>bucket-1</code>. In this example, the CloudTrail user specified an empty prefix, and the option to log both <code>Read</code> and <code>Write</code> data events.</p> <ol> <li> <p>A user uploads an image file to <code>bucket-1</code>.</p> </li> <li> <p>The <code>PutObject</code> API operation is an Amazon S3 object-level API. It is recorded as a data event in CloudTrail. Because the CloudTrail user specified an S3 bucket with an empty prefix, events that occur on any object in that bucket are logged. The trail processes and logs the event.</p> </li> <li> <p>A user uploads an object to an Amazon S3 bucket named <code>arn:aws:s3:::bucket-2</code>.</p> </li> <li> <p>The <code>PutObject</code> API operation occurred for an object in an S3 bucket that the CloudTrail user didn&#39;t specify for the trail. The trail doesn’t log the event.</p> </li> </ol> <p>The following example demonstrates how logging works when you configure logging of AWS Lambda data events for a Lambda function named <i>MyLambdaFunction</i>, but not for all AWS Lambda functions.</p> <ol> <li> <p>A user runs a script that includes a call to the <i>MyLambdaFunction</i> function and the <i>MyOtherLambdaFunction</i> function.</p> </li> <li> <p>The <code>Invoke</code> API operation on <i>MyLambdaFunction</i> is an AWS Lambda API. It is recorded as a data event in CloudTrail. Because the CloudTrail user specified logging data events for <i>MyLambdaFunction</i>, any invocations of that function are logged. The trail processes and logs the event. </p> </li> <li> <p>The <code>Invoke</code> API operation on <i>MyOtherLambdaFunction</i> is an AWS Lambda API. Because the CloudTrail user did not specify logging data events for all Lambda functions, the <code>Invoke</code> operation for <i>MyOtherLambdaFunction</i> does not match the function specified for the trail. The trail doesn’t log the event. </p> </li> </ol></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataResource {
    /// <p>The resource type in which you want to log data events. You can specify <code>AWS::S3::Object</code> or <code>AWS::Lambda::Function</code> resources.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p><p>An array of Amazon Resource Name (ARN) strings or partial ARN strings for the specified objects.</p> <ul> <li> <p>To log data events for all objects in all S3 buckets in your AWS account, specify the prefix as <code>arn:aws:s3:::</code>. </p> <note> <p>This will also enable logging of data event activity performed by any user or role in your AWS account, even if that activity is performed on a bucket that belongs to another AWS account. </p> </note> </li> <li> <p>To log data events for all objects in an S3 bucket, specify the bucket and an empty object prefix such as <code>arn:aws:s3:::bucket-1/</code>. The trail logs data events for all objects in this S3 bucket.</p> </li> <li> <p>To log data events for specific objects, specify the S3 bucket and object prefix such as <code>arn:aws:s3:::bucket-1/example-images</code>. The trail logs data events for objects in this S3 bucket that match the prefix.</p> </li> <li> <p>To log data events for all functions in your AWS account, specify the prefix as <code>arn:aws:lambda</code>.</p> <note> <p>This will also enable logging of <code>Invoke</code> activity performed by any user or role in your AWS account, even if that activity is performed on a function that belongs to another AWS account. </p> </note> </li> <li> <p>To log data events for a specific Lambda function, specify the function ARN.</p> <note> <p>Lambda function ARNs are exact. For example, if you specify a function ARN <i>arn:aws:lambda:us-west-2:111111111111:function:helloworld</i>, data events will only be logged for <i>arn:aws:lambda:us-west-2:111111111111:function:helloworld</i>. They will not be logged for <i>arn:aws:lambda:us-west-2:111111111111:function:helloworld2</i>.</p> </note> </li> </ul></p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The request that specifies the name of a trail to delete.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTrailRequest {
    /// <p>Specifies the name or the CloudTrail ARN of the trail to be deleted. The format of a trail ARN is: <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTrailResponse {}

/// <p>Returns information about the trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTrailsRequest {
    /// <p>Specifies whether to include shadow trails in the response. A shadow trail is the replication in a region of a trail that was created in a different region, or in the case of an organization trail, the replication of an organization trail in member accounts. If you do not include shadow trails, organization trails in a member account and region replication trails will not be returned. The default is true.</p>
    #[serde(rename = "includeShadowTrails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shadow_trails: Option<bool>,
    /// <p><p>Specifies a list of trail names, trail ARNs, or both, of the trails to describe. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p> <p>If an empty list is specified, information for the trail in the current region is returned.</p> <ul> <li> <p>If an empty list is specified and <code>IncludeShadowTrails</code> is false, then information for all trails in the current region is returned.</p> </li> <li> <p>If an empty list is specified and IncludeShadowTrails is null or true, then information for all trails in the current region and any associated shadow trails in other regions is returned.</p> </li> </ul> <note> <p>If one or more trail names are specified, information is returned only if the names match the names of trails belonging only to the current region. To return information about a trail in another region, you must specify its trail ARN.</p> </note></p>
    #[serde(rename = "trailNameList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name_list: Option<Vec<String>>,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTrailsResponse {
    /// <p>The list of trail objects. Trail objects with string values are only returned if values for the objects exist in a trail's configuration. For example, <code>SNSTopicName</code> and <code>SNSTopicARN</code> are only returned in results if a trail is configured to send SNS notifications. Similarly, <code>KMSKeyId</code> only appears in results if a trail's log files are encrypted with AWS KMS-managed keys.</p>
    #[serde(rename = "trailList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_list: Option<Vec<Trail>>,
}

/// <p>Contains information about an event that was returned by a lookup request. The result includes a representation of a CloudTrail event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Event {
    /// <p>The AWS access key ID that was used to sign the request. If the request was made with temporary security credentials, this is the access key ID of the temporary credentials.</p>
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// <p>A JSON string that contains a representation of the event returned.</p>
    #[serde(rename = "CloudTrailEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail_event: Option<String>,
    /// <p>The CloudTrail ID of the event returned.</p>
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p>The name of the event returned.</p>
    #[serde(rename = "EventName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// <p>The AWS service that the request was made to.</p>
    #[serde(rename = "EventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    /// <p>The date and time of the event returned.</p>
    #[serde(rename = "EventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    /// <p>Information about whether the event is a write event or a read event. </p>
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<String>,
    /// <p>A list of resources referenced by the event returned.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
    /// <p>A user name or role name of the requester that called the API in the event returned.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Use event selectors to further specify the management and data event settings for your trail. By default, trails created without specific event selectors will be configured to log all read and write management events, and no data events. When an event occurs in your account, CloudTrail evaluates the event selector for all trails. For each trail, if the event matches any event selector, the trail processes and logs the event. If the event doesn't match any event selector, the trail doesn't log the event.</p> <p>You can configure up to five event selectors for a trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventSelector {
    /// <p>CloudTrail supports data event logging for Amazon S3 objects and AWS Lambda functions. You can specify up to 250 resources for an individual event selector, but the total number of data resources cannot exceed 250 across all event selectors in a trail. This limit does not apply if you configure resource logging for all data events. </p> <p>For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-and-data-events-with-cloudtrail.html#logging-data-events">Data Events</a> and <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/WhatIsCloudTrail-Limits.html">Limits in AWS CloudTrail</a> in the <i>AWS CloudTrail User Guide</i>.</p>
    #[serde(rename = "DataResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_resources: Option<Vec<DataResource>>,
    /// <p>An optional list of service event sources from which you do not want management events to be logged on your trail. In this release, the list can be empty (disables the filter), or it can filter out AWS Key Management Service events by containing <code>"kms.amazonaws.com"</code>. By default, <code>ExcludeManagementEventSources</code> is empty, and AWS KMS events are included in events that are logged to your trail. </p>
    #[serde(rename = "ExcludeManagementEventSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_management_event_sources: Option<Vec<String>>,
    /// <p>Specify if you want your event selector to include management events for your trail.</p> <p> For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-and-data-events-with-cloudtrail.html#logging-management-events">Management Events</a> in the <i>AWS CloudTrail User Guide</i>.</p> <p>By default, the value is <code>true</code>.</p>
    #[serde(rename = "IncludeManagementEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_management_events: Option<bool>,
    /// <p>Specify if you want your trail to log read-only events, write-only events, or all. For example, the EC2 <code>GetConsoleOutput</code> is a read-only API operation and <code>RunInstances</code> is a write-only API operation.</p> <p> By default, the value is <code>All</code>.</p>
    #[serde(rename = "ReadWriteType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_write_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEventSelectorsRequest {
    /// <p>Specifies the name of the trail or trail ARN. If you specify a trail name, the string must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are not valid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul> <p>If you specify a trail ARN, it must be in the format:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "TrailName")]
    pub trail_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEventSelectorsResponse {
    /// <p>The event selectors that are configured for the trail.</p>
    #[serde(rename = "EventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_selectors: Option<Vec<EventSelector>>,
    /// <p>The specified trail ARN that has the event selectors.</p>
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInsightSelectorsRequest {
    /// <p>Specifies the name of the trail or trail ARN. If you specify a trail name, the string must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are not valid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul> <p>If you specify a trail ARN, it must be in the format:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "TrailName")]
    pub trail_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInsightSelectorsResponse {
    /// <p>A JSON string that contains the insight types you want to log on a trail. In this release, only <code>ApiCallRateInsight</code> is supported as an insight type.</p>
    #[serde(rename = "InsightSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_selectors: Option<Vec<InsightSelector>>,
    /// <p>The Amazon Resource Name (ARN) of a trail for which you want to get Insights selectors.</p>
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTrailRequest {
    /// <p>The name or the Amazon Resource Name (ARN) of the trail for which you want to retrieve settings information.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTrailResponse {
    #[serde(rename = "Trail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail: Option<Trail>,
}

/// <p>The name of a trail about which you want the current status.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTrailStatusRequest {
    /// <p>Specifies the name or the CloudTrail ARN of the trail for which you are requesting status. To get the status of a shadow trail (a replication of the trail in another region), you must specify its ARN. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTrailStatusResponse {
    /// <p>Whether the CloudTrail is currently logging AWS API calls.</p>
    #[serde(rename = "IsLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_logging: Option<bool>,
    /// <p>Displays any CloudWatch Logs error that CloudTrail encountered when attempting to deliver logs to CloudWatch Logs.</p>
    #[serde(rename = "LatestCloudWatchLogsDeliveryError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_cloud_watch_logs_delivery_error: Option<String>,
    /// <p>Displays the most recent date and time when CloudTrail delivered logs to CloudWatch Logs.</p>
    #[serde(rename = "LatestCloudWatchLogsDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_cloud_watch_logs_delivery_time: Option<f64>,
    /// <p>This field is no longer in use.</p>
    #[serde(rename = "LatestDeliveryAttemptSucceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_attempt_succeeded: Option<String>,
    /// <p>This field is no longer in use.</p>
    #[serde(rename = "LatestDeliveryAttemptTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_attempt_time: Option<String>,
    /// <p><p>Displays any Amazon S3 error that CloudTrail encountered when attempting to deliver log files to the designated bucket. For more information see the topic <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html">Error Responses</a> in the Amazon S3 API Reference. </p> <note> <p>This error occurs only when there is a problem with the destination S3 bucket and will not occur for timeouts. To resolve the issue, create a new bucket and call <code>UpdateTrail</code> to specify the new bucket, or fix the existing objects so that CloudTrail can again write to the bucket.</p> </note></p>
    #[serde(rename = "LatestDeliveryError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_error: Option<String>,
    /// <p>Specifies the date and time that CloudTrail last delivered log files to an account's Amazon S3 bucket.</p>
    #[serde(rename = "LatestDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_time: Option<f64>,
    /// <p><p>Displays any Amazon S3 error that CloudTrail encountered when attempting to deliver a digest file to the designated bucket. For more information see the topic <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html">Error Responses</a> in the Amazon S3 API Reference. </p> <note> <p>This error occurs only when there is a problem with the destination S3 bucket and will not occur for timeouts. To resolve the issue, create a new bucket and call <code>UpdateTrail</code> to specify the new bucket, or fix the existing objects so that CloudTrail can again write to the bucket.</p> </note></p>
    #[serde(rename = "LatestDigestDeliveryError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_digest_delivery_error: Option<String>,
    /// <p>Specifies the date and time that CloudTrail last delivered a digest file to an account's Amazon S3 bucket.</p>
    #[serde(rename = "LatestDigestDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_digest_delivery_time: Option<f64>,
    /// <p>This field is no longer in use.</p>
    #[serde(rename = "LatestNotificationAttemptSucceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_attempt_succeeded: Option<String>,
    /// <p>This field is no longer in use.</p>
    #[serde(rename = "LatestNotificationAttemptTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_attempt_time: Option<String>,
    /// <p>Displays any Amazon SNS error that CloudTrail encountered when attempting to send a notification. For more information about Amazon SNS errors, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/welcome.html">Amazon SNS Developer Guide</a>. </p>
    #[serde(rename = "LatestNotificationError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_error: Option<String>,
    /// <p>Specifies the date and time of the most recent Amazon SNS notification that CloudTrail has written a new log file to an account's Amazon S3 bucket.</p>
    #[serde(rename = "LatestNotificationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_notification_time: Option<f64>,
    /// <p>Specifies the most recent date and time when CloudTrail started recording API calls for an AWS account.</p>
    #[serde(rename = "StartLoggingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_logging_time: Option<f64>,
    /// <p>Specifies the most recent date and time when CloudTrail stopped recording API calls for an AWS account.</p>
    #[serde(rename = "StopLoggingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_logging_time: Option<f64>,
    /// <p>This field is no longer in use.</p>
    #[serde(rename = "TimeLoggingStarted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_logging_started: Option<String>,
    /// <p>This field is no longer in use.</p>
    #[serde(rename = "TimeLoggingStopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_logging_stopped: Option<String>,
}

/// <p>A JSON string that contains a list of insight types that are logged on a trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsightSelector {
    /// <p>The type of insights to log on a trail. In this release, only <code>ApiCallRateInsight</code> is supported as an insight type.</p>
    #[serde(rename = "InsightType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_type: Option<String>,
}

/// <p>Requests the public keys for a specified time range.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPublicKeysRequest {
    /// <p>Optionally specifies, in UTC, the end of the time range to look up public keys for CloudTrail digest files. If not specified, the current time is used.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Reserved for future use.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optionally specifies, in UTC, the start of the time range to look up public keys for CloudTrail digest files. If not specified, the current time is used, and the current public key is returned.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPublicKeysResponse {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Contains an array of PublicKey objects.</p> <note> <p>The returned public keys may have validity time ranges that overlap.</p> </note></p>
    #[serde(rename = "PublicKeyList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_list: Option<Vec<PublicKey>>,
}

/// <p>Specifies a list of trail tags to return.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsRequest {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies a list of trail ARNs whose tags will be listed. The list has a limit of 20 ARNs. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "ResourceIdList")]
    pub resource_id_list: Vec<String>,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsResponse {
    /// <p>Reserved for future use.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of resource tags.</p>
    #[serde(rename = "ResourceTagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tag_list: Option<Vec<ResourceTag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTrailsRequest {
    /// <p>The token to use to get the next page of results after a previous API call. This token must be passed in with the same parameters that were specified in the the original call. For example, if the original call specified an AttributeKey of 'Username' with a value of 'root', the call with NextToken should include those same parameters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTrailsResponse {
    /// <p>The token to use to get the next page of results after a previous API call. If the token does not appear, there are no more results to return. The token must be passed in with the same parameters as the previous call. For example, if the original call specified an AttributeKey of 'Username' with a value of 'root', the call with NextToken should include those same parameters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the name, ARN, and home region of trails in the current account.</p>
    #[serde(rename = "Trails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trails: Option<Vec<TrailInfo>>,
}

/// <p>Specifies an attribute and value that filter the events returned.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LookupAttribute {
    /// <p>Specifies an attribute on which to filter the events returned.</p>
    #[serde(rename = "AttributeKey")]
    pub attribute_key: String,
    /// <p>Specifies a value for the specified AttributeKey.</p>
    #[serde(rename = "AttributeValue")]
    pub attribute_value: String,
}

/// <p>Contains a request for LookupEvents.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LookupEventsRequest {
    /// <p>Specifies that only events that occur before or at the specified time are returned. If the specified end time is before the specified start time, an error is returned.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Specifies the event category. If you do not specify an event category, events of the category are not returned in the response. For example, if you do not specify <code>insight</code> as the value of <code>EventCategory</code>, no Insights events are returned.</p>
    #[serde(rename = "EventCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category: Option<String>,
    /// <p>Contains a list of lookup attributes. Currently the list can contain only one item.</p>
    #[serde(rename = "LookupAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_attributes: Option<Vec<LookupAttribute>>,
    /// <p>The number of events to return. Possible values are 1 through 50. The default is 50.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to use to get the next page of results after a previous API call. This token must be passed in with the same parameters that were specified in the the original call. For example, if the original call specified an AttributeKey of 'Username' with a value of 'root', the call with NextToken should include those same parameters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies that only events that occur after or at the specified time are returned. If the specified start time is after the specified end time, an error is returned.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p>Contains a response to a LookupEvents action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LookupEventsResponse {
    /// <p>A list of events returned based on the lookup attributes specified and the CloudTrail event. The events list is sorted by time. The most recent event is listed first.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p>The token to use to get the next page of results after a previous API call. If the token does not appear, there are no more results to return. The token must be passed in with the same parameters as the previous call. For example, if the original call specified an AttributeKey of 'Username' with a value of 'root', the call with NextToken should include those same parameters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains information about a returned public key.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PublicKey {
    /// <p>The fingerprint of the public key.</p>
    #[serde(rename = "Fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// <p>The ending time of validity of the public key.</p>
    #[serde(rename = "ValidityEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_end_time: Option<f64>,
    /// <p>The starting time of validity of the public key.</p>
    #[serde(rename = "ValidityStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_start_time: Option<f64>,
    /// <p>The DER encoded public key value in PKCS#1 format.</p>
    #[serde(rename = "Value")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEventSelectorsRequest {
    /// <p>Specifies the settings for your event selectors. You can configure up to five event selectors for a trail.</p>
    #[serde(rename = "EventSelectors")]
    pub event_selectors: Vec<EventSelector>,
    /// <p>Specifies the name of the trail or trail ARN. If you specify a trail name, the string must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul> <p>If you specify a trail ARN, it must be in the format:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "TrailName")]
    pub trail_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEventSelectorsResponse {
    /// <p>Specifies the event selectors configured for your trail.</p>
    #[serde(rename = "EventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_selectors: Option<Vec<EventSelector>>,
    /// <p>Specifies the ARN of the trail that was updated with event selectors. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutInsightSelectorsRequest {
    /// <p>A JSON string that contains the insight types you want to log on a trail. In this release, only <code>ApiCallRateInsight</code> is supported as an insight type.</p>
    #[serde(rename = "InsightSelectors")]
    pub insight_selectors: Vec<InsightSelector>,
    /// <p>The name of the CloudTrail trail for which you want to change or add Insights selectors.</p>
    #[serde(rename = "TrailName")]
    pub trail_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutInsightSelectorsResponse {
    /// <p>A JSON string that contains the insight types you want to log on a trail. In this release, only <code>ApiCallRateInsight</code> is supported as an insight type.</p>
    #[serde(rename = "InsightSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_selectors: Option<Vec<InsightSelector>>,
    /// <p>The Amazon Resource Name (ARN) of a trail for which you want to change or add Insights selectors.</p>
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// <p>Specifies the tags to remove from a trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsRequest {
    /// <p>Specifies the ARN of the trail from which tags should be removed. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>Specifies a list of tags to be removed.</p>
    #[serde(rename = "TagsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTagsResponse {}

/// <p>Specifies the type and name of a resource referenced by an event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Resource {
    /// <p>The name of the resource referenced by the event returned. These are user-created names whose values will depend on the environment. For example, the resource name might be "auto-scaling-test-group" for an Auto Scaling Group or "i-1234567" for an EC2 Instance.</p>
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of a resource referenced by the event returned. When the resource type cannot be determined, null is returned. Some examples of resource types are: <b>Instance</b> for EC2, <b>Trail</b> for CloudTrail, <b>DBInstance</b> for RDS, and <b>AccessKey</b> for IAM. To learn more about how to look up and filter events by the resource types supported for a service, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/view-cloudtrail-events-console.html#filtering-cloudtrail-events">Filtering CloudTrail Events</a>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>A resource tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceTag {
    /// <p>Specifies the ARN of the resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>A list of tags.</p>
    #[serde(rename = "TagsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_list: Option<Vec<Tag>>,
}

/// <p>The request to CloudTrail to start logging AWS API calls for an account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartLoggingRequest {
    /// <p>Specifies the name or the CloudTrail ARN of the trail for which CloudTrail logs AWS API calls. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartLoggingResponse {}

/// <p>Passes the request to CloudTrail to stop logging AWS API calls for the specified account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopLoggingRequest {
    /// <p>Specifies the name or the CloudTrail ARN of the trail for which CloudTrail will stop logging AWS API calls. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopLoggingResponse {}

/// <p>A custom key-value pair associated with a resource such as a CloudTrail trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key in a key-value pair. The key must be must be no longer than 128 Unicode characters. The key must be unique for the resource to which it applies.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value in a key-value pair of a tag. The value must be no longer than 256 Unicode characters.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The settings for a trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Trail {
    /// <p>Specifies an Amazon Resource Name (ARN), a unique identifier that represents the log group to which CloudTrail logs will be delivered.</p>
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// <p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// <p>Specifies if the trail has custom event selectors.</p>
    #[serde(rename = "HasCustomEventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_custom_event_selectors: Option<bool>,
    /// <p>Specifies whether a trail has insight types specified in an <code>InsightSelector</code> list.</p>
    #[serde(rename = "HasInsightSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_insight_selectors: Option<bool>,
    /// <p>The region in which the trail was created.</p>
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// <p>Set to <b>True</b> to include AWS API calls from AWS global services such as IAM. Otherwise, <b>False</b>.</p>
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    /// <p>Specifies whether the trail exists only in one region or exists in all regions.</p>
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    /// <p>Specifies whether the trail is an organization trail.</p>
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    /// <p>Specifies the KMS key ID that encrypts the logs delivered by CloudTrail. The value is a fully specified ARN to a KMS key in the format:</p> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Specifies whether log file validation is enabled.</p>
    #[serde(rename = "LogFileValidationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,
    /// <p>Name of the trail set by calling <a>CreateTrail</a>. The maximum length is 128 characters.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Name of the Amazon S3 bucket into which CloudTrail delivers your trail files. See <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/create_trail_naming_policy.html">Amazon S3 Bucket Naming Requirements</a>.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html">Finding Your CloudTrail Log Files</a>.The maximum length is 200 characters.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>Specifies the ARN of the Amazon SNS topic that CloudTrail uses to send notifications when log files are delivered. The format of a topic ARN is:</p> <p> <code>arn:aws:sns:us-east-2:123456789012:MyTopic</code> </p>
    #[serde(rename = "SnsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>Specifies the ARN of the trail. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// <p>Information about a CloudTrail trail, including the trail's name, home region, and Amazon Resource Name (ARN).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrailInfo {
    /// <p>The AWS region in which a trail was created.</p>
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// <p>The name of a trail.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of a trail.</p>
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// <p>Specifies settings to update for the trail.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTrailRequest {
    /// <p>Specifies a log group name using an Amazon Resource Name (ARN), a unique identifier that represents the log group to which CloudTrail logs will be delivered. Not required unless you specify CloudWatchLogsRoleArn.</p>
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// <p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// <p><p>Specifies whether log file validation is enabled. The default is false.</p> <note> <p>When you disable log file integrity validation, the chain of digest files is broken after one hour. CloudTrail will not create digest files for log files that were delivered during a period in which log file integrity validation was disabled. For example, if you enable log file integrity validation at noon on January 1, disable it at noon on January 2, and re-enable it at noon on January 10, digest files will not be created for the log files delivered from noon on January 2 to noon on January 10. The same applies whenever you stop CloudTrail logging or delete a trail.</p> </note></p>
    #[serde(rename = "EnableLogFileValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_log_file_validation: Option<bool>,
    /// <p>Specifies whether the trail is publishing events from global services such as IAM to the log files.</p>
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    /// <p>Specifies whether the trail applies only to the current region or to all regions. The default is false. If the trail exists only in the current region and this value is set to true, shadow trails (replications of the trail) will be created in the other regions. If the trail exists in all regions and this value is set to false, the trail will remain in the region where it was created, and its shadow trails in other regions will be deleted. As a best practice, consider using trails that log events in all regions.</p>
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    /// <p>Specifies whether the trail is applied to all accounts in an organization in AWS Organizations, or only for the current AWS account. The default is false, and cannot be true unless the call is made on behalf of an AWS account that is the master account for an organization in AWS Organizations. If the trail is not an organization trail and this is set to true, the trail will be created in all AWS accounts that belong to the organization. If the trail is an organization trail and this is set to false, the trail will remain in the current AWS account but be deleted from all member accounts in the organization.</p>
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    /// <p><p>Specifies the KMS key ID to use to encrypt the logs delivered by CloudTrail. The value can be an alias name prefixed by &quot;alias/&quot;, a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <p>Examples:</p> <ul> <li> <p>alias/MyAliasName</p> </li> <li> <p>arn:aws:kms:us-east-2:123456789012:alias/MyAliasName</p> </li> <li> <p>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>12345678-1234-1234-1234-123456789012</p> </li> </ul></p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Specifies the name of the trail or trail ARN. If <code>Name</code> is a trail name, the string must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul> <p>If <code>Name</code> is a trail ARN, it must be in the format:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies the name of the Amazon S3 bucket designated for publishing log files. See <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/create_trail_naming_policy.html">Amazon S3 Bucket Naming Requirements</a>.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html">Finding Your CloudTrail Log Files</a>. The maximum length is 200 characters.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>Specifies the name of the Amazon SNS topic defined for notification of log file delivery. The maximum length is 256 characters.</p>
    #[serde(rename = "SnsTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
}

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTrailResponse {
    /// <p>Specifies the Amazon Resource Name (ARN) of the log group to which CloudTrail logs will be delivered.</p>
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// <p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// <p>Specifies whether the trail is publishing events from global services such as IAM to the log files.</p>
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    /// <p>Specifies whether the trail exists in one region or in all regions.</p>
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    /// <p>Specifies whether the trail is an organization trail.</p>
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    /// <p>Specifies the KMS key ID that encrypts the logs delivered by CloudTrail. The value is a fully specified ARN to a KMS key in the format:</p> <p> <code>arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Specifies whether log file integrity validation is enabled.</p>
    #[serde(rename = "LogFileValidationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,
    /// <p>Specifies the name of the trail.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies the name of the Amazon S3 bucket designated for publishing log files.</p>
    #[serde(rename = "S3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    /// <p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html">Finding Your CloudTrail Log Files</a>.</p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// <p>Specifies the ARN of the Amazon SNS topic that CloudTrail uses to send notifications when log files are delivered. The format of a topic ARN is:</p> <p> <code>arn:aws:sns:us-east-2:123456789012:MyTopic</code> </p>
    #[serde(rename = "SnsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>Specifies the ARN of the trail that was updated. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    #[serde(rename = "TrailARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// Errors returned by AddTags
#[derive(Debug, PartialEq)]
pub enum AddTagsError {
    /// <p>This exception is thrown when an operation is called with an invalid trail ARN. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    CloudTrailARNInvalid(String),
    /// <p>This exception is thrown when the specified tag key or values are not valid. It can also occur if there are duplicate tags or too many tags on the resource.</p>
    InvalidTagParameter(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the specified resource is not found.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the specified resource type is not supported by CloudTrail.</p>
    ResourceTypeNotSupported(String),
    /// <p>The number of tags per trail has exceeded the permitted amount. Currently, the limit is 50.</p>
    TagsLimitExceeded(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl AddTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudTrailARNInvalidException" => {
                    return RusotoError::Service(AddTagsError::CloudTrailARNInvalid(err.msg))
                }
                "InvalidTagParameterException" => {
                    return RusotoError::Service(AddTagsError::InvalidTagParameter(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(AddTagsError::InvalidTrailName(err.msg))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(AddTagsError::NotOrganizationMasterAccount(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(AddTagsError::OperationNotPermitted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddTagsError::ResourceNotFound(err.msg))
                }
                "ResourceTypeNotSupportedException" => {
                    return RusotoError::Service(AddTagsError::ResourceTypeNotSupported(err.msg))
                }
                "TagsLimitExceededException" => {
                    return RusotoError::Service(AddTagsError::TagsLimitExceeded(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(AddTagsError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AddTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsError::CloudTrailARNInvalid(ref cause) => write!(f, "{}", cause),
            AddTagsError::InvalidTagParameter(ref cause) => write!(f, "{}", cause),
            AddTagsError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            AddTagsError::NotOrganizationMasterAccount(ref cause) => write!(f, "{}", cause),
            AddTagsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            AddTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddTagsError::ResourceTypeNotSupported(ref cause) => write!(f, "{}", cause),
            AddTagsError::TagsLimitExceeded(ref cause) => write!(f, "{}", cause),
            AddTagsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsError {}
/// Errors returned by CreateTrail
#[derive(Debug, PartialEq)]
pub enum CreateTrailError {
    /// <p>This exception is thrown when trusted access has not been enabled between AWS CloudTrail and AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Enabling Trusted Access with Other AWS Services</a> and <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>. </p>
    CloudTrailAccessNotEnabled(String),
    /// <p>Cannot set a CloudWatch Logs delivery for this region.</p>
    CloudWatchLogsDeliveryUnavailable(String),
    /// <p>This exception is thrown when the IAM user or role that is used to create the organization trail is lacking one or more required permissions for creating an organization trail in a required service. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    InsufficientDependencyServiceAccessPermission(String),
    /// <p>This exception is thrown when the policy on the S3 bucket or KMS key is not sufficient.</p>
    InsufficientEncryptionPolicy(String),
    /// <p>This exception is thrown when the policy on the S3 bucket is not sufficient.</p>
    InsufficientS3BucketPolicy(String),
    /// <p>This exception is thrown when the policy on the SNS topic is not sufficient.</p>
    InsufficientSnsTopicPolicy(String),
    /// <p>This exception is thrown when the provided CloudWatch log group is not valid.</p>
    InvalidCloudWatchLogsLogGroupArn(String),
    /// <p>This exception is thrown when the provided role is not valid.</p>
    InvalidCloudWatchLogsRoleArn(String),
    /// <p>This exception is thrown when the KMS key ARN is invalid.</p>
    InvalidKmsKeyId(String),
    /// <p>This exception is thrown when the combination of parameters provided is not valid.</p>
    InvalidParameterCombination(String),
    /// <p>This exception is thrown when the provided S3 bucket name is not valid.</p>
    InvalidS3BucketName(String),
    /// <p>This exception is thrown when the provided S3 prefix is not valid.</p>
    InvalidS3Prefix(String),
    /// <p>This exception is thrown when the provided SNS topic name is not valid.</p>
    InvalidSnsTopicName(String),
    /// <p>This exception is thrown when the specified tag key or values are not valid. It can also occur if there are duplicate tags or too many tags on the resource.</p>
    InvalidTagParameter(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when there is an issue with the specified KMS key and the trail can’t be updated.</p>
    Kms(String),
    /// <p>This exception is no longer in use.</p>
    KmsKeyDisabled(String),
    /// <p>This exception is thrown when the KMS key does not exist, or when the S3 bucket and the KMS key are not in the same region.</p>
    KmsKeyNotFound(String),
    /// <p>This exception is thrown when the maximum number of trails is reached.</p>
    MaximumNumberOfTrailsExceeded(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when AWS Organizations is not configured to support all features. All features must be enabled in AWS Organization to support creating an organization trail. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    OrganizationNotInAllFeaturesMode(String),
    /// <p>This exception is thrown when the request is made from an AWS account that is not a member of an organization. To make this request, sign in using the credentials of an account that belongs to an organization.</p>
    OrganizationsNotInUse(String),
    /// <p>This exception is thrown when the specified S3 bucket does not exist.</p>
    S3BucketDoesNotExist(String),
    /// <p>This exception is thrown when the specified trail already exists.</p>
    TrailAlreadyExists(String),
    /// <p>This exception is no longer in use.</p>
    TrailNotProvided(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl CreateTrailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTrailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudTrailAccessNotEnabledException" => {
                    return RusotoError::Service(CreateTrailError::CloudTrailAccessNotEnabled(
                        err.msg,
                    ))
                }
                "CloudWatchLogsDeliveryUnavailableException" => {
                    return RusotoError::Service(
                        CreateTrailError::CloudWatchLogsDeliveryUnavailable(err.msg),
                    )
                }
                "InsufficientDependencyServiceAccessPermissionException" => {
                    return RusotoError::Service(
                        CreateTrailError::InsufficientDependencyServiceAccessPermission(err.msg),
                    )
                }
                "InsufficientEncryptionPolicyException" => {
                    return RusotoError::Service(CreateTrailError::InsufficientEncryptionPolicy(
                        err.msg,
                    ))
                }
                "InsufficientS3BucketPolicyException" => {
                    return RusotoError::Service(CreateTrailError::InsufficientS3BucketPolicy(
                        err.msg,
                    ))
                }
                "InsufficientSnsTopicPolicyException" => {
                    return RusotoError::Service(CreateTrailError::InsufficientSnsTopicPolicy(
                        err.msg,
                    ))
                }
                "InvalidCloudWatchLogsLogGroupArnException" => {
                    return RusotoError::Service(
                        CreateTrailError::InvalidCloudWatchLogsLogGroupArn(err.msg),
                    )
                }
                "InvalidCloudWatchLogsRoleArnException" => {
                    return RusotoError::Service(CreateTrailError::InvalidCloudWatchLogsRoleArn(
                        err.msg,
                    ))
                }
                "InvalidKmsKeyIdException" => {
                    return RusotoError::Service(CreateTrailError::InvalidKmsKeyId(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(CreateTrailError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidS3BucketNameException" => {
                    return RusotoError::Service(CreateTrailError::InvalidS3BucketName(err.msg))
                }
                "InvalidS3PrefixException" => {
                    return RusotoError::Service(CreateTrailError::InvalidS3Prefix(err.msg))
                }
                "InvalidSnsTopicNameException" => {
                    return RusotoError::Service(CreateTrailError::InvalidSnsTopicName(err.msg))
                }
                "InvalidTagParameterException" => {
                    return RusotoError::Service(CreateTrailError::InvalidTagParameter(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(CreateTrailError::InvalidTrailName(err.msg))
                }
                "KmsException" => return RusotoError::Service(CreateTrailError::Kms(err.msg)),
                "KmsKeyDisabledException" => {
                    return RusotoError::Service(CreateTrailError::KmsKeyDisabled(err.msg))
                }
                "KmsKeyNotFoundException" => {
                    return RusotoError::Service(CreateTrailError::KmsKeyNotFound(err.msg))
                }
                "MaximumNumberOfTrailsExceededException" => {
                    return RusotoError::Service(CreateTrailError::MaximumNumberOfTrailsExceeded(
                        err.msg,
                    ))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(CreateTrailError::NotOrganizationMasterAccount(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateTrailError::OperationNotPermitted(err.msg))
                }
                "OrganizationNotInAllFeaturesModeException" => {
                    return RusotoError::Service(
                        CreateTrailError::OrganizationNotInAllFeaturesMode(err.msg),
                    )
                }
                "OrganizationsNotInUseException" => {
                    return RusotoError::Service(CreateTrailError::OrganizationsNotInUse(err.msg))
                }
                "S3BucketDoesNotExistException" => {
                    return RusotoError::Service(CreateTrailError::S3BucketDoesNotExist(err.msg))
                }
                "TrailAlreadyExistsException" => {
                    return RusotoError::Service(CreateTrailError::TrailAlreadyExists(err.msg))
                }
                "TrailNotProvidedException" => {
                    return RusotoError::Service(CreateTrailError::TrailNotProvided(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateTrailError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTrailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTrailError::CloudTrailAccessNotEnabled(ref cause) => write!(f, "{}", cause),
            CreateTrailError::CloudWatchLogsDeliveryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTrailError::InsufficientDependencyServiceAccessPermission(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateTrailError::InsufficientEncryptionPolicy(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InsufficientS3BucketPolicy(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InsufficientSnsTopicPolicy(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidCloudWatchLogsLogGroupArn(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidCloudWatchLogsRoleArn(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidKmsKeyId(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidS3BucketName(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidS3Prefix(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidSnsTopicName(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidTagParameter(ref cause) => write!(f, "{}", cause),
            CreateTrailError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            CreateTrailError::Kms(ref cause) => write!(f, "{}", cause),
            CreateTrailError::KmsKeyDisabled(ref cause) => write!(f, "{}", cause),
            CreateTrailError::KmsKeyNotFound(ref cause) => write!(f, "{}", cause),
            CreateTrailError::MaximumNumberOfTrailsExceeded(ref cause) => write!(f, "{}", cause),
            CreateTrailError::NotOrganizationMasterAccount(ref cause) => write!(f, "{}", cause),
            CreateTrailError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateTrailError::OrganizationNotInAllFeaturesMode(ref cause) => write!(f, "{}", cause),
            CreateTrailError::OrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            CreateTrailError::S3BucketDoesNotExist(ref cause) => write!(f, "{}", cause),
            CreateTrailError::TrailAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateTrailError::TrailNotProvided(ref cause) => write!(f, "{}", cause),
            CreateTrailError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTrailError {}
/// Errors returned by DeleteTrail
#[derive(Debug, PartialEq)]
pub enum DeleteTrailError {
    /// <p>This exception is thrown when the IAM user or role that is used to create the organization trail is lacking one or more required permissions for creating an organization trail in a required service. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    InsufficientDependencyServiceAccessPermission(String),
    /// <p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
    InvalidHomeRegion(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DeleteTrailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTrailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientDependencyServiceAccessPermissionException" => {
                    return RusotoError::Service(
                        DeleteTrailError::InsufficientDependencyServiceAccessPermission(err.msg),
                    )
                }
                "InvalidHomeRegionException" => {
                    return RusotoError::Service(DeleteTrailError::InvalidHomeRegion(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(DeleteTrailError::InvalidTrailName(err.msg))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(DeleteTrailError::NotOrganizationMasterAccount(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteTrailError::OperationNotPermitted(err.msg))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(DeleteTrailError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteTrailError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTrailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTrailError::InsufficientDependencyServiceAccessPermission(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteTrailError::InvalidHomeRegion(ref cause) => write!(f, "{}", cause),
            DeleteTrailError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            DeleteTrailError::NotOrganizationMasterAccount(ref cause) => write!(f, "{}", cause),
            DeleteTrailError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteTrailError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTrailError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTrailError {}
/// Errors returned by DescribeTrails
#[derive(Debug, PartialEq)]
pub enum DescribeTrailsError {
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl DescribeTrailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTrailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidTrailNameException" => {
                    return RusotoError::Service(DescribeTrailsError::InvalidTrailName(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DescribeTrailsError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DescribeTrailsError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTrailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTrailsError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            DescribeTrailsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DescribeTrailsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTrailsError {}
/// Errors returned by GetEventSelectors
#[derive(Debug, PartialEq)]
pub enum GetEventSelectorsError {
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl GetEventSelectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEventSelectorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidTrailNameException" => {
                    return RusotoError::Service(GetEventSelectorsError::InvalidTrailName(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GetEventSelectorsError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(GetEventSelectorsError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetEventSelectorsError::UnsupportedOperation(
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
impl fmt::Display for GetEventSelectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEventSelectorsError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            GetEventSelectorsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GetEventSelectorsError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            GetEventSelectorsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEventSelectorsError {}
/// Errors returned by GetInsightSelectors
#[derive(Debug, PartialEq)]
pub enum GetInsightSelectorsError {
    /// <p>If you run <code>GetInsightSelectors</code> on a trail that does not have Insights events enabled, the operation throws the exception <code>InsightNotEnabledException</code>.</p>
    InsightNotEnabled(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl GetInsightSelectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInsightSelectorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsightNotEnabledException" => {
                    return RusotoError::Service(GetInsightSelectorsError::InsightNotEnabled(
                        err.msg,
                    ))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(GetInsightSelectorsError::InvalidTrailName(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GetInsightSelectorsError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(GetInsightSelectorsError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetInsightSelectorsError::UnsupportedOperation(
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
impl fmt::Display for GetInsightSelectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInsightSelectorsError::InsightNotEnabled(ref cause) => write!(f, "{}", cause),
            GetInsightSelectorsError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            GetInsightSelectorsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GetInsightSelectorsError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            GetInsightSelectorsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInsightSelectorsError {}
/// Errors returned by GetTrail
#[derive(Debug, PartialEq)]
pub enum GetTrailError {
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl GetTrailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTrailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidTrailNameException" => {
                    return RusotoError::Service(GetTrailError::InvalidTrailName(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GetTrailError::OperationNotPermitted(err.msg))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(GetTrailError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetTrailError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTrailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTrailError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            GetTrailError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GetTrailError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            GetTrailError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTrailError {}
/// Errors returned by GetTrailStatus
#[derive(Debug, PartialEq)]
pub enum GetTrailStatusError {
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl GetTrailStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTrailStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidTrailNameException" => {
                    return RusotoError::Service(GetTrailStatusError::InvalidTrailName(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GetTrailStatusError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(GetTrailStatusError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetTrailStatusError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTrailStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTrailStatusError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            GetTrailStatusError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GetTrailStatusError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            GetTrailStatusError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTrailStatusError {}
/// Errors returned by ListPublicKeys
#[derive(Debug, PartialEq)]
pub enum ListPublicKeysError {
    /// <p>Occurs if the timestamp values are invalid. Either the start time occurs after the end time or the time range is outside the range of possible values.</p>
    InvalidTimeRange(String),
    /// <p>Reserved for future use.</p>
    InvalidToken(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl ListPublicKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPublicKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidTimeRangeException" => {
                    return RusotoError::Service(ListPublicKeysError::InvalidTimeRange(err.msg))
                }
                "InvalidTokenException" => {
                    return RusotoError::Service(ListPublicKeysError::InvalidToken(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ListPublicKeysError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ListPublicKeysError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPublicKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPublicKeysError::InvalidTimeRange(ref cause) => write!(f, "{}", cause),
            ListPublicKeysError::InvalidToken(ref cause) => write!(f, "{}", cause),
            ListPublicKeysError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ListPublicKeysError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPublicKeysError {}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>This exception is thrown when an operation is called with an invalid trail ARN. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    CloudTrailARNInvalid(String),
    /// <p>Reserved for future use.</p>
    InvalidToken(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the specified resource is not found.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the specified resource type is not supported by CloudTrail.</p>
    ResourceTypeNotSupported(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudTrailARNInvalidException" => {
                    return RusotoError::Service(ListTagsError::CloudTrailARNInvalid(err.msg))
                }
                "InvalidTokenException" => {
                    return RusotoError::Service(ListTagsError::InvalidToken(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(ListTagsError::InvalidTrailName(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ListTagsError::OperationNotPermitted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsError::ResourceNotFound(err.msg))
                }
                "ResourceTypeNotSupportedException" => {
                    return RusotoError::Service(ListTagsError::ResourceTypeNotSupported(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ListTagsError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsError::CloudTrailARNInvalid(ref cause) => write!(f, "{}", cause),
            ListTagsError::InvalidToken(ref cause) => write!(f, "{}", cause),
            ListTagsError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            ListTagsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ListTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsError::ResourceTypeNotSupported(ref cause) => write!(f, "{}", cause),
            ListTagsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsError {}
/// Errors returned by ListTrails
#[derive(Debug, PartialEq)]
pub enum ListTrailsError {
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl ListTrailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTrailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ListTrailsError::OperationNotPermitted(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ListTrailsError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTrailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTrailsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ListTrailsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTrailsError {}
/// Errors returned by LookupEvents
#[derive(Debug, PartialEq)]
pub enum LookupEventsError {
    /// <p>Occurs if an event category that is not valid is specified as a value of <code>EventCategory</code>.</p>
    InvalidEventCategory(String),
    /// <p>Occurs when an invalid lookup attribute is specified.</p>
    InvalidLookupAttributes(String),
    /// <p>This exception is thrown if the limit specified is invalid.</p>
    InvalidMaxResults(String),
    /// <p>Invalid token or token that was previously used in a request with different parameters. This exception is thrown if the token is invalid.</p>
    InvalidNextToken(String),
    /// <p>Occurs if the timestamp values are invalid. Either the start time occurs after the end time or the time range is outside the range of possible values.</p>
    InvalidTimeRange(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl LookupEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LookupEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidEventCategoryException" => {
                    return RusotoError::Service(LookupEventsError::InvalidEventCategory(err.msg))
                }
                "InvalidLookupAttributesException" => {
                    return RusotoError::Service(LookupEventsError::InvalidLookupAttributes(
                        err.msg,
                    ))
                }
                "InvalidMaxResultsException" => {
                    return RusotoError::Service(LookupEventsError::InvalidMaxResults(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(LookupEventsError::InvalidNextToken(err.msg))
                }
                "InvalidTimeRangeException" => {
                    return RusotoError::Service(LookupEventsError::InvalidTimeRange(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(LookupEventsError::OperationNotPermitted(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(LookupEventsError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for LookupEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LookupEventsError::InvalidEventCategory(ref cause) => write!(f, "{}", cause),
            LookupEventsError::InvalidLookupAttributes(ref cause) => write!(f, "{}", cause),
            LookupEventsError::InvalidMaxResults(ref cause) => write!(f, "{}", cause),
            LookupEventsError::InvalidNextToken(ref cause) => write!(f, "{}", cause),
            LookupEventsError::InvalidTimeRange(ref cause) => write!(f, "{}", cause),
            LookupEventsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            LookupEventsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for LookupEventsError {}
/// Errors returned by PutEventSelectors
#[derive(Debug, PartialEq)]
pub enum PutEventSelectorsError {
    /// <p>This exception is thrown when the IAM user or role that is used to create the organization trail is lacking one or more required permissions for creating an organization trail in a required service. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    InsufficientDependencyServiceAccessPermission(String),
    /// <p><p>This exception is thrown when the <code>PutEventSelectors</code> operation is called with a number of event selectors or data resources that is not valid. The combination of event selectors and data resources is not valid. A trail can have up to 5 event selectors. A trail is limited to 250 data resources. These data resources can be distributed across event selectors, but the overall total cannot exceed 250.</p> <p>You can:</p> <ul> <li> <p>Specify a valid number of event selectors (1 to 5) for a trail.</p> </li> <li> <p>Specify a valid number of data resources (1 to 250) for an event selector. The limit of number of resources on an individual event selector is configurable up to 250. However, this upper limit is allowed only if the total number of data resources does not exceed 250 across all event selectors for a trail.</p> </li> <li> <p>Specify a valid value for a parameter. For example, specifying the <code>ReadWriteType</code> parameter with a value of <code>read-only</code> is invalid.</p> </li> </ul></p>
    InvalidEventSelectors(String),
    /// <p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
    InvalidHomeRegion(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl PutEventSelectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEventSelectorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientDependencyServiceAccessPermissionException" => {
                    return RusotoError::Service(
                        PutEventSelectorsError::InsufficientDependencyServiceAccessPermission(
                            err.msg,
                        ),
                    )
                }
                "InvalidEventSelectorsException" => {
                    return RusotoError::Service(PutEventSelectorsError::InvalidEventSelectors(
                        err.msg,
                    ))
                }
                "InvalidHomeRegionException" => {
                    return RusotoError::Service(PutEventSelectorsError::InvalidHomeRegion(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(PutEventSelectorsError::InvalidTrailName(err.msg))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(
                        PutEventSelectorsError::NotOrganizationMasterAccount(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(PutEventSelectorsError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(PutEventSelectorsError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(PutEventSelectorsError::UnsupportedOperation(
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
impl fmt::Display for PutEventSelectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEventSelectorsError::InsufficientDependencyServiceAccessPermission(ref cause) => {
                write!(f, "{}", cause)
            }
            PutEventSelectorsError::InvalidEventSelectors(ref cause) => write!(f, "{}", cause),
            PutEventSelectorsError::InvalidHomeRegion(ref cause) => write!(f, "{}", cause),
            PutEventSelectorsError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            PutEventSelectorsError::NotOrganizationMasterAccount(ref cause) => {
                write!(f, "{}", cause)
            }
            PutEventSelectorsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            PutEventSelectorsError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            PutEventSelectorsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutEventSelectorsError {}
/// Errors returned by PutInsightSelectors
#[derive(Debug, PartialEq)]
pub enum PutInsightSelectorsError {
    /// <p>This exception is thrown when the policy on the S3 bucket or KMS key is not sufficient.</p>
    InsufficientEncryptionPolicy(String),
    /// <p>This exception is thrown when the policy on the S3 bucket is not sufficient.</p>
    InsufficientS3BucketPolicy(String),
    /// <p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
    InvalidHomeRegion(String),
    /// <p>The formatting or syntax of the <code>InsightSelectors</code> JSON statement in your <code>PutInsightSelectors</code> or <code>GetInsightSelectors</code> request is not valid, or the specified insight type in the <code>InsightSelectors</code> statement is not a valid insight type.</p>
    InvalidInsightSelectors(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl PutInsightSelectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutInsightSelectorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientEncryptionPolicyException" => {
                    return RusotoError::Service(
                        PutInsightSelectorsError::InsufficientEncryptionPolicy(err.msg),
                    )
                }
                "InsufficientS3BucketPolicyException" => {
                    return RusotoError::Service(
                        PutInsightSelectorsError::InsufficientS3BucketPolicy(err.msg),
                    )
                }
                "InvalidHomeRegionException" => {
                    return RusotoError::Service(PutInsightSelectorsError::InvalidHomeRegion(
                        err.msg,
                    ))
                }
                "InvalidInsightSelectorsException" => {
                    return RusotoError::Service(PutInsightSelectorsError::InvalidInsightSelectors(
                        err.msg,
                    ))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(PutInsightSelectorsError::InvalidTrailName(
                        err.msg,
                    ))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(
                        PutInsightSelectorsError::NotOrganizationMasterAccount(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(PutInsightSelectorsError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(PutInsightSelectorsError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(PutInsightSelectorsError::UnsupportedOperation(
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
impl fmt::Display for PutInsightSelectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutInsightSelectorsError::InsufficientEncryptionPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            PutInsightSelectorsError::InsufficientS3BucketPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            PutInsightSelectorsError::InvalidHomeRegion(ref cause) => write!(f, "{}", cause),
            PutInsightSelectorsError::InvalidInsightSelectors(ref cause) => write!(f, "{}", cause),
            PutInsightSelectorsError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            PutInsightSelectorsError::NotOrganizationMasterAccount(ref cause) => {
                write!(f, "{}", cause)
            }
            PutInsightSelectorsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            PutInsightSelectorsError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            PutInsightSelectorsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutInsightSelectorsError {}
/// Errors returned by RemoveTags
#[derive(Debug, PartialEq)]
pub enum RemoveTagsError {
    /// <p>This exception is thrown when an operation is called with an invalid trail ARN. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code> </p>
    CloudTrailARNInvalid(String),
    /// <p>This exception is thrown when the specified tag key or values are not valid. It can also occur if there are duplicate tags or too many tags on the resource.</p>
    InvalidTagParameter(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the specified resource is not found.</p>
    ResourceNotFound(String),
    /// <p>This exception is thrown when the specified resource type is not supported by CloudTrail.</p>
    ResourceTypeNotSupported(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl RemoveTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudTrailARNInvalidException" => {
                    return RusotoError::Service(RemoveTagsError::CloudTrailARNInvalid(err.msg))
                }
                "InvalidTagParameterException" => {
                    return RusotoError::Service(RemoveTagsError::InvalidTagParameter(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(RemoveTagsError::InvalidTrailName(err.msg))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(RemoveTagsError::NotOrganizationMasterAccount(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(RemoveTagsError::OperationNotPermitted(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemoveTagsError::ResourceNotFound(err.msg))
                }
                "ResourceTypeNotSupportedException" => {
                    return RusotoError::Service(RemoveTagsError::ResourceTypeNotSupported(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(RemoveTagsError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsError::CloudTrailARNInvalid(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::InvalidTagParameter(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::NotOrganizationMasterAccount(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::ResourceTypeNotSupported(ref cause) => write!(f, "{}", cause),
            RemoveTagsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsError {}
/// Errors returned by StartLogging
#[derive(Debug, PartialEq)]
pub enum StartLoggingError {
    /// <p>This exception is thrown when the IAM user or role that is used to create the organization trail is lacking one or more required permissions for creating an organization trail in a required service. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    InsufficientDependencyServiceAccessPermission(String),
    /// <p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
    InvalidHomeRegion(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl StartLoggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartLoggingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientDependencyServiceAccessPermissionException" => {
                    return RusotoError::Service(
                        StartLoggingError::InsufficientDependencyServiceAccessPermission(err.msg),
                    )
                }
                "InvalidHomeRegionException" => {
                    return RusotoError::Service(StartLoggingError::InvalidHomeRegion(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(StartLoggingError::InvalidTrailName(err.msg))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(StartLoggingError::NotOrganizationMasterAccount(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StartLoggingError::OperationNotPermitted(err.msg))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(StartLoggingError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(StartLoggingError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartLoggingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartLoggingError::InsufficientDependencyServiceAccessPermission(ref cause) => {
                write!(f, "{}", cause)
            }
            StartLoggingError::InvalidHomeRegion(ref cause) => write!(f, "{}", cause),
            StartLoggingError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            StartLoggingError::NotOrganizationMasterAccount(ref cause) => write!(f, "{}", cause),
            StartLoggingError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StartLoggingError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            StartLoggingError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartLoggingError {}
/// Errors returned by StopLogging
#[derive(Debug, PartialEq)]
pub enum StopLoggingError {
    /// <p>This exception is thrown when the IAM user or role that is used to create the organization trail is lacking one or more required permissions for creating an organization trail in a required service. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    InsufficientDependencyServiceAccessPermission(String),
    /// <p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
    InvalidHomeRegion(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl StopLoggingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopLoggingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientDependencyServiceAccessPermissionException" => {
                    return RusotoError::Service(
                        StopLoggingError::InsufficientDependencyServiceAccessPermission(err.msg),
                    )
                }
                "InvalidHomeRegionException" => {
                    return RusotoError::Service(StopLoggingError::InvalidHomeRegion(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(StopLoggingError::InvalidTrailName(err.msg))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(StopLoggingError::NotOrganizationMasterAccount(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StopLoggingError::OperationNotPermitted(err.msg))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(StopLoggingError::TrailNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(StopLoggingError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopLoggingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopLoggingError::InsufficientDependencyServiceAccessPermission(ref cause) => {
                write!(f, "{}", cause)
            }
            StopLoggingError::InvalidHomeRegion(ref cause) => write!(f, "{}", cause),
            StopLoggingError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            StopLoggingError::NotOrganizationMasterAccount(ref cause) => write!(f, "{}", cause),
            StopLoggingError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StopLoggingError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            StopLoggingError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopLoggingError {}
/// Errors returned by UpdateTrail
#[derive(Debug, PartialEq)]
pub enum UpdateTrailError {
    /// <p>This exception is thrown when trusted access has not been enabled between AWS CloudTrail and AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Enabling Trusted Access with Other AWS Services</a> and <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>. </p>
    CloudTrailAccessNotEnabled(String),
    /// <p>Cannot set a CloudWatch Logs delivery for this region.</p>
    CloudWatchLogsDeliveryUnavailable(String),
    /// <p>This exception is thrown when the IAM user or role that is used to create the organization trail is lacking one or more required permissions for creating an organization trail in a required service. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    InsufficientDependencyServiceAccessPermission(String),
    /// <p>This exception is thrown when the policy on the S3 bucket or KMS key is not sufficient.</p>
    InsufficientEncryptionPolicy(String),
    /// <p>This exception is thrown when the policy on the S3 bucket is not sufficient.</p>
    InsufficientS3BucketPolicy(String),
    /// <p>This exception is thrown when the policy on the SNS topic is not sufficient.</p>
    InsufficientSnsTopicPolicy(String),
    /// <p>This exception is thrown when the provided CloudWatch log group is not valid.</p>
    InvalidCloudWatchLogsLogGroupArn(String),
    /// <p>This exception is thrown when the provided role is not valid.</p>
    InvalidCloudWatchLogsRoleArn(String),
    /// <p><p>This exception is thrown when the <code>PutEventSelectors</code> operation is called with a number of event selectors or data resources that is not valid. The combination of event selectors and data resources is not valid. A trail can have up to 5 event selectors. A trail is limited to 250 data resources. These data resources can be distributed across event selectors, but the overall total cannot exceed 250.</p> <p>You can:</p> <ul> <li> <p>Specify a valid number of event selectors (1 to 5) for a trail.</p> </li> <li> <p>Specify a valid number of data resources (1 to 250) for an event selector. The limit of number of resources on an individual event selector is configurable up to 250. However, this upper limit is allowed only if the total number of data resources does not exceed 250 across all event selectors for a trail.</p> </li> <li> <p>Specify a valid value for a parameter. For example, specifying the <code>ReadWriteType</code> parameter with a value of <code>read-only</code> is invalid.</p> </li> </ul></p>
    InvalidEventSelectors(String),
    /// <p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
    InvalidHomeRegion(String),
    /// <p>This exception is thrown when the KMS key ARN is invalid.</p>
    InvalidKmsKeyId(String),
    /// <p>This exception is thrown when the combination of parameters provided is not valid.</p>
    InvalidParameterCombination(String),
    /// <p>This exception is thrown when the provided S3 bucket name is not valid.</p>
    InvalidS3BucketName(String),
    /// <p>This exception is thrown when the provided S3 prefix is not valid.</p>
    InvalidS3Prefix(String),
    /// <p>This exception is thrown when the provided SNS topic name is not valid.</p>
    InvalidSnsTopicName(String),
    /// <p><p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (<em>), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-</em>namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul></p>
    InvalidTrailName(String),
    /// <p>This exception is thrown when there is an issue with the specified KMS key and the trail can’t be updated.</p>
    Kms(String),
    /// <p>This exception is no longer in use.</p>
    KmsKeyDisabled(String),
    /// <p>This exception is thrown when the KMS key does not exist, or when the S3 bucket and the KMS key are not in the same region.</p>
    KmsKeyNotFound(String),
    /// <p>This exception is thrown when the AWS account making the request to create or update an organization trail is not the master account for an organization in AWS Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    NotOrganizationMasterAccount(String),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This exception is thrown when AWS Organizations is not configured to support all features. All features must be enabled in AWS Organization to support creating an organization trail. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>.</p>
    OrganizationNotInAllFeaturesMode(String),
    /// <p>This exception is thrown when the request is made from an AWS account that is not a member of an organization. To make this request, sign in using the credentials of an account that belongs to an organization.</p>
    OrganizationsNotInUse(String),
    /// <p>This exception is thrown when the specified S3 bucket does not exist.</p>
    S3BucketDoesNotExist(String),
    /// <p>This exception is thrown when the trail with the given name is not found.</p>
    TrailNotFound(String),
    /// <p>This exception is no longer in use.</p>
    TrailNotProvided(String),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperation(String),
}

impl UpdateTrailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTrailError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudTrailAccessNotEnabledException" => {
                    return RusotoError::Service(UpdateTrailError::CloudTrailAccessNotEnabled(
                        err.msg,
                    ))
                }
                "CloudWatchLogsDeliveryUnavailableException" => {
                    return RusotoError::Service(
                        UpdateTrailError::CloudWatchLogsDeliveryUnavailable(err.msg),
                    )
                }
                "InsufficientDependencyServiceAccessPermissionException" => {
                    return RusotoError::Service(
                        UpdateTrailError::InsufficientDependencyServiceAccessPermission(err.msg),
                    )
                }
                "InsufficientEncryptionPolicyException" => {
                    return RusotoError::Service(UpdateTrailError::InsufficientEncryptionPolicy(
                        err.msg,
                    ))
                }
                "InsufficientS3BucketPolicyException" => {
                    return RusotoError::Service(UpdateTrailError::InsufficientS3BucketPolicy(
                        err.msg,
                    ))
                }
                "InsufficientSnsTopicPolicyException" => {
                    return RusotoError::Service(UpdateTrailError::InsufficientSnsTopicPolicy(
                        err.msg,
                    ))
                }
                "InvalidCloudWatchLogsLogGroupArnException" => {
                    return RusotoError::Service(
                        UpdateTrailError::InvalidCloudWatchLogsLogGroupArn(err.msg),
                    )
                }
                "InvalidCloudWatchLogsRoleArnException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidCloudWatchLogsRoleArn(
                        err.msg,
                    ))
                }
                "InvalidEventSelectorsException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidEventSelectors(err.msg))
                }
                "InvalidHomeRegionException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidHomeRegion(err.msg))
                }
                "InvalidKmsKeyIdException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidKmsKeyId(err.msg))
                }
                "InvalidParameterCombinationException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidParameterCombination(
                        err.msg,
                    ))
                }
                "InvalidS3BucketNameException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidS3BucketName(err.msg))
                }
                "InvalidS3PrefixException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidS3Prefix(err.msg))
                }
                "InvalidSnsTopicNameException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidSnsTopicName(err.msg))
                }
                "InvalidTrailNameException" => {
                    return RusotoError::Service(UpdateTrailError::InvalidTrailName(err.msg))
                }
                "KmsException" => return RusotoError::Service(UpdateTrailError::Kms(err.msg)),
                "KmsKeyDisabledException" => {
                    return RusotoError::Service(UpdateTrailError::KmsKeyDisabled(err.msg))
                }
                "KmsKeyNotFoundException" => {
                    return RusotoError::Service(UpdateTrailError::KmsKeyNotFound(err.msg))
                }
                "NotOrganizationMasterAccountException" => {
                    return RusotoError::Service(UpdateTrailError::NotOrganizationMasterAccount(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateTrailError::OperationNotPermitted(err.msg))
                }
                "OrganizationNotInAllFeaturesModeException" => {
                    return RusotoError::Service(
                        UpdateTrailError::OrganizationNotInAllFeaturesMode(err.msg),
                    )
                }
                "OrganizationsNotInUseException" => {
                    return RusotoError::Service(UpdateTrailError::OrganizationsNotInUse(err.msg))
                }
                "S3BucketDoesNotExistException" => {
                    return RusotoError::Service(UpdateTrailError::S3BucketDoesNotExist(err.msg))
                }
                "TrailNotFoundException" => {
                    return RusotoError::Service(UpdateTrailError::TrailNotFound(err.msg))
                }
                "TrailNotProvidedException" => {
                    return RusotoError::Service(UpdateTrailError::TrailNotProvided(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(UpdateTrailError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateTrailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTrailError::CloudTrailAccessNotEnabled(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::CloudWatchLogsDeliveryUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateTrailError::InsufficientDependencyServiceAccessPermission(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateTrailError::InsufficientEncryptionPolicy(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InsufficientS3BucketPolicy(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InsufficientSnsTopicPolicy(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidCloudWatchLogsLogGroupArn(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidCloudWatchLogsRoleArn(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidEventSelectors(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidHomeRegion(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidKmsKeyId(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidS3BucketName(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidS3Prefix(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidSnsTopicName(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::InvalidTrailName(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::Kms(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::KmsKeyDisabled(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::KmsKeyNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::NotOrganizationMasterAccount(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::OrganizationNotInAllFeaturesMode(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::OrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::S3BucketDoesNotExist(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::TrailNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::TrailNotProvided(ref cause) => write!(f, "{}", cause),
            UpdateTrailError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTrailError {}
/// Trait representing the capabilities of the CloudTrail API. CloudTrail clients implement this trait.
#[async_trait]
pub trait CloudTrail {
    /// <p>Adds one or more tags to a trail, up to a limit of 50. Overwrites an existing tag's value when a new value is specified for an existing tag key. Tag key names must be unique for a trail; you cannot have two keys with the same name but different values. If you specify a key without a value, the tag will be created with the specified key and a value of null. You can tag a trail that applies to all AWS Regions only from the Region in which the trail was created (also known as its home region).</p>
    async fn add_tags(
        &self,
        input: AddTagsRequest,
    ) -> Result<AddTagsResponse, RusotoError<AddTagsError>>;

    /// <p>Creates a trail that specifies the settings for delivery of log data to an Amazon S3 bucket. </p>
    async fn create_trail(
        &self,
        input: CreateTrailRequest,
    ) -> Result<CreateTrailResponse, RusotoError<CreateTrailError>>;

    /// <p>Deletes a trail. This operation must be called from the region in which the trail was created. <code>DeleteTrail</code> cannot be called on the shadow trails (replicated trails in other regions) of a trail that is enabled in all regions.</p>
    async fn delete_trail(
        &self,
        input: DeleteTrailRequest,
    ) -> Result<DeleteTrailResponse, RusotoError<DeleteTrailError>>;

    /// <p>Retrieves settings for one or more trails associated with the current region for your account.</p>
    async fn describe_trails(
        &self,
        input: DescribeTrailsRequest,
    ) -> Result<DescribeTrailsResponse, RusotoError<DescribeTrailsError>>;

    /// <p>Describes the settings for the event selectors that you configured for your trail. The information returned for your event selectors includes the following:</p> <ul> <li> <p>If your event selector includes read-only events, write-only events, or all events. This applies to both management events and data events.</p> </li> <li> <p>If your event selector includes management events.</p> </li> <li> <p>If your event selector includes data events, the Amazon S3 objects or AWS Lambda functions that you are logging for data events.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-and-data-events-with-cloudtrail.html">Logging Data and Management Events for Trails </a> in the <i>AWS CloudTrail User Guide</i>.</p>
    async fn get_event_selectors(
        &self,
        input: GetEventSelectorsRequest,
    ) -> Result<GetEventSelectorsResponse, RusotoError<GetEventSelectorsError>>;

    /// <p>Describes the settings for the Insights event selectors that you configured for your trail. <code>GetInsightSelectors</code> shows if CloudTrail Insights event logging is enabled on the trail, and if it is, which insight types are enabled. If you run <code>GetInsightSelectors</code> on a trail that does not have Insights events enabled, the operation throws the exception <code>InsightNotEnabledException</code> </p> <p>For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-insights-events-with-cloudtrail.html">Logging CloudTrail Insights Events for Trails </a> in the <i>AWS CloudTrail User Guide</i>.</p>
    async fn get_insight_selectors(
        &self,
        input: GetInsightSelectorsRequest,
    ) -> Result<GetInsightSelectorsResponse, RusotoError<GetInsightSelectorsError>>;

    /// <p>Returns settings information for a specified trail.</p>
    async fn get_trail(
        &self,
        input: GetTrailRequest,
    ) -> Result<GetTrailResponse, RusotoError<GetTrailError>>;

    /// <p>Returns a JSON-formatted list of information about the specified trail. Fields include information on delivery errors, Amazon SNS and Amazon S3 errors, and start and stop logging times for each trail. This operation returns trail status from a single region. To return trail status from all regions, you must call the operation on each region.</p>
    async fn get_trail_status(
        &self,
        input: GetTrailStatusRequest,
    ) -> Result<GetTrailStatusResponse, RusotoError<GetTrailStatusError>>;

    /// <p><p>Returns all public keys whose private keys were used to sign the digest files within the specified time range. The public key is needed to validate digest files that were signed with its corresponding private key.</p> <note> <p>CloudTrail uses different private/public key pairs per region. Each digest file is signed with a private key unique to its region. Therefore, when you validate a digest file from a particular region, you must look in the same region for its corresponding public key.</p> </note></p>
    async fn list_public_keys(
        &self,
        input: ListPublicKeysRequest,
    ) -> Result<ListPublicKeysResponse, RusotoError<ListPublicKeysError>>;

    /// <p>Lists the tags for the trail in the current region.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>>;

    /// <p>Lists trails that are in the current account.</p>
    async fn list_trails(
        &self,
        input: ListTrailsRequest,
    ) -> Result<ListTrailsResponse, RusotoError<ListTrailsError>>;

    /// <p><p>Looks up <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-concepts.html#cloudtrail-concepts-management-events">management events</a> or <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-concepts.html#cloudtrail-concepts-insights-events">CloudTrail Insights events</a> that are captured by CloudTrail. You can look up events that occurred in a region within the last 90 days. Lookup supports the following attributes for management events:</p> <ul> <li> <p>AWS access key</p> </li> <li> <p>Event ID</p> </li> <li> <p>Event name</p> </li> <li> <p>Event source</p> </li> <li> <p>Read only</p> </li> <li> <p>Resource name</p> </li> <li> <p>Resource type</p> </li> <li> <p>User name</p> </li> </ul> <p>Lookup supports the following attributes for Insights events:</p> <ul> <li> <p>Event ID</p> </li> <li> <p>Event name</p> </li> <li> <p>Event source</p> </li> </ul> <p>All attributes are optional. The default number of results returned is 50, with a maximum of 50 possible. The response includes a token that you can use to get the next page of results.</p> <important> <p>The rate of lookup requests is limited to two per second per account. If this limit is exceeded, a throttling error occurs.</p> </important></p>
    async fn lookup_events(
        &self,
        input: LookupEventsRequest,
    ) -> Result<LookupEventsResponse, RusotoError<LookupEventsError>>;

    /// <p>Configures an event selector for your trail. Use event selectors to further specify the management and data event settings for your trail. By default, trails created without specific event selectors will be configured to log all read and write management events, and no data events. </p> <p>When an event occurs in your account, CloudTrail evaluates the event selectors in all trails. For each trail, if the event matches any event selector, the trail processes and logs the event. If the event doesn't match any event selector, the trail doesn't log the event. </p> <p>Example</p> <ol> <li> <p>You create an event selector for a trail and specify that you want write-only events.</p> </li> <li> <p>The EC2 <code>GetConsoleOutput</code> and <code>RunInstances</code> API operations occur in your account.</p> </li> <li> <p>CloudTrail evaluates whether the events match your event selectors.</p> </li> <li> <p>The <code>RunInstances</code> is a write-only event and it matches your event selector. The trail logs the event.</p> </li> <li> <p>The <code>GetConsoleOutput</code> is a read-only event but it doesn't match your event selector. The trail doesn't log the event. </p> </li> </ol> <p>The <code>PutEventSelectors</code> operation must be called from the region in which the trail was created; otherwise, an <code>InvalidHomeRegionException</code> is thrown.</p> <p>You can configure up to five event selectors for each trail. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-and-data-events-with-cloudtrail.html">Logging Data and Management Events for Trails </a> and <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/WhatIsCloudTrail-Limits.html">Limits in AWS CloudTrail</a> in the <i>AWS CloudTrail User Guide</i>.</p>
    async fn put_event_selectors(
        &self,
        input: PutEventSelectorsRequest,
    ) -> Result<PutEventSelectorsResponse, RusotoError<PutEventSelectorsError>>;

    /// <p>Lets you enable Insights event logging by specifying the Insights selectors that you want to enable on an existing trail. You also use <code>PutInsightSelectors</code> to turn off Insights event logging, by passing an empty list of insight types. In this release, only <code>ApiCallRateInsight</code> is supported as an Insights selector.</p>
    async fn put_insight_selectors(
        &self,
        input: PutInsightSelectorsRequest,
    ) -> Result<PutInsightSelectorsResponse, RusotoError<PutInsightSelectorsError>>;

    /// <p>Removes the specified tags from a trail.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsRequest,
    ) -> Result<RemoveTagsResponse, RusotoError<RemoveTagsError>>;

    /// <p>Starts the recording of AWS API calls and log file delivery for a trail. For a trail that is enabled in all regions, this operation must be called from the region in which the trail was created. This operation cannot be called on the shadow trails (replicated trails in other regions) of a trail that is enabled in all regions.</p>
    async fn start_logging(
        &self,
        input: StartLoggingRequest,
    ) -> Result<StartLoggingResponse, RusotoError<StartLoggingError>>;

    /// <p>Suspends the recording of AWS API calls and log file delivery for the specified trail. Under most circumstances, there is no need to use this action. You can update a trail without stopping it first. This action is the only way to stop recording. For a trail enabled in all regions, this operation must be called from the region in which the trail was created, or an <code>InvalidHomeRegionException</code> will occur. This operation cannot be called on the shadow trails (replicated trails in other regions) of a trail enabled in all regions.</p>
    async fn stop_logging(
        &self,
        input: StopLoggingRequest,
    ) -> Result<StopLoggingResponse, RusotoError<StopLoggingError>>;

    /// <p>Updates the settings that specify delivery of log files. Changes to a trail do not require stopping the CloudTrail service. Use this action to designate an existing bucket for log delivery. If the existing bucket has previously been a target for CloudTrail log files, an IAM policy exists for the bucket. <code>UpdateTrail</code> must be called from the region in which the trail was created; otherwise, an <code>InvalidHomeRegionException</code> is thrown.</p>
    async fn update_trail(
        &self,
        input: UpdateTrailRequest,
    ) -> Result<UpdateTrailResponse, RusotoError<UpdateTrailError>>;
}
/// A client for the CloudTrail API.
#[derive(Clone)]
pub struct CloudTrailClient {
    client: Client,
    region: region::Region,
}

impl CloudTrailClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudTrailClient {
        CloudTrailClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudTrailClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CloudTrailClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CloudTrailClient {
        CloudTrailClient { client, region }
    }
}

#[async_trait]
impl CloudTrail for CloudTrailClient {
    /// <p>Adds one or more tags to a trail, up to a limit of 50. Overwrites an existing tag's value when a new value is specified for an existing tag key. Tag key names must be unique for a trail; you cannot have two keys with the same name but different values. If you specify a key without a value, the tag will be created with the specified key and a value of null. You can tag a trail that applies to all AWS Regions only from the Region in which the trail was created (also known as its home region).</p>
    async fn add_tags(
        &self,
        input: AddTagsRequest,
    ) -> Result<AddTagsResponse, RusotoError<AddTagsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.AddTags",
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
            proto::json::ResponsePayload::new(&response).deserialize::<AddTagsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AddTagsError::from_response(response))
        }
    }

    /// <p>Creates a trail that specifies the settings for delivery of log data to an Amazon S3 bucket. </p>
    async fn create_trail(
        &self,
        input: CreateTrailRequest,
    ) -> Result<CreateTrailResponse, RusotoError<CreateTrailError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.CreateTrail",
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
            proto::json::ResponsePayload::new(&response).deserialize::<CreateTrailResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTrailError::from_response(response))
        }
    }

    /// <p>Deletes a trail. This operation must be called from the region in which the trail was created. <code>DeleteTrail</code> cannot be called on the shadow trails (replicated trails in other regions) of a trail that is enabled in all regions.</p>
    async fn delete_trail(
        &self,
        input: DeleteTrailRequest,
    ) -> Result<DeleteTrailResponse, RusotoError<DeleteTrailError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.DeleteTrail",
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
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteTrailResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTrailError::from_response(response))
        }
    }

    /// <p>Retrieves settings for one or more trails associated with the current region for your account.</p>
    async fn describe_trails(
        &self,
        input: DescribeTrailsRequest,
    ) -> Result<DescribeTrailsResponse, RusotoError<DescribeTrailsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.DescribeTrails",
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
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeTrailsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTrailsError::from_response(response))
        }
    }

    /// <p>Describes the settings for the event selectors that you configured for your trail. The information returned for your event selectors includes the following:</p> <ul> <li> <p>If your event selector includes read-only events, write-only events, or all events. This applies to both management events and data events.</p> </li> <li> <p>If your event selector includes management events.</p> </li> <li> <p>If your event selector includes data events, the Amazon S3 objects or AWS Lambda functions that you are logging for data events.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-and-data-events-with-cloudtrail.html">Logging Data and Management Events for Trails </a> in the <i>AWS CloudTrail User Guide</i>.</p>
    async fn get_event_selectors(
        &self,
        input: GetEventSelectorsRequest,
    ) -> Result<GetEventSelectorsResponse, RusotoError<GetEventSelectorsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.GetEventSelectors",
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
                .deserialize::<GetEventSelectorsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetEventSelectorsError::from_response(response))
        }
    }

    /// <p>Describes the settings for the Insights event selectors that you configured for your trail. <code>GetInsightSelectors</code> shows if CloudTrail Insights event logging is enabled on the trail, and if it is, which insight types are enabled. If you run <code>GetInsightSelectors</code> on a trail that does not have Insights events enabled, the operation throws the exception <code>InsightNotEnabledException</code> </p> <p>For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-insights-events-with-cloudtrail.html">Logging CloudTrail Insights Events for Trails </a> in the <i>AWS CloudTrail User Guide</i>.</p>
    async fn get_insight_selectors(
        &self,
        input: GetInsightSelectorsRequest,
    ) -> Result<GetInsightSelectorsResponse, RusotoError<GetInsightSelectorsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.GetInsightSelectors",
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
                .deserialize::<GetInsightSelectorsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetInsightSelectorsError::from_response(response))
        }
    }

    /// <p>Returns settings information for a specified trail.</p>
    async fn get_trail(
        &self,
        input: GetTrailRequest,
    ) -> Result<GetTrailResponse, RusotoError<GetTrailError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.GetTrail",
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
            proto::json::ResponsePayload::new(&response).deserialize::<GetTrailResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetTrailError::from_response(response))
        }
    }

    /// <p>Returns a JSON-formatted list of information about the specified trail. Fields include information on delivery errors, Amazon SNS and Amazon S3 errors, and start and stop logging times for each trail. This operation returns trail status from a single region. To return trail status from all regions, you must call the operation on each region.</p>
    async fn get_trail_status(
        &self,
        input: GetTrailStatusRequest,
    ) -> Result<GetTrailStatusResponse, RusotoError<GetTrailStatusError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.GetTrailStatus",
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
            proto::json::ResponsePayload::new(&response).deserialize::<GetTrailStatusResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetTrailStatusError::from_response(response))
        }
    }

    /// <p><p>Returns all public keys whose private keys were used to sign the digest files within the specified time range. The public key is needed to validate digest files that were signed with its corresponding private key.</p> <note> <p>CloudTrail uses different private/public key pairs per region. Each digest file is signed with a private key unique to its region. Therefore, when you validate a digest file from a particular region, you must look in the same region for its corresponding public key.</p> </note></p>
    async fn list_public_keys(
        &self,
        input: ListPublicKeysRequest,
    ) -> Result<ListPublicKeysResponse, RusotoError<ListPublicKeysError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.ListPublicKeys",
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
            proto::json::ResponsePayload::new(&response).deserialize::<ListPublicKeysResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListPublicKeysError::from_response(response))
        }
    }

    /// <p>Lists the tags for the trail in the current region.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.ListTags",
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
            proto::json::ResponsePayload::new(&response).deserialize::<ListTagsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsError::from_response(response))
        }
    }

    /// <p>Lists trails that are in the current account.</p>
    async fn list_trails(
        &self,
        input: ListTrailsRequest,
    ) -> Result<ListTrailsResponse, RusotoError<ListTrailsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.ListTrails",
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
            proto::json::ResponsePayload::new(&response).deserialize::<ListTrailsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTrailsError::from_response(response))
        }
    }

    /// <p><p>Looks up <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-concepts.html#cloudtrail-concepts-management-events">management events</a> or <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-concepts.html#cloudtrail-concepts-insights-events">CloudTrail Insights events</a> that are captured by CloudTrail. You can look up events that occurred in a region within the last 90 days. Lookup supports the following attributes for management events:</p> <ul> <li> <p>AWS access key</p> </li> <li> <p>Event ID</p> </li> <li> <p>Event name</p> </li> <li> <p>Event source</p> </li> <li> <p>Read only</p> </li> <li> <p>Resource name</p> </li> <li> <p>Resource type</p> </li> <li> <p>User name</p> </li> </ul> <p>Lookup supports the following attributes for Insights events:</p> <ul> <li> <p>Event ID</p> </li> <li> <p>Event name</p> </li> <li> <p>Event source</p> </li> </ul> <p>All attributes are optional. The default number of results returned is 50, with a maximum of 50 possible. The response includes a token that you can use to get the next page of results.</p> <important> <p>The rate of lookup requests is limited to two per second per account. If this limit is exceeded, a throttling error occurs.</p> </important></p>
    async fn lookup_events(
        &self,
        input: LookupEventsRequest,
    ) -> Result<LookupEventsResponse, RusotoError<LookupEventsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.LookupEvents",
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
            proto::json::ResponsePayload::new(&response).deserialize::<LookupEventsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(LookupEventsError::from_response(response))
        }
    }

    /// <p>Configures an event selector for your trail. Use event selectors to further specify the management and data event settings for your trail. By default, trails created without specific event selectors will be configured to log all read and write management events, and no data events. </p> <p>When an event occurs in your account, CloudTrail evaluates the event selectors in all trails. For each trail, if the event matches any event selector, the trail processes and logs the event. If the event doesn't match any event selector, the trail doesn't log the event. </p> <p>Example</p> <ol> <li> <p>You create an event selector for a trail and specify that you want write-only events.</p> </li> <li> <p>The EC2 <code>GetConsoleOutput</code> and <code>RunInstances</code> API operations occur in your account.</p> </li> <li> <p>CloudTrail evaluates whether the events match your event selectors.</p> </li> <li> <p>The <code>RunInstances</code> is a write-only event and it matches your event selector. The trail logs the event.</p> </li> <li> <p>The <code>GetConsoleOutput</code> is a read-only event but it doesn't match your event selector. The trail doesn't log the event. </p> </li> </ol> <p>The <code>PutEventSelectors</code> operation must be called from the region in which the trail was created; otherwise, an <code>InvalidHomeRegionException</code> is thrown.</p> <p>You can configure up to five event selectors for each trail. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-and-data-events-with-cloudtrail.html">Logging Data and Management Events for Trails </a> and <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/WhatIsCloudTrail-Limits.html">Limits in AWS CloudTrail</a> in the <i>AWS CloudTrail User Guide</i>.</p>
    async fn put_event_selectors(
        &self,
        input: PutEventSelectorsRequest,
    ) -> Result<PutEventSelectorsResponse, RusotoError<PutEventSelectorsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.PutEventSelectors",
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
                .deserialize::<PutEventSelectorsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutEventSelectorsError::from_response(response))
        }
    }

    /// <p>Lets you enable Insights event logging by specifying the Insights selectors that you want to enable on an existing trail. You also use <code>PutInsightSelectors</code> to turn off Insights event logging, by passing an empty list of insight types. In this release, only <code>ApiCallRateInsight</code> is supported as an Insights selector.</p>
    async fn put_insight_selectors(
        &self,
        input: PutInsightSelectorsRequest,
    ) -> Result<PutInsightSelectorsResponse, RusotoError<PutInsightSelectorsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.PutInsightSelectors",
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
                .deserialize::<PutInsightSelectorsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(PutInsightSelectorsError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from a trail.</p>
    async fn remove_tags(
        &self,
        input: RemoveTagsRequest,
    ) -> Result<RemoveTagsResponse, RusotoError<RemoveTagsError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.RemoveTags",
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
            proto::json::ResponsePayload::new(&response).deserialize::<RemoveTagsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveTagsError::from_response(response))
        }
    }

    /// <p>Starts the recording of AWS API calls and log file delivery for a trail. For a trail that is enabled in all regions, this operation must be called from the region in which the trail was created. This operation cannot be called on the shadow trails (replicated trails in other regions) of a trail that is enabled in all regions.</p>
    async fn start_logging(
        &self,
        input: StartLoggingRequest,
    ) -> Result<StartLoggingResponse, RusotoError<StartLoggingError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.StartLogging",
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
            proto::json::ResponsePayload::new(&response).deserialize::<StartLoggingResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartLoggingError::from_response(response))
        }
    }

    /// <p>Suspends the recording of AWS API calls and log file delivery for the specified trail. Under most circumstances, there is no need to use this action. You can update a trail without stopping it first. This action is the only way to stop recording. For a trail enabled in all regions, this operation must be called from the region in which the trail was created, or an <code>InvalidHomeRegionException</code> will occur. This operation cannot be called on the shadow trails (replicated trails in other regions) of a trail enabled in all regions.</p>
    async fn stop_logging(
        &self,
        input: StopLoggingRequest,
    ) -> Result<StopLoggingResponse, RusotoError<StopLoggingError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.StopLogging",
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
            proto::json::ResponsePayload::new(&response).deserialize::<StopLoggingResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopLoggingError::from_response(response))
        }
    }

    /// <p>Updates the settings that specify delivery of log files. Changes to a trail do not require stopping the CloudTrail service. Use this action to designate an existing bucket for log delivery. If the existing bucket has previously been a target for CloudTrail log files, an IAM policy exists for the bucket. <code>UpdateTrail</code> must be called from the region in which the trail was created; otherwise, an <code>InvalidHomeRegionException</code> is thrown.</p>
    async fn update_trail(
        &self,
        input: UpdateTrailRequest,
    ) -> Result<UpdateTrailResponse, RusotoError<UpdateTrailError>> {
        let mut request = SignedRequest::new("POST", "cloudtrail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.UpdateTrail",
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
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateTrailResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateTrailError::from_response(response))
        }
    }
}
