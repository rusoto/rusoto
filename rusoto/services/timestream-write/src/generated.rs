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

impl TimestreamWriteClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "timestream", &self.region, request_uri);
        request.set_endpoint_prefix("ingest.timestream".to_string());

        request.set_content_type("application/x-amz-json-1.0".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDatabaseRequest {
    /// <p>The name of the Timestream database.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The KMS key for the database. If the KMS key is not specified, the database will be encrypted with a Timestream managed KMS key located in your account. Refer to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed KMS keys</a> for more info.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> A list of key-value pairs to label the table. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDatabaseResponse {
    /// <p>The newly created Timestream database.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTableRequest {
    /// <p>The name of the Timestream database.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The duration for which your time series data must be stored in the memory store and the magnetic store.</p>
    #[serde(rename = "RetentionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_properties: Option<RetentionProperties>,
    /// <p>The name of the Timestream table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p> A list of key-value pairs to label the table. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTableResponse {
    /// <p>The newly created Timestream table.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

/// <p>A top level container for a table. Databases and tables are the fundamental management concepts in Amazon Timestream. All tables in a database are encrypted with the same KMS key.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Database {
    /// <p>The Amazon Resource Name that uniquely identifies this database.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time when the database was created, calculated from the Unix epoch time.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the Timestream database.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The identifier of the KMS key used to encrypt the data stored in the database.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> The last time that this database was updated. </p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The total number of tables found within a Timestream database. </p>
    #[serde(rename = "TableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_count: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDatabaseRequest {
    /// <p>The name of the Timestream database to be deleted.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTableRequest {
    /// <p>The name of the database where the Timestream database is to be deleted.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the Timestream table to be deleted.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDatabaseRequest {
    /// <p>The name of the Timestream database.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDatabaseResponse {
    /// <p>The name of the Timestream table.</p>
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointsRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointsResponse {
    /// <p>An <code>Endpoints</code> object is returned when a <code>DescribeEndpoints</code> request is made.</p>
    #[serde(rename = "Endpoints")]
    pub endpoints: Vec<Endpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTableRequest {
    /// <p>The name of the Timestream database.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The name of the Timestream table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTableResponse {
    /// <p>The Timestream table.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

/// <p>Dimension represents the meta data attributes of the time series. For example, the name and availability zone of an EC2 instance or the name of the manufacturer of a wind turbine are dimensions. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Dimension {
    /// <p>The data type of the dimension for the time series data point.</p>
    #[serde(rename = "DimensionValueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value_type: Option<String>,
    /// <p> Dimension represents the meta data attributes of the time series. For example, the name and availability zone of an EC2 instance or the name of the manufacturer of a wind turbine are dimensions. </p> <p>For constraints on Dimension names, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html#limits.naming">Naming Constraints</a>.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value of the dimension.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>Represents an available endpoint against which to make API calls agaisnt, as well as the TTL for that endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Endpoint {
    /// <p>An endpoint address.</p>
    #[serde(rename = "Address")]
    pub address: String,
    /// <p>The TTL for the endpoint, in minutes.</p>
    #[serde(rename = "CachePeriodInMinutes")]
    pub cache_period_in_minutes: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDatabasesRequest {
    /// <p>The total number of items to return in the output. If the total number of items available is more than the value specified, a NextToken is provided in the output. To resume pagination, provide the NextToken value as argument of a subsequent API invocation.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token. To resume pagination, provide the NextToken value as argument of a subsequent API invocation.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDatabasesResponse {
    /// <p>A list of database names.</p>
    #[serde(rename = "Databases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<Database>>,
    /// <p>The pagination token. This parameter is returned when the response is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTablesRequest {
    /// <p>The name of the Timestream database.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The total number of items to return in the output. If the total number of items available is more than the value specified, a NextToken is provided in the output. To resume pagination, provide the NextToken value as argument of a subsequent API invocation.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token. To resume pagination, provide the NextToken value as argument of a subsequent API invocation.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTablesResponse {
    /// <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tables.</p>
    #[serde(rename = "Tables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<Table>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p> The Timestream resource with tags to be listed. This value is an Amazon Resource Name (ARN). </p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p> The tags currently associated with the Timestream resource. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Record represents a time series data point being written into Timestream. Each record contains an array of dimensions. Dimensions represent the meta data attributes of a time series data point such as the instance name or availability zone of an EC2 instance. A record also contains the measure name which is the name of the measure being collected for example the CPU utilization of an EC2 instance. A record also contains the measure value and the value type which is the data type of the measure value. In addition, the record contains the timestamp when the measure was collected that the timestamp unit which represents the granularity of the timestamp. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Record {
    /// <p>Contains the list of dimensions for time series data points.</p>
    #[serde(rename = "Dimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>Measure represents the data attribute of the time series. For example, the CPU utilization of an EC2 instance or the RPM of a wind turbine are measures. </p>
    #[serde(rename = "MeasureName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_name: Option<String>,
    /// <p> Contains the measure value for the time series data point. </p>
    #[serde(rename = "MeasureValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_value: Option<String>,
    /// <p> Contains the data type of the measure value for the time series data point. </p>
    #[serde(rename = "MeasureValueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_value_type: Option<String>,
    /// <p> Contains the time at which the measure value for the data point was collected. The time value plus the unit provides the time elapsed since the epoch. For example, if the time value is <code>12345</code> and the unit is <code>ms</code>, then <code>12345 ms</code> have elapsed since the epoch. </p>
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// <p> The granularity of the timestamp unit. It indicates if the time value is in seconds, milliseconds, nanoseconds or other supported values. </p>
    #[serde(rename = "TimeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
    /// <p>64-bit attribute used for record updates. Write requests for duplicate data with a higher version number will update the existing measure value and version. In cases where the measure value is the same, <code>Version</code> will still be updated . Default value is to 1.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p> Records that were not successfully inserted into Timestream due to data validation issues that must be resolved prior to reinserting time series data into the system. </p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RejectedRecord {
    /// <p>The existing version of the record. This value is populated in scenarios where an identical record exists with a higher version than the version in the write request.</p>
    pub existing_version: Option<i64>,
    /// <p> The reason why a record was not successfully inserted into Timestream. Possible causes of failure include: </p> <ul> <li> <p> Records with duplicate data where there are multiple records with the same dimensions, timestamps, and measure names but different measure values. </p> </li> <li> <p> Records with timestamps that lie outside the retention duration of the memory store </p> <note> <p>When the retention window is updated, you will receive a <code>RejectedRecords</code> exception if you immediately try to ingest data within the new window. To avoid a <code>RejectedRecords</code> exception, wait until the duration of the new window to ingest new data. For further information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/best-practices.html#configuration"> Best Practices for Configuring Timestream</a> and <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/storage.html">the explanation of how storage works in Timestream</a>.</p> </note> </li> <li> <p> Records with dimensions or measures that exceed the Timestream defined limits. </p> </li> </ul> <p> For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    pub reason: Option<String>,
    /// <p> The index of the record in the input request for WriteRecords. Indexes begin with 0. </p>
    pub record_index: Option<i64>,
}

/// <p>Retention properties contain the duration for which your time series data must be stored in the magnetic store and the memory store. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetentionProperties {
    /// <p>The duration for which data must be stored in the magnetic store. </p>
    #[serde(rename = "MagneticStoreRetentionPeriodInDays")]
    pub magnetic_store_retention_period_in_days: i64,
    /// <p>The duration for which data must be stored in the memory store. </p>
    #[serde(rename = "MemoryStoreRetentionPeriodInHours")]
    pub memory_store_retention_period_in_hours: i64,
}

/// <p>Table represents a database table in Timestream. Tables contain one or more related time series. You can modify the retention duration of the memory store and the magnetic store for a table. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Table {
    /// <p>The Amazon Resource Name that uniquely identifies this table.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time when the Timestream table was created. </p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the Timestream database that contains this table.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The time when the Timestream table was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The retention duration for the memory store and magnetic store.</p>
    #[serde(rename = "RetentionProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_properties: Option<RetentionProperties>,
    /// <p>The name of the Timestream table.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p><p>The current state of the table:</p> <ul> <li> <p> <code>DELETING</code> - The table is being deleted.</p> </li> <li> <p> <code>ACTIVE</code> - The table is ready for use.</p> </li> </ul></p>
    #[serde(rename = "TableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,
}

/// <p> A tag is a label that you assign to a Timestream database and/or table. Each tag consists of a key and an optional value, both of which you define. Tags enable you to categorize databases and/or tables, for example, by purpose, owner, or environment. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p> The key of the tag. Tag keys are case sensitive. </p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p> The value of the tag. Tag values are case-sensitive and can be null. </p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p> Identifies the Timestream resource to which tags should be added. This value is an Amazon Resource Name (ARN). </p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p> The tags to be assigned to the Timestream resource. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p> The Timestream resource that the tags will be removed from. This value is an Amazon Resource Name (ARN). </p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p> A list of tags keys. Existing tags of the resource whose keys are members of this list will be removed from the Timestream resource. </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDatabaseRequest {
    /// <p> The name of the database. </p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p><p> The identifier of the new KMS key (<code>KmsKeyId</code>) to be used to encrypt the data stored in the database. If the <code>KmsKeyId</code> currently registered with the database is the same as the <code>KmsKeyId</code> in the request, there will not be any update. </p> <p>You can specify the <code>KmsKeyId</code> using any of the following:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-1:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-1:111122223333:alias/ExampleAlias</code> </p> </li> </ul></p>
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDatabaseResponse {
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateTableRequest {
    /// <p>The name of the Timestream database.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>The retention duration of the memory store and the magnetic store.</p>
    #[serde(rename = "RetentionProperties")]
    pub retention_properties: RetentionProperties,
    /// <p>The name of the Timesream table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateTableResponse {
    /// <p>The updated Timestream table.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WriteRecordsRequest {
    /// <p>A record containing the common measure and dimension attributes shared across all the records in the request. The measure and dimension attributes specified in here will be merged with the measure and dimension attributes in the records object when the data is written into Timestream. </p>
    #[serde(rename = "CommonAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_attributes: Option<Record>,
    /// <p>The name of the Timestream database.</p>
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// <p>An array of records containing the unique dimension and measure attributes for each time series data point. </p>
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
    /// <p>The name of the Timesream table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// Errors returned by CreateDatabase
#[derive(Debug, PartialEq)]
pub enum CreateDatabaseError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p>Timestream was unable to process this request because it contains resource that already exists.</p>
    Conflict(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p> Instance quota of resource exceeded for this account.</p>
    ServiceQuotaExceeded(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl CreateDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDatabaseError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDatabaseError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateDatabaseError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(CreateDatabaseError::InvalidEndpoint(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateDatabaseError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDatabaseError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateDatabaseError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateDatabaseError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDatabaseError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            CreateDatabaseError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateDatabaseError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDatabaseError {}
/// Errors returned by CreateTable
#[derive(Debug, PartialEq)]
pub enum CreateTableError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p>Timestream was unable to process this request because it contains resource that already exists.</p>
    Conflict(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Instance quota of resource exceeded for this account.</p>
    ServiceQuotaExceeded(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl CreateTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateTableError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateTableError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateTableError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(CreateTableError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateTableError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CreateTableError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateTableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTableError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateTableError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateTableError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateTableError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            CreateTableError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateTableError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            CreateTableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTableError {}
/// Errors returned by DeleteDatabase
#[derive(Debug, PartialEq)]
pub enum DeleteDatabaseError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl DeleteDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDatabaseError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDatabaseError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(DeleteDatabaseError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDatabaseError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDatabaseError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteDatabaseError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDatabaseError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            DeleteDatabaseError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDatabaseError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDatabaseError {}
/// Errors returned by DeleteTable
#[derive(Debug, PartialEq)]
pub enum DeleteTableError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl DeleteTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteTableError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteTableError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(DeleteTableError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTableError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteTableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTableError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteTableError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteTableError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            DeleteTableError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTableError {}
/// Errors returned by DescribeDatabase
#[derive(Debug, PartialEq)]
pub enum DescribeDatabaseError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl DescribeDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeDatabaseError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeDatabaseError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(DescribeDatabaseError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDatabaseError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDatabaseError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeDatabaseError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeDatabaseError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            DescribeDatabaseError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDatabaseError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDatabaseError {}
/// Errors returned by DescribeEndpoints
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointsError {
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl DescribeEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeEndpointsError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeEndpointsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEndpointsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEndpointsError {}
/// Errors returned by DescribeTable
#[derive(Debug, PartialEq)]
pub enum DescribeTableError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl DescribeTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeTableError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DescribeTableError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(DescribeTableError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTableError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeTableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTableError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeTableError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeTableError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            DescribeTableError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTableError {}
/// Errors returned by ListDatabases
#[derive(Debug, PartialEq)]
pub enum ListDatabasesError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl ListDatabasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatabasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDatabasesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListDatabasesError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(ListDatabasesError::InvalidEndpoint(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDatabasesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDatabasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDatabasesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDatabasesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDatabasesError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            ListDatabasesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDatabasesError {}
/// Errors returned by ListTables
#[derive(Debug, PartialEq)]
pub enum ListTablesError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl ListTablesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTablesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTablesError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListTablesError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(ListTablesError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTablesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTablesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTablesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTablesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTablesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTablesError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            ListTablesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTablesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTablesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidEndpointException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTagsForResourceError::Throttling(err.msg))
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
            ListTagsForResourceError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Instance quota of resource exceeded for this account.</p>
    ServiceQuotaExceeded(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidEndpointException" => {
                    return RusotoError::Service(TagResourceError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(TagResourceError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(TagResourceError::Throttling(err.msg))
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
            TagResourceError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Instance quota of resource exceeded for this account.</p>
    ServiceQuotaExceeded(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidEndpointException" => {
                    return RusotoError::Service(UntagResourceError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(UntagResourceError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UntagResourceError::Throttling(err.msg))
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
            UntagResourceError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateDatabase
#[derive(Debug, PartialEq)]
pub enum UpdateDatabaseError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Instance quota of resource exceeded for this account.</p>
    ServiceQuotaExceeded(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl UpdateDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDatabaseError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateDatabaseError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(UpdateDatabaseError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDatabaseError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(UpdateDatabaseError::ServiceQuotaExceeded(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDatabaseError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDatabaseError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDatabaseError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateDatabaseError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateDatabaseError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            UpdateDatabaseError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateDatabaseError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
            UpdateDatabaseError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDatabaseError {}
/// Errors returned by UpdateTable
#[derive(Debug, PartialEq)]
pub enum UpdateTableError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl UpdateTableError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTableError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateTableError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateTableError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(UpdateTableError::InvalidEndpoint(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateTableError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateTableError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateTableError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateTableError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateTableError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateTableError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            UpdateTableError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateTableError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateTableError {}
/// Errors returned by WriteRecords
#[derive(Debug, PartialEq)]
pub enum WriteRecordsError {
    /// <p>You are not authorized to perform this action.</p>
    AccessDenied(String),
    /// <p> Timestream was unable to fully process this request because of an internal server error.</p>
    InternalServer(String),
    /// <p>The requested endpoint was invalid.</p>
    InvalidEndpoint(String),
    /// <p> WriteRecords would throw this exception in the following cases: </p> <ul> <li> <p> Records with duplicate data where there are multiple records with the same dimensions, timestamps, and measure names but different measure values. </p> </li> <li> <p> Records with timestamps that lie outside the retention duration of the memory store </p> </li> <li> <p> Records with dimensions or measures that exceed the Timestream defined limits. </p> </li> </ul> <p> For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    RejectedRecords(String),
    /// <p>The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE.</p>
    ResourceNotFound(String),
    /// <p> Too many requests were made by a user exceeding service quotas. The request was throttled.</p>
    Throttling(String),
}

impl WriteRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<WriteRecordsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(WriteRecordsError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(WriteRecordsError::InternalServer(err.msg))
                }
                "InvalidEndpointException" => {
                    return RusotoError::Service(WriteRecordsError::InvalidEndpoint(err.msg))
                }
                "RejectedRecordsException" => {
                    return RusotoError::Service(WriteRecordsError::RejectedRecords(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(WriteRecordsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(WriteRecordsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for WriteRecordsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WriteRecordsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            WriteRecordsError::InternalServer(ref cause) => write!(f, "{}", cause),
            WriteRecordsError::InvalidEndpoint(ref cause) => write!(f, "{}", cause),
            WriteRecordsError::RejectedRecords(ref cause) => write!(f, "{}", cause),
            WriteRecordsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            WriteRecordsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for WriteRecordsError {}
/// Trait representing the capabilities of the Timestream Write API. Timestream Write clients implement this trait.
#[async_trait]
pub trait TimestreamWrite {
    /// <p>Creates a new Timestream database. If the KMS key is not specified, the database will be encrypted with a Timestream managed KMS key located in your account. Refer to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed KMS keys</a> for more info. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn create_database(
        &self,
        input: CreateDatabaseRequest,
    ) -> Result<CreateDatabaseResponse, RusotoError<CreateDatabaseError>>;

    /// <p>The CreateTable operation adds a new table to an existing database in your account. In an AWS account, table names must be at least unique within each Region if they are in the same database. You may have identical table names in the same Region if the tables are in seperate databases. While creating the table, you must specify the table name, database name, and the retention properties. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn create_table(
        &self,
        input: CreateTableRequest,
    ) -> Result<CreateTableResponse, RusotoError<CreateTableError>>;

    /// <p>Deletes a given Timestream database. <i>This is an irreversible operation. After a database is deleted, the time series data from its tables cannot be recovered.</i> </p> <p>All tables in the database must be deleted first, or a ValidationException error will be thrown. </p> <p>Due to the nature of distributed retries, the operation can return either success or a ResourceNotFoundException. Clients should consider them equivalent.</p>
    async fn delete_database(
        &self,
        input: DeleteDatabaseRequest,
    ) -> Result<(), RusotoError<DeleteDatabaseError>>;

    /// <p>Deletes a given Timestream table. This is an irreversible operation. After a Timestream database table is deleted, the time series data stored in the table cannot be recovered. </p> <p>Due to the nature of distributed retries, the operation can return either success or a ResourceNotFoundException. Clients should consider them equivalent.</p>
    async fn delete_table(
        &self,
        input: DeleteTableRequest,
    ) -> Result<(), RusotoError<DeleteTableError>>;

    /// <p>Returns information about the database, including the database name, time that the database was created, and the total number of tables found within the database. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide.</p>
    async fn describe_database(
        &self,
        input: DescribeDatabaseRequest,
    ) -> Result<DescribeDatabaseResponse, RusotoError<DescribeDatabaseError>>;

    /// <p>DescribeEndpoints returns a list of available endpoints to make Timestream API calls against. This API is available through both Write and Query.</p> <p>Because Timestream’s SDKs are designed to transparently work with the service’s architecture, including the management and mapping of the service endpoints, <i>it is not recommended that you use this API unless</i>:</p> <ul> <li> <p>Your application uses a programming language that does not yet have SDK support</p> </li> <li> <p>You require better control over the client-side implementation</p> </li> </ul> <p>For detailed information on how to use DescribeEndpoints, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/Using-API.endpoint-discovery.html">The Endpoint Discovery Pattern and REST APIs</a>.</p>
    async fn describe_endpoints(
        &self,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>>;

    /// <p>Returns information about the table, including the table name, database name, retention duration of the memory store and the magnetic store. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn describe_table(
        &self,
        input: DescribeTableRequest,
    ) -> Result<DescribeTableResponse, RusotoError<DescribeTableError>>;

    /// <p>Returns a list of your Timestream databases. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn list_databases(
        &self,
        input: ListDatabasesRequest,
    ) -> Result<ListDatabasesResponse, RusotoError<ListDatabasesError>>;

    /// <p>A list of tables, along with the name, status and retention properties of each table. </p>
    async fn list_tables(
        &self,
        input: ListTablesRequest,
    ) -> Result<ListTablesResponse, RusotoError<ListTablesError>>;

    /// <p> List all tags on a Timestream resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p> Associate a set of tags with a Timestream resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p> Removes the association of tags from a Timestream resource. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p> Modifies the KMS key for an existing database. While updating the database, you must specify the database name and the identifier of the new KMS key to be used (<code>KmsKeyId</code>). If there are any concurrent <code>UpdateDatabase</code> requests, first writer wins. </p>
    async fn update_database(
        &self,
        input: UpdateDatabaseRequest,
    ) -> Result<UpdateDatabaseResponse, RusotoError<UpdateDatabaseError>>;

    /// <p>Modifies the retention duration of the memory store and magnetic store for your Timestream table. Note that the change in retention duration takes effect immediately. For example, if the retention period of the memory store was initially set to 2 hours and then changed to 24 hours, the memory store will be capable of holding 24 hours of data, but will be populated with 24 hours of data 22 hours after this change was made. Timestream does not retrieve data from the magnetic store to populate the memory store. </p> <p>Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide.</p>
    async fn update_table(
        &self,
        input: UpdateTableRequest,
    ) -> Result<UpdateTableResponse, RusotoError<UpdateTableError>>;

    /// <p>The WriteRecords operation enables you to write your time series data into Timestream. You can specify a single data point or a batch of data points to be inserted into the system. Timestream offers you with a flexible schema that auto detects the column names and data types for your Timestream tables based on the dimension names and data types of the data points you specify when invoking writes into the database. Timestream support eventual consistency read semantics. This means that when you query data immediately after writing a batch of data into Timestream, the query results might not reflect the results of a recently completed write operation. The results may also include some stale data. If you repeat the query request after a short time, the results should return the latest data. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn write_records(
        &self,
        input: WriteRecordsRequest,
    ) -> Result<(), RusotoError<WriteRecordsError>>;
}
/// A client for the Timestream Write API.
#[derive(Clone)]
pub struct TimestreamWriteClient {
    client: Client,
    region: region::Region,
}

impl TimestreamWriteClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> TimestreamWriteClient {
        TimestreamWriteClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TimestreamWriteClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        TimestreamWriteClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> TimestreamWriteClient {
        TimestreamWriteClient { client, region }
    }
}

#[async_trait]
impl TimestreamWrite for TimestreamWriteClient {
    /// <p>Creates a new Timestream database. If the KMS key is not specified, the database will be encrypted with a Timestream managed KMS key located in your account. Refer to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed KMS keys</a> for more info. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn create_database(
        &self,
        input: CreateDatabaseRequest,
    ) -> Result<CreateDatabaseResponse, RusotoError<CreateDatabaseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.CreateDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDatabaseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDatabaseResponse, _>()
    }

    /// <p>The CreateTable operation adds a new table to an existing database in your account. In an AWS account, table names must be at least unique within each Region if they are in the same database. You may have identical table names in the same Region if the tables are in seperate databases. While creating the table, you must specify the table name, database name, and the retention properties. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn create_table(
        &self,
        input: CreateTableRequest,
    ) -> Result<CreateTableResponse, RusotoError<CreateTableError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.CreateTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTableError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTableResponse, _>()
    }

    /// <p>Deletes a given Timestream database. <i>This is an irreversible operation. After a database is deleted, the time series data from its tables cannot be recovered.</i> </p> <p>All tables in the database must be deleted first, or a ValidationException error will be thrown. </p> <p>Due to the nature of distributed retries, the operation can return either success or a ResourceNotFoundException. Clients should consider them equivalent.</p>
    async fn delete_database(
        &self,
        input: DeleteDatabaseRequest,
    ) -> Result<(), RusotoError<DeleteDatabaseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.DeleteDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDatabaseError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes a given Timestream table. This is an irreversible operation. After a Timestream database table is deleted, the time series data stored in the table cannot be recovered. </p> <p>Due to the nature of distributed retries, the operation can return either success or a ResourceNotFoundException. Clients should consider them equivalent.</p>
    async fn delete_table(
        &self,
        input: DeleteTableRequest,
    ) -> Result<(), RusotoError<DeleteTableError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.DeleteTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTableError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Returns information about the database, including the database name, time that the database was created, and the total number of tables found within the database. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide.</p>
    async fn describe_database(
        &self,
        input: DescribeDatabaseRequest,
    ) -> Result<DescribeDatabaseResponse, RusotoError<DescribeDatabaseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.DescribeDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDatabaseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeDatabaseResponse, _>()
    }

    /// <p>DescribeEndpoints returns a list of available endpoints to make Timestream API calls against. This API is available through both Write and Query.</p> <p>Because Timestream’s SDKs are designed to transparently work with the service’s architecture, including the management and mapping of the service endpoints, <i>it is not recommended that you use this API unless</i>:</p> <ul> <li> <p>Your application uses a programming language that does not yet have SDK support</p> </li> <li> <p>You require better control over the client-side implementation</p> </li> </ul> <p>For detailed information on how to use DescribeEndpoints, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/Using-API.endpoint-discovery.html">The Endpoint Discovery Pattern and REST APIs</a>.</p>
    async fn describe_endpoints(
        &self,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.DescribeEndpoints");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DescribeEndpointsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEndpointsResponse, _>()
    }

    /// <p>Returns information about the table, including the table name, database name, retention duration of the memory store and the magnetic store. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn describe_table(
        &self,
        input: DescribeTableRequest,
    ) -> Result<DescribeTableResponse, RusotoError<DescribeTableError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.DescribeTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTableError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeTableResponse, _>()
    }

    /// <p>Returns a list of your Timestream databases. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn list_databases(
        &self,
        input: ListDatabasesRequest,
    ) -> Result<ListDatabasesResponse, RusotoError<ListDatabasesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.ListDatabases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDatabasesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListDatabasesResponse, _>()
    }

    /// <p>A list of tables, along with the name, status and retention properties of each table. </p>
    async fn list_tables(
        &self,
        input: ListTablesRequest,
    ) -> Result<ListTablesResponse, RusotoError<ListTablesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.ListTables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTablesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTablesResponse, _>()
    }

    /// <p> List all tags on a Timestream resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p> Associate a set of tags with a Timestream resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p> Removes the association of tags from a Timestream resource. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p> Modifies the KMS key for an existing database. While updating the database, you must specify the database name and the identifier of the new KMS key to be used (<code>KmsKeyId</code>). If there are any concurrent <code>UpdateDatabase</code> requests, first writer wins. </p>
    async fn update_database(
        &self,
        input: UpdateDatabaseRequest,
    ) -> Result<UpdateDatabaseResponse, RusotoError<UpdateDatabaseError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.UpdateDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDatabaseError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateDatabaseResponse, _>()
    }

    /// <p>Modifies the retention duration of the memory store and magnetic store for your Timestream table. Note that the change in retention duration takes effect immediately. For example, if the retention period of the memory store was initially set to 2 hours and then changed to 24 hours, the memory store will be capable of holding 24 hours of data, but will be populated with 24 hours of data 22 hours after this change was made. Timestream does not retrieve data from the magnetic store to populate the memory store. </p> <p>Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide.</p>
    async fn update_table(
        &self,
        input: UpdateTableRequest,
    ) -> Result<UpdateTableResponse, RusotoError<UpdateTableError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.UpdateTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateTableError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateTableResponse, _>()
    }

    /// <p>The WriteRecords operation enables you to write your time series data into Timestream. You can specify a single data point or a batch of data points to be inserted into the system. Timestream offers you with a flexible schema that auto detects the column names and data types for your Timestream tables based on the dimension names and data types of the data points you specify when invoking writes into the database. Timestream support eventual consistency read semantics. This means that when you query data immediately after writing a batch of data into Timestream, the query results might not reflect the results of a recently completed write operation. The results may also include some stale data. If you repeat the query request after a short time, the results should return the latest data. Service quotas apply. For more information, see <a href="https://docs.aws.amazon.com/timestream/latest/developerguide/ts-limits.html">Access Management</a> in the Timestream Developer Guide. </p>
    async fn write_records(
        &self,
        input: WriteRecordsRequest,
    ) -> Result<(), RusotoError<WriteRecordsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Timestream_20181101.WriteRecords");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, WriteRecordsError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }
}
