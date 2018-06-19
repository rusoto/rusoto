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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Represents an attribute for describing the key schema for the table and indexes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeDefinition {
    /// <p>A name for the attribute.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p><p>The data type for the attribute, where:</p> <ul> <li> <p> <code>S</code> - the attribute is of type String</p> </li> <li> <p> <code>N</code> - the attribute is of type Number</p> </li> <li> <p> <code>B</code> - the attribute is of type Binary</p> </li> </ul></p>
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
}

/// <p>Represents the data for an attribute.</p> <p>Each attribute value is described as a name-value pair. The name is the data type, and the value is the data itself.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes">Data Types</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeValue {
    /// <p>An attribute of type Binary. For example:</p> <p> <code>"B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk"</code> </p>
    #[serde(rename = "B")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub b: Option<Vec<u8>>,
    /// <p>An attribute of type Boolean. For example:</p> <p> <code>"BOOL": true</code> </p>
    #[serde(rename = "BOOL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    /// <p>An attribute of type Binary Set. For example:</p> <p> <code>"BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="]</code> </p>
    #[serde(rename = "BS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<Vec<Vec<u8>>>,
    /// <p>An attribute of type List. For example:</p> <p> <code>"L": ["Cookies", "Coffee", 3.14159]</code> </p>
    #[serde(rename = "L")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l: Option<Vec<AttributeValue>>,
    /// <p>An attribute of type Map. For example:</p> <p> <code>"M": {"Name": {"S": "Joe"}, "Age": {"N": "35"}}</code> </p>
    #[serde(rename = "M")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>An attribute of type Number. For example:</p> <p> <code>"N": "123.45"</code> </p> <p>Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across languages and libraries. However, DynamoDB treats them as number type attributes for mathematical operations.</p>
    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    /// <p>An attribute of type Number Set. For example:</p> <p> <code>"NS": ["42.2", "-19", "7.5", "3.14"]</code> </p> <p>Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across languages and libraries. However, DynamoDB treats them as number type attributes for mathematical operations.</p>
    #[serde(rename = "NS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<Vec<String>>,
    /// <p>An attribute of type Null. For example:</p> <p> <code>"NULL": true</code> </p>
    #[serde(rename = "NULL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
    /// <p>An attribute of type String. For example:</p> <p> <code>"S": "Hello"</code> </p>
    #[serde(rename = "S")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// <p>An attribute of type String Set. For example:</p> <p> <code>"SS": ["Giraffe", "Hippo" ,"Zebra"]</code> </p>
    #[serde(rename = "SS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss: Option<Vec<String>>,
}

/// <p>For the <code>UpdateItem</code> operation, represents the attributes to be modified, the action to perform on each, and the new value for each.</p> <note> <p>You cannot use <code>UpdateItem</code> to update any primary key attributes. Instead, you will need to delete the item, and then use <code>PutItem</code> to create a new item with new attributes.</p> </note> <p>Attribute values cannot be null; string and binary type attributes must have lengths greater than zero; and set type attributes must not be empty. Requests with empty values will be rejected with a <code>ValidationException</code> exception.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttributeValueUpdate {
    /// <p><p>Specifies how to perform the update. Valid values are <code>PUT</code> (default), <code>DELETE</code>, and <code>ADD</code>. The behavior depends on whether the specified primary key already exists in the table.</p> <p> <b>If an item with the specified <i>Key</i> is found in the table:</b> </p> <ul> <li> <p> <code>PUT</code> - Adds the specified attribute to the item. If the attribute already exists, it is replaced by the new value. </p> </li> <li> <p> <code>DELETE</code> - If no value is specified, the attribute and its value are removed from the item. The data type of the specified value must match the existing value&#39;s data type.</p> <p>If a <i>set</i> of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>[a,b,c]</code> and the <code>DELETE</code> action specified <code>[a,c]</code>, then the final attribute value would be <code>[b]</code>. Specifying an empty set is an error.</p> </li> <li> <p> <code>ADD</code> - If the attribute does not already exist, then the attribute and its values are added to the item. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p> <ul> <li> <p>If the existing attribute is a number, and if <code>Value</code> is also a number, then the <code>Value</code> is mathematically added to the existing attribute. If <code>Value</code> is a negative number, then it is subtracted from the existing attribute.</p> <note> <p> If you use <code>ADD</code> to increment or decrement a number value for an item that doesn&#39;t exist before the update, DynamoDB uses 0 as the initial value.</p> <p>In addition, if you use <code>ADD</code> to update an existing item, and intend to increment or decrement an attribute value which does not yet exist, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update does not yet have an attribute named <i>itemcount</i>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway, even though it currently does not exist. DynamoDB will create the <i>itemcount</i> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <i>itemcount</i> attribute in the item, with a value of <code>3</code>.</p> </note> </li> <li> <p>If the existing data type is a set, and if the <code>Value</code> is also a set, then the <code>Value</code> is added to the existing set. (This is a <i>set</i> operation, not mathematical addition.) For example, if the attribute value was the set <code>[1,2]</code>, and the <code>ADD</code> action specified <code>[3]</code>, then the final attribute value would be <code>[1,2,3]</code>. An error occurs if an Add action is specified for a set attribute and the attribute type specified does not match the existing set type. </p> <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <code>Value</code> must also be a set of strings. The same holds true for number sets and binary sets.</p> </li> </ul> <p>This action is only valid for an existing attribute whose data type is number or is a set. Do not use <code>ADD</code> for any other data types.</p> </li> </ul> <p> <b>If no item with the specified <i>Key</i> is found:</b> </p> <ul> <li> <p> <code>PUT</code> - DynamoDB creates a new item with the specified primary key, and then adds the attribute. </p> </li> <li> <p> <code>DELETE</code> - Nothing happens; there is no attribute to delete.</p> </li> <li> <p> <code>ADD</code> - DynamoDB creates an item with the supplied primary key and number (or set of numbers) for the attribute value. The only data types allowed are number and number set; no other data types can be specified.</p> </li> </ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>Represents the data for an attribute.</p> <p>Each attribute value is described as a name-value pair. The name is the data type, and the value is the data itself.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes">Data Types</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
}

/// <p>Contains the description of the backup created for the table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BackupDescription {
    /// <p>Contains the details of the backup created for the table. </p>
    #[serde(rename = "BackupDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_details: Option<BackupDetails>,
    /// <p>Contains the details of the table when the backup was created. </p>
    #[serde(rename = "SourceTableDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_details: Option<SourceTableDetails>,
    /// <p>Contains the details of the features enabled on the table when the backup was created. For example, LSIs, GSIs, streams, TTL.</p>
    #[serde(rename = "SourceTableFeatureDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_feature_details: Option<SourceTableFeatureDetails>,
}

/// <p>Contains the details of the backup created for the table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BackupDetails {
    /// <p>ARN associated with the backup.</p>
    #[serde(rename = "BackupArn")]
    pub backup_arn: String,
    /// <p>Time at which the backup was created. This is the request time of the backup. </p>
    #[serde(rename = "BackupCreationDateTime")]
    pub backup_creation_date_time: f64,
    /// <p>Name of the requested backup.</p>
    #[serde(rename = "BackupName")]
    pub backup_name: String,
    /// <p>Size of the backup in bytes.</p>
    #[serde(rename = "BackupSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_bytes: Option<i64>,
    /// <p>Backup can be in one of the following states: CREATING, ACTIVE, DELETED. </p>
    #[serde(rename = "BackupStatus")]
    pub backup_status: String,
}

/// <p>Contains details for the backup.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BackupSummary {
    /// <p>ARN associated with the backup.</p>
    #[serde(rename = "BackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_arn: Option<String>,
    /// <p>Time at which the backup was created.</p>
    #[serde(rename = "BackupCreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_creation_date_time: Option<f64>,
    /// <p>Name of the specified backup.</p>
    #[serde(rename = "BackupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    /// <p>Size of the backup in bytes.</p>
    #[serde(rename = "BackupSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_bytes: Option<i64>,
    /// <p>Backup can be in one of the following states: CREATING, ACTIVE, DELETED.</p>
    #[serde(rename = "BackupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<String>,
    /// <p>ARN associated with the table.</p>
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    /// <p>Unique identifier for the table.</p>
    #[serde(rename = "TableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// <p>Name of the table.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

/// <p>Represents the input of a <code>BatchGetItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetItemInput {
    /// <p><p>A map of one or more table names and, for each table, a map that describes one or more items to retrieve from that table. Each table name can be used only once per <code>BatchGetItem</code> request.</p> <p>Each element in the map of items to retrieve consists of the following:</p> <ul> <li> <p> <code>ConsistentRead</code> - If <code>true</code>, a strongly consistent read is used; if <code>false</code> (the default), an eventually consistent read is used.</p> </li> <li> <p> <code>ExpressionAttributeNames</code> - One or more substitution tokens for attribute names in the <code>ProjectionExpression</code> parameter. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p> <code>{&quot;#P&quot;:&quot;Percentile&quot;}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </li> <li> <p> <code>Keys</code> - An array of primary key attribute values that define specific items in the table. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key value. For a composite key, you must provide <i>both</i> the partition key value and the sort key value.</p> </li> <li> <p> <code>ProjectionExpression</code> - A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </li> <li> <p> <code>AttributesToGet</code> - This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p> </li> </ul></p>
    #[serde(rename = "RequestItems")]
    pub request_items: ::std::collections::HashMap<String, KeysAndAttributes>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
}

/// <p>Represents the output of a <code>BatchGetItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchGetItemOutput {
    /// <p><p>The read capacity units consumed by the entire <code>BatchGetItem</code> operation.</p> <p>Each element consists of:</p> <ul> <li> <p> <code>TableName</code> - The table that consumed the provisioned throughput.</p> </li> <li> <p> <code>CapacityUnits</code> - The total number of capacity units consumed.</p> </li> </ul></p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<Vec<ConsumedCapacity>>,
    /// <p>A map of table name to a list of items. Each object in <code>Responses</code> consists of a table name, along with a map of attribute data consisting of the data type and attribute value.</p>
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<
        ::std::collections::HashMap<
            String,
            Vec<::std::collections::HashMap<String, AttributeValue>>,
        >,
    >,
    /// <p>A map of tables and their respective keys that were not processed with the current response. The <code>UnprocessedKeys</code> value is in the same form as <code>RequestItems</code>, so the value can be provided directly to a subsequent <code>BatchGetItem</code> operation. For more information, see <code>RequestItems</code> in the Request Parameters section.</p> <p>Each element consists of:</p> <ul> <li> <p> <code>Keys</code> - An array of primary key attribute values that define specific items in the table.</p> </li> <li> <p> <code>ProjectionExpression</code> - One or more attributes to be retrieved from the table or index. By default, all attributes are returned. If a requested attribute is not found, it does not appear in the result.</p> </li> <li> <p> <code>ConsistentRead</code> - The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p> </li> </ul> <p>If there are no unprocessed keys remaining, the response contains an empty <code>UnprocessedKeys</code> map.</p>
    #[serde(rename = "UnprocessedKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_keys: Option<::std::collections::HashMap<String, KeysAndAttributes>>,
}

/// <p>Represents the input of a <code>BatchWriteItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchWriteItemInput {
    /// <p><p>A map of one or more table names and, for each table, a list of operations to be performed (<code>DeleteRequest</code> or <code>PutRequest</code>). Each element in the map consists of the following:</p> <ul> <li> <p> <code>DeleteRequest</code> - Perform a <code>DeleteItem</code> operation on the specified item. The item to be deleted is identified by a <code>Key</code> subelement:</p> <ul> <li> <p> <code>Key</code> - A map of primary key attribute values that uniquely identify the item. Each entry in this map consists of an attribute name and an attribute value. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for <i>both</i> the partition key and the sort key.</p> </li> </ul> </li> <li> <p> <code>PutRequest</code> - Perform a <code>PutItem</code> operation on the specified item. The item to be put is identified by an <code>Item</code> subelement:</p> <ul> <li> <p> <code>Item</code> - A map of attributes and their values. Each entry in this map consists of an attribute name and an attribute value. Attribute values must not be null; string and binary type attributes must have lengths greater than zero; and set type attributes must not be empty. Requests that contain empty values will be rejected with a <code>ValidationException</code> exception.</p> <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table&#39;s attribute definition.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "RequestItems")]
    pub request_items: ::std::collections::HashMap<String, Vec<WriteRequest>>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
}

/// <p>Represents the output of a <code>BatchWriteItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchWriteItemOutput {
    /// <p><p>The capacity units consumed by the entire <code>BatchWriteItem</code> operation.</p> <p>Each element consists of:</p> <ul> <li> <p> <code>TableName</code> - The table that consumed the provisioned throughput.</p> </li> <li> <p> <code>CapacityUnits</code> - The total number of capacity units consumed.</p> </li> </ul></p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<Vec<ConsumedCapacity>>,
    /// <p><p>A list of tables that were processed by <code>BatchWriteItem</code> and, for each table, information about any item collections that were affected by individual <code>DeleteItem</code> or <code>PutItem</code> operations.</p> <p>Each entry consists of the following subelements:</p> <ul> <li> <p> <code>ItemCollectionKey</code> - The partition key value of the item collection. This is the same as the partition key value of the item.</p> </li> <li> <p> <code>SizeEstimateRangeGB</code> - An estimate of item collection size, expressed in GB. This is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on the table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li> </ul></p>
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics:
        Option<::std::collections::HashMap<String, Vec<ItemCollectionMetrics>>>,
    /// <p>A map of tables and requests against those tables that were not processed. The <code>UnprocessedItems</code> value is in the same form as <code>RequestItems</code>, so you can provide this value directly to a subsequent <code>BatchGetItem</code> operation. For more information, see <code>RequestItems</code> in the Request Parameters section.</p> <p>Each <code>UnprocessedItems</code> entry consists of a table name and, for that table, a list of operations to perform (<code>DeleteRequest</code> or <code>PutRequest</code>).</p> <ul> <li> <p> <code>DeleteRequest</code> - Perform a <code>DeleteItem</code> operation on the specified item. The item to be deleted is identified by a <code>Key</code> subelement:</p> <ul> <li> <p> <code>Key</code> - A map of primary key attribute values that uniquely identify the item. Each entry in this map consists of an attribute name and an attribute value.</p> </li> </ul> </li> <li> <p> <code>PutRequest</code> - Perform a <code>PutItem</code> operation on the specified item. The item to be put is identified by an <code>Item</code> subelement:</p> <ul> <li> <p> <code>Item</code> - A map of attributes and their values. Each entry in this map consists of an attribute name and an attribute value. Attribute values must not be null; string and binary type attributes must have lengths greater than zero; and set type attributes must not be empty. Requests that contain empty values will be rejected with a <code>ValidationException</code> exception.</p> <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p> </li> </ul> </li> </ul> <p>If there are no unprocessed items remaining, the response contains an empty <code>UnprocessedItems</code> map.</p>
    #[serde(rename = "UnprocessedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_items: Option<::std::collections::HashMap<String, Vec<WriteRequest>>>,
}

/// <p>Represents the amount of provisioned throughput capacity consumed on a table or an index.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Capacity {
    /// <p>The total number of capacity units consumed on a table or an index.</p>
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<f64>,
}

/// <p><p>Represents the selection criteria for a <code>Query</code> or <code>Scan</code> operation:</p> <ul> <li> <p>For a <code>Query</code> operation, <code>Condition</code> is used for specifying the <code>KeyConditions</code> to use when querying a table or an index. For <code>KeyConditions</code>, only the following comparison operators are supported:</p> <p> <code>EQ | LE | LT | GE | GT | BEGINS_WITH | BETWEEN</code> </p> <p> <code>Condition</code> is also used in a <code>QueryFilter</code>, which evaluates the query results and returns only the desired values.</p> </li> <li> <p>For a <code>Scan</code> operation, <code>Condition</code> is used in a <code>ScanFilter</code>, which evaluates the scan results and returns only the desired values.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Condition {
    /// <p>One or more values to evaluate against the supplied attribute. The number of values in the list depends on the <code>ComparisonOperator</code> being used.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href="http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p>
    #[serde(rename = "AttributeValueList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value_list: Option<Vec<AttributeValue>>,
    /// <p>A comparator for evaluating attributes. For example, equals, greater than, less than, etc.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS | BEGINS_WITH | IN | BETWEEN</code> </p> <p>The following are descriptions of each comparison operator.</p> <ul> <li> <p> <code>EQ</code> : Equal. <code>EQ</code> is supported for all data types, including lists and maps.</p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{"S":"6"}</code> does not equal <code>{"N":"6"}</code>. Also, <code>{"N":"6"}</code> does not equal <code>{"NS":["6", "2", "1"]}</code>.</p> <p/> </li> <li> <p> <code>NE</code> : Not equal. <code>NE</code> is supported for all data types, including lists and maps.</p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <code>AttributeValue</code> of a different type than the one provided in the request, the value does not match. For example, <code>{"S":"6"}</code> does not equal <code>{"N":"6"}</code>. Also, <code>{"N":"6"}</code> does not equal <code>{"NS":["6", "2", "1"]}</code>.</p> <p/> </li> <li> <p> <code>LE</code> : Less than or equal. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{"S":"6"}</code> does not equal <code>{"N":"6"}</code>. Also, <code>{"N":"6"}</code> does not compare to <code>{"NS":["6", "2", "1"]}</code>.</p> <p/> </li> <li> <p> <code>LT</code> : Less than. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> of type String, Number, or Binary (not a set type). If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{"S":"6"}</code> does not equal <code>{"N":"6"}</code>. Also, <code>{"N":"6"}</code> does not compare to <code>{"NS":["6", "2", "1"]}</code>.</p> <p/> </li> <li> <p> <code>GE</code> : Greater than or equal. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{"S":"6"}</code> does not equal <code>{"N":"6"}</code>. Also, <code>{"N":"6"}</code> does not compare to <code>{"NS":["6", "2", "1"]}</code>.</p> <p/> </li> <li> <p> <code>GT</code> : Greater than. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{"S":"6"}</code> does not equal <code>{"N":"6"}</code>. Also, <code>{"N":"6"}</code> does not compare to <code>{"NS":["6", "2", "1"]}</code>.</p> <p/> </li> <li> <p> <code>NOT_NULL</code> : The attribute exists. <code>NOT_NULL</code> is supported for all data types, including lists and maps.</p> <note> <p>This operator tests for the existence of an attribute, not its data type. If the data type of attribute "<code>a</code>" is null, and you evaluate it using <code>NOT_NULL</code>, the result is a Boolean <code>true</code>. This result is because the attribute "<code>a</code>" exists; its data type is not relevant to the <code>NOT_NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>NULL</code> : The attribute does not exist. <code>NULL</code> is supported for all data types, including lists and maps.</p> <note> <p>This operator tests for the nonexistence of an attribute, not its data type. If the data type of attribute "<code>a</code>" is null, and you evaluate it using <code>NULL</code>, the result is a Boolean <code>false</code>. This is because the attribute "<code>a</code>" exists; its data type is not relevant to the <code>NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>CONTAINS</code> : Checks for a subsequence, or value in a set.</p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is of type String, then the operator checks for a substring match. If the target attribute of the comparison is of type Binary, then the operator looks for a subsequence of the target that matches the input. If the target attribute of the comparison is a set ("<code>SS</code>", "<code>NS</code>", or "<code>BS</code>"), then the operator evaluates to true if it finds an exact match with any member of the set.</p> <p>CONTAINS is supported for lists: When evaluating "<code>a CONTAINS b</code>", "<code>a</code>" can be a list; however, "<code>b</code>" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>NOT_CONTAINS</code> : Checks for absence of a subsequence, or absence of a value in a set.</p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is a String, then the operator checks for the absence of a substring match. If the target attribute of the comparison is Binary, then the operator checks for the absence of a subsequence of the target that matches the input. If the target attribute of the comparison is a set ("<code>SS</code>", "<code>NS</code>", or "<code>BS</code>"), then the operator evaluates to true if it <i>does not</i> find an exact match with any member of the set.</p> <p>NOT_CONTAINS is supported for lists: When evaluating "<code>a NOT CONTAINS b</code>", "<code>a</code>" can be a list; however, "<code>b</code>" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>BEGINS_WITH</code> : Checks for a prefix. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> of type String or Binary (not a Number or a set type). The target attribute of the comparison must be of type String or Binary (not a Number or a set type).</p> <p/> </li> <li> <p> <code>IN</code> : Checks for matching elements in a list.</p> <p> <code>AttributeValueList</code> can contain one or more <code>AttributeValue</code> elements of type String, Number, or Binary. These attributes are compared against an existing attribute of an item. If any elements of the input are equal to the item attribute, the expression evaluates to true.</p> </li> <li> <p> <code>BETWEEN</code> : Greater than or equal to the first value, and less than or equal to the second value. </p> <p> <code>AttributeValueList</code> must contain two <code>AttributeValue</code> elements of the same type, either String, Number, or Binary (not a set type). A target attribute matches if the target value is greater than, or equal to, the first element and less than, or equal to, the second element. If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{"S":"6"}</code> does not compare to <code>{"N":"6"}</code>. Also, <code>{"N":"6"}</code> does not compare to <code>{"NS":["6", "2", "1"]}</code> </p> </li> </ul> <p>For usage examples of <code>AttributeValueList</code> and <code>ComparisonOperator</code>, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.html">Legacy Conditional Parameters</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
}

/// <p>The capacity units consumed by an operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the request asked for it. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ConsumedCapacity {
    /// <p>The total number of capacity units consumed by the operation.</p>
    #[serde(rename = "CapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<f64>,
    /// <p>The amount of throughput consumed on each global index affected by the operation.</p>
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<::std::collections::HashMap<String, Capacity>>,
    /// <p>The amount of throughput consumed on each local index affected by the operation.</p>
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<::std::collections::HashMap<String, Capacity>>,
    /// <p>The amount of throughput consumed on the table affected by the operation.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Capacity>,
    /// <p>The name of the table that was affected by the operation.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

/// <p>Represents the continuous backups and point in time recovery settings on the table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ContinuousBackupsDescription {
    /// <p> <code>ContinuousBackupsStatus</code> can be one of the following states : ENABLED, DISABLED</p>
    #[serde(rename = "ContinuousBackupsStatus")]
    pub continuous_backups_status: String,
    /// <p>The description of the point in time recovery settings applied to the table.</p>
    #[serde(rename = "PointInTimeRecoveryDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_description: Option<PointInTimeRecoveryDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBackupInput {
    /// <p>Specified name for the backup.</p>
    #[serde(rename = "BackupName")]
    pub backup_name: String,
    /// <p>The name of the table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateBackupOutput {
    /// <p>Contains the details of the backup created for the table.</p>
    #[serde(rename = "BackupDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_details: Option<BackupDetails>,
}

/// <p>Represents a new global secondary index to be added to an existing table.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGlobalSecondaryIndexAction {
    /// <p>The name of the global secondary index to be created.</p>
    #[serde(rename = "IndexName")]
    pub index_name: String,
    /// <p>The key schema for the global secondary index.</p>
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchemaElement>,
    /// <p>Represents attributes that are copied (projected) from the table into an index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    #[serde(rename = "Projection")]
    pub projection: Projection,
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p> <p>For current minimum and maximum provisioned throughput values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGlobalTableInput {
    /// <p>The global table name.</p>
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: String,
    /// <p>The regions where the global table needs to be created.</p>
    #[serde(rename = "ReplicationGroup")]
    pub replication_group: Vec<Replica>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateGlobalTableOutput {
    /// <p>Contains the details of the global table.</p>
    #[serde(rename = "GlobalTableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_description: Option<GlobalTableDescription>,
}

/// <p>Represents a replica to be added.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateReplicaAction {
    /// <p>The region of the replica to be added.</p>
    #[serde(rename = "RegionName")]
    pub region_name: String,
}

/// <p>Represents the input of a <code>CreateTable</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTableInput {
    /// <p>An array of attributes that describe the key schema for the table and indexes.</p>
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Vec<AttributeDefinition>,
    /// <p><p>One or more global secondary indexes (the maximum is five) to be created on the table. Each global secondary index in the array includes the following:</p> <ul> <li> <p> <code>IndexName</code> - The name of the global secondary index. Must be unique only for this table.</p> <p/> </li> <li> <p> <code>KeySchema</code> - Specifies the key schema for the global secondary index.</p> </li> <li> <p> <code>Projection</code> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p> <ul> <li> <p> <code>ProjectionType</code> - One of the following:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <code>NonKeyAttributes</code>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul> </li> <li> <p> <code>NonKeyAttributes</code> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <code>NonKeyAttributes</code>, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p> </li> </ul> </li> <li> <p> <code>ProvisionedThroughput</code> - The provisioned throughput settings for the global secondary index, consisting of read and write capacity units.</p> </li> </ul></p>
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndex>>,
    /// <p>Specifies the attributes that make up the primary key for a table or an index. The attributes in <code>KeySchema</code> must also be defined in the <code>AttributeDefinitions</code> array. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html">Data Model</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Each <code>KeySchemaElement</code> in the array is composed of:</p> <ul> <li> <p> <code>AttributeName</code> - The name of this key attribute.</p> </li> <li> <p> <code>KeyType</code> - The role that the key attribute will assume:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term "hash attribute" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term "range attribute" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note> <p>For a simple primary key (partition key), you must provide exactly one element with a <code>KeyType</code> of <code>HASH</code>.</p> <p>For a composite primary key (partition key and sort key), you must provide exactly two elements, in this order: The first element must have a <code>KeyType</code> of <code>HASH</code>, and the second element must have a <code>KeyType</code> of <code>RANGE</code>.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTables.primary.key">Specifying the Primary Key</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchemaElement>,
    /// <p><p>One or more local secondary indexes (the maximum is five) to be created on the table. Each index is scoped to a given partition key value. There is a 10 GB size limit per partition key value; otherwise, the size of a local secondary index is unconstrained.</p> <p>Each local secondary index in the array includes the following:</p> <ul> <li> <p> <code>IndexName</code> - The name of the local secondary index. Must be unique only for this table.</p> <p/> </li> <li> <p> <code>KeySchema</code> - Specifies the key schema for the local secondary index. The key schema must begin with the same partition key as the table.</p> </li> <li> <p> <code>Projection</code> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p> <ul> <li> <p> <code>ProjectionType</code> - One of the following:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <code>NonKeyAttributes</code>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul> </li> <li> <p> <code>NonKeyAttributes</code> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <code>NonKeyAttributes</code>, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndex>>,
    /// <p>Represents the provisioned throughput settings for a specified table or index. The settings can be modified using the <code>UpdateTable</code> operation.</p> <p>For current minimum and maximum provisioned throughput values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
    /// <p>Represents the settings used to enable server-side encryption.</p>
    #[serde(rename = "SSESpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_specification: Option<SSESpecification>,
    /// <p><p>The settings for DynamoDB Streams on the table. These settings consist of:</p> <ul> <li> <p> <code>StreamEnabled</code> - Indicates whether Streams is to be enabled (true) or disabled (false).</p> </li> <li> <p> <code>StreamViewType</code> - When an item in the table is modified, <code>StreamViewType</code> determines what information is written to the table&#39;s stream. Valid values for <code>StreamViewType</code> are:</p> <ul> <li> <p> <code>KEYS<em>ONLY</code> - Only the key attributes of the modified item are written to the stream.</p> </li> <li> <p> <code>NEW</em>IMAGE</code> - The entire item, as it appears after it was modified, is written to the stream.</p> </li> <li> <p> <code>OLD<em>IMAGE</code> - The entire item, as it appeared before it was modified, is written to the stream.</p> </li> <li> <p> <code>NEW</em>AND<em>OLD</em>IMAGES</code> - Both the new and the old item images of the item are written to the stream.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "StreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    /// <p>The name of the table to create.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Represents the output of a <code>CreateTable</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTableOutput {
    /// <p>Represents the properties of the table.</p>
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBackupInput {
    /// <p>The ARN associated with the backup.</p>
    #[serde(rename = "BackupArn")]
    pub backup_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteBackupOutput {
    /// <p>Contains the description of the backup created for the table.</p>
    #[serde(rename = "BackupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_description: Option<BackupDescription>,
}

/// <p>Represents a global secondary index to be deleted from an existing table.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGlobalSecondaryIndexAction {
    /// <p>The name of the global secondary index to be deleted.</p>
    #[serde(rename = "IndexName")]
    pub index_name: String,
}

/// <p>Represents the input of a <code>DeleteItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteItemInput {
    /// <p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p> <p>An expression can contain any of the following:</p> <ul> <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li> <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li> <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li> </ul> <p>For more information on condition expressions, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "Expected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<::std::collections::HashMap<String, ExpectedAttributeValue>>,
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p> <code>{"#P":"Percentile"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<::std::collections::HashMap<String, String>>,
    /// <p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p> <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    #[serde(rename = "Key")]
    pub key: ::std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
    /// <p><p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were deleted. For <code>DeleteItem</code>, the valid values are:</p> <ul> <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li> <li> <p> <code>ALL<em>OLD</code> - The content of the old item is returned.</p> </li> </ul> <note> <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>DeleteItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL</em>OLD</code>.</p> </note></p>
    #[serde(rename = "ReturnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<String>,
    /// <p>The name of the table from which to delete the item.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Represents the output of a <code>DeleteItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteItemOutput {
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the item as it appeared before the <code>DeleteItem</code> operation. This map appears in the response only if <code>ReturnValues</code> was specified as <code>ALL_OLD</code> in the request.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The capacity units consumed by the <code>DeleteItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    /// <p><p>Information about item collections, if any, that were affected by the <code>DeleteItem</code> operation. <code>ItemCollectionMetrics</code> is only returned if the <code>ReturnItemCollectionMetrics</code> parameter was specified. If the table does not have any local secondary indexes, this information is not returned in the response.</p> <p>Each <code>ItemCollectionMetrics</code> element consists of:</p> <ul> <li> <p> <code>ItemCollectionKey</code> - The partition key value of the item collection. This is the same as the partition key value of the item itself.</p> </li> <li> <p> <code>SizeEstimateRangeGB</code> - An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li> </ul></p>
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

/// <p>Represents a replica to be removed.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReplicaAction {
    /// <p>The region of the replica to be removed.</p>
    #[serde(rename = "RegionName")]
    pub region_name: String,
}

/// <p>Represents a request to perform a <code>DeleteItem</code> operation on an item.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRequest {
    /// <p>A map of attribute name to attribute values, representing the primary key of the item to delete. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema.</p>
    #[serde(rename = "Key")]
    pub key: ::std::collections::HashMap<String, AttributeValue>,
}

/// <p>Represents the input of a <code>DeleteTable</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTableInput {
    /// <p>The name of the table to delete.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Represents the output of a <code>DeleteTable</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteTableOutput {
    /// <p>Represents the properties of a table.</p>
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBackupInput {
    /// <p>The ARN associated with the backup.</p>
    #[serde(rename = "BackupArn")]
    pub backup_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeBackupOutput {
    /// <p>Contains the description of the backup created for the table.</p>
    #[serde(rename = "BackupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_description: Option<BackupDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeContinuousBackupsInput {
    /// <p>Name of the table for which the customer wants to check the continuous backups and point in time recovery settings.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeContinuousBackupsOutput {
    /// <p> <code>ContinuousBackupsDescription</code> can be one of the following : ENABLED, DISABLED. </p>
    #[serde(rename = "ContinuousBackupsDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_backups_description: Option<ContinuousBackupsDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeGlobalTableInput {
    /// <p>The name of the global table.</p>
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeGlobalTableOutput {
    /// <p>Contains the details of the global table.</p>
    #[serde(rename = "GlobalTableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_description: Option<GlobalTableDescription>,
}

/// <p>Represents the input of a <code>DescribeLimits</code> operation. Has no content.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLimitsInput {}

/// <p>Represents the output of a <code>DescribeLimits</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeLimitsOutput {
    /// <p>The maximum total read capacity units that your account allows you to provision across all of your tables in this region.</p>
    #[serde(rename = "AccountMaxReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_max_read_capacity_units: Option<i64>,
    /// <p>The maximum total write capacity units that your account allows you to provision across all of your tables in this region.</p>
    #[serde(rename = "AccountMaxWriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_max_write_capacity_units: Option<i64>,
    /// <p>The maximum read capacity units that your account allows you to provision for a new table that you are creating in this region, including the read capacity units provisioned for its global secondary indexes (GSIs).</p>
    #[serde(rename = "TableMaxReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_max_read_capacity_units: Option<i64>,
    /// <p>The maximum write capacity units that your account allows you to provision for a new table that you are creating in this region, including the write capacity units provisioned for its global secondary indexes (GSIs).</p>
    #[serde(rename = "TableMaxWriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_max_write_capacity_units: Option<i64>,
}

/// <p>Represents the input of a <code>DescribeTable</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTableInput {
    /// <p>The name of the table to describe.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Represents the output of a <code>DescribeTable</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTableOutput {
    /// <p>The properties of the table.</p>
    #[serde(rename = "Table")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTimeToLiveInput {
    /// <p>The name of the table to be described.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTimeToLiveOutput {
    /// <p><p/></p>
    #[serde(rename = "TimeToLiveDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_description: Option<TimeToLiveDescription>,
}

/// <p>Represents a condition to be compared with an attribute value. This condition can be used with <code>DeleteItem</code>, <code>PutItem</code> or <code>UpdateItem</code> operations; if the comparison evaluates to true, the operation succeeds; if not, the operation fails. You can use <code>ExpectedAttributeValue</code> in one of two different ways:</p> <ul> <li> <p>Use <code>AttributeValueList</code> to specify one or more values to compare against an attribute. Use <code>ComparisonOperator</code> to specify how you want to perform the comparison. If the comparison evaluates to true, then the conditional operation succeeds.</p> </li> <li> <p>Use <code>Value</code> to specify a value that DynamoDB will compare against an attribute. If the values match, then <code>ExpectedAttributeValue</code> evaluates to true and the conditional operation succeeds. Optionally, you can also set <code>Exists</code> to false, indicating that you <i>do not</i> expect to find the attribute value in the table. In this case, the conditional operation succeeds only if the comparison evaluates to false.</p> </li> </ul> <p> <code>Value</code> and <code>Exists</code> are incompatible with <code>AttributeValueList</code> and <code>ComparisonOperator</code>. Note that if you use both sets of parameters at once, DynamoDB will return a <code>ValidationException</code> exception.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExpectedAttributeValue {
    /// <p>One or more values to evaluate against the supplied attribute. The number of values in the list depends on the <code>ComparisonOperator</code> being used.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href="http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p> <p>For information on specifying data types in JSON, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataFormat.html">JSON Data Format</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "AttributeValueList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value_list: Option<Vec<AttributeValue>>,
    /// <p><p>A comparator for evaluating attributes in the <code>AttributeValueList</code>. For example, equals, greater than, less than, etc.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT<em>NULL | NULL | CONTAINS | NOT</em>CONTAINS | BEGINS<em>WITH | IN | BETWEEN</code> </p> <p>The following are descriptions of each comparison operator.</p> <ul> <li> <p> <code>EQ</code> : Equal. <code>EQ</code> is supported for all data types, including lists and maps.</p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{&quot;S&quot;:&quot;6&quot;}</code> does not equal <code>{&quot;N&quot;:&quot;6&quot;}</code>. Also, <code>{&quot;N&quot;:&quot;6&quot;}</code> does not equal <code>{&quot;NS&quot;:[&quot;6&quot;, &quot;2&quot;, &quot;1&quot;]}</code>.</p> <p/> </li> <li> <p> <code>NE</code> : Not equal. <code>NE</code> is supported for all data types, including lists and maps.</p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <code>AttributeValue</code> of a different type than the one provided in the request, the value does not match. For example, <code>{&quot;S&quot;:&quot;6&quot;}</code> does not equal <code>{&quot;N&quot;:&quot;6&quot;}</code>. Also, <code>{&quot;N&quot;:&quot;6&quot;}</code> does not equal <code>{&quot;NS&quot;:[&quot;6&quot;, &quot;2&quot;, &quot;1&quot;]}</code>.</p> <p/> </li> <li> <p> <code>LE</code> : Less than or equal. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{&quot;S&quot;:&quot;6&quot;}</code> does not equal <code>{&quot;N&quot;:&quot;6&quot;}</code>. Also, <code>{&quot;N&quot;:&quot;6&quot;}</code> does not compare to <code>{&quot;NS&quot;:[&quot;6&quot;, &quot;2&quot;, &quot;1&quot;]}</code>.</p> <p/> </li> <li> <p> <code>LT</code> : Less than. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> of type String, Number, or Binary (not a set type). If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{&quot;S&quot;:&quot;6&quot;}</code> does not equal <code>{&quot;N&quot;:&quot;6&quot;}</code>. Also, <code>{&quot;N&quot;:&quot;6&quot;}</code> does not compare to <code>{&quot;NS&quot;:[&quot;6&quot;, &quot;2&quot;, &quot;1&quot;]}</code>.</p> <p/> </li> <li> <p> <code>GE</code> : Greater than or equal. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{&quot;S&quot;:&quot;6&quot;}</code> does not equal <code>{&quot;N&quot;:&quot;6&quot;}</code>. Also, <code>{&quot;N&quot;:&quot;6&quot;}</code> does not compare to <code>{&quot;NS&quot;:[&quot;6&quot;, &quot;2&quot;, &quot;1&quot;]}</code>.</p> <p/> </li> <li> <p> <code>GT</code> : Greater than. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{&quot;S&quot;:&quot;6&quot;}</code> does not equal <code>{&quot;N&quot;:&quot;6&quot;}</code>. Also, <code>{&quot;N&quot;:&quot;6&quot;}</code> does not compare to <code>{&quot;NS&quot;:[&quot;6&quot;, &quot;2&quot;, &quot;1&quot;]}</code>.</p> <p/> </li> <li> <p> <code>NOT</em>NULL</code> : The attribute exists. <code>NOT<em>NULL</code> is supported for all data types, including lists and maps.</p> <note> <p>This operator tests for the existence of an attribute, not its data type. If the data type of attribute &quot;<code>a</code>&quot; is null, and you evaluate it using <code>NOT</em>NULL</code>, the result is a Boolean <code>true</code>. This result is because the attribute &quot;<code>a</code>&quot; exists; its data type is not relevant to the <code>NOT<em>NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>NULL</code> : The attribute does not exist. <code>NULL</code> is supported for all data types, including lists and maps.</p> <note> <p>This operator tests for the nonexistence of an attribute, not its data type. If the data type of attribute &quot;<code>a</code>&quot; is null, and you evaluate it using <code>NULL</code>, the result is a Boolean <code>false</code>. This is because the attribute &quot;<code>a</code>&quot; exists; its data type is not relevant to the <code>NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>CONTAINS</code> : Checks for a subsequence, or value in a set.</p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is of type String, then the operator checks for a substring match. If the target attribute of the comparison is of type Binary, then the operator looks for a subsequence of the target that matches the input. If the target attribute of the comparison is a set (&quot;<code>SS</code>&quot;, &quot;<code>NS</code>&quot;, or &quot;<code>BS</code>&quot;), then the operator evaluates to true if it finds an exact match with any member of the set.</p> <p>CONTAINS is supported for lists: When evaluating &quot;<code>a CONTAINS b</code>&quot;, &quot;<code>a</code>&quot; can be a list; however, &quot;<code>b</code>&quot; cannot be a set, a map, or a list.</p> </li> <li> <p> <code>NOT</em>CONTAINS</code> : Checks for absence of a subsequence, or absence of a value in a set.</p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is a String, then the operator checks for the absence of a substring match. If the target attribute of the comparison is Binary, then the operator checks for the absence of a subsequence of the target that matches the input. If the target attribute of the comparison is a set (&quot;<code>SS</code>&quot;, &quot;<code>NS</code>&quot;, or &quot;<code>BS</code>&quot;), then the operator evaluates to true if it <i>does not</i> find an exact match with any member of the set.</p> <p>NOT<em>CONTAINS is supported for lists: When evaluating &quot;<code>a NOT CONTAINS b</code>&quot;, &quot;<code>a</code>&quot; can be a list; however, &quot;<code>b</code>&quot; cannot be a set, a map, or a list.</p> </li> <li> <p> <code>BEGINS</em>WITH</code> : Checks for a prefix. </p> <p> <code>AttributeValueList</code> can contain only one <code>AttributeValue</code> of type String or Binary (not a Number or a set type). The target attribute of the comparison must be of type String or Binary (not a Number or a set type).</p> <p/> </li> <li> <p> <code>IN</code> : Checks for matching elements in a list.</p> <p> <code>AttributeValueList</code> can contain one or more <code>AttributeValue</code> elements of type String, Number, or Binary. These attributes are compared against an existing attribute of an item. If any elements of the input are equal to the item attribute, the expression evaluates to true.</p> </li> <li> <p> <code>BETWEEN</code> : Greater than or equal to the first value, and less than or equal to the second value. </p> <p> <code>AttributeValueList</code> must contain two <code>AttributeValue</code> elements of the same type, either String, Number, or Binary (not a set type). A target attribute matches if the target value is greater than, or equal to, the first element and less than, or equal to, the second element. If an item contains an <code>AttributeValue</code> element of a different type than the one provided in the request, the value does not match. For example, <code>{&quot;S&quot;:&quot;6&quot;}</code> does not compare to <code>{&quot;N&quot;:&quot;6&quot;}</code>. Also, <code>{&quot;N&quot;:&quot;6&quot;}</code> does not compare to <code>{&quot;NS&quot;:[&quot;6&quot;, &quot;2&quot;, &quot;1&quot;]}</code> </p> </li> </ul></p>
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    /// <p><p>Causes DynamoDB to evaluate the value before attempting a conditional operation:</p> <ul> <li> <p>If <code>Exists</code> is <code>true</code>, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the operation succeeds. If it is not found, the operation fails with a <code>ConditionalCheckFailedException</code>.</p> </li> <li> <p>If <code>Exists</code> is <code>false</code>, DynamoDB assumes that the attribute value does not exist in the table. If in fact the value does not exist, then the assumption is valid and the operation succeeds. If the value is found, despite the assumption that it does not exist, the operation fails with a <code>ConditionalCheckFailedException</code>.</p> </li> </ul> <p>The default setting for <code>Exists</code> is <code>true</code>. If you supply a <code>Value</code> all by itself, DynamoDB assumes the attribute exists: You don&#39;t have to set <code>Exists</code> to <code>true</code>, because it is implied.</p> <p>DynamoDB returns a <code>ValidationException</code> if:</p> <ul> <li> <p> <code>Exists</code> is <code>true</code> but there is no <code>Value</code> to check. (You expect a value to exist, but don&#39;t specify what that value is.)</p> </li> <li> <p> <code>Exists</code> is <code>false</code> but you also provide a <code>Value</code>. (You cannot expect an attribute to have a value, while also expecting it not to exist.)</p> </li> </ul></p>
    #[serde(rename = "Exists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    /// <p>Represents the data for the expected attribute.</p> <p>Each attribute value is described as a name-value pair. The name is the data type, and the value is the data itself.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes">Data Types</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
}

/// <p>Represents the input of a <code>GetItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetItemInput {
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    /// <p>Determines the read consistency model: If set to <code>true</code>, then the operation uses strongly consistent reads; otherwise, the operation uses eventually consistent reads.</p>
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p> <code>{"#P":"Percentile"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<::std::collections::HashMap<String, String>>,
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to retrieve.</p> <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    #[serde(rename = "Key")]
    pub key: ::std::collections::HashMap<String, AttributeValue>,
    /// <p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    /// <p>The name of the table containing the requested item.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Represents the output of a <code>GetItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetItemOutput {
    /// <p>The capacity units consumed by the <code>GetItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, as specified by <code>ProjectionExpression</code>.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<::std::collections::HashMap<String, AttributeValue>>,
}

/// <p>Represents the properties of a global secondary index.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GlobalSecondaryIndex {
    /// <p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>
    #[serde(rename = "IndexName")]
    pub index_name: String,
    /// <p><p>The complete key schema for a global secondary index, which consists of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term &quot;hash attribute&quot; derives from DynamoDB&#39; usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term &quot;range attribute&quot; derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note></p>
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchemaElement>,
    /// <p>Represents attributes that are copied (projected) from the table into the global secondary index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. </p>
    #[serde(rename = "Projection")]
    pub projection: Projection,
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p> <p>For current minimum and maximum provisioned throughput values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
}

/// <p>Represents the properties of a global secondary index.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GlobalSecondaryIndexDescription {
    /// <p><p>Indicates whether the index is currently backfilling. <i>Backfilling</i> is the process of reading items from the table and determining whether they can be added to the index. (Not all items will qualify: For example, a partition key cannot have any duplicate values.) If an item can be added to the index, DynamoDB will do so. After all items have been processed, the backfilling operation is complete and <code>Backfilling</code> is false.</p> <note> <p>For indexes that were created during a <code>CreateTable</code> operation, the <code>Backfilling</code> attribute does not appear in the <code>DescribeTable</code> output.</p> </note></p>
    #[serde(rename = "Backfilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfilling: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the index.</p>
    #[serde(rename = "IndexArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    /// <p>The name of the global secondary index.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The total size of the specified index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    #[serde(rename = "IndexSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_size_bytes: Option<i64>,
    /// <p><p>The current state of the global secondary index:</p> <ul> <li> <p> <code>CREATING</code> - The index is being created.</p> </li> <li> <p> <code>UPDATING</code> - The index is being updated.</p> </li> <li> <p> <code>DELETING</code> - The index is being deleted.</p> </li> <li> <p> <code>ACTIVE</code> - The index is ready for use.</p> </li> </ul></p>
    #[serde(rename = "IndexStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    /// <p>The number of items in the specified index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    /// <p><p>The complete key schema for a global secondary index, which consists of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term &quot;hash attribute&quot; derives from DynamoDB&#39; usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term &quot;range attribute&quot; derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note></p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    /// <p>Represents attributes that are copied (projected) from the table into the global secondary index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. </p>
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p> <p>For current minimum and maximum provisioned throughput values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
}

/// <p>Represents the properties of a global secondary index for the table when the backup was created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GlobalSecondaryIndexInfo {
    /// <p>The name of the global secondary index.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p><p>The complete key schema for a global secondary index, which consists of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term &quot;hash attribute&quot; derives from DynamoDB&#39; usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term &quot;range attribute&quot; derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note></p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    /// <p>Represents attributes that are copied (projected) from the table into the global secondary index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. </p>
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
    /// <p>Represents the provisioned throughput settings for the specified global secondary index. </p>
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

/// <p><p>Represents one of the following:</p> <ul> <li> <p>A new global secondary index to be added to an existing table.</p> </li> <li> <p>New provisioned throughput parameters for an existing global secondary index.</p> </li> <li> <p>An existing global secondary index to be removed from an existing table.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GlobalSecondaryIndexUpdate {
    /// <p><p>The parameters required for creating a global secondary index on an existing table:</p> <ul> <li> <p> <code>IndexName </code> </p> </li> <li> <p> <code>KeySchema </code> </p> </li> <li> <p> <code>AttributeDefinitions </code> </p> </li> <li> <p> <code>Projection </code> </p> </li> <li> <p> <code>ProvisionedThroughput </code> </p> </li> </ul></p>
    #[serde(rename = "Create")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateGlobalSecondaryIndexAction>,
    /// <p>The name of an existing global secondary index to be removed.</p>
    #[serde(rename = "Delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteGlobalSecondaryIndexAction>,
    /// <p>The name of an existing global secondary index, along with new provisioned throughput settings to be applied to that index.</p>
    #[serde(rename = "Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateGlobalSecondaryIndexAction>,
}

/// <p>Represents the properties of a global table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GlobalTable {
    /// <p>The global table name.</p>
    #[serde(rename = "GlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<String>,
    /// <p>The regions where the global table has replicas.</p>
    #[serde(rename = "ReplicationGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<Vec<Replica>>,
}

/// <p>Contains details about the global table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GlobalTableDescription {
    /// <p>The creation time of the global table.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The unique identifier of the global table.</p>
    #[serde(rename = "GlobalTableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_arn: Option<String>,
    /// <p>The global table name.</p>
    #[serde(rename = "GlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_name: Option<String>,
    /// <p><p>The current state of the global table:</p> <ul> <li> <p> <code>CREATING</code> - The global table is being created.</p> </li> <li> <p> <code>UPDATING</code> - The global table is being updated.</p> </li> <li> <p> <code>DELETING</code> - The global table is being deleted.</p> </li> <li> <p> <code>ACTIVE</code> - The global table is ready for use.</p> </li> </ul></p>
    #[serde(rename = "GlobalTableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_status: Option<String>,
    /// <p>The regions where the global table has replicas.</p>
    #[serde(rename = "ReplicationGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<Vec<ReplicaDescription>>,
}

/// <p>Information about item collections, if any, that were affected by the operation. <code>ItemCollectionMetrics</code> is only returned if the request asked for it. If the table does not have any local secondary indexes, this information is not returned in the response.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ItemCollectionMetrics {
    /// <p>The partition key value of the item collection. This value is the same as the partition key value of the item.</p>
    #[serde(rename = "ItemCollectionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_key: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p>
    #[serde(rename = "SizeEstimateRangeGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_estimate_range_gb: Option<Vec<f64>>,
}

/// <p>Represents <i>a single element</i> of a key schema. A key schema specifies the attributes that make up the primary key of a table, or the key attributes of an index.</p> <p>A <code>KeySchemaElement</code> represents exactly one attribute of the primary key. For example, a simple primary key would be represented by one <code>KeySchemaElement</code> (for the partition key). A composite primary key would require one <code>KeySchemaElement</code> for the partition key, and another <code>KeySchemaElement</code> for the sort key.</p> <p>A <code>KeySchemaElement</code> must be a scalar, top-level attribute (not a nested attribute). The data type must be one of String, Number, or Binary. The attribute cannot be nested within a List or a Map.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeySchemaElement {
    /// <p>The name of a key attribute.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p><p>The role that this key attribute will assume:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term &quot;hash attribute&quot; derives from DynamoDB&#39; usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term &quot;range attribute&quot; derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note></p>
    #[serde(rename = "KeyType")]
    pub key_type: String,
}

/// <p>Represents a set of primary keys and, for each key, the attributes to retrieve from the table.</p> <p>For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key. For a composite primary key, you must provide <i>both</i> the partition key and the sort key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeysAndAttributes {
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.html">Legacy Conditional Parameters</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    /// <p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p> <code>{"#P":"Percentile"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<::std::collections::HashMap<String, String>>,
    /// <p>The primary key attribute values that define the items and the attributes associated with the items.</p>
    #[serde(rename = "Keys")]
    pub keys: Vec<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the <code>ProjectionExpression</code> must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListBackupsInput {
    /// <p> <code>LastEvaluatedBackupARN</code> returned by the previous ListBackups call. </p>
    #[serde(rename = "ExclusiveStartBackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_backup_arn: Option<String>,
    /// <p>Maximum number of backups to return at once.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The backups from the table specified by TableName are listed. </p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    #[serde(rename = "TimeRangeLowerBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_lower_bound: Option<f64>,
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive. </p>
    #[serde(rename = "TimeRangeUpperBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_upper_bound: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListBackupsOutput {
    /// <p>List of <code>BackupSummary</code> objects.</p>
    #[serde(rename = "BackupSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_summaries: Option<Vec<BackupSummary>>,
    /// <p>Last evaluated BackupARN.</p>
    #[serde(rename = "LastEvaluatedBackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_backup_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGlobalTablesInput {
    /// <p>The first global table name that this operation will evaluate.</p>
    #[serde(rename = "ExclusiveStartGlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_global_table_name: Option<String>,
    /// <p>The maximum number of table names to return.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Lists the global tables in a specific region.</p>
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListGlobalTablesOutput {
    /// <p>List of global table names.</p>
    #[serde(rename = "GlobalTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_tables: Option<Vec<GlobalTable>>,
    /// <p>Last evaluated global table name.</p>
    #[serde(rename = "LastEvaluatedGlobalTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_global_table_name: Option<String>,
}

/// <p>Represents the input of a <code>ListTables</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTablesInput {
    /// <p>The first table name that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedTableName</code> in a previous operation, so that you can obtain the next page of results.</p>
    #[serde(rename = "ExclusiveStartTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_table_name: Option<String>,
    /// <p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// <p>Represents the output of a <code>ListTables</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTablesOutput {
    /// <p>The name of the last table in the current page of results. Use this value as the <code>ExclusiveStartTableName</code> in a new request to obtain the next page of results, until all the table names are returned.</p> <p>If you do not receive a <code>LastEvaluatedTableName</code> value in the response, this means that there are no more table names to be retrieved.</p>
    #[serde(rename = "LastEvaluatedTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_table_name: Option<String>,
    /// <p>The names of the tables associated with the current account at the current endpoint. The maximum size of this array is 100.</p> <p>If <code>LastEvaluatedTableName</code> also appears in the output, you can use this value as the <code>ExclusiveStartTableName</code> parameter in a subsequent <code>ListTables</code> request and obtain the next page of results.</p>
    #[serde(rename = "TableNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsOfResourceInput {
    /// <p>An optional string that, if supplied, must be copied from the output of a previous call to ListTagOfResource. When provided in this manner, this API fetches the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon DynamoDB resource with tags to be listed. This value is an Amazon Resource Name (ARN).</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsOfResourceOutput {
    /// <p>If this value is returned, there are additional results to be displayed. To retrieve them, call ListTagsOfResource again, with NextToken set to this value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags currently associated with the Amazon DynamoDB resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents the properties of a local secondary index.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LocalSecondaryIndex {
    /// <p>The name of the local secondary index. The name must be unique among all other indexes on this table.</p>
    #[serde(rename = "IndexName")]
    pub index_name: String,
    /// <p><p>The complete key schema for the local secondary index, consisting of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term &quot;hash attribute&quot; derives from DynamoDB&#39; usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term &quot;range attribute&quot; derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note></p>
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchemaElement>,
    /// <p>Represents attributes that are copied (projected) from the table into the local secondary index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. </p>
    #[serde(rename = "Projection")]
    pub projection: Projection,
}

/// <p>Represents the properties of a local secondary index.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LocalSecondaryIndexDescription {
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the index.</p>
    #[serde(rename = "IndexArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    /// <p>Represents the name of the local secondary index.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The total size of the specified index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    #[serde(rename = "IndexSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_size_bytes: Option<i64>,
    /// <p>The number of items in the specified index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    /// <p><p>The complete key schema for the local secondary index, consisting of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term &quot;hash attribute&quot; derives from DynamoDB&#39; usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term &quot;range attribute&quot; derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note></p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    /// <p>Represents attributes that are copied (projected) from the table into the global secondary index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. </p>
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
}

/// <p>Represents the properties of a local secondary index for the table when the backup was created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LocalSecondaryIndexInfo {
    /// <p>Represents the name of the local secondary index.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p><p>The complete key schema for a local secondary index, which consists of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term &quot;hash attribute&quot; derives from DynamoDB&#39; usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term &quot;range attribute&quot; derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note></p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    /// <p>Represents attributes that are copied (projected) from the table into the global secondary index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. </p>
    #[serde(rename = "Projection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<Projection>,
}

/// <p>The description of the point in time settings applied to the table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PointInTimeRecoveryDescription {
    /// <p>Specifies the earliest point in time you can restore your table to. It is equal to the maximum of point in time recovery enabled time and <code>CurrentTime</code> - <code>PointInTimeRecoveryPeriod</code>.</p>
    #[serde(rename = "EarliestRestorableDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_restorable_date_time: Option<f64>,
    /// <p> <code>LatestRestorableDateTime</code> is 5 minutes from now and there is a +/- 1 minute fuzziness on the restore times. </p>
    #[serde(rename = "LatestRestorableDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_date_time: Option<f64>,
    /// <p><p>The current state of point in time recovery:</p> <ul> <li> <p> <code>ENABLING</code> - Point in time recovery is being enabled.</p> </li> <li> <p> <code>ENABLED</code> - Point in time recovery is enabled.</p> </li> <li> <p> <code>DISABLED</code> - Point in time recovery is disabled.</p> </li> </ul></p>
    #[serde(rename = "PointInTimeRecoveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_in_time_recovery_status: Option<String>,
}

/// <p>Represents the settings used to enable point in time recovery.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PointInTimeRecoverySpecification {
    /// <p>Indicates whether point in time recovery is enabled (true) or disabled (false) on the table.</p>
    #[serde(rename = "PointInTimeRecoveryEnabled")]
    pub point_in_time_recovery_enabled: bool,
}

/// <p>Represents attributes that are copied (projected) from the table into an index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Projection {
    /// <p>Represents the non-key attribute names which will be projected into the index.</p> <p>For local secondary indexes, the total count of <code>NonKeyAttributes</code> summed across all of the local secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p>
    #[serde(rename = "NonKeyAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_key_attributes: Option<Vec<String>>,
    /// <p><p>The set of attributes that are projected into the index:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <code>NonKeyAttributes</code>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul></p>
    #[serde(rename = "ProjectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_type: Option<String>,
}

/// <p>Represents the provisioned throughput settings for a specified table or index. The settings can be modified using the <code>UpdateTable</code> operation.</p> <p>For current minimum and maximum provisioned throughput values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedThroughput {
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: i64,
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: i64,
}

/// <p>Represents the provisioned throughput settings for the table, consisting of read and write capacity units, along with data about increases and decreases.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ProvisionedThroughputDescription {
    /// <p>The date and time of the last provisioned throughput decrease for this table.</p>
    #[serde(rename = "LastDecreaseDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_decrease_date_time: Option<f64>,
    /// <p>The date and time of the last provisioned throughput increase for this table.</p>
    #[serde(rename = "LastIncreaseDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_increase_date_time: Option<f64>,
    /// <p>The number of provisioned throughput decreases for this table during this UTC calendar day. For current maximums on provisioned throughput decreases, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "NumberOfDecreasesToday")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_decreases_today: Option<i64>,
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. Eventually consistent reads require less effort than strongly consistent reads, so a setting of 50 <code>ReadCapacityUnits</code> per second provides 100 eventually consistent <code>ReadCapacityUnits</code> per second.</p>
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i64>,
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>.</p>
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<i64>,
}

/// <p>Represents the input of a <code>PutItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutItemInput {
    /// <p>A condition that must be satisfied in order for a conditional <code>PutItem</code> operation to succeed.</p> <p>An expression can contain any of the following:</p> <ul> <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li> <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li> <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li> </ul> <p>For more information on condition expressions, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "Expected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<::std::collections::HashMap<String, ExpectedAttributeValue>>,
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p> <code>{"#P":"Percentile"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<::std::collections::HashMap<String, String>>,
    /// <p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>A map of attribute name/value pairs, one for each attribute. Only the primary key attributes are required; you can optionally provide other attribute name-value pairs for the item.</p> <p>You must provide all of the attributes for the primary key. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide both values for both the partition key and the sort key.</p> <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p> <p>For more information about primary keys, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html#DataModelPrimaryKey">Primary Key</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Each element in the <code>Item</code> map is an <code>AttributeValue</code> object.</p>
    #[serde(rename = "Item")]
    pub item: ::std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
    /// <p><p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were updated with the <code>PutItem</code> request. For <code>PutItem</code>, the valid values are:</p> <ul> <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li> <li> <p> <code>ALL<em>OLD</code> - If <code>PutItem</code> overwrote an attribute name-value pair, then the content of the old item is returned.</p> </li> </ul> <note> <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>PutItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL</em>OLD</code>.</p> </note></p>
    #[serde(rename = "ReturnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<String>,
    /// <p>The name of the table to contain the item.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Represents the output of a <code>PutItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutItemOutput {
    /// <p>The attribute values as they appeared before the <code>PutItem</code> operation, but only if <code>ReturnValues</code> is specified as <code>ALL_OLD</code> in the request. Each element consists of an attribute name and an attribute value.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The capacity units consumed by the <code>PutItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    /// <p><p>Information about item collections, if any, that were affected by the <code>PutItem</code> operation. <code>ItemCollectionMetrics</code> is only returned if the <code>ReturnItemCollectionMetrics</code> parameter was specified. If the table does not have any local secondary indexes, this information is not returned in the response.</p> <p>Each <code>ItemCollectionMetrics</code> element consists of:</p> <ul> <li> <p> <code>ItemCollectionKey</code> - The partition key value of the item collection. This is the same as the partition key value of the item itself.</p> </li> <li> <p> <code>SizeEstimateRangeGB</code> - An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li> </ul></p>
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

/// <p>Represents a request to perform a <code>PutItem</code> operation on an item.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutRequest {
    /// <p>A map of attribute name to attribute values, representing the primary key of an item to be processed by <code>PutItem</code>. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema. If any attributes are present in the item which are part of an index key schema for the table, their types must match the index key schema.</p>
    #[serde(rename = "Item")]
    pub item: ::std::collections::HashMap<String, AttributeValue>,
}

/// <p>Represents the input of a <code>Query</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct QueryInput {
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    /// <p>Determines the read consistency model: If set to <code>true</code>, then the operation uses strongly consistent reads; otherwise, the operation uses eventually consistent reads.</p> <p>Strongly consistent reads are not supported on global secondary indexes. If you query a global secondary index with <code>ConsistentRead</code> set to <code>true</code>, you will receive a <code>ValidationException</code>.</p>
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    /// <p>The primary key of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedKey</code> in the previous operation.</p> <p>The data type for <code>ExclusiveStartKey</code> must be String, Number or Binary. No set data types are allowed.</p>
    #[serde(rename = "ExclusiveStartKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_key: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p> <code>{"#P":"Percentile"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<::std::collections::HashMap<String, String>>,
    /// <p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>A string that contains conditions that DynamoDB applies after the <code>Query</code> operation, but before the data is returned to you. Items that do not satisfy the <code>FilterExpression</code> criteria are not returned.</p> <p>A <code>FilterExpression</code> does not allow key attributes. You cannot define a filter expression based on a partition key or a sort key.</p> <note> <p>A <code>FilterExpression</code> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#FilteringResults">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The name of an index to query. This index can be any local secondary index or global secondary index on the table. Note that if you use the <code>IndexName</code> parameter, you must also provide <code>TableName.</code> </p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The condition that specifies the key value(s) for items to be retrieved by the <code>Query</code> action.</p> <p>The condition must perform an equality test on a single partition key value.</p> <p>The condition can optionally perform one of several comparison tests on a single sort key value. This allows <code>Query</code> to retrieve one item with a given partition key value and sort key value, or several items that have the same partition key value but different sort key values.</p> <p>The partition key equality test is required, and must be specified in the following format:</p> <p> <code>partitionKeyName</code> <i>=</i> <code>:partitionkeyval</code> </p> <p>If you also want to provide a condition for the sort key, it must be combined using <code>AND</code> with the condition for the sort key. Following is an example, using the <b>=</b> comparison operator for the sort key:</p> <p> <code>partitionKeyName</code> <code>=</code> <code>:partitionkeyval</code> <code>AND</code> <code>sortKeyName</code> <code>=</code> <code>:sortkeyval</code> </p> <p>Valid comparisons for the sort key condition are as follows:</p> <ul> <li> <p> <code>sortKeyName</code> <code>=</code> <code>:sortkeyval</code> - true if the sort key value is equal to <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <code>&lt;</code> <code>:sortkeyval</code> - true if the sort key value is less than <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <code>&lt;=</code> <code>:sortkeyval</code> - true if the sort key value is less than or equal to <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <code>&gt;</code> <code>:sortkeyval</code> - true if the sort key value is greater than <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <code>&gt;= </code> <code>:sortkeyval</code> - true if the sort key value is greater than or equal to <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <code>BETWEEN</code> <code>:sortkeyval1</code> <code>AND</code> <code>:sortkeyval2</code> - true if the sort key value is greater than or equal to <code>:sortkeyval1</code>, and less than or equal to <code>:sortkeyval2</code>.</p> </li> <li> <p> <code>begins_with (</code> <code>sortKeyName</code>, <code>:sortkeyval</code> <code>)</code> - true if the sort key value begins with a particular operand. (You cannot use this function with a sort key that is of type Number.) Note that the function name <code>begins_with</code> is case-sensitive.</p> </li> </ul> <p>Use the <code>ExpressionAttributeValues</code> parameter to replace tokens such as <code>:partitionval</code> and <code>:sortval</code> with actual values at runtime.</p> <p>You can optionally use the <code>ExpressionAttributeNames</code> parameter to replace the names of the partition key and sort key with placeholder tokens. This option might be necessary if an attribute name conflicts with a DynamoDB reserved word. For example, the following <code>KeyConditionExpression</code> parameter causes an error because <i>Size</i> is a reserved word:</p> <ul> <li> <p> <code>Size = :myval</code> </p> </li> </ul> <p>To work around this, define a placeholder (such a <code>#S</code>) to represent the attribute name <i>Size</i>. <code>KeyConditionExpression</code> then is as follows:</p> <ul> <li> <p> <code>#S = :myval</code> </p> </li> </ul> <p>For a list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>For more information on <code>ExpressionAttributeNames</code> and <code>ExpressionAttributeValues</code>, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ExpressionPlaceholders.html">Using Placeholders for Attribute Names and Values</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "KeyConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_condition_expression: Option<String>,
    /// <p>This is a legacy parameter. Use <code>KeyConditionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.KeyConditions.html">KeyConditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "KeyConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_conditions: Option<::std::collections::HashMap<String, Condition>>,
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation, so that you can pick up where you left off. Also, if the processed data set size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html">Query and Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
    /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.QueryFilter.html">QueryFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "QueryFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_filter: Option<::std::collections::HashMap<String, Condition>>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    /// <p>Specifies the order for index traversal: If <code>true</code> (default), the traversal is performed in ascending order; if <code>false</code>, the traversal is performed in descending order. </p> <p>Items with the same partition key value are stored in sorted order by sort key. If the sort key data type is Number, the results are stored in numeric order. For type String, the results are stored in order of UTF-8 bytes. For type Binary, DynamoDB treats each byte of the binary data as unsigned.</p> <p>If <code>ScanIndexForward</code> is <code>true</code>, DynamoDB returns the results in the order in which they are stored (by sort key value). This is the default behavior. If <code>ScanIndexForward</code> is <code>false</code>, DynamoDB reads the results in reverse order by sort key value, and then returns the results to the client.</p>
    #[serde(rename = "ScanIndexForward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_index_forward: Option<bool>,
    /// <p><p>The attributes to be returned in the result. You can retrieve all item attributes, specific item attributes, the count of matching items, or in the case of an index, some or all of the attributes projected into the index.</p> <ul> <li> <p> <code>ALL<em>ATTRIBUTES</code> - Returns all of the item attributes from the specified table or index. If you query a local secondary index, then for each matching item in the index DynamoDB will fetch the entire item from the parent table. If the index is configured to project all item attributes, then all of the data can be obtained from the local secondary index, and no fetching is required.</p> </li> <li> <p> <code>ALL</em>PROJECTED<em>ATTRIBUTES</code> - Allowed only when querying an index. Retrieves all attributes that have been projected into the index. If the index is configured to project all attributes, this return value is equivalent to specifying <code>ALL</em>ATTRIBUTES</code>.</p> </li> <li> <p> <code>COUNT</code> - Returns the number of matching items, rather than the matching items themselves.</p> </li> <li> <p> <code>SPECIFIC<em>ATTRIBUTES</code> - Returns only the attributes listed in <code>AttributesToGet</code>. This return value is equivalent to specifying <code>AttributesToGet</code> without specifying any value for <code>Select</code>.</p> <p>If you query or scan a local secondary index and request only attributes that are projected into that index, the operation will read only the index and not the table. If any of the requested attributes are not projected into the local secondary index, DynamoDB will fetch each of these attributes from the parent table. This extra fetching incurs additional throughput cost and latency.</p> <p>If you query or scan a global secondary index, you can only request attributes that are projected into the index. Global secondary index queries cannot fetch attributes from the parent table.</p> </li> </ul> <p>If neither <code>Select</code> nor <code>AttributesToGet</code> are specified, DynamoDB defaults to <code>ALL</em>ATTRIBUTES</code> when accessing a table, and <code>ALL<em>PROJECTED</em>ATTRIBUTES</code> when accessing an index. You cannot use both <code>Select</code> and <code>AttributesToGet</code> together in a single request, unless the value for <code>Select</code> is <code>SPECIFIC<em>ATTRIBUTES</code>. (This usage is equivalent to specifying <code>AttributesToGet</code> without any value for <code>Select</code>.)</p> <note> <p>If you use the <code>ProjectionExpression</code> parameter, then the value for <code>Select</code> can only be <code>SPECIFIC</em>ATTRIBUTES</code>. Any other value for <code>Select</code> will return an error.</p> </note></p>
    #[serde(rename = "Select")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
    /// <p>The name of the table containing the requested items.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Represents the output of a <code>Query</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct QueryOutput {
    /// <p>The capacity units consumed by the <code>Query</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    /// <p>The number of items in the response.</p> <p>If you used a <code>QueryFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p> <p>If you did not use a filter in the request, then <code>Count</code> and <code>ScannedCount</code> are the same.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>An array of item attributes that match the query criteria. Each element in this array consists of an attribute name and the value for that attribute.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<::std::collections::HashMap<String, AttributeValue>>>,
    /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p> <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
    #[serde(rename = "LastEvaluatedKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The number of items evaluated, before any <code>QueryFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Query</code> operation. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
    #[serde(rename = "ScannedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_count: Option<i64>,
}

/// <p>Represents the properties of a replica.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Replica {
    /// <p>The region where the replica needs to be created.</p>
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

/// <p>Contains the details of the replica.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReplicaDescription {
    /// <p>The name of the region.</p>
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

/// <p><p>Represents one of the following:</p> <ul> <li> <p>A new replica to be added to an existing global table.</p> </li> <li> <p>New parameters for an existing replica.</p> </li> <li> <p>An existing replica to be removed from an existing global table.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReplicaUpdate {
    /// <p>The parameters required for creating a replica on an existing global table.</p>
    #[serde(rename = "Create")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<CreateReplicaAction>,
    /// <p>The name of the existing replica to be removed.</p>
    #[serde(rename = "Delete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteReplicaAction>,
}

/// <p>Contains details for the restore.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RestoreSummary {
    /// <p>Point in time or source backup time.</p>
    #[serde(rename = "RestoreDateTime")]
    pub restore_date_time: f64,
    /// <p>Indicates if a restore is in progress or not.</p>
    #[serde(rename = "RestoreInProgress")]
    pub restore_in_progress: bool,
    /// <p>ARN of the backup from which the table was restored.</p>
    #[serde(rename = "SourceBackupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_arn: Option<String>,
    /// <p>ARN of the source table of the backup that is being restored.</p>
    #[serde(rename = "SourceTableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RestoreTableFromBackupInput {
    /// <p>The ARN associated with the backup.</p>
    #[serde(rename = "BackupArn")]
    pub backup_arn: String,
    /// <p>The name of the new table to which the backup must be restored.</p>
    #[serde(rename = "TargetTableName")]
    pub target_table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RestoreTableFromBackupOutput {
    /// <p>The description of the table created from an existing backup.</p>
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RestoreTableToPointInTimeInput {
    /// <p>Time in the past to restore the table to.</p>
    #[serde(rename = "RestoreDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date_time: Option<f64>,
    /// <p>Name of the source table that is being restored.</p>
    #[serde(rename = "SourceTableName")]
    pub source_table_name: String,
    /// <p>The name of the new table to which it must be restored to.</p>
    #[serde(rename = "TargetTableName")]
    pub target_table_name: String,
    /// <p>Restore the table to the latest possible time. <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. </p>
    #[serde(rename = "UseLatestRestorableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RestoreTableToPointInTimeOutput {
    /// <p>Represents the properties of a table.</p>
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

/// <p>The description of the server-side encryption status on the specified table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SSEDescription {
    /// <p><p>The current state of server-side encryption:</p> <ul> <li> <p> <code>ENABLING</code> - Server-side encryption is being enabled.</p> </li> <li> <p> <code>ENABLED</code> - Server-side encryption is enabled.</p> </li> <li> <p> <code>DISABLING</code> - Server-side encryption is being disabled.</p> </li> <li> <p> <code>DISABLED</code> - Server-side encryption is disabled.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the settings used to enable server-side encryption.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SSESpecification {
    /// <p>Indicates whether server-side encryption is enabled (true) or disabled (false) on the table.</p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

/// <p>Represents the input of a <code>Scan</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScanInput {
    /// <p>This is a legacy parameter. Use <code>ProjectionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributesToGet.html">AttributesToGet</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "AttributesToGet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_get: Option<Vec<String>>,
    /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    /// <p>A Boolean value that determines the read consistency model during the scan:</p> <ul> <li> <p>If <code>ConsistentRead</code> is <code>false</code>, then the data returned from <code>Scan</code> might not contain the results from other recently completed write operations (PutItem, UpdateItem or DeleteItem).</p> </li> <li> <p>If <code>ConsistentRead</code> is <code>true</code>, then all of the write operations that completed before the <code>Scan</code> began are guaranteed to be contained in the <code>Scan</code> response.</p> </li> </ul> <p>The default setting for <code>ConsistentRead</code> is <code>false</code>.</p> <p>The <code>ConsistentRead</code> parameter is not supported on global secondary indexes. If you scan a global secondary index with <code>ConsistentRead</code> set to true, you will receive a <code>ValidationException</code>.</p>
    #[serde(rename = "ConsistentRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistent_read: Option<bool>,
    /// <p>The primary key of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedKey</code> in the previous operation.</p> <p>The data type for <code>ExclusiveStartKey</code> must be String, Number or Binary. No set data types are allowed.</p> <p>In a parallel scan, a <code>Scan</code> request that includes <code>ExclusiveStartKey</code> must specify the same segment whose previous <code>Scan</code> returned the corresponding value of <code>LastEvaluatedKey</code>.</p>
    #[serde(rename = "ExclusiveStartKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_key: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p> <code>{"#P":"Percentile"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<::std::collections::HashMap<String, String>>,
    /// <p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>A string that contains conditions that DynamoDB applies after the <code>Scan</code> operation, but before the data is returned to you. Items that do not satisfy the <code>FilterExpression</code> criteria are not returned.</p> <note> <p>A <code>FilterExpression</code> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#FilteringResults">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>The name of a secondary index to scan. This index can be any local secondary index or global secondary index. Note that if you use the <code>IndexName</code> parameter, you must also provide <code>TableName</code>.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation, so that you can pick up where you left off. Also, if the processed data set size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html">Query and Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A string that identifies one or more attributes to retrieve from the specified table or index. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProjectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_expression: Option<String>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    /// <p>This is a legacy parameter. Use <code>FilterExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ScanFilter.html">ScanFilter</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ScanFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_filter: Option<::std::collections::HashMap<String, Condition>>,
    /// <p>For a parallel <code>Scan</code> request, <code>Segment</code> identifies an individual segment to be scanned by an application worker.</p> <p>Segment IDs are zero-based, so the first segment is always 0. For example, if you want to use four application threads to scan a table or an index, then the first thread specifies a <code>Segment</code> value of 0, the second thread specifies 1, and so on.</p> <p>The value of <code>LastEvaluatedKey</code> returned from a parallel <code>Scan</code> request must be used as <code>ExclusiveStartKey</code> with the same segment ID in a subsequent <code>Scan</code> operation.</p> <p>The value for <code>Segment</code> must be greater than or equal to 0, and less than the value provided for <code>TotalSegments</code>.</p> <p>If you provide <code>Segment</code>, you must also provide <code>TotalSegments</code>.</p>
    #[serde(rename = "Segment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<i64>,
    /// <p><p>The attributes to be returned in the result. You can retrieve all item attributes, specific item attributes, the count of matching items, or in the case of an index, some or all of the attributes projected into the index.</p> <ul> <li> <p> <code>ALL<em>ATTRIBUTES</code> - Returns all of the item attributes from the specified table or index. If you query a local secondary index, then for each matching item in the index DynamoDB will fetch the entire item from the parent table. If the index is configured to project all item attributes, then all of the data can be obtained from the local secondary index, and no fetching is required.</p> </li> <li> <p> <code>ALL</em>PROJECTED<em>ATTRIBUTES</code> - Allowed only when querying an index. Retrieves all attributes that have been projected into the index. If the index is configured to project all attributes, this return value is equivalent to specifying <code>ALL</em>ATTRIBUTES</code>.</p> </li> <li> <p> <code>COUNT</code> - Returns the number of matching items, rather than the matching items themselves.</p> </li> <li> <p> <code>SPECIFIC<em>ATTRIBUTES</code> - Returns only the attributes listed in <code>AttributesToGet</code>. This return value is equivalent to specifying <code>AttributesToGet</code> without specifying any value for <code>Select</code>.</p> <p>If you query or scan a local secondary index and request only attributes that are projected into that index, the operation will read only the index and not the table. If any of the requested attributes are not projected into the local secondary index, DynamoDB will fetch each of these attributes from the parent table. This extra fetching incurs additional throughput cost and latency.</p> <p>If you query or scan a global secondary index, you can only request attributes that are projected into the index. Global secondary index queries cannot fetch attributes from the parent table.</p> </li> </ul> <p>If neither <code>Select</code> nor <code>AttributesToGet</code> are specified, DynamoDB defaults to <code>ALL</em>ATTRIBUTES</code> when accessing a table, and <code>ALL<em>PROJECTED</em>ATTRIBUTES</code> when accessing an index. You cannot use both <code>Select</code> and <code>AttributesToGet</code> together in a single request, unless the value for <code>Select</code> is <code>SPECIFIC<em>ATTRIBUTES</code>. (This usage is equivalent to specifying <code>AttributesToGet</code> without any value for <code>Select</code>.)</p> <note> <p>If you use the <code>ProjectionExpression</code> parameter, then the value for <code>Select</code> can only be <code>SPECIFIC</em>ATTRIBUTES</code>. Any other value for <code>Select</code> will return an error.</p> </note></p>
    #[serde(rename = "Select")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
    /// <p>The name of the table containing the requested items; or, if you provide <code>IndexName</code>, the name of the table to which that index belongs.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>For a parallel <code>Scan</code> request, <code>TotalSegments</code> represents the total number of segments into which the <code>Scan</code> operation will be divided. The value of <code>TotalSegments</code> corresponds to the number of application workers that will perform the parallel scan. For example, if you want to use four application threads to scan a table or an index, specify a <code>TotalSegments</code> value of 4.</p> <p>The value for <code>TotalSegments</code> must be greater than or equal to 1, and less than or equal to 1000000. If you specify a <code>TotalSegments</code> value of 1, the <code>Scan</code> operation will be sequential rather than parallel.</p> <p>If you specify <code>TotalSegments</code>, you must also specify <code>Segment</code>.</p>
    #[serde(rename = "TotalSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_segments: Option<i64>,
}

/// <p>Represents the output of a <code>Scan</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScanOutput {
    /// <p>The capacity units consumed by the <code>Scan</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    /// <p>The number of items in the response.</p> <p>If you set <code>ScanFilter</code> in the request, then <code>Count</code> is the number of items returned after the filter was applied, and <code>ScannedCount</code> is the number of matching items before the filter was applied.</p> <p>If you did not use a filter in the request, then <code>Count</code> is the same as <code>ScannedCount</code>.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>An array of item attributes that match the scan criteria. Each element in this array consists of an attribute name and the value for that attribute.</p>
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<::std::collections::HashMap<String, AttributeValue>>>,
    /// <p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <code>LastEvaluatedKey</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p> <p>If <code>LastEvaluatedKey</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedKey</code> is empty.</p>
    #[serde(rename = "LastEvaluatedKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The number of items evaluated, before any <code>ScanFilter</code> is applied. A high <code>ScannedCount</code> value with few, or no, <code>Count</code> results indicates an inefficient <code>Scan</code> operation. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>If you did not use a filter in the request, then <code>ScannedCount</code> is the same as <code>Count</code>.</p>
    #[serde(rename = "ScannedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_count: Option<i64>,
}

/// <p>Contains the details of the table when the backup was created. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SourceTableDetails {
    /// <p>Number of items in the table. Please note this is an approximate value. </p>
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    /// <p>Schema of the table. </p>
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchemaElement>,
    /// <p>Read IOPs and Write IOPS on the table when the backup was created.</p>
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
    /// <p>ARN of the table for which backup was created. </p>
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    /// <p>Time when the source table was created. </p>
    #[serde(rename = "TableCreationDateTime")]
    pub table_creation_date_time: f64,
    /// <p>Unique identifier for the table for which the backup was created. </p>
    #[serde(rename = "TableId")]
    pub table_id: String,
    /// <p>The name of the table for which the backup was created. </p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>Size of the table in bytes. Please note this is an approximate value.</p>
    #[serde(rename = "TableSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<i64>,
}

/// <p>Contains the details of the features enabled on the table when the backup was created. For example, LSIs, GSIs, streams, TTL. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SourceTableFeatureDetails {
    /// <p>Represents the GSI properties for the table when the backup was created. It includes the IndexName, KeySchema, Projection and ProvisionedThroughput for the GSIs on the table at the time of backup. </p>
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndexInfo>>,
    /// <p>Represents the LSI properties for the table when the backup was created. It includes the IndexName, KeySchema and Projection for the LSIs on the table at the time of backup. </p>
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndexInfo>>,
    /// <p>The description of the server-side encryption status on the table when the backup was created.</p>
    #[serde(rename = "SSEDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_description: Option<SSEDescription>,
    /// <p>Stream settings on the table when the backup was created.</p>
    #[serde(rename = "StreamDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_description: Option<StreamSpecification>,
    /// <p>Time to Live settings on the table when the backup was created.</p>
    #[serde(rename = "TimeToLiveDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_description: Option<TimeToLiveDescription>,
}

/// <p>Represents the DynamoDB Streams configuration for a table in DynamoDB.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamSpecification {
    /// <p>Indicates whether DynamoDB Streams is enabled (true) or disabled (false) on the table.</p>
    #[serde(rename = "StreamEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_enabled: Option<bool>,
    /// <p><p> When an item in the table is modified, <code>StreamViewType</code> determines what information is written to the stream for this table. Valid values for <code>StreamViewType</code> are:</p> <ul> <li> <p> <code>KEYS<em>ONLY</code> - Only the key attributes of the modified item are written to the stream.</p> </li> <li> <p> <code>NEW</em>IMAGE</code> - The entire item, as it appears after it was modified, is written to the stream.</p> </li> <li> <p> <code>OLD<em>IMAGE</code> - The entire item, as it appeared before it was modified, is written to the stream.</p> </li> <li> <p> <code>NEW</em>AND<em>OLD</em>IMAGES</code> - Both the new and the old item images of the item are written to the stream.</p> </li> </ul></p>
    #[serde(rename = "StreamViewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
}

/// <p>Represents the properties of a table.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TableDescription {
    /// <p><p>An array of <code>AttributeDefinition</code> objects. Each of these objects describes one attribute in the table and index key schema.</p> <p>Each <code>AttributeDefinition</code> object in this array is composed of:</p> <ul> <li> <p> <code>AttributeName</code> - The name of the attribute.</p> </li> <li> <p> <code>AttributeType</code> - The data type for the attribute.</p> </li> </ul></p>
    #[serde(rename = "AttributeDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<Vec<AttributeDefinition>>,
    /// <p>The date and time when the table was created, in <a href="http://www.epochconverter.com/">UNIX epoch time</a> format.</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The global secondary indexes, if any, on the table. Each index is scoped to a given partition key value. Each element is composed of:</p> <ul> <li> <p> <code>Backfilling</code> - If true, then the index is currently in the backfilling phase. Backfilling occurs only when a new global secondary index is added to the table; it is the process by which DynamoDB populates the new index with data from the table. (This attribute does not appear for indexes that were created during a <code>CreateTable</code> operation.)</p> </li> <li> <p> <code>IndexName</code> - The name of the global secondary index.</p> </li> <li> <p> <code>IndexSizeBytes</code> - The total size of the global secondary index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value. </p> </li> <li> <p> <code>IndexStatus</code> - The current status of the global secondary index:</p> <ul> <li> <p> <code>CREATING</code> - The index is being created.</p> </li> <li> <p> <code>UPDATING</code> - The index is being updated.</p> </li> <li> <p> <code>DELETING</code> - The index is being deleted.</p> </li> <li> <p> <code>ACTIVE</code> - The index is ready for use.</p> </li> </ul> </li> <li> <p> <code>ItemCount</code> - The number of items in the global secondary index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value. </p> </li> <li> <p> <code>KeySchema</code> - Specifies the complete index key schema. The attribute names in the key schema must be between 1 and 255 characters (inclusive). The key schema must begin with the same partition key as the table.</p> </li> <li> <p> <code>Projection</code> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p> <ul> <li> <p> <code>ProjectionType</code> - One of the following:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <code>NonKeyAttributes</code>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul> </li> <li> <p> <code>NonKeyAttributes</code> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <code>NonKeyAttributes</code>, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p> </li> </ul> </li> <li> <p> <code>ProvisionedThroughput</code> - The provisioned throughput settings for the global secondary index, consisting of read and write capacity units, along with data about increases and decreases. </p> </li> </ul> <p>If the table is in the <code>DELETING</code> state, no information about indexes will be returned.</p>
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndexDescription>>,
    /// <p>The number of items in the specified table. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    #[serde(rename = "ItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    /// <p>The primary key structure for the table. Each <code>KeySchemaElement</code> consists of:</p> <ul> <li> <p> <code>AttributeName</code> - The name of the attribute.</p> </li> <li> <p> <code>KeyType</code> - The role of the attribute:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term "hash attribute" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term "range attribute" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note> </li> </ul> <p>For more information about primary keys, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html#DataModelPrimaryKey">Primary Key</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "KeySchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the latest stream for this table.</p>
    #[serde(rename = "LatestStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_arn: Option<String>,
    /// <p><p>A timestamp, in ISO 8601 format, for this stream.</p> <p>Note that <code>LatestStreamLabel</code> is not a unique identifier for the stream, because it is possible that a stream from another table might have the same timestamp. However, the combination of the following three elements is guaranteed to be unique:</p> <ul> <li> <p>the AWS customer ID.</p> </li> <li> <p>the table name.</p> </li> <li> <p>the <code>StreamLabel</code>.</p> </li> </ul></p>
    #[serde(rename = "LatestStreamLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_label: Option<String>,
    /// <p>Represents one or more local secondary indexes on the table. Each index is scoped to a given partition key value. Tables with one or more local secondary indexes are subject to an item collection size limit, where the amount of data within a given item collection cannot exceed 10 GB. Each element is composed of:</p> <ul> <li> <p> <code>IndexName</code> - The name of the local secondary index.</p> </li> <li> <p> <code>KeySchema</code> - Specifies the complete index key schema. The attribute names in the key schema must be between 1 and 255 characters (inclusive). The key schema must begin with the same partition key as the table.</p> </li> <li> <p> <code>Projection</code> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p> <ul> <li> <p> <code>ProjectionType</code> - One of the following:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <code>NonKeyAttributes</code>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul> </li> <li> <p> <code>NonKeyAttributes</code> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <code>NonKeyAttributes</code>, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p> </li> </ul> </li> <li> <p> <code>IndexSizeBytes</code> - Represents the total size of the index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p> </li> <li> <p> <code>ItemCount</code> - Represents the number of items in the index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p> </li> </ul> <p>If the table is in the <code>DELETING</code> state, no information about indexes will be returned.</p>
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndexDescription>>,
    /// <p>The provisioned throughput settings for the table, consisting of read and write capacity units, along with data about increases and decreases.</p>
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
    /// <p>Contains details for the restore.</p>
    #[serde(rename = "RestoreSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_summary: Option<RestoreSummary>,
    /// <p>The description of the server-side encryption status on the specified table.</p>
    #[serde(rename = "SSEDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_description: Option<SSEDescription>,
    /// <p>The current DynamoDB Streams configuration for the table.</p>
    #[serde(rename = "StreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the table.</p>
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    /// <p>Unique identifier for the table for which the backup was created. </p>
    #[serde(rename = "TableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// <p>The name of the table.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The total size of the specified table, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>
    #[serde(rename = "TableSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<i64>,
    /// <p><p>The current state of the table:</p> <ul> <li> <p> <code>CREATING</code> - The table is being created.</p> </li> <li> <p> <code>UPDATING</code> - The table is being updated.</p> </li> <li> <p> <code>DELETING</code> - The table is being deleted.</p> </li> <li> <p> <code>ACTIVE</code> - The table is ready for use.</p> </li> </ul></p>
    #[serde(rename = "TableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,
}

/// <p>Describes a tag. A tag is a key-value pair. You can add up to 50 tags to a single DynamoDB table. </p> <p> AWS-assigned tag names and values are automatically assigned the aws: prefix, which the user cannot assign. AWS-assigned tag names do not count towards the tag limit of 50. User-assigned tag names have the prefix user: in the Cost Allocation Report. You cannot backdate the application of a tag. </p> <p>For an overview on tagging DynamoDB resources, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html">Tagging for DynamoDB</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.Tag keys are case sensitive. Each DynamoDB table can only have up to one tag with the same key. If you try to add an existing tag (same key), the existing tag value will be updated to the new value. </p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag. Tag values are case-sensitive and can be null.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceInput {
    /// <p>Identifies the Amazon DynamoDB resource to which tags should be added. This value is an Amazon Resource Name (ARN).</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tags to be assigned to the Amazon DynamoDB resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>The description of the Time to Live (TTL) status on the specified table. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TimeToLiveDescription {
    /// <p> The name of the Time to Live attribute for items in the table.</p>
    #[serde(rename = "AttributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p> The Time to Live status for the table.</p>
    #[serde(rename = "TimeToLiveStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_status: Option<String>,
}

/// <p>Represents the settings used to enable or disable Time to Live for the specified table.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeToLiveSpecification {
    /// <p>The name of the Time to Live attribute used to store the expiration time for items in the table.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>Indicates whether Time To Live is to be enabled (true) or disabled (false) on the table.</p>
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceInput {
    /// <p>The Amazon DyanamoDB resource the tags will be removed from. This value is an Amazon Resource Name (ARN).</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>A list of tag keys. Existing tags of the resource whose keys are members of this list will be removed from the Amazon DynamoDB resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateContinuousBackupsInput {
    /// <p>Represents the settings used to enable point in time recovery.</p>
    #[serde(rename = "PointInTimeRecoverySpecification")]
    pub point_in_time_recovery_specification: PointInTimeRecoverySpecification,
    /// <p>The name of the table.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateContinuousBackupsOutput {
    /// <p>Represents the continuous backups and point in time recovery settings on the table.</p>
    #[serde(rename = "ContinuousBackupsDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_backups_description: Option<ContinuousBackupsDescription>,
}

/// <p>Represents the new provisioned throughput settings to be applied to a global secondary index.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGlobalSecondaryIndexAction {
    /// <p>The name of the global secondary index to be updated.</p>
    #[serde(rename = "IndexName")]
    pub index_name: String,
    /// <p>Represents the provisioned throughput settings for the specified global secondary index.</p> <p>For current minimum and maximum provisioned throughput values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGlobalTableInput {
    /// <p>The global table name.</p>
    #[serde(rename = "GlobalTableName")]
    pub global_table_name: String,
    /// <p>A list of regions that should be added or removed from the global table.</p>
    #[serde(rename = "ReplicaUpdates")]
    pub replica_updates: Vec<ReplicaUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateGlobalTableOutput {
    /// <p>Contains the details of the global table.</p>
    #[serde(rename = "GlobalTableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_description: Option<GlobalTableDescription>,
}

/// <p>Represents the input of an <code>UpdateItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateItemInput {
    /// <p>This is a legacy parameter. Use <code>UpdateExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.AttributeUpdates.html">AttributeUpdates</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "AttributeUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_updates: Option<::std::collections::HashMap<String, AttributeValueUpdate>>,
    /// <p>A condition that must be satisfied in order for a conditional update to succeed.</p> <p>An expression can contain any of the following:</p> <ul> <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li> <li> <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code> </p> </li> <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li> </ul> <p>For more information on condition expressions, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConditionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConditionalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_operator: Option<String>,
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "Expected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<::std::collections::HashMap<String, ExpectedAttributeValue>>,
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p> <ul> <li> <p> <code>{"#P":"Percentile"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_names: Option<::std::collections::HashMap<String, String>>,
    /// <p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p> <p> <code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ExpressionAttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_attribute_values: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p> <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    #[serde(rename = "Key")]
    pub key: ::std::collections::HashMap<String, AttributeValue>,
    #[serde(rename = "ReturnConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_consumed_capacity: Option<String>,
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    #[serde(rename = "ReturnItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_item_collection_metrics: Option<String>,
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appear before or after they are updated. For <code>UpdateItem</code>, the valid values are:</p> <ul> <li> <p> <code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p> </li> <li> <p> <code>ALL_OLD</code> - Returns all of the attributes of the item, as they appeared before the UpdateItem operation.</p> </li> <li> <p> <code>UPDATED_OLD</code> - Returns only the updated attributes, as they appeared before the UpdateItem operation.</p> </li> <li> <p> <code>ALL_NEW</code> - Returns all of the attributes of the item, as they appear after the UpdateItem operation.</p> </li> <li> <p> <code>UPDATED_NEW</code> - Returns only the updated attributes, as they appear after the UpdateItem operation.</p> </li> </ul> <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p> <p>The values returned are strongly consistent.</p>
    #[serde(rename = "ReturnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<String>,
    /// <p>The name of the table containing the item to update.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new value(s) for them.</p> <p>The following action values are available for <code>UpdateExpression</code>.</p> <ul> <li> <p> <code>SET</code> - Adds one or more attributes and values to an item. If any of these attribute already exist, they are replaced by the new values. You can also use <code>SET</code> to add or subtract from an attribute that is of type Number. For example: <code>SET myNum = myNum + :val</code> </p> <p> <code>SET</code> supports the following functions:</p> <ul> <li> <p> <code>if_not_exists (path, operand)</code> - if the item does not contain an attribute at the specified path, then <code>if_not_exists</code> evaluates to operand; otherwise, it evaluates to path. You can use this function to avoid overwriting an attribute that may already be present in the item.</p> </li> <li> <p> <code>list_append (operand, operand)</code> - evaluates to a list with a new element added to it. You can append the new element to the start or the end of the list by reversing the order of the operands.</p> </li> </ul> <p>These function names are case-sensitive.</p> </li> <li> <p> <code>REMOVE</code> - Removes one or more attributes from an item.</p> </li> <li> <p> <code>ADD</code> - Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p> <ul> <li> <p>If the existing attribute is a number, and if <code>Value</code> is also a number, then <code>Value</code> is mathematically added to the existing attribute. If <code>Value</code> is a negative number, then it is subtracted from the existing attribute.</p> <note> <p>If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value.</p> <p>Similarly, if you use <code>ADD</code> for an existing item to increment or decrement an attribute value that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update doesn't have an attribute named <i>itemcount</i>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway. DynamoDB will create the <i>itemcount</i> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <i>itemcount</i> attribute in the item, with a value of <code>3</code>.</p> </note> </li> <li> <p>If the existing data type is a set and if <code>Value</code> is also a set, then <code>Value</code> is added to the existing set. For example, if the attribute value is the set <code>[1,2]</code>, and the <code>ADD</code> action specified <code>[3]</code>, then the final attribute value is <code>[1,2,3]</code>. An error occurs if an <code>ADD</code> action is specified for a set attribute and the attribute type specified does not match the existing set type. </p> <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <code>Value</code> must also be a set of strings.</p> </li> </ul> <important> <p>The <code>ADD</code> action only supports Number and set data types. In addition, <code>ADD</code> can only be used on top-level attributes, not nested attributes.</p> </important> </li> <li> <p> <code>DELETE</code> - Deletes an element from a set.</p> <p>If a set of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>[a,b,c]</code> and the <code>DELETE</code> action specifies <code>[a,c]</code>, then the final attribute value is <code>[b]</code>. Specifying an empty set is an error.</p> <important> <p>The <code>DELETE</code> action only supports set data types. In addition, <code>DELETE</code> can only be used on top-level attributes, not nested attributes.</p> </important> </li> </ul> <p>You can have many actions in a single expression, such as the following: <code>SET a=:value1, b=:value2 DELETE :value3, :value4, :value5</code> </p> <p>For more information on update expressions, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Modifying.html">Modifying Items and Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "UpdateExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_expression: Option<String>,
}

/// <p>Represents the output of an <code>UpdateItem</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateItemOutput {
    /// <p>A map of attribute values as they appear before or after the <code>UpdateItem</code> operation, as determined by the <code>ReturnValues</code> parameter.</p> <p>The <code>Attributes</code> map is only present if <code>ReturnValues</code> was specified as something other than <code>NONE</code> in the request. Each element represents one attribute.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>The capacity units consumed by the <code>UpdateItem</code> operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <code>ConsumedCapacity</code> is only returned if the <code>ReturnConsumedCapacity</code> parameter was specified. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    #[serde(rename = "ConsumedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<ConsumedCapacity>,
    /// <p><p>Information about item collections, if any, that were affected by the <code>UpdateItem</code> operation. <code>ItemCollectionMetrics</code> is only returned if the <code>ReturnItemCollectionMetrics</code> parameter was specified. If the table does not have any local secondary indexes, this information is not returned in the response.</p> <p>Each <code>ItemCollectionMetrics</code> element consists of:</p> <ul> <li> <p> <code>ItemCollectionKey</code> - The partition key value of the item collection. This is the same as the partition key value of the item itself.</p> </li> <li> <p> <code>SizeEstimateRangeGB</code> - An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li> </ul></p>
    #[serde(rename = "ItemCollectionMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_collection_metrics: Option<ItemCollectionMetrics>,
}

/// <p>Represents the input of an <code>UpdateTable</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTableInput {
    /// <p>An array of attributes that describe the key schema for the table and indexes. If you are adding a new global secondary index to the table, <code>AttributeDefinitions</code> must include the key element(s) of the new index.</p>
    #[serde(rename = "AttributeDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<Vec<AttributeDefinition>>,
    /// <p>An array of one or more global secondary indexes for the table. For each index in the array, you can request one action:</p> <ul> <li> <p> <code>Create</code> - add a new global secondary index to the table.</p> </li> <li> <p> <code>Update</code> - modify the provisioned throughput settings of an existing global secondary index.</p> </li> <li> <p> <code>Delete</code> - remove a global secondary index from the table.</p> </li> </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.OnlineOps.html">Managing Global Secondary Indexes</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    #[serde(rename = "GlobalSecondaryIndexUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_index_updates: Option<Vec<GlobalSecondaryIndexUpdate>>,
    /// <p>The new provisioned throughput settings for the specified table or index.</p>
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    /// <p><p>Represents the DynamoDB Streams configuration for the table.</p> <note> <p>You will receive a <code>ResourceInUseException</code> if you attempt to enable a stream on a table that already has a stream, or if you attempt to disable a stream on a table which does not have a stream.</p> </note></p>
    #[serde(rename = "StreamSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<StreamSpecification>,
    /// <p>The name of the table to be updated.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Represents the output of an <code>UpdateTable</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateTableOutput {
    /// <p>Represents the properties of the table.</p>
    #[serde(rename = "TableDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_description: Option<TableDescription>,
}

/// <p>Represents the input of an <code>UpdateTimeToLive</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTimeToLiveInput {
    /// <p>The name of the table to be configured.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// <p>Represents the settings used to enable or disable Time to Live for the specified table.</p>
    #[serde(rename = "TimeToLiveSpecification")]
    pub time_to_live_specification: TimeToLiveSpecification,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateTimeToLiveOutput {
    /// <p>Represents the output of an <code>UpdateTimeToLive</code> operation.</p>
    #[serde(rename = "TimeToLiveSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live_specification: Option<TimeToLiveSpecification>,
}

/// <p>Represents an operation to perform - either <code>DeleteItem</code> or <code>PutItem</code>. You can only request one of these operations, not both, in a single <code>WriteRequest</code>. If you do need to perform both of these operations, you will need to provide two separate <code>WriteRequest</code> objects.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteRequest {
    /// <p>A request to perform a <code>DeleteItem</code> operation.</p>
    #[serde(rename = "DeleteRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_request: Option<DeleteRequest>,
    /// <p>A request to perform a <code>PutItem</code> operation.</p>
    #[serde(rename = "PutRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_request: Option<PutRequest>,
}

/// Errors returned by BatchGetItem
#[derive(Debug, PartialEq)]
pub enum BatchGetItemError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchGetItemError {
    pub fn from_body(body: &str) -> BatchGetItemError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        BatchGetItemError::InternalServerError(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        BatchGetItemError::ProvisionedThroughputExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        BatchGetItemError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetItemError::Validation(error_message.to_string())
                    }
                    _ => BatchGetItemError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetItemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetItemError {
    fn from(err: serde_json::error::Error) -> BatchGetItemError {
        BatchGetItemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetItemError {
    fn from(err: CredentialsError) -> BatchGetItemError {
        BatchGetItemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetItemError {
    fn from(err: HttpDispatchError) -> BatchGetItemError {
        BatchGetItemError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchGetItemError {
    fn from(err: io::Error) -> BatchGetItemError {
        BatchGetItemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchGetItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetItemError {
    fn description(&self) -> &str {
        match *self {
            BatchGetItemError::InternalServerError(ref cause) => cause,
            BatchGetItemError::ProvisionedThroughputExceeded(ref cause) => cause,
            BatchGetItemError::ResourceNotFound(ref cause) => cause,
            BatchGetItemError::Validation(ref cause) => cause,
            BatchGetItemError::Credentials(ref err) => err.description(),
            BatchGetItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchGetItemError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchWriteItem
#[derive(Debug, PartialEq)]
pub enum BatchWriteItemError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
    ItemCollectionSizeLimitExceeded(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl BatchWriteItemError {
    pub fn from_body(body: &str) -> BatchWriteItemError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        BatchWriteItemError::InternalServerError(String::from(error_message))
                    }
                    "ItemCollectionSizeLimitExceededException" => {
                        BatchWriteItemError::ItemCollectionSizeLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ProvisionedThroughputExceededException" => {
                        BatchWriteItemError::ProvisionedThroughputExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        BatchWriteItemError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchWriteItemError::Validation(error_message.to_string())
                    }
                    _ => BatchWriteItemError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchWriteItemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchWriteItemError {
    fn from(err: serde_json::error::Error) -> BatchWriteItemError {
        BatchWriteItemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchWriteItemError {
    fn from(err: CredentialsError) -> BatchWriteItemError {
        BatchWriteItemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchWriteItemError {
    fn from(err: HttpDispatchError) -> BatchWriteItemError {
        BatchWriteItemError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchWriteItemError {
    fn from(err: io::Error) -> BatchWriteItemError {
        BatchWriteItemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchWriteItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchWriteItemError {
    fn description(&self) -> &str {
        match *self {
            BatchWriteItemError::InternalServerError(ref cause) => cause,
            BatchWriteItemError::ItemCollectionSizeLimitExceeded(ref cause) => cause,
            BatchWriteItemError::ProvisionedThroughputExceeded(ref cause) => cause,
            BatchWriteItemError::ResourceNotFound(ref cause) => cause,
            BatchWriteItemError::Validation(ref cause) => cause,
            BatchWriteItemError::Credentials(ref err) => err.description(),
            BatchWriteItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchWriteItemError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBackup
#[derive(Debug, PartialEq)]
pub enum CreateBackupError {
    /// <p>There is another ongoing conflicting backup control plane operation on the table. The backups is either being created, deleted or restored to a table.</p>
    BackupInUse(String),
    /// <p>Backups have not yet been enabled for this table.</p>
    ContinuousBackupsUnavailable(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>A target table with the specified name is either being created or deleted. </p>
    TableInUse(String),
    /// <p>A source table with the name <code>TableName</code> does not currently exist within the subscriber's account.</p>
    TableNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateBackupError {
    pub fn from_body(body: &str) -> CreateBackupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BackupInUseException" => {
                        CreateBackupError::BackupInUse(String::from(error_message))
                    }
                    "ContinuousBackupsUnavailableException" => {
                        CreateBackupError::ContinuousBackupsUnavailable(String::from(error_message))
                    }
                    "InternalServerError" => {
                        CreateBackupError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateBackupError::LimitExceeded(String::from(error_message))
                    }
                    "TableInUseException" => {
                        CreateBackupError::TableInUse(String::from(error_message))
                    }
                    "TableNotFoundException" => {
                        CreateBackupError::TableNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateBackupError::Validation(error_message.to_string())
                    }
                    _ => CreateBackupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateBackupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateBackupError {
    fn from(err: serde_json::error::Error) -> CreateBackupError {
        CreateBackupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBackupError {
    fn from(err: CredentialsError) -> CreateBackupError {
        CreateBackupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBackupError {
    fn from(err: HttpDispatchError) -> CreateBackupError {
        CreateBackupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBackupError {
    fn from(err: io::Error) -> CreateBackupError {
        CreateBackupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBackupError {
    fn description(&self) -> &str {
        match *self {
            CreateBackupError::BackupInUse(ref cause) => cause,
            CreateBackupError::ContinuousBackupsUnavailable(ref cause) => cause,
            CreateBackupError::InternalServerError(ref cause) => cause,
            CreateBackupError::LimitExceeded(ref cause) => cause,
            CreateBackupError::TableInUse(ref cause) => cause,
            CreateBackupError::TableNotFound(ref cause) => cause,
            CreateBackupError::Validation(ref cause) => cause,
            CreateBackupError::Credentials(ref err) => err.description(),
            CreateBackupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBackupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGlobalTable
#[derive(Debug, PartialEq)]
pub enum CreateGlobalTableError {
    /// <p>The specified global table already exists.</p>
    GlobalTableAlreadyExists(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>A source table with the name <code>TableName</code> does not currently exist within the subscriber's account.</p>
    TableNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateGlobalTableError {
    pub fn from_body(body: &str) -> CreateGlobalTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "GlobalTableAlreadyExistsException" => {
                        CreateGlobalTableError::GlobalTableAlreadyExists(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => {
                        CreateGlobalTableError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateGlobalTableError::LimitExceeded(String::from(error_message))
                    }
                    "TableNotFoundException" => {
                        CreateGlobalTableError::TableNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateGlobalTableError::Validation(error_message.to_string())
                    }
                    _ => CreateGlobalTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateGlobalTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateGlobalTableError {
    fn from(err: serde_json::error::Error) -> CreateGlobalTableError {
        CreateGlobalTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGlobalTableError {
    fn from(err: CredentialsError) -> CreateGlobalTableError {
        CreateGlobalTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGlobalTableError {
    fn from(err: HttpDispatchError) -> CreateGlobalTableError {
        CreateGlobalTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateGlobalTableError {
    fn from(err: io::Error) -> CreateGlobalTableError {
        CreateGlobalTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateGlobalTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGlobalTableError {
    fn description(&self) -> &str {
        match *self {
            CreateGlobalTableError::GlobalTableAlreadyExists(ref cause) => cause,
            CreateGlobalTableError::InternalServerError(ref cause) => cause,
            CreateGlobalTableError::LimitExceeded(ref cause) => cause,
            CreateGlobalTableError::TableNotFound(ref cause) => cause,
            CreateGlobalTableError::Validation(ref cause) => cause,
            CreateGlobalTableError::Credentials(ref err) => err.description(),
            CreateGlobalTableError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateGlobalTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTable
#[derive(Debug, PartialEq)]
pub enum CreateTableError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTableError {
    pub fn from_body(body: &str) -> CreateTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        CreateTableError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateTableError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        CreateTableError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTableError::Validation(error_message.to_string())
                    }
                    _ => CreateTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTableError {
    fn from(err: serde_json::error::Error) -> CreateTableError {
        CreateTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTableError {
    fn from(err: CredentialsError) -> CreateTableError {
        CreateTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTableError {
    fn from(err: HttpDispatchError) -> CreateTableError {
        CreateTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTableError {
    fn from(err: io::Error) -> CreateTableError {
        CreateTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTableError {
    fn description(&self) -> &str {
        match *self {
            CreateTableError::InternalServerError(ref cause) => cause,
            CreateTableError::LimitExceeded(ref cause) => cause,
            CreateTableError::ResourceInUse(ref cause) => cause,
            CreateTableError::Validation(ref cause) => cause,
            CreateTableError::Credentials(ref err) => err.description(),
            CreateTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBackup
#[derive(Debug, PartialEq)]
pub enum DeleteBackupError {
    /// <p>There is another ongoing conflicting backup control plane operation on the table. The backups is either being created, deleted or restored to a table.</p>
    BackupInUse(String),
    /// <p>Backup not found for the given BackupARN. </p>
    BackupNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteBackupError {
    pub fn from_body(body: &str) -> DeleteBackupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BackupInUseException" => {
                        DeleteBackupError::BackupInUse(String::from(error_message))
                    }
                    "BackupNotFoundException" => {
                        DeleteBackupError::BackupNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteBackupError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteBackupError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteBackupError::Validation(error_message.to_string())
                    }
                    _ => DeleteBackupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteBackupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteBackupError {
    fn from(err: serde_json::error::Error) -> DeleteBackupError {
        DeleteBackupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBackupError {
    fn from(err: CredentialsError) -> DeleteBackupError {
        DeleteBackupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBackupError {
    fn from(err: HttpDispatchError) -> DeleteBackupError {
        DeleteBackupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBackupError {
    fn from(err: io::Error) -> DeleteBackupError {
        DeleteBackupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBackupError {
    fn description(&self) -> &str {
        match *self {
            DeleteBackupError::BackupInUse(ref cause) => cause,
            DeleteBackupError::BackupNotFound(ref cause) => cause,
            DeleteBackupError::InternalServerError(ref cause) => cause,
            DeleteBackupError::LimitExceeded(ref cause) => cause,
            DeleteBackupError::Validation(ref cause) => cause,
            DeleteBackupError::Credentials(ref err) => err.description(),
            DeleteBackupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBackupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteItem
#[derive(Debug, PartialEq)]
pub enum DeleteItemError {
    /// <p>A condition specified in the operation could not be evaluated.</p>
    ConditionalCheckFailed(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
    ItemCollectionSizeLimitExceeded(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteItemError {
    pub fn from_body(body: &str) -> DeleteItemError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConditionalCheckFailedException" => {
                        DeleteItemError::ConditionalCheckFailed(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteItemError::InternalServerError(String::from(error_message))
                    }
                    "ItemCollectionSizeLimitExceededException" => {
                        DeleteItemError::ItemCollectionSizeLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ProvisionedThroughputExceededException" => {
                        DeleteItemError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteItemError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => DeleteItemError::Validation(error_message.to_string()),
                    _ => DeleteItemError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteItemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteItemError {
    fn from(err: serde_json::error::Error) -> DeleteItemError {
        DeleteItemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteItemError {
    fn from(err: CredentialsError) -> DeleteItemError {
        DeleteItemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteItemError {
    fn from(err: HttpDispatchError) -> DeleteItemError {
        DeleteItemError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteItemError {
    fn from(err: io::Error) -> DeleteItemError {
        DeleteItemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteItemError {
    fn description(&self) -> &str {
        match *self {
            DeleteItemError::ConditionalCheckFailed(ref cause) => cause,
            DeleteItemError::InternalServerError(ref cause) => cause,
            DeleteItemError::ItemCollectionSizeLimitExceeded(ref cause) => cause,
            DeleteItemError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteItemError::ResourceNotFound(ref cause) => cause,
            DeleteItemError::Validation(ref cause) => cause,
            DeleteItemError::Credentials(ref err) => err.description(),
            DeleteItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteItemError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTable
#[derive(Debug, PartialEq)]
pub enum DeleteTableError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
    ResourceInUse(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTableError {
    pub fn from_body(body: &str) -> DeleteTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DeleteTableError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteTableError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteTableError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteTableError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTableError::Validation(error_message.to_string())
                    }
                    _ => DeleteTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTableError {
    fn from(err: serde_json::error::Error) -> DeleteTableError {
        DeleteTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTableError {
    fn from(err: CredentialsError) -> DeleteTableError {
        DeleteTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTableError {
    fn from(err: HttpDispatchError) -> DeleteTableError {
        DeleteTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTableError {
    fn from(err: io::Error) -> DeleteTableError {
        DeleteTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTableError {
    fn description(&self) -> &str {
        match *self {
            DeleteTableError::InternalServerError(ref cause) => cause,
            DeleteTableError::LimitExceeded(ref cause) => cause,
            DeleteTableError::ResourceInUse(ref cause) => cause,
            DeleteTableError::ResourceNotFound(ref cause) => cause,
            DeleteTableError::Validation(ref cause) => cause,
            DeleteTableError::Credentials(ref err) => err.description(),
            DeleteTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBackup
#[derive(Debug, PartialEq)]
pub enum DescribeBackupError {
    /// <p>Backup not found for the given BackupARN. </p>
    BackupNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBackupError {
    pub fn from_body(body: &str) -> DescribeBackupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BackupNotFoundException" => {
                        DescribeBackupError::BackupNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DescribeBackupError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBackupError::Validation(error_message.to_string())
                    }
                    _ => DescribeBackupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBackupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBackupError {
    fn from(err: serde_json::error::Error) -> DescribeBackupError {
        DescribeBackupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBackupError {
    fn from(err: CredentialsError) -> DescribeBackupError {
        DescribeBackupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBackupError {
    fn from(err: HttpDispatchError) -> DescribeBackupError {
        DescribeBackupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBackupError {
    fn from(err: io::Error) -> DescribeBackupError {
        DescribeBackupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBackupError {
    fn description(&self) -> &str {
        match *self {
            DescribeBackupError::BackupNotFound(ref cause) => cause,
            DescribeBackupError::InternalServerError(ref cause) => cause,
            DescribeBackupError::Validation(ref cause) => cause,
            DescribeBackupError::Credentials(ref err) => err.description(),
            DescribeBackupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBackupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeContinuousBackups
#[derive(Debug, PartialEq)]
pub enum DescribeContinuousBackupsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>A source table with the name <code>TableName</code> does not currently exist within the subscriber's account.</p>
    TableNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeContinuousBackupsError {
    pub fn from_body(body: &str) -> DescribeContinuousBackupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => DescribeContinuousBackupsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "TableNotFoundException" => {
                        DescribeContinuousBackupsError::TableNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeContinuousBackupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeContinuousBackupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeContinuousBackupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeContinuousBackupsError {
    fn from(err: serde_json::error::Error) -> DescribeContinuousBackupsError {
        DescribeContinuousBackupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeContinuousBackupsError {
    fn from(err: CredentialsError) -> DescribeContinuousBackupsError {
        DescribeContinuousBackupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeContinuousBackupsError {
    fn from(err: HttpDispatchError) -> DescribeContinuousBackupsError {
        DescribeContinuousBackupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeContinuousBackupsError {
    fn from(err: io::Error) -> DescribeContinuousBackupsError {
        DescribeContinuousBackupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeContinuousBackupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeContinuousBackupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeContinuousBackupsError::InternalServerError(ref cause) => cause,
            DescribeContinuousBackupsError::TableNotFound(ref cause) => cause,
            DescribeContinuousBackupsError::Validation(ref cause) => cause,
            DescribeContinuousBackupsError::Credentials(ref err) => err.description(),
            DescribeContinuousBackupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeContinuousBackupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeGlobalTable
#[derive(Debug, PartialEq)]
pub enum DescribeGlobalTableError {
    /// <p>The specified global table does not exist.</p>
    GlobalTableNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeGlobalTableError {
    pub fn from_body(body: &str) -> DescribeGlobalTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "GlobalTableNotFoundException" => {
                        DescribeGlobalTableError::GlobalTableNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DescribeGlobalTableError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeGlobalTableError::Validation(error_message.to_string())
                    }
                    _ => DescribeGlobalTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeGlobalTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeGlobalTableError {
    fn from(err: serde_json::error::Error) -> DescribeGlobalTableError {
        DescribeGlobalTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeGlobalTableError {
    fn from(err: CredentialsError) -> DescribeGlobalTableError {
        DescribeGlobalTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeGlobalTableError {
    fn from(err: HttpDispatchError) -> DescribeGlobalTableError {
        DescribeGlobalTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeGlobalTableError {
    fn from(err: io::Error) -> DescribeGlobalTableError {
        DescribeGlobalTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeGlobalTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeGlobalTableError {
    fn description(&self) -> &str {
        match *self {
            DescribeGlobalTableError::GlobalTableNotFound(ref cause) => cause,
            DescribeGlobalTableError::InternalServerError(ref cause) => cause,
            DescribeGlobalTableError::Validation(ref cause) => cause,
            DescribeGlobalTableError::Credentials(ref err) => err.description(),
            DescribeGlobalTableError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeGlobalTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLimits
#[derive(Debug, PartialEq)]
pub enum DescribeLimitsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeLimitsError {
    pub fn from_body(body: &str) -> DescribeLimitsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeLimitsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeLimitsError::Validation(error_message.to_string())
                    }
                    _ => DescribeLimitsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeLimitsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeLimitsError {
    fn from(err: serde_json::error::Error) -> DescribeLimitsError {
        DescribeLimitsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLimitsError {
    fn from(err: CredentialsError) -> DescribeLimitsError {
        DescribeLimitsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLimitsError {
    fn from(err: HttpDispatchError) -> DescribeLimitsError {
        DescribeLimitsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLimitsError {
    fn from(err: io::Error) -> DescribeLimitsError {
        DescribeLimitsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLimitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLimitsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLimitsError::InternalServerError(ref cause) => cause,
            DescribeLimitsError::Validation(ref cause) => cause,
            DescribeLimitsError::Credentials(ref err) => err.description(),
            DescribeLimitsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeLimitsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTable
#[derive(Debug, PartialEq)]
pub enum DescribeTableError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTableError {
    pub fn from_body(body: &str) -> DescribeTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeTableError::InternalServerError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeTableError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTableError::Validation(error_message.to_string())
                    }
                    _ => DescribeTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTableError {
    fn from(err: serde_json::error::Error) -> DescribeTableError {
        DescribeTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTableError {
    fn from(err: CredentialsError) -> DescribeTableError {
        DescribeTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTableError {
    fn from(err: HttpDispatchError) -> DescribeTableError {
        DescribeTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTableError {
    fn from(err: io::Error) -> DescribeTableError {
        DescribeTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTableError {
    fn description(&self) -> &str {
        match *self {
            DescribeTableError::InternalServerError(ref cause) => cause,
            DescribeTableError::ResourceNotFound(ref cause) => cause,
            DescribeTableError::Validation(ref cause) => cause,
            DescribeTableError::Credentials(ref err) => err.description(),
            DescribeTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTimeToLive
#[derive(Debug, PartialEq)]
pub enum DescribeTimeToLiveError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTimeToLiveError {
    pub fn from_body(body: &str) -> DescribeTimeToLiveError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        DescribeTimeToLiveError::InternalServerError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeTimeToLiveError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTimeToLiveError::Validation(error_message.to_string())
                    }
                    _ => DescribeTimeToLiveError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTimeToLiveError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTimeToLiveError {
    fn from(err: serde_json::error::Error) -> DescribeTimeToLiveError {
        DescribeTimeToLiveError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTimeToLiveError {
    fn from(err: CredentialsError) -> DescribeTimeToLiveError {
        DescribeTimeToLiveError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTimeToLiveError {
    fn from(err: HttpDispatchError) -> DescribeTimeToLiveError {
        DescribeTimeToLiveError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTimeToLiveError {
    fn from(err: io::Error) -> DescribeTimeToLiveError {
        DescribeTimeToLiveError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTimeToLiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTimeToLiveError {
    fn description(&self) -> &str {
        match *self {
            DescribeTimeToLiveError::InternalServerError(ref cause) => cause,
            DescribeTimeToLiveError::ResourceNotFound(ref cause) => cause,
            DescribeTimeToLiveError::Validation(ref cause) => cause,
            DescribeTimeToLiveError::Credentials(ref err) => err.description(),
            DescribeTimeToLiveError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTimeToLiveError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetItem
#[derive(Debug, PartialEq)]
pub enum GetItemError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetItemError {
    pub fn from_body(body: &str) -> GetItemError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        GetItemError::InternalServerError(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        GetItemError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetItemError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => GetItemError::Validation(error_message.to_string()),
                    _ => GetItemError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetItemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetItemError {
    fn from(err: serde_json::error::Error) -> GetItemError {
        GetItemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetItemError {
    fn from(err: CredentialsError) -> GetItemError {
        GetItemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetItemError {
    fn from(err: HttpDispatchError) -> GetItemError {
        GetItemError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetItemError {
    fn from(err: io::Error) -> GetItemError {
        GetItemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetItemError {
    fn description(&self) -> &str {
        match *self {
            GetItemError::InternalServerError(ref cause) => cause,
            GetItemError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetItemError::ResourceNotFound(ref cause) => cause,
            GetItemError::Validation(ref cause) => cause,
            GetItemError::Credentials(ref err) => err.description(),
            GetItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetItemError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBackups
#[derive(Debug, PartialEq)]
pub enum ListBackupsError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListBackupsError {
    pub fn from_body(body: &str) -> ListBackupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListBackupsError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListBackupsError::Validation(error_message.to_string())
                    }
                    _ => ListBackupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBackupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBackupsError {
    fn from(err: serde_json::error::Error) -> ListBackupsError {
        ListBackupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBackupsError {
    fn from(err: CredentialsError) -> ListBackupsError {
        ListBackupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBackupsError {
    fn from(err: HttpDispatchError) -> ListBackupsError {
        ListBackupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListBackupsError {
    fn from(err: io::Error) -> ListBackupsError {
        ListBackupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListBackupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBackupsError {
    fn description(&self) -> &str {
        match *self {
            ListBackupsError::InternalServerError(ref cause) => cause,
            ListBackupsError::Validation(ref cause) => cause,
            ListBackupsError::Credentials(ref err) => err.description(),
            ListBackupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBackupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGlobalTables
#[derive(Debug, PartialEq)]
pub enum ListGlobalTablesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListGlobalTablesError {
    pub fn from_body(body: &str) -> ListGlobalTablesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListGlobalTablesError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListGlobalTablesError::Validation(error_message.to_string())
                    }
                    _ => ListGlobalTablesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListGlobalTablesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListGlobalTablesError {
    fn from(err: serde_json::error::Error) -> ListGlobalTablesError {
        ListGlobalTablesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGlobalTablesError {
    fn from(err: CredentialsError) -> ListGlobalTablesError {
        ListGlobalTablesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGlobalTablesError {
    fn from(err: HttpDispatchError) -> ListGlobalTablesError {
        ListGlobalTablesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListGlobalTablesError {
    fn from(err: io::Error) -> ListGlobalTablesError {
        ListGlobalTablesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListGlobalTablesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGlobalTablesError {
    fn description(&self) -> &str {
        match *self {
            ListGlobalTablesError::InternalServerError(ref cause) => cause,
            ListGlobalTablesError::Validation(ref cause) => cause,
            ListGlobalTablesError::Credentials(ref err) => err.description(),
            ListGlobalTablesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGlobalTablesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTables
#[derive(Debug, PartialEq)]
pub enum ListTablesError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTablesError {
    pub fn from_body(body: &str) -> ListTablesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListTablesError::InternalServerError(String::from(error_message))
                    }
                    "ValidationException" => ListTablesError::Validation(error_message.to_string()),
                    _ => ListTablesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTablesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTablesError {
    fn from(err: serde_json::error::Error) -> ListTablesError {
        ListTablesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTablesError {
    fn from(err: CredentialsError) -> ListTablesError {
        ListTablesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTablesError {
    fn from(err: HttpDispatchError) -> ListTablesError {
        ListTablesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTablesError {
    fn from(err: io::Error) -> ListTablesError {
        ListTablesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTablesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTablesError {
    fn description(&self) -> &str {
        match *self {
            ListTablesError::InternalServerError(ref cause) => cause,
            ListTablesError::Validation(ref cause) => cause,
            ListTablesError::Credentials(ref err) => err.description(),
            ListTablesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTablesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsOfResource
#[derive(Debug, PartialEq)]
pub enum ListTagsOfResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsOfResourceError {
    pub fn from_body(body: &str) -> ListTagsOfResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ListTagsOfResourceError::InternalServerError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTagsOfResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsOfResourceError::Validation(error_message.to_string())
                    }
                    _ => ListTagsOfResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsOfResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsOfResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsOfResourceError {
        ListTagsOfResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsOfResourceError {
    fn from(err: CredentialsError) -> ListTagsOfResourceError {
        ListTagsOfResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsOfResourceError {
    fn from(err: HttpDispatchError) -> ListTagsOfResourceError {
        ListTagsOfResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsOfResourceError {
    fn from(err: io::Error) -> ListTagsOfResourceError {
        ListTagsOfResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsOfResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsOfResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsOfResourceError::InternalServerError(ref cause) => cause,
            ListTagsOfResourceError::ResourceNotFound(ref cause) => cause,
            ListTagsOfResourceError::Validation(ref cause) => cause,
            ListTagsOfResourceError::Credentials(ref err) => err.description(),
            ListTagsOfResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsOfResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutItem
#[derive(Debug, PartialEq)]
pub enum PutItemError {
    /// <p>A condition specified in the operation could not be evaluated.</p>
    ConditionalCheckFailed(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
    ItemCollectionSizeLimitExceeded(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutItemError {
    pub fn from_body(body: &str) -> PutItemError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConditionalCheckFailedException" => {
                        PutItemError::ConditionalCheckFailed(String::from(error_message))
                    }
                    "InternalServerError" => {
                        PutItemError::InternalServerError(String::from(error_message))
                    }
                    "ItemCollectionSizeLimitExceededException" => {
                        PutItemError::ItemCollectionSizeLimitExceeded(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        PutItemError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutItemError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => PutItemError::Validation(error_message.to_string()),
                    _ => PutItemError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutItemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutItemError {
    fn from(err: serde_json::error::Error) -> PutItemError {
        PutItemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutItemError {
    fn from(err: CredentialsError) -> PutItemError {
        PutItemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutItemError {
    fn from(err: HttpDispatchError) -> PutItemError {
        PutItemError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutItemError {
    fn from(err: io::Error) -> PutItemError {
        PutItemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutItemError {
    fn description(&self) -> &str {
        match *self {
            PutItemError::ConditionalCheckFailed(ref cause) => cause,
            PutItemError::InternalServerError(ref cause) => cause,
            PutItemError::ItemCollectionSizeLimitExceeded(ref cause) => cause,
            PutItemError::ProvisionedThroughputExceeded(ref cause) => cause,
            PutItemError::ResourceNotFound(ref cause) => cause,
            PutItemError::Validation(ref cause) => cause,
            PutItemError::Credentials(ref err) => err.description(),
            PutItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutItemError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Query
#[derive(Debug, PartialEq)]
pub enum QueryError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl QueryError {
    pub fn from_body(body: &str) -> QueryError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        QueryError::InternalServerError(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        QueryError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        QueryError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => QueryError::Validation(error_message.to_string()),
                    _ => QueryError::Unknown(String::from(body)),
                }
            }
            Err(_) => QueryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for QueryError {
    fn from(err: serde_json::error::Error) -> QueryError {
        QueryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for QueryError {
    fn from(err: CredentialsError) -> QueryError {
        QueryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for QueryError {
    fn from(err: HttpDispatchError) -> QueryError {
        QueryError::HttpDispatch(err)
    }
}
impl From<io::Error> for QueryError {
    fn from(err: io::Error) -> QueryError {
        QueryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for QueryError {
    fn description(&self) -> &str {
        match *self {
            QueryError::InternalServerError(ref cause) => cause,
            QueryError::ProvisionedThroughputExceeded(ref cause) => cause,
            QueryError::ResourceNotFound(ref cause) => cause,
            QueryError::Validation(ref cause) => cause,
            QueryError::Credentials(ref err) => err.description(),
            QueryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            QueryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreTableFromBackup
#[derive(Debug, PartialEq)]
pub enum RestoreTableFromBackupError {
    /// <p>There is another ongoing conflicting backup control plane operation on the table. The backups is either being created, deleted or restored to a table.</p>
    BackupInUse(String),
    /// <p>Backup not found for the given BackupARN. </p>
    BackupNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>A target table with the specified name already exists. </p>
    TableAlreadyExists(String),
    /// <p>A target table with the specified name is either being created or deleted. </p>
    TableInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RestoreTableFromBackupError {
    pub fn from_body(body: &str) -> RestoreTableFromBackupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BackupInUseException" => {
                        RestoreTableFromBackupError::BackupInUse(String::from(error_message))
                    }
                    "BackupNotFoundException" => {
                        RestoreTableFromBackupError::BackupNotFound(String::from(error_message))
                    }
                    "InternalServerError" => RestoreTableFromBackupError::InternalServerError(
                        String::from(error_message),
                    ),
                    "LimitExceededException" => {
                        RestoreTableFromBackupError::LimitExceeded(String::from(error_message))
                    }
                    "TableAlreadyExistsException" => {
                        RestoreTableFromBackupError::TableAlreadyExists(String::from(error_message))
                    }
                    "TableInUseException" => {
                        RestoreTableFromBackupError::TableInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        RestoreTableFromBackupError::Validation(error_message.to_string())
                    }
                    _ => RestoreTableFromBackupError::Unknown(String::from(body)),
                }
            }
            Err(_) => RestoreTableFromBackupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RestoreTableFromBackupError {
    fn from(err: serde_json::error::Error) -> RestoreTableFromBackupError {
        RestoreTableFromBackupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RestoreTableFromBackupError {
    fn from(err: CredentialsError) -> RestoreTableFromBackupError {
        RestoreTableFromBackupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreTableFromBackupError {
    fn from(err: HttpDispatchError) -> RestoreTableFromBackupError {
        RestoreTableFromBackupError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreTableFromBackupError {
    fn from(err: io::Error) -> RestoreTableFromBackupError {
        RestoreTableFromBackupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreTableFromBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreTableFromBackupError {
    fn description(&self) -> &str {
        match *self {
            RestoreTableFromBackupError::BackupInUse(ref cause) => cause,
            RestoreTableFromBackupError::BackupNotFound(ref cause) => cause,
            RestoreTableFromBackupError::InternalServerError(ref cause) => cause,
            RestoreTableFromBackupError::LimitExceeded(ref cause) => cause,
            RestoreTableFromBackupError::TableAlreadyExists(ref cause) => cause,
            RestoreTableFromBackupError::TableInUse(ref cause) => cause,
            RestoreTableFromBackupError::Validation(ref cause) => cause,
            RestoreTableFromBackupError::Credentials(ref err) => err.description(),
            RestoreTableFromBackupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RestoreTableFromBackupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreTableToPointInTime
#[derive(Debug, PartialEq)]
pub enum RestoreTableToPointInTimeError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>An invalid restore time was specified. RestoreDateTime must be between EarliestRestorableDateTime and LatestRestorableDateTime.</p>
    InvalidRestoreTime(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>Point in time recovery has not yet been enabled for this source table.</p>
    PointInTimeRecoveryUnavailable(String),
    /// <p>A target table with the specified name already exists. </p>
    TableAlreadyExists(String),
    /// <p>A target table with the specified name is either being created or deleted. </p>
    TableInUse(String),
    /// <p>A source table with the name <code>TableName</code> does not currently exist within the subscriber's account.</p>
    TableNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RestoreTableToPointInTimeError {
    pub fn from_body(body: &str) -> RestoreTableToPointInTimeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => RestoreTableToPointInTimeError::InternalServerError(
                        String::from(error_message),
                    ),
                    "InvalidRestoreTimeException" => {
                        RestoreTableToPointInTimeError::InvalidRestoreTime(String::from(
                            error_message,
                        ))
                    }
                    "LimitExceededException" => {
                        RestoreTableToPointInTimeError::LimitExceeded(String::from(error_message))
                    }
                    "PointInTimeRecoveryUnavailableException" => {
                        RestoreTableToPointInTimeError::PointInTimeRecoveryUnavailable(
                            String::from(error_message),
                        )
                    }
                    "TableAlreadyExistsException" => {
                        RestoreTableToPointInTimeError::TableAlreadyExists(String::from(
                            error_message,
                        ))
                    }
                    "TableInUseException" => {
                        RestoreTableToPointInTimeError::TableInUse(String::from(error_message))
                    }
                    "TableNotFoundException" => {
                        RestoreTableToPointInTimeError::TableNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RestoreTableToPointInTimeError::Validation(error_message.to_string())
                    }
                    _ => RestoreTableToPointInTimeError::Unknown(String::from(body)),
                }
            }
            Err(_) => RestoreTableToPointInTimeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RestoreTableToPointInTimeError {
    fn from(err: serde_json::error::Error) -> RestoreTableToPointInTimeError {
        RestoreTableToPointInTimeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RestoreTableToPointInTimeError {
    fn from(err: CredentialsError) -> RestoreTableToPointInTimeError {
        RestoreTableToPointInTimeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreTableToPointInTimeError {
    fn from(err: HttpDispatchError) -> RestoreTableToPointInTimeError {
        RestoreTableToPointInTimeError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreTableToPointInTimeError {
    fn from(err: io::Error) -> RestoreTableToPointInTimeError {
        RestoreTableToPointInTimeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreTableToPointInTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreTableToPointInTimeError {
    fn description(&self) -> &str {
        match *self {
            RestoreTableToPointInTimeError::InternalServerError(ref cause) => cause,
            RestoreTableToPointInTimeError::InvalidRestoreTime(ref cause) => cause,
            RestoreTableToPointInTimeError::LimitExceeded(ref cause) => cause,
            RestoreTableToPointInTimeError::PointInTimeRecoveryUnavailable(ref cause) => cause,
            RestoreTableToPointInTimeError::TableAlreadyExists(ref cause) => cause,
            RestoreTableToPointInTimeError::TableInUse(ref cause) => cause,
            RestoreTableToPointInTimeError::TableNotFound(ref cause) => cause,
            RestoreTableToPointInTimeError::Validation(ref cause) => cause,
            RestoreTableToPointInTimeError::Credentials(ref err) => err.description(),
            RestoreTableToPointInTimeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RestoreTableToPointInTimeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Scan
#[derive(Debug, PartialEq)]
pub enum ScanError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ScanError {
    pub fn from_body(body: &str) -> ScanError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        ScanError::InternalServerError(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        ScanError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ScanError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => ScanError::Validation(error_message.to_string()),
                    _ => ScanError::Unknown(String::from(body)),
                }
            }
            Err(_) => ScanError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ScanError {
    fn from(err: serde_json::error::Error) -> ScanError {
        ScanError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ScanError {
    fn from(err: CredentialsError) -> ScanError {
        ScanError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ScanError {
    fn from(err: HttpDispatchError) -> ScanError {
        ScanError::HttpDispatch(err)
    }
}
impl From<io::Error> for ScanError {
    fn from(err: io::Error) -> ScanError {
        ScanError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ScanError {
    fn description(&self) -> &str {
        match *self {
            ScanError::InternalServerError(ref cause) => cause,
            ScanError::ProvisionedThroughputExceeded(ref cause) => cause,
            ScanError::ResourceNotFound(ref cause) => cause,
            ScanError::Validation(ref cause) => cause,
            ScanError::Credentials(ref err) => err.description(),
            ScanError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ScanError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
    ResourceInUse(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        TagResourceError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        TagResourceError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        TagResourceError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            TagResourceError::InternalServerError(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::ResourceInUse(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
    ResourceInUse(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        UntagResourceError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UntagResourceError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UntagResourceError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UntagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            UntagResourceError::InternalServerError(ref cause) => cause,
            UntagResourceError::LimitExceeded(ref cause) => cause,
            UntagResourceError::ResourceInUse(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateContinuousBackups
#[derive(Debug, PartialEq)]
pub enum UpdateContinuousBackupsError {
    /// <p>Backups have not yet been enabled for this table.</p>
    ContinuousBackupsUnavailable(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>A source table with the name <code>TableName</code> does not currently exist within the subscriber's account.</p>
    TableNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateContinuousBackupsError {
    pub fn from_body(body: &str) -> UpdateContinuousBackupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ContinuousBackupsUnavailableException" => {
                        UpdateContinuousBackupsError::ContinuousBackupsUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "InternalServerError" => UpdateContinuousBackupsError::InternalServerError(
                        String::from(error_message),
                    ),
                    "TableNotFoundException" => {
                        UpdateContinuousBackupsError::TableNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateContinuousBackupsError::Validation(error_message.to_string())
                    }
                    _ => UpdateContinuousBackupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateContinuousBackupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateContinuousBackupsError {
    fn from(err: serde_json::error::Error) -> UpdateContinuousBackupsError {
        UpdateContinuousBackupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateContinuousBackupsError {
    fn from(err: CredentialsError) -> UpdateContinuousBackupsError {
        UpdateContinuousBackupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateContinuousBackupsError {
    fn from(err: HttpDispatchError) -> UpdateContinuousBackupsError {
        UpdateContinuousBackupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateContinuousBackupsError {
    fn from(err: io::Error) -> UpdateContinuousBackupsError {
        UpdateContinuousBackupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateContinuousBackupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateContinuousBackupsError {
    fn description(&self) -> &str {
        match *self {
            UpdateContinuousBackupsError::ContinuousBackupsUnavailable(ref cause) => cause,
            UpdateContinuousBackupsError::InternalServerError(ref cause) => cause,
            UpdateContinuousBackupsError::TableNotFound(ref cause) => cause,
            UpdateContinuousBackupsError::Validation(ref cause) => cause,
            UpdateContinuousBackupsError::Credentials(ref err) => err.description(),
            UpdateContinuousBackupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateContinuousBackupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGlobalTable
#[derive(Debug, PartialEq)]
pub enum UpdateGlobalTableError {
    /// <p>The specified global table does not exist.</p>
    GlobalTableNotFound(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>The specified replica is already part of the global table.</p>
    ReplicaAlreadyExists(String),
    /// <p>The specified replica is no longer part of the global table.</p>
    ReplicaNotFound(String),
    /// <p>A source table with the name <code>TableName</code> does not currently exist within the subscriber's account.</p>
    TableNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateGlobalTableError {
    pub fn from_body(body: &str) -> UpdateGlobalTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "GlobalTableNotFoundException" => {
                        UpdateGlobalTableError::GlobalTableNotFound(String::from(error_message))
                    }
                    "InternalServerError" => {
                        UpdateGlobalTableError::InternalServerError(String::from(error_message))
                    }
                    "ReplicaAlreadyExistsException" => {
                        UpdateGlobalTableError::ReplicaAlreadyExists(String::from(error_message))
                    }
                    "ReplicaNotFoundException" => {
                        UpdateGlobalTableError::ReplicaNotFound(String::from(error_message))
                    }
                    "TableNotFoundException" => {
                        UpdateGlobalTableError::TableNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateGlobalTableError::Validation(error_message.to_string())
                    }
                    _ => UpdateGlobalTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateGlobalTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateGlobalTableError {
    fn from(err: serde_json::error::Error) -> UpdateGlobalTableError {
        UpdateGlobalTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateGlobalTableError {
    fn from(err: CredentialsError) -> UpdateGlobalTableError {
        UpdateGlobalTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateGlobalTableError {
    fn from(err: HttpDispatchError) -> UpdateGlobalTableError {
        UpdateGlobalTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateGlobalTableError {
    fn from(err: io::Error) -> UpdateGlobalTableError {
        UpdateGlobalTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateGlobalTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGlobalTableError {
    fn description(&self) -> &str {
        match *self {
            UpdateGlobalTableError::GlobalTableNotFound(ref cause) => cause,
            UpdateGlobalTableError::InternalServerError(ref cause) => cause,
            UpdateGlobalTableError::ReplicaAlreadyExists(ref cause) => cause,
            UpdateGlobalTableError::ReplicaNotFound(ref cause) => cause,
            UpdateGlobalTableError::TableNotFound(ref cause) => cause,
            UpdateGlobalTableError::Validation(ref cause) => cause,
            UpdateGlobalTableError::Credentials(ref err) => err.description(),
            UpdateGlobalTableError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateGlobalTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateItem
#[derive(Debug, PartialEq)]
pub enum UpdateItemError {
    /// <p>A condition specified in the operation could not be evaluated.</p>
    ConditionalCheckFailed(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
    ItemCollectionSizeLimitExceeded(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateItemError {
    pub fn from_body(body: &str) -> UpdateItemError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConditionalCheckFailedException" => {
                        UpdateItemError::ConditionalCheckFailed(String::from(error_message))
                    }
                    "InternalServerError" => {
                        UpdateItemError::InternalServerError(String::from(error_message))
                    }
                    "ItemCollectionSizeLimitExceededException" => {
                        UpdateItemError::ItemCollectionSizeLimitExceeded(String::from(
                            error_message,
                        ))
                    }
                    "ProvisionedThroughputExceededException" => {
                        UpdateItemError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateItemError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => UpdateItemError::Validation(error_message.to_string()),
                    _ => UpdateItemError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateItemError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateItemError {
    fn from(err: serde_json::error::Error) -> UpdateItemError {
        UpdateItemError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateItemError {
    fn from(err: CredentialsError) -> UpdateItemError {
        UpdateItemError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateItemError {
    fn from(err: HttpDispatchError) -> UpdateItemError {
        UpdateItemError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateItemError {
    fn from(err: io::Error) -> UpdateItemError {
        UpdateItemError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateItemError {
    fn description(&self) -> &str {
        match *self {
            UpdateItemError::ConditionalCheckFailed(ref cause) => cause,
            UpdateItemError::InternalServerError(ref cause) => cause,
            UpdateItemError::ItemCollectionSizeLimitExceeded(ref cause) => cause,
            UpdateItemError::ProvisionedThroughputExceeded(ref cause) => cause,
            UpdateItemError::ResourceNotFound(ref cause) => cause,
            UpdateItemError::Validation(ref cause) => cause,
            UpdateItemError::Credentials(ref err) => err.description(),
            UpdateItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateItemError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTable
#[derive(Debug, PartialEq)]
pub enum UpdateTableError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
    ResourceInUse(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTableError {
    pub fn from_body(body: &str) -> UpdateTableError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        UpdateTableError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateTableError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateTableError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateTableError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateTableError::Validation(error_message.to_string())
                    }
                    _ => UpdateTableError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateTableError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateTableError {
    fn from(err: serde_json::error::Error) -> UpdateTableError {
        UpdateTableError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTableError {
    fn from(err: CredentialsError) -> UpdateTableError {
        UpdateTableError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTableError {
    fn from(err: HttpDispatchError) -> UpdateTableError {
        UpdateTableError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTableError {
    fn from(err: io::Error) -> UpdateTableError {
        UpdateTableError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTableError {
    fn description(&self) -> &str {
        match *self {
            UpdateTableError::InternalServerError(ref cause) => cause,
            UpdateTableError::LimitExceeded(ref cause) => cause,
            UpdateTableError::ResourceInUse(ref cause) => cause,
            UpdateTableError::ResourceNotFound(ref cause) => cause,
            UpdateTableError::Validation(ref cause) => cause,
            UpdateTableError::Credentials(ref err) => err.description(),
            UpdateTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateTableError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateTimeToLive
#[derive(Debug, PartialEq)]
pub enum UpdateTimeToLiveError {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Up to 50 <code>CreateBackup</code> operations are allowed per second, per account. There is no limit to the number of daily on-demand backups that can be taken. </p> <p>Up to 10 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> <p>For tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
    LimitExceeded(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
    ResourceInUse(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateTimeToLiveError {
    pub fn from_body(body: &str) -> UpdateTimeToLiveError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalServerError" => {
                        UpdateTimeToLiveError::InternalServerError(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateTimeToLiveError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateTimeToLiveError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateTimeToLiveError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateTimeToLiveError::Validation(error_message.to_string())
                    }
                    _ => UpdateTimeToLiveError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateTimeToLiveError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateTimeToLiveError {
    fn from(err: serde_json::error::Error) -> UpdateTimeToLiveError {
        UpdateTimeToLiveError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateTimeToLiveError {
    fn from(err: CredentialsError) -> UpdateTimeToLiveError {
        UpdateTimeToLiveError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateTimeToLiveError {
    fn from(err: HttpDispatchError) -> UpdateTimeToLiveError {
        UpdateTimeToLiveError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateTimeToLiveError {
    fn from(err: io::Error) -> UpdateTimeToLiveError {
        UpdateTimeToLiveError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateTimeToLiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTimeToLiveError {
    fn description(&self) -> &str {
        match *self {
            UpdateTimeToLiveError::InternalServerError(ref cause) => cause,
            UpdateTimeToLiveError::LimitExceeded(ref cause) => cause,
            UpdateTimeToLiveError::ResourceInUse(ref cause) => cause,
            UpdateTimeToLiveError::ResourceNotFound(ref cause) => cause,
            UpdateTimeToLiveError::Validation(ref cause) => cause,
            UpdateTimeToLiveError::Credentials(ref err) => err.description(),
            UpdateTimeToLiveError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateTimeToLiveError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the DynamoDB API. DynamoDB clients implement this trait.
pub trait DynamoDb {
    /// <p>The <code>BatchGetItem</code> operation returns the attributes of one or more items from one or more tables. You identify requested items by primary key.</p> <p>A single operation can retrieve up to 16 MB of data, which can contain as many as 100 items. <code>BatchGetItem</code> will return a partial result if the response size limit is exceeded, the table's provisioned throughput is exceeded, or an internal processing failure occurs. If a partial result is returned, the operation returns a value for <code>UnprocessedKeys</code>. You can use this value to retry the operation starting with the next item to get.</p> <important> <p>If you request more than 100 items <code>BatchGetItem</code> will return a <code>ValidationException</code> with the message "Too many items requested for the BatchGetItem call".</p> </important> <p>For example, if you ask to retrieve 100 items, but each individual item is 300 KB in size, the system returns 52 items (so as not to exceed the 16 MB limit). It also returns an appropriate <code>UnprocessedKeys</code> value so you can get the next page of results. If desired, your application can include its own logic to assemble the pages of results into one data set.</p> <p>If <i>none</i> of the items can be processed due to insufficient provisioned throughput on all of the tables in the request, then <code>BatchGetItem</code> will return a <code>ProvisionedThroughputExceededException</code>. If <i>at least one</i> of the items is successfully processed, then <code>BatchGetItem</code> completes successfully, while returning the keys of the unread items in <code>UnprocessedKeys</code>.</p> <important> <p>If DynamoDB returns any unprocessed items, you should retry the batch operation on those items. However, <i>we strongly recommend that you use an exponential backoff algorithm</i>. If you retry the batch operation immediately, the underlying read or write requests can still fail due to throttling on the individual tables. If you delay the batch operation using exponential backoff, the individual requests in the batch are much more likely to succeed.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOperations">Batch Operations and Error Handling</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </important> <p>By default, <code>BatchGetItem</code> performs eventually consistent reads on every table in the request. If you want strongly consistent reads instead, you can set <code>ConsistentRead</code> to <code>true</code> for any or all tables.</p> <p>In order to minimize response latency, <code>BatchGetItem</code> retrieves items in parallel.</p> <p>When designing your application, keep in mind that DynamoDB does not return items in any particular order. To help parse the response by item, include the primary key values for the items in your request in the <code>ProjectionExpression</code> parameter.</p> <p>If a requested item does not exist, it is not returned in the result. Requests for nonexistent items consume the minimum read capacity units according to the type of read. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#CapacityUnitCalculations">Capacity Units Calculations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn batch_get_item(
        &self,
        input: BatchGetItemInput,
    ) -> RusotoFuture<BatchGetItemOutput, BatchGetItemError>;

    /// <p><p>The <code>BatchWriteItem</code> operation puts or deletes multiple items in one or more tables. A single call to <code>BatchWriteItem</code> can write up to 16 MB of data, which can comprise as many as 25 put or delete requests. Individual items to be written can be as large as 400 KB.</p> <note> <p> <code>BatchWriteItem</code> cannot update items. To update items, use the <code>UpdateItem</code> action.</p> </note> <p>The individual <code>PutItem</code> and <code>DeleteItem</code> operations specified in <code>BatchWriteItem</code> are atomic; however <code>BatchWriteItem</code> as a whole is not. If any requested operations fail because the table&#39;s provisioned throughput is exceeded or an internal processing failure occurs, the failed operations are returned in the <code>UnprocessedItems</code> response parameter. You can investigate and optionally resend the requests. Typically, you would call <code>BatchWriteItem</code> in a loop. Each iteration would check for unprocessed items and submit a new <code>BatchWriteItem</code> request with those unprocessed items until all items have been processed.</p> <p>Note that if <i>none</i> of the items can be processed due to insufficient provisioned throughput on all of the tables in the request, then <code>BatchWriteItem</code> will return a <code>ProvisionedThroughputExceededException</code>.</p> <important> <p>If DynamoDB returns any unprocessed items, you should retry the batch operation on those items. However, <i>we strongly recommend that you use an exponential backoff algorithm</i>. If you retry the batch operation immediately, the underlying read or write requests can still fail due to throttling on the individual tables. If you delay the batch operation using exponential backoff, the individual requests in the batch are much more likely to succeed.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOperations">Batch Operations and Error Handling</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </important> <p>With <code>BatchWriteItem</code>, you can efficiently write or delete large amounts of data, such as from Amazon Elastic MapReduce (EMR), or copy data from another database into DynamoDB. In order to improve performance with these large-scale operations, <code>BatchWriteItem</code> does not behave in the same way as individual <code>PutItem</code> and <code>DeleteItem</code> calls would. For example, you cannot specify conditions on individual put and delete requests, and <code>BatchWriteItem</code> does not return deleted items in the response.</p> <p>If you use a programming language that supports concurrency, you can use threads to write items in parallel. Your application must include the necessary logic to manage the threads. With languages that don&#39;t support threading, you must update or delete the specified items one at a time. In both situations, <code>BatchWriteItem</code> performs the specified put and delete operations in parallel, giving you the power of the thread pool approach without having to introduce complexity into your application.</p> <p>Parallel processing reduces latency, but each specified put and delete request consumes the same number of write capacity units whether it is processed in parallel or not. Delete operations on nonexistent items consume one write capacity unit.</p> <p>If one or more of the following is true, DynamoDB rejects the entire batch write operation:</p> <ul> <li> <p>One or more tables specified in the <code>BatchWriteItem</code> request does not exist.</p> </li> <li> <p>Primary key attributes specified on an item in the request do not match those in the corresponding table&#39;s primary key schema.</p> </li> <li> <p>You try to perform multiple operations on the same item in the same <code>BatchWriteItem</code> request. For example, you cannot put and delete the same item in the same <code>BatchWriteItem</code> request. </p> </li> <li> <p> Your request contains at least two items with identical hash and range keys (which essentially is two put operations). </p> </li> <li> <p>There are more than 25 requests in the batch.</p> </li> <li> <p>Any individual item in a batch exceeds 400 KB.</p> </li> <li> <p>The total request size exceeds 16 MB.</p> </li> </ul></p>
    fn batch_write_item(
        &self,
        input: BatchWriteItemInput,
    ) -> RusotoFuture<BatchWriteItemOutput, BatchWriteItemError>;

    /// <p><p>Creates a backup for an existing table.</p> <p> Each time you create an On-Demand Backup, the entire table data is backed up. There is no limit to the number of on-demand backups that can be taken. </p> <p> When you create an On-Demand Backup, a time marker of the request is cataloged, and the backup is created asynchronously, by applying all changes until the time of the request to the last full table snapshot. Backup requests are processed instantaneously and become available for restore within minutes. </p> <p>You can call <code>CreateBackup</code> at a maximum rate of 50 times per second.</p> <p>All backups in DynamoDB work without consuming any provisioned throughput on the table.</p> <p> If you submit a backup request on 2018-12-14 at 14:25:00, the backup is guaranteed to contain all data committed to the table up to 14:24:00, and data committed after 14:26:00 will not be. The backup may or may not contain data modifications made between 14:24:00 and 14:26:00. On-Demand Backup does not support causal consistency. </p> <p> Along with data, the following are also included on the backups: </p> <ul> <li> <p>Global secondary indexes (GSIs)</p> </li> <li> <p>Local secondary indexes (LSIs)</p> </li> <li> <p>Streams</p> </li> <li> <p>Provisioned read and write capacity</p> </li> </ul></p>
    fn create_backup(
        &self,
        input: CreateBackupInput,
    ) -> RusotoFuture<CreateBackupOutput, CreateBackupError>;

    /// <p><p>Creates a global table from an existing table. A global table creates a replication relationship between two or more DynamoDB tables with the same table name in the provided regions. </p> <p> Tables can only be added as the replicas of a global table group under the following conditions: </p> <ul> <li> <p> The tables must have the same name. </p> </li> <li> <p> The tables must contain no items. </p> </li> <li> <p> The tables must have the same hash key and sort key (if present). </p> </li> <li> <p> The tables must have DynamoDB Streams enabled (NEW<em>AND</em>OLD_IMAGES). </p> </li> </ul></p>
    fn create_global_table(
        &self,
        input: CreateGlobalTableInput,
    ) -> RusotoFuture<CreateGlobalTableOutput, CreateGlobalTableError>;

    /// <p>The <code>CreateTable</code> operation adds a new table to your account. In an AWS account, table names must be unique within each region. That is, you can have two tables with same name if you create the tables in different regions.</p> <p> <code>CreateTable</code> is an asynchronous operation. Upon receiving a <code>CreateTable</code> request, DynamoDB immediately returns a response with a <code>TableStatus</code> of <code>CREATING</code>. After the table is created, DynamoDB sets the <code>TableStatus</code> to <code>ACTIVE</code>. You can perform read and write operations only on an <code>ACTIVE</code> table. </p> <p>You can optionally define secondary indexes on the new table, as part of the <code>CreateTable</code> operation. If you want to create multiple tables with secondary indexes on them, you must create the tables sequentially. Only one table with secondary indexes can be in the <code>CREATING</code> state at any given time.</p> <p>You can use the <code>DescribeTable</code> action to check the table status.</p>
    fn create_table(
        &self,
        input: CreateTableInput,
    ) -> RusotoFuture<CreateTableOutput, CreateTableError>;

    /// <p>Deletes an existing backup of a table.</p> <p>You can call <code>DeleteBackup</code> at a maximum rate of 10 times per second.</p>
    fn delete_backup(
        &self,
        input: DeleteBackupInput,
    ) -> RusotoFuture<DeleteBackupOutput, DeleteBackupError>;

    /// <p>Deletes a single item in a table by primary key. You can perform a conditional delete operation that deletes the item if it exists, or if it has an expected attribute value.</p> <p>In addition to deleting an item, you can also return the item's attribute values in the same operation, using the <code>ReturnValues</code> parameter.</p> <p>Unless you specify conditions, the <code>DeleteItem</code> is an idempotent operation; running it multiple times on the same item or attribute does <i>not</i> result in an error response.</p> <p>Conditional deletes are useful for deleting items only if specific conditions are met. If those conditions are met, DynamoDB performs the delete. Otherwise, the item is not deleted.</p>
    fn delete_item(
        &self,
        input: DeleteItemInput,
    ) -> RusotoFuture<DeleteItemOutput, DeleteItemError>;

    /// <p>The <code>DeleteTable</code> operation deletes a table and all of its items. After a <code>DeleteTable</code> request, the specified table is in the <code>DELETING</code> state until DynamoDB completes the deletion. If the table is in the <code>ACTIVE</code> state, you can delete it. If a table is in <code>CREATING</code> or <code>UPDATING</code> states, then DynamoDB returns a <code>ResourceInUseException</code>. If the specified table does not exist, DynamoDB returns a <code>ResourceNotFoundException</code>. If table is already in the <code>DELETING</code> state, no error is returned. </p> <note> <p>DynamoDB might continue to accept data read and write operations, such as <code>GetItem</code> and <code>PutItem</code>, on a table in the <code>DELETING</code> state until the table deletion is complete.</p> </note> <p>When you delete a table, any indexes on that table are also deleted.</p> <p>If you have DynamoDB Streams enabled on the table, then the corresponding stream on that table goes into the <code>DISABLED</code> state, and the stream is automatically deleted after 24 hours.</p> <p>Use the <code>DescribeTable</code> action to check the status of the table. </p>
    fn delete_table(
        &self,
        input: DeleteTableInput,
    ) -> RusotoFuture<DeleteTableOutput, DeleteTableError>;

    /// <p>Describes an existing backup of a table.</p> <p>You can call <code>DescribeBackup</code> at a maximum rate of 10 times per second.</p>
    fn describe_backup(
        &self,
        input: DescribeBackupInput,
    ) -> RusotoFuture<DescribeBackupOutput, DescribeBackupError>;

    /// <p>Checks the status of continuous backups and point in time recovery on the specified table. Continuous backups are <code>ENABLED</code> on all tables at table creation. If point in time recovery is enabled, <code>PointInTimeRecoveryStatus</code> will be set to ENABLED.</p> <p> Once continuous backups and point in time recovery are enabled, you can restore to any point in time within <code>EarliestRestorableDateTime</code> and <code>LatestRestorableDateTime</code>. </p> <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. You can restore your table to any point in time during the last 35 days with a 1-minute granularity. </p> <p>You can call <code>DescribeContinuousBackups</code> at a maximum rate of 10 times per second.</p>
    fn describe_continuous_backups(
        &self,
        input: DescribeContinuousBackupsInput,
    ) -> RusotoFuture<DescribeContinuousBackupsOutput, DescribeContinuousBackupsError>;

    /// <p>Returns information about the specified global table.</p>
    fn describe_global_table(
        &self,
        input: DescribeGlobalTableInput,
    ) -> RusotoFuture<DescribeGlobalTableOutput, DescribeGlobalTableError>;

    /// <p>Returns the current provisioned-capacity limits for your AWS account in a region, both for the region as a whole and for any one DynamoDB table that you create there.</p> <p>When you establish an AWS account, the account has initial limits on the maximum read capacity units and write capacity units that you can provision across all of your DynamoDB tables in a given region. Also, there are per-table limits that apply when you create a table there. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> page in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Although you can increase these limits by filing a case at <a href="https://console.aws.amazon.com/support/home#/">AWS Support Center</a>, obtaining the increase is not instantaneous. The <code>DescribeLimits</code> action lets you write code to compare the capacity you are currently using to those limits imposed by your account so that you have enough time to apply for an increase before you hit a limit.</p> <p>For example, you could use one of the AWS SDKs to do the following:</p> <ol> <li> <p>Call <code>DescribeLimits</code> for a particular region to obtain your current account limits on provisioned capacity there.</p> </li> <li> <p>Create a variable to hold the aggregate read capacity units provisioned for all your tables in that region, and one to hold the aggregate write capacity units. Zero them both.</p> </li> <li> <p>Call <code>ListTables</code> to obtain a list of all your DynamoDB tables.</p> </li> <li> <p>For each table name listed by <code>ListTables</code>, do the following:</p> <ul> <li> <p>Call <code>DescribeTable</code> with the table name.</p> </li> <li> <p>Use the data returned by <code>DescribeTable</code> to add the read capacity units and write capacity units provisioned for the table itself to your variables.</p> </li> <li> <p>If the table has one or more global secondary indexes (GSIs), loop over these GSIs and add their provisioned capacity values to your variables as well.</p> </li> </ul> </li> <li> <p>Report the account limits for that region returned by <code>DescribeLimits</code>, along with the total current provisioned capacity levels you have calculated.</p> </li> </ol> <p>This will let you see whether you are getting close to your account-level limits.</p> <p>The per-table limits apply only when you are creating a new table. They restrict the sum of the provisioned capacity of the new table itself and all its global secondary indexes.</p> <p>For existing tables and their GSIs, DynamoDB will not let you increase provisioned capacity extremely rapidly, but the only upper limit that applies is that the aggregate provisioned capacity over all your tables and GSIs cannot exceed either of the per-account limits.</p> <note> <p> <code>DescribeLimits</code> should only be called periodically. You can expect throttling errors if you call it more than once in a minute.</p> </note> <p>The <code>DescribeLimits</code> Request element has no content.</p>
    fn describe_limits(&self) -> RusotoFuture<DescribeLimitsOutput, DescribeLimitsError>;

    /// <p><p>Returns information about the table, including the current status of the table, when it was created, the primary key schema, and any indexes on the table.</p> <note> <p>If you issue a <code>DescribeTable</code> request immediately after a <code>CreateTable</code> request, DynamoDB might return a <code>ResourceNotFoundException</code>. This is because <code>DescribeTable</code> uses an eventually consistent query, and the metadata for your table might not be available at that moment. Wait for a few seconds, and then try the <code>DescribeTable</code> request again.</p> </note></p>
    fn describe_table(
        &self,
        input: DescribeTableInput,
    ) -> RusotoFuture<DescribeTableOutput, DescribeTableError>;

    /// <p>Gives a description of the Time to Live (TTL) status on the specified table. </p>
    fn describe_time_to_live(
        &self,
        input: DescribeTimeToLiveInput,
    ) -> RusotoFuture<DescribeTimeToLiveOutput, DescribeTimeToLiveError>;

    /// <p>The <code>GetItem</code> operation returns a set of attributes for the item with the given primary key. If there is no matching item, <code>GetItem</code> does not return any data and there will be no <code>Item</code> element in the response.</p> <p> <code>GetItem</code> provides an eventually consistent read by default. If your application requires a strongly consistent read, set <code>ConsistentRead</code> to <code>true</code>. Although a strongly consistent read might take more time than an eventually consistent read, it always returns the last updated value.</p>
    fn get_item(&self, input: GetItemInput) -> RusotoFuture<GetItemOutput, GetItemError>;

    /// <p>List backups associated with an AWS account. To list backups for a given table, specify <code>TableName</code>. <code>ListBackups</code> returns a paginated list of results with at most 1MB worth of items in a page. You can also specify a limit for the maximum number of entries to be returned in a page. </p> <p>In the request, start time is inclusive but end time is exclusive. Note that these limits are for the time at which the original backup was requested.</p> <p>You can call <code>ListBackups</code> a maximum of 5 times per second.</p>
    fn list_backups(
        &self,
        input: ListBackupsInput,
    ) -> RusotoFuture<ListBackupsOutput, ListBackupsError>;

    /// <p>Lists all global tables that have a replica in the specified region.</p>
    fn list_global_tables(
        &self,
        input: ListGlobalTablesInput,
    ) -> RusotoFuture<ListGlobalTablesOutput, ListGlobalTablesError>;

    /// <p>Returns an array of table names associated with the current account and endpoint. The output from <code>ListTables</code> is paginated, with each page returning a maximum of 100 table names.</p>
    fn list_tables(
        &self,
        input: ListTablesInput,
    ) -> RusotoFuture<ListTablesOutput, ListTablesError>;

    /// <p>List all tags on an Amazon DynamoDB resource. You can call ListTagsOfResource up to 10 times per second, per account.</p> <p>For an overview on tagging DynamoDB resources, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html">Tagging for DynamoDB</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn list_tags_of_resource(
        &self,
        input: ListTagsOfResourceInput,
    ) -> RusotoFuture<ListTagsOfResourceOutput, ListTagsOfResourceError>;

    /// <p>Creates a new item, or replaces an old item with a new item. If an item that has the same primary key as the new item already exists in the specified table, the new item completely replaces the existing item. You can perform a conditional put operation (add a new item if one with the specified primary key doesn't exist), or replace an existing item if it has certain attribute values. You can return the item's attribute values in the same operation, using the <code>ReturnValues</code> parameter.</p> <important> <p>This topic provides general information about the <code>PutItem</code> API.</p> <p>For information on how to call the <code>PutItem</code> API using the AWS SDK in specific languages, see the following:</p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/goto/aws-cli/dynamodb-2012-08-10/PutItem"> PutItem in the AWS Command Line Interface </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/DotNetSDKV3/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for .NET </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForCpp/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for C++ </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForGoV1/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for Go </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForJava/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for Java </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/AWSJavaScriptSDK/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for JavaScript </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForPHPV3/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for PHP V3 </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/boto3/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for Python </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForRubyV2/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for Ruby V2 </a> </p> </li> </ul> </important> <p>When you add an item, the primary key attribute(s) are the only required attributes. Attribute values cannot be null. String and Binary type attributes must have lengths greater than zero. Set type attributes cannot be empty. Requests with empty values will be rejected with a <code>ValidationException</code> exception.</p> <note> <p>To prevent a new item from replacing an existing item, use a conditional expression that contains the <code>attribute_not_exists</code> function with the name of the attribute being used as the partition key for the table. Since every record must contain that attribute, the <code>attribute_not_exists</code> function will only succeed if no matching item exists.</p> </note> <p>For more information about <code>PutItem</code>, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithItems.html">Working with Items</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn put_item(&self, input: PutItemInput) -> RusotoFuture<PutItemOutput, PutItemError>;

    /// <p>The <code>Query</code> operation finds items based on primary key values. You can query any table or secondary index that has a composite primary key (a partition key and a sort key). </p> <p>Use the <code>KeyConditionExpression</code> parameter to provide a specific value for the partition key. The <code>Query</code> operation will return all of the items from the table or index with that partition key value. You can optionally narrow the scope of the <code>Query</code> operation by specifying a sort key value and a comparison operator in <code>KeyConditionExpression</code>. To further refine the <code>Query</code> results, you can optionally provide a <code>FilterExpression</code>. A <code>FilterExpression</code> determines which items within the results should be returned to you. All of the other results are discarded. </p> <p> A <code>Query</code> operation always returns a result set. If no matching items are found, the result set will be empty. Queries that do not return results consume the minimum number of read capacity units for that type of read operation. </p> <note> <p> DynamoDB calculates the number of read capacity units consumed based on item size, not on the amount of data that is returned to an application. The number of capacity units consumed will be the same whether you request all of the attributes (the default behavior) or just some of them (using a projection expression). The number will also be the same whether or not you use a <code>FilterExpression</code>. </p> </note> <p> <code>Query</code> results are always sorted by the sort key value. If the data type of the sort key is Number, the results are returned in numeric order; otherwise, the results are returned in order of UTF-8 bytes. By default, the sort order is ascending. To reverse the order, set the <code>ScanIndexForward</code> parameter to false. </p> <p> A single <code>Query</code> operation will read up to the maximum number of items set (if using the <code>Limit</code> parameter) or a maximum of 1 MB of data and then apply any filtering to the results using <code>FilterExpression</code>. If <code>LastEvaluatedKey</code> is present in the response, you will need to paginate the result set. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.html#Query.Pagination">Paginating the Results</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p> <p> <code>FilterExpression</code> is applied after a <code>Query</code> finishes, but before the results are returned. A <code>FilterExpression</code> cannot contain partition key or sort key attributes. You need to specify those attributes in the <code>KeyConditionExpression</code>. </p> <note> <p> A <code>Query</code> operation can return an empty result set and a <code>LastEvaluatedKey</code> if all the items read for the page of results are filtered out. </p> </note> <p>You can query a table, a local secondary index, or a global secondary index. For a query on a table or on a local secondary index, you can set the <code>ConsistentRead</code> parameter to <code>true</code> and obtain a strongly consistent result. Global secondary indexes support eventually consistent reads only, so do not specify <code>ConsistentRead</code> when querying a global secondary index.</p>
    fn query(&self, input: QueryInput) -> RusotoFuture<QueryOutput, QueryError>;

    /// <p><p>Creates a new table from an existing backup. Any number of users can execute up to 4 concurrent restores (any type of restore) in a given account. </p> <p>You can call <code>RestoreTableFromBackup</code> at a maximum rate of 10 times per second.</p> <p>You must manually set up the following on the restored table:</p> <ul> <li> <p>Auto scaling policies</p> </li> <li> <p>IAM policies</p> </li> <li> <p>Cloudwatch metrics and alarms</p> </li> <li> <p>Tags</p> </li> <li> <p>Stream settings</p> </li> <li> <p>Time to Live (TTL) settings</p> </li> </ul></p>
    fn restore_table_from_backup(
        &self,
        input: RestoreTableFromBackupInput,
    ) -> RusotoFuture<RestoreTableFromBackupOutput, RestoreTableFromBackupError>;

    /// <p><p>Restores the specified table to the specified point in time within <code>EarliestRestorableDateTime</code> and <code>LatestRestorableDateTime</code>. You can restore your table to any point in time during the last 35 days with a 1-minute granularity. Any number of users can execute up to 4 concurrent restores (any type of restore) in a given account. </p> <p>You must manually set up the following on the restored table:</p> <ul> <li> <p>Auto scaling policies</p> </li> <li> <p>IAM policies</p> </li> <li> <p>Cloudwatch metrics and alarms</p> </li> <li> <p>Tags</p> </li> <li> <p>Stream settings</p> </li> <li> <p>Time to Live (TTL) settings</p> </li> <li> <p>Point in time recovery settings</p> </li> </ul></p>
    fn restore_table_to_point_in_time(
        &self,
        input: RestoreTableToPointInTimeInput,
    ) -> RusotoFuture<RestoreTableToPointInTimeOutput, RestoreTableToPointInTimeError>;

    /// <p>The <code>Scan</code> operation returns one or more items and item attributes by accessing every item in a table or a secondary index. To have DynamoDB return fewer items, you can provide a <code>FilterExpression</code> operation.</p> <p>If the total number of scanned items exceeds the maximum data set size limit of 1 MB, the scan stops and results are returned to the user as a <code>LastEvaluatedKey</code> value to continue the scan in a subsequent operation. The results also include the number of items exceeding the limit. A scan can result in no table data meeting the filter criteria. </p> <p>A single <code>Scan</code> operation will read up to the maximum number of items set (if using the <code>Limit</code> parameter) or a maximum of 1 MB of data and then apply any filtering to the results using <code>FilterExpression</code>. If <code>LastEvaluatedKey</code> is present in the response, you will need to paginate the result set. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.Pagination">Paginating the Results</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p> <p> <code>Scan</code> operations proceed sequentially; however, for faster performance on a large table or secondary index, applications can request a parallel <code>Scan</code> operation by providing the <code>Segment</code> and <code>TotalSegments</code> parameters. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.ParallelScan">Parallel Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p> <code>Scan</code> uses eventually consistent reads when accessing the data in a table; therefore, the result set might not include the changes to data in the table immediately before the operation began. If you need a consistent copy of the data, as of the time that the <code>Scan</code> begins, you can set the <code>ConsistentRead</code> parameter to <code>true</code>.</p>
    fn scan(&self, input: ScanInput) -> RusotoFuture<ScanOutput, ScanError>;

    /// <p>Associate a set of tags with an Amazon DynamoDB resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. You can call TagResource up to 5 times per second, per account. </p> <p>For an overview on tagging DynamoDB resources, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html">Tagging for DynamoDB</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn tag_resource(&self, input: TagResourceInput) -> RusotoFuture<(), TagResourceError>;

    /// <p>Removes the association of tags from an Amazon DynamoDB resource. You can call UntagResource up to 5 times per second, per account. </p> <p>For an overview on tagging DynamoDB resources, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html">Tagging for DynamoDB</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn untag_resource(&self, input: UntagResourceInput) -> RusotoFuture<(), UntagResourceError>;

    /// <p> <code>UpdateContinuousBackups</code> enables or disables point in time recovery for the specified table. A successful <code>UpdateContinuousBackups</code> call returns the current <code>ContinuousBackupsDescription</code>. Continuous backups are <code>ENABLED</code> on all tables at table creation. If point in time recovery is enabled, <code>PointInTimeRecoveryStatus</code> will be set to ENABLED.</p> <p> Once continuous backups and point in time recovery are enabled, you can restore to any point in time within <code>EarliestRestorableDateTime</code> and <code>LatestRestorableDateTime</code>. </p> <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. You can restore your table to any point in time during the last 35 days with a 1-minute granularity. </p>
    fn update_continuous_backups(
        &self,
        input: UpdateContinuousBackupsInput,
    ) -> RusotoFuture<UpdateContinuousBackupsOutput, UpdateContinuousBackupsError>;

    /// <p><p>Adds or removes replicas in the specified global table. The global table must already exist to be able to use this operation. Any replica to be added must be empty, must have the same name as the global table, must have the same key schema, and must have DynamoDB Streams enabled.</p> <note> <p>Although you can use <code>UpdateGlobalTable</code> to add replicas and remove replicas in a single request, for simplicity we recommend that you issue separate requests for adding or removing replicas.</p> </note></p>
    fn update_global_table(
        &self,
        input: UpdateGlobalTableInput,
    ) -> RusotoFuture<UpdateGlobalTableOutput, UpdateGlobalTableError>;

    /// <p>Edits an existing item's attributes, or adds a new item to the table if it does not already exist. You can put, delete, or add attribute values. You can also perform a conditional update on an existing item (insert a new attribute name-value pair if it doesn't exist, or replace an existing name-value pair if it has certain expected attribute values).</p> <p>You can also return the item's attribute values in the same <code>UpdateItem</code> operation using the <code>ReturnValues</code> parameter.</p>
    fn update_item(
        &self,
        input: UpdateItemInput,
    ) -> RusotoFuture<UpdateItemOutput, UpdateItemError>;

    /// <p>Modifies the provisioned throughput settings, global secondary indexes, or DynamoDB Streams settings for a given table.</p> <p>You can only perform one of the following operations at once:</p> <ul> <li> <p>Modify the provisioned throughput settings of the table.</p> </li> <li> <p>Enable or disable Streams on the table.</p> </li> <li> <p>Remove a global secondary index from the table.</p> </li> <li> <p>Create a new global secondary index on the table. Once the index begins backfilling, you can use <code>UpdateTable</code> to perform other operations.</p> </li> </ul> <p> <code>UpdateTable</code> is an asynchronous operation; while it is executing, the table status changes from <code>ACTIVE</code> to <code>UPDATING</code>. While it is <code>UPDATING</code>, you cannot issue another <code>UpdateTable</code> request. When the table returns to the <code>ACTIVE</code> state, the <code>UpdateTable</code> operation is complete.</p>
    fn update_table(
        &self,
        input: UpdateTableInput,
    ) -> RusotoFuture<UpdateTableOutput, UpdateTableError>;

    /// <p>The UpdateTimeToLive method will enable or disable TTL for the specified table. A successful <code>UpdateTimeToLive</code> call returns the current <code>TimeToLiveSpecification</code>; it may take up to one hour for the change to fully process. Any additional <code>UpdateTimeToLive</code> calls for the same table during this one hour duration result in a <code>ValidationException</code>. </p> <p>TTL compares the current time in epoch time format to the time stored in the TTL attribute of an item. If the epoch time value stored in the attribute is less than the current time, the item is marked as expired and subsequently deleted.</p> <note> <p> The epoch time format is the number of seconds elapsed since 12:00:00 AM January 1st, 1970 UTC. </p> </note> <p>DynamoDB deletes expired items on a best-effort basis to ensure availability of throughput for other data operations. </p> <important> <p>DynamoDB typically deletes expired items within two days of expiration. The exact duration within which an item gets deleted after expiration is specific to the nature of the workload. Items that have expired and not been deleted will still show up in reads, queries, and scans.</p> </important> <p>As items are deleted, they are removed from any Local Secondary Index and Global Secondary Index immediately in the same eventually consistent way as a standard delete operation.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/TTL.html">Time To Live</a> in the Amazon DynamoDB Developer Guide. </p>
    fn update_time_to_live(
        &self,
        input: UpdateTimeToLiveInput,
    ) -> RusotoFuture<UpdateTimeToLiveOutput, UpdateTimeToLiveError>;
}
/// A client for the DynamoDB API.
pub struct DynamoDbClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl DynamoDbClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> DynamoDbClient {
        DynamoDbClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> DynamoDbClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DynamoDbClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> DynamoDb for DynamoDbClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>The <code>BatchGetItem</code> operation returns the attributes of one or more items from one or more tables. You identify requested items by primary key.</p> <p>A single operation can retrieve up to 16 MB of data, which can contain as many as 100 items. <code>BatchGetItem</code> will return a partial result if the response size limit is exceeded, the table's provisioned throughput is exceeded, or an internal processing failure occurs. If a partial result is returned, the operation returns a value for <code>UnprocessedKeys</code>. You can use this value to retry the operation starting with the next item to get.</p> <important> <p>If you request more than 100 items <code>BatchGetItem</code> will return a <code>ValidationException</code> with the message "Too many items requested for the BatchGetItem call".</p> </important> <p>For example, if you ask to retrieve 100 items, but each individual item is 300 KB in size, the system returns 52 items (so as not to exceed the 16 MB limit). It also returns an appropriate <code>UnprocessedKeys</code> value so you can get the next page of results. If desired, your application can include its own logic to assemble the pages of results into one data set.</p> <p>If <i>none</i> of the items can be processed due to insufficient provisioned throughput on all of the tables in the request, then <code>BatchGetItem</code> will return a <code>ProvisionedThroughputExceededException</code>. If <i>at least one</i> of the items is successfully processed, then <code>BatchGetItem</code> completes successfully, while returning the keys of the unread items in <code>UnprocessedKeys</code>.</p> <important> <p>If DynamoDB returns any unprocessed items, you should retry the batch operation on those items. However, <i>we strongly recommend that you use an exponential backoff algorithm</i>. If you retry the batch operation immediately, the underlying read or write requests can still fail due to throttling on the individual tables. If you delay the batch operation using exponential backoff, the individual requests in the batch are much more likely to succeed.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOperations">Batch Operations and Error Handling</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </important> <p>By default, <code>BatchGetItem</code> performs eventually consistent reads on every table in the request. If you want strongly consistent reads instead, you can set <code>ConsistentRead</code> to <code>true</code> for any or all tables.</p> <p>In order to minimize response latency, <code>BatchGetItem</code> retrieves items in parallel.</p> <p>When designing your application, keep in mind that DynamoDB does not return items in any particular order. To help parse the response by item, include the primary key values for the items in your request in the <code>ProjectionExpression</code> parameter.</p> <p>If a requested item does not exist, it is not returned in the result. Requests for nonexistent items consume the minimum read capacity units according to the type of read. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#CapacityUnitCalculations">Capacity Units Calculations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn batch_get_item(
        &self,
        input: BatchGetItemInput,
    ) -> RusotoFuture<BatchGetItemOutput, BatchGetItemError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.BatchGetItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchGetItemOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetItemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>The <code>BatchWriteItem</code> operation puts or deletes multiple items in one or more tables. A single call to <code>BatchWriteItem</code> can write up to 16 MB of data, which can comprise as many as 25 put or delete requests. Individual items to be written can be as large as 400 KB.</p> <note> <p> <code>BatchWriteItem</code> cannot update items. To update items, use the <code>UpdateItem</code> action.</p> </note> <p>The individual <code>PutItem</code> and <code>DeleteItem</code> operations specified in <code>BatchWriteItem</code> are atomic; however <code>BatchWriteItem</code> as a whole is not. If any requested operations fail because the table&#39;s provisioned throughput is exceeded or an internal processing failure occurs, the failed operations are returned in the <code>UnprocessedItems</code> response parameter. You can investigate and optionally resend the requests. Typically, you would call <code>BatchWriteItem</code> in a loop. Each iteration would check for unprocessed items and submit a new <code>BatchWriteItem</code> request with those unprocessed items until all items have been processed.</p> <p>Note that if <i>none</i> of the items can be processed due to insufficient provisioned throughput on all of the tables in the request, then <code>BatchWriteItem</code> will return a <code>ProvisionedThroughputExceededException</code>.</p> <important> <p>If DynamoDB returns any unprocessed items, you should retry the batch operation on those items. However, <i>we strongly recommend that you use an exponential backoff algorithm</i>. If you retry the batch operation immediately, the underlying read or write requests can still fail due to throttling on the individual tables. If you delay the batch operation using exponential backoff, the individual requests in the batch are much more likely to succeed.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOperations">Batch Operations and Error Handling</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </important> <p>With <code>BatchWriteItem</code>, you can efficiently write or delete large amounts of data, such as from Amazon Elastic MapReduce (EMR), or copy data from another database into DynamoDB. In order to improve performance with these large-scale operations, <code>BatchWriteItem</code> does not behave in the same way as individual <code>PutItem</code> and <code>DeleteItem</code> calls would. For example, you cannot specify conditions on individual put and delete requests, and <code>BatchWriteItem</code> does not return deleted items in the response.</p> <p>If you use a programming language that supports concurrency, you can use threads to write items in parallel. Your application must include the necessary logic to manage the threads. With languages that don&#39;t support threading, you must update or delete the specified items one at a time. In both situations, <code>BatchWriteItem</code> performs the specified put and delete operations in parallel, giving you the power of the thread pool approach without having to introduce complexity into your application.</p> <p>Parallel processing reduces latency, but each specified put and delete request consumes the same number of write capacity units whether it is processed in parallel or not. Delete operations on nonexistent items consume one write capacity unit.</p> <p>If one or more of the following is true, DynamoDB rejects the entire batch write operation:</p> <ul> <li> <p>One or more tables specified in the <code>BatchWriteItem</code> request does not exist.</p> </li> <li> <p>Primary key attributes specified on an item in the request do not match those in the corresponding table&#39;s primary key schema.</p> </li> <li> <p>You try to perform multiple operations on the same item in the same <code>BatchWriteItem</code> request. For example, you cannot put and delete the same item in the same <code>BatchWriteItem</code> request. </p> </li> <li> <p> Your request contains at least two items with identical hash and range keys (which essentially is two put operations). </p> </li> <li> <p>There are more than 25 requests in the batch.</p> </li> <li> <p>Any individual item in a batch exceeds 400 KB.</p> </li> <li> <p>The total request size exceeds 16 MB.</p> </li> </ul></p>
    fn batch_write_item(
        &self,
        input: BatchWriteItemInput,
    ) -> RusotoFuture<BatchWriteItemOutput, BatchWriteItemError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.BatchWriteItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchWriteItemOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(BatchWriteItemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a backup for an existing table.</p> <p> Each time you create an On-Demand Backup, the entire table data is backed up. There is no limit to the number of on-demand backups that can be taken. </p> <p> When you create an On-Demand Backup, a time marker of the request is cataloged, and the backup is created asynchronously, by applying all changes until the time of the request to the last full table snapshot. Backup requests are processed instantaneously and become available for restore within minutes. </p> <p>You can call <code>CreateBackup</code> at a maximum rate of 50 times per second.</p> <p>All backups in DynamoDB work without consuming any provisioned throughput on the table.</p> <p> If you submit a backup request on 2018-12-14 at 14:25:00, the backup is guaranteed to contain all data committed to the table up to 14:24:00, and data committed after 14:26:00 will not be. The backup may or may not contain data modifications made between 14:24:00 and 14:26:00. On-Demand Backup does not support causal consistency. </p> <p> Along with data, the following are also included on the backups: </p> <ul> <li> <p>Global secondary indexes (GSIs)</p> </li> <li> <p>Local secondary indexes (LSIs)</p> </li> <li> <p>Streams</p> </li> <li> <p>Provisioned read and write capacity</p> </li> </ul></p>
    fn create_backup(
        &self,
        input: CreateBackupInput,
    ) -> RusotoFuture<CreateBackupOutput, CreateBackupError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.CreateBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateBackupOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateBackupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a global table from an existing table. A global table creates a replication relationship between two or more DynamoDB tables with the same table name in the provided regions. </p> <p> Tables can only be added as the replicas of a global table group under the following conditions: </p> <ul> <li> <p> The tables must have the same name. </p> </li> <li> <p> The tables must contain no items. </p> </li> <li> <p> The tables must have the same hash key and sort key (if present). </p> </li> <li> <p> The tables must have DynamoDB Streams enabled (NEW<em>AND</em>OLD_IMAGES). </p> </li> </ul></p>
    fn create_global_table(
        &self,
        input: CreateGlobalTableInput,
    ) -> RusotoFuture<CreateGlobalTableOutput, CreateGlobalTableError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.CreateGlobalTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateGlobalTableOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateGlobalTableError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>CreateTable</code> operation adds a new table to your account. In an AWS account, table names must be unique within each region. That is, you can have two tables with same name if you create the tables in different regions.</p> <p> <code>CreateTable</code> is an asynchronous operation. Upon receiving a <code>CreateTable</code> request, DynamoDB immediately returns a response with a <code>TableStatus</code> of <code>CREATING</code>. After the table is created, DynamoDB sets the <code>TableStatus</code> to <code>ACTIVE</code>. You can perform read and write operations only on an <code>ACTIVE</code> table. </p> <p>You can optionally define secondary indexes on the new table, as part of the <code>CreateTable</code> operation. If you want to create multiple tables with secondary indexes on them, you must create the tables sequentially. Only one table with secondary indexes can be in the <code>CREATING</code> state at any given time.</p> <p>You can use the <code>DescribeTable</code> action to check the table status.</p>
    fn create_table(
        &self,
        input: CreateTableInput,
    ) -> RusotoFuture<CreateTableOutput, CreateTableError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.CreateTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateTableOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTableError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an existing backup of a table.</p> <p>You can call <code>DeleteBackup</code> at a maximum rate of 10 times per second.</p>
    fn delete_backup(
        &self,
        input: DeleteBackupInput,
    ) -> RusotoFuture<DeleteBackupOutput, DeleteBackupError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.DeleteBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteBackupOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteBackupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a single item in a table by primary key. You can perform a conditional delete operation that deletes the item if it exists, or if it has an expected attribute value.</p> <p>In addition to deleting an item, you can also return the item's attribute values in the same operation, using the <code>ReturnValues</code> parameter.</p> <p>Unless you specify conditions, the <code>DeleteItem</code> is an idempotent operation; running it multiple times on the same item or attribute does <i>not</i> result in an error response.</p> <p>Conditional deletes are useful for deleting items only if specific conditions are met. If those conditions are met, DynamoDB performs the delete. Otherwise, the item is not deleted.</p>
    fn delete_item(
        &self,
        input: DeleteItemInput,
    ) -> RusotoFuture<DeleteItemOutput, DeleteItemError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.DeleteItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteItemOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteItemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>DeleteTable</code> operation deletes a table and all of its items. After a <code>DeleteTable</code> request, the specified table is in the <code>DELETING</code> state until DynamoDB completes the deletion. If the table is in the <code>ACTIVE</code> state, you can delete it. If a table is in <code>CREATING</code> or <code>UPDATING</code> states, then DynamoDB returns a <code>ResourceInUseException</code>. If the specified table does not exist, DynamoDB returns a <code>ResourceNotFoundException</code>. If table is already in the <code>DELETING</code> state, no error is returned. </p> <note> <p>DynamoDB might continue to accept data read and write operations, such as <code>GetItem</code> and <code>PutItem</code>, on a table in the <code>DELETING</code> state until the table deletion is complete.</p> </note> <p>When you delete a table, any indexes on that table are also deleted.</p> <p>If you have DynamoDB Streams enabled on the table, then the corresponding stream on that table goes into the <code>DISABLED</code> state, and the stream is automatically deleted after 24 hours.</p> <p>Use the <code>DescribeTable</code> action to check the status of the table. </p>
    fn delete_table(
        &self,
        input: DeleteTableInput,
    ) -> RusotoFuture<DeleteTableOutput, DeleteTableError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.DeleteTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTableOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTableError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes an existing backup of a table.</p> <p>You can call <code>DescribeBackup</code> at a maximum rate of 10 times per second.</p>
    fn describe_backup(
        &self,
        input: DescribeBackupInput,
    ) -> RusotoFuture<DescribeBackupOutput, DescribeBackupError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.DescribeBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeBackupOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBackupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Checks the status of continuous backups and point in time recovery on the specified table. Continuous backups are <code>ENABLED</code> on all tables at table creation. If point in time recovery is enabled, <code>PointInTimeRecoveryStatus</code> will be set to ENABLED.</p> <p> Once continuous backups and point in time recovery are enabled, you can restore to any point in time within <code>EarliestRestorableDateTime</code> and <code>LatestRestorableDateTime</code>. </p> <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. You can restore your table to any point in time during the last 35 days with a 1-minute granularity. </p> <p>You can call <code>DescribeContinuousBackups</code> at a maximum rate of 10 times per second.</p>
    fn describe_continuous_backups(
        &self,
        input: DescribeContinuousBackupsInput,
    ) -> RusotoFuture<DescribeContinuousBackupsOutput, DescribeContinuousBackupsError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "DynamoDB_20120810.DescribeContinuousBackups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeContinuousBackupsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeContinuousBackupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the specified global table.</p>
    fn describe_global_table(
        &self,
        input: DescribeGlobalTableInput,
    ) -> RusotoFuture<DescribeGlobalTableOutput, DescribeGlobalTableError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.DescribeGlobalTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeGlobalTableOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeGlobalTableError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the current provisioned-capacity limits for your AWS account in a region, both for the region as a whole and for any one DynamoDB table that you create there.</p> <p>When you establish an AWS account, the account has initial limits on the maximum read capacity units and write capacity units that you can provision across all of your DynamoDB tables in a given region. Also, there are per-table limits that apply when you create a table there. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Limits</a> page in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Although you can increase these limits by filing a case at <a href="https://console.aws.amazon.com/support/home#/">AWS Support Center</a>, obtaining the increase is not instantaneous. The <code>DescribeLimits</code> action lets you write code to compare the capacity you are currently using to those limits imposed by your account so that you have enough time to apply for an increase before you hit a limit.</p> <p>For example, you could use one of the AWS SDKs to do the following:</p> <ol> <li> <p>Call <code>DescribeLimits</code> for a particular region to obtain your current account limits on provisioned capacity there.</p> </li> <li> <p>Create a variable to hold the aggregate read capacity units provisioned for all your tables in that region, and one to hold the aggregate write capacity units. Zero them both.</p> </li> <li> <p>Call <code>ListTables</code> to obtain a list of all your DynamoDB tables.</p> </li> <li> <p>For each table name listed by <code>ListTables</code>, do the following:</p> <ul> <li> <p>Call <code>DescribeTable</code> with the table name.</p> </li> <li> <p>Use the data returned by <code>DescribeTable</code> to add the read capacity units and write capacity units provisioned for the table itself to your variables.</p> </li> <li> <p>If the table has one or more global secondary indexes (GSIs), loop over these GSIs and add their provisioned capacity values to your variables as well.</p> </li> </ul> </li> <li> <p>Report the account limits for that region returned by <code>DescribeLimits</code>, along with the total current provisioned capacity levels you have calculated.</p> </li> </ol> <p>This will let you see whether you are getting close to your account-level limits.</p> <p>The per-table limits apply only when you are creating a new table. They restrict the sum of the provisioned capacity of the new table itself and all its global secondary indexes.</p> <p>For existing tables and their GSIs, DynamoDB will not let you increase provisioned capacity extremely rapidly, but the only upper limit that applies is that the aggregate provisioned capacity over all your tables and GSIs cannot exceed either of the per-account limits.</p> <note> <p> <code>DescribeLimits</code> should only be called periodically. You can expect throttling errors if you call it more than once in a minute.</p> </note> <p>The <code>DescribeLimits</code> Request element has no content.</p>
    fn describe_limits(&self) -> RusotoFuture<DescribeLimitsOutput, DescribeLimitsError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.DescribeLimits");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeLimitsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeLimitsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Returns information about the table, including the current status of the table, when it was created, the primary key schema, and any indexes on the table.</p> <note> <p>If you issue a <code>DescribeTable</code> request immediately after a <code>CreateTable</code> request, DynamoDB might return a <code>ResourceNotFoundException</code>. This is because <code>DescribeTable</code> uses an eventually consistent query, and the metadata for your table might not be available at that moment. Wait for a few seconds, and then try the <code>DescribeTable</code> request again.</p> </note></p>
    fn describe_table(
        &self,
        input: DescribeTableInput,
    ) -> RusotoFuture<DescribeTableOutput, DescribeTableError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.DescribeTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTableOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTableError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gives a description of the Time to Live (TTL) status on the specified table. </p>
    fn describe_time_to_live(
        &self,
        input: DescribeTimeToLiveInput,
    ) -> RusotoFuture<DescribeTimeToLiveOutput, DescribeTimeToLiveError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.DescribeTimeToLive");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTimeToLiveOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTimeToLiveError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>GetItem</code> operation returns a set of attributes for the item with the given primary key. If there is no matching item, <code>GetItem</code> does not return any data and there will be no <code>Item</code> element in the response.</p> <p> <code>GetItem</code> provides an eventually consistent read by default. If your application requires a strongly consistent read, set <code>ConsistentRead</code> to <code>true</code>. Although a strongly consistent read might take more time than an eventually consistent read, it always returns the last updated value.</p>
    fn get_item(&self, input: GetItemInput) -> RusotoFuture<GetItemOutput, GetItemError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.GetItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetItemOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetItemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>List backups associated with an AWS account. To list backups for a given table, specify <code>TableName</code>. <code>ListBackups</code> returns a paginated list of results with at most 1MB worth of items in a page. You can also specify a limit for the maximum number of entries to be returned in a page. </p> <p>In the request, start time is inclusive but end time is exclusive. Note that these limits are for the time at which the original backup was requested.</p> <p>You can call <code>ListBackups</code> a maximum of 5 times per second.</p>
    fn list_backups(
        &self,
        input: ListBackupsInput,
    ) -> RusotoFuture<ListBackupsOutput, ListBackupsError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.ListBackups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListBackupsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListBackupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all global tables that have a replica in the specified region.</p>
    fn list_global_tables(
        &self,
        input: ListGlobalTablesInput,
    ) -> RusotoFuture<ListGlobalTablesOutput, ListGlobalTablesError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.ListGlobalTables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListGlobalTablesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListGlobalTablesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns an array of table names associated with the current account and endpoint. The output from <code>ListTables</code> is paginated, with each page returning a maximum of 100 table names.</p>
    fn list_tables(
        &self,
        input: ListTablesInput,
    ) -> RusotoFuture<ListTablesOutput, ListTablesError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.ListTables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTablesOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTablesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>List all tags on an Amazon DynamoDB resource. You can call ListTagsOfResource up to 10 times per second, per account.</p> <p>For an overview on tagging DynamoDB resources, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html">Tagging for DynamoDB</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn list_tags_of_resource(
        &self,
        input: ListTagsOfResourceInput,
    ) -> RusotoFuture<ListTagsOfResourceOutput, ListTagsOfResourceError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.ListTagsOfResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsOfResourceOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsOfResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a new item, or replaces an old item with a new item. If an item that has the same primary key as the new item already exists in the specified table, the new item completely replaces the existing item. You can perform a conditional put operation (add a new item if one with the specified primary key doesn't exist), or replace an existing item if it has certain attribute values. You can return the item's attribute values in the same operation, using the <code>ReturnValues</code> parameter.</p> <important> <p>This topic provides general information about the <code>PutItem</code> API.</p> <p>For information on how to call the <code>PutItem</code> API using the AWS SDK in specific languages, see the following:</p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/goto/aws-cli/dynamodb-2012-08-10/PutItem"> PutItem in the AWS Command Line Interface </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/DotNetSDKV3/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for .NET </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForCpp/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for C++ </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForGoV1/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for Go </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForJava/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for Java </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/AWSJavaScriptSDK/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for JavaScript </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForPHPV3/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for PHP V3 </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/boto3/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for Python </a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/goto/SdkForRubyV2/dynamodb-2012-08-10/PutItem"> PutItem in the AWS SDK for Ruby V2 </a> </p> </li> </ul> </important> <p>When you add an item, the primary key attribute(s) are the only required attributes. Attribute values cannot be null. String and Binary type attributes must have lengths greater than zero. Set type attributes cannot be empty. Requests with empty values will be rejected with a <code>ValidationException</code> exception.</p> <note> <p>To prevent a new item from replacing an existing item, use a conditional expression that contains the <code>attribute_not_exists</code> function with the name of the attribute being used as the partition key for the table. Since every record must contain that attribute, the <code>attribute_not_exists</code> function will only succeed if no matching item exists.</p> </note> <p>For more information about <code>PutItem</code>, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithItems.html">Working with Items</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn put_item(&self, input: PutItemInput) -> RusotoFuture<PutItemOutput, PutItemError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.PutItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutItemOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutItemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>Query</code> operation finds items based on primary key values. You can query any table or secondary index that has a composite primary key (a partition key and a sort key). </p> <p>Use the <code>KeyConditionExpression</code> parameter to provide a specific value for the partition key. The <code>Query</code> operation will return all of the items from the table or index with that partition key value. You can optionally narrow the scope of the <code>Query</code> operation by specifying a sort key value and a comparison operator in <code>KeyConditionExpression</code>. To further refine the <code>Query</code> results, you can optionally provide a <code>FilterExpression</code>. A <code>FilterExpression</code> determines which items within the results should be returned to you. All of the other results are discarded. </p> <p> A <code>Query</code> operation always returns a result set. If no matching items are found, the result set will be empty. Queries that do not return results consume the minimum number of read capacity units for that type of read operation. </p> <note> <p> DynamoDB calculates the number of read capacity units consumed based on item size, not on the amount of data that is returned to an application. The number of capacity units consumed will be the same whether you request all of the attributes (the default behavior) or just some of them (using a projection expression). The number will also be the same whether or not you use a <code>FilterExpression</code>. </p> </note> <p> <code>Query</code> results are always sorted by the sort key value. If the data type of the sort key is Number, the results are returned in numeric order; otherwise, the results are returned in order of UTF-8 bytes. By default, the sort order is ascending. To reverse the order, set the <code>ScanIndexForward</code> parameter to false. </p> <p> A single <code>Query</code> operation will read up to the maximum number of items set (if using the <code>Limit</code> parameter) or a maximum of 1 MB of data and then apply any filtering to the results using <code>FilterExpression</code>. If <code>LastEvaluatedKey</code> is present in the response, you will need to paginate the result set. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.html#Query.Pagination">Paginating the Results</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p> <p> <code>FilterExpression</code> is applied after a <code>Query</code> finishes, but before the results are returned. A <code>FilterExpression</code> cannot contain partition key or sort key attributes. You need to specify those attributes in the <code>KeyConditionExpression</code>. </p> <note> <p> A <code>Query</code> operation can return an empty result set and a <code>LastEvaluatedKey</code> if all the items read for the page of results are filtered out. </p> </note> <p>You can query a table, a local secondary index, or a global secondary index. For a query on a table or on a local secondary index, you can set the <code>ConsistentRead</code> parameter to <code>true</code> and obtain a strongly consistent result. Global secondary indexes support eventually consistent reads only, so do not specify <code>ConsistentRead</code> when querying a global secondary index.</p>
    fn query(&self, input: QueryInput) -> RusotoFuture<QueryOutput, QueryError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.Query");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<QueryOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(QueryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a new table from an existing backup. Any number of users can execute up to 4 concurrent restores (any type of restore) in a given account. </p> <p>You can call <code>RestoreTableFromBackup</code> at a maximum rate of 10 times per second.</p> <p>You must manually set up the following on the restored table:</p> <ul> <li> <p>Auto scaling policies</p> </li> <li> <p>IAM policies</p> </li> <li> <p>Cloudwatch metrics and alarms</p> </li> <li> <p>Tags</p> </li> <li> <p>Stream settings</p> </li> <li> <p>Time to Live (TTL) settings</p> </li> </ul></p>
    fn restore_table_from_backup(
        &self,
        input: RestoreTableFromBackupInput,
    ) -> RusotoFuture<RestoreTableFromBackupOutput, RestoreTableFromBackupError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.RestoreTableFromBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RestoreTableFromBackupOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RestoreTableFromBackupError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Restores the specified table to the specified point in time within <code>EarliestRestorableDateTime</code> and <code>LatestRestorableDateTime</code>. You can restore your table to any point in time during the last 35 days with a 1-minute granularity. Any number of users can execute up to 4 concurrent restores (any type of restore) in a given account. </p> <p>You must manually set up the following on the restored table:</p> <ul> <li> <p>Auto scaling policies</p> </li> <li> <p>IAM policies</p> </li> <li> <p>Cloudwatch metrics and alarms</p> </li> <li> <p>Tags</p> </li> <li> <p>Stream settings</p> </li> <li> <p>Time to Live (TTL) settings</p> </li> <li> <p>Point in time recovery settings</p> </li> </ul></p>
    fn restore_table_to_point_in_time(
        &self,
        input: RestoreTableToPointInTimeInput,
    ) -> RusotoFuture<RestoreTableToPointInTimeOutput, RestoreTableToPointInTimeError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header(
            "x-amz-target",
            "DynamoDB_20120810.RestoreTableToPointInTime",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RestoreTableToPointInTimeOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RestoreTableToPointInTimeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The <code>Scan</code> operation returns one or more items and item attributes by accessing every item in a table or a secondary index. To have DynamoDB return fewer items, you can provide a <code>FilterExpression</code> operation.</p> <p>If the total number of scanned items exceeds the maximum data set size limit of 1 MB, the scan stops and results are returned to the user as a <code>LastEvaluatedKey</code> value to continue the scan in a subsequent operation. The results also include the number of items exceeding the limit. A scan can result in no table data meeting the filter criteria. </p> <p>A single <code>Scan</code> operation will read up to the maximum number of items set (if using the <code>Limit</code> parameter) or a maximum of 1 MB of data and then apply any filtering to the results using <code>FilterExpression</code>. If <code>LastEvaluatedKey</code> is present in the response, you will need to paginate the result set. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.Pagination">Paginating the Results</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p> <p> <code>Scan</code> operations proceed sequentially; however, for faster performance on a large table or secondary index, applications can request a parallel <code>Scan</code> operation by providing the <code>Segment</code> and <code>TotalSegments</code> parameters. For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Scan.html#Scan.ParallelScan">Parallel Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p> <code>Scan</code> uses eventually consistent reads when accessing the data in a table; therefore, the result set might not include the changes to data in the table immediately before the operation began. If you need a consistent copy of the data, as of the time that the <code>Scan</code> begins, you can set the <code>ConsistentRead</code> parameter to <code>true</code>.</p>
    fn scan(&self, input: ScanInput) -> RusotoFuture<ScanOutput, ScanError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.Scan");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ScanOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ScanError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associate a set of tags with an Amazon DynamoDB resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. You can call TagResource up to 5 times per second, per account. </p> <p>For an overview on tagging DynamoDB resources, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html">Tagging for DynamoDB</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn tag_resource(&self, input: TagResourceInput) -> RusotoFuture<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the association of tags from an Amazon DynamoDB resource. You can call UntagResource up to 5 times per second, per account. </p> <p>For an overview on tagging DynamoDB resources, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html">Tagging for DynamoDB</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    fn untag_resource(&self, input: UntagResourceInput) -> RusotoFuture<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> <code>UpdateContinuousBackups</code> enables or disables point in time recovery for the specified table. A successful <code>UpdateContinuousBackups</code> call returns the current <code>ContinuousBackupsDescription</code>. Continuous backups are <code>ENABLED</code> on all tables at table creation. If point in time recovery is enabled, <code>PointInTimeRecoveryStatus</code> will be set to ENABLED.</p> <p> Once continuous backups and point in time recovery are enabled, you can restore to any point in time within <code>EarliestRestorableDateTime</code> and <code>LatestRestorableDateTime</code>. </p> <p> <code>LatestRestorableDateTime</code> is typically 5 minutes before the current time. You can restore your table to any point in time during the last 35 days with a 1-minute granularity. </p>
    fn update_continuous_backups(
        &self,
        input: UpdateContinuousBackupsInput,
    ) -> RusotoFuture<UpdateContinuousBackupsOutput, UpdateContinuousBackupsError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.UpdateContinuousBackups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateContinuousBackupsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateContinuousBackupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Adds or removes replicas in the specified global table. The global table must already exist to be able to use this operation. Any replica to be added must be empty, must have the same name as the global table, must have the same key schema, and must have DynamoDB Streams enabled.</p> <note> <p>Although you can use <code>UpdateGlobalTable</code> to add replicas and remove replicas in a single request, for simplicity we recommend that you issue separate requests for adding or removing replicas.</p> </note></p>
    fn update_global_table(
        &self,
        input: UpdateGlobalTableInput,
    ) -> RusotoFuture<UpdateGlobalTableOutput, UpdateGlobalTableError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.UpdateGlobalTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateGlobalTableOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateGlobalTableError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Edits an existing item's attributes, or adds a new item to the table if it does not already exist. You can put, delete, or add attribute values. You can also perform a conditional update on an existing item (insert a new attribute name-value pair if it doesn't exist, or replace an existing name-value pair if it has certain expected attribute values).</p> <p>You can also return the item's attribute values in the same <code>UpdateItem</code> operation using the <code>ReturnValues</code> parameter.</p>
    fn update_item(
        &self,
        input: UpdateItemInput,
    ) -> RusotoFuture<UpdateItemOutput, UpdateItemError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.UpdateItem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateItemOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateItemError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Modifies the provisioned throughput settings, global secondary indexes, or DynamoDB Streams settings for a given table.</p> <p>You can only perform one of the following operations at once:</p> <ul> <li> <p>Modify the provisioned throughput settings of the table.</p> </li> <li> <p>Enable or disable Streams on the table.</p> </li> <li> <p>Remove a global secondary index from the table.</p> </li> <li> <p>Create a new global secondary index on the table. Once the index begins backfilling, you can use <code>UpdateTable</code> to perform other operations.</p> </li> </ul> <p> <code>UpdateTable</code> is an asynchronous operation; while it is executing, the table status changes from <code>ACTIVE</code> to <code>UPDATING</code>. While it is <code>UPDATING</code>, you cannot issue another <code>UpdateTable</code> request. When the table returns to the <code>ACTIVE</code> state, the <code>UpdateTable</code> operation is complete.</p>
    fn update_table(
        &self,
        input: UpdateTableInput,
    ) -> RusotoFuture<UpdateTableOutput, UpdateTableError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.UpdateTable");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateTableOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTableError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>The UpdateTimeToLive method will enable or disable TTL for the specified table. A successful <code>UpdateTimeToLive</code> call returns the current <code>TimeToLiveSpecification</code>; it may take up to one hour for the change to fully process. Any additional <code>UpdateTimeToLive</code> calls for the same table during this one hour duration result in a <code>ValidationException</code>. </p> <p>TTL compares the current time in epoch time format to the time stored in the TTL attribute of an item. If the epoch time value stored in the attribute is less than the current time, the item is marked as expired and subsequently deleted.</p> <note> <p> The epoch time format is the number of seconds elapsed since 12:00:00 AM January 1st, 1970 UTC. </p> </note> <p>DynamoDB deletes expired items on a best-effort basis to ensure availability of throughput for other data operations. </p> <important> <p>DynamoDB typically deletes expired items within two days of expiration. The exact duration within which an item gets deleted after expiration is specific to the nature of the workload. Items that have expired and not been deleted will still show up in reads, queries, and scans.</p> </important> <p>As items are deleted, they are removed from any Local Secondary Index and Global Secondary Index immediately in the same eventually consistent way as a standard delete operation.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/TTL.html">Time To Live</a> in the Amazon DynamoDB Developer Guide. </p>
    fn update_time_to_live(
        &self,
        input: UpdateTimeToLiveInput,
    ) -> RusotoFuture<UpdateTimeToLiveOutput, UpdateTimeToLiveError> {
        let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "DynamoDB_20120810.UpdateTimeToLive");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateTimeToLiveOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateTimeToLiveError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
