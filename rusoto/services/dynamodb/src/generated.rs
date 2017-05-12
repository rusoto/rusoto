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
pub type AttributeAction = String;
#[doc="<p>Represents an attribute for describing the key schema for the table and indexes.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct AttributeDefinition {
                #[doc="<p>A name for the attribute.</p>"]
#[serde(rename="AttributeName")]
pub attribute_name: KeySchemaAttributeName,
#[doc="<p>The data type for the attribute, where:</p> <ul> <li><p> <code>S</code> - the attribute is of type String</p> </li> <li><p> <code>N</code> - the attribute is of type Number</p> </li> <li><p> <code>B</code> - the attribute is of type Binary</p> </li> </ul>"]
#[serde(rename="AttributeType")]
pub attribute_type: ScalarAttributeType,
            }
            
pub type AttributeDefinitions = Vec<AttributeDefinition>;
pub type AttributeMap = ::std::collections::HashMap<AttributeName, AttributeValue>;
pub type AttributeName = String;
pub type AttributeNameList = Vec<AttributeName>;
pub type AttributeUpdates = ::std::collections::HashMap<AttributeName, AttributeValueUpdate>;
#[doc="<p>Represents the data for an attribute. You can set one, and only one, of the elements.</p> <p>Each attribute in an item is a name-value pair. An attribute can be single-valued or multi-valued set. For example, a book item can have title and authors attributes. Each book has one title but can have many authors. The multi-valued attribute is a set; duplicate values are not allowed.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct AttributeValue {
                #[doc="<p>A Binary data type.</p>"]
#[serde(rename="B")]
#[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
pub b: Option<BinaryAttributeValue>,
#[doc="<p>A Boolean data type.</p>"]
#[serde(rename="BOOL")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub bool: Option<BooleanAttributeValue>,
#[doc="<p>A Binary Set data type.</p>"]
#[serde(rename="BS")]
pub bs: Option<BinarySetAttributeValue>,
#[doc="<p>A List of attribute values.</p>"]
#[serde(rename="L")]
pub l: Option<ListAttributeValue>,
#[doc="<p>A Map of attribute values.</p>"]
#[serde(rename="M")]
pub m: Option<MapAttributeValue>,
#[doc="<p>A Number data type.</p>"]
#[serde(rename="N")]
pub n: Option<NumberAttributeValue>,
#[doc="<p>A Number Set data type.</p>"]
#[serde(rename="NS")]
pub ns: Option<NumberSetAttributeValue>,
#[doc="<p>A Null data type.</p>"]
#[serde(rename="NULL")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub null: Option<NullAttributeValue>,
#[doc="<p>A String data type.</p>"]
#[serde(rename="S")]
pub s: Option<StringAttributeValue>,
#[doc="<p>A String Set data type.</p>"]
#[serde(rename="SS")]
pub ss: Option<StringSetAttributeValue>,
            }
            
pub type AttributeValueList = Vec<AttributeValue>;
#[doc="<p>For the <i>UpdateItem</i> operation, represents the attributes to be modified, the action to perform on each, and the new value for each.</p> <note> <p>You cannot use <i>UpdateItem</i> to update any primary key attributes. Instead, you will need to delete the item, and then use <i>PutItem</i> to create a new item with new attributes.</p> </note> <p>Attribute values cannot be null; string and binary type attributes must have lengths greater than zero; and set type attributes must not be empty. Requests with empty values will be rejected with a <i>ValidationException</i> exception.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AttributeValueUpdate {
                #[doc="<p>Specifies how to perform the update. Valid values are <code>PUT</code> (default), <code>DELETE</code>, and <code>ADD</code>. The behavior depends on whether the specified primary key already exists in the table.</p> <p> <b>If an item with the specified <i>Key</i> is found in the table:</b> </p> <ul> <li> <p> <code>PUT</code> - Adds the specified attribute to the item. If the attribute already exists, it is replaced by the new value. </p> </li> <li> <p> <code>DELETE</code> - If no value is specified, the attribute and its value are removed from the item. The data type of the specified value must match the existing value's data type.</p> <p>If a <i>set</i> of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>[a,b,c]</code> and the <i>DELETE</i> action specified <code>[a,c]</code>, then the final attribute value would be <code>[b]</code>. Specifying an empty set is an error.</p> </li> <li> <p> <code>ADD</code> - If the attribute does not already exist, then the attribute and its values are added to the item. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p> <ul> <li> <p>If the existing attribute is a number, and if <i>Value</i> is also a number, then the <i>Value</i> is mathematically added to the existing attribute. If <i>Value</i> is a negative number, then it is subtracted from the existing attribute.</p> <note> <p> If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses 0 as the initial value.</p> <p>In addition, if you use <code>ADD</code> to update an existing item, and intend to increment or decrement an attribute value which does not yet exist, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update does not yet have an attribute named <i>itemcount</i>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway, even though it currently does not exist. DynamoDB will create the <i>itemcount</i> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <i>itemcount</i> attribute in the item, with a value of <code>3</code>.</p> </note> </li> <li> <p>If the existing data type is a set, and if the <i>Value</i> is also a set, then the <i>Value</i> is added to the existing set. (This is a <i>set</i> operation, not mathematical addition.) For example, if the attribute value was the set <code>[1,2]</code>, and the <code>ADD</code> action specified <code>[3]</code>, then the final attribute value would be <code>[1,2,3]</code>. An error occurs if an Add action is specified for a set attribute and the attribute type specified does not match the existing set type. </p> <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <i>Value</i> must also be a set of strings. The same holds true for number sets and binary sets.</p> </li> </ul> <p>This action is only valid for an existing attribute whose data type is number or is a set. Do not use <code>ADD</code> for any other data types.</p> </li> </ul> <p> <b>If no item with the specified <i>Key</i> is found:</b> </p> <ul> <li> <p> <code>PUT</code> - DynamoDB creates a new item with the specified primary key, and then adds the attribute. </p> </li> <li> <p> <code>DELETE</code> - Nothing happens; there is no attribute to delete.</p> </li> <li> <p> <code>ADD</code> - DynamoDB creates an item with the supplied primary key and number (or set of numbers) for the attribute value. The only data types allowed are number and number set; no other data types can be specified.</p> </li> </ul>"]
#[serde(rename="Action")]
pub action: Option<AttributeAction>,
#[serde(rename="Value")]
pub value: Option<AttributeValue>,
            }
            
pub type Backfilling = bool;
#[doc="<p>Represents the input of a <i>BatchGetItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct BatchGetItemInput {
                #[doc="<p>A map of one or more table names and, for each table, a map that describes one or more items to retrieve from that table. Each table name can be used only once per <i>BatchGetItem</i> request.</p> <p>Each element in the map of items to retrieve consists of the following:</p> <ul> <li> <p> <i>ConsistentRead</i> - If <code>true</code>, a strongly consistent read is used; if <code>false</code> (the default), an eventually consistent read is used.</p> </li> <li> <p> <i>ExpressionAttributeNames</i> - One or more substitution tokens for attribute names in the <i>ProjectionExpression</i> parameter. The following are some use cases for using <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p> <code>{\"#P\":\"Percentile\"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </li> <li> <p> <i>Keys</i> - An array of primary key attribute values that define specific items in the table. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key value. For a composite key, you must provide <i>both</i> the partition key value and the sort key value.</p> </li> <li> <p> <i>ProjectionExpression</i> - A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </li> <li> <p> <i>AttributesToGet</i> - </p> <important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ProjectionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> <p>This parameter allows you to retrieve attributes of type List or Map; however, it cannot retrieve individual elements within a List or a Map.</p> </important> <p>The names of one or more attributes to retrieve. If no attribute names are provided, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>Note that <i>AttributesToGet</i> has no effect on provisioned throughput consumption. DynamoDB determines capacity units consumed based on item size, not on the amount of data that is returned to an application.</p> </li> </ul>"]
#[serde(rename="RequestItems")]
pub request_items: BatchGetRequestMap,
#[serde(rename="ReturnConsumedCapacity")]
pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
            }
            
#[doc="<p>Represents the output of a <i>BatchGetItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BatchGetItemOutput {
                #[doc="<p>The read capacity units consumed by the operation.</p> <p>Each element consists of:</p> <ul> <li> <p> <i>TableName</i> - The table that consumed the provisioned throughput.</p> </li> <li> <p> <i>CapacityUnits</i> - The total number of capacity units consumed.</p> </li> </ul>"]
#[serde(rename="ConsumedCapacity")]
pub consumed_capacity: Option<ConsumedCapacityMultiple>,
#[doc="<p>A map of table name to a list of items. Each object in <i>Responses</i> consists of a table name, along with a map of attribute data consisting of the data type and attribute value.</p>"]
#[serde(rename="Responses")]
pub responses: Option<BatchGetResponseMap>,
#[doc="<p>A map of tables and their respective keys that were not processed with the current response. The <i>UnprocessedKeys</i> value is in the same form as <i>RequestItems</i>, so the value can be provided directly to a subsequent <i>BatchGetItem</i> operation. For more information, see <i>RequestItems</i> in the Request Parameters section.</p> <p>Each element consists of:</p> <ul> <li> <p> <i>Keys</i> - An array of primary key attribute values that define specific items in the table.</p> </li> <li> <p> <i>AttributesToGet</i> - One or more attributes to be retrieved from the table or index. By default, all attributes are returned. If a requested attribute is not found, it does not appear in the result.</p> </li> <li> <p> <i>ConsistentRead</i> - The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p> </li> </ul> <p>If there are no unprocessed keys remaining, the response contains an empty <i>UnprocessedKeys</i> map.</p>"]
#[serde(rename="UnprocessedKeys")]
pub unprocessed_keys: Option<BatchGetRequestMap>,
            }
            
pub type BatchGetRequestMap = ::std::collections::HashMap<TableName, KeysAndAttributes>;
pub type BatchGetResponseMap = ::std::collections::HashMap<TableName, ItemList>;
#[doc="<p>Represents the input of a <i>BatchWriteItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct BatchWriteItemInput {
                #[doc="<p>A map of one or more table names and, for each table, a list of operations to be performed (<i>DeleteRequest</i> or <i>PutRequest</i>). Each element in the map consists of the following:</p> <ul> <li> <p> <i>DeleteRequest</i> - Perform a <i>DeleteItem</i> operation on the specified item. The item to be deleted is identified by a <i>Key</i> subelement:</p> <ul> <li> <p> <i>Key</i> - A map of primary key attribute values that uniquely identify the ! item. Each entry in this map consists of an attribute name and an attribute value. For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for <i>both</i> the partition key and the sort key.</p> </li> </ul> </li> <li> <p> <i>PutRequest</i> - Perform a <i>PutItem</i> operation on the specified item. The item to be put is identified by an <i>Item</i> subelement:</p> <ul> <li> <p> <i>Item</i> - A map of attributes and their values. Each entry in this map consists of an attribute name and an attribute value. Attribute values must not be null; string and binary type attributes must have lengths greater than zero; and set type attributes must not be empty. Requests that contain empty values will be rejected with a <i>ValidationException</i> exception.</p> <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p> </li> </ul> </li> </ul>"]
#[serde(rename="RequestItems")]
pub request_items: BatchWriteItemRequestMap,
#[serde(rename="ReturnConsumedCapacity")]
pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
#[doc="<p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>"]
#[serde(rename="ReturnItemCollectionMetrics")]
pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
            }
            
#[doc="<p>Represents the output of a <i>BatchWriteItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct BatchWriteItemOutput {
                #[doc="<p>The capacity units consumed by the operation.</p> <p>Each element consists of:</p> <ul> <li> <p> <i>TableName</i> - The table that consumed the provisioned throughput.</p> </li> <li> <p> <i>CapacityUnits</i> - The total number of capacity units consumed.</p> </li> </ul>"]
#[serde(rename="ConsumedCapacity")]
pub consumed_capacity: Option<ConsumedCapacityMultiple>,
#[doc="<p>A list of tables that were processed by <i>BatchWriteItem</i> and, for each table, information about any item collections that were affected by individual <i>DeleteItem</i> or <i>PutItem</i> operations.</p> <p>Each entry consists of the following subelements:</p> <ul> <li> <p> <i>ItemCollectionKey</i> - The partition key value of the item collection. This is the same as the partition key value of the item.</p> </li> <li> <p> <i>SizeEstimateRange</i> - An estimate of item collection size, expressed in GB. This is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on the table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li> </ul>"]
#[serde(rename="ItemCollectionMetrics")]
pub item_collection_metrics: Option<ItemCollectionMetricsPerTable>,
#[doc="<p>A map of tables and requests against those tables that were not processed. The <i>UnprocessedItems</i> value is in the same form as <i>RequestItems</i>, so you can provide this value directly to a subsequent <i>BatchGetItem</i> operation. For more information, see <i>RequestItems</i> in the Request Parameters section.</p> <p>Each <i>UnprocessedItems</i> entry consists of a table name and, for that table, a list of operations to perform (<i>DeleteRequest</i> or <i>PutRequest</i>).</p> <ul> <li> <p> <i>DeleteRequest</i> - Perform a <i>DeleteItem</i> operation on the specified item. The item to be deleted is identified by a <i>Key</i> subelement:</p> <ul> <li> <p> <i>Key</i> - A map of primary key attribute values that uniquely identify the item. Each entry in this map consists of an attribute name and an attribute value.</p> </li> </ul> </li> <li> <p> <i>PutRequest</i> - Perform a <i>PutItem</i> operation on the specified item. The item to be put is identified by an <i>Item</i> subelement:</p> <ul> <li> <p> <i>Item</i> - A map of attributes and their values. Each entry in this map consists of an attribute name and an attribute value. Attribute values must not be null; string and binary type attributes must have lengths greater than zero; and set type attributes must not be empty. Requests that contain empty values will be rejected with a <i>ValidationException</i> exception.</p> <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p> </li> </ul> </li> </ul> <p>If there are no unprocessed items remaining, the response contains an empty <i>UnprocessedItems</i> map.</p>"]
#[serde(rename="UnprocessedItems")]
pub unprocessed_items: Option<BatchWriteItemRequestMap>,
            }
            
pub type BatchWriteItemRequestMap = ::std::collections::HashMap<TableName, WriteRequests>;
pub type BinaryAttributeValue = Vec<u8>;
pub type BinarySetAttributeValue = Vec<BinaryAttributeValue>;
pub type BooleanAttributeValue = bool;
pub type BooleanObject = bool;
#[doc="<p>Represents the amount of provisioned throughput capacity consumed on a table or an index.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Capacity {
                #[doc="<p>The total number of capacity units consumed on a table or an index.</p>"]
#[serde(rename="CapacityUnits")]
pub capacity_units: Option<ConsumedCapacityUnits>,
            }
            
pub type ComparisonOperator = String;
#[doc="<p>Represents the selection criteria for a <i>Query</i> or <i>Scan</i> operation:</p> <ul> <li> <p>For a <i>Query</i> operation, <i>Condition</i> is used for specifying the <i>KeyConditions</i> to use when querying a table or an index. For <i>KeyConditions</i>, only the following comparison operators are supported:</p> <p> <code>EQ | LE | LT | GE | GT | BEGINS_WITH | BETWEEN</code> </p> <p> <i>Condition</i> is also used in a <i>QueryFilter</i>, which evaluates the query results and returns only the desired values.</p> </li> <li> <p>For a <i>Scan</i> operation, <i>Condition</i> is used in a <i>ScanFilter</i>, which evaluates the scan results and returns only the desired values.</p> </li> </ul>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct Condition {
                #[doc="<p>One or more values to evaluate against the supplied attribute. The number of values in the list depends on the <i>ComparisonOperator</i> being used.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href=\"http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters\">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p>"]
#[serde(rename="AttributeValueList")]
pub attribute_value_list: Option<AttributeValueList>,
#[doc="<p>A comparator for evaluating attributes. For example, equals, greater than, less than, etc.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS | BEGINS_WITH | IN | BETWEEN</code> </p> <p>The following are descriptions of each comparison operator.</p> <ul> <li> <p> <code>EQ</code> : Equal. <code>EQ</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NE</code> : Not equal. <code>NE</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LE</code> : Less than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LT</code> : Less than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GE</code> : Greater than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GT</code> : Greater than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NOT_NULL</code> : The attribute exists. <code>NOT_NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the existence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NOT_NULL</code>, the result is a Boolean <i>true</i>. This result is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NOT_NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>NULL</code> : The attribute does not exist. <code>NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the nonexistence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NULL</code>, the result is a Boolean <i>false</i>. This is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>CONTAINS</code> : Checks for a subsequence, or value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is of type String, then the operator checks for a substring match. If the target attribute of the comparison is of type Binary, then the operator looks for a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it finds an exact match with any member of the set.</p> <p>CONTAINS is supported for lists: When evaluating \"<code>a CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>NOT_CONTAINS</code> : Checks for absence of a subsequence, or absence of a value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is a String, then the operator checks for the absence of a substring match. If the target attribute of the comparison is Binary, then the operator checks for the absence of a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it <i>does not</i> find an exact match with any member of the set.</p> <p>NOT_CONTAINS is supported for lists: When evaluating \"<code>a NOT CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>BEGINS_WITH</code> : Checks for a prefix. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String or Binary (not a Number or a set type). The target attribute of the comparison must be of type String or Binary (not a Number or a set type).</p> <p/> </li> <li> <p> <code>IN</code> : Checks for matching elements within two sets.</p> <p> <i>AttributeValueList</i> can contain one or more <i>AttributeValue</i> elements of type String, Number, or Binary (not a set type). These attributes are compared against an existing set type attribute of an item. If any elements of the input set are present in the item attribute, the expression evaluates to true.</p> </li> <li> <p> <code>BETWEEN</code> : Greater than or equal to the first value, and less than or equal to the second value. </p> <p> <i>AttributeValueList</i> must contain two <i>AttributeValue</i> elements of the same type, either String, Number, or Binary (not a set type). A target attribute matches if the target value is greater than, or equal to, the first element and less than, or equal to, the second element. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not compare to <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code> </p> </li> </ul> <p>For usage examples of <i>AttributeValueList</i> and <i>ComparisonOperator</i>, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.html\">Legacy Conditional Parameters</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ComparisonOperator")]
pub comparison_operator: ComparisonOperator,
            }
            
pub type ConditionExpression = String;
pub type ConditionalOperator = String;
pub type ConsistentRead = bool;
#[doc="<p>The capacity units consumed by an operation. The data returned includes the total provisioned throughput consumed, along with statistics for the table and any indexes involved in the operation. <i>ConsumedCapacity</i> is only returned if the request asked for it. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughputIntro.html\">Provisioned Throughput</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ConsumedCapacity {
                #[doc="<p>The total number of capacity units consumed by the operation.</p>"]
#[serde(rename="CapacityUnits")]
pub capacity_units: Option<ConsumedCapacityUnits>,
#[doc="<p>The amount of throughput consumed on each global index affected by the operation.</p>"]
#[serde(rename="GlobalSecondaryIndexes")]
pub global_secondary_indexes: Option<SecondaryIndexesCapacityMap>,
#[doc="<p>The amount of throughput consumed on each local index affected by the operation.</p>"]
#[serde(rename="LocalSecondaryIndexes")]
pub local_secondary_indexes: Option<SecondaryIndexesCapacityMap>,
#[doc="<p>The amount of throughput consumed on the table affected by the operation.</p>"]
#[serde(rename="Table")]
pub table: Option<Capacity>,
#[doc="<p>The name of the table that was affected by the operation.</p>"]
#[serde(rename="TableName")]
pub table_name: Option<TableName>,
            }
            
pub type ConsumedCapacityMultiple = Vec<ConsumedCapacity>;
pub type ConsumedCapacityUnits = f64;
#[doc="<p>Represents a new global secondary index to be added to an existing table.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateGlobalSecondaryIndexAction {
                #[doc="<p>The name of the global secondary index to be created.</p>"]
#[serde(rename="IndexName")]
pub index_name: IndexName,
#[doc="<p>The key schema for the global secondary index.</p>"]
#[serde(rename="KeySchema")]
pub key_schema: KeySchema,
#[serde(rename="Projection")]
pub projection: Projection,
#[serde(rename="ProvisionedThroughput")]
pub provisioned_throughput: ProvisionedThroughput,
            }
            
#[doc="<p>Represents the input of a <i>CreateTable</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateTableInput {
                #[doc="<p>An array of attributes that describe the key schema for the table and indexes.</p>"]
#[serde(rename="AttributeDefinitions")]
pub attribute_definitions: AttributeDefinitions,
#[doc="<p>One or more global secondary indexes (the maximum is five) to be created on the table. Each global secondary index in the array includes the following:</p> <ul> <li> <p> <i>IndexName</i> - The name of the global secondary index. Must be unique only for this table.</p> <p/> </li> <li> <p> <i>KeySchema</i> - Specifies the key schema for the global secondary index.</p> </li> <li> <p> <i>Projection</i> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p> <ul> <li> <p> <i>ProjectionType</i> - One of the following:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <i>NonKeyAttributes</i>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul> </li> <li> <p> <i>NonKeyAttributes</i> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <i>NonKeyAttributes</i>, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p> </li> </ul> </li> <li> <p> <i>ProvisionedThroughput</i> - The provisioned throughput settings for the global secondary index, consisting of read and write capacity units.</p> </li> </ul>"]
#[serde(rename="GlobalSecondaryIndexes")]
pub global_secondary_indexes: Option<GlobalSecondaryIndexList>,
#[doc="<p>Specifies the attributes that make up the primary key for a table or an index. The attributes in <i>KeySchema</i> must also be defined in the <i>AttributeDefinitions</i> array. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html\">Data Model</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Each <i>KeySchemaElement</i> in the array is composed of:</p> <ul> <li> <p> <i>AttributeName</i> - The name of this key attribute.</p> </li> <li> <p> <i>KeyType</i> - The role that the key attribute will assume:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term \"hash attribute\" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term \"range attribute\" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note> <p>For a simple primary key (partition key), you must provide exactly one element with a <i>KeyType</i> of <code>HASH</code>.</p> <p>For a composite primary key (partition key and sort key), you must provide exactly two elements, in this order: The first element must have a <i>KeyType</i> of <code>HASH</code>, and the second element must have a <i>KeyType</i> of <code>RANGE</code>.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTables.primary.key\">Specifying the Primary Key</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="KeySchema")]
pub key_schema: KeySchema,
#[doc="<p>One or more local secondary indexes (the maximum is five) to be created on the table. Each index is scoped to a given partition key value. There is a 10 GB size limit per partition key value; otherwise, the size of a local secondary index is unconstrained.</p> <p>Each local secondary index in the array includes the following:</p> <ul> <li> <p> <i>IndexName</i> - The name of the local secondary index. Must be unique only for this table.</p> <p/> </li> <li> <p> <i>KeySchema</i> - Specifies the key schema for the local secondary index. The key schema must begin with the same partition key as the table.</p> </li> <li> <p> <i>Projection</i> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p> <ul> <li> <p> <i>ProjectionType</i> - One of the following:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <i>NonKeyAttributes</i>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul> </li> <li> <p> <i>NonKeyAttributes</i> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <i>NonKeyAttributes</i>, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p> </li> </ul> </li> </ul>"]
#[serde(rename="LocalSecondaryIndexes")]
pub local_secondary_indexes: Option<LocalSecondaryIndexList>,
#[serde(rename="ProvisionedThroughput")]
pub provisioned_throughput: ProvisionedThroughput,
#[doc="<p>The settings for DynamoDB Streams on the table. These settings consist of:</p> <ul> <li> <p> <i>StreamEnabled</i> - Indicates whether Streams is to be enabled (true) or disabled (false).</p> </li> <li> <p> <i>StreamViewType</i> - When an item in the table is modified, <i>StreamViewType</i> determines what information is written to the table's stream. Valid values for <i>StreamViewType</i> are:</p> <ul> <li> <p> <i>KEYS_ONLY</i> - Only the key attributes of the modified item are written to the stream.</p> </li> <li> <p> <i>NEW_IMAGE</i> - The entire item, as it appears after it was modified, is written to the stream.</p> </li> <li> <p> <i>OLD_IMAGE</i> - The entire item, as it appeared before it was modified, is written to the stream.</p> </li> <li> <p> <i>NEW_AND_OLD_IMAGES</i> - Both the new and the old item images of the item are written to the stream.</p> </li> </ul> </li> </ul>"]
#[serde(rename="StreamSpecification")]
pub stream_specification: Option<StreamSpecification>,
#[doc="<p>The name of the table to create.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
            }
            
#[doc="<p>Represents the output of a <i>CreateTable</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateTableOutput {
                #[serde(rename="TableDescription")]
pub table_description: Option<TableDescription>,
            }
            
pub type Date = f64;
#[doc="<p>Represents a global secondary index to be deleted from an existing table.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteGlobalSecondaryIndexAction {
                #[doc="<p>The name of the global secondary index to be deleted.</p>"]
#[serde(rename="IndexName")]
pub index_name: IndexName,
            }
            
#[doc="<p>Represents the input of a <i>DeleteItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteItemInput {
                #[doc="<p>A condition that must be satisfied in order for a conditional <i>DeleteItem</i> to succeed.</p> <p>An expression can contain any of the following:</p> <ul> <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li> <li> <p>Comparison operators: <code> = | &amp;#x3C;&amp;#x3E; | &amp;#x3C; | &amp;#x3E; | &amp;#x3C;= | &amp;#x3E;= | BETWEEN | IN</code> </p> </li> <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li> </ul> <p>For more information on condition expressions, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html\">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>ConditionExpression</i> replaces the legacy <i>ConditionalOperator</i> and <i>Expected</i> parameters.</p> </note>"]
#[serde(rename="ConditionExpression")]
pub condition_expression: Option<ConditionExpression>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ConditionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A logical operator to apply to the conditions in the <i>Expected</i> map:</p> <ul> <li> <p> <code>AND</code> - If all of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> <li> <p> <code>OR</code> - If at least one of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> </ul> <p>If you omit <i>ConditionalOperator</i>, then <code>AND</code> is the default.</p> <p>The operation will succeed only if the entire map evaluates to true.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note>"]
#[serde(rename="ConditionalOperator")]
pub conditional_operator: Option<ConditionalOperator>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ConditionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A map of attribute/condition pairs. <i>Expected</i> provides a conditional block for the <i>DeleteItem</i> operation.</p> <p>Each element of <i>Expected</i> consists of an attribute name, a comparison operator, and one or more values. DynamoDB compares the attribute with the value(s) you supplied, using the comparison operator. For each <i>Expected</i> element, the result of the evaluation is either true or false.</p> <p>If you specify more than one element in the <i>Expected</i> map, then by default all of the conditions must evaluate to true. In other words, the conditions are ANDed together. (You can use the <i>ConditionalOperator</i> parameter to OR the conditions instead. If you do this, then at least one of the conditions must evaluate to true, rather than all of them.)</p> <p>If the <i>Expected</i> map evaluates to true, then the conditional operation succeeds; otherwise, it fails.</p> <p> <i>Expected</i> contains the following:</p> <ul> <li> <p> <i>AttributeValueList</i> - One or more values to evaluate against the supplied attribute. The number of values in the list depends on the <i>ComparisonOperator</i> being used.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href=\"http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters\">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For type Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p> </li> <li> <p> <i>ComparisonOperator</i> - A comparator for evaluating attributes in the <i>AttributeValueList</i>. When performing the comparison, DynamoDB uses strongly consistent reads.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS | BEGINS_WITH | IN | BETWEEN</code> </p> <p>The following are descriptions of each comparison operator.</p> <ul> <li> <p> <code>EQ</code> : Equal. <code>EQ</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NE</code> : Not equal. <code>NE</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LE</code> : Less than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LT</code> : Less than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GE</code> : Greater than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GT</code> : Greater than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NOT_NULL</code> : The attribute exists. <code>NOT_NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the existence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NOT_NULL</code>, the result is a Boolean <i>true</i>. This result is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NOT_NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>NULL</code> : The attribute does not exist. <code>NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the nonexistence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NULL</code>, the result is a Boolean <i>false</i>. This is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>CONTAINS</code> : Checks for a subsequence, or value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is of type String, then the operator checks for a substring match. If the target attribute of the comparison is of type Binary, then the operator looks for a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it finds an exact match with any member of the set.</p> <p>CONTAINS is supported for lists: When evaluating \"<code>a CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>NOT_CONTAINS</code> : Checks for absence of a subsequence, or absence of a value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is a String, then the operator checks for the absence of a substring match. If the target attribute of the comparison is Binary, then the operator checks for the absence of a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it <i>does not</i> find an exact match with any member of the set.</p> <p>NOT_CONTAINS is supported for lists: When evaluating \"<code>a NOT CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>BEGINS_WITH</code> : Checks for a prefix. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String or Binary (not a Number or a set type). The target attribute of the comparison must be of type String or Binary (not a Number or a set type).</p> <p/> </li> <li> <p> <code>IN</code> : Checks for matching elements within two sets.</p> <p> <i>AttributeValueList</i> can contain one or more <i>AttributeValue</i> elements of type String, Number, or Binary (not a set type). These attributes are compared against an existing set type attribute of an item. If any elements of the input set are present in the item attribute, the expression evaluates to true.</p> </li> <li> <p> <code>BETWEEN</code> : Greater than or equal to the first value, and less than or equal to the second value. </p> <p> <i>AttributeValueList</i> must contain two <i>AttributeValue</i> elements of the same type, either String, Number, or Binary (not a set type). A target attribute matches if the target value is greater than, or equal to, the first element and less than, or equal to, the second element. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not compare to <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code> </p> </li> </ul> </li> </ul> <p>For usage examples of <i>AttributeValueList</i> and <i>ComparisonOperator</i>, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.html\">Legacy Conditional Parameters</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>For backward compatibility with previous DynamoDB releases, the following parameters can be used instead of <i>AttributeValueList</i> and <i>ComparisonOperator</i>:</p> <ul> <li> <p> <i>Value</i> - A value for DynamoDB to compare with an attribute.</p> </li> <li> <p> <i>Exists</i> - A Boolean value that causes DynamoDB to evaluate the value before attempting the conditional operation:</p> <ul> <li> <p>If <i>Exists</i> is <code>true</code>, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the condition evaluates to true; otherwise the condition evaluate to false.</p> </li> <li> <p>If <i>Exists</i> is <code>false</code>, DynamoDB assumes that the attribute value does <i>not</i> exist in the table. If in fact the value does not exist, then the assumption is valid and the condition evaluates to true. If the value is found, despite the assumption that it does not exist, the condition evaluates to false.</p> </li> </ul> <p>Note that the default value for <i>Exists</i> is <code>true</code>.</p> </li> </ul> <p>The <i>Value</i> and <i>Exists</i> parameters are incompatible with <i>AttributeValueList</i> and <i>ComparisonOperator</i>. Note that if you use both sets of parameters at once, DynamoDB will return a <i>ValidationException</i> exception.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note>"]
#[serde(rename="Expected")]
pub expected: Option<ExpectedAttributeMap>,
#[doc="<p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p> <code>{\"#P\":\"Percentile\"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeNames")]
pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
#[doc="<p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <i>ExpressionAttributeValues</i> as follows:</p> <p> <code>{ \":avail\":{\"S\":\"Available\"}, \":back\":{\"S\":\"Backordered\"}, \":disc\":{\"S\":\"Discontinued\"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html\">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeValues")]
pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
#[doc="<p>A map of attribute names to <i>AttributeValue</i> objects, representing the primary key of the item to delete.</p> <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>"]
#[serde(rename="Key")]
pub key: Key,
#[serde(rename="ReturnConsumedCapacity")]
pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
#[doc="<p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>"]
#[serde(rename="ReturnItemCollectionMetrics")]
pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
#[doc="<p>Use <i>ReturnValues</i> if you want to get the item attributes as they appeared before they were deleted. For <i>DeleteItem</i>, the valid values are:</p> <ul> <li> <p> <code>NONE</code> - If <i>ReturnValues</i> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <i>ReturnValues</i>.)</p> </li> <li> <p> <code>ALL_OLD</code> - The content of the old item is returned.</p> </li> </ul> <note> <p>The <i>ReturnValues</i> parameter is used by several DynamoDB operations; however, <i>DeleteItem</i> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p> </note>"]
#[serde(rename="ReturnValues")]
pub return_values: Option<ReturnValue>,
#[doc="<p>The name of the table from which to delete the item.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
            }
            
#[doc="<p>Represents the output of a <i>DeleteItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteItemOutput {
                #[doc="<p>A map of attribute names to <i>AttributeValue</i> objects, representing the item as it appeared before the <i>DeleteItem</i> operation. This map appears in the response only if <i>ReturnValues</i> was specified as <code>ALL_OLD</code> in the request.</p>"]
#[serde(rename="Attributes")]
pub attributes: Option<AttributeMap>,
#[serde(rename="ConsumedCapacity")]
pub consumed_capacity: Option<ConsumedCapacity>,
#[doc="<p>Information about item collections, if any, that were affected by the operation. <i>ItemCollectionMetrics</i> is only returned if the request asked for it. If the table does not have any local secondary indexes, this information is not returned in the response.</p> <p>Each <i>ItemCollectionMetrics</i> element consists of:</p> <ul> <li> <p> <i>ItemCollectionKey</i> - The partition key value of the item collection. This is the same as the partition key value of the item itself.</p> </li> <li> <p> <i>SizeEstimateRange</i> - An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li> </ul>"]
#[serde(rename="ItemCollectionMetrics")]
pub item_collection_metrics: Option<ItemCollectionMetrics>,
            }
            
#[doc="<p>Represents a request to perform a <i>DeleteItem</i> operation on an item.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct DeleteRequest {
                #[doc="<p>A map of attribute name to attribute values, representing the primary key of the item to delete. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema.</p>"]
#[serde(rename="Key")]
pub key: Key,
            }
            
#[doc="<p>Represents the input of a <i>DeleteTable</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteTableInput {
                #[doc="<p>The name of the table to delete.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
            }
            
#[doc="<p>Represents the output of a <i>DeleteTable</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteTableOutput {
                #[serde(rename="TableDescription")]
pub table_description: Option<TableDescription>,
            }
            
#[doc="<p>Represents the input of a <i>DescribeLimits</i> operation. Has no content.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeLimitsInput;
            
#[doc="<p>Represents the output of a <i>DescribeLimits</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeLimitsOutput {
                #[doc="<p>The maximum total read capacity units that your account allows you to provision across all of your tables in this region.</p>"]
#[serde(rename="AccountMaxReadCapacityUnits")]
pub account_max_read_capacity_units: Option<PositiveLongObject>,
#[doc="<p>The maximum total write capacity units that your account allows you to provision across all of your tables in this region.</p>"]
#[serde(rename="AccountMaxWriteCapacityUnits")]
pub account_max_write_capacity_units: Option<PositiveLongObject>,
#[doc="<p>The maximum read capacity units that your account allows you to provision for a new table that you are creating in this region, including the read capacity units provisioned for its global secondary indexes (GSIs).</p>"]
#[serde(rename="TableMaxReadCapacityUnits")]
pub table_max_read_capacity_units: Option<PositiveLongObject>,
#[doc="<p>The maximum write capacity units that your account allows you to provision for a new table that you are creating in this region, including the write capacity units provisioned for its global secondary indexes (GSIs).</p>"]
#[serde(rename="TableMaxWriteCapacityUnits")]
pub table_max_write_capacity_units: Option<PositiveLongObject>,
            }
            
#[doc="<p>Represents the input of a <i>DescribeTable</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeTableInput {
                #[doc="<p>The name of the table to describe.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
            }
            
#[doc="<p>Represents the output of a <i>DescribeTable</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeTableOutput {
                #[serde(rename="Table")]
pub table: Option<TableDescription>,
            }
            
pub type ErrorMessage = String;
pub type ExpectedAttributeMap = ::std::collections::HashMap<AttributeName, ExpectedAttributeValue>;
#[doc="<p>Represents a condition to be compared with an attribute value. This condition can be used with <i>DeleteItem</i>, <i>PutItem</i> or <i>UpdateItem</i> operations; if the comparison evaluates to true, the operation succeeds; if not, the operation fails. You can use <i>ExpectedAttributeValue</i> in one of two different ways:</p> <ul> <li> <p>Use <i>AttributeValueList</i> to specify one or more values to compare against an attribute. Use <i>ComparisonOperator</i> to specify how you want to perform the comparison. If the comparison evaluates to true, then the conditional operation succeeds.</p> </li> <li> <p>Use <i>Value</i> to specify a value that DynamoDB will compare against an attribute. If the values match, then <i>ExpectedAttributeValue</i> evaluates to true and the conditional operation succeeds. Optionally, you can also set <i>Exists</i> to false, indicating that you <i>do not</i> expect to find the attribute value in the table. In this case, the conditional operation succeeds only if the comparison evaluates to false.</p> </li> </ul> <p> <i>Value</i> and <i>Exists</i> are incompatible with <i>AttributeValueList</i> and <i>ComparisonOperator</i>. Note that if you use both sets of parameters at once, DynamoDB will return a <i>ValidationException</i> exception.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ExpectedAttributeValue {
                #[doc="<p>One or more values to evaluate against the supplied attribute. The number of values in the list depends on the <i>ComparisonOperator</i> being used.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href=\"http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters\">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p> <p>For information on specifying data types in JSON, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataFormat.html\">JSON Data Format</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="AttributeValueList")]
pub attribute_value_list: Option<AttributeValueList>,
#[doc="<p>A comparator for evaluating attributes in the <i>AttributeValueList</i>. For example, equals, greater than, less than, etc.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS | BEGINS_WITH | IN | BETWEEN</code> </p> <p>The following are descriptions of each comparison operator.</p> <ul> <li> <p> <code>EQ</code> : Equal. <code>EQ</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NE</code> : Not equal. <code>NE</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LE</code> : Less than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LT</code> : Less than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GE</code> : Greater than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GT</code> : Greater than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NOT_NULL</code> : The attribute exists. <code>NOT_NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the existence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NOT_NULL</code>, the result is a Boolean <i>true</i>. This result is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NOT_NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>NULL</code> : The attribute does not exist. <code>NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the nonexistence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NULL</code>, the result is a Boolean <i>false</i>. This is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>CONTAINS</code> : Checks for a subsequence, or value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is of type String, then the operator checks for a substring match. If the target attribute of the comparison is of type Binary, then the operator looks for a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it finds an exact match with any member of the set.</p> <p>CONTAINS is supported for lists: When evaluating \"<code>a CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>NOT_CONTAINS</code> : Checks for absence of a subsequence, or absence of a value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is a String, then the operator checks for the absence of a substring match. If the target attribute of the comparison is Binary, then the operator checks for the absence of a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it <i>does not</i> find an exact match with any member of the set.</p> <p>NOT_CONTAINS is supported for lists: When evaluating \"<code>a NOT CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>BEGINS_WITH</code> : Checks for a prefix. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String or Binary (not a Number or a set type). The target attribute of the comparison must be of type String or Binary (not a Number or a set type).</p> <p/> </li> <li> <p> <code>IN</code> : Checks for matching elements within two sets.</p> <p> <i>AttributeValueList</i> can contain one or more <i>AttributeValue</i> elements of type String, Number, or Binary (not a set type). These attributes are compared against an existing set type attribute of an item. If any elements of the input set are present in the item attribute, the expression evaluates to true.</p> </li> <li> <p> <code>BETWEEN</code> : Greater than or equal to the first value, and less than or equal to the second value. </p> <p> <i>AttributeValueList</i> must contain two <i>AttributeValue</i> elements of the same type, either String, Number, or Binary (not a set type). A target attribute matches if the target value is greater than, or equal to, the first element and less than, or equal to, the second element. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not compare to <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code> </p> </li> </ul>"]
#[serde(rename="ComparisonOperator")]
pub comparison_operator: Option<ComparisonOperator>,
#[doc="<p>Causes DynamoDB to evaluate the value before attempting a conditional operation:</p> <ul> <li> <p>If <i>Exists</i> is <code>true</code>, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the operation succeeds. If it is not found, the operation fails with a <i>ConditionalCheckFailedException</i>.</p> </li> <li> <p>If <i>Exists</i> is <code>false</code>, DynamoDB assumes that the attribute value does not exist in the table. If in fact the value does not exist, then the assumption is valid and the operation succeeds. If the value is found, despite the assumption that it does not exist, the operation fails with a <i>ConditionalCheckFailedException</i>.</p> </li> </ul> <p>The default setting for <i>Exists</i> is <code>true</code>. If you supply a <i>Value</i> all by itself, DynamoDB assumes the attribute exists: You don't have to set <i>Exists</i> to <code>true</code>, because it is implied.</p> <p>DynamoDB returns a <i>ValidationException</i> if:</p> <ul> <li> <p> <i>Exists</i> is <code>true</code> but there is no <i>Value</i> to check. (You expect a value to exist, but don't specify what that value is.)</p> </li> <li> <p> <i>Exists</i> is <code>false</code> but you also provide a <i>Value</i>. (You cannot expect an attribute to have a value, while also expecting it not to exist.)</p> </li> </ul>"]
#[serde(rename="Exists")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub exists: Option<BooleanObject>,
#[serde(rename="Value")]
pub value: Option<AttributeValue>,
            }
            
pub type ExpressionAttributeNameMap = ::std::collections::HashMap<ExpressionAttributeNameVariable, AttributeName>;
pub type ExpressionAttributeNameVariable = String;
pub type ExpressionAttributeValueMap = ::std::collections::HashMap<ExpressionAttributeValueVariable, AttributeValue>;
pub type ExpressionAttributeValueVariable = String;
pub type FilterConditionMap = ::std::collections::HashMap<AttributeName, Condition>;
#[doc="<p>Represents the input of a <i>GetItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetItemInput {
                #[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ProjectionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> <p>This parameter allows you to retrieve attributes of type List or Map; however, it cannot retrieve individual elements within a List or a Map.</p> </important> <p>The names of one or more attributes to retrieve. If no attribute names are provided, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>Note that <i>AttributesToGet</i> has no effect on provisioned throughput consumption. DynamoDB determines capacity units consumed based on item size, not on the amount of data that is returned to an application.</p>"]
#[serde(rename="AttributesToGet")]
pub attributes_to_get: Option<AttributeNameList>,
#[doc="<p>Determines the read consistency model: If set to <code>true</code>, then the operation uses strongly consistent reads; otherwise, the operation uses eventually consistent reads.</p>"]
#[serde(rename="ConsistentRead")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub consistent_read: Option<ConsistentRead>,
#[doc="<p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p> <code>{\"#P\":\"Percentile\"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeNames")]
pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
#[doc="<p>A map of attribute names to <i>AttributeValue</i> objects, representing the primary key of the item to retrieve.</p> <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>"]
#[serde(rename="Key")]
pub key: Key,
#[doc="<p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>ProjectionExpression</i> replaces the legacy <i>AttributesToGet</i> parameter.</p> </note>"]
#[serde(rename="ProjectionExpression")]
pub projection_expression: Option<ProjectionExpression>,
#[serde(rename="ReturnConsumedCapacity")]
pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
#[doc="<p>The name of the table containing the requested item.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
            }
            
#[doc="<p>Represents the output of a <i>GetItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetItemOutput {
                #[serde(rename="ConsumedCapacity")]
pub consumed_capacity: Option<ConsumedCapacity>,
#[doc="<p>A map of attribute names to <i>AttributeValue</i> objects, as specified by <i>AttributesToGet</i>.</p>"]
#[serde(rename="Item")]
pub item: Option<AttributeMap>,
            }
            
#[doc="<p>Represents the properties of a global secondary index.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GlobalSecondaryIndex {
                #[doc="<p>The name of the global secondary index. The name must be unique among all other indexes on this table.</p>"]
#[serde(rename="IndexName")]
pub index_name: IndexName,
#[doc="<p>The complete key schema for a global secondary index, which consists of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term \"hash attribute\" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term \"range attribute\" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note>"]
#[serde(rename="KeySchema")]
pub key_schema: KeySchema,
#[serde(rename="Projection")]
pub projection: Projection,
#[serde(rename="ProvisionedThroughput")]
pub provisioned_throughput: ProvisionedThroughput,
            }
            
#[doc="<p>Represents the properties of a global secondary index.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GlobalSecondaryIndexDescription {
                #[doc="<p>Indicates whether the index is currently backfilling. <i>Backfilling</i> is the process of reading items from the table and determining whether they can be added to the index. (Not all items will qualify: For example, a partition key cannot have any duplicate values.) If an item can be added to the index, DynamoDB will do so. After all items have been processed, the backfilling operation is complete and <i>Backfilling</i> is false.</p> <note> <p>For indexes that were created during a <i>CreateTable</i> operation, the <i>Backfilling</i> attribute does not appear in the <i>DescribeTable</i> output.</p> </note>"]
#[serde(rename="Backfilling")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub backfilling: Option<Backfilling>,
#[doc="<p>The Amazon Resource Name (ARN) that uniquely identifies the index.</p>"]
#[serde(rename="IndexArn")]
pub index_arn: Option<String>,
#[doc="<p>The name of the global secondary index.</p>"]
#[serde(rename="IndexName")]
pub index_name: Option<IndexName>,
#[doc="<p>The total size of the specified index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>"]
#[serde(rename="IndexSizeBytes")]
pub index_size_bytes: Option<Long>,
#[doc="<p>The current state of the global secondary index:</p> <ul> <li> <p> <i>CREATING</i> - The index is being created.</p> </li> <li> <p> <i>UPDATING</i> - The index is being updated.</p> </li> <li> <p> <i>DELETING</i> - The index is being deleted.</p> </li> <li> <p> <i>ACTIVE</i> - The index is ready for use.</p> </li> </ul>"]
#[serde(rename="IndexStatus")]
pub index_status: Option<IndexStatus>,
#[doc="<p>The number of items in the specified index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>"]
#[serde(rename="ItemCount")]
pub item_count: Option<Long>,
#[doc="<p>The complete key schema for a global secondary index, which consists of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term \"hash attribute\" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term \"range attribute\" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note>"]
#[serde(rename="KeySchema")]
pub key_schema: Option<KeySchema>,
#[serde(rename="Projection")]
pub projection: Option<Projection>,
#[serde(rename="ProvisionedThroughput")]
pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
            }
            
pub type GlobalSecondaryIndexDescriptionList = Vec<GlobalSecondaryIndexDescription>;
pub type GlobalSecondaryIndexList = Vec<GlobalSecondaryIndex>;
#[doc="<p>Represents one of the following:</p> <ul> <li> <p>A new global secondary index to be added to an existing table.</p> </li> <li> <p>New provisioned throughput parameters for an existing global secondary index.</p> </li> <li> <p>An existing global secondary index to be removed from an existing table.</p> </li> </ul>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GlobalSecondaryIndexUpdate {
                #[doc="<p>The parameters required for creating a global secondary index on an existing table:</p> <ul> <li> <p> <code>IndexName </code> </p> </li> <li> <p> <code>KeySchema </code> </p> </li> <li> <p> <code>AttributeDefinitions </code> </p> </li> <li> <p> <code>Projection </code> </p> </li> <li> <p> <code>ProvisionedThroughput </code> </p> </li> </ul>"]
#[serde(rename="Create")]
pub create: Option<CreateGlobalSecondaryIndexAction>,
#[doc="<p>The name of an existing global secondary index to be removed.</p>"]
#[serde(rename="Delete")]
pub delete: Option<DeleteGlobalSecondaryIndexAction>,
#[doc="<p>The name of an existing global secondary index, along with new provisioned throughput settings to be applied to that index.</p>"]
#[serde(rename="Update")]
pub update: Option<UpdateGlobalSecondaryIndexAction>,
            }
            
pub type GlobalSecondaryIndexUpdateList = Vec<GlobalSecondaryIndexUpdate>;
pub type IndexName = String;
pub type IndexStatus = String;
pub type Integer = i64;
pub type ItemCollectionKeyAttributeMap = ::std::collections::HashMap<AttributeName, AttributeValue>;
#[doc="<p>Information about item collections, if any, that were affected by the operation. <i>ItemCollectionMetrics</i> is only returned if the request asked for it. If the table does not have any local secondary indexes, this information is not returned in the response.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ItemCollectionMetrics {
                #[doc="<p>The partition key value of the item collection. This value is the same as the partition key value of the item.</p>"]
#[serde(rename="ItemCollectionKey")]
pub item_collection_key: Option<ItemCollectionKeyAttributeMap>,
#[doc="<p>An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p>"]
#[serde(rename="SizeEstimateRangeGB")]
pub size_estimate_range_gb: Option<ItemCollectionSizeEstimateRange>,
            }
            
pub type ItemCollectionMetricsMultiple = Vec<ItemCollectionMetrics>;
pub type ItemCollectionMetricsPerTable = ::std::collections::HashMap<TableName, ItemCollectionMetricsMultiple>;
pub type ItemCollectionSizeEstimateBound = f64;
pub type ItemCollectionSizeEstimateRange = Vec<ItemCollectionSizeEstimateBound>;
pub type ItemList = Vec<AttributeMap>;
pub type Key = ::std::collections::HashMap<AttributeName, AttributeValue>;
pub type KeyConditions = ::std::collections::HashMap<AttributeName, Condition>;
pub type KeyExpression = String;
pub type KeyList = Vec<Key>;
pub type KeySchema = Vec<KeySchemaElement>;
pub type KeySchemaAttributeName = String;
#[doc="<p>Represents <i>a single element</i> of a key schema. A key schema specifies the attributes that make up the primary key of a table, or the key attributes of an index.</p> <p>A <i>KeySchemaElement</i> represents exactly one attribute of the primary key. For example, a simple primary key would be represented by one <i>KeySchemaElement</i> (for the partition key). A composite primary key would require one <i>KeySchemaElement</i> for the partition key, and another <i>KeySchemaElement</i> for the sort key.</p> <p>A <i>KeySchemaElement</i> must be a scalar, top-level attribute (not a nested attribute). The data type must be one of String, Number, or Binary. The attribute cannot be nested within a List or a Map.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct KeySchemaElement {
                #[doc="<p>The name of a key attribute.</p>"]
#[serde(rename="AttributeName")]
pub attribute_name: KeySchemaAttributeName,
#[doc="<p>The role that this key attribute will assume:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term \"hash attribute\" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term \"range attribute\" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note>"]
#[serde(rename="KeyType")]
pub key_type: KeyType,
            }
            
pub type KeyType = String;
#[doc="<p>Represents a set of primary keys and, for each key, the attributes to retrieve from the table.</p> <p>For each primary key, you must provide <i>all</i> of the key attributes. For example, with a simple primary key, you only need to provide the partition key. For a composite primary key, you must provide <i>both</i> the partition key and the sort key.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct KeysAndAttributes {
                #[doc="<p>One or more attributes to retrieve from the table or index. If no attribute names are specified then all attributes will be returned. If any of the specified attributes are not found, they will not appear in the result.</p>"]
#[serde(rename="AttributesToGet")]
pub attributes_to_get: Option<AttributeNameList>,
#[doc="<p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>"]
#[serde(rename="ConsistentRead")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub consistent_read: Option<ConsistentRead>,
#[doc="<p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p> <code>{\"#P\":\"Percentile\"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeNames")]
pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
#[doc="<p>The primary key attribute values that define the items and the attributes associated with the items.</p>"]
#[serde(rename="Keys")]
pub keys: KeyList,
#[doc="<p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the <i>ProjectionExpression</i> must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>ProjectionExpression</i> replaces the legacy <i>AttributesToGet</i> parameter.</p> </note>"]
#[serde(rename="ProjectionExpression")]
pub projection_expression: Option<ProjectionExpression>,
            }
            
pub type ListAttributeValue = Vec<AttributeValue>;
#[doc="<p>Represents the input of a <i>ListTables</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListTablesInput {
                #[doc="<p>The first table name that this operation will evaluate. Use the value that was returned for <i>LastEvaluatedTableName</i> in a previous operation, so that you can obtain the next page of results.</p>"]
#[serde(rename="ExclusiveStartTableName")]
pub exclusive_start_table_name: Option<TableName>,
#[doc="<p>A maximum number of table names to return. If this parameter is not specified, the limit is 100.</p>"]
#[serde(rename="Limit")]
pub limit: Option<ListTablesInputLimit>,
            }
            
pub type ListTablesInputLimit = i64;
#[doc="<p>Represents the output of a <i>ListTables</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListTablesOutput {
                #[doc="<p>The name of the last table in the current page of results. Use this value as the <i>ExclusiveStartTableName</i> in a new request to obtain the next page of results, until all the table names are returned.</p> <p>If you do not receive a <i>LastEvaluatedTableName</i> value in the response, this means that there are no more table names to be retrieved.</p>"]
#[serde(rename="LastEvaluatedTableName")]
pub last_evaluated_table_name: Option<TableName>,
#[doc="<p>The names of the tables associated with the current account at the current endpoint. The maximum size of this array is 100.</p> <p>If <i>LastEvaluatedTableName</i> also appears in the output, you can use this value as the <i>ExclusiveStartTableName</i> parameter in a subsequent <i>ListTables</i> request and obtain the next page of results.</p>"]
#[serde(rename="TableNames")]
pub table_names: Option<TableNameList>,
            }
            
#[doc="<p>Represents the properties of a local secondary index.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct LocalSecondaryIndex {
                #[doc="<p>The name of the local secondary index. The name must be unique among all other indexes on this table.</p>"]
#[serde(rename="IndexName")]
pub index_name: IndexName,
#[doc="<p>The complete key schema for the local secondary index, consisting of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term \"hash attribute\" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term \"range attribute\" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note>"]
#[serde(rename="KeySchema")]
pub key_schema: KeySchema,
#[serde(rename="Projection")]
pub projection: Projection,
            }
            
#[doc="<p>Represents the properties of a local secondary index.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct LocalSecondaryIndexDescription {
                #[doc="<p>The Amazon Resource Name (ARN) that uniquely identifies the index.</p>"]
#[serde(rename="IndexArn")]
pub index_arn: Option<String>,
#[doc="<p>Represents the name of the local secondary index.</p>"]
#[serde(rename="IndexName")]
pub index_name: Option<IndexName>,
#[doc="<p>The total size of the specified index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>"]
#[serde(rename="IndexSizeBytes")]
pub index_size_bytes: Option<Long>,
#[doc="<p>The number of items in the specified index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>"]
#[serde(rename="ItemCount")]
pub item_count: Option<Long>,
#[doc="<p>The complete key schema for the local secondary index, consisting of one or more pairs of attribute names and key types:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term \"hash attribute\" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term \"range attribute\" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note>"]
#[serde(rename="KeySchema")]
pub key_schema: Option<KeySchema>,
#[serde(rename="Projection")]
pub projection: Option<Projection>,
            }
            
pub type LocalSecondaryIndexDescriptionList = Vec<LocalSecondaryIndexDescription>;
pub type LocalSecondaryIndexList = Vec<LocalSecondaryIndex>;
pub type Long = i64;
pub type MapAttributeValue = ::std::collections::HashMap<AttributeName, AttributeValue>;
pub type NonKeyAttributeName = String;
pub type NonKeyAttributeNameList = Vec<NonKeyAttributeName>;
pub type NullAttributeValue = bool;
pub type NumberAttributeValue = String;
pub type NumberSetAttributeValue = Vec<NumberAttributeValue>;
pub type PositiveIntegerObject = i64;
pub type PositiveLongObject = i64;
#[doc="<p>Represents attributes that are copied (projected) from the table into an index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Projection {
                #[doc="<p>Represents the non-key attribute names which will be projected into the index.</p> <p>For local secondary indexes, the total count of <i>NonKeyAttributes</i> summed across all of the local secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p>"]
#[serde(rename="NonKeyAttributes")]
pub non_key_attributes: Option<NonKeyAttributeNameList>,
#[doc="<p>The set of attributes that are projected into the index:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <i>NonKeyAttributes</i>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul>"]
#[serde(rename="ProjectionType")]
pub projection_type: Option<ProjectionType>,
            }
            
pub type ProjectionExpression = String;
pub type ProjectionType = String;
#[doc="<p>Represents the provisioned throughput settings for a specified table or index. The settings can be modified using the <i>UpdateTable</i> operation.</p> <p>For current minimum and maximum provisioned throughput values, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html\">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ProvisionedThroughput {
                #[doc="<p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <i>ThrottlingException</i>. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput\">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ReadCapacityUnits")]
pub read_capacity_units: PositiveLongObject,
#[doc="<p>The maximum number of writes consumed per second before DynamoDB returns a <i>ThrottlingException</i>. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput\">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="WriteCapacityUnits")]
pub write_capacity_units: PositiveLongObject,
            }
            
#[doc="<p>Represents the provisioned throughput settings for the table, consisting of read and write capacity units, along with data about increases and decreases.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ProvisionedThroughputDescription {
                #[doc="<p>The date and time of the last provisioned throughput decrease for this table.</p>"]
#[serde(rename="LastDecreaseDateTime")]
pub last_decrease_date_time: Option<Date>,
#[doc="<p>The date and time of the last provisioned throughput increase for this table.</p>"]
#[serde(rename="LastIncreaseDateTime")]
pub last_increase_date_time: Option<Date>,
#[doc="<p>The number of provisioned throughput decreases for this table during this UTC calendar day. For current maximums on provisioned throughput decreases, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html\">Limits</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="NumberOfDecreasesToday")]
pub number_of_decreases_today: Option<PositiveLongObject>,
#[doc="<p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <i>ThrottlingException</i>. Eventually consistent reads require less effort than strongly consistent reads, so a setting of 50 <i>ReadCapacityUnits</i> per second provides 100 eventually consistent <i>ReadCapacityUnits</i> per second.</p>"]
#[serde(rename="ReadCapacityUnits")]
pub read_capacity_units: Option<PositiveLongObject>,
#[doc="<p>The maximum number of writes consumed per second before DynamoDB returns a <i>ThrottlingException</i>.</p>"]
#[serde(rename="WriteCapacityUnits")]
pub write_capacity_units: Option<PositiveLongObject>,
            }
            
#[doc="<p>Represents the input of a <i>PutItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutItemInput {
                #[doc="<p>A condition that must be satisfied in order for a conditional <i>PutItem</i> operation to succeed.</p> <p>An expression can contain any of the following:</p> <ul> <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li> <li> <p>Comparison operators: <code> = | &amp;#x3C;&amp;#x3E; | &amp;#x3C; | &amp;#x3E; | &amp;#x3C;= | &amp;#x3E;= | BETWEEN | IN</code> </p> </li> <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li> </ul> <p>For more information on condition expressions, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html\">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>ConditionExpression</i> replaces the legacy <i>ConditionalOperator</i> and <i>Expected</i> parameters.</p> </note>"]
#[serde(rename="ConditionExpression")]
pub condition_expression: Option<ConditionExpression>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ConditionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A logical operator to apply to the conditions in the <i>Expected</i> map:</p> <ul> <li> <p> <code>AND</code> - If all of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> <li> <p> <code>OR</code> - If at least one of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> </ul> <p>If you omit <i>ConditionalOperator</i>, then <code>AND</code> is the default.</p> <p>The operation will succeed only if the entire map evaluates to true.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note>"]
#[serde(rename="ConditionalOperator")]
pub conditional_operator: Option<ConditionalOperator>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ConditionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A map of attribute/condition pairs. <i>Expected</i> provides a conditional block for the <i>PutItem</i> operation.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note> <p>Each element of <i>Expected</i> consists of an attribute name, a comparison operator, and one or more values. DynamoDB compares the attribute with the value(s) you supplied, using the comparison operator. For each <i>Expected</i> element, the result of the evaluation is either true or false.</p> <p>If you specify more than one element in the <i>Expected</i> map, then by default all of the conditions must evaluate to true. In other words, the conditions are ANDed together. (You can use the <i>ConditionalOperator</i> parameter to OR the conditions instead. If you do this, then at least one of the conditions must evaluate to true, rather than all of them.)</p> <p>If the <i>Expected</i> map evaluates to true, then the conditional operation succeeds; otherwise, it fails.</p> <p> <i>Expected</i> contains the following:</p> <ul> <li> <p> <i>AttributeValueList</i> - One or more values to evaluate against the supplied attribute. The number of values in the list depends on the <i>ComparisonOperator</i> being used.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href=\"http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters\">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For type Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p> </li> <li> <p> <i>ComparisonOperator</i> - A comparator for evaluating attributes in the <i>AttributeValueList</i>. When performing the comparison, DynamoDB uses strongly consistent reads.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS | BEGINS_WITH | IN | BETWEEN</code> </p> <p>The following are descriptions of each comparison operator.</p> <ul> <li> <p> <code>EQ</code> : Equal. <code>EQ</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NE</code> : Not equal. <code>NE</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LE</code> : Less than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LT</code> : Less than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GE</code> : Greater than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GT</code> : Greater than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NOT_NULL</code> : The attribute exists. <code>NOT_NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the existence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NOT_NULL</code>, the result is a Boolean <i>true</i>. This result is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NOT_NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>NULL</code> : The attribute does not exist. <code>NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the nonexistence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NULL</code>, the result is a Boolean <i>false</i>. This is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>CONTAINS</code> : Checks for a subsequence, or value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is of type String, then the operator checks for a substring match. If the target attribute of the comparison is of type Binary, then the operator looks for a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it finds an exact match with any member of the set.</p> <p>CONTAINS is supported for lists: When evaluating \"<code>a CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>NOT_CONTAINS</code> : Checks for absence of a subsequence, or absence of a value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is a String, then the operator checks for the absence of a substring match. If the target attribute of the comparison is Binary, then the operator checks for the absence of a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it <i>does not</i> find an exact match with any member of the set.</p> <p>NOT_CONTAINS is supported for lists: When evaluating \"<code>a NOT CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>BEGINS_WITH</code> : Checks for a prefix. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String or Binary (not a Number or a set type). The target attribute of the comparison must be of type String or Binary (not a Number or a set type).</p> <p/> </li> <li> <p> <code>IN</code> : Checks for matching elements within two sets.</p> <p> <i>AttributeValueList</i> can contain one or more <i>AttributeValue</i> elements of type String, Number, or Binary (not a set type). These attributes are compared against an existing set type attribute of an item. If any elements of the input set are present in the item attribute, the expression evaluates to true.</p> </li> <li> <p> <code>BETWEEN</code> : Greater than or equal to the first value, and less than or equal to the second value. </p> <p> <i>AttributeValueList</i> must contain two <i>AttributeValue</i> elements of the same type, either String, Number, or Binary (not a set type). A target attribute matches if the target value is greater than, or equal to, the first element and less than, or equal to, the second element. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not compare to <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code> </p> </li> </ul> </li> </ul> <p>For usage examples of <i>AttributeValueList</i> and <i>ComparisonOperator</i>, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.html\">Legacy Conditional Parameters</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>For backward compatibility with previous DynamoDB releases, the following parameters can be used instead of <i>AttributeValueList</i> and <i>ComparisonOperator</i>:</p> <ul> <li> <p> <i>Value</i> - A value for DynamoDB to compare with an attribute.</p> </li> <li> <p> <i>Exists</i> - A Boolean value that causes DynamoDB to evaluate the value before attempting the conditional operation:</p> <ul> <li> <p>If <i>Exists</i> is <code>true</code>, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the condition evaluates to true; otherwise the condition evaluate to false.</p> </li> <li> <p>If <i>Exists</i> is <code>false</code>, DynamoDB assumes that the attribute value does <i>not</i> exist in the table. If in fact the value does not exist, then the assumption is valid and the condition evaluates to true. If the value is found, despite the assumption that it does not exist, the condition evaluates to false.</p> </li> </ul> <p>Note that the default value for <i>Exists</i> is <code>true</code>.</p> </li> </ul> <p>The <i>Value</i> and <i>Exists</i> parameters are incompatible with <i>AttributeValueList</i> and <i>ComparisonOperator</i>. Note that if you use both sets of parameters at once, DynamoDB will return a <i>ValidationException</i> exception.</p>"]
#[serde(rename="Expected")]
pub expected: Option<ExpectedAttributeMap>,
#[doc="<p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p> <code>{\"#P\":\"Percentile\"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeNames")]
pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
#[doc="<p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <i>ExpressionAttributeValues</i> as follows:</p> <p> <code>{ \":avail\":{\"S\":\"Available\"}, \":back\":{\"S\":\"Backordered\"}, \":disc\":{\"S\":\"Discontinued\"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html\">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeValues")]
pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
#[doc="<p>A map of attribute name/value pairs, one for each attribute. Only the primary key attributes are required; you can optionally provide other attribute name-value pairs for the item.</p> <p>You must provide all of the attributes for the primary key. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide both values for both the partition key and the sort key.</p> <p>If you specify any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p> <p>For more information about primary keys, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html#DataModelPrimaryKey\">Primary Key</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Each element in the <i>Item</i> map is an <i>AttributeValue</i> object.</p>"]
#[serde(rename="Item")]
pub item: PutItemInputAttributeMap,
#[serde(rename="ReturnConsumedCapacity")]
pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
#[doc="<p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>"]
#[serde(rename="ReturnItemCollectionMetrics")]
pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
#[doc="<p>Use <i>ReturnValues</i> if you want to get the item attributes as they appeared before they were updated with the <i>PutItem</i> request. For <i>PutItem</i>, the valid values are:</p> <ul> <li> <p> <code>NONE</code> - If <i>ReturnValues</i> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <i>ReturnValues</i>.)</p> </li> <li> <p> <code>ALL_OLD</code> - If <i>PutItem</i> overwrote an attribute name-value pair, then the content of the old item is returned.</p> </li> </ul> <note> <p>The <i>ReturnValues</i> parameter is used by several DynamoDB operations; however, <i>PutItem</i> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p> </note>"]
#[serde(rename="ReturnValues")]
pub return_values: Option<ReturnValue>,
#[doc="<p>The name of the table to contain the item.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
            }
            
pub type PutItemInputAttributeMap = ::std::collections::HashMap<AttributeName, AttributeValue>;
#[doc="<p>Represents the output of a <i>PutItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutItemOutput {
                #[doc="<p>The attribute values as they appeared before the <i>PutItem</i> operation, but only if <i>ReturnValues</i> is specified as <code>ALL_OLD</code> in the request. Each element consists of an attribute name and an attribute value.</p>"]
#[serde(rename="Attributes")]
pub attributes: Option<AttributeMap>,
#[serde(rename="ConsumedCapacity")]
pub consumed_capacity: Option<ConsumedCapacity>,
#[doc="<p>Information about item collections, if any, that were affected by the operation. <i>ItemCollectionMetrics</i> is only returned if the request asked for it. If the table does not have any local secondary indexes, this information is not returned in the response.</p> <p>Each <i>ItemCollectionMetrics</i> element consists of:</p> <ul> <li> <p> <i>ItemCollectionKey</i> - The partition key value of the item collection. This is the same as the partition key value of the item itself.</p> </li> <li> <p> <i>SizeEstimateRange</i> - An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.</p> <p>The estimate is subject to change over time; therefore, do not rely on the precision or accuracy of the estimate.</p> </li> </ul>"]
#[serde(rename="ItemCollectionMetrics")]
pub item_collection_metrics: Option<ItemCollectionMetrics>,
            }
            
#[doc="<p>Represents a request to perform a <i>PutItem</i> operation on an item.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct PutRequest {
                #[doc="<p>A map of attribute name to attribute values, representing the primary key of an item to be processed by <i>PutItem</i>. All of the table's primary key attributes must be specified, and their data types must match those of the table's key schema. If any attributes are present in the item which are part of an index key schema for the table, their types must match the index key schema.</p>"]
#[serde(rename="Item")]
pub item: PutItemInputAttributeMap,
            }
            
#[doc="<p>Represents the input of a <i>Query</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct QueryInput {
                #[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ProjectionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> <p>This parameter allows you to retrieve attributes of type List or Map; however, it cannot retrieve individual elements within a List or a Map.</p> </important> <p>The names of one or more attributes to retrieve. If no attribute names are provided, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>Note that <i>AttributesToGet</i> has no effect on provisioned throughput consumption. DynamoDB determines capacity units consumed based on item size, not on the amount of data that is returned to an application.</p> <p>You cannot use both <i>AttributesToGet</i> and <i>Select</i> together in a <i>Query</i> request, <i>unless</i> the value for <i>Select</i> is <code>SPECIFIC_ATTRIBUTES</code>. (This usage is equivalent to specifying <i>AttributesToGet</i> without any value for <i>Select</i>.)</p> <p>If you query a local secondary index and request only attributes that are projected into that index, the operation will read only the index and not the table. If any of the requested attributes are not projected into the local secondary index, DynamoDB will fetch each of these attributes from the parent table. This extra fetching incurs additional throughput cost and latency.</p> <p>If you query a global secondary index, you can only request attributes that are projected into the index. Global secondary index queries cannot fetch attributes from the parent table.</p>"]
#[serde(rename="AttributesToGet")]
pub attributes_to_get: Option<AttributeNameList>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>FilterExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A logical operator to apply to the conditions in a <i>QueryFilter</i> map:</p> <ul> <li> <p> <code>AND</code> - If all of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> <li> <p> <code>OR</code> - If at least one of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> </ul> <p>If you omit <i>ConditionalOperator</i>, then <code>AND</code> is the default.</p> <p>The operation will succeed only if the entire map evaluates to true.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note>"]
#[serde(rename="ConditionalOperator")]
pub conditional_operator: Option<ConditionalOperator>,
#[doc="<p>Determines the read consistency model: If set to <code>true</code>, then the operation uses strongly consistent reads; otherwise, the operation uses eventually consistent reads.</p> <p>Strongly consistent reads are not supported on global secondary indexes. If you query a global secondary index with <i>ConsistentRead</i> set to <code>true</code>, you will receive a <i>ValidationException</i>.</p>"]
#[serde(rename="ConsistentRead")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub consistent_read: Option<ConsistentRead>,
#[doc="<p>The primary key of the first item that this operation will evaluate. Use the value that was returned for <i>LastEvaluatedKey</i> in the previous operation.</p> <p>The data type for <i>ExclusiveStartKey</i> must be String, Number or Binary. No set data types are allowed.</p>"]
#[serde(rename="ExclusiveStartKey")]
pub exclusive_start_key: Option<Key>,
#[doc="<p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p> <code>{\"#P\":\"Percentile\"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeNames")]
pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
#[doc="<p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <i>ExpressionAttributeValues</i> as follows:</p> <p> <code>{ \":avail\":{\"S\":\"Available\"}, \":back\":{\"S\":\"Backordered\"}, \":disc\":{\"S\":\"Discontinued\"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html\">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeValues")]
pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
#[doc="<p>A string that contains conditions that DynamoDB applies after the <i>Query</i> operation, but before the data is returned to you. Items that do not satisfy the <i>FilterExpression</i> criteria are not returned.</p> <note> <p>A <i>FilterExpression</i> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#FilteringResults\">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>FilterExpression</i> replaces the legacy <i>QueryFilter</i> and <i>ConditionalOperator</i> parameters.</p> </note>"]
#[serde(rename="FilterExpression")]
pub filter_expression: Option<ConditionExpression>,
#[doc="<p>The name of an index to query. This index can be any local secondary index or global secondary index on the table. Note that if you use the <i>IndexName</i> parameter, you must also provide <i>TableName.</i> </p>"]
#[serde(rename="IndexName")]
pub index_name: Option<IndexName>,
#[doc="<p>The condition that specifies the key value(s) for items to be retrieved by the <i>Query</i> action.</p> <p>The condition must perform an equality test on a single partition key value. The condition can also perform one of several comparison tests on a single sort key value. <i>Query</i> can use <i>KeyConditionExpression</i> to retrieve one item with a given partition key value and sort key value, or several items that have the same partition key value but different sort key values.</p> <p>The partition key equality test is required, and must be specified in the following format:</p> <p> <code>partitionKeyName</code> <i>=</i> <code>:partitionkeyval</code> </p> <p>If you also want to provide a condition for the sort key, it must be combined using <i>AND</i> with the condition for the sort key. Following is an example, using the <b>=</b> comparison operator for the sort key:</p> <p> <code>partitionKeyName</code> <i>=</i> <code>:partitionkeyval</code> <i>AND</i> <code>sortKeyName</code> <i>=</i> <code>:sortkeyval</code> </p> <p>Valid comparisons for the sort key condition are as follows:</p> <ul> <li> <p> <code>sortKeyName</code> <i>=</i> <code>:sortkeyval</code> - true if the sort key value is equal to <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <i>&lt;</i> <code>:sortkeyval</code> - true if the sort key value is less than <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <i>&lt;=</i> <code>:sortkeyval</code> - true if the sort key value is less than or equal to <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <i>&gt;</i> <code>:sortkeyval</code> - true if the sort key value is greater than <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <i>&gt;= </i> <code>:sortkeyval</code> - true if the sort key value is greater than or equal to <code>:sortkeyval</code>.</p> </li> <li> <p> <code>sortKeyName</code> <i>BETWEEN</i> <code>:sortkeyval1</code> <i>AND</i> <code>:sortkeyval2</code> - true if the sort key value is greater than or equal to <code>:sortkeyval1</code>, and less than or equal to <code>:sortkeyval2</code>.</p> </li> <li> <p> <i>begins_with (</i> <code>sortKeyName</code>, <code>:sortkeyval</code> <i>)</i> - true if the sort key value begins with a particular operand. (You cannot use this function with a sort key that is of type Number.) Note that the function name <code>begins_with</code> is case-sensitive.</p> </li> </ul> <p>Use the <i>ExpressionAttributeValues</i> parameter to replace tokens such as <code>:partitionval</code> and <code>:sortval</code> with actual values at runtime.</p> <p>You can optionally use the <i>ExpressionAttributeNames</i> parameter to replace the names of the partition key and sort key with placeholder tokens. This option might be necessary if an attribute name conflicts with a DynamoDB reserved word. For example, the following <i>KeyConditionExpression</i> parameter causes an error because <i>Size</i> is a reserved word:</p> <ul> <li><p> <code>Size = :myval</code> </p> </li> </ul> <p>To work around this, define a placeholder (such a <code>#S</code>) to represent the attribute name <i>Size</i>. <i>KeyConditionExpression</i> then is as follows:</p> <ul> <li><p> <code>#S = :myval</code> </p> </li> </ul> <p>For a list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>For more information on <i>ExpressionAttributeNames</i> and <i>ExpressionAttributeValues</i>, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ExpressionPlaceholders.html\">Using Placeholders for Attribute Names and Values</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>KeyConditionExpression</i> replaces the legacy <i>KeyConditions</i> parameter.</p> </note>"]
#[serde(rename="KeyConditionExpression")]
pub key_condition_expression: Option<KeyExpression>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>KeyConditionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>The selection criteria for the query. For a query on a table, you can have conditions only on the table primary key attributes. You must provide the partition key name and value as an <code>EQ</code> condition. You can optionally provide a second condition, referring to the sort key.</p> <note> <p>If you don't provide a sort key condition, all of the items that match the partition key will be retrieved. If a <i>FilterExpression</i> or <i>QueryFilter</i> is present, it will be applied after the items are retrieved.</p> </note> <p>For a query on an index, you can have conditions only on the index key attributes. You must provide the index partition key name and value as an <code>EQ</code> condition. You can optionally provide a second condition, referring to the index sort key.</p> <p>Each <i>KeyConditions</i> element consists of an attribute name to compare, along with the following:</p> <ul> <li> <p> <i>AttributeValueList</i> - One or more values to evaluate against the supplied attribute. The number of values in the list depends on the <i>ComparisonOperator</i> being used.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href=\"http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters\">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p> </li> <li> <p> <i>ComparisonOperator</i> - A comparator for evaluating attributes, for example, equals, greater than, less than, and so on.</p> <p>For <i>KeyConditions</i>, only the following comparison operators are supported:</p> <p> <code>EQ | LE | LT | GE | GT | BEGINS_WITH | BETWEEN</code> </p> <p>The following are descriptions of these comparison operators.</p> <ul> <li> <p> <code>EQ</code> : Equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one specified in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LE</code> : Less than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LT</code> : Less than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GE</code> : Greater than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GT</code> : Greater than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>BEGINS_WITH</code> : Checks for a prefix. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String or Binary (not a Number or a set type). The target attribute of the comparison must be of type String or Binary (not a Number or a set type).</p> <p/> </li> <li> <p> <code>BETWEEN</code> : Greater than or equal to the first value, and less than or equal to the second value. </p> <p> <i>AttributeValueList</i> must contain two <i>AttributeValue</i> elements of the same type, either String, Number, or Binary (not a set type). A target attribute matches if the target value is greater than, or equal to, the first element and less than, or equal to, the second element. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not compare to <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code> </p> </li> </ul> </li> </ul> <p>For usage examples of <i>AttributeValueList</i> and <i>ComparisonOperator</i>, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.html\">Legacy Conditional Parameters</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="KeyConditions")]
pub key_conditions: Option<KeyConditions>,
#[doc="<p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, and a key in <i>LastEvaluatedKey</i> to apply in a subsequent operation, so that you can pick up where you left off. Also, if the processed data set size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <i>LastEvaluatedKey</i> to apply in a subsequent operation to continue the operation. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html\">Query and Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="Limit")]
pub limit: Option<PositiveIntegerObject>,
#[doc="<p>A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>ProjectionExpression</i> replaces the legacy <i>AttributesToGet</i> parameter.</p> </note>"]
#[serde(rename="ProjectionExpression")]
pub projection_expression: Option<ProjectionExpression>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>FilterExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A condition that evaluates the query results after the items are read and returns only the desired values.</p> <p>This parameter does not support attributes of type List or Map.</p> <note> <p>A <i>QueryFilter</i> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>If you provide more than one condition in the <i>QueryFilter</i> map, then by default all of the conditions must evaluate to true. In other words, the conditions are ANDed together. (You can use the <i>ConditionalOperator</i> parameter to OR the conditions instead. If you do this, then at least one of the conditions must evaluate to true, rather than all of them.)</p> <p>Note that <i>QueryFilter</i> does not allow key attributes. You cannot define a filter condition on a partition key or a sort key.</p> <p>Each <i>QueryFilter</i> element consists of an attribute name to compare, along with the following:</p> <ul> <li> <p> <i>AttributeValueList</i> - One or more values to evaluate against the supplied attribute. The number of values in the list depends on the operator specified in <i>ComparisonOperator</i>.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href=\"http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters\">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For type Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p> <p>For information on specifying data types in JSON, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataFormat.html\">JSON Data Format</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </li> <li> <p> <i>ComparisonOperator</i> - A comparator for evaluating attributes. For example, equals, greater than, less than, etc.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS | BEGINS_WITH | IN | BETWEEN</code> </p> <p>For complete descriptions of all comparison operators, see the <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Condition.html\">Condition</a> data type.</p> </li> </ul>"]
#[serde(rename="QueryFilter")]
pub query_filter: Option<FilterConditionMap>,
#[serde(rename="ReturnConsumedCapacity")]
pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
#[doc="<p>Specifies the order for index traversal: If <code>true</code> (default), the traversal is performed in ascending order; if <code>false</code>, the traversal is performed in descending order. </p> <p>Items with the same partition key value are stored in sorted order by sort key. If the sort key data type is Number, the results are stored in numeric order. For type String, the results are stored in order of ASCII character code values. For type Binary, DynamoDB treats each byte of the binary data as unsigned.</p> <p>If <i>ScanIndexForward</i> is <code>true</code>, DynamoDB returns the results in the order in which they are stored (by sort key value). This is the default behavior. If <i>ScanIndexForward</i> is <code>false</code>, DynamoDB reads the results in reverse order by sort key value, and then returns the results to the client.</p>"]
#[serde(rename="ScanIndexForward")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub scan_index_forward: Option<BooleanObject>,
#[doc="<p>The attributes to be returned in the result. You can retrieve all item attributes, specific item attributes, the count of matching items, or in the case of an index, some or all of the attributes projected into the index.</p> <ul> <li> <p> <code>ALL_ATTRIBUTES</code> - Returns all of the item attributes from the specified table or index. If you query a local secondary index, then for each matching item in the index DynamoDB will fetch the entire item from the parent table. If the index is configured to project all item attributes, then all of the data can be obtained from the local secondary index, and no fetching is required.</p> </li> <li> <p> <code>ALL_PROJECTED_ATTRIBUTES</code> - Allowed only when querying an index. Retrieves all attributes that have been projected into the index. If the index is configured to project all attributes, this return value is equivalent to specifying <code>ALL_ATTRIBUTES</code>.</p> </li> <li> <p> <code>COUNT</code> - Returns the number of matching items, rather than the matching items themselves.</p> </li> <li> <p> <code>SPECIFIC_ATTRIBUTES</code> - Returns only the attributes listed in <i>AttributesToGet</i>. This return value is equivalent to specifying <i>AttributesToGet</i> without specifying any value for <i>Select</i>.</p> <p>If you query a local secondary index and request only attributes that are projected into that index, the operation will read only the index and not the table. If any of the requested attributes are not projected into the local secondary index, DynamoDB will fetch each of these attributes from the parent table. This extra fetching incurs additional throughput cost and latency.</p> <p>If you query a global secondary index, you can only request attributes that are projected into the index. Global secondary index queries cannot fetch attributes from the parent table.</p> </li> </ul> <p>If neither <i>Select</i> nor <i>AttributesToGet</i> are specified, DynamoDB defaults to <code>ALL_ATTRIBUTES</code> when accessing a table, and <code>ALL_PROJECTED_ATTRIBUTES</code> when accessing an index. You cannot use both <i>Select</i> and <i>AttributesToGet</i> together in a single request, unless the value for <i>Select</i> is <code>SPECIFIC_ATTRIBUTES</code>. (This usage is equivalent to specifying <i>AttributesToGet</i> without any value for <i>Select</i>.)</p> <note> <p>If you use the <i>ProjectionExpression</i> parameter, then the value for <i>Select</i> can only be <code>SPECIFIC_ATTRIBUTES</code>. Any other value for <i>Select</i> will return an error.</p> </note>"]
#[serde(rename="Select")]
pub select: Option<Select>,
#[doc="<p>The name of the table containing the requested items.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
            }
            
#[doc="<p>Represents the output of a <i>Query</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct QueryOutput {
                #[serde(rename="ConsumedCapacity")]
pub consumed_capacity: Option<ConsumedCapacity>,
#[doc="<p>The number of items in the response.</p> <p>If you used a <i>QueryFilter</i> in the request, then <i>Count</i> is the number of items returned after the filter was applied, and <i>ScannedCount</i> is the number of matching items before the filter was applied.</p> <p>If you did not use a filter in the request, then <i>Count</i> and <i>ScannedCount</i> are the same.</p>"]
#[serde(rename="Count")]
pub count: Option<Integer>,
#[doc="<p>An array of item attributes that match the query criteria. Each element in this array consists of an attribute name and the value for that attribute.</p>"]
#[serde(rename="Items")]
pub items: Option<ItemList>,
#[doc="<p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <i>LastEvaluatedKey</i> is empty, then the \"last page\" of results has been processed and there is no more data to be retrieved.</p> <p>If <i>LastEvaluatedKey</i> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <i>LastEvaluatedKey</i> is empty.</p>"]
#[serde(rename="LastEvaluatedKey")]
pub last_evaluated_key: Option<Key>,
#[doc="<p>The number of items evaluated, before any <i>QueryFilter</i> is applied. A high <i>ScannedCount</i> value with few, or no, <i>Count</i> results indicates an inefficient <i>Query</i> operation. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count\">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>If you did not use a filter in the request, then <i>ScannedCount</i> is the same as <i>Count</i>.</p>"]
#[serde(rename="ScannedCount")]
pub scanned_count: Option<Integer>,
            }
            
#[doc="<p>Determines the level of detail about provisioned throughput consumption that is returned in the response:</p> <ul> <li> <p> <i>INDEXES</i> - The response includes the aggregate <i>ConsumedCapacity</i> for the operation, together with <i>ConsumedCapacity</i> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <i>GetItem</i> and <i>BatchGetItem</i>, do not access any indexes at all. In these cases, specifying <i>INDEXES</i> will only return <i>ConsumedCapacity</i> information for table(s).</p> </li> <li> <p> <i>TOTAL</i> - The response includes only the aggregate <i>ConsumedCapacity</i> for the operation.</p> </li> <li> <p> <i>NONE</i> - No <i>ConsumedCapacity</i> details are included in the response.</p> </li> </ul>"]
pub type ReturnConsumedCapacity = String;
pub type ReturnItemCollectionMetrics = String;
pub type ReturnValue = String;
pub type ScalarAttributeType = String;
#[doc="<p>Represents the input of a <i>Scan</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ScanInput {
                #[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ProjectionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> <p>This parameter allows you to retrieve attributes of type List or Map; however, it cannot retrieve individual elements within a List or a Map.</p> </important> <p>The names of one or more attributes to retrieve. If no attribute names are provided, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>Note that <i>AttributesToGet</i> has no effect on provisioned throughput consumption. DynamoDB determines capacity units consumed based on item size, not on the amount of data that is returned to an application.</p>"]
#[serde(rename="AttributesToGet")]
pub attributes_to_get: Option<AttributeNameList>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>FilterExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A logical operator to apply to the conditions in a <i>ScanFilter</i> map:</p> <ul> <li> <p> <code>AND</code> - If all of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> <li> <p> <code>OR</code> - If at least one of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> </ul> <p>If you omit <i>ConditionalOperator</i>, then <code>AND</code> is the default.</p> <p>The operation will succeed only if the entire map evaluates to true.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note>"]
#[serde(rename="ConditionalOperator")]
pub conditional_operator: Option<ConditionalOperator>,
#[doc="<p>A Boolean value that determines the read consistency model during the scan:</p> <ul> <li> <p>If <i>ConsistentRead</i> is <code>false</code>, then the data returned from <i>Scan</i> might not contain the results from other recently completed write operations (PutItem, UpdateItem or DeleteItem).</p> </li> <li> <p>If <i>ConsistentRead</i> is <code>true</code>, then all of the write operations that completed before the <i>Scan</i> began are guaranteed to be contained in the <i>Scan</i> response.</p> </li> </ul> <p>The default setting for <i>ConsistentRead</i> is <code>false</code>.</p> <p>The <i>ConsistentRead</i> parameter is not supported on global secondary indexes. If you scan a global secondary index with <i>ConsistentRead</i> set to true, you will receive a <i>ValidationException</i>.</p>"]
#[serde(rename="ConsistentRead")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub consistent_read: Option<ConsistentRead>,
#[doc="<p>The primary key of the first item that this operation will evaluate. Use the value that was returned for <i>LastEvaluatedKey</i> in the previous operation.</p> <p>The data type for <i>ExclusiveStartKey</i> must be String, Number or Binary. No set data types are allowed.</p> <p>In a parallel scan, a <i>Scan</i> request that includes <i>ExclusiveStartKey</i> must specify the same segment whose previous <i>Scan</i> returned the corresponding value of <i>LastEvaluatedKey</i>.</p>"]
#[serde(rename="ExclusiveStartKey")]
pub exclusive_start_key: Option<Key>,
#[doc="<p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p> <code>{\"#P\":\"Percentile\"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeNames")]
pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
#[doc="<p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <i>ExpressionAttributeValues</i> as follows:</p> <p> <code>{ \":avail\":{\"S\":\"Available\"}, \":back\":{\"S\":\"Backordered\"}, \":disc\":{\"S\":\"Discontinued\"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html\">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeValues")]
pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
#[doc="<p>A string that contains conditions that DynamoDB applies after the <i>Scan</i> operation, but before the data is returned to you. Items that do not satisfy the <i>FilterExpression</i> criteria are not returned.</p> <note> <p>A <i>FilterExpression</i> is applied after the items have already been read; the process of filtering does not consume any additional read capacity units.</p> </note> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#FilteringResults\">Filter Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>FilterExpression</i> replaces the legacy <i>ScanFilter</i> and <i>ConditionalOperator</i> parameters.</p> </note>"]
#[serde(rename="FilterExpression")]
pub filter_expression: Option<ConditionExpression>,
#[doc="<p>The name of a secondary index to scan. This index can be any local secondary index or global secondary index. Note that if you use the <code>IndexName</code> parameter, you must also provide <code>TableName</code>.</p>"]
#[serde(rename="IndexName")]
pub index_name: Option<IndexName>,
#[doc="<p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, and a key in <i>LastEvaluatedKey</i> to apply in a subsequent operation, so that you can pick up where you left off. Also, if the processed data set size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <i>LastEvaluatedKey</i> to apply in a subsequent operation to continue the operation. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html\">Query and Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="Limit")]
pub limit: Option<PositiveIntegerObject>,
#[doc="<p>A string that identifies one or more attributes to retrieve from the specified table or index. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.</p> <p>If no attribute names are specified, then all attributes will be returned. If any of the requested attributes are not found, they will not appear in the result.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>ProjectionExpression</i> replaces the legacy <i>AttributesToGet</i> parameter.</p> </note>"]
#[serde(rename="ProjectionExpression")]
pub projection_expression: Option<ProjectionExpression>,
#[serde(rename="ReturnConsumedCapacity")]
pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>FilterExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A condition that evaluates the scan results and returns only the desired values.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note> <p>If you specify more than one condition in the <i>ScanFilter</i> map, then by default all of the conditions must evaluate to true. In other words, the conditions are ANDed together. (You can use the <i>ConditionalOperator</i> parameter to OR the conditions instead. If you do this, then at least one of the conditions must evaluate to true, rather than all of them.)</p> <p>Each <i>ScanFilter</i> element consists of an attribute name to compare, along with the following:</p> <ul> <li> <p> <i>AttributeValueList</i> - One or more values to evaluate against the supplied attribute. The number of values in the list depends on the operator specified in <i>ComparisonOperator</i> .</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href=\"http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters\">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p> <p>For information on specifying data types in JSON, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataFormat.html\">JSON Data Format</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </li> <li> <p> <i>ComparisonOperator</i> - A comparator for evaluating attributes. For example, equals, greater than, less than, etc.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS | BEGINS_WITH | IN | BETWEEN</code> </p> <p>For complete descriptions of all comparison operators, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Condition.html\">Condition</a>.</p> </li> </ul>"]
#[serde(rename="ScanFilter")]
pub scan_filter: Option<FilterConditionMap>,
#[doc="<p>For a parallel <i>Scan</i> request, <i>Segment</i> identifies an individual segment to be scanned by an application worker.</p> <p>Segment IDs are zero-based, so the first segment is always 0. For example, if you want to use four application threads to scan a table or an index, then the first thread specifies a <i>Segment</i> value of 0, the second thread specifies 1, and so on.</p> <p>The value of <i>LastEvaluatedKey</i> returned from a parallel <i>Scan</i> request must be used as <i>ExclusiveStartKey</i> with the same segment ID in a subsequent <i>Scan</i> operation.</p> <p>The value for <i>Segment</i> must be greater than or equal to 0, and less than the value provided for <i>TotalSegments</i>.</p> <p>If you provide <i>Segment</i>, you must also provide <i>TotalSegments</i>.</p>"]
#[serde(rename="Segment")]
pub segment: Option<ScanSegment>,
#[doc="<p>The attributes to be returned in the result. You can retrieve all item attributes, specific item attributes, or the count of matching items.</p> <ul> <li> <p> <code>ALL_ATTRIBUTES</code> - Returns all of the item attributes.</p> </li> <li> <p> <code>ALL_PROJECTED_ATTRIBUTES</code> - Allowed only when querying an index. Retrieves all attributes that have been projected into the index. If the index is configured to project all attributes, this return value is equivalent to specifying <code>ALL_ATTRIBUTES</code>.</p> </li> <li> <p> <code>COUNT</code> - Returns the number of matching items, rather than the matching items themselves.</p> </li> <li> <p> <code>SPECIFIC_ATTRIBUTES</code> - Returns only the attributes listed in <i>AttributesToGet</i>. This return value is equivalent to specifying <i>AttributesToGet</i> without specifying any value for <i>Select</i>.</p> </li> </ul> <p>If neither <i>Select</i> nor <i>AttributesToGet</i> are specified, DynamoDB defaults to <code>ALL_ATTRIBUTES</code>. You cannot use both <i>AttributesToGet</i> and <i>Select</i> together in a single request, unless the value for <i>Select</i> is <code>SPECIFIC_ATTRIBUTES</code>. (This usage is equivalent to specifying <i>AttributesToGet</i> without any value for <i>Select</i>.)</p>"]
#[serde(rename="Select")]
pub select: Option<Select>,
#[doc="<p>The name of the table containing the requested items; or, if you provide <code>IndexName</code>, the name of the table to which that index belongs.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
#[doc="<p>For a parallel <i>Scan</i> request, <i>TotalSegments</i> represents the total number of segments into which the <i>Scan</i> operation will be divided. The value of <i>TotalSegments</i> corresponds to the number of application workers that will perform the parallel scan. For example, if you want to use four application threads to scan a table or an index, specify a <i>TotalSegments</i> value of 4.</p> <p>The value for <i>TotalSegments</i> must be greater than or equal to 1, and less than or equal to 1000000. If you specify a <i>TotalSegments</i> value of 1, the <i>Scan</i> operation will be sequential rather than parallel.</p> <p>If you specify <i>TotalSegments</i>, you must also specify <i>Segment</i>.</p>"]
#[serde(rename="TotalSegments")]
pub total_segments: Option<ScanTotalSegments>,
            }
            
#[doc="<p>Represents the output of a <i>Scan</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ScanOutput {
                #[serde(rename="ConsumedCapacity")]
pub consumed_capacity: Option<ConsumedCapacity>,
#[doc="<p>The number of items in the response.</p> <p>If you set <i>ScanFilter</i> in the request, then <i>Count</i> is the number of items returned after the filter was applied, and <i>ScannedCount</i> is the number of matching items before the filter was applied.</p> <p>If you did not use a filter in the request, then <i>Count</i> is the same as <i>ScannedCount</i>.</p>"]
#[serde(rename="Count")]
pub count: Option<Integer>,
#[doc="<p>An array of item attributes that match the scan criteria. Each element in this array consists of an attribute name and the value for that attribute.</p>"]
#[serde(rename="Items")]
pub items: Option<ItemList>,
#[doc="<p>The primary key of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p> <p>If <i>LastEvaluatedKey</i> is empty, then the \"last page\" of results has been processed and there is no more data to be retrieved.</p> <p>If <i>LastEvaluatedKey</i> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <i>LastEvaluatedKey</i> is empty.</p>"]
#[serde(rename="LastEvaluatedKey")]
pub last_evaluated_key: Option<Key>,
#[doc="<p>The number of items evaluated, before any <i>ScanFilter</i> is applied. A high <i>ScannedCount</i> value with few, or no, <i>Count</i> results indicates an inefficient <i>Scan</i> operation. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count\">Count and ScannedCount</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>If you did not use a filter in the request, then <i>ScannedCount</i> is the same as <i>Count</i>.</p>"]
#[serde(rename="ScannedCount")]
pub scanned_count: Option<Integer>,
            }
            
pub type ScanSegment = i64;
pub type ScanTotalSegments = i64;
pub type SecondaryIndexesCapacityMap = ::std::collections::HashMap<IndexName, Capacity>;
pub type Select = String;
pub type StreamArn = String;
pub type StreamEnabled = bool;
#[doc="<p>Represents the DynamoDB Streams configuration for a table in DynamoDB.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct StreamSpecification {
                #[doc="<p>Indicates whether DynamoDB Streams is enabled (true) or disabled (false) on the table.</p>"]
#[serde(rename="StreamEnabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub stream_enabled: Option<StreamEnabled>,
#[doc="<p>The DynamoDB Streams settings for the table. These settings consist of:</p> <ul> <li> <p> <i>StreamEnabled</i> - Indicates whether DynamoDB Streams is enabled (true) or disabled (false) on the table.</p> </li> <li> <p> <i>StreamViewType</i> - When an item in the table is modified, <i>StreamViewType</i> determines what information is written to the stream for this table. Valid values for <i>StreamViewType</i> are:</p> <ul> <li> <p> <i>KEYS_ONLY</i> - Only the key attributes of the modified item are written to the stream.</p> </li> <li> <p> <i>NEW_IMAGE</i> - The entire item, as it appears after it was modified, is written to the stream.</p> </li> <li> <p> <i>OLD_IMAGE</i> - The entire item, as it appeared before it was modified, is written to the stream.</p> </li> <li> <p> <i>NEW_AND_OLD_IMAGES</i> - Both the new and the old item images of the item are written to the stream.</p> </li> </ul> </li> </ul>"]
#[serde(rename="StreamViewType")]
pub stream_view_type: Option<StreamViewType>,
            }
            
pub type StreamViewType = String;
pub type StringAttributeValue = String;
pub type StringSetAttributeValue = Vec<StringAttributeValue>;
#[doc="<p>Represents the properties of a table.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct TableDescription {
                #[doc="<p>An array of <i>AttributeDefinition</i> objects. Each of these objects describes one attribute in the table and index key schema.</p> <p>Each <i>AttributeDefinition</i> object in this array is composed of:</p> <ul> <li> <p> <i>AttributeName</i> - The name of the attribute.</p> </li> <li> <p> <i>AttributeType</i> - The data type for the attribute.</p> </li> </ul>"]
#[serde(rename="AttributeDefinitions")]
pub attribute_definitions: Option<AttributeDefinitions>,
#[doc="<p>The date and time when the table was created, in <a href=\"http://www.epochconverter.com/\">UNIX epoch time</a> format.</p>"]
#[serde(rename="CreationDateTime")]
pub creation_date_time: Option<Date>,
#[doc="<p>The global secondary indexes, if any, on the table. Each index is scoped to a given partition key value. Each element is composed of:</p> <ul> <li> <p> <i>Backfilling</i> - If true, then the index is currently in the backfilling phase. Backfilling occurs only when a new global secondary index is added to the table; it is the process by which DynamoDB populates the new index with data from the table. (This attribute does not appear for indexes that were created during a <i>CreateTable</i> operation.)</p> </li> <li> <p> <i>IndexName</i> - The name of the global secondary index.</p> </li> <li> <p> <i>IndexSizeBytes</i> - The total size of the global secondary index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value. </p> </li> <li> <p> <i>IndexStatus</i> - The current status of the global secondary index:</p> <ul> <li> <p> <i>CREATING</i> - The index is being created.</p> </li> <li> <p> <i>UPDATING</i> - The index is being updated.</p> </li> <li> <p> <i>DELETING</i> - The index is being deleted.</p> </li> <li> <p> <i>ACTIVE</i> - The index is ready for use.</p> </li> </ul> </li> <li> <p> <i>ItemCount</i> - The number of items in the global secondary index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value. </p> </li> <li> <p> <i>KeySchema</i> - Specifies the complete index key schema. The attribute names in the key schema must be between 1 and 255 characters (inclusive). The key schema must begin with the same partition key as the table.</p> </li> <li> <p> <i>Projection</i> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p> <ul> <li> <p> <i>ProjectionType</i> - One of the following:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <i>NonKeyAttributes</i>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul> </li> <li> <p> <i>NonKeyAttributes</i> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <i>NonKeyAttributes</i>, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p> </li> </ul> </li> <li> <p> <i>ProvisionedThroughput</i> - The provisioned throughput settings for the global secondary index, consisting of read and write capacity units, along with data about increases and decreases. </p> </li> </ul> <p>If the table is in the <code>DELETING</code> state, no information about indexes will be returned.</p>"]
#[serde(rename="GlobalSecondaryIndexes")]
pub global_secondary_indexes: Option<GlobalSecondaryIndexDescriptionList>,
#[doc="<p>The number of items in the specified table. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>"]
#[serde(rename="ItemCount")]
pub item_count: Option<Long>,
#[doc="<p>The primary key structure for the table. Each <i>KeySchemaElement</i> consists of:</p> <ul> <li> <p> <i>AttributeName</i> - The name of the attribute.</p> </li> <li> <p> <i>KeyType</i> - The role of the attribute:</p> <ul> <li> <p> <code>HASH</code> - partition key</p> </li> <li> <p> <code>RANGE</code> - sort key</p> </li> </ul> <note> <p>The partition key of an item is also known as its <i>hash attribute</i>. The term \"hash attribute\" derives from DynamoDB' usage of an internal hash function to evenly distribute data items across partitions, based on their partition key values.</p> <p>The sort key of an item is also known as its <i>range attribute</i>. The term \"range attribute\" derives from the way DynamoDB stores items with the same partition key physically close together, in sorted order by the sort key value.</p> </note> </li> </ul> <p>For more information about primary keys, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html#DataModelPrimaryKey\">Primary Key</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="KeySchema")]
pub key_schema: Option<KeySchema>,
#[doc="<p>The Amazon Resource Name (ARN) that uniquely identifies the latest stream for this table.</p>"]
#[serde(rename="LatestStreamArn")]
pub latest_stream_arn: Option<StreamArn>,
#[doc="<p>A timestamp, in ISO 8601 format, for this stream.</p> <p>Note that <i>LatestStreamLabel</i> is not a unique identifier for the stream, because it is possible that a stream from another table might have the same timestamp. However, the combination of the following three elements is guaranteed to be unique:</p> <ul> <li> <p>the AWS customer ID.</p> </li> <li> <p>the table name.</p> </li> <li> <p>the <i>StreamLabel</i>.</p> </li> </ul>"]
#[serde(rename="LatestStreamLabel")]
pub latest_stream_label: Option<String>,
#[doc="<p>Represents one or more local secondary indexes on the table. Each index is scoped to a given partition key value. Tables with one or more local secondary indexes are subject to an item collection size limit, where the amount of data within a given item collection cannot exceed 10 GB. Each element is composed of:</p> <ul> <li> <p> <i>IndexName</i> - The name of the local secondary index.</p> </li> <li> <p> <i>KeySchema</i> - Specifies the complete index key schema. The attribute names in the key schema must be between 1 and 255 characters (inclusive). The key schema must begin with the same partition key as the table.</p> </li> <li> <p> <i>Projection</i> - Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:</p> <ul> <li> <p> <i>ProjectionType</i> - One of the following:</p> <ul> <li> <p> <code>KEYS_ONLY</code> - Only the index and primary keys are projected into the index.</p> </li> <li> <p> <code>INCLUDE</code> - Only the specified table attributes are projected into the index. The list of projected attributes are in <i>NonKeyAttributes</i>.</p> </li> <li> <p> <code>ALL</code> - All of the table attributes are projected into the index.</p> </li> </ul> </li> <li> <p> <i>NonKeyAttributes</i> - A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in <i>NonKeyAttributes</i>, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.</p> </li> </ul> </li> <li> <p> <i>IndexSizeBytes</i> - Represents the total size of the index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p> </li> <li> <p> <i>ItemCount</i> - Represents the number of items in the index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p> </li> </ul> <p>If the table is in the <code>DELETING</code> state, no information about indexes will be returned.</p>"]
#[serde(rename="LocalSecondaryIndexes")]
pub local_secondary_indexes: Option<LocalSecondaryIndexDescriptionList>,
#[doc="<p>The provisioned throughput settings for the table, consisting of read and write capacity units, along with data about increases and decreases.</p>"]
#[serde(rename="ProvisionedThroughput")]
pub provisioned_throughput: Option<ProvisionedThroughputDescription>,
#[doc="<p>The current DynamoDB Streams configuration for the table.</p>"]
#[serde(rename="StreamSpecification")]
pub stream_specification: Option<StreamSpecification>,
#[doc="<p>The Amazon Resource Name (ARN) that uniquely identifies the table.</p>"]
#[serde(rename="TableArn")]
pub table_arn: Option<String>,
#[doc="<p>The name of the table.</p>"]
#[serde(rename="TableName")]
pub table_name: Option<TableName>,
#[doc="<p>The total size of the specified table, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.</p>"]
#[serde(rename="TableSizeBytes")]
pub table_size_bytes: Option<Long>,
#[doc="<p>The current state of the table:</p> <ul> <li> <p> <i>CREATING</i> - The table is being created.</p> </li> <li> <p> <i>UPDATING</i> - The table is being updated.</p> </li> <li> <p> <i>DELETING</i> - The table is being deleted.</p> </li> <li> <p> <i>ACTIVE</i> - The table is ready for use.</p> </li> </ul>"]
#[serde(rename="TableStatus")]
pub table_status: Option<TableStatus>,
            }
            
pub type TableName = String;
pub type TableNameList = Vec<TableName>;
pub type TableStatus = String;
pub type UpdateExpression = String;
#[doc="<p>Represents the new provisioned throughput settings to be applied to a global secondary index.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateGlobalSecondaryIndexAction {
                #[doc="<p>The name of the global secondary index to be updated.</p>"]
#[serde(rename="IndexName")]
pub index_name: IndexName,
#[serde(rename="ProvisionedThroughput")]
pub provisioned_throughput: ProvisionedThroughput,
            }
            
#[doc="<p>Represents the input of an <i>UpdateItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateItemInput {
                #[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>UpdateExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> <p>This parameter can be used for modifying top-level attributes; however, it does not support individual list or map elements.</p> </important> <p>The names of attributes to be modified, the action to perform on each, and the new value for each. If you are updating an attribute that is an index key attribute for any indexes on that table, the attribute type must match the index key type defined in the <i>AttributesDefinition</i> of the table description. You can use <i>UpdateItem</i> to update any non-key attributes.</p> <p>Attribute values cannot be null. String and Binary type attributes must have lengths greater than zero. Set type attributes must not be empty. Requests with empty values will be rejected with a <i>ValidationException</i> exception.</p> <p>Each <i>AttributeUpdates</i> element consists of an attribute name to modify, along with the following:</p> <ul> <li> <p> <i>Value</i> - The new value, if applicable, for this attribute.</p> </li> <li> <p> <i>Action</i> - A value that specifies how to perform the update. This action is only valid for an existing attribute whose data type is Number or is a set; do not use <code>ADD</code> for other data types. </p> <p>If an item with the specified primary key is found in the table, the following values perform the following actions:</p> <ul> <li> <p> <code>PUT</code> - Adds the specified attribute to the item. If the attribute already exists, it is replaced by the new value. </p> </li> <li> <p> <code>DELETE</code> - Removes the attribute and its value, if no value is specified for <code>DELETE</code>. The data type of the specified value must match the existing value's data type.</p> <p>If a set of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>[a,b,c]</code> and the <code>DELETE</code> action specifies <code>[a,c]</code>, then the final attribute value is <code>[b]</code>. Specifying an empty set is an error.</p> </li> <li> <p> <code>ADD</code> - Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p> <ul> <li> <p>If the existing attribute is a number, and if <i>Value</i> is also a number, then <i>Value</i> is mathematically added to the existing attribute. If <i>Value</i> is a negative number, then it is subtracted from the existing attribute.</p> <note> <p>If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses 0 as the initial value.</p> <p>Similarly, if you use <code>ADD</code> for an existing item to increment or decrement an attribute value that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update doesn't have an attribute named <i>itemcount</i>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway. DynamoDB will create the <i>itemcount</i> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <i>itemcount</i> attribute, with a value of <code>3</code>.</p> </note> </li> <li> <p>If the existing data type is a set, and if <i>Value</i> is also a set, then <i>Value</i> is appended to the existing set. For example, if the attribute value is the set <code>[1,2]</code>, and the <code>ADD</code> action specified <code>[3]</code>, then the final attribute value is <code>[1,2,3]</code>. An error occurs if an <code>ADD</code> action is specified for a set attribute and the attribute type specified does not match the existing set type. </p> <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, <i>Value</i> must also be a set of strings.</p> </li> </ul> </li> </ul> <p>If no item with the specified key is found in the table, the following values perform the following actions:</p> <ul> <li> <p> <code>PUT</code> - Causes DynamoDB to create a new item with the specified primary key, and then adds the attribute. </p> </li> <li> <p> <code>DELETE</code> - Nothing happens, because attributes cannot be deleted from a nonexistent item. The operation succeeds, but DynamoDB does not create a new item.</p> </li> <li> <p> <code>ADD</code> - Causes DynamoDB to create an item with the supplied primary key and number (or set of numbers) for the attribute value. The only data types allowed are Number and Number Set.</p> </li> </ul> </li> </ul> <p>If you provide any attributes that are part of an index key, then the data types for those attributes must match those of the schema in the table's attribute definition.</p>"]
#[serde(rename="AttributeUpdates")]
pub attribute_updates: Option<AttributeUpdates>,
#[doc="<p>A condition that must be satisfied in order for a conditional update to succeed.</p> <p>An expression can contain any of the following:</p> <ul> <li> <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code> </p> <p>These function names are case-sensitive.</p> </li> <li> <p>Comparison operators: <code> = | &amp;#x3C;&amp;#x3E; | &amp;#x3C; | &amp;#x3E; | &amp;#x3C;= | &amp;#x3E;= | BETWEEN | IN</code> </p> </li> <li> <p> Logical operators: <code>AND | OR | NOT</code> </p> </li> </ul> <p>For more information on condition expressions, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html\">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>ConditionExpression</i> replaces the legacy <i>ConditionalOperator</i> and <i>Expected</i> parameters.</p> </note>"]
#[serde(rename="ConditionExpression")]
pub condition_expression: Option<ConditionExpression>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i>ConditionExpression</i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A logical operator to apply to the conditions in the <i>Expected</i> map:</p> <ul> <li> <p> <code>AND</code> - If all of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> <li> <p> <code>OR</code> - If at least one of the conditions evaluate to true, then the entire map evaluates to true.</p> </li> </ul> <p>If you omit <i>ConditionalOperator</i>, then <code>AND</code> is the default.</p> <p>The operation will succeed only if the entire map evaluates to true.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note>"]
#[serde(rename="ConditionalOperator")]
pub conditional_operator: Option<ConditionalOperator>,
#[doc="<important> <p>This is a legacy parameter, for backward compatibility. New applications should use <i> ConditionExpression </i> instead. Do not combine legacy parameters and expression parameters in a single API call; otherwise, DynamoDB will return a <i>ValidationException</i> exception.</p> </important> <p>A map of attribute/condition pairs. <i>Expected</i> provides a conditional block for the <i>UpdateItem</i> operation.</p> <p>Each element of <i>Expected</i> consists of an attribute name, a comparison operator, and one or more values. DynamoDB compares the attribute with the value(s) you supplied, using the comparison operator. For each <i>Expected</i> element, the result of the evaluation is either true or false.</p> <p>If you specify more than one element in the <i>Expected</i> map, then by default all of the conditions must evaluate to true. In other words, the conditions are ANDed together. (You can use the <i>ConditionalOperator</i> parameter to OR the conditions instead. If you do this, then at least one of the conditions must evaluate to true, rather than all of them.)</p> <p>If the <i>Expected</i> map evaluates to true, then the conditional operation succeeds; otherwise, it fails.</p> <p> <i>Expected</i> contains the following:</p> <ul> <li> <p> <i>AttributeValueList</i> - One or more values to evaluate against the supplied attribute. The number of values in the list depends on the <i>ComparisonOperator</i> being used.</p> <p>For type Number, value comparisons are numeric.</p> <p>String value comparisons for greater than, equals, or less than are based on ASCII character code values. For example, <code>a</code> is greater than <code>A</code>, and <code>a</code> is greater than <code>B</code>. For a list of code values, see <a href=\"http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters\">http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters</a>.</p> <p>For type Binary, DynamoDB treats each byte of the binary data as unsigned when it compares binary values.</p> </li> <li> <p> <i>ComparisonOperator</i> - A comparator for evaluating attributes in the <i>AttributeValueList</i>. When performing the comparison, DynamoDB uses strongly consistent reads.</p> <p>The following comparison operators are available:</p> <p> <code>EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS | BEGINS_WITH | IN | BETWEEN</code> </p> <p>The following are descriptions of each comparison operator.</p> <ul> <li> <p> <code>EQ</code> : Equal. <code>EQ</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NE</code> : Not equal. <code>NE</code> is supported for all datatypes, including lists and maps.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, Binary, String Set, Number Set, or Binary Set. If an item contains an <i>AttributeValue</i> of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not equal <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LE</code> : Less than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>LT</code> : Less than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GE</code> : Greater than or equal. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>GT</code> : Greater than. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not equal <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code>.</p> <p/> </li> <li> <p> <code>NOT_NULL</code> : The attribute exists. <code>NOT_NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the existence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NOT_NULL</code>, the result is a Boolean <i>true</i>. This result is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NOT_NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>NULL</code> : The attribute does not exist. <code>NULL</code> is supported for all datatypes, including lists and maps.</p> <note> <p>This operator tests for the nonexistence of an attribute, not its data type. If the data type of attribute \"<code>a</code>\" is null, and you evaluate it using <code>NULL</code>, the result is a Boolean <i>false</i>. This is because the attribute \"<code>a</code>\" exists; its data type is not relevant to the <code>NULL</code> comparison operator.</p> </note> </li> <li> <p> <code>CONTAINS</code> : Checks for a subsequence, or value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is of type String, then the operator checks for a substring match. If the target attribute of the comparison is of type Binary, then the operator looks for a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it finds an exact match with any member of the set.</p> <p>CONTAINS is supported for lists: When evaluating \"<code>a CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>NOT_CONTAINS</code> : Checks for absence of a subsequence, or absence of a value in a set.</p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> element of type String, Number, or Binary (not a set type). If the target attribute of the comparison is a String, then the operator checks for the absence of a substring match. If the target attribute of the comparison is Binary, then the operator checks for the absence of a subsequence of the target that matches the input. If the target attribute of the comparison is a set (\"<code>SS</code>\", \"<code>NS</code>\", or \"<code>BS</code>\"), then the operator evaluates to true if it <i>does not</i> find an exact match with any member of the set.</p> <p>NOT_CONTAINS is supported for lists: When evaluating \"<code>a NOT CONTAINS b</code>\", \"<code>a</code>\" can be a list; however, \"<code>b</code>\" cannot be a set, a map, or a list.</p> </li> <li> <p> <code>BEGINS_WITH</code> : Checks for a prefix. </p> <p> <i>AttributeValueList</i> can contain only one <i>AttributeValue</i> of type String or Binary (not a Number or a set type). The target attribute of the comparison must be of type String or Binary (not a Number or a set type).</p> <p/> </li> <li> <p> <code>IN</code> : Checks for matching elements within two sets.</p> <p> <i>AttributeValueList</i> can contain one or more <i>AttributeValue</i> elements of type String, Number, or Binary (not a set type). These attributes are compared against an existing set type attribute of an item. If any elements of the input set are present in the item attribute, the expression evaluates to true.</p> </li> <li> <p> <code>BETWEEN</code> : Greater than or equal to the first value, and less than or equal to the second value. </p> <p> <i>AttributeValueList</i> must contain two <i>AttributeValue</i> elements of the same type, either String, Number, or Binary (not a set type). A target attribute matches if the target value is greater than, or equal to, the first element and less than, or equal to, the second element. If an item contains an <i>AttributeValue</i> element of a different type than the one provided in the request, the value does not match. For example, <code>{\"S\":\"6\"}</code> does not compare to <code>{\"N\":\"6\"}</code>. Also, <code>{\"N\":\"6\"}</code> does not compare to <code>{\"NS\":[\"6\", \"2\", \"1\"]}</code> </p> </li> </ul> </li> </ul> <p>For usage examples of <i>AttributeValueList</i> and <i>ComparisonOperator</i>, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.html\">Legacy Conditional Parameters</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>For backward compatibility with previous DynamoDB releases, the following parameters can be used instead of <i>AttributeValueList</i> and <i>ComparisonOperator</i>:</p> <ul> <li> <p> <i>Value</i> - A value for DynamoDB to compare with an attribute.</p> </li> <li> <p> <i>Exists</i> - A Boolean value that causes DynamoDB to evaluate the value before attempting the conditional operation:</p> <ul> <li> <p>If <i>Exists</i> is <code>true</code>, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the condition evaluates to true; otherwise the condition evaluate to false.</p> </li> <li> <p>If <i>Exists</i> is <code>false</code>, DynamoDB assumes that the attribute value does <i>not</i> exist in the table. If in fact the value does not exist, then the assumption is valid and the condition evaluates to true. If the value is found, despite the assumption that it does not exist, the condition evaluates to false.</p> </li> </ul> <p>Note that the default value for <i>Exists</i> is <code>true</code>.</p> </li> </ul> <p>The <i>Value</i> and <i>Exists</i> parameters are incompatible with <i>AttributeValueList</i> and <i>ComparisonOperator</i>. Note that if you use both sets of parameters at once, DynamoDB will return a <i>ValidationException</i> exception.</p> <note> <p>This parameter does not support attributes of type List or Map.</p> </note>"]
#[serde(rename="Expected")]
pub expected: Option<ExpectedAttributeMap>,
#[doc="<p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p> </li> <li> <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p> </li> <li> <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p> </li> </ul> <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p> <ul> <li> <p> <code>Percentile</code> </p> </li> </ul> <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html\">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <i>ExpressionAttributeNames</i>:</p> <ul> <li> <p> <code>{\"#P\":\"Percentile\"}</code> </p> </li> </ul> <p>You could then use this substitution in an expression, as in this example:</p> <ul> <li> <p> <code>#P = :val</code> </p> </li> </ul> <note> <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p> </note> <p>For more information on expression attribute names, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html\">Accessing Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeNames")]
pub expression_attribute_names: Option<ExpressionAttributeNameMap>,
#[doc="<p>One or more values that can be substituted in an expression.</p> <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following: </p> <p> <code>Available | Backordered | Discontinued</code> </p> <p>You would first need to specify <i>ExpressionAttributeValues</i> as follows:</p> <p> <code>{ \":avail\":{\"S\":\"Available\"}, \":back\":{\"S\":\"Backordered\"}, \":disc\":{\"S\":\"Discontinued\"} }</code> </p> <p>You could then use these values in an expression, such as this:</p> <p> <code>ProductStatus IN (:avail, :back, :disc)</code> </p> <p>For more information on expression attribute values, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html\">Specifying Conditions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
#[serde(rename="ExpressionAttributeValues")]
pub expression_attribute_values: Option<ExpressionAttributeValueMap>,
#[doc="<p>The primary key of the item to be updated. Each element consists of an attribute name and a value for that attribute.</p> <p>For the primary key, you must provide all of the attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>"]
#[serde(rename="Key")]
pub key: Key,
#[serde(rename="ReturnConsumedCapacity")]
pub return_consumed_capacity: Option<ReturnConsumedCapacity>,
#[doc="<p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>"]
#[serde(rename="ReturnItemCollectionMetrics")]
pub return_item_collection_metrics: Option<ReturnItemCollectionMetrics>,
#[doc="<p>Use <i>ReturnValues</i> if you want to get the item attributes as they appeared either before or after they were updated. For <i>UpdateItem</i>, the valid values are:</p> <ul> <li> <p> <code>NONE</code> - If <i>ReturnValues</i> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <i>ReturnValues</i>.)</p> </li> <li> <p> <code>ALL_OLD</code> - If <i>UpdateItem</i> overwrote an attribute name-value pair, then the content of the old item is returned.</p> </li> <li> <p> <code>UPDATED_OLD</code> - The old versions of only the updated attributes are returned.</p> </li> <li> <p> <code>ALL_NEW</code> - All of the attributes of the new version of the item are returned.</p> </li> <li> <p> <code>UPDATED_NEW</code> - The new versions of only the updated attributes are returned.</p> </li> </ul> <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No Read Capacity Units are consumed.</p> <p>Values returned are strongly consistent</p>"]
#[serde(rename="ReturnValues")]
pub return_values: Option<ReturnValue>,
#[doc="<p>The name of the table containing the item to update.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
#[doc="<p>An expression that defines one or more attributes to be updated, the action to be performed on them, and new value(s) for them.</p> <p>The following action values are available for <i>UpdateExpression</i>.</p> <ul> <li> <p> <code>SET</code> - Adds one or more attributes and values to an item. If any of these attribute already exist, they are replaced by the new values. You can also use <code>SET</code> to add or subtract from an attribute that is of type Number. For example: <code>SET myNum = myNum + :val</code> </p> <p> <code>SET</code> supports the following functions:</p> <ul> <li> <p> <code>if_not_exists (path, operand)</code> - if the item does not contain an attribute at the specified path, then <code>if_not_exists</code> evaluates to operand; otherwise, it evaluates to path. You can use this function to avoid overwriting an attribute that may already be present in the item.</p> </li> <li> <p> <code>list_append (operand, operand)</code> - evaluates to a list with a new element added to it. You can append the new element to the start or the end of the list by reversing the order of the operands.</p> </li> </ul> <p>These function names are case-sensitive.</p> </li> <li> <p> <code>REMOVE</code> - Removes one or more attributes from an item.</p> </li> <li> <p> <code>ADD</code> - Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of <code>ADD</code> depends on the data type of the attribute:</p> <ul> <li> <p>If the existing attribute is a number, and if <i>Value</i> is also a number, then <i>Value</i> is mathematically added to the existing attribute. If <i>Value</i> is a negative number, then it is subtracted from the existing attribute.</p> <note> <p>If you use <code>ADD</code> to increment or decrement a number value for an item that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value.</p> <p>Similarly, if you use <code>ADD</code> for an existing item to increment or decrement an attribute value that doesn't exist before the update, DynamoDB uses <code>0</code> as the initial value. For example, suppose that the item you want to update doesn't have an attribute named <i>itemcount</i>, but you decide to <code>ADD</code> the number <code>3</code> to this attribute anyway. DynamoDB will create the <i>itemcount</i> attribute, set its initial value to <code>0</code>, and finally add <code>3</code> to it. The result will be a new <i>itemcount</i> attribute in the item, with a value of <code>3</code>.</p> </note> </li> <li> <p>If the existing data type is a set and if <i>Value</i> is also a set, then <i>Value</i> is added to the existing set. For example, if the attribute value is the set <code>[1,2]</code>, and the <code>ADD</code> action specified <code>[3]</code>, then the final attribute value is <code>[1,2,3]</code>. An error occurs if an <code>ADD</code> action is specified for a set attribute and the attribute type specified does not match the existing set type. </p> <p>Both sets must have the same primitive data type. For example, if the existing data type is a set of strings, the <i>Value</i> must also be a set of strings.</p> </li> </ul> <important> <p>The <code>ADD</code> action only supports Number and set data types. In addition, <code>ADD</code> can only be used on top-level attributes, not nested attributes.</p> </important> </li> <li> <p> <code>DELETE</code> - Deletes an element from a set.</p> <p>If a set of values is specified, then those values are subtracted from the old set. For example, if the attribute value was the set <code>[a,b,c]</code> and the <code>DELETE</code> action specifies <code>[a,c]</code>, then the final attribute value is <code>[b]</code>. Specifying an empty set is an error.</p> <important> <p>The <code>DELETE</code> action only supports set data types. In addition, <code>DELETE</code> can only be used on top-level attributes, not nested attributes.</p> </important> </li> </ul> <p>You can have many actions in a single expression, such as the following: <code>SET a=:value1, b=:value2 DELETE :value3, :value4, :value5</code> </p> <p>For more information on update expressions, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Modifying.html\">Modifying Items and Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <note> <p> <i>UpdateExpression</i> replaces the legacy <i>AttributeUpdates</i> parameter.</p> </note>"]
#[serde(rename="UpdateExpression")]
pub update_expression: Option<UpdateExpression>,
            }
            
#[doc="<p>Represents the output of an <i>UpdateItem</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateItemOutput {
                #[doc="<p>A map of attribute values as they appeared before the <i>UpdateItem</i> operation. This map only appears if <i>ReturnValues</i> was specified as something other than <code>NONE</code> in the request. Each element represents one attribute.</p>"]
#[serde(rename="Attributes")]
pub attributes: Option<AttributeMap>,
#[serde(rename="ConsumedCapacity")]
pub consumed_capacity: Option<ConsumedCapacity>,
#[serde(rename="ItemCollectionMetrics")]
pub item_collection_metrics: Option<ItemCollectionMetrics>,
            }
            
#[doc="<p>Represents the input of an <i>UpdateTable</i> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateTableInput {
                #[doc="<p>An array of attributes that describe the key schema for the table and indexes. If you are adding a new global secondary index to the table, <i>AttributeDefinitions</i> must include the key element(s) of the new index.</p>"]
#[serde(rename="AttributeDefinitions")]
pub attribute_definitions: Option<AttributeDefinitions>,
#[doc="<p>An array of one or more global secondary indexes for the table. For each index in the array, you can request one action:</p> <ul> <li> <p> <i>Create</i> - add a new global secondary index to the table.</p> </li> <li> <p> <i>Update</i> - modify the provisioned throughput settings of an existing global secondary index.</p> </li> <li> <p> <i>Delete</i> - remove a global secondary index from the table.</p> </li> </ul> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.OnlineOps.html\">Managing Global Secondary Indexes</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>"]
#[serde(rename="GlobalSecondaryIndexUpdates")]
pub global_secondary_index_updates: Option<GlobalSecondaryIndexUpdateList>,
#[serde(rename="ProvisionedThroughput")]
pub provisioned_throughput: Option<ProvisionedThroughput>,
#[doc="<p>Represents the DynamoDB Streams configuration for the table.</p> <note> <p>You will receive a <i>ResourceInUseException</i> if you attempt to enable a stream on a table that already has a stream, or if you attempt to disable a stream on a table which does not have a stream.</p> </note>"]
#[serde(rename="StreamSpecification")]
pub stream_specification: Option<StreamSpecification>,
#[doc="<p>The name of the table to be updated.</p>"]
#[serde(rename="TableName")]
pub table_name: TableName,
            }
            
#[doc="<p>Represents the output of an <i>UpdateTable</i> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateTableOutput {
                #[serde(rename="TableDescription")]
pub table_description: Option<TableDescription>,
            }
            
#[doc="<p>Represents an operation to perform - either <i>DeleteItem</i> or <i>PutItem</i>. You can only request one of these operations, not both, in a single <i>WriteRequest</i>. If you do need to perform both of these operations, you will need to provide two separate <i>WriteRequest</i> objects.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct WriteRequest {
                #[doc="<p>A request to perform a <i>DeleteItem</i> operation.</p>"]
#[serde(rename="DeleteRequest")]
pub delete_request: Option<DeleteRequest>,
#[doc="<p>A request to perform a <i>PutItem</i> operation.</p>"]
#[serde(rename="PutRequest")]
pub put_request: Option<PutRequest>,
            }
            
pub type WriteRequests = Vec<WriteRequest>;
/// Errors returned by BatchGetItem
                #[derive(Debug, PartialEq)]
                pub enum BatchGetItemError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
ProvisionedThroughputExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchGetItemError {
                    pub fn from_body(body: &str) -> BatchGetItemError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => BatchGetItemError::InternalServerError(String::from(error_message)),"ResourceNotFoundException" => BatchGetItemError::ResourceNotFound(String::from(error_message)),"ProvisionedThroughputExceededException" => BatchGetItemError::ProvisionedThroughputExceeded(String::from(error_message)),"ValidationException" => BatchGetItemError::Validation(error_message.to_string()),_ => BatchGetItemError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => BatchGetItemError::Unknown(String::from(body))
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
                impl fmt::Display for BatchGetItemError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchGetItemError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchGetItemError::InternalServerError(ref cause) => cause,BatchGetItemError::ProvisionedThroughputExceeded(ref cause) => cause,BatchGetItemError::ResourceNotFound(ref cause) => cause,BatchGetItemError::Validation(ref cause) => cause,BatchGetItemError::Credentials(ref err) => err.description(),BatchGetItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchGetItemError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by BatchWriteItem
                #[derive(Debug, PartialEq)]
                pub enum BatchWriteItemError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
ProvisionedThroughputExceeded(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
ItemCollectionSizeLimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl BatchWriteItemError {
                    pub fn from_body(body: &str) -> BatchWriteItemError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ItemCollectionSizeLimitExceededException" => BatchWriteItemError::ItemCollectionSizeLimitExceeded(String::from(error_message)),"ResourceNotFoundException" => BatchWriteItemError::ResourceNotFound(String::from(error_message)),"ProvisionedThroughputExceededException" => BatchWriteItemError::ProvisionedThroughputExceeded(String::from(error_message)),"InternalServerError" => BatchWriteItemError::InternalServerError(String::from(error_message)),"ValidationException" => BatchWriteItemError::Validation(error_message.to_string()),_ => BatchWriteItemError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => BatchWriteItemError::Unknown(String::from(body))
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
                impl fmt::Display for BatchWriteItemError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for BatchWriteItemError {
                    fn description(&self) -> &str {
                        match *self {
                            BatchWriteItemError::ItemCollectionSizeLimitExceeded(ref cause) => cause,BatchWriteItemError::ProvisionedThroughputExceeded(ref cause) => cause,BatchWriteItemError::ResourceNotFound(ref cause) => cause,BatchWriteItemError::InternalServerError(ref cause) => cause,BatchWriteItemError::Validation(ref cause) => cause,BatchWriteItemError::Credentials(ref err) => err.description(),BatchWriteItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),BatchWriteItemError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateTable
                #[derive(Debug, PartialEq)]
                pub enum CreateTableError {
                    
///<p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
ResourceInUse(String),
///<p>The number of concurrent table requests (cumulative number of tables in the <code>CREATING</code>, <code>DELETING</code> or <code>UPDATING</code> state) exceeds the maximum allowed of 10.</p> <p>Also, for tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
LimitExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateTableError {
                    pub fn from_body(body: &str) -> CreateTableError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "LimitExceededException" => CreateTableError::LimitExceeded(String::from(error_message)),"InternalServerError" => CreateTableError::InternalServerError(String::from(error_message)),"ResourceInUseException" => CreateTableError::ResourceInUse(String::from(error_message)),"ValidationException" => CreateTableError::Validation(error_message.to_string()),_ => CreateTableError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateTableError::Unknown(String::from(body))
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
                impl fmt::Display for CreateTableError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateTableError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateTableError::ResourceInUse(ref cause) => cause,CreateTableError::InternalServerError(ref cause) => cause,CreateTableError::LimitExceeded(ref cause) => cause,CreateTableError::Validation(ref cause) => cause,CreateTableError::Credentials(ref err) => err.description(),CreateTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateTableError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteItem
                #[derive(Debug, PartialEq)]
                pub enum DeleteItemError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
ItemCollectionSizeLimitExceeded(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>A condition specified in the operation could not be evaluated.</p>
ConditionalCheckFailed(String),
///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
ProvisionedThroughputExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteItemError {
                    pub fn from_body(body: &str) -> DeleteItemError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ConditionalCheckFailedException" => DeleteItemError::ConditionalCheckFailed(String::from(error_message)),"ProvisionedThroughputExceededException" => DeleteItemError::ProvisionedThroughputExceeded(String::from(error_message)),"InternalServerError" => DeleteItemError::InternalServerError(String::from(error_message)),"ResourceNotFoundException" => DeleteItemError::ResourceNotFound(String::from(error_message)),"ItemCollectionSizeLimitExceededException" => DeleteItemError::ItemCollectionSizeLimitExceeded(String::from(error_message)),"ValidationException" => DeleteItemError::Validation(error_message.to_string()),_ => DeleteItemError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteItemError::Unknown(String::from(body))
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
                impl fmt::Display for DeleteItemError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteItemError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteItemError::ItemCollectionSizeLimitExceeded(ref cause) => cause,DeleteItemError::ResourceNotFound(ref cause) => cause,DeleteItemError::ProvisionedThroughputExceeded(ref cause) => cause,DeleteItemError::ConditionalCheckFailed(ref cause) => cause,DeleteItemError::InternalServerError(ref cause) => cause,DeleteItemError::Validation(ref cause) => cause,DeleteItemError::Credentials(ref err) => err.description(),DeleteItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteItemError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteTable
                #[derive(Debug, PartialEq)]
                pub enum DeleteTableError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
ResourceInUse(String),
///<p>The number of concurrent table requests (cumulative number of tables in the <code>CREATING</code>, <code>DELETING</code> or <code>UPDATING</code> state) exceeds the maximum allowed of 10.</p> <p>Also, for tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
LimitExceeded(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteTableError {
                    pub fn from_body(body: &str) -> DeleteTableError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DeleteTableError::InternalServerError(String::from(error_message)),"ResourceInUseException" => DeleteTableError::ResourceInUse(String::from(error_message)),"LimitExceededException" => DeleteTableError::LimitExceeded(String::from(error_message)),"ResourceNotFoundException" => DeleteTableError::ResourceNotFound(String::from(error_message)),"ValidationException" => DeleteTableError::Validation(error_message.to_string()),_ => DeleteTableError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteTableError::Unknown(String::from(body))
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
                impl fmt::Display for DeleteTableError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteTableError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteTableError::ResourceInUse(ref cause) => cause,DeleteTableError::ResourceNotFound(ref cause) => cause,DeleteTableError::InternalServerError(ref cause) => cause,DeleteTableError::LimitExceeded(ref cause) => cause,DeleteTableError::Validation(ref cause) => cause,DeleteTableError::Credentials(ref err) => err.description(),DeleteTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteTableError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeLimits
                #[derive(Debug, PartialEq)]
                pub enum DescribeLimitsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeLimitsError {
                    pub fn from_body(body: &str) -> DescribeLimitsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeLimitsError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeLimitsError::Validation(error_message.to_string()),_ => DescribeLimitsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeLimitsError::Unknown(String::from(body))
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
                impl fmt::Display for DescribeLimitsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeLimitsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeLimitsError::InternalServerError(ref cause) => cause,DescribeLimitsError::Validation(ref cause) => cause,DescribeLimitsError::Credentials(ref err) => err.description(),DescribeLimitsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeLimitsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeTable
                #[derive(Debug, PartialEq)]
                pub enum DescribeTableError {
                    
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeTableError {
                    pub fn from_body(body: &str) -> DescribeTableError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceNotFoundException" => DescribeTableError::ResourceNotFound(String::from(error_message)),"InternalServerError" => DescribeTableError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeTableError::Validation(error_message.to_string()),_ => DescribeTableError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeTableError::Unknown(String::from(body))
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
                impl fmt::Display for DescribeTableError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeTableError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeTableError::ResourceNotFound(ref cause) => cause,DescribeTableError::InternalServerError(ref cause) => cause,DescribeTableError::Validation(ref cause) => cause,DescribeTableError::Credentials(ref err) => err.description(),DescribeTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeTableError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetItem
                #[derive(Debug, PartialEq)]
                pub enum GetItemError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
ProvisionedThroughputExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetItemError {
                    pub fn from_body(body: &str) -> GetItemError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ProvisionedThroughputExceededException" => GetItemError::ProvisionedThroughputExceeded(String::from(error_message)),"InternalServerError" => GetItemError::InternalServerError(String::from(error_message)),"ResourceNotFoundException" => GetItemError::ResourceNotFound(String::from(error_message)),"ValidationException" => GetItemError::Validation(error_message.to_string()),_ => GetItemError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetItemError::Unknown(String::from(body))
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
                impl fmt::Display for GetItemError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetItemError {
                    fn description(&self) -> &str {
                        match *self {
                            GetItemError::ProvisionedThroughputExceeded(ref cause) => cause,GetItemError::ResourceNotFound(ref cause) => cause,GetItemError::InternalServerError(ref cause) => cause,GetItemError::Validation(ref cause) => cause,GetItemError::Credentials(ref err) => err.description(),GetItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetItemError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTables
                #[derive(Debug, PartialEq)]
                pub enum ListTablesError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTablesError {
                    pub fn from_body(body: &str) -> ListTablesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ListTablesError::InternalServerError(String::from(error_message)),"ValidationException" => ListTablesError::Validation(error_message.to_string()),_ => ListTablesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListTablesError::Unknown(String::from(body))
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
                impl fmt::Display for ListTablesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTablesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTablesError::InternalServerError(ref cause) => cause,ListTablesError::Validation(ref cause) => cause,ListTablesError::Credentials(ref err) => err.description(),ListTablesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTablesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutItem
                #[derive(Debug, PartialEq)]
                pub enum PutItemError {
                    
///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
ProvisionedThroughputExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>A condition specified in the operation could not be evaluated.</p>
ConditionalCheckFailed(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
ItemCollectionSizeLimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutItemError {
                    pub fn from_body(body: &str) -> PutItemError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceNotFoundException" => PutItemError::ResourceNotFound(String::from(error_message)),"ProvisionedThroughputExceededException" => PutItemError::ProvisionedThroughputExceeded(String::from(error_message)),"ConditionalCheckFailedException" => PutItemError::ConditionalCheckFailed(String::from(error_message)),"ItemCollectionSizeLimitExceededException" => PutItemError::ItemCollectionSizeLimitExceeded(String::from(error_message)),"InternalServerError" => PutItemError::InternalServerError(String::from(error_message)),"ValidationException" => PutItemError::Validation(error_message.to_string()),_ => PutItemError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => PutItemError::Unknown(String::from(body))
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
                impl fmt::Display for PutItemError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutItemError {
                    fn description(&self) -> &str {
                        match *self {
                            PutItemError::ResourceNotFound(ref cause) => cause,PutItemError::InternalServerError(ref cause) => cause,PutItemError::ItemCollectionSizeLimitExceeded(ref cause) => cause,PutItemError::ProvisionedThroughputExceeded(ref cause) => cause,PutItemError::ConditionalCheckFailed(ref cause) => cause,PutItemError::Validation(ref cause) => cause,PutItemError::Credentials(ref err) => err.description(),PutItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),PutItemError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by Query
                #[derive(Debug, PartialEq)]
                pub enum QueryError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
ProvisionedThroughputExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl QueryError {
                    pub fn from_body(body: &str) -> QueryError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ProvisionedThroughputExceededException" => QueryError::ProvisionedThroughputExceeded(String::from(error_message)),"InternalServerError" => QueryError::InternalServerError(String::from(error_message)),"ResourceNotFoundException" => QueryError::ResourceNotFound(String::from(error_message)),"ValidationException" => QueryError::Validation(error_message.to_string()),_ => QueryError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => QueryError::Unknown(String::from(body))
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
                impl fmt::Display for QueryError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for QueryError {
                    fn description(&self) -> &str {
                        match *self {
                            QueryError::ProvisionedThroughputExceeded(ref cause) => cause,QueryError::ResourceNotFound(ref cause) => cause,QueryError::InternalServerError(ref cause) => cause,QueryError::Validation(ref cause) => cause,QueryError::Credentials(ref err) => err.description(),QueryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),QueryError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by Scan
                #[derive(Debug, PartialEq)]
                pub enum ScanError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
ProvisionedThroughputExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ScanError {
                    pub fn from_body(body: &str) -> ScanError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ScanError::InternalServerError(String::from(error_message)),"ProvisionedThroughputExceededException" => ScanError::ProvisionedThroughputExceeded(String::from(error_message)),"ResourceNotFoundException" => ScanError::ResourceNotFound(String::from(error_message)),"ValidationException" => ScanError::Validation(error_message.to_string()),_ => ScanError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ScanError::Unknown(String::from(body))
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
                impl fmt::Display for ScanError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ScanError {
                    fn description(&self) -> &str {
                        match *self {
                            ScanError::InternalServerError(ref cause) => cause,ScanError::ResourceNotFound(ref cause) => cause,ScanError::ProvisionedThroughputExceeded(ref cause) => cause,ScanError::Validation(ref cause) => cause,ScanError::Credentials(ref err) => err.description(),ScanError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ScanError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateItem
                #[derive(Debug, PartialEq)]
                pub enum UpdateItemError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>An item collection is too large. This exception is only returned for tables that have one or more local secondary indexes.</p>
ItemCollectionSizeLimitExceeded(String),
///<p>A condition specified in the operation could not be evaluated.</p>
ConditionalCheckFailed(String),
///<p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#APIRetries">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
ProvisionedThroughputExceeded(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateItemError {
                    pub fn from_body(body: &str) -> UpdateItemError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ConditionalCheckFailedException" => UpdateItemError::ConditionalCheckFailed(String::from(error_message)),"InternalServerError" => UpdateItemError::InternalServerError(String::from(error_message)),"ProvisionedThroughputExceededException" => UpdateItemError::ProvisionedThroughputExceeded(String::from(error_message)),"ItemCollectionSizeLimitExceededException" => UpdateItemError::ItemCollectionSizeLimitExceeded(String::from(error_message)),"ResourceNotFoundException" => UpdateItemError::ResourceNotFound(String::from(error_message)),"ValidationException" => UpdateItemError::Validation(error_message.to_string()),_ => UpdateItemError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateItemError::Unknown(String::from(body))
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
                impl fmt::Display for UpdateItemError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateItemError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateItemError::ConditionalCheckFailed(ref cause) => cause,UpdateItemError::ItemCollectionSizeLimitExceeded(ref cause) => cause,UpdateItemError::ProvisionedThroughputExceeded(ref cause) => cause,UpdateItemError::ResourceNotFound(ref cause) => cause,UpdateItemError::InternalServerError(ref cause) => cause,UpdateItemError::Validation(ref cause) => cause,UpdateItemError::Credentials(ref err) => err.description(),UpdateItemError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateItemError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateTable
                #[derive(Debug, PartialEq)]
                pub enum UpdateTableError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The operation conflicts with the resource's availability. For example, you attempted to recreate an existing table, or tried to delete a table currently in the <code>CREATING</code> state.</p>
ResourceInUse(String),
///<p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
ResourceNotFound(String),
///<p>The number of concurrent table requests (cumulative number of tables in the <code>CREATING</code>, <code>DELETING</code> or <code>UPDATING</code> state) exceeds the maximum allowed of 10.</p> <p>Also, for tables with secondary indexes, only one of those tables can be in the <code>CREATING</code> state at any point in time. Do not attempt to create more than one such table simultaneously.</p> <p>The total limit of tables in the <code>ACTIVE</code> state is 250.</p>
LimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateTableError {
                    pub fn from_body(body: &str) -> UpdateTableError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceInUseException" => UpdateTableError::ResourceInUse(String::from(error_message)),"ResourceNotFoundException" => UpdateTableError::ResourceNotFound(String::from(error_message)),"LimitExceededException" => UpdateTableError::LimitExceeded(String::from(error_message)),"InternalServerError" => UpdateTableError::InternalServerError(String::from(error_message)),"ValidationException" => UpdateTableError::Validation(error_message.to_string()),_ => UpdateTableError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateTableError::Unknown(String::from(body))
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
                impl fmt::Display for UpdateTableError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateTableError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateTableError::ResourceInUse(ref cause) => cause,UpdateTableError::LimitExceeded(ref cause) => cause,UpdateTableError::ResourceNotFound(ref cause) => cause,UpdateTableError::InternalServerError(ref cause) => cause,UpdateTableError::Validation(ref cause) => cause,UpdateTableError::Credentials(ref err) => err.description(),UpdateTableError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateTableError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Trait representing the capabilities of the DynamoDB API. DynamoDB clients implement this trait.
        pub trait DynamoDb {
        

                #[doc="<p>The <i>BatchGetItem</i> operation returns the attributes of one or more items from one or more tables. You identify requested items by primary key.</p> <p>A single operation can retrieve up to 16 MB of data, which can contain as many as 100 items. <i>BatchGetItem</i> will return a partial result if the response size limit is exceeded, the table's provisioned throughput is exceeded, or an internal processing failure occurs. If a partial result is returned, the operation returns a value for <i>UnprocessedKeys</i>. You can use this value to retry the operation starting with the next item to get.</p> <important> <p>If you request more than 100 items <i>BatchGetItem</i> will return a <i>ValidationException</i> with the message \"Too many items requested for the BatchGetItem call\".</p> </important> <p>For example, if you ask to retrieve 100 items, but each individual item is 300 KB in size, the system returns 52 items (so as not to exceed the 16 MB limit). It also returns an appropriate <i>UnprocessedKeys</i> value so you can get the next page of results. If desired, your application can include its own logic to assemble the pages of results into one data set.</p> <p>If <i>none</i> of the items can be processed due to insufficient provisioned throughput on all of the tables in the request, then <i>BatchGetItem</i> will return a <i>ProvisionedThroughputExceededException</i>. If <i>at least one</i> of the items is successfully processed, then <i>BatchGetItem</i> completes successfully, while returning the keys of the unread items in <i>UnprocessedKeys</i>.</p> <important> <p>If DynamoDB returns any unprocessed items, you should retry the batch operation on those items. However, <i>we strongly recommend that you use an exponential backoff algorithm</i>. If you retry the batch operation immediately, the underlying read or write requests can still fail due to throttling on the individual tables. If you delay the batch operation using exponential backoff, the individual requests in the batch are much more likely to succeed.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOperations\">Batch Operations and Error Handling</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </important> <p>By default, <i>BatchGetItem</i> performs eventually consistent reads on every table in the request. If you want strongly consistent reads instead, you can set <i>ConsistentRead</i> to <code>true</code> for any or all tables.</p> <p>In order to minimize response latency, <i>BatchGetItem</i> retrieves items in parallel.</p> <p>When designing your application, keep in mind that DynamoDB does not return items in any particular order. To help parse the response by item, include the primary key values for the items in your request in the <i>AttributesToGet</i> parameter.</p> <p>If a requested item does not exist, it is not returned in the result. Requests for nonexistent items consume the minimum read capacity units according to the type of read. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#CapacityUnitCalculations\">Capacity Units Calculations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
                fn batch_get_item(&self, input: &BatchGetItemInput)  -> Result<BatchGetItemOutput, BatchGetItemError>;
                

                #[doc="<p>The <i>BatchWriteItem</i> operation puts or deletes multiple items in one or more tables. A single call to <i>BatchWriteItem</i> can write up to 16 MB of data, which can comprise as many as 25 put or delete requests. Individual items to be written can be as large as 400 KB.</p> <note> <p> <i>BatchWriteItem</i> cannot update items. To update items, use the <i>UpdateItem</i> API.</p> </note> <p>The individual <i>PutItem</i> and <i>DeleteItem</i> operations specified in <i>BatchWriteItem</i> are atomic; however <i>BatchWriteItem</i> as a whole is not. If any requested operations fail because the table's provisioned throughput is exceeded or an internal processing failure occurs, the failed operations are returned in the <i>UnprocessedItems</i> response parameter. You can investigate and optionally resend the requests. Typically, you would call <i>BatchWriteItem</i> in a loop. Each iteration would check for unprocessed items and submit a new <i>BatchWriteItem</i> request with those unprocessed items until all items have been processed.</p> <p>Note that if <i>none</i> of the items can be processed due to insufficient provisioned throughput on all of the tables in the request, then <i>BatchWriteItem</i> will return a <i>ProvisionedThroughputExceededException</i>.</p> <important> <p>If DynamoDB returns any unprocessed items, you should retry the batch operation on those items. However, <i>we strongly recommend that you use an exponential backoff algorithm</i>. If you retry the batch operation immediately, the underlying read or write requests can still fail due to throttling on the individual tables. If you delay the batch operation using exponential backoff, the individual requests in the batch are much more likely to succeed.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOperations\">Batch Operations and Error Handling</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </important> <p>With <i>BatchWriteItem</i>, you can efficiently write or delete large amounts of data, such as from Amazon Elastic MapReduce (EMR), or copy data from another database into DynamoDB. In order to improve performance with these large-scale operations, <i>BatchWriteItem</i> does not behave in the same way as individual <i>PutItem</i> and <i>DeleteItem</i> calls would. For example, you cannot specify conditions on individual put and delete requests, and <i>BatchWriteItem</i> does not return deleted items in the response.</p> <p>If you use a programming language that supports concurrency, you can use threads to write items in parallel. Your application must include the necessary logic to manage the threads. With languages that don't support threading, you must update or delete the specified items one at a time. In both situations, <i>BatchWriteItem</i> provides an alternative where the API performs the specified put and delete operations in parallel, giving you the power of the thread pool approach without having to introduce complexity into your application.</p> <p>Parallel processing reduces latency, but each specified put and delete request consumes the same number of write capacity units whether it is processed in parallel or not. Delete operations on nonexistent items consume one write capacity unit.</p> <p>If one or more of the following is true, DynamoDB rejects the entire batch write operation:</p> <ul> <li> <p>One or more tables specified in the <i>BatchWriteItem</i> request does not exist.</p> </li> <li> <p>Primary key attributes specified on an item in the request do not match those in the corresponding table's primary key schema.</p> </li> <li> <p>You try to perform multiple operations on the same item in the same <i>BatchWriteItem</i> request. For example, you cannot put and delete the same item in the same <i>BatchWriteItem</i> request. </p> </li> <li> <p>There are more than 25 requests in the batch.</p> </li> <li> <p>Any individual item in a batch exceeds 400 KB.</p> </li> <li> <p>The total request size exceeds 16 MB.</p> </li> </ul>"]
                fn batch_write_item(&self, input: &BatchWriteItemInput)  -> Result<BatchWriteItemOutput, BatchWriteItemError>;
                

                #[doc="<p>The <i>CreateTable</i> operation adds a new table to your account. In an AWS account, table names must be unique within each region. That is, you can have two tables with same name if you create the tables in different regions.</p> <p> <i>CreateTable</i> is an asynchronous operation. Upon receiving a <i>CreateTable</i> request, DynamoDB immediately returns a response with a <i>TableStatus</i> of <code>CREATING</code>. After the table is created, DynamoDB sets the <i>TableStatus</i> to <code>ACTIVE</code>. You can perform read and write operations only on an <code>ACTIVE</code> table. </p> <p>You can optionally define secondary indexes on the new table, as part of the <i>CreateTable</i> operation. If you want to create multiple tables with secondary indexes on them, you must create the tables sequentially. Only one table with secondary indexes can be in the <code>CREATING</code> state at any given time.</p> <p>You can use the <i>DescribeTable</i> API to check the table status.</p>"]
                fn create_table(&self, input: &CreateTableInput)  -> Result<CreateTableOutput, CreateTableError>;
                

                #[doc="<p>Deletes a single item in a table by primary key. You can perform a conditional delete operation that deletes the item if it exists, or if it has an expected attribute value.</p> <p>In addition to deleting an item, you can also return the item's attribute values in the same operation, using the <i>ReturnValues</i> parameter.</p> <p>Unless you specify conditions, the <i>DeleteItem</i> is an idempotent operation; running it multiple times on the same item or attribute does <i>not</i> result in an error response.</p> <p>Conditional deletes are useful for deleting items only if specific conditions are met. If those conditions are met, DynamoDB performs the delete. Otherwise, the item is not deleted.</p>"]
                fn delete_item(&self, input: &DeleteItemInput)  -> Result<DeleteItemOutput, DeleteItemError>;
                

                #[doc="<p>The <i>DeleteTable</i> operation deletes a table and all of its items. After a <i>DeleteTable</i> request, the specified table is in the <code>DELETING</code> state until DynamoDB completes the deletion. If the table is in the <code>ACTIVE</code> state, you can delete it. If a table is in <code>CREATING</code> or <code>UPDATING</code> states, then DynamoDB returns a <i>ResourceInUseException</i>. If the specified table does not exist, DynamoDB returns a <i>ResourceNotFoundException</i>. If table is already in the <code>DELETING</code> state, no error is returned. </p> <note> <p>DynamoDB might continue to accept data read and write operations, such as <i>GetItem</i> and <i>PutItem</i>, on a table in the <code>DELETING</code> state until the table deletion is complete.</p> </note> <p>When you delete a table, any indexes on that table are also deleted.</p> <p>If you have DynamoDB Streams enabled on the table, then the corresponding stream on that table goes into the <code>DISABLED</code> state, and the stream is automatically deleted after 24 hours.</p> <p>Use the <i>DescribeTable</i> API to check the status of the table. </p>"]
                fn delete_table(&self, input: &DeleteTableInput)  -> Result<DeleteTableOutput, DeleteTableError>;
                

                #[doc="<p>Returns the current provisioned-capacity limits for your AWS account in a region, both for the region as a whole and for any one DynamoDB table that you create there.</p> <p>When you establish an AWS account, the account has initial limits on the maximum read capacity units and write capacity units that you can provision across all of your DynamoDB tables in a given region. Also, there are per-table limits that apply when you create a table there. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html\">Limits</a> page in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Although you can increase these limits by filing a case at <a href=\"https://console.aws.amazon.com/support/home#/\">AWS Support Center</a>, obtaining the increase is not instantaneous. The <i>DescribeLimits</i> API lets you write code to compare the capacity you are currently using to those limits imposed by your account so that you have enough time to apply for an increase before you hit a limit.</p> <p>For example, you could use one of the AWS SDKs to do the following:</p> <ol> <li><p>Call <i>DescribeLimits</i> for a particular region to obtain your current account limits on provisioned capacity there.</p> </li> <li><p>Create a variable to hold the aggregate read capacity units provisioned for all your tables in that region, and one to hold the aggregate write capacity units. Zero them both.</p> </li> <li><p>Call <i>ListTables</i> to obtain a list of all your DynamoDB tables.</p> </li> <li> <p>For each table name listed by <i>ListTables</i>, do the following:</p> <ul> <li><p>Call <i>DescribeTable</i> with the table name.</p> </li> <li><p>Use the data returned by <i>DescribeTable</i> to add the read capacity units and write capacity units provisioned for the table itself to your variables.</p> </li> <li><p>If the table has one or more global secondary indexes (GSIs), loop over these GSIs and add their provisioned capacity values to your variables as well.</p> </li> </ul> </li> <li><p>Report the account limits for that region returned by <i>DescribeLimits</i>, along with the total current provisioned capacity levels you have calculated.</p> </li> </ol> <p>This will let you see whether you are getting close to your account-level limits.</p> <p>The per-table limits apply only when you are creating a new table. They restrict the sum of the provisioned capacity of the new table itself and all its global secondary indexes.</p> <p>For existing tables and their GSIs, DynamoDB will not let you increase provisioned capacity extremely rapidly, but the only upper limit that applies is that the aggregate provisioned capacity over all your tables and GSIs cannot exceed either of the per-account limits.</p> <note> <p> <i>DescribeLimits</i> should only be called periodically. You can expect throttling errors if you call it more than once in a minute.</p> </note> <p>The <i>DescribeLimits</i> Request element has no content.</p>"]
                fn describe_limits(&self, input: &DescribeLimitsInput)  -> Result<DescribeLimitsOutput, DescribeLimitsError>;
                

                #[doc="<p>Returns information about the table, including the current status of the table, when it was created, the primary key schema, and any indexes on the table.</p> <note> <p>If you issue a <i>DescribeTable</i> request immediately after a <i>CreateTable</i> request, DynamoDB might return a <i>ResourceNotFoundException</i>. This is because <i>DescribeTable</i> uses an eventually consistent query, and the metadata for your table might not be available at that moment. Wait for a few seconds, and then try the <i>DescribeTable</i> request again.</p> </note>"]
                fn describe_table(&self, input: &DescribeTableInput)  -> Result<DescribeTableOutput, DescribeTableError>;
                

                #[doc="<p>The <i>GetItem</i> operation returns a set of attributes for the item with the given primary key. If there is no matching item, <i>GetItem</i> does not return any data.</p> <p> <i>GetItem</i> provides an eventually consistent read by default. If your application requires a strongly consistent read, set <i>ConsistentRead</i> to <code>true</code>. Although a strongly consistent read might take more time than an eventually consistent read, it always returns the last updated value.</p>"]
                fn get_item(&self, input: &GetItemInput)  -> Result<GetItemOutput, GetItemError>;
                

                #[doc="<p>Returns an array of table names associated with the current account and endpoint. The output from <i>ListTables</i> is paginated, with each page returning a maximum of 100 table names.</p>"]
                fn list_tables(&self, input: &ListTablesInput)  -> Result<ListTablesOutput, ListTablesError>;
                

                #[doc="<p>Creates a new item, or replaces an old item with a new item. If an item that has the same primary key as the new item already exists in the specified table, the new item completely replaces the existing item. You can perform a conditional put operation (add a new item if one with the specified primary key doesn't exist), or replace an existing item if it has certain attribute values.</p> <p>In addition to putting an item, you can also return the item's attribute values in the same operation, using the <i>ReturnValues</i> parameter.</p> <p>When you add an item, the primary key attribute(s) are the only required attributes. Attribute values cannot be null. String and Binary type attributes must have lengths greater than zero. Set type attributes cannot be empty. Requests with empty values will be rejected with a <i>ValidationException</i> exception.</p> <p>You can request that <i>PutItem</i> return either a copy of the original item (before the update) or a copy of the updated item (after the update). For more information, see the <i>ReturnValues</i> description below.</p> <note> <p>To prevent a new item from replacing an existing item, use a conditional expression that contains the <code>attribute_not_exists</code> function with the name of the attribute being used as the partition key for the table. Since every record must contain that attribute, the <code>attribute_not_exists</code> function will only succeed if no matching item exists.</p> </note> <p>For more information about using this API, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithItems.html\">Working with Items</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
                fn put_item(&self, input: &PutItemInput)  -> Result<PutItemOutput, PutItemError>;
                

                #[doc="<p>A <i>Query</i> operation uses the primary key of a table or a secondary index to directly access items from that table or index.</p> <p>Use the <i>KeyConditionExpression</i> parameter to provide a specific value for the partition key. The <i>Query</i> operation will return all of the items from the table or index with that partition key value. You can optionally narrow the scope of the <i>Query</i> operation by specifying a sort key value and a comparison operator in <i>KeyConditionExpression</i>. You can use the <i>ScanIndexForward</i> parameter to get results in forward or reverse order, by sort key.</p> <p>Queries that do not return results consume the minimum number of read capacity units for that type of read operation.</p> <p>If the total number of items meeting the query criteria exceeds the result set size limit of 1 MB, the query stops and results are returned to the user with the <i>LastEvaluatedKey</i> element to continue the query in a subsequent operation. Unlike a <i>Scan</i> operation, a <i>Query</i> operation never returns both an empty result set and a <i>LastEvaluatedKey</i> value. <i>LastEvaluatedKey</i> is only provided if you have used the <i>Limit</i> parameter, or if the result set exceeds 1 MB (prior to applying a filter). </p> <p>You can query a table, a local secondary index, or a global secondary index. For a query on a table or on a local secondary index, you can set the <i>ConsistentRead</i> parameter to <code>true</code> and obtain a strongly consistent result. Global secondary indexes support eventually consistent reads only, so do not specify <i>ConsistentRead</i> when querying a global secondary index.</p>"]
                fn query(&self, input: &QueryInput)  -> Result<QueryOutput, QueryError>;
                

                #[doc="<p>The <i>Scan</i> operation returns one or more items and item attributes by accessing every item in a table or a secondary index. To have DynamoDB return fewer items, you can provide a <i>ScanFilter</i> operation.</p> <p>If the total number of scanned items exceeds the maximum data set size limit of 1 MB, the scan stops and results are returned to the user as a <i>LastEvaluatedKey</i> value to continue the scan in a subsequent operation. The results also include the number of items exceeding the limit. A scan can result in no table data meeting the filter criteria. </p> <p>By default, <i>Scan</i> operations proceed sequentially; however, for faster performance on a large table or secondary index, applications can request a parallel <i>Scan</i> operation by providing the <i>Segment</i> and <i>TotalSegments</i> parameters. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#QueryAndScanParallelScan\">Parallel Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>By default, <i>Scan</i> uses eventually consistent reads when accessing the data in a table; therefore, the result set might not include the changes to data in the table immediately before the operation began. If you need a consistent copy of the data, as of the time that the Scan begins, you can set the <i>ConsistentRead</i> parameter to <i>true</i>.</p>"]
                fn scan(&self, input: &ScanInput)  -> Result<ScanOutput, ScanError>;
                

                #[doc="<p>Edits an existing item's attributes, or adds a new item to the table if it does not already exist. You can put, delete, or add attribute values. You can also perform a conditional update on an existing item (insert a new attribute name-value pair if it doesn't exist, or replace an existing name-value pair if it has certain expected attribute values).</p> <p>You can also return the item's attribute values in the same <i>UpdateItem</i> operation using the <i>ReturnValues</i> parameter.</p>"]
                fn update_item(&self, input: &UpdateItemInput)  -> Result<UpdateItemOutput, UpdateItemError>;
                

                #[doc="<p>Modifies the provisioned throughput settings, global secondary indexes, or DynamoDB Streams settings for a given table.</p> <p>You can only perform one of the following operations at once:</p> <ul> <li> <p>Modify the provisioned throughput settings of the table.</p> </li> <li> <p>Enable or disable Streams on the table.</p> </li> <li> <p>Remove a global secondary index from the table.</p> </li> <li> <p>Create a new global secondary index on the table. Once the index begins backfilling, you can use <i>UpdateTable</i> to perform other operations.</p> </li> </ul> <p> <i>UpdateTable</i> is an asynchronous operation; while it is executing, the table status changes from <code>ACTIVE</code> to <code>UPDATING</code>. While it is <code>UPDATING</code>, you cannot issue another <i>UpdateTable</i> request. When the table returns to the <code>ACTIVE</code> state, the <i>UpdateTable</i> operation is complete.</p>"]
                fn update_table(&self, input: &UpdateTableInput)  -> Result<UpdateTableOutput, UpdateTableError>;
                
}
/// A client for the DynamoDB API.
        pub struct DynamoDbClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> DynamoDbClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  DynamoDbClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }
        }

        impl<P, D> DynamoDb for DynamoDbClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
        

                #[doc="<p>The <i>BatchGetItem</i> operation returns the attributes of one or more items from one or more tables. You identify requested items by primary key.</p> <p>A single operation can retrieve up to 16 MB of data, which can contain as many as 100 items. <i>BatchGetItem</i> will return a partial result if the response size limit is exceeded, the table's provisioned throughput is exceeded, or an internal processing failure occurs. If a partial result is returned, the operation returns a value for <i>UnprocessedKeys</i>. You can use this value to retry the operation starting with the next item to get.</p> <important> <p>If you request more than 100 items <i>BatchGetItem</i> will return a <i>ValidationException</i> with the message \"Too many items requested for the BatchGetItem call\".</p> </important> <p>For example, if you ask to retrieve 100 items, but each individual item is 300 KB in size, the system returns 52 items (so as not to exceed the 16 MB limit). It also returns an appropriate <i>UnprocessedKeys</i> value so you can get the next page of results. If desired, your application can include its own logic to assemble the pages of results into one data set.</p> <p>If <i>none</i> of the items can be processed due to insufficient provisioned throughput on all of the tables in the request, then <i>BatchGetItem</i> will return a <i>ProvisionedThroughputExceededException</i>. If <i>at least one</i> of the items is successfully processed, then <i>BatchGetItem</i> completes successfully, while returning the keys of the unread items in <i>UnprocessedKeys</i>.</p> <important> <p>If DynamoDB returns any unprocessed items, you should retry the batch operation on those items. However, <i>we strongly recommend that you use an exponential backoff algorithm</i>. If you retry the batch operation immediately, the underlying read or write requests can still fail due to throttling on the individual tables. If you delay the batch operation using exponential backoff, the individual requests in the batch are much more likely to succeed.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOperations\">Batch Operations and Error Handling</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </important> <p>By default, <i>BatchGetItem</i> performs eventually consistent reads on every table in the request. If you want strongly consistent reads instead, you can set <i>ConsistentRead</i> to <code>true</code> for any or all tables.</p> <p>In order to minimize response latency, <i>BatchGetItem</i> retrieves items in parallel.</p> <p>When designing your application, keep in mind that DynamoDB does not return items in any particular order. To help parse the response by item, include the primary key values for the items in your request in the <i>AttributesToGet</i> parameter.</p> <p>If a requested item does not exist, it is not returned in the result. Requests for nonexistent items consume the minimum read capacity units according to the type of read. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#CapacityUnitCalculations\">Capacity Units Calculations</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
                fn batch_get_item(&self, input: &BatchGetItemInput)  -> Result<BatchGetItemOutput, BatchGetItemError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.BatchGetItem");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetItemOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(BatchGetItemError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>The <i>BatchWriteItem</i> operation puts or deletes multiple items in one or more tables. A single call to <i>BatchWriteItem</i> can write up to 16 MB of data, which can comprise as many as 25 put or delete requests. Individual items to be written can be as large as 400 KB.</p> <note> <p> <i>BatchWriteItem</i> cannot update items. To update items, use the <i>UpdateItem</i> API.</p> </note> <p>The individual <i>PutItem</i> and <i>DeleteItem</i> operations specified in <i>BatchWriteItem</i> are atomic; however <i>BatchWriteItem</i> as a whole is not. If any requested operations fail because the table's provisioned throughput is exceeded or an internal processing failure occurs, the failed operations are returned in the <i>UnprocessedItems</i> response parameter. You can investigate and optionally resend the requests. Typically, you would call <i>BatchWriteItem</i> in a loop. Each iteration would check for unprocessed items and submit a new <i>BatchWriteItem</i> request with those unprocessed items until all items have been processed.</p> <p>Note that if <i>none</i> of the items can be processed due to insufficient provisioned throughput on all of the tables in the request, then <i>BatchWriteItem</i> will return a <i>ProvisionedThroughputExceededException</i>.</p> <important> <p>If DynamoDB returns any unprocessed items, you should retry the batch operation on those items. However, <i>we strongly recommend that you use an exponential backoff algorithm</i>. If you retry the batch operation immediately, the underlying read or write requests can still fail due to throttling on the individual tables. If you delay the batch operation using exponential backoff, the individual requests in the batch are much more likely to succeed.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOperations\">Batch Operations and Error Handling</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> </important> <p>With <i>BatchWriteItem</i>, you can efficiently write or delete large amounts of data, such as from Amazon Elastic MapReduce (EMR), or copy data from another database into DynamoDB. In order to improve performance with these large-scale operations, <i>BatchWriteItem</i> does not behave in the same way as individual <i>PutItem</i> and <i>DeleteItem</i> calls would. For example, you cannot specify conditions on individual put and delete requests, and <i>BatchWriteItem</i> does not return deleted items in the response.</p> <p>If you use a programming language that supports concurrency, you can use threads to write items in parallel. Your application must include the necessary logic to manage the threads. With languages that don't support threading, you must update or delete the specified items one at a time. In both situations, <i>BatchWriteItem</i> provides an alternative where the API performs the specified put and delete operations in parallel, giving you the power of the thread pool approach without having to introduce complexity into your application.</p> <p>Parallel processing reduces latency, but each specified put and delete request consumes the same number of write capacity units whether it is processed in parallel or not. Delete operations on nonexistent items consume one write capacity unit.</p> <p>If one or more of the following is true, DynamoDB rejects the entire batch write operation:</p> <ul> <li> <p>One or more tables specified in the <i>BatchWriteItem</i> request does not exist.</p> </li> <li> <p>Primary key attributes specified on an item in the request do not match those in the corresponding table's primary key schema.</p> </li> <li> <p>You try to perform multiple operations on the same item in the same <i>BatchWriteItem</i> request. For example, you cannot put and delete the same item in the same <i>BatchWriteItem</i> request. </p> </li> <li> <p>There are more than 25 requests in the batch.</p> </li> <li> <p>Any individual item in a batch exceeds 400 KB.</p> </li> <li> <p>The total request size exceeds 16 MB.</p> </li> </ul>"]
                fn batch_write_item(&self, input: &BatchWriteItemInput)  -> Result<BatchWriteItemOutput, BatchWriteItemError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.BatchWriteItem");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchWriteItemOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(BatchWriteItemError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>The <i>CreateTable</i> operation adds a new table to your account. In an AWS account, table names must be unique within each region. That is, you can have two tables with same name if you create the tables in different regions.</p> <p> <i>CreateTable</i> is an asynchronous operation. Upon receiving a <i>CreateTable</i> request, DynamoDB immediately returns a response with a <i>TableStatus</i> of <code>CREATING</code>. After the table is created, DynamoDB sets the <i>TableStatus</i> to <code>ACTIVE</code>. You can perform read and write operations only on an <code>ACTIVE</code> table. </p> <p>You can optionally define secondary indexes on the new table, as part of the <i>CreateTable</i> operation. If you want to create multiple tables with secondary indexes on them, you must create the tables sequentially. Only one table with secondary indexes can be in the <code>CREATING</code> state at any given time.</p> <p>You can use the <i>DescribeTable</i> API to check the table status.</p>"]
                fn create_table(&self, input: &CreateTableInput)  -> Result<CreateTableOutput, CreateTableError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.CreateTable");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateTableOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateTableError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a single item in a table by primary key. You can perform a conditional delete operation that deletes the item if it exists, or if it has an expected attribute value.</p> <p>In addition to deleting an item, you can also return the item's attribute values in the same operation, using the <i>ReturnValues</i> parameter.</p> <p>Unless you specify conditions, the <i>DeleteItem</i> is an idempotent operation; running it multiple times on the same item or attribute does <i>not</i> result in an error response.</p> <p>Conditional deletes are useful for deleting items only if specific conditions are met. If those conditions are met, DynamoDB performs the delete. Otherwise, the item is not deleted.</p>"]
                fn delete_item(&self, input: &DeleteItemInput)  -> Result<DeleteItemOutput, DeleteItemError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.DeleteItem");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteItemOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteItemError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>The <i>DeleteTable</i> operation deletes a table and all of its items. After a <i>DeleteTable</i> request, the specified table is in the <code>DELETING</code> state until DynamoDB completes the deletion. If the table is in the <code>ACTIVE</code> state, you can delete it. If a table is in <code>CREATING</code> or <code>UPDATING</code> states, then DynamoDB returns a <i>ResourceInUseException</i>. If the specified table does not exist, DynamoDB returns a <i>ResourceNotFoundException</i>. If table is already in the <code>DELETING</code> state, no error is returned. </p> <note> <p>DynamoDB might continue to accept data read and write operations, such as <i>GetItem</i> and <i>PutItem</i>, on a table in the <code>DELETING</code> state until the table deletion is complete.</p> </note> <p>When you delete a table, any indexes on that table are also deleted.</p> <p>If you have DynamoDB Streams enabled on the table, then the corresponding stream on that table goes into the <code>DISABLED</code> state, and the stream is automatically deleted after 24 hours.</p> <p>Use the <i>DescribeTable</i> API to check the status of the table. </p>"]
                fn delete_table(&self, input: &DeleteTableInput)  -> Result<DeleteTableOutput, DeleteTableError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.DeleteTable");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteTableOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteTableError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns the current provisioned-capacity limits for your AWS account in a region, both for the region as a whole and for any one DynamoDB table that you create there.</p> <p>When you establish an AWS account, the account has initial limits on the maximum read capacity units and write capacity units that you can provision across all of your DynamoDB tables in a given region. Also, there are per-table limits that apply when you create a table there. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html\">Limits</a> page in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>Although you can increase these limits by filing a case at <a href=\"https://console.aws.amazon.com/support/home#/\">AWS Support Center</a>, obtaining the increase is not instantaneous. The <i>DescribeLimits</i> API lets you write code to compare the capacity you are currently using to those limits imposed by your account so that you have enough time to apply for an increase before you hit a limit.</p> <p>For example, you could use one of the AWS SDKs to do the following:</p> <ol> <li><p>Call <i>DescribeLimits</i> for a particular region to obtain your current account limits on provisioned capacity there.</p> </li> <li><p>Create a variable to hold the aggregate read capacity units provisioned for all your tables in that region, and one to hold the aggregate write capacity units. Zero them both.</p> </li> <li><p>Call <i>ListTables</i> to obtain a list of all your DynamoDB tables.</p> </li> <li> <p>For each table name listed by <i>ListTables</i>, do the following:</p> <ul> <li><p>Call <i>DescribeTable</i> with the table name.</p> </li> <li><p>Use the data returned by <i>DescribeTable</i> to add the read capacity units and write capacity units provisioned for the table itself to your variables.</p> </li> <li><p>If the table has one or more global secondary indexes (GSIs), loop over these GSIs and add their provisioned capacity values to your variables as well.</p> </li> </ul> </li> <li><p>Report the account limits for that region returned by <i>DescribeLimits</i>, along with the total current provisioned capacity levels you have calculated.</p> </li> </ol> <p>This will let you see whether you are getting close to your account-level limits.</p> <p>The per-table limits apply only when you are creating a new table. They restrict the sum of the provisioned capacity of the new table itself and all its global secondary indexes.</p> <p>For existing tables and their GSIs, DynamoDB will not let you increase provisioned capacity extremely rapidly, but the only upper limit that applies is that the aggregate provisioned capacity over all your tables and GSIs cannot exceed either of the per-account limits.</p> <note> <p> <i>DescribeLimits</i> should only be called periodically. You can expect throttling errors if you call it more than once in a minute.</p> </note> <p>The <i>DescribeLimits</i> Request element has no content.</p>"]
                fn describe_limits(&self, input: &DescribeLimitsInput)  -> Result<DescribeLimitsOutput, DescribeLimitsError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.DescribeLimits");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeLimitsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeLimitsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns information about the table, including the current status of the table, when it was created, the primary key schema, and any indexes on the table.</p> <note> <p>If you issue a <i>DescribeTable</i> request immediately after a <i>CreateTable</i> request, DynamoDB might return a <i>ResourceNotFoundException</i>. This is because <i>DescribeTable</i> uses an eventually consistent query, and the metadata for your table might not be available at that moment. Wait for a few seconds, and then try the <i>DescribeTable</i> request again.</p> </note>"]
                fn describe_table(&self, input: &DescribeTableInput)  -> Result<DescribeTableOutput, DescribeTableError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.DescribeTable");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeTableOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeTableError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>The <i>GetItem</i> operation returns a set of attributes for the item with the given primary key. If there is no matching item, <i>GetItem</i> does not return any data.</p> <p> <i>GetItem</i> provides an eventually consistent read by default. If your application requires a strongly consistent read, set <i>ConsistentRead</i> to <code>true</code>. Although a strongly consistent read might take more time than an eventually consistent read, it always returns the last updated value.</p>"]
                fn get_item(&self, input: &GetItemInput)  -> Result<GetItemOutput, GetItemError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.GetItem");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetItemOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetItemError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns an array of table names associated with the current account and endpoint. The output from <i>ListTables</i> is paginated, with each page returning a maximum of 100 table names.</p>"]
                fn list_tables(&self, input: &ListTablesInput)  -> Result<ListTablesOutput, ListTablesError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.ListTables");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTablesOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListTablesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a new item, or replaces an old item with a new item. If an item that has the same primary key as the new item already exists in the specified table, the new item completely replaces the existing item. You can perform a conditional put operation (add a new item if one with the specified primary key doesn't exist), or replace an existing item if it has certain attribute values.</p> <p>In addition to putting an item, you can also return the item's attribute values in the same operation, using the <i>ReturnValues</i> parameter.</p> <p>When you add an item, the primary key attribute(s) are the only required attributes. Attribute values cannot be null. String and Binary type attributes must have lengths greater than zero. Set type attributes cannot be empty. Requests with empty values will be rejected with a <i>ValidationException</i> exception.</p> <p>You can request that <i>PutItem</i> return either a copy of the original item (before the update) or a copy of the updated item (after the update). For more information, see the <i>ReturnValues</i> description below.</p> <note> <p>To prevent a new item from replacing an existing item, use a conditional expression that contains the <code>attribute_not_exists</code> function with the name of the attribute being used as the partition key for the table. Since every record must contain that attribute, the <code>attribute_not_exists</code> function will only succeed if no matching item exists.</p> </note> <p>For more information about using this API, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithItems.html\">Working with Items</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>"]
                fn put_item(&self, input: &PutItemInput)  -> Result<PutItemOutput, PutItemError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.PutItem");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutItemOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(PutItemError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>A <i>Query</i> operation uses the primary key of a table or a secondary index to directly access items from that table or index.</p> <p>Use the <i>KeyConditionExpression</i> parameter to provide a specific value for the partition key. The <i>Query</i> operation will return all of the items from the table or index with that partition key value. You can optionally narrow the scope of the <i>Query</i> operation by specifying a sort key value and a comparison operator in <i>KeyConditionExpression</i>. You can use the <i>ScanIndexForward</i> parameter to get results in forward or reverse order, by sort key.</p> <p>Queries that do not return results consume the minimum number of read capacity units for that type of read operation.</p> <p>If the total number of items meeting the query criteria exceeds the result set size limit of 1 MB, the query stops and results are returned to the user with the <i>LastEvaluatedKey</i> element to continue the query in a subsequent operation. Unlike a <i>Scan</i> operation, a <i>Query</i> operation never returns both an empty result set and a <i>LastEvaluatedKey</i> value. <i>LastEvaluatedKey</i> is only provided if you have used the <i>Limit</i> parameter, or if the result set exceeds 1 MB (prior to applying a filter). </p> <p>You can query a table, a local secondary index, or a global secondary index. For a query on a table or on a local secondary index, you can set the <i>ConsistentRead</i> parameter to <code>true</code> and obtain a strongly consistent result. Global secondary indexes support eventually consistent reads only, so do not specify <i>ConsistentRead</i> when querying a global secondary index.</p>"]
                fn query(&self, input: &QueryInput)  -> Result<QueryOutput, QueryError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.Query");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<QueryOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(QueryError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>The <i>Scan</i> operation returns one or more items and item attributes by accessing every item in a table or a secondary index. To have DynamoDB return fewer items, you can provide a <i>ScanFilter</i> operation.</p> <p>If the total number of scanned items exceeds the maximum data set size limit of 1 MB, the scan stops and results are returned to the user as a <i>LastEvaluatedKey</i> value to continue the scan in a subsequent operation. The results also include the number of items exceeding the limit. A scan can result in no table data meeting the filter criteria. </p> <p>By default, <i>Scan</i> operations proceed sequentially; however, for faster performance on a large table or secondary index, applications can request a parallel <i>Scan</i> operation by providing the <i>Segment</i> and <i>TotalSegments</i> parameters. For more information, see <a href=\"http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#QueryAndScanParallelScan\">Parallel Scan</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p> <p>By default, <i>Scan</i> uses eventually consistent reads when accessing the data in a table; therefore, the result set might not include the changes to data in the table immediately before the operation began. If you need a consistent copy of the data, as of the time that the Scan begins, you can set the <i>ConsistentRead</i> parameter to <i>true</i>.</p>"]
                fn scan(&self, input: &ScanInput)  -> Result<ScanOutput, ScanError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.Scan");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ScanOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ScanError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Edits an existing item's attributes, or adds a new item to the table if it does not already exist. You can put, delete, or add attribute values. You can also perform a conditional update on an existing item (insert a new attribute name-value pair if it doesn't exist, or replace an existing name-value pair if it has certain expected attribute values).</p> <p>You can also return the item's attribute values in the same <i>UpdateItem</i> operation using the <i>ReturnValues</i> parameter.</p>"]
                fn update_item(&self, input: &UpdateItemInput)  -> Result<UpdateItemOutput, UpdateItemError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.UpdateItem");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateItemOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateItemError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Modifies the provisioned throughput settings, global secondary indexes, or DynamoDB Streams settings for a given table.</p> <p>You can only perform one of the following operations at once:</p> <ul> <li> <p>Modify the provisioned throughput settings of the table.</p> </li> <li> <p>Enable or disable Streams on the table.</p> </li> <li> <p>Remove a global secondary index from the table.</p> </li> <li> <p>Create a new global secondary index on the table. Once the index begins backfilling, you can use <i>UpdateTable</i> to perform other operations.</p> </li> </ul> <p> <i>UpdateTable</i> is an asynchronous operation; while it is executing, the table status changes from <code>ACTIVE</code> to <code>UPDATING</code>. While it is <code>UPDATING</code>, you cannot issue another <i>UpdateTable</i> request. When the table returns to the <code>ACTIVE</code> state, the <i>UpdateTable</i> operation is complete.</p>"]
                fn update_table(&self, input: &UpdateTableInput)  -> Result<UpdateTableOutput, UpdateTableError> {
                    let mut request = SignedRequest::new("POST", "dynamodb", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.0".to_owned());
                    request.add_header("x-amz-target", "DynamoDB_20120810.UpdateTable");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateTableOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateTableError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                
}

            #[cfg(test)]
            mod protocol_tests {
                
            }
            
