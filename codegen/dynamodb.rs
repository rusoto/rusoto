use std::collections::HashMap;
use std::error::Error;
use rustc_serialize::json;
use std::io::Read;
/// Represents the output of a _PutItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutItemOutput {
	/// The attribute values as they appeared before the _PutItem_ operation, but only
	/// if _ReturnValues_ is specified as `ALL_OLD` in the request. Each element
	/// consists of an attribute name and an attribute value.
	pub Attributes: Option<AttributeMap>,
	/// Information about item collections, if any, that were affected by the
	/// operation. _ItemCollectionMetrics_ is only returned if the request asked for
	/// it. If the table does not have any local secondary indexes, this information
	/// is not returned in the response.
	/// Each _ItemCollectionMetrics_ element consists of:
	///   * _ItemCollectionKey_ \- The hash key value of the item collection. This is the same as the hash key of the item.
	///   * _SizeEstimateRange_ \- An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.
	/// The estimate is subject to change over time; therefore, do not rely on the
	/// precision or accuracy of the estimate.
	pub ItemCollectionMetrics: Option<ItemCollectionMetrics>,
	pub ConsumedCapacity: Option<ConsumedCapacity>,
}

/// Represents the input of a _PutItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutItemInput {
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ConditionExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A logical operator to apply to the conditions in the _Expected_ map:
	///   * `AND` \- If all of the conditions evaluate to true, then the entire map evaluates to true.
	///   * `OR` \- If at least one of the conditions evaluate to true, then the entire map evaluates to true.
	/// If you omit _ConditionalOperator_, then `AND` is the default.
	/// The operation will succeed only if the entire map evaluates to true.
	/// This parameter does not support attributes of type List or Map.
	pub ConditionalOperator: Option<ConditionalOperator>,
	/// One or more substitution tokens for attribute names in an expression. The
	/// following are some use cases for using _ExpressionAttributeNames_:
	///   * To access an attribute whose name conflicts with a DynamoDB reserved word.
	///   * To create a placeholder for repeating occurrences of an attribute name in an expression.
	///   * To prevent special characters in an attribute name from being misinterpreted in an expression.
	/// Use the **#** character in an expression to dereference an attribute name. For
	/// example, consider the following attribute name:
	///   * `Percentile`
	/// The name of this attribute conflicts with a reserved word, so it cannot be
	/// used directly in an expression. (For the complete list of reserved words, see
	/// [Reserved Words](http://docs.aws.amazon.com/amazondynamodb/latest/developergui
	/// de/ReservedWords.html) in the _Amazon DynamoDB Developer Guide_). To work
	/// around this, you could specify the following for _ExpressionAttributeNames_:
	///   * `{"#P":"Percentile"}`
	/// You could then use this substitution in an expression, as in this example:
	///   * `#P = :val`
	/// Tokens that begin with the **:** character are _expression attribute values_,
	/// which are placeholders for the actual value at runtime.
	/// For more information on expression attribute names, see [Accessing Item Attrib
	/// utes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressi
	/// ons.AccessingItemAttributes.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeNames: Option<ExpressionAttributeNameMap>,
	/// Use _ReturnValues_ if you want to get the item attributes as they appeared
	/// before they were updated with the _PutItem_ request. For _PutItem_, the valid
	/// values are:
	///   * `NONE` \- If _ReturnValues_ is not specified, or if its value is `NONE`, then nothing is returned. (This setting is the default for _ReturnValues_.)
	///   * `ALL_OLD` \- If _PutItem_ overwrote an attribute name-value pair, then the content of the old item is returned.
	/// Other "Valid Values" are not relevant to PutItem.
	pub ReturnValues: Option<ReturnValue>,
	/// A condition that must be satisfied in order for a conditional _PutItem_
	/// operation to succeed.
	/// An expression can contain any of the following:
	///   * Functions: `attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size`
	/// These function names are case-sensitive.
	///   * Comparison operators: ` = | <> | < | > | <= | >= | BETWEEN | IN`
	///   * Logical operators: `AND | OR | NOT`
	/// For more information on condition expressions, see [Specifying Conditions](htt
	/// p://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Speci
	/// fyingConditions.html) in the _Amazon DynamoDB Developer Guide_.
	/// _ConditionExpression_ replaces the legacy _ConditionalOperator_ and _Expected_
	/// parameters.
	pub ConditionExpression: Option<ConditionExpression>,
	/// The name of the table to contain the item.
	pub TableName: TableName,
	/// Determines whether item collection metrics are returned. If set to `SIZE`, the
	/// response includes statistics about item collections, if any, that were
	/// modified during the operation are returned in the response. If set to `NONE`
	/// (the default), no statistics are returned.
	pub ReturnItemCollectionMetrics: Option<ReturnItemCollectionMetrics>,
	pub ReturnConsumedCapacity: Option<ReturnConsumedCapacity>,
	/// A map of attribute name/value pairs, one for each attribute. Only the primary
	/// key attributes are required; you can optionally provide other attribute name-
	/// value pairs for the item.
	/// You must provide all of the attributes for the primary key. For example, with
	/// a hash type primary key, you only need to provide the hash attribute. For a
	/// hash-and-range type primary key, you must provide both the hash attribute and
	/// the range attribute.
	/// If you specify any attributes that are part of an index key, then the data
	/// types for those attributes must match those of the schema in the table's
	/// attribute definition.
	/// For more information about primary keys, see [Primary Key](http://docs.aws.ama
	/// zon.com/amazondynamodb/latest/developerguide/DataModel.html#DataModelPrimaryKe
	/// y) in the _Amazon DynamoDB Developer Guide_.
	/// Each element in the _Item_ map is an _AttributeValue_ object.
	pub Item: PutItemInputAttributeMap,
	/// One or more values that can be substituted in an expression.
	/// Use the **:** (colon) character in an expression to dereference an attribute
	/// value. For example, suppose that you wanted to check whether the value of the
	/// _ProductStatus_ attribute was one of the following:
	/// `Available | Backordered | Discontinued`
	/// You would first need to specify _ExpressionAttributeValues_ as follows:
	/// `{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"},
	/// ":disc":{"S":"Discontinued"} }`
	/// You could then use these values in an expression, such as this:
	/// `ProductStatus IN (:avail, :back, :disc)`
	/// For more information on expression attribute values, see [Specifying Condition
	/// s](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions
	/// .SpecifyingConditions.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeValues: Option<ExpressionAttributeValueMap>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ConditionExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A map of attribute/condition pairs. _Expected_ provides a conditional block
	/// for the _PutItem_ operation.
	/// This parameter does not support attributes of type List or Map.
	/// Each element of _Expected_ consists of an attribute name, a comparison
	/// operator, and one or more values. DynamoDB compares the attribute with the
	/// value(s) you supplied, using the comparison operator. For each _Expected_
	/// element, the result of the evaluation is either true or false.
	/// If you specify more than one element in the _Expected_ map, then by default
	/// all of the conditions must evaluate to true. In other words, the conditions
	/// are ANDed together. (You can use the _ConditionalOperator_ parameter to OR the
	/// conditions instead. If you do this, then at least one of the conditions must
	/// evaluate to true, rather than all of them.)
	/// If the _Expected_ map evaluates to true, then the conditional operation
	/// succeeds; otherwise, it fails.
	/// _Expected_ contains the following:
	///   * _AttributeValueList_ \- One or more values to evaluate against the supplied attribute. The number of values in the list depends on the _ComparisonOperator_ being used.
	/// For type Number, value comparisons are numeric.
	/// String value comparisons for greater than, equals, or less than are based on
	/// ASCII character code values. For example, `a` is greater than `A`, and `a` is
	/// greater than `B`. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	/// For type Binary, DynamoDB treats each byte of the binary data as unsigned when
	/// it compares binary values.
	///   * _ComparisonOperator_ \- A comparator for evaluating attributes in the _AttributeValueList_. When performing the comparison, DynamoDB uses strongly consistent reads.
	/// The following comparison operators are available:
	/// `EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS |
	/// BEGINS_WITH | IN | BETWEEN`
	/// The following are descriptions of each comparison operator.
	///     * `EQ` : Equal. `EQ` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, Binary, String Set, Number Set, or Binary Set. If an item
	/// contains an _AttributeValue_ element of a different type than the one provided
	/// in the request, the value does not match. For example, `{"S":"6"}` does not
	/// equal `{"N":"6"}`. Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///     * `NE` : Not equal. `NE` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, Binary, String Set, Number Set, or Binary Set. If an item contains an
	/// _AttributeValue_ of a different type than the one provided in the request, the
	/// value does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`.
	/// Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///     * `LE` : Less than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `LT` : Less than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, or Binary (not a set type). If an item contains an _AttributeValue_
	/// element of a different type than the one provided in the request, the value
	/// does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`. Also,
	/// `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `GE` : Greater than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `GT` : Greater than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `NOT_NULL` : The attribute exists. `NOT_NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the existence of an attribute, not its data type. If
	/// the data type of attribute "`a`" is null, and you evaluate it using
	/// `NOT_NULL`, the result is a Boolean _true_. This result is because the
	/// attribute "`a`" exists; its data type is not relevant to the `NOT_NULL`
	/// comparison operator.
	///     * `NULL` : The attribute does not exist. `NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the nonexistence of an attribute, not its data type.
	/// If the data type of attribute "`a`" is null, and you evaluate it using `NULL`,
	/// the result is a Boolean _false_. This is because the attribute "`a`" exists;
	/// its data type is not relevant to the `NULL` comparison operator.
	///     * `CONTAINS` : Checks for a subsequence, or value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is of type String, then the operator checks for a substring match.
	/// If the target attribute of the comparison is of type Binary, then the operator
	/// looks for a subsequence of the target that matches the input. If the target
	/// attribute of the comparison is a set ("`SS`", "`NS`", or "`BS`"), then the
	/// operator evaluates to true if it finds an exact match with any member of the
	/// set.
	/// CONTAINS is supported for lists: When evaluating "`a CONTAINS b`", "`a`" can
	/// be a list; however, "`b`" cannot be a set, a map, or a list.
	///     * `NOT_CONTAINS` : Checks for absence of a subsequence, or absence of a value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is a String, then the operator checks for the absence of a
	/// substring match. If the target attribute of the comparison is Binary, then the
	/// operator checks for the absence of a subsequence of the target that matches
	/// the input. If the target attribute of the comparison is a set ("`SS`", "`NS`",
	/// or "`BS`"), then the operator evaluates to true if it _does not_ find an exact
	/// match with any member of the set.
	/// NOT_CONTAINS is supported for lists: When evaluating "`a NOT CONTAINS b`",
	/// "`a`" can be a list; however, "`b`" cannot be a set, a map, or a list.
	///     * `BEGINS_WITH` : Checks for a prefix. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String or
	/// Binary (not a Number or a set type). The target attribute of the comparison
	/// must be of type String or Binary (not a Number or a set type).
	///     * `IN` : Checks for matching elements within two sets.
	/// _AttributeValueList_ can contain one or more _AttributeValue_ elements of type
	/// String, Number, or Binary (not a set type). These attributes are compared
	/// against an existing set type attribute of an item. If any elements of the
	/// input set are present in the item attribute, the expression evaluates to true.
	///     * `BETWEEN` : Greater than or equal to the first value, and less than or equal to the second value. 
	/// _AttributeValueList_ must contain two _AttributeValue_ elements of the same
	/// type, either String, Number, or Binary (not a set type). A target attribute
	/// matches if the target value is greater than, or equal to, the first element
	/// and less than, or equal to, the second element. If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not compare
	/// to `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`
	/// For usage examples of _AttributeValueList_ and _ComparisonOperator_, see
	/// [Legacy Conditional Parameters](http://docs.aws.amazon.com/amazondynamodb/late
	/// st/developerguide/LegacyConditionalParameters.html) in the _Amazon DynamoDB
	/// Developer Guide_.
	/// For backward compatibility with previous DynamoDB releases, the following
	/// parameters can be used instead of _AttributeValueList_ and
	/// _ComparisonOperator_:
	///   * _Value_ \- A value for DynamoDB to compare with an attribute.
	///   * _Exists_ \- A Boolean value that causes DynamoDB to evaluate the value before attempting the conditional operation:
	///     * If _Exists_ is `true`, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the condition evaluates to true; otherwise the condition evaluate to false.
	///     * If _Exists_ is `false`, DynamoDB assumes that the attribute value does _not_ exist in the table. If in fact the value does not exist, then the assumption is valid and the condition evaluates to true. If the value is found, despite the assumption that it does not exist, the condition evaluates to false.
	/// Note that the default value for _Exists_ is `true`.
	/// The _Value_ and _Exists_ parameters are incompatible with _AttributeValueList_
	/// and _ComparisonOperator_. Note that if you use both sets of parameters at
	/// once, DynamoDB will return a _ValidationException_ exception.
	pub Expected: Option<ExpectedAttributeMap>,
}

pub type GlobalSecondaryIndexDescriptionList = Vec<GlobalSecondaryIndexDescription>;
pub type KeyExpression = String;
/// Represents _a single element_ of a key schema. A key schema specifies the
/// attributes that make up the primary key of a table, or the key attributes of
/// an index.
/// A _KeySchemaElement_ represents exactly one attribute of the primary key. For
/// example, a hash type primary key would be represented by one
/// _KeySchemaElement_. A hash-and-range type primary key would require one
/// _KeySchemaElement_ for the hash attribute, and another _KeySchemaElement_ for
/// the range attribute.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct KeySchemaElement {
	/// The attribute data, consisting of the data type and the attribute value
	/// itself.
	pub KeyType: KeyType,
	/// The name of a key attribute.
	pub AttributeName: KeySchemaAttributeName,
}

/// Represents the output of a _BatchWriteItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct BatchWriteItemOutput {
	/// A map of tables and requests against those tables that were not processed. The
	/// _UnprocessedItems_ value is in the same form as _RequestItems_, so you can
	/// provide this value directly to a subsequent _BatchGetItem_ operation. For more
	/// information, see _RequestItems_ in the Request Parameters section.
	/// Each _UnprocessedItems_ entry consists of a table name and, for that table, a
	/// list of operations to perform (_DeleteRequest_ or _PutRequest_).
	///   * _DeleteRequest_ \- Perform a _DeleteItem_ operation on the specified item. The item to be deleted is identified by a _Key_ subelement:
	///     * _Key_ \- A map of primary key attribute values that uniquely identify the item. Each entry in this map consists of an attribute name and an attribute value.
	///   * _PutRequest_ \- Perform a _PutItem_ operation on the specified item. The item to be put is identified by an _Item_ subelement:
	///     * _Item_ \- A map of attributes and their values. Each entry in this map consists of an attribute name and an attribute value. Attribute values must not be null; string and binary type attributes must have lengths greater than zero; and set type attributes must not be empty. Requests that contain empty values will be rejected with a _ValidationException_ exception.
	/// If you specify any attributes that are part of an index key, then the data
	/// types for those attributes must match those of the schema in the table's
	/// attribute definition.
	/// If there are no unprocessed items remaining, the response contains an empty
	/// _UnprocessedItems_ map.
	pub UnprocessedItems: Option<BatchWriteItemRequestMap>,
	/// A list of tables that were processed by _BatchWriteItem_ and, for each table,
	/// information about any item collections that were affected by individual
	/// _DeleteItem_ or _PutItem_ operations.
	/// Each entry consists of the following subelements:
	///   * _ItemCollectionKey_ \- The hash key value of the item collection. This is the same as the hash key of the item.
	///   * _SizeEstimateRange_ \- An estimate of item collection size, expressed in GB. This is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on the table. Use this estimate to measure whether a local secondary index is approaching its size limit.
	/// The estimate is subject to change over time; therefore, do not rely on the
	/// precision or accuracy of the estimate.
	pub ItemCollectionMetrics: Option<ItemCollectionMetricsPerTable>,
	/// The capacity units consumed by the operation.
	/// Each element consists of:
	///   * _TableName_ \- The table that consumed the provisioned throughput.
	///   * _CapacityUnits_ \- The total number of capacity units consumed.
	pub ConsumedCapacity: Option<ConsumedCapacityMultiple>,
}

/// Represents the new provisioned throughput settings to be applied to a global
/// secondary index.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct UpdateGlobalSecondaryIndexAction {
	pub ProvisionedThroughput: ProvisionedThroughput,
	/// The name of the global secondary index to be updated.
	pub IndexName: IndexName,
}

/// Represents the input of a _BatchGetItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct BatchGetItemInput {
	/// A map of one or more table names and, for each table, a map that describes one
	/// or more items to retrieve from that table. Each table name can be used only
	/// once per _BatchGetItem_ request.
	/// Each element in the map of items to retrieve consists of the following:
	///   * _ConsistentRead_ \- If `true`, a strongly consistent read is used; if `false` (the default), an eventually consistent read is used.
	///   * _ExpressionAttributeNames_ \- One or more substitution tokens for attribute names in the _ProjectionExpression_ parameter. The following are some use cases for using _ExpressionAttributeNames_:
	///     * To access an attribute whose name conflicts with a DynamoDB reserved word.
	///     * To create a placeholder for repeating occurrences of an attribute name in an expression.
	///     * To prevent special characters in an attribute name from being misinterpreted in an expression.
	/// Use the **#** character in an expression to dereference an attribute name. For
	/// example, consider the following attribute name:
	///     * `Percentile`
	/// The name of this attribute conflicts with a reserved word, so it cannot be
	/// used directly in an expression. (For the complete list of reserved words, see
	/// [Reserved Words](http://docs.aws.amazon.com/amazondynamodb/latest/developergui
	/// de/ReservedWords.html) in the _Amazon DynamoDB Developer Guide_). To work
	/// around this, you could specify the following for _ExpressionAttributeNames_:
	///     * `{"#P":"Percentile"}`
	/// You could then use this substitution in an expression, as in this example:
	///     * `#P = :val`
	/// Tokens that begin with the **:** character are _expression attribute values_,
	/// which are placeholders for the actual value at runtime.
	/// For more information on expression attribute names, see [Accessing Item Attrib
	/// utes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressi
	/// ons.AccessingItemAttributes.html) in the _Amazon DynamoDB Developer Guide_.
	///   * _Keys_ \- An array of primary key attribute values that define specific items in the table. For each primary key, you must provide _all_ of the key attributes. For example, with a hash type primary key, you only need to provide the hash attribute. For a hash-and-range type primary key, you must provide _both_ the hash attribute and the range attribute.
	///   * _ProjectionExpression_ \- A string that identifies one or more attributes to retrieve from the table. These attributes can include scalars, sets, or elements of a JSON document. The attributes in the expression must be separated by commas.
	/// If no attribute names are specified, then all attributes will be returned. If
	/// any of the requested attributes are not found, they will not appear in the
	/// result.
	/// For more information, see [Accessing Item Attributes](http://docs.aws.amazon.c
	/// om/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.ht
	/// ml) in the _Amazon DynamoDB Developer Guide_.
	///   * _AttributesToGet_ \- 
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ProjectionExpression_ instead. Do not combine legacy parameters
	/// and expression parameters in a single API call; otherwise, DynamoDB will
	/// return a _ValidationException_ exception.
	/// This parameter allows you to retrieve attributes of type List or Map; however,
	/// it cannot retrieve individual elements within a List or a Map.
	/// The names of one or more attributes to retrieve. If no attribute names are
	/// provided, then all attributes will be returned. If any of the requested
	/// attributes are not found, they will not appear in the result.
	/// Note that _AttributesToGet_ has no effect on provisioned throughput
	/// consumption. DynamoDB determines capacity units consumed based on item size,
	/// not on the amount of data that is returned to an application.
	pub RequestItems: BatchGetRequestMap,
	pub ReturnConsumedCapacity: Option<ReturnConsumedCapacity>,
}

/// Represents the input of a _GetItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetItemInput {
	/// Determines the read consistency model: If set to `true`, then the operation
	/// uses strongly consistent reads; otherwise, the operation uses eventually
	/// consistent reads.
	pub ConsistentRead: Option<ConsistentRead>,
	/// A string that identifies one or more attributes to retrieve from the table.
	/// These attributes can include scalars, sets, or elements of a JSON document.
	/// The attributes in the expression must be separated by commas.
	/// If no attribute names are specified, then all attributes will be returned. If
	/// any of the requested attributes are not found, they will not appear in the
	/// result.
	/// For more information, see [Accessing Item Attributes](http://docs.aws.amazon.c
	/// om/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.ht
	/// ml) in the _Amazon DynamoDB Developer Guide_.
	/// _ProjectionExpression_ replaces the legacy _AttributesToGet_ parameter.
	pub ProjectionExpression: Option<ProjectionExpression>,
	/// One or more substitution tokens for attribute names in an expression. The
	/// following are some use cases for using _ExpressionAttributeNames_:
	///   * To access an attribute whose name conflicts with a DynamoDB reserved word.
	///   * To create a placeholder for repeating occurrences of an attribute name in an expression.
	///   * To prevent special characters in an attribute name from being misinterpreted in an expression.
	/// Use the **#** character in an expression to dereference an attribute name. For
	/// example, consider the following attribute name:
	///   * `Percentile`
	/// The name of this attribute conflicts with a reserved word, so it cannot be
	/// used directly in an expression. (For the complete list of reserved words, see
	/// [Reserved Words](http://docs.aws.amazon.com/amazondynamodb/latest/developergui
	/// de/ReservedWords.html) in the _Amazon DynamoDB Developer Guide_). To work
	/// around this, you could specify the following for _ExpressionAttributeNames_:
	///   * `{"#P":"Percentile"}`
	/// You could then use this substitution in an expression, as in this example:
	///   * `#P = :val`
	/// Tokens that begin with the **:** character are _expression attribute values_,
	/// which are placeholders for the actual value at runtime.
	/// For more information on expression attribute names, see [Accessing Item Attrib
	/// utes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressi
	/// ons.AccessingItemAttributes.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeNames: Option<ExpressionAttributeNameMap>,
	/// The name of the table containing the requested item.
	pub TableName: TableName,
	pub ReturnConsumedCapacity: Option<ReturnConsumedCapacity>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ProjectionExpression_ instead. Do not combine legacy parameters
	/// and expression parameters in a single API call; otherwise, DynamoDB will
	/// return a _ValidationException_ exception.
	/// This parameter allows you to retrieve attributes of type List or Map; however,
	/// it cannot retrieve individual elements within a List or a Map.
	/// The names of one or more attributes to retrieve. If no attribute names are
	/// provided, then all attributes will be returned. If any of the requested
	/// attributes are not found, they will not appear in the result.
	/// Note that _AttributesToGet_ has no effect on provisioned throughput
	/// consumption. DynamoDB determines capacity units consumed based on item size,
	/// not on the amount of data that is returned to an application.
	pub AttributesToGet: Option<AttributeNameList>,
	/// A map of attribute names to _AttributeValue_ objects, representing the primary
	/// key of the item to retrieve.
	/// For the primary key, you must provide all of the attributes. For example, with
	/// a hash type primary key, you only need to provide the hash attribute. For a
	/// hash-and-range type primary key, you must provide both the hash attribute and
	/// the range attribute.
	pub Key: Key,
}

pub type ItemCollectionKeyAttributeMap = HashMap<AttributeName,AttributeValue>;
/// Your request rate is too high. The AWS SDKs for DynamoDB automatically retry
/// requests that receive this exception. Your request is eventually successful,
/// unless your retry queue is too large to finish. Reduce the frequency of
/// requests and use exponential backoff. For more information, go to [Error
/// Retries and Exponential Backoff](http://docs.aws.amazon.com/amazondynamodb/lat
/// est/developerguide/ErrorHandling.html#APIRetries) in the _Amazon DynamoDB
/// Developer Guide_.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ProvisionedThroughputExceededException {
	/// You exceeded your maximum allowed provisioned throughput.
	pub message: Option<ErrorMessage>,
}

/// Represents the output of a _ListTables_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListTablesOutput {
	/// The name of the last table in the current page of results. Use this value as
	/// the _ExclusiveStartTableName_ in a new request to obtain the next page of
	/// results, until all the table names are returned.
	/// If you do not receive a _LastEvaluatedTableName_ value in the response, this
	/// means that there are no more table names to be retrieved.
	pub LastEvaluatedTableName: Option<TableName>,
	/// The names of the tables associated with the current account at the current
	/// endpoint. The maximum size of this array is 100.
	/// If _LastEvaluatedTableName_ also appears in the output, you can use this value
	/// as the _ExclusiveStartTableName_ parameter in a subsequent _ListTables_
	/// request and obtain the next page of results.
	pub TableNames: Option<TableNameList>,
}

pub type ReturnValue = String;
/// Represents the output of a _GetItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetItemOutput {
	/// A map of attribute names to _AttributeValue_ objects, as specified by
	/// _AttributesToGet_.
	pub Item: Option<AttributeMap>,
	pub ConsumedCapacity: Option<ConsumedCapacity>,
}

pub type KeyType = String;
/// Represents a set of primary keys and, for each key, the attributes to retrieve
/// from the table.
/// For each primary key, you must provide _all_ of the key attributes. For
/// example, with a hash type primary key, you only need to provide the hash
/// attribute. For a hash-and-range type primary key, you must provide _both_ the
/// hash attribute and the range attribute.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct KeysAndAttributes {
	/// The primary key attribute values that define the items and the attributes
	/// associated with the items.
	pub Keys: KeyList,
	/// The consistency of a read operation. If set to `true`, then a strongly
	/// consistent read is used; otherwise, an eventually consistent read is used.
	pub ConsistentRead: Option<ConsistentRead>,
	/// One or more attributes to retrieve from the table or index. If no attribute
	/// names are specified then all attributes will be returned. If any of the
	/// specified attributes are not found, they will not appear in the result.
	pub AttributesToGet: Option<AttributeNameList>,
	/// A string that identifies one or more attributes to retrieve from the table.
	/// These attributes can include scalars, sets, or elements of a JSON document.
	/// The attributes in the _ProjectionExpression_ must be separated by commas.
	/// If no attribute names are specified, then all attributes will be returned. If
	/// any of the requested attributes are not found, they will not appear in the
	/// result.
	/// For more information, see [Accessing Item Attributes](http://docs.aws.amazon.c
	/// om/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.ht
	/// ml) in the _Amazon DynamoDB Developer Guide_.
	/// _ProjectionExpression_ replaces the legacy _AttributesToGet_ parameter.
	pub ProjectionExpression: Option<ProjectionExpression>,
	/// One or more substitution tokens for attribute names in an expression. The
	/// following are some use cases for using _ExpressionAttributeNames_:
	///   * To access an attribute whose name conflicts with a DynamoDB reserved word.
	///   * To create a placeholder for repeating occurrences of an attribute name in an expression.
	///   * To prevent special characters in an attribute name from being misinterpreted in an expression.
	/// Use the **#** character in an expression to dereference an attribute name. For
	/// example, consider the following attribute name:
	///   * `Percentile`
	/// The name of this attribute conflicts with a reserved word, so it cannot be
	/// used directly in an expression. (For the complete list of reserved words, see
	/// [Reserved Words](http://docs.aws.amazon.com/amazondynamodb/latest/developergui
	/// de/ReservedWords.html) in the _Amazon DynamoDB Developer Guide_). To work
	/// around this, you could specify the following for _ExpressionAttributeNames_:
	///   * `{"#P":"Percentile"}`
	/// You could then use this substitution in an expression, as in this example:
	///   * `#P = :val`
	/// Tokens that begin with the **:** character are _expression attribute values_,
	/// which are placeholders for the actual value at runtime.
	/// For more information on expression attribute names, see [Accessing Item Attrib
	/// utes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressi
	/// ons.AccessingItemAttributes.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeNames: Option<ExpressionAttributeNameMap>,
}

pub type AttributeMap = HashMap<AttributeName,AttributeValue>;
pub type ScalarAttributeType = String;
/// An error occurred on the server side.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct InternalServerError {
	/// The server encountered an internal error trying to fulfill the request.
	pub message: Option<ErrorMessage>,
}

/// Represents the input of a _CreateTable_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateTableInput {
	/// One or more global secondary indexes (the maximum is five) to be created on
	/// the table. Each global secondary index in the array includes the following:
	///   * _IndexName_ \- The name of the global secondary index. Must be unique only for this table.
	///   * _KeySchema_ \- Specifies the key schema for the global secondary index.
	///   * _Projection_ \- Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:
	///     * _ProjectionType_ \- One of the following:
	///       * `KEYS_ONLY` \- Only the index and primary keys are projected into the index.
	///       * `INCLUDE` \- Only the specified table attributes are projected into the index. The list of projected attributes are in _NonKeyAttributes_.
	///       * `ALL` \- All of the table attributes are projected into the index.
	///     * _NonKeyAttributes_ \- A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in _NonKeyAttributes_, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.
	///   * _ProvisionedThroughput_ \- The provisioned throughput settings for the global secondary index, consisting of read and write capacity units.
	pub GlobalSecondaryIndexes: Option<GlobalSecondaryIndexList>,
	/// An array of attributes that describe the key schema for the table and indexes.
	pub AttributeDefinitions: AttributeDefinitions,
	/// One or more local secondary indexes (the maximum is five) to be created on the
	/// table. Each index is scoped to a given hash key value. There is a 10 GB size
	/// limit per hash key; otherwise, the size of a local secondary index is
	/// unconstrained.
	/// Each local secondary index in the array includes the following:
	///   * _IndexName_ \- The name of the local secondary index. Must be unique only for this table.
	///   * _KeySchema_ \- Specifies the key schema for the local secondary index. The key schema must begin with the same hash key attribute as the table.
	///   * _Projection_ \- Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:
	///     * _ProjectionType_ \- One of the following:
	///       * `KEYS_ONLY` \- Only the index and primary keys are projected into the index.
	///       * `INCLUDE` \- Only the specified table attributes are projected into the index. The list of projected attributes are in _NonKeyAttributes_.
	///       * `ALL` \- All of the table attributes are projected into the index.
	///     * _NonKeyAttributes_ \- A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in _NonKeyAttributes_, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.
	pub LocalSecondaryIndexes: Option<LocalSecondaryIndexList>,
	pub ProvisionedThroughput: ProvisionedThroughput,
	/// The name of the table to create.
	pub TableName: TableName,
	/// The settings for DynamoDB Streams on the table. These settings consist of:
	///   * _StreamEnabled_ \- Indicates whether Streams is to be enabled (true) or disabled (false).
	///   * _StreamViewType_ \- When an item in the table is modified, _StreamViewType_ determines what information is written to the table's stream. Valid values for _StreamViewType_ are:
	///     * _KEYS_ONLY_ \- Only the key attributes of the modified item are written to the stream.
	///     * _NEW_IMAGE_ \- The entire item, as it appears after it was modified, is written to the stream.
	///     * _OLD_IMAGE_ \- The entire item, as it appeared before it was modified, is written to the stream.
	///     * _NEW_AND_OLD_IMAGES_ \- Both the new and the old item images of the item are written to the stream.
	pub StreamSpecification: Option<StreamSpecification>,
	/// Specifies the attributes that make up the primary key for a table or an index.
	/// The attributes in _KeySchema_ must also be defined in the
	/// _AttributeDefinitions_ array. For more information, see [Data Model](http://do
	/// cs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html) in the
	/// _Amazon DynamoDB Developer Guide_.
	/// Each _KeySchemaElement_ in the array is composed of:
	///   * _AttributeName_ \- The name of this key attribute.
	///   * _KeyType_ \- Determines whether the key attribute is `HASH` or `RANGE`.
	/// For a primary key that consists of a hash attribute, you must provide exactly
	/// one element with a _KeyType_ of `HASH`.
	/// For a primary key that consists of hash and range attributes, you must provide
	/// exactly two elements, in this order: The first element must have a _KeyType_
	/// of `HASH`, and the second element must have a _KeyType_ of `RANGE`.
	/// For more information, see [Specifying the Primary Key](http://docs.aws.amazon.
	/// com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTab
	/// les.primary.key) in the _Amazon DynamoDB Developer Guide_.
	pub KeySchema: KeySchema,
}

pub type PositiveLongObject = i64;
/// Represents the data for an attribute. You can set one, and only one, of the
/// elements.
/// Each attribute in an item is a name-value pair. An attribute can be single-
/// valued or multi-valued set. For example, a book item can have title and
/// authors attributes. Each book has one title but can have many authors. The
/// multi-valued attribute is a set; duplicate values are not allowed.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct AttributeValue {
	/// A Binary data type.
	pub B: Option<BinaryAttributeValue>,
	/// A Null data type.
	pub NULL: Option<NullAttributeValue>,
	/// A String Set data type.
	pub SS: Option<StringSetAttributeValue>,
	/// A Map of attribute values.
	pub M: Option<MapAttributeValue>,
	/// A List of attribute values.
	pub L: Option<ListAttributeValue>,
	/// A Number data type.
	pub N: Option<NumberAttributeValue>,
	/// A String data type.
	pub S: Option<StringAttributeValue>,
	/// A Boolean data type.
	pub BOOL: Option<BooleanAttributeValue>,
	/// A Binary Set data type.
	pub BS: Option<BinarySetAttributeValue>,
	/// A Number Set data type.
	pub NS: Option<NumberSetAttributeValue>,
}

pub type StreamEnabled = bool;
/// Represents the properties of a local secondary index.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct LocalSecondaryIndex {
	/// The complete key schema for the local secondary index, consisting of one or
	/// more pairs of attribute names and key types (`HASH` or `RANGE`).
	pub KeySchema: KeySchema,
	/// The name of the local secondary index. The name must be unique among all other
	/// indexes on this table.
	pub IndexName: IndexName,
	pub Projection: Projection,
}

pub type AttributeAction = String;
pub type KeySchema = Vec<KeySchemaElement>;
pub type StreamViewType = String;
pub type ListTablesInputLimit = i32;
pub type ScanTotalSegments = i32;
pub type ConditionalOperator = String;
pub type LocalSecondaryIndexDescriptionList = Vec<LocalSecondaryIndexDescription>;
pub type LocalSecondaryIndexList = Vec<LocalSecondaryIndex>;
pub type UpdateExpression = String;
pub type Long = i64;
/// Represents the properties of a global secondary index.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GlobalSecondaryIndexDescription {
	/// The total size of the specified index, in bytes. DynamoDB updates this value
	/// approximately every six hours. Recent changes might not be reflected in this
	/// value.
	pub IndexSizeBytes: Option<Long>,
	/// The name of the global secondary index.
	pub IndexName: Option<IndexName>,
	pub Projection: Option<Projection>,
	pub ProvisionedThroughput: Option<ProvisionedThroughputDescription>,
	/// The current state of the global secondary index:
	///   * _CREATING_ \- The index is being created.
	///   * _UPDATING_ \- The index is being updated.
	///   * _DELETING_ \- The index is being deleted.
	///   * _ACTIVE_ \- The index is ready for use.
	pub IndexStatus: Option<IndexStatus>,
	/// Indicates whether the index is currently backfilling. _Backfilling_ is the
	/// process of reading items from the table and determining whether they can be
	/// added to the index. (Not all items will qualify: For example, a hash key
	/// attribute cannot have any duplicates.) If an item can be added to the index,
	/// DynamoDB will do so. After all items have been processed, the backfilling
	/// operation is complete and _Backfilling_ is false.
	/// For indexes that were created during a _CreateTable_ operation, the
	/// _Backfilling_ attribute does not appear in the _DescribeTable_ output.
	pub Backfilling: Option<Backfilling>,
	/// The complete key schema for the global secondary index, consisting of one or
	/// more pairs of attribute names and key types (`HASH` or `RANGE`).
	pub KeySchema: Option<KeySchema>,
	/// The Amazon Resource Name (ARN) that uniquely identifies the index.
	pub IndexArn: Option<String>,
	/// The number of items in the specified index. DynamoDB updates this value
	/// approximately every six hours. Recent changes might not be reflected in this
	/// value.
	pub ItemCount: Option<Long>,
}

pub type NumberSetAttributeValue = Vec<NumberAttributeValue>;
/// Represents the output of a _CreateTable_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateTableOutput {
	pub TableDescription: Option<TableDescription>,
}

/// Represents the properties of a global secondary index.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GlobalSecondaryIndex {
	/// The complete key schema for a global secondary index, which consists of one or
	/// more pairs of attribute names and key types (`HASH` or `RANGE`).
	pub KeySchema: KeySchema,
	/// The name of the global secondary index. The name must be unique among all
	/// other indexes on this table.
	pub IndexName: IndexName,
	pub Projection: Projection,
	pub ProvisionedThroughput: ProvisionedThroughput,
}

pub type GlobalSecondaryIndexUpdateList = Vec<GlobalSecondaryIndexUpdate>;
pub type ListAttributeValue = Vec<AttributeValue>;
pub type StringAttributeValue = String;
/// The operation tried to access a nonexistent table or index. The resource might
/// not be specified correctly, or its status might not be `ACTIVE`.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ResourceNotFoundException {
	/// The resource which is being requested does not exist.
	pub message: Option<ErrorMessage>,
}

pub type ConsumedCapacityUnits = f64;
pub type NumberAttributeValue = String;
pub type ReturnItemCollectionMetrics = String;
pub type FilterConditionMap = HashMap<AttributeName,Condition>;
/// Represents the DynamoDB Streams configuration for a table in DynamoDB.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct StreamSpecification {
	/// The DynamoDB Streams settings for the table. These settings consist of:
	///   * _StreamEnabled_ \- Indicates whether DynamoDB Streams is enabled (true) or disabled (false) on the table.
	///   * _StreamViewType_ \- When an item in the table is modified, _StreamViewType_ determines what information is written to the stream for this table. Valid values for _StreamViewType_ are:
	///     * _KEYS_ONLY_ \- Only the key attributes of the modified item are written to the stream.
	///     * _NEW_IMAGE_ \- The entire item, as it appears after it was modified, is written to the stream.
	///     * _OLD_IMAGE_ \- The entire item, as it appeared before it was modified, is written to the stream.
	///     * _NEW_AND_OLD_IMAGES_ \- Both the new and the old item images of the item are written to the stream.
	pub StreamViewType: Option<StreamViewType>,
	/// Indicates whether DynamoDB Streams is enabled (true) or disabled (false) on
	/// the table.
	pub StreamEnabled: Option<StreamEnabled>,
}

/// Represents the input of an _UpdateItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct UpdateItemInput {
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _UpdateExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// This parameter can be used for modifying top-level attributes; however, it
	/// does not support individual list or map elements.
	/// The names of attributes to be modified, the action to perform on each, and the
	/// new value for each. If you are updating an attribute that is an index key
	/// attribute for any indexes on that table, the attribute type must match the
	/// index key type defined in the _AttributesDefinition_ of the table description.
	/// You can use _UpdateItem_ to update any nonkey attributes.
	/// Attribute values cannot be null. String and Binary type attributes must have
	/// lengths greater than zero. Set type attributes must not be empty. Requests
	/// with empty values will be rejected with a _ValidationException_ exception.
	/// Each _AttributeUpdates_ element consists of an attribute name to modify, along
	/// with the following:
	///   * _Value_ \- The new value, if applicable, for this attribute.
	///   * _Action_ \- A value that specifies how to perform the update. This action is only valid for an existing attribute whose data type is Number or is a set; do not use `ADD` for other data types. 
	/// If an item with the specified primary key is found in the table, the following
	/// values perform the following actions:
	///     * `PUT` \- Adds the specified attribute to the item. If the attribute already exists, it is replaced by the new value. 
	///     * `DELETE` \- Removes the attribute and its value, if no value is specified for `DELETE`. The data type of the specified value must match the existing value's data type.
	/// If a set of values is specified, then those values are subtracted from the old
	/// set. For example, if the attribute value was the set `[a,b,c]` and the
	/// `DELETE` action specifies `[a,c]`, then the final attribute value is `[b]`.
	/// Specifying an empty set is an error.
	///     * `ADD` \- Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of `ADD` depends on the data type of the attribute:
	///       * If the existing attribute is a number, and if _Value_ is also a number, then _Value_ is mathematically added to the existing attribute. If _Value_ is a negative number, then it is subtracted from the existing attribute.
	/// If you use `ADD` to increment or decrement a number value for an item that
	/// doesn't exist before the update, DynamoDB uses 0 as the initial value.
	/// Similarly, if you use `ADD` for an existing item to increment or decrement an
	/// attribute value that doesn't exist before the update, DynamoDB uses `0` as the
	/// initial value. For example, suppose that the item you want to update doesn't
	/// have an attribute named _itemcount_, but you decide to `ADD` the number `3` to
	/// this attribute anyway. DynamoDB will create the _itemcount_ attribute, set its
	/// initial value to `0`, and finally add `3` to it. The result will be a new
	/// _itemcount_ attribute, with a value of `3`.
	///       * If the existing data type is a set, and if _Value_ is also a set, then _Value_ is appended to the existing set. For example, if the attribute value is the set `[1,2]`, and the `ADD` action specified `[3]`, then the final attribute value is `[1,2,3]`. An error occurs if an `ADD` action is specified for a set attribute and the attribute type specified does not match the existing set type. 
	/// Both sets must have the same primitive data type. For example, if the existing
	/// data type is a set of strings, _Value_ must also be a set of strings.
	/// If no item with the specified key is found in the table, the following values
	/// perform the following actions:
	///     * `PUT` \- Causes DynamoDB to create a new item with the specified primary key, and then adds the attribute. 
	///     * `DELETE` \- Nothing happens, because attributes cannot be deleted from a nonexistent item. The operation succeeds, but DynamoDB does not create a new item.
	///     * `ADD` \- Causes DynamoDB to create an item with the supplied primary key and number (or set of numbers) for the attribute value. The only data types allowed are Number and Number Set.
	/// If you provide any attributes that are part of an index key, then the data
	/// types for those attributes must match those of the schema in the table's
	/// attribute definition.
	pub AttributeUpdates: Option<AttributeUpdates>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ConditionExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A logical operator to apply to the conditions in the _Expected_ map:
	///   * `AND` \- If all of the conditions evaluate to true, then the entire map evaluates to true.
	///   * `OR` \- If at least one of the conditions evaluate to true, then the entire map evaluates to true.
	/// If you omit _ConditionalOperator_, then `AND` is the default.
	/// The operation will succeed only if the entire map evaluates to true.
	/// This parameter does not support attributes of type List or Map.
	pub ConditionalOperator: Option<ConditionalOperator>,
	/// One or more substitution tokens for attribute names in an expression. The
	/// following are some use cases for using _ExpressionAttributeNames_:
	///   * To access an attribute whose name conflicts with a DynamoDB reserved word.
	///   * To create a placeholder for repeating occurrences of an attribute name in an expression.
	///   * To prevent special characters in an attribute name from being misinterpreted in an expression.
	/// Use the **#** character in an expression to dereference an attribute name. For
	/// example, consider the following attribute name:
	///   * `Percentile`
	/// The name of this attribute conflicts with a reserved word, so it cannot be
	/// used directly in an expression. (For the complete list of reserved words, see
	/// [Reserved Words](http://docs.aws.amazon.com/amazondynamodb/latest/developergui
	/// de/ReservedWords.html) in the _Amazon DynamoDB Developer Guide_). To work
	/// around this, you could specify the following for _ExpressionAttributeNames_:
	///   * `{"#P":"Percentile"}`
	/// You could then use this substitution in an expression, as in this example:
	///   * `#P = :val`
	/// Tokens that begin with the **:** character are _expression attribute values_,
	/// which are placeholders for the actual value at runtime.
	/// For more information on expression attribute names, see [Accessing Item Attrib
	/// utes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressi
	/// ons.AccessingItemAttributes.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeNames: Option<ExpressionAttributeNameMap>,
	/// Use _ReturnValues_ if you want to get the item attributes as they appeared
	/// either before or after they were updated. For _UpdateItem_, the valid values
	/// are:
	///   * `NONE` \- If _ReturnValues_ is not specified, or if its value is `NONE`, then nothing is returned. (This setting is the default for _ReturnValues_.)
	///   * `ALL_OLD` \- If _UpdateItem_ overwrote an attribute name-value pair, then the content of the old item is returned.
	///   * `UPDATED_OLD` \- The old versions of only the updated attributes are returned.
	///   * `ALL_NEW` \- All of the attributes of the new version of the item are returned.
	///   * `UPDATED_NEW` \- The new versions of only the updated attributes are returned.
	pub ReturnValues: Option<ReturnValue>,
	/// A condition that must be satisfied in order for a conditional update to
	/// succeed.
	/// An expression can contain any of the following:
	///   * Functions: `attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size`
	/// These function names are case-sensitive.
	///   * Comparison operators: ` = | <> | < | > | <= | >= | BETWEEN | IN`
	///   * Logical operators: `AND | OR | NOT`
	/// For more information on condition expressions, see [Specifying Conditions](htt
	/// p://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Speci
	/// fyingConditions.html) in the _Amazon DynamoDB Developer Guide_.
	/// _ConditionExpression_ replaces the legacy _ConditionalOperator_ and _Expected_
	/// parameters.
	pub ConditionExpression: Option<ConditionExpression>,
	/// The name of the table containing the item to update.
	pub TableName: TableName,
	/// Determines whether item collection metrics are returned. If set to `SIZE`, the
	/// response includes statistics about item collections, if any, that were
	/// modified during the operation are returned in the response. If set to `NONE`
	/// (the default), no statistics are returned.
	pub ReturnItemCollectionMetrics: Option<ReturnItemCollectionMetrics>,
	pub ReturnConsumedCapacity: Option<ReturnConsumedCapacity>,
	/// An expression that defines one or more attributes to be updated, the action to
	/// be performed on them, and new value(s) for them.
	/// The following action values are available for _UpdateExpression_.
	///   * `SET` \- Adds one or more attributes and values to an item. If any of these attribute already exist, they are replaced by the new values. You can also use `SET` to add or subtract from an attribute that is of type Number. For example: `SET myNum = myNum + :val`
	/// `SET` supports the following functions:
	///     * `if_not_exists (path, operand)` \- if the item does not contain an attribute at the specified path, then `if_not_exists` evaluates to operand; otherwise, it evaluates to path. You can use this function to avoid overwriting an attribute that may already be present in the item.
	///     * `list_append (operand, operand)` \- evaluates to a list with a new element added to it. You can append the new element to the start or the end of the list by reversing the order of the operands.
	/// These function names are case-sensitive.
	///   * `REMOVE` \- Removes one or more attributes from an item.
	///   * `ADD` \- Adds the specified value to the item, if the attribute does not already exist. If the attribute does exist, then the behavior of `ADD` depends on the data type of the attribute:
	///     * If the existing attribute is a number, and if _Value_ is also a number, then _Value_ is mathematically added to the existing attribute. If _Value_ is a negative number, then it is subtracted from the existing attribute.
	/// If you use `ADD` to increment or decrement a number value for an item that
	/// doesn't exist before the update, DynamoDB uses `0` as the initial value.
	/// Similarly, if you use `ADD` for an existing item to increment or decrement an
	/// attribute value that doesn't exist before the update, DynamoDB uses `0` as the
	/// initial value. For example, suppose that the item you want to update doesn't
	/// have an attribute named _itemcount_, but you decide to `ADD` the number `3` to
	/// this attribute anyway. DynamoDB will create the _itemcount_ attribute, set its
	/// initial value to `0`, and finally add `3` to it. The result will be a new
	/// _itemcount_ attribute in the item, with a value of `3`.
	///     * If the existing data type is a set and if _Value_ is also a set, then _Value_ is added to the existing set. For example, if the attribute value is the set `[1,2]`, and the `ADD` action specified `[3]`, then the final attribute value is `[1,2,3]`. An error occurs if an `ADD` action is specified for a set attribute and the attribute type specified does not match the existing set type. 
	/// Both sets must have the same primitive data type. For example, if the existing
	/// data type is a set of strings, the _Value_ must also be a set of strings.
	/// The `ADD` action only supports Number and set data types. In addition, `ADD`
	/// can only be used on top-level attributes, not nested attributes.
	///   * `DELETE` \- Deletes an element from a set.
	/// If a set of values is specified, then those values are subtracted from the old
	/// set. For example, if the attribute value was the set `[a,b,c]` and the
	/// `DELETE` action specifies `[a,c]`, then the final attribute value is `[b]`.
	/// Specifying an empty set is an error.
	/// The `DELETE` action only supports set data types. In addition, `DELETE` can
	/// only be used on top-level attributes, not nested attributes.
	/// You can have many actions in a single expression, such as the following: `SET
	/// a=:value1, b=:value2 DELETE :value3, :value4, :value5`
	/// For more information on update expressions, see [Modifying Items and Attribute
	/// s](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions
	/// .Modifying.html) in the _Amazon DynamoDB Developer Guide_.
	/// _UpdateExpression_ replaces the legacy _AttributeUpdates_ parameter.
	pub UpdateExpression: Option<UpdateExpression>,
	/// One or more values that can be substituted in an expression.
	/// Use the **:** (colon) character in an expression to dereference an attribute
	/// value. For example, suppose that you wanted to check whether the value of the
	/// _ProductStatus_ attribute was one of the following:
	/// `Available | Backordered | Discontinued`
	/// You would first need to specify _ExpressionAttributeValues_ as follows:
	/// `{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"},
	/// ":disc":{"S":"Discontinued"} }`
	/// You could then use these values in an expression, such as this:
	/// `ProductStatus IN (:avail, :back, :disc)`
	/// For more information on expression attribute values, see [Specifying Condition
	/// s](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions
	/// .SpecifyingConditions.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeValues: Option<ExpressionAttributeValueMap>,
	/// The primary key of the item to be updated. Each element consists of an
	/// attribute name and a value for that attribute.
	/// For the primary key, you must provide all of the attributes. For example, with
	/// a hash type primary key, you only need to provide the hash attribute. For a
	/// hash-and-range type primary key, you must provide both the hash attribute and
	/// the range attribute.
	pub Key: Key,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ ConditionExpression _ instead. Do not combine legacy parameters
	/// and expression parameters in a single API call; otherwise, DynamoDB will
	/// return a _ValidationException_ exception.
	/// A map of attribute/condition pairs. _Expected_ provides a conditional block
	/// for the _UpdateItem_ operation.
	/// Each element of _Expected_ consists of an attribute name, a comparison
	/// operator, and one or more values. DynamoDB compares the attribute with the
	/// value(s) you supplied, using the comparison operator. For each _Expected_
	/// element, the result of the evaluation is either true or false.
	/// If you specify more than one element in the _Expected_ map, then by default
	/// all of the conditions must evaluate to true. In other words, the conditions
	/// are ANDed together. (You can use the _ConditionalOperator_ parameter to OR the
	/// conditions instead. If you do this, then at least one of the conditions must
	/// evaluate to true, rather than all of them.)
	/// If the _Expected_ map evaluates to true, then the conditional operation
	/// succeeds; otherwise, it fails.
	/// _Expected_ contains the following:
	///   * _AttributeValueList_ \- One or more values to evaluate against the supplied attribute. The number of values in the list depends on the _ComparisonOperator_ being used.
	/// For type Number, value comparisons are numeric.
	/// String value comparisons for greater than, equals, or less than are based on
	/// ASCII character code values. For example, `a` is greater than `A`, and `a` is
	/// greater than `B`. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	/// For type Binary, DynamoDB treats each byte of the binary data as unsigned when
	/// it compares binary values.
	///   * _ComparisonOperator_ \- A comparator for evaluating attributes in the _AttributeValueList_. When performing the comparison, DynamoDB uses strongly consistent reads.
	/// The following comparison operators are available:
	/// `EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS |
	/// BEGINS_WITH | IN | BETWEEN`
	/// The following are descriptions of each comparison operator.
	///     * `EQ` : Equal. `EQ` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, Binary, String Set, Number Set, or Binary Set. If an item
	/// contains an _AttributeValue_ element of a different type than the one provided
	/// in the request, the value does not match. For example, `{"S":"6"}` does not
	/// equal `{"N":"6"}`. Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///     * `NE` : Not equal. `NE` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, Binary, String Set, Number Set, or Binary Set. If an item contains an
	/// _AttributeValue_ of a different type than the one provided in the request, the
	/// value does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`.
	/// Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///     * `LE` : Less than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `LT` : Less than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, or Binary (not a set type). If an item contains an _AttributeValue_
	/// element of a different type than the one provided in the request, the value
	/// does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`. Also,
	/// `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `GE` : Greater than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `GT` : Greater than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `NOT_NULL` : The attribute exists. `NOT_NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the existence of an attribute, not its data type. If
	/// the data type of attribute "`a`" is null, and you evaluate it using
	/// `NOT_NULL`, the result is a Boolean _true_. This result is because the
	/// attribute "`a`" exists; its data type is not relevant to the `NOT_NULL`
	/// comparison operator.
	///     * `NULL` : The attribute does not exist. `NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the nonexistence of an attribute, not its data type.
	/// If the data type of attribute "`a`" is null, and you evaluate it using `NULL`,
	/// the result is a Boolean _false_. This is because the attribute "`a`" exists;
	/// its data type is not relevant to the `NULL` comparison operator.
	///     * `CONTAINS` : Checks for a subsequence, or value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is of type String, then the operator checks for a substring match.
	/// If the target attribute of the comparison is of type Binary, then the operator
	/// looks for a subsequence of the target that matches the input. If the target
	/// attribute of the comparison is a set ("`SS`", "`NS`", or "`BS`"), then the
	/// operator evaluates to true if it finds an exact match with any member of the
	/// set.
	/// CONTAINS is supported for lists: When evaluating "`a CONTAINS b`", "`a`" can
	/// be a list; however, "`b`" cannot be a set, a map, or a list.
	///     * `NOT_CONTAINS` : Checks for absence of a subsequence, or absence of a value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is a String, then the operator checks for the absence of a
	/// substring match. If the target attribute of the comparison is Binary, then the
	/// operator checks for the absence of a subsequence of the target that matches
	/// the input. If the target attribute of the comparison is a set ("`SS`", "`NS`",
	/// or "`BS`"), then the operator evaluates to true if it _does not_ find an exact
	/// match with any member of the set.
	/// NOT_CONTAINS is supported for lists: When evaluating "`a NOT CONTAINS b`",
	/// "`a`" can be a list; however, "`b`" cannot be a set, a map, or a list.
	///     * `BEGINS_WITH` : Checks for a prefix. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String or
	/// Binary (not a Number or a set type). The target attribute of the comparison
	/// must be of type String or Binary (not a Number or a set type).
	///     * `IN` : Checks for matching elements within two sets.
	/// _AttributeValueList_ can contain one or more _AttributeValue_ elements of type
	/// String, Number, or Binary (not a set type). These attributes are compared
	/// against an existing set type attribute of an item. If any elements of the
	/// input set are present in the item attribute, the expression evaluates to true.
	///     * `BETWEEN` : Greater than or equal to the first value, and less than or equal to the second value. 
	/// _AttributeValueList_ must contain two _AttributeValue_ elements of the same
	/// type, either String, Number, or Binary (not a set type). A target attribute
	/// matches if the target value is greater than, or equal to, the first element
	/// and less than, or equal to, the second element. If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not compare
	/// to `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`
	/// For usage examples of _AttributeValueList_ and _ComparisonOperator_, see
	/// [Legacy Conditional Parameters](http://docs.aws.amazon.com/amazondynamodb/late
	/// st/developerguide/LegacyConditionalParameters.html) in the _Amazon DynamoDB
	/// Developer Guide_.
	/// For backward compatibility with previous DynamoDB releases, the following
	/// parameters can be used instead of _AttributeValueList_ and
	/// _ComparisonOperator_:
	///   * _Value_ \- A value for DynamoDB to compare with an attribute.
	///   * _Exists_ \- A Boolean value that causes DynamoDB to evaluate the value before attempting the conditional operation:
	///     * If _Exists_ is `true`, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the condition evaluates to true; otherwise the condition evaluate to false.
	///     * If _Exists_ is `false`, DynamoDB assumes that the attribute value does _not_ exist in the table. If in fact the value does not exist, then the assumption is valid and the condition evaluates to true. If the value is found, despite the assumption that it does not exist, the condition evaluates to false.
	/// Note that the default value for _Exists_ is `true`.
	/// The _Value_ and _Exists_ parameters are incompatible with _AttributeValueList_
	/// and _ComparisonOperator_. Note that if you use both sets of parameters at
	/// once, DynamoDB will return a _ValidationException_ exception.
	/// This parameter does not support attributes of type List or Map.
	pub Expected: Option<ExpectedAttributeMap>,
}

pub type Key = HashMap<AttributeName,AttributeValue>;
pub type BatchGetRequestMap = HashMap<TableName,KeysAndAttributes>;
pub type AttributeUpdates = HashMap<AttributeName,AttributeValueUpdate>;
/// A condition specified in the operation could not be evaluated.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ConditionalCheckFailedException {
	/// The conditional request failed.
	pub message: Option<ErrorMessage>,
}

pub type KeySchemaAttributeName = String;
pub type ItemCollectionMetricsMultiple = Vec<ItemCollectionMetrics>;
/// Represents the input of an _UpdateTable_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct UpdateTableInput {
	/// An array of one or more global secondary indexes for the table. For each index
	/// in the array, you can request one action:
	///   * _Create_ \- add a new global secondary index to the table.
	///   * _Update_ \- modify the provisioned throughput settings of an existing global secondary index.
	///   * _Delete_ \- remove a global secondary index from the table.
	/// For more information, see [Managing Global Secondary Indexes](http://docs.aws.
	/// amazon.com/amazondynamodb/latest/developerguide/GSI.OnlineOps.html) in the
	/// _Amazon DynamoDB Developer Guide_.
	pub GlobalSecondaryIndexUpdates: Option<GlobalSecondaryIndexUpdateList>,
	pub ProvisionedThroughput: Option<ProvisionedThroughput>,
	/// Represents the DynamoDB Streams configuration for the table.
	/// You will receive a _ResourceInUseException_ if you attempt to enable a stream
	/// on a table that already has a stream, or if you attempt to disable a stream on
	/// a table which does not have a stream.
	pub StreamSpecification: Option<StreamSpecification>,
	/// The name of the table to be updated.
	pub TableName: TableName,
	/// An array of attributes that describe the key schema for the table and indexes.
	/// If you are adding a new global secondary index to the table,
	/// _AttributeDefinitions_ must include the key element(s) of the new index.
	pub AttributeDefinitions: Option<AttributeDefinitions>,
}

/// Represents the input of a _DescribeTable_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DescribeTableInput {
	/// The name of the table to describe.
	pub TableName: TableName,
}

pub type NonKeyAttributeNameList = Vec<NonKeyAttributeName>;
pub type ExpressionAttributeNameVariable = String;
pub type IndexStatus = String;
/// Represents the provisioned throughput settings for the table, consisting of
/// read and write capacity units, along with data about increases and decreases.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ProvisionedThroughputDescription {
	/// The number of provisioned throughput decreases for this table during this UTC
	/// calendar day. For current maximums on provisioned throughput decreases, see [L
	/// imits](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.
	/// html) in the _Amazon DynamoDB Developer Guide_.
	pub NumberOfDecreasesToday: Option<PositiveLongObject>,
	/// The maximum number of writes consumed per second before DynamoDB returns a
	/// _ThrottlingException_.
	pub WriteCapacityUnits: Option<PositiveLongObject>,
	/// The date and time of the last provisioned throughput increase for this table.
	pub LastIncreaseDateTime: Option<Date>,
	/// The maximum number of strongly consistent reads consumed per second before
	/// DynamoDB returns a _ThrottlingException_. Eventually consistent reads require
	/// less effort than strongly consistent reads, so a setting of 50
	/// _ReadCapacityUnits_ per second provides 100 eventually consistent
	/// _ReadCapacityUnits_ per second.
	pub ReadCapacityUnits: Option<PositiveLongObject>,
	/// The date and time of the last provisioned throughput decrease for this table.
	pub LastDecreaseDateTime: Option<Date>,
}

pub type ErrorMessage = String;
pub type AttributeValueList = Vec<AttributeValue>;
/// Represents the input of a _BatchWriteItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct BatchWriteItemInput {
	/// A map of one or more table names and, for each table, a list of operations to
	/// be performed (_DeleteRequest_ or _PutRequest_). Each element in the map
	/// consists of the following:
	///   * _DeleteRequest_ \- Perform a _DeleteItem_ operation on the specified item. The item to be deleted is identified by a _Key_ subelement:
	///     * _Key_ \- A map of primary key attribute values that uniquely identify the ! item. Each entry in this map consists of an attribute name and an attribute value. For each primary key, you must provide _all_ of the key attributes. For example, with a hash type primary key, you only need to provide the hash attribute. For a hash-and-range type primary key, you must provide _both_ the hash attribute and the range attribute.
	///   * _PutRequest_ \- Perform a _PutItem_ operation on the specified item. The item to be put is identified by an _Item_ subelement:
	///     * _Item_ \- A map of attributes and their values. Each entry in this map consists of an attribute name and an attribute value. Attribute values must not be null; string and binary type attributes must have lengths greater than zero; and set type attributes must not be empty. Requests that contain empty values will be rejected with a _ValidationException_ exception.
	/// If you specify any attributes that are part of an index key, then the data
	/// types for those attributes must match those of the schema in the table's
	/// attribute definition.
	pub RequestItems: BatchWriteItemRequestMap,
	pub ReturnConsumedCapacity: Option<ReturnConsumedCapacity>,
	/// Determines whether item collection metrics are returned. If set to `SIZE`, the
	/// response includes statistics about item collections, if any, that were
	/// modified during the operation are returned in the response. If set to `NONE`
	/// (the default), no statistics are returned.
	pub ReturnItemCollectionMetrics: Option<ReturnItemCollectionMetrics>,
}

pub type BatchGetResponseMap = HashMap<TableName,ItemList>;
/// Information about item collections, if any, that were affected by the
/// operation. _ItemCollectionMetrics_ is only returned if the request asked for
/// it. If the table does not have any local secondary indexes, this information
/// is not returned in the response.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ItemCollectionMetrics {
	/// The hash key value of the item collection. This value is the same as the hash
	/// key of the item.
	pub ItemCollectionKey: Option<ItemCollectionKeyAttributeMap>,
	/// An estimate of item collection size, in gigabytes. This value is a two-element
	/// array containing a lower bound and an upper bound for the estimate. The
	/// estimate includes the size of all the items in the table, plus the size of all
	/// attributes projected into all of the local secondary indexes on that table.
	/// Use this estimate to measure whether a local secondary index is approaching
	/// its size limit.
	/// The estimate is subject to change over time; therefore, do not rely on the
	/// precision or accuracy of the estimate.
	pub SizeEstimateRangeGB: Option<ItemCollectionSizeEstimateRange>,
}

/// Represents an attribute for describing the key schema for the table and
/// indexes.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct AttributeDefinition {
	/// A name for the attribute.
	pub AttributeName: KeySchemaAttributeName,
	/// The data type for the attribute.
	pub AttributeType: ScalarAttributeType,
}

pub type StringSetAttributeValue = Vec<StringAttributeValue>;
/// Represents the output of a _DeleteTable_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteTableOutput {
	pub TableDescription: Option<TableDescription>,
}

pub type Select = String;
/// Represents the amount of provisioned throughput capacity consumed on a table
/// or an index.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Capacity {
	/// The total number of capacity units consumed on a table or an index.
	pub CapacityUnits: Option<ConsumedCapacityUnits>,
}

/// Represents the output of a _Query_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct QueryOutput {
	/// The number of items in the response.
	/// If you used a _QueryFilter_ in the request, then _Count_ is the number of
	/// items returned after the filter was applied, and _ScannedCount_ is the number
	/// of matching items before&gt; the filter was applied.
	/// If you did not use a filter in the request, then _Count_ and _ScannedCount_
	/// are the same.
	pub Count: Option<Integer>,
	/// An array of item attributes that match the query criteria. Each element in
	/// this array consists of an attribute name and the value for that attribute.
	pub Items: Option<ItemList>,
	/// The primary key of the item where the operation stopped, inclusive of the
	/// previous result set. Use this value to start a new operation, excluding this
	/// value in the new request.
	/// If _LastEvaluatedKey_ is empty, then the "last page" of results has been
	/// processed and there is no more data to be retrieved.
	/// If _LastEvaluatedKey_ is not empty, it does not necessarily mean that there is
	/// more data in the result set. The only way to know when you have reached the
	/// end of the result set is when _LastEvaluatedKey_ is empty.
	pub LastEvaluatedKey: Option<Key>,
	/// The number of items evaluated, before any _QueryFilter_ is applied. A high
	/// _ScannedCount_ value with few, or no, _Count_ results indicates an inefficient
	/// _Query_ operation. For more information, see [Count and ScannedCount](http://d
	/// ocs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Coun
	/// t) in the _Amazon DynamoDB Developer Guide_.
	/// If you did not use a filter in the request, then _ScannedCount_ is the same as
	/// _Count_.
	pub ScannedCount: Option<Integer>,
	pub ConsumedCapacity: Option<ConsumedCapacity>,
}

pub type ScanSegment = i32;
/// Represents the input of a _ListTables_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListTablesInput {
	/// A maximum number of table names to return. If this parameter is not specified,
	/// the limit is 100.
	pub Limit: Option<ListTablesInputLimit>,
	/// The first table name that this operation will evaluate. Use the value that was
	/// returned for _LastEvaluatedTableName_ in a previous operation, so that you can
	/// obtain the next page of results.
	pub ExclusiveStartTableName: Option<TableName>,
}

/// The number of concurrent table requests (cumulative number of tables in the
/// `CREATING`, `DELETING` or `UPDATING` state) exceeds the maximum allowed of 10.
/// Also, for tables with secondary indexes, only one of those tables can be in
/// the `CREATING` state at any point in time. Do not attempt to create more than
/// one such table simultaneously.
/// The total limit of tables in the `ACTIVE` state is 250.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct LimitExceededException {
	/// Too many operations for a given subscriber.
	pub message: Option<ErrorMessage>,
}

pub type NonKeyAttributeName = String;
pub type NullAttributeValue = bool;
pub type ProjectionType = String;
pub type ItemCollectionSizeEstimateRange = Vec<ItemCollectionSizeEstimateBound>;
/// Represents one of the following:
///   * A new global secondary index to be added to an existing table.
///   * New provisioned throughput parameters for an existing global secondary index.
///   * An existing global secondary index to be removed from an existing table.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GlobalSecondaryIndexUpdate {
	/// The parameters required for creating a global secondary index on an existing
	/// table:
	///   * `IndexName `
	///   * `KeySchema `
	///   * `AttributeDefinitions `
	///   * `Projection `
	///   * `ProvisionedThroughput `
	pub Create: Option<CreateGlobalSecondaryIndexAction>,
	/// The name of an existing global secondary index, along with new provisioned
	/// throughput settings to be applied to that index.
	pub Update: Option<UpdateGlobalSecondaryIndexAction>,
	/// The name of an existing global secondary index to be removed.
	pub Delete: Option<DeleteGlobalSecondaryIndexAction>,
}

pub type BinarySetAttributeValue = Vec<BinaryAttributeValue>;
/// The operation conflicts with the resource's availability. For example, you
/// attempted to recreate an existing table, or tried to delete a table currently
/// in the `CREATING` state.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ResourceInUseException {
	/// The resource which is being attempted to be changed is in use.
	pub message: Option<ErrorMessage>,
}

pub type ExpressionAttributeNameMap = HashMap<ExpressionAttributeNameVariable,AttributeName>;
/// Represents the input of a _Query_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct QueryInput {
	/// A string that contains conditions that DynamoDB applies after the _Query_
	/// operation, but before the data is returned to you. Items that do not satisfy
	/// the _FilterExpression_ criteria are not returned.
	/// A _FilterExpression_ is applied after the items have already been read; the
	/// process of filtering does not consume any additional read capacity units.
	/// For more information, see [Filter Expressions](http://docs.aws.amazon.com/amaz
	/// ondynamodb/latest/developerguide/QueryAndScan.html#FilteringResults) in the
	/// _Amazon DynamoDB Developer Guide_.
	/// _FilterExpression_ replaces the legacy _QueryFilter_ and _ConditionalOperator_
	/// parameters.
	pub FilterExpression: Option<ConditionExpression>,
	/// Determines the read consistency model: If set to `true`, then the operation
	/// uses strongly consistent reads; otherwise, the operation uses eventually
	/// consistent reads.
	/// Strongly consistent reads are not supported on global secondary indexes. If
	/// you query a global secondary index with _ConsistentRead_ set to `true`, you
	/// will receive a _ValidationException_.
	pub ConsistentRead: Option<ConsistentRead>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _FilterExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A logical operator to apply to the conditions in a _QueryFilter_ map:
	///   * `AND` \- If all of the conditions evaluate to true, then the entire map evaluates to true.
	///   * `OR` \- If at least one of the conditions evaluate to true, then the entire map evaluates to true.
	/// If you omit _ConditionalOperator_, then `AND` is the default.
	/// The operation will succeed only if the entire map evaluates to true.
	/// This parameter does not support attributes of type List or Map.
	pub ConditionalOperator: Option<ConditionalOperator>,
	/// The name of an index to query. This index can be any local secondary index or
	/// global secondary index on the table. Note that if you use the _IndexName_
	/// parameter, you must also provide _TableName._
	pub IndexName: Option<IndexName>,
	/// A string that identifies one or more attributes to retrieve from the table.
	/// These attributes can include scalars, sets, or elements of a JSON document.
	/// The attributes in the expression must be separated by commas.
	/// If no attribute names are specified, then all attributes will be returned. If
	/// any of the requested attributes are not found, they will not appear in the
	/// result.
	/// For more information, see [Accessing Item Attributes](http://docs.aws.amazon.c
	/// om/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.ht
	/// ml) in the _Amazon DynamoDB Developer Guide_.
	/// _ProjectionExpression_ replaces the legacy _AttributesToGet_ parameter.
	pub ProjectionExpression: Option<ProjectionExpression>,
	/// One or more substitution tokens for attribute names in an expression. The
	/// following are some use cases for using _ExpressionAttributeNames_:
	///   * To access an attribute whose name conflicts with a DynamoDB reserved word.
	///   * To create a placeholder for repeating occurrences of an attribute name in an expression.
	///   * To prevent special characters in an attribute name from being misinterpreted in an expression.
	/// Use the **#** character in an expression to dereference an attribute name. For
	/// example, consider the following attribute name:
	///   * `Percentile`
	/// The name of this attribute conflicts with a reserved word, so it cannot be
	/// used directly in an expression. (For the complete list of reserved words, see
	/// [Reserved Words](http://docs.aws.amazon.com/amazondynamodb/latest/developergui
	/// de/ReservedWords.html) in the _Amazon DynamoDB Developer Guide_). To work
	/// around this, you could specify the following for _ExpressionAttributeNames_:
	///   * `{"#P":"Percentile"}`
	/// You could then use this substitution in an expression, as in this example:
	///   * `#P = :val`
	/// Tokens that begin with the **:** character are _expression attribute values_,
	/// which are placeholders for the actual value at runtime.
	/// For more information on expression attribute names, see [Accessing Item Attrib
	/// utes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressi
	/// ons.AccessingItemAttributes.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeNames: Option<ExpressionAttributeNameMap>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _FilterExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A condition that evaluates the query results after the items are read and
	/// returns only the desired values.
	/// This parameter does not support attributes of type List or Map.
	/// A _QueryFilter_ is applied after the items have already been read; the process
	/// of filtering does not consume any additional read capacity units.
	/// If you provide more than one condition in the _QueryFilter_ map, then by
	/// default all of the conditions must evaluate to true. In other words, the
	/// conditions are ANDed together. (You can use the _ConditionalOperator_
	/// parameter to OR the conditions instead. If you do this, then at least one of
	/// the conditions must evaluate to true, rather than all of them.)
	/// Note that _QueryFilter_ does not allow key attributes. You cannot define a
	/// filter condition on a hash key or range key.
	/// Each _QueryFilter_ element consists of an attribute name to compare, along
	/// with the following:
	///   * _AttributeValueList_ \- One or more values to evaluate against the supplied attribute. The number of values in the list depends on the operator specified in _ComparisonOperator_.
	/// For type Number, value comparisons are numeric.
	/// String value comparisons for greater than, equals, or less than are based on
	/// ASCII character code values. For example, `a` is greater than `A`, and `a` is
	/// greater than `B`. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	/// For type Binary, DynamoDB treats each byte of the binary data as unsigned when
	/// it compares binary values.
	/// For information on specifying data types in JSON, see [JSON Data Format](http:
	/// //docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataFormat.html) in
	/// the _Amazon DynamoDB Developer Guide_.
	///   * _ComparisonOperator_ \- A comparator for evaluating attributes. For example, equals, greater than, less than, etc.
	/// The following comparison operators are available:
	/// `EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS |
	/// BEGINS_WITH | IN | BETWEEN`
	/// For complete descriptions of all comparison operators, see the [Condition](htt
	/// p://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Condition.html)
	/// data type.
	pub QueryFilter: Option<FilterConditionMap>,
	/// The name of the table containing the requested items.
	pub TableName: TableName,
	pub ReturnConsumedCapacity: Option<ReturnConsumedCapacity>,
	/// The primary key of the first item that this operation will evaluate. Use the
	/// value that was returned for _LastEvaluatedKey_ in the previous operation.
	/// The data type for _ExclusiveStartKey_ must be String, Number or Binary. No set
	/// data types are allowed.
	pub ExclusiveStartKey: Option<Key>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ProjectionExpression_ instead. Do not combine legacy parameters
	/// and expression parameters in a single API call; otherwise, DynamoDB will
	/// return a _ValidationException_ exception.
	/// This parameter allows you to retrieve attributes of type List or Map; however,
	/// it cannot retrieve individual elements within a List or a Map.
	/// The names of one or more attributes to retrieve. If no attribute names are
	/// provided, then all attributes will be returned. If any of the requested
	/// attributes are not found, they will not appear in the result.
	/// Note that _AttributesToGet_ has no effect on provisioned throughput
	/// consumption. DynamoDB determines capacity units consumed based on item size,
	/// not on the amount of data that is returned to an application.
	/// You cannot use both _AttributesToGet_ and _Select_ together in a _Query_
	/// request, _unless_ the value for _Select_ is `SPECIFIC_ATTRIBUTES`. (This usage
	/// is equivalent to specifying _AttributesToGet_ without any value for _Select_.)
	/// If you query a local secondary index and request only attributes that are
	/// projected into that index, the operation will read only the index and not the
	/// table. If any of the requested attributes are not projected into the local
	/// secondary index, DynamoDB will fetch each of these attributes from the parent
	/// table. This extra fetching incurs additional throughput cost and latency.
	/// If you query a global secondary index, you can only request attributes that
	/// are projected into the index. Global secondary index queries cannot fetch
	/// attributes from the parent table.
	pub AttributesToGet: Option<AttributeNameList>,
	/// The maximum number of items to evaluate (not necessarily the number of
	/// matching items). If DynamoDB processes the number of items up to the limit
	/// while processing the results, it stops the operation and returns the matching
	/// values up to that point, and a key in _LastEvaluatedKey_ to apply in a
	/// subsequent operation, so that you can pick up where you left off. Also, if the
	/// processed data set size exceeds 1 MB before DynamoDB reaches this limit, it
	/// stops the operation and returns the matching values up to the limit, and a key
	/// in _LastEvaluatedKey_ to apply in a subsequent operation to continue the
	/// operation. For more information, see [Query and Scan](http://docs.aws.amazon.c
	/// om/amazondynamodb/latest/developerguide/QueryAndScan.html) in the _Amazon
	/// DynamoDB Developer Guide_.
	pub Limit: Option<PositiveIntegerObject>,
	/// One or more values that can be substituted in an expression.
	/// Use the **:** (colon) character in an expression to dereference an attribute
	/// value. For example, suppose that you wanted to check whether the value of the
	/// _ProductStatus_ attribute was one of the following:
	/// `Available | Backordered | Discontinued`
	/// You would first need to specify _ExpressionAttributeValues_ as follows:
	/// `{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"},
	/// ":disc":{"S":"Discontinued"} }`
	/// You could then use these values in an expression, such as this:
	/// `ProductStatus IN (:avail, :back, :disc)`
	/// For more information on expression attribute values, see [Specifying Condition
	/// s](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions
	/// .SpecifyingConditions.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeValues: Option<ExpressionAttributeValueMap>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _KeyConditionExpression_ instead. Do not combine legacy parameters
	/// and expression parameters in a single API call; otherwise, DynamoDB will
	/// return a _ValidationException_ exception.
	/// The selection criteria for the query. For a query on a table, you can have
	/// conditions only on the table primary key attributes. You must provide the hash
	/// key attribute name and value as an `EQ` condition. You can optionally provide
	/// a second condition, referring to the range key attribute.
	/// If you don't provide a range key condition, all of the items that match the
	/// hash key will be retrieved. If a _FilterExpression_ or _QueryFilter_ is
	/// present, it will be applied after the items are retrieved.
	/// For a query on an index, you can have conditions only on the index key
	/// attributes. You must provide the index hash attribute name and value as an
	/// `EQ` condition. You can optionally provide a second condition, referring to
	/// the index key range attribute.
	/// Each _KeyConditions_ element consists of an attribute name to compare, along
	/// with the following:
	///   * _AttributeValueList_ \- One or more values to evaluate against the supplied attribute. The number of values in the list depends on the _ComparisonOperator_ being used.
	/// For type Number, value comparisons are numeric.
	/// String value comparisons for greater than, equals, or less than are based on
	/// ASCII character code values. For example, `a` is greater than `A`, and `a` is
	/// greater than `B`. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	/// For Binary, DynamoDB treats each byte of the binary data as unsigned when it
	/// compares binary values.
	///   * _ComparisonOperator_ \- A comparator for evaluating attributes, for example, equals, greater than, less than, and so on.
	/// For _KeyConditions_, only the following comparison operators are supported:
	/// `EQ | LE | LT | GE | GT | BEGINS_WITH | BETWEEN`
	/// The following are descriptions of these comparison operators.
	///     * `EQ` : Equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, or Binary (not a set type). If an item contains an _AttributeValue_
	/// element of a different type than the one specified in the request, the value
	/// does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`. Also,
	/// `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///     * `LE` : Less than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `LT` : Less than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, or Binary (not a set type). If an item contains an _AttributeValue_
	/// element of a different type than the one provided in the request, the value
	/// does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`. Also,
	/// `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `GE` : Greater than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `GT` : Greater than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `BEGINS_WITH` : Checks for a prefix. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String or
	/// Binary (not a Number or a set type). The target attribute of the comparison
	/// must be of type String or Binary (not a Number or a set type).
	///     * `BETWEEN` : Greater than or equal to the first value, and less than or equal to the second value. 
	/// _AttributeValueList_ must contain two _AttributeValue_ elements of the same
	/// type, either String, Number, or Binary (not a set type). A target attribute
	/// matches if the target value is greater than, or equal to, the first element
	/// and less than, or equal to, the second element. If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not compare
	/// to `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`
	/// For usage examples of _AttributeValueList_ and _ComparisonOperator_, see
	/// [Legacy Conditional Parameters](http://docs.aws.amazon.com/amazondynamodb/late
	/// st/developerguide/LegacyConditionalParameters.html) in the _Amazon DynamoDB
	/// Developer Guide_.
	pub KeyConditions: Option<KeyConditions>,
	/// The condition that specifies the key value(s) for items to be retrieved by the
	/// _Query_ action.
	/// The condition must perform an equality test on a single hash key value. The
	/// condition can also perform one of several comparison tests on a single range
	/// key value. _Query_ can use _KeyConditionExpression_ to retrieve one item with
	/// a given hash and range key value, or several items that have the same hash key
	/// value but different range key values.
	/// The hash key equality test is required, and must be specified in the following
	/// format:
	/// `hashAttributeName` _=_ `:hashval`
	/// If you also want to provide a range key condition, it must be combined using
	/// _AND_ with the hash key condition. Following is an example, using the **=**
	/// comparison operator for the range key:
	/// `hashAttributeName` _=_ `:hashval` _AND_ `rangeAttributeName` _=_ `:rangeval`
	/// Valid comparisons for the range key condition are as follows:
	///   * `rangeAttributeName` _=_ `:rangeval` \- true if the range key is equal to `:rangeval`.
	///   * `rangeAttributeName` _&lt;_ `:rangeval` \- true if the range key is less than `:rangeval`.
	///   * `rangeAttributeName` _&lt;=_ `:rangeval` \- true if the range key is less than or equal to `:rangeval`.
	///   * `rangeAttributeName` _&gt;_ `:rangeval` \- true if the range key is greater than `:rangeval`.
	///   * `rangeAttributeName` _&gt;= _`:rangeval` \- true if the range key is greater than or equal to `:rangeval`.
	///   * `rangeAttributeName` _BETWEEN_ `:rangeval1` _AND_ `:rangeval2` \- true if the range key is greater than or equal to `:rangeval1`, and less than or equal to `:rangeval2`.
	///   * _begins_with (_`rangeAttributeName`, `:rangeval`_)_ \- true if the range key begins with a particular operand. (You cannot use this function with a range key that is of type Number.) Note that the function name `begins_with` is case-sensitive.
	/// Use the _ExpressionAttributeValues_ parameter to replace tokens such as
	/// `:hashval` and `:rangeval` with actual values at runtime.
	/// You can optionally use the _ExpressionAttributeNames_ parameter to replace the
	/// names of the hash and range attributes with placeholder tokens. This option
	/// might be necessary if an attribute name conflicts with a DynamoDB reserved
	/// word. For example, the following _KeyConditionExpression_ parameter causes an
	/// error because _Size_ is a reserved word:
	///   * `Size = :myval`
	/// To work around this, define a placeholder (such a `#S`) to represent the
	/// attribute name _Size_. _KeyConditionExpression_ then is as follows:
	///   * `#S = :myval`
	/// For a list of reserved words, see [Reserved Words](http://docs.aws.amazon.com/
	/// amazondynamodb/latest/developerguide/ReservedWords.html) in the _Amazon
	/// DynamoDB Developer Guide_.
	/// For more information on _ExpressionAttributeNames_ and
	/// _ExpressionAttributeValues_, see [Using Placeholders for Attribute Names and V
	/// alues](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Express
	/// ionPlaceholders.html) in the _Amazon DynamoDB Developer Guide_.
	/// _KeyConditionExpression_ replaces the legacy _KeyConditions_ parameter.
	pub KeyConditionExpression: Option<KeyExpression>,
	/// Specifies the order in which to return the query results - either ascending
	/// (`true`) or descending (`false`).
	/// Items with the same hash key are stored in sorted order by range key .If the
	/// range key data type is Number, the results are stored in numeric order. For
	/// type String, the results are returned in order of ASCII character code values.
	/// For type Binary, DynamoDB treats each byte of the binary data as unsigned.
	/// If _ScanIndexForward_ is `true`, DynamoDB returns the results in order, by
	/// range key. This is the default behavior.
	/// If _ScanIndexForward_ is `false`, DynamoDB sorts the results in descending
	/// order by range key, and then returns the results to the client.
	pub ScanIndexForward: Option<BooleanObject>,
	/// The attributes to be returned in the result. You can retrieve all item
	/// attributes, specific item attributes, the count of matching items, or in the
	/// case of an index, some or all of the attributes projected into the index.
	///   * `ALL_ATTRIBUTES` \- Returns all of the item attributes from the specified table or index. If you query a local secondary index, then for each matching item in the index DynamoDB will fetch the entire item from the parent table. If the index is configured to project all item attributes, then all of the data can be obtained from the local secondary index, and no fetching is required.
	///   * `ALL_PROJECTED_ATTRIBUTES` \- Allowed only when querying an index. Retrieves all attributes that have been projected into the index. If the index is configured to project all attributes, this return value is equivalent to specifying `ALL_ATTRIBUTES`.
	///   * `COUNT` \- Returns the number of matching items, rather than the matching items themselves.
	///   * `SPECIFIC_ATTRIBUTES` \- Returns only the attributes listed in _AttributesToGet_. This return value is equivalent to specifying _AttributesToGet_ without specifying any value for _Select_.
	/// If you query a local secondary index and request only attributes that are
	/// projected into that index, the operation will read only the index and not the
	/// table. If any of the requested attributes are not projected into the local
	/// secondary index, DynamoDB will fetch each of these attributes from the parent
	/// table. This extra fetching incurs additional throughput cost and latency.
	/// If you query a global secondary index, you can only request attributes that
	/// are projected into the index. Global secondary index queries cannot fetch
	/// attributes from the parent table.
	/// If neither _Select_ nor _AttributesToGet_ are specified, DynamoDB defaults to
	/// `ALL_ATTRIBUTES` when accessing a table, and `ALL_PROJECTED_ATTRIBUTES` when
	/// accessing an index. You cannot use both _Select_ and _AttributesToGet_
	/// together in a single request, unless the value for _Select_ is
	/// `SPECIFIC_ATTRIBUTES`. (This usage is equivalent to specifying
	/// _AttributesToGet_ without any value for _Select_.)
	/// If you use the _ProjectionExpression_ parameter, then the value for _Select_
	/// can only be `SPECIFIC_ATTRIBUTES`. Any other value for _Select_ will return an
	/// error.
	pub Select: Option<Select>,
}

pub type ExpressionAttributeValueMap = HashMap<ExpressionAttributeValueVariable,AttributeValue>;
/// For the _UpdateItem_ operation, represents the attributes to be modified, the
/// action to perform on each, and the new value for each.
/// You cannot use _UpdateItem_ to update any primary key attributes. Instead, you
/// will need to delete the item, and then use _PutItem_ to create a new item with
/// new attributes.
/// Attribute values cannot be null; string and binary type attributes must have
/// lengths greater than zero; and set type attributes must not be empty. Requests
/// with empty values will be rejected with a _ValidationException_ exception.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct AttributeValueUpdate {
	/// Specifies how to perform the update. Valid values are `PUT` (default),
	/// `DELETE`, and `ADD`. The behavior depends on whether the specified primary key
	/// already exists in the table.
	/// **If an item with the specified _Key_ is found in the table:**
	///   * `PUT` \- Adds the specified attribute to the item. If the attribute already exists, it is replaced by the new value. 
	///   * `DELETE` \- If no value is specified, the attribute and its value are removed from the item. The data type of the specified value must match the existing value's data type.
	/// If a _set_ of values is specified, then those values are subtracted from the
	/// old set. For example, if the attribute value was the set `[a,b,c]` and the
	/// _DELETE_ action specified `[a,c]`, then the final attribute value would be
	/// `[b]`. Specifying an empty set is an error.
	///   * `ADD` \- If the attribute does not already exist, then the attribute and its values are added to the item. If the attribute does exist, then the behavior of `ADD` depends on the data type of the attribute:
	///     * If the existing attribute is a number, and if _Value_ is also a number, then the _Value_ is mathematically added to the existing attribute. If _Value_ is a negative number, then it is subtracted from the existing attribute.
	/// If you use `ADD` to increment or decrement a number value for an item that
	/// doesn't exist before the update, DynamoDB uses 0 as the initial value.
	/// In addition, if you use `ADD` to update an existing item, and intend to
	/// increment or decrement an attribute value which does not yet exist, DynamoDB
	/// uses `0` as the initial value. For example, suppose that the item you want to
	/// update does not yet have an attribute named _itemcount_, but you decide to
	/// `ADD` the number `3` to this attribute anyway, even though it currently does
	/// not exist. DynamoDB will create the _itemcount_ attribute, set its initial
	/// value to `0`, and finally add `3` to it. The result will be a new _itemcount_
	/// attribute in the item, with a value of `3`.
	///     * If the existing data type is a set, and if the _Value_ is also a set, then the _Value_ is added to the existing set. (This is a _set_ operation, not mathematical addition.) For example, if the attribute value was the set `[1,2]`, and the `ADD` action specified `[3]`, then the final attribute value would be `[1,2,3]`. An error occurs if an Add action is specified for a set attribute and the attribute type specified does not match the existing set type. 
	/// Both sets must have the same primitive data type. For example, if the existing
	/// data type is a set of strings, the _Value_ must also be a set of strings. The
	/// same holds true for number sets and binary sets.
	/// This action is only valid for an existing attribute whose data type is number
	/// or is a set. Do not use `ADD` for any other data types.
	/// **If no item with the specified _Key_ is found:**
	///   * `PUT` \- DynamoDB creates a new item with the specified primary key, and then adds the attribute. 
	///   * `DELETE` \- Nothing happens; there is no attribute to delete.
	///   * `ADD` \- DynamoDB creates an item with the supplied primary key and number (or set of numbers) for the attribute value. The only data types allowed are number and number set; no other data types can be specified.
	pub Action: Option<AttributeAction>,
	pub Value: Option<AttributeValue>,
}

pub type Date = f64;
pub type Integer = i32;
/// Represents the output of a _DeleteItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteItemOutput {
	/// A map of attribute names to _AttributeValue_ objects, representing the item as
	/// it appeared before the _DeleteItem_ operation. This map appears in the
	/// response only if _ReturnValues_ was specified as `ALL_OLD` in the request.
	pub Attributes: Option<AttributeMap>,
	/// Information about item collections, if any, that were affected by the
	/// operation. _ItemCollectionMetrics_ is only returned if the request asked for
	/// it. If the table does not have any local secondary indexes, this information
	/// is not returned in the response.
	/// Each _ItemCollectionMetrics_ element consists of:
	///   * _ItemCollectionKey_ \- The hash key value of the item collection. This is the same as the hash key of the item.
	///   * _SizeEstimateRange_ \- An estimate of item collection size, in gigabytes. This value is a two-element array containing a lower bound and an upper bound for the estimate. The estimate includes the size of all the items in the table, plus the size of all attributes projected into all of the local secondary indexes on that table. Use this estimate to measure whether a local secondary index is approaching its size limit.
	/// The estimate is subject to change over time; therefore, do not rely on the
	/// precision or accuracy of the estimate.
	pub ItemCollectionMetrics: Option<ItemCollectionMetrics>,
	pub ConsumedCapacity: Option<ConsumedCapacity>,
}

/// Represents the selection criteria for a _Query_ or _Scan_ operation:
///   * For a _Query_ operation, _Condition_ is used for specifying the _KeyConditions_ to use when querying a table or an index. For _KeyConditions_, only the following comparison operators are supported:
/// `EQ | LE | LT | GE | GT | BEGINS_WITH | BETWEEN`
/// _Condition_ is also used in a _QueryFilter_, which evaluates the query results
/// and returns only the desired values.
///   * For a _Scan_ operation, _Condition_ is used in a _ScanFilter_, which evaluates the scan results and returns only the desired values.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Condition {
	/// A comparator for evaluating attributes. For example, equals, greater than,
	/// less than, etc.
	/// The following comparison operators are available:
	/// `EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS |
	/// BEGINS_WITH | IN | BETWEEN`
	/// The following are descriptions of each comparison operator.
	///   * `EQ` : Equal. `EQ` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, Binary, String Set, Number Set, or Binary Set. If an item
	/// contains an _AttributeValue_ element of a different type than the one provided
	/// in the request, the value does not match. For example, `{"S":"6"}` does not
	/// equal `{"N":"6"}`. Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///   * `NE` : Not equal. `NE` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, Binary, String Set, Number Set, or Binary Set. If an item contains an
	/// _AttributeValue_ of a different type than the one provided in the request, the
	/// value does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`.
	/// Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///   * `LE` : Less than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///   * `LT` : Less than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, or Binary (not a set type). If an item contains an _AttributeValue_
	/// element of a different type than the one provided in the request, the value
	/// does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`. Also,
	/// `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///   * `GE` : Greater than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///   * `GT` : Greater than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///   * `NOT_NULL` : The attribute exists. `NOT_NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the existence of an attribute, not its data type. If
	/// the data type of attribute "`a`" is null, and you evaluate it using
	/// `NOT_NULL`, the result is a Boolean _true_. This result is because the
	/// attribute "`a`" exists; its data type is not relevant to the `NOT_NULL`
	/// comparison operator.
	///   * `NULL` : The attribute does not exist. `NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the nonexistence of an attribute, not its data type.
	/// If the data type of attribute "`a`" is null, and you evaluate it using `NULL`,
	/// the result is a Boolean _false_. This is because the attribute "`a`" exists;
	/// its data type is not relevant to the `NULL` comparison operator.
	///   * `CONTAINS` : Checks for a subsequence, or value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is of type String, then the operator checks for a substring match.
	/// If the target attribute of the comparison is of type Binary, then the operator
	/// looks for a subsequence of the target that matches the input. If the target
	/// attribute of the comparison is a set ("`SS`", "`NS`", or "`BS`"), then the
	/// operator evaluates to true if it finds an exact match with any member of the
	/// set.
	/// CONTAINS is supported for lists: When evaluating "`a CONTAINS b`", "`a`" can
	/// be a list; however, "`b`" cannot be a set, a map, or a list.
	///   * `NOT_CONTAINS` : Checks for absence of a subsequence, or absence of a value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is a String, then the operator checks for the absence of a
	/// substring match. If the target attribute of the comparison is Binary, then the
	/// operator checks for the absence of a subsequence of the target that matches
	/// the input. If the target attribute of the comparison is a set ("`SS`", "`NS`",
	/// or "`BS`"), then the operator evaluates to true if it _does not_ find an exact
	/// match with any member of the set.
	/// NOT_CONTAINS is supported for lists: When evaluating "`a NOT CONTAINS b`",
	/// "`a`" can be a list; however, "`b`" cannot be a set, a map, or a list.
	///   * `BEGINS_WITH` : Checks for a prefix. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String or
	/// Binary (not a Number or a set type). The target attribute of the comparison
	/// must be of type String or Binary (not a Number or a set type).
	///   * `IN` : Checks for matching elements within two sets.
	/// _AttributeValueList_ can contain one or more _AttributeValue_ elements of type
	/// String, Number, or Binary (not a set type). These attributes are compared
	/// against an existing set type attribute of an item. If any elements of the
	/// input set are present in the item attribute, the expression evaluates to true.
	///   * `BETWEEN` : Greater than or equal to the first value, and less than or equal to the second value. 
	/// _AttributeValueList_ must contain two _AttributeValue_ elements of the same
	/// type, either String, Number, or Binary (not a set type). A target attribute
	/// matches if the target value is greater than, or equal to, the first element
	/// and less than, or equal to, the second element. If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not compare
	/// to `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`
	/// For usage examples of _AttributeValueList_ and _ComparisonOperator_, see
	/// [Legacy Conditional Parameters](http://docs.aws.amazon.com/amazondynamodb/late
	/// st/developerguide/LegacyConditionalParameters.html) in the _Amazon DynamoDB
	/// Developer Guide_.
	pub ComparisonOperator: ComparisonOperator,
	/// One or more values to evaluate against the supplied attribute. The number of
	/// values in the list depends on the _ComparisonOperator_ being used.
	/// For type Number, value comparisons are numeric.
	/// String value comparisons for greater than, equals, or less than are based on
	/// ASCII character code values. For example, `a` is greater than `A`, and `a` is
	/// greater than `B`. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	/// For Binary, DynamoDB treats each byte of the binary data as unsigned when it
	/// compares binary values.
	pub AttributeValueList: Option<AttributeValueList>,
}

pub type TableNameList = Vec<TableName>;
/// Represents the input of a _Scan_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ScanInput {
	/// A string that contains conditions that DynamoDB applies after the _Scan_
	/// operation, but before the data is returned to you. Items that do not satisfy
	/// the _FilterExpression_ criteria are not returned.
	/// A _FilterExpression_ is applied after the items have already been read; the
	/// process of filtering does not consume any additional read capacity units.
	/// For more information, see [Filter Expressions](http://docs.aws.amazon.com/amaz
	/// ondynamodb/latest/developerguide/QueryAndScan.html#FilteringResults) in the
	/// _Amazon DynamoDB Developer Guide_.
	/// _FilterExpression_ replaces the legacy _ScanFilter_ and _ConditionalOperator_
	/// parameters.
	pub FilterExpression: Option<ConditionExpression>,
	/// A Boolean value that determines the read consistency model during the scan:
	///   * If _ConsistentRead_ is `false`, then _Scan_ will use eventually consistent reads. The data returned from _Scan_ might not contain the results of other recently completed write operations (PutItem, UpdateItem or DeleteItem). The _Scan_ response might include some stale data.
	///   * If _ConsistentRead_ is `true`, then _Scan_ will use strongly consistent reads. All of the write operations that completed before the _Scan_ began are guaranteed to be contained in the _Scan_ response.
	/// The default setting for _ConsistentRead_ is `false`, meaning that eventually
	/// consistent reads will be used.
	/// Strongly consistent reads are not supported on global secondary indexes. If
	/// you scan a global secondary index with _ConsistentRead_ set to true, you will
	/// receive a _ValidationException_.
	pub ConsistentRead: Option<ConsistentRead>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _FilterExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A logical operator to apply to the conditions in a _ScanFilter_ map:
	///   * `AND` \- If all of the conditions evaluate to true, then the entire map evaluates to true.
	///   * `OR` \- If at least one of the conditions evaluate to true, then the entire map evaluates to true.
	/// If you omit _ConditionalOperator_, then `AND` is the default.
	/// The operation will succeed only if the entire map evaluates to true.
	/// This parameter does not support attributes of type List or Map.
	pub ConditionalOperator: Option<ConditionalOperator>,
	/// The name of a secondary index to scan. This index can be any local secondary
	/// index or global secondary index. Note that if you use the `IndexName`
	/// parameter, you must also provide `TableName`.
	pub IndexName: Option<IndexName>,
	/// A string that identifies one or more attributes to retrieve from the specified
	/// table or index. These attributes can include scalars, sets, or elements of a
	/// JSON document. The attributes in the expression must be separated by commas.
	/// If no attribute names are specified, then all attributes will be returned. If
	/// any of the requested attributes are not found, they will not appear in the
	/// result.
	/// For more information, see [Accessing Item Attributes](http://docs.aws.amazon.c
	/// om/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.ht
	/// ml) in the _Amazon DynamoDB Developer Guide_.
	/// _ProjectionExpression_ replaces the legacy _AttributesToGet_ parameter.
	pub ProjectionExpression: Option<ProjectionExpression>,
	/// One or more substitution tokens for attribute names in an expression. The
	/// following are some use cases for using _ExpressionAttributeNames_:
	///   * To access an attribute whose name conflicts with a DynamoDB reserved word.
	///   * To create a placeholder for repeating occurrences of an attribute name in an expression.
	///   * To prevent special characters in an attribute name from being misinterpreted in an expression.
	/// Use the **#** character in an expression to dereference an attribute name. For
	/// example, consider the following attribute name:
	///   * `Percentile`
	/// The name of this attribute conflicts with a reserved word, so it cannot be
	/// used directly in an expression. (For the complete list of reserved words, see
	/// [Reserved Words](http://docs.aws.amazon.com/amazondynamodb/latest/developergui
	/// de/ReservedWords.html) in the _Amazon DynamoDB Developer Guide_). To work
	/// around this, you could specify the following for _ExpressionAttributeNames_:
	///   * `{"#P":"Percentile"}`
	/// You could then use this substitution in an expression, as in this example:
	///   * `#P = :val`
	/// Tokens that begin with the **:** character are _expression attribute values_,
	/// which are placeholders for the actual value at runtime.
	/// For more information on expression attribute names, see [Accessing Item Attrib
	/// utes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressi
	/// ons.AccessingItemAttributes.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeNames: Option<ExpressionAttributeNameMap>,
	/// The name of the table containing the requested items; or, if you provide
	/// `IndexName`, the name of the table to which that index belongs.
	pub TableName: TableName,
	pub ReturnConsumedCapacity: Option<ReturnConsumedCapacity>,
	/// The primary key of the first item that this operation will evaluate. Use the
	/// value that was returned for _LastEvaluatedKey_ in the previous operation.
	/// The data type for _ExclusiveStartKey_ must be String, Number or Binary. No set
	/// data types are allowed.
	/// In a parallel scan, a _Scan_ request that includes _ExclusiveStartKey_ must
	/// specify the same segment whose previous _Scan_ returned the corresponding
	/// value of _LastEvaluatedKey_.
	pub ExclusiveStartKey: Option<Key>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ProjectionExpression_ instead. Do not combine legacy parameters
	/// and expression parameters in a single API call; otherwise, DynamoDB will
	/// return a _ValidationException_ exception.
	/// This parameter allows you to retrieve attributes of type List or Map; however,
	/// it cannot retrieve individual elements within a List or a Map.
	/// The names of one or more attributes to retrieve. If no attribute names are
	/// provided, then all attributes will be returned. If any of the requested
	/// attributes are not found, they will not appear in the result.
	/// Note that _AttributesToGet_ has no effect on provisioned throughput
	/// consumption. DynamoDB determines capacity units consumed based on item size,
	/// not on the amount of data that is returned to an application.
	pub AttributesToGet: Option<AttributeNameList>,
	/// The maximum number of items to evaluate (not necessarily the number of
	/// matching items). If DynamoDB processes the number of items up to the limit
	/// while processing the results, it stops the operation and returns the matching
	/// values up to that point, and a key in _LastEvaluatedKey_ to apply in a
	/// subsequent operation, so that you can pick up where you left off. Also, if the
	/// processed data set size exceeds 1 MB before DynamoDB reaches this limit, it
	/// stops the operation and returns the matching values up to the limit, and a key
	/// in _LastEvaluatedKey_ to apply in a subsequent operation to continue the
	/// operation. For more information, see [Query and Scan](http://docs.aws.amazon.c
	/// om/amazondynamodb/latest/developerguide/QueryAndScan.html) in the _Amazon
	/// DynamoDB Developer Guide_.
	pub Limit: Option<PositiveIntegerObject>,
	/// For a parallel _Scan_ request, _TotalSegments_ represents the total number of
	/// segments into which the _Scan_ operation will be divided. The value of
	/// _TotalSegments_ corresponds to the number of application workers that will
	/// perform the parallel scan. For example, if you want to use four application
	/// threads to scan a table or an index, specify a _TotalSegments_ value of 4.
	/// The value for _TotalSegments_ must be greater than or equal to 1, and less
	/// than or equal to 1000000. If you specify a _TotalSegments_ value of 1, the
	/// _Scan_ operation will be sequential rather than parallel.
	/// If you specify _TotalSegments_, you must also specify _Segment_.
	pub TotalSegments: Option<ScanTotalSegments>,
	/// For a parallel _Scan_ request, _Segment_ identifies an individual segment to
	/// be scanned by an application worker.
	/// Segment IDs are zero-based, so the first segment is always 0. For example, if
	/// you want to use four application threads to scan a table or an index, then the
	/// first thread specifies a _Segment_ value of 0, the second thread specifies 1,
	/// and so on.
	/// The value of _LastEvaluatedKey_ returned from a parallel _Scan_ request must
	/// be used as _ExclusiveStartKey_ with the same segment ID in a subsequent _Scan_
	/// operation.
	/// The value for _Segment_ must be greater than or equal to 0, and less than the
	/// value provided for _TotalSegments_.
	/// If you provide _Segment_, you must also provide _TotalSegments_.
	pub Segment: Option<ScanSegment>,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _FilterExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A condition that evaluates the scan results and returns only the desired
	/// values.
	/// This parameter does not support attributes of type List or Map.
	/// If you specify more than one condition in the _ScanFilter_ map, then by
	/// default all of the conditions must evaluate to true. In other words, the
	/// conditions are ANDed together. (You can use the _ConditionalOperator_
	/// parameter to OR the conditions instead. If you do this, then at least one of
	/// the conditions must evaluate to true, rather than all of them.)
	/// Each _ScanFilter_ element consists of an attribute name to compare, along with
	/// the following:
	///   * _AttributeValueList_ \- One or more values to evaluate against the supplied attribute. The number of values in the list depends on the operator specified in _ComparisonOperator_ .
	/// For type Number, value comparisons are numeric.
	/// String value comparisons for greater than, equals, or less than are based on
	/// ASCII character code values. For example, `a` is greater than `A`, and `a` is
	/// greater than `B`. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	/// For Binary, DynamoDB treats each byte of the binary data as unsigned when it
	/// compares binary values.
	/// For information on specifying data types in JSON, see [JSON Data Format](http:
	/// //docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataFormat.html) in
	/// the _Amazon DynamoDB Developer Guide_.
	///   * _ComparisonOperator_ \- A comparator for evaluating attributes. For example, equals, greater than, less than, etc.
	/// The following comparison operators are available:
	/// `EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS |
	/// BEGINS_WITH | IN | BETWEEN`
	/// For complete descriptions of all comparison operators, see [Condition](http://
	/// docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Condition.html).
	pub ScanFilter: Option<FilterConditionMap>,
	/// The attributes to be returned in the result. You can retrieve all item
	/// attributes, specific item attributes, or the count of matching items.
	///   * `ALL_ATTRIBUTES` \- Returns all of the item attributes.
	///   * `COUNT` \- Returns the number of matching items, rather than the matching items themselves.
	///   * `SPECIFIC_ATTRIBUTES` \- Returns only the attributes listed in _AttributesToGet_. This return value is equivalent to specifying _AttributesToGet_ without specifying any value for _Select_.
	/// If neither _Select_ nor _AttributesToGet_ are specified, DynamoDB defaults to
	/// `ALL_ATTRIBUTES`. You cannot use both _AttributesToGet_ and _Select_ together
	/// in a single request, unless the value for _Select_ is `SPECIFIC_ATTRIBUTES`.
	/// (This usage is equivalent to specifying _AttributesToGet_ without any value
	/// for _Select_.)
	pub Select: Option<Select>,
	/// One or more values that can be substituted in an expression.
	/// Use the **:** (colon) character in an expression to dereference an attribute
	/// value. For example, suppose that you wanted to check whether the value of the
	/// _ProductStatus_ attribute was one of the following:
	/// `Available | Backordered | Discontinued`
	/// You would first need to specify _ExpressionAttributeValues_ as follows:
	/// `{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"},
	/// ":disc":{"S":"Discontinued"} }`
	/// You could then use these values in an expression, such as this:
	/// `ProductStatus IN (:avail, :back, :disc)`
	/// For more information on expression attribute values, see [Specifying Condition
	/// s](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions
	/// .SpecifyingConditions.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeValues: Option<ExpressionAttributeValueMap>,
}

/// Determines the level of detail about provisioned throughput consumption that
/// is returned in the response:
///   * _INDEXES_ \- The response includes the aggregate _ConsumedCapacity_ for the operation, together with _ConsumedCapacity_ for each table and secondary index that was accessed.
/// Note that some operations, such as _GetItem_ and _BatchGetItem_, do not access
/// any indexes at all. In these cases, specifying _INDEXES_ will only return
/// _ConsumedCapacity_ information for table(s).
///   * _TOTAL_ \- The response includes only the aggregate _ConsumedCapacity_ for the operation.
///   * _NONE_ \- No _ConsumedCapacity_ details are included in the response.
pub type ReturnConsumedCapacity = String;
pub type Backfilling = bool;
pub type BooleanObject = bool;
/// Represents the output of a _BatchGetItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct BatchGetItemOutput {
	/// A map of tables and their respective keys that were not processed with the
	/// current response. The _UnprocessedKeys_ value is in the same form as
	/// _RequestItems_, so the value can be provided directly to a subsequent
	/// _BatchGetItem_ operation. For more information, see _RequestItems_ in the
	/// Request Parameters section.
	/// Each element consists of:
	///   * _Keys_ \- An array of primary key attribute values that define specific items in the table.
	///   * _AttributesToGet_ \- One or more attributes to be retrieved from the table or index. By default, all attributes are returned. If a requested attribute is not found, it does not appear in the result.
	///   * _ConsistentRead_ \- The consistency of a read operation. If set to `true`, then a strongly consistent read is used; otherwise, an eventually consistent read is used.
	/// If there are no unprocessed keys remaining, the response contains an empty
	/// _UnprocessedKeys_ map.
	pub UnprocessedKeys: Option<BatchGetRequestMap>,
	/// A map of table name to a list of items. Each object in _Responses_ consists of
	/// a table name, along with a map of attribute data consisting of the data type
	/// and attribute value.
	pub Responses: Option<BatchGetResponseMap>,
	/// The read capacity units consumed by the operation.
	/// Each element consists of:
	///   * _TableName_ \- The table that consumed the provisioned throughput.
	///   * _CapacityUnits_ \- The total number of capacity units consumed.
	pub ConsumedCapacity: Option<ConsumedCapacityMultiple>,
}

/// Represents attributes that are copied (projected) from the table into an
/// index. These are in addition to the primary key attributes and index key
/// attributes, which are automatically projected.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Projection {
	/// The set of attributes that are projected into the index:
	///   * `KEYS_ONLY` \- Only the index and primary keys are projected into the index.
	///   * `INCLUDE` \- Only the specified table attributes are projected into the index. The list of projected attributes are in _NonKeyAttributes_.
	///   * `ALL` \- All of the table attributes are projected into the index.
	pub ProjectionType: Option<ProjectionType>,
	/// Represents the non-key attribute names which will be projected into the index.
	/// For local secondary indexes, the total count of _NonKeyAttributes_ summed
	/// across all of the local secondary indexes, must not exceed 20. If you project
	/// the same attribute into two different indexes, this counts as two distinct
	/// attributes when determining the total.
	pub NonKeyAttributes: Option<NonKeyAttributeNameList>,
}

pub type BatchWriteItemRequestMap = HashMap<TableName,WriteRequests>;
pub type MapAttributeValue = HashMap<AttributeName,AttributeValue>;
/// Represents the output of a _DescribeTable_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DescribeTableOutput {
	pub Table: Option<TableDescription>,
}

pub type AttributeName = String;
pub type ComparisonOperator = String;
pub type TableName = String;
pub type AttributeNameList = Vec<AttributeName>;
pub type KeyList = Vec<Key>;
pub type ItemCollectionMetricsPerTable = HashMap<TableName,ItemCollectionMetricsMultiple>;
pub type SecondaryIndexesCapacityMap = HashMap<IndexName,Capacity>;
/// Represents the input of a _DeleteTable_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteTableInput {
	/// The name of the table to delete.
	pub TableName: TableName,
}

/// Represents the provisioned throughput settings for a specified table or index.
/// The settings can be modified using the _UpdateTable_ operation.
/// For current minimum and maximum provisioned throughput values, see [Limits](ht
/// tp://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html) in
/// the _Amazon DynamoDB Developer Guide_.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ProvisionedThroughput {
	/// The maximum number of writes consumed per second before DynamoDB returns a
	/// _ThrottlingException_. For more information, see [Specifying Read and Write Re
	/// quirements](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Wo
	/// rkingWithTables.html#ProvisionedThroughput) in the _Amazon DynamoDB Developer
	/// Guide_.
	pub WriteCapacityUnits: PositiveLongObject,
	/// The maximum number of strongly consistent reads consumed per second before
	/// DynamoDB returns a _ThrottlingException_. For more information, see
	/// [Specifying Read and Write Requirements](http://docs.aws.amazon.com/amazondyna
	/// modb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput) in
	/// the _Amazon DynamoDB Developer Guide_.
	pub ReadCapacityUnits: PositiveLongObject,
}

pub type ConditionExpression = String;
/// Represents a condition to be compared with an attribute value. This condition
/// can be used with _DeleteItem_, _PutItem_ or _UpdateItem_ operations; if the
/// comparison evaluates to true, the operation succeeds; if not, the operation
/// fails. You can use _ExpectedAttributeValue_ in one of two different ways:
///   * Use _AttributeValueList_ to specify one or more values to compare against an attribute. Use _ComparisonOperator_ to specify how you want to perform the comparison. If the comparison evaluates to true, then the conditional operation succeeds.
///   * Use _Value_ to specify a value that DynamoDB will compare against an attribute. If the values match, then _ExpectedAttributeValue_ evaluates to true and the conditional operation succeeds. Optionally, you can also set _Exists_ to false, indicating that you _do not_ expect to find the attribute value in the table. In this case, the conditional operation succeeds only if the comparison evaluates to false.
/// _Value_ and _Exists_ are incompatible with _AttributeValueList_ and
/// _ComparisonOperator_. Note that if you use both sets of parameters at once,
/// DynamoDB will return a _ValidationException_ exception.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ExpectedAttributeValue {
	/// A comparator for evaluating attributes in the _AttributeValueList_. For
	/// example, equals, greater than, less than, etc.
	/// The following comparison operators are available:
	/// `EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS |
	/// BEGINS_WITH | IN | BETWEEN`
	/// The following are descriptions of each comparison operator.
	///   * `EQ` : Equal. `EQ` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, Binary, String Set, Number Set, or Binary Set. If an item
	/// contains an _AttributeValue_ element of a different type than the one provided
	/// in the request, the value does not match. For example, `{"S":"6"}` does not
	/// equal `{"N":"6"}`. Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///   * `NE` : Not equal. `NE` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, Binary, String Set, Number Set, or Binary Set. If an item contains an
	/// _AttributeValue_ of a different type than the one provided in the request, the
	/// value does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`.
	/// Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///   * `LE` : Less than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///   * `LT` : Less than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, or Binary (not a set type). If an item contains an _AttributeValue_
	/// element of a different type than the one provided in the request, the value
	/// does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`. Also,
	/// `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///   * `GE` : Greater than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///   * `GT` : Greater than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///   * `NOT_NULL` : The attribute exists. `NOT_NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the existence of an attribute, not its data type. If
	/// the data type of attribute "`a`" is null, and you evaluate it using
	/// `NOT_NULL`, the result is a Boolean _true_. This result is because the
	/// attribute "`a`" exists; its data type is not relevant to the `NOT_NULL`
	/// comparison operator.
	///   * `NULL` : The attribute does not exist. `NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the nonexistence of an attribute, not its data type.
	/// If the data type of attribute "`a`" is null, and you evaluate it using `NULL`,
	/// the result is a Boolean _false_. This is because the attribute "`a`" exists;
	/// its data type is not relevant to the `NULL` comparison operator.
	///   * `CONTAINS` : Checks for a subsequence, or value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is of type String, then the operator checks for a substring match.
	/// If the target attribute of the comparison is of type Binary, then the operator
	/// looks for a subsequence of the target that matches the input. If the target
	/// attribute of the comparison is a set ("`SS`", "`NS`", or "`BS`"), then the
	/// operator evaluates to true if it finds an exact match with any member of the
	/// set.
	/// CONTAINS is supported for lists: When evaluating "`a CONTAINS b`", "`a`" can
	/// be a list; however, "`b`" cannot be a set, a map, or a list.
	///   * `NOT_CONTAINS` : Checks for absence of a subsequence, or absence of a value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is a String, then the operator checks for the absence of a
	/// substring match. If the target attribute of the comparison is Binary, then the
	/// operator checks for the absence of a subsequence of the target that matches
	/// the input. If the target attribute of the comparison is a set ("`SS`", "`NS`",
	/// or "`BS`"), then the operator evaluates to true if it _does not_ find an exact
	/// match with any member of the set.
	/// NOT_CONTAINS is supported for lists: When evaluating "`a NOT CONTAINS b`",
	/// "`a`" can be a list; however, "`b`" cannot be a set, a map, or a list.
	///   * `BEGINS_WITH` : Checks for a prefix. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String or
	/// Binary (not a Number or a set type). The target attribute of the comparison
	/// must be of type String or Binary (not a Number or a set type).
	///   * `IN` : Checks for matching elements within two sets.
	/// _AttributeValueList_ can contain one or more _AttributeValue_ elements of type
	/// String, Number, or Binary (not a set type). These attributes are compared
	/// against an existing set type attribute of an item. If any elements of the
	/// input set are present in the item attribute, the expression evaluates to true.
	///   * `BETWEEN` : Greater than or equal to the first value, and less than or equal to the second value. 
	/// _AttributeValueList_ must contain two _AttributeValue_ elements of the same
	/// type, either String, Number, or Binary (not a set type). A target attribute
	/// matches if the target value is greater than, or equal to, the first element
	/// and less than, or equal to, the second element. If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not compare
	/// to `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`
	pub ComparisonOperator: Option<ComparisonOperator>,
	/// Causes DynamoDB to evaluate the value before attempting a conditional
	/// operation:
	///   * If _Exists_ is `true`, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the operation succeeds. If it is not found, the operation fails with a _ConditionalCheckFailedException_.
	///   * If _Exists_ is `false`, DynamoDB assumes that the attribute value does not exist in the table. If in fact the value does not exist, then the assumption is valid and the operation succeeds. If the value is found, despite the assumption that it does not exist, the operation fails with a _ConditionalCheckFailedException_.
	/// The default setting for _Exists_ is `true`. If you supply a _Value_ all by
	/// itself, DynamoDB assumes the attribute exists: You don't have to set _Exists_
	/// to `true`, because it is implied.
	/// DynamoDB returns a _ValidationException_ if:
	///   * _Exists_ is `true` but there is no _Value_ to check. (You expect a value to exist, but don't specify what that value is.)
	///   * _Exists_ is `false` but you also provide a _Value_. (You cannot expect an attribute to have a value, while also expecting it not to exist.)
	pub Exists: Option<BooleanObject>,
	pub Value: Option<AttributeValue>,
	/// One or more values to evaluate against the supplied attribute. The number of
	/// values in the list depends on the _ComparisonOperator_ being used.
	/// For type Number, value comparisons are numeric.
	/// String value comparisons for greater than, equals, or less than are based on
	/// ASCII character code values. For example, `a` is greater than `A`, and `a` is
	/// greater than `B`. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	/// For Binary, DynamoDB treats each byte of the binary data as unsigned when it
	/// compares binary values.
	/// For information on specifying data types in JSON, see [JSON Data Format](http:
	/// //docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataFormat.html) in
	/// the _Amazon DynamoDB Developer Guide_.
	pub AttributeValueList: Option<AttributeValueList>,
}

/// Represents the input of a _DeleteItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteItemInput {
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ConditionExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A logical operator to apply to the conditions in the _Expected_ map:
	///   * `AND` \- If all of the conditions evaluate to true, then the entire map evaluates to true.
	///   * `OR` \- If at least one of the conditions evaluate to true, then the entire map evaluates to true.
	/// If you omit _ConditionalOperator_, then `AND` is the default.
	/// The operation will succeed only if the entire map evaluates to true.
	/// This parameter does not support attributes of type List or Map.
	pub ConditionalOperator: Option<ConditionalOperator>,
	/// One or more substitution tokens for attribute names in an expression. The
	/// following are some use cases for using _ExpressionAttributeNames_:
	///   * To access an attribute whose name conflicts with a DynamoDB reserved word.
	///   * To create a placeholder for repeating occurrences of an attribute name in an expression.
	///   * To prevent special characters in an attribute name from being misinterpreted in an expression.
	/// Use the **#** character in an expression to dereference an attribute name. For
	/// example, consider the following attribute name:
	///   * `Percentile`
	/// The name of this attribute conflicts with a reserved word, so it cannot be
	/// used directly in an expression. (For the complete list of reserved words, see
	/// [Reserved Words](http://docs.aws.amazon.com/amazondynamodb/latest/developergui
	/// de/ReservedWords.html) in the _Amazon DynamoDB Developer Guide_). To work
	/// around this, you could specify the following for _ExpressionAttributeNames_:
	///   * `{"#P":"Percentile"}`
	/// You could then use this substitution in an expression, as in this example:
	///   * `#P = :val`
	/// Tokens that begin with the **:** character are _expression attribute values_,
	/// which are placeholders for the actual value at runtime.
	/// For more information on expression attribute names, see [Accessing Item Attrib
	/// utes](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressi
	/// ons.AccessingItemAttributes.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeNames: Option<ExpressionAttributeNameMap>,
	/// Use _ReturnValues_ if you want to get the item attributes as they appeared
	/// before they were deleted. For _DeleteItem_, the valid values are:
	///   * `NONE` \- If _ReturnValues_ is not specified, or if its value is `NONE`, then nothing is returned. (This setting is the default for _ReturnValues_.)
	///   * `ALL_OLD` \- The content of the old item is returned.
	pub ReturnValues: Option<ReturnValue>,
	/// A condition that must be satisfied in order for a conditional _DeleteItem_ to
	/// succeed.
	/// An expression can contain any of the following:
	///   * Functions: `attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size`
	/// These function names are case-sensitive.
	///   * Comparison operators: ` = | <> | < | > | <= | >= | BETWEEN | IN`
	///   * Logical operators: `AND | OR | NOT`
	/// For more information on condition expressions, see [Specifying Conditions](htt
	/// p://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Speci
	/// fyingConditions.html) in the _Amazon DynamoDB Developer Guide_.
	/// _ConditionExpression_ replaces the legacy _ConditionalOperator_ and _Expected_
	/// parameters.
	pub ConditionExpression: Option<ConditionExpression>,
	/// The name of the table from which to delete the item.
	pub TableName: TableName,
	/// Determines whether item collection metrics are returned. If set to `SIZE`, the
	/// response includes statistics about item collections, if any, that were
	/// modified during the operation are returned in the response. If set to `NONE`
	/// (the default), no statistics are returned.
	pub ReturnItemCollectionMetrics: Option<ReturnItemCollectionMetrics>,
	pub ReturnConsumedCapacity: Option<ReturnConsumedCapacity>,
	/// One or more values that can be substituted in an expression.
	/// Use the **:** (colon) character in an expression to dereference an attribute
	/// value. For example, suppose that you wanted to check whether the value of the
	/// _ProductStatus_ attribute was one of the following:
	/// `Available | Backordered | Discontinued`
	/// You would first need to specify _ExpressionAttributeValues_ as follows:
	/// `{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"},
	/// ":disc":{"S":"Discontinued"} }`
	/// You could then use these values in an expression, such as this:
	/// `ProductStatus IN (:avail, :back, :disc)`
	/// For more information on expression attribute values, see [Specifying Condition
	/// s](http://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions
	/// .SpecifyingConditions.html) in the _Amazon DynamoDB Developer Guide_.
	pub ExpressionAttributeValues: Option<ExpressionAttributeValueMap>,
	/// A map of attribute names to _AttributeValue_ objects, representing the primary
	/// key of the item to delete.
	/// For the primary key, you must provide all of the attributes. For example, with
	/// a hash type primary key, you only need to provide the hash attribute. For a
	/// hash-and-range type primary key, you must provide both the hash attribute and
	/// the range attribute.
	pub Key: Key,
	/// This is a legacy parameter, for backward compatibility. New applications
	/// should use _ConditionExpression_ instead. Do not combine legacy parameters and
	/// expression parameters in a single API call; otherwise, DynamoDB will return a
	/// _ValidationException_ exception.
	/// A map of attribute/condition pairs. _Expected_ provides a conditional block
	/// for the _DeleteItem_ operation.
	/// Each element of _Expected_ consists of an attribute name, a comparison
	/// operator, and one or more values. DynamoDB compares the attribute with the
	/// value(s) you supplied, using the comparison operator. For each _Expected_
	/// element, the result of the evaluation is either true or false.
	/// If you specify more than one element in the _Expected_ map, then by default
	/// all of the conditions must evaluate to true. In other words, the conditions
	/// are ANDed together. (You can use the _ConditionalOperator_ parameter to OR the
	/// conditions instead. If you do this, then at least one of the conditions must
	/// evaluate to true, rather than all of them.)
	/// If the _Expected_ map evaluates to true, then the conditional operation
	/// succeeds; otherwise, it fails.
	/// _Expected_ contains the following:
	///   * _AttributeValueList_ \- One or more values to evaluate against the supplied attribute. The number of values in the list depends on the _ComparisonOperator_ being used.
	/// For type Number, value comparisons are numeric.
	/// String value comparisons for greater than, equals, or less than are based on
	/// ASCII character code values. For example, `a` is greater than `A`, and `a` is
	/// greater than `B`. For a list of code values, see
	/// <http://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters>.
	/// For type Binary, DynamoDB treats each byte of the binary data as unsigned when
	/// it compares binary values.
	///   * _ComparisonOperator_ \- A comparator for evaluating attributes in the _AttributeValueList_. When performing the comparison, DynamoDB uses strongly consistent reads.
	/// The following comparison operators are available:
	/// `EQ | NE | LE | LT | GE | GT | NOT_NULL | NULL | CONTAINS | NOT_CONTAINS |
	/// BEGINS_WITH | IN | BETWEEN`
	/// The following are descriptions of each comparison operator.
	///     * `EQ` : Equal. `EQ` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, Binary, String Set, Number Set, or Binary Set. If an item
	/// contains an _AttributeValue_ element of a different type than the one provided
	/// in the request, the value does not match. For example, `{"S":"6"}` does not
	/// equal `{"N":"6"}`. Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///     * `NE` : Not equal. `NE` is supported for all datatypes, including lists and maps.
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, Binary, String Set, Number Set, or Binary Set. If an item contains an
	/// _AttributeValue_ of a different type than the one provided in the request, the
	/// value does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`.
	/// Also, `{"N":"6"}` does not equal `{"NS":["6", "2", "1"]}`.
	///     * `LE` : Less than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `LT` : Less than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String,
	/// Number, or Binary (not a set type). If an item contains an _AttributeValue_
	/// element of a different type than the one provided in the request, the value
	/// does not match. For example, `{"S":"6"}` does not equal `{"N":"6"}`. Also,
	/// `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `GE` : Greater than or equal. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `GT` : Greater than. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not equal
	/// `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`.
	///     * `NOT_NULL` : The attribute exists. `NOT_NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the existence of an attribute, not its data type. If
	/// the data type of attribute "`a`" is null, and you evaluate it using
	/// `NOT_NULL`, the result is a Boolean _true_. This result is because the
	/// attribute "`a`" exists; its data type is not relevant to the `NOT_NULL`
	/// comparison operator.
	///     * `NULL` : The attribute does not exist. `NULL` is supported for all datatypes, including lists and maps.
	/// This operator tests for the nonexistence of an attribute, not its data type.
	/// If the data type of attribute "`a`" is null, and you evaluate it using `NULL`,
	/// the result is a Boolean _false_. This is because the attribute "`a`" exists;
	/// its data type is not relevant to the `NULL` comparison operator.
	///     * `CONTAINS` : Checks for a subsequence, or value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is of type String, then the operator checks for a substring match.
	/// If the target attribute of the comparison is of type Binary, then the operator
	/// looks for a subsequence of the target that matches the input. If the target
	/// attribute of the comparison is a set ("`SS`", "`NS`", or "`BS`"), then the
	/// operator evaluates to true if it finds an exact match with any member of the
	/// set.
	/// CONTAINS is supported for lists: When evaluating "`a CONTAINS b`", "`a`" can
	/// be a list; however, "`b`" cannot be a set, a map, or a list.
	///     * `NOT_CONTAINS` : Checks for absence of a subsequence, or absence of a value in a set.
	/// _AttributeValueList_ can contain only one _AttributeValue_ element of type
	/// String, Number, or Binary (not a set type). If the target attribute of the
	/// comparison is a String, then the operator checks for the absence of a
	/// substring match. If the target attribute of the comparison is Binary, then the
	/// operator checks for the absence of a subsequence of the target that matches
	/// the input. If the target attribute of the comparison is a set ("`SS`", "`NS`",
	/// or "`BS`"), then the operator evaluates to true if it _does not_ find an exact
	/// match with any member of the set.
	/// NOT_CONTAINS is supported for lists: When evaluating "`a NOT CONTAINS b`",
	/// "`a`" can be a list; however, "`b`" cannot be a set, a map, or a list.
	///     * `BEGINS_WITH` : Checks for a prefix. 
	/// _AttributeValueList_ can contain only one _AttributeValue_ of type String or
	/// Binary (not a Number or a set type). The target attribute of the comparison
	/// must be of type String or Binary (not a Number or a set type).
	///     * `IN` : Checks for matching elements within two sets.
	/// _AttributeValueList_ can contain one or more _AttributeValue_ elements of type
	/// String, Number, or Binary (not a set type). These attributes are compared
	/// against an existing set type attribute of an item. If any elements of the
	/// input set are present in the item attribute, the expression evaluates to true.
	///     * `BETWEEN` : Greater than or equal to the first value, and less than or equal to the second value. 
	/// _AttributeValueList_ must contain two _AttributeValue_ elements of the same
	/// type, either String, Number, or Binary (not a set type). A target attribute
	/// matches if the target value is greater than, or equal to, the first element
	/// and less than, or equal to, the second element. If an item contains an
	/// _AttributeValue_ element of a different type than the one provided in the
	/// request, the value does not match. For example, `{"S":"6"}` does not compare
	/// to `{"N":"6"}`. Also, `{"N":"6"}` does not compare to `{"NS":["6", "2", "1"]}`
	/// For usage examples of _AttributeValueList_ and _ComparisonOperator_, see
	/// [Legacy Conditional Parameters](http://docs.aws.amazon.com/amazondynamodb/late
	/// st/developerguide/LegacyConditionalParameters.html) in the _Amazon DynamoDB
	/// Developer Guide_.
	/// For backward compatibility with previous DynamoDB releases, the following
	/// parameters can be used instead of _AttributeValueList_ and
	/// _ComparisonOperator_:
	///   * _Value_ \- A value for DynamoDB to compare with an attribute.
	///   * _Exists_ \- A Boolean value that causes DynamoDB to evaluate the value before attempting the conditional operation:
	///     * If _Exists_ is `true`, DynamoDB will check to see if that attribute value already exists in the table. If it is found, then the condition evaluates to true; otherwise the condition evaluate to false.
	///     * If _Exists_ is `false`, DynamoDB assumes that the attribute value does _not_ exist in the table. If in fact the value does not exist, then the assumption is valid and the condition evaluates to true. If the value is found, despite the assumption that it does not exist, the condition evaluates to false.
	/// Note that the default value for _Exists_ is `true`.
	/// The _Value_ and _Exists_ parameters are incompatible with _AttributeValueList_
	/// and _ComparisonOperator_. Note that if you use both sets of parameters at
	/// once, DynamoDB will return a _ValidationException_ exception.
	/// This parameter does not support attributes of type List or Map.
	pub Expected: Option<ExpectedAttributeMap>,
}

pub type StreamArn = String;
/// Represents a new global secondary index to be added to an existing table.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateGlobalSecondaryIndexAction {
	/// The key schema for the global secondary index.
	pub KeySchema: KeySchema,
	/// The name of the global secondary index to be created.
	pub IndexName: IndexName,
	pub Projection: Projection,
	pub ProvisionedThroughput: ProvisionedThroughput,
}

pub type BinaryAttributeValue = Vec<u8>;
/// Represents the output of a _Scan_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ScanOutput {
	/// The number of items in the response.
	/// If you set _ScanFilter_ in the request, then _Count_ is the number of items
	/// returned after the filter was applied, and _ScannedCount_ is the number of
	/// matching items before the filter was applied.
	/// If you did not use a filter in the request, then _Count_ is the same as
	/// _ScannedCount_.
	pub Count: Option<Integer>,
	/// An array of item attributes that match the scan criteria. Each element in this
	/// array consists of an attribute name and the value for that attribute.
	pub Items: Option<ItemList>,
	/// The primary key of the item where the operation stopped, inclusive of the
	/// previous result set. Use this value to start a new operation, excluding this
	/// value in the new request.
	/// If _LastEvaluatedKey_ is empty, then the "last page" of results has been
	/// processed and there is no more data to be retrieved.
	/// If _LastEvaluatedKey_ is not empty, it does not necessarily mean that there is
	/// more data in the result set. The only way to know when you have reached the
	/// end of the result set is when _LastEvaluatedKey_ is empty.
	pub LastEvaluatedKey: Option<Key>,
	/// The number of items evaluated, before any _ScanFilter_ is applied. A high
	/// _ScannedCount_ value with few, or no, _Count_ results indicates an inefficient
	/// _Scan_ operation. For more information, see [Count and ScannedCount](http://do
	/// cs.aws.amazon.com/amazondynamodb/latest/developerguide/QueryAndScan.html#Count
	/// ) in the _Amazon DynamoDB Developer Guide_.
	/// If you did not use a filter in the request, then _ScannedCount_ is the same as
	/// _Count_.
	pub ScannedCount: Option<Integer>,
	pub ConsumedCapacity: Option<ConsumedCapacity>,
}

/// Represents the output of an _UpdateTable_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct UpdateTableOutput {
	pub TableDescription: Option<TableDescription>,
}

/// Represents the output of an _UpdateItem_ operation.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct UpdateItemOutput {
	/// A map of attribute values as they appeared before the _UpdateItem_ operation.
	/// This map only appears if _ReturnValues_ was specified as something other than
	/// `NONE` in the request. Each element represents one attribute.
	pub Attributes: Option<AttributeMap>,
	pub ItemCollectionMetrics: Option<ItemCollectionMetrics>,
	pub ConsumedCapacity: Option<ConsumedCapacity>,
}

/// Represents the properties of a local secondary index.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct LocalSecondaryIndexDescription {
	/// The total size of the specified index, in bytes. DynamoDB updates this value
	/// approximately every six hours. Recent changes might not be reflected in this
	/// value.
	pub IndexSizeBytes: Option<Long>,
	/// Represents the name of the local secondary index.
	pub IndexName: Option<IndexName>,
	pub Projection: Option<Projection>,
	/// The Amazon Resource Name (ARN) that uniquely identifies the index.
	pub IndexArn: Option<String>,
	/// The complete index key schema, which consists of one or more pairs of
	/// attribute names and key types (`HASH` or `RANGE`).
	pub KeySchema: Option<KeySchema>,
	/// The number of items in the specified index. DynamoDB updates this value
	/// approximately every six hours. Recent changes might not be reflected in this
	/// value.
	pub ItemCount: Option<Long>,
}

pub type WriteRequests = Vec<WriteRequest>;
pub type GlobalSecondaryIndexList = Vec<GlobalSecondaryIndex>;
pub type TableStatus = String;
pub type KeyConditions = HashMap<AttributeName,Condition>;
/// Represents a request to perform a _PutItem_ operation on an item.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutRequest {
	/// A map of attribute name to attribute values, representing the primary key of
	/// an item to be processed by _PutItem_. All of the table's primary key
	/// attributes must be specified, and their data types must match those of the
	/// table's key schema. If any attributes are present in the item which are part
	/// of an index key schema for the table, their types must match the index key
	/// schema.
	pub Item: PutItemInputAttributeMap,
}

pub type ConsistentRead = bool;
pub type ExpectedAttributeMap = HashMap<AttributeName,ExpectedAttributeValue>;
pub type ExpressionAttributeValueVariable = String;
pub type ConsumedCapacityMultiple = Vec<ConsumedCapacity>;
pub type PutItemInputAttributeMap = HashMap<AttributeName,AttributeValue>;
pub type ItemList = Vec<AttributeMap>;
/// The capacity units consumed by an operation. The data returned includes the
/// total provisioned throughput consumed, along with statistics for the table and
/// any indexes involved in the operation. _ConsumedCapacity_ is only returned if
/// the request asked for it. For more information, see [Provisioned Throughput](h
/// ttp://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThro
/// ughputIntro.html) in the _Amazon DynamoDB Developer Guide_.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ConsumedCapacity {
	/// The total number of capacity units consumed by the operation.
	pub CapacityUnits: Option<ConsumedCapacityUnits>,
	/// The amount of throughput consumed on each global index affected by the
	/// operation.
	pub GlobalSecondaryIndexes: Option<SecondaryIndexesCapacityMap>,
	/// The name of the table that was affected by the operation.
	pub TableName: Option<TableName>,
	/// The amount of throughput consumed on each local index affected by the
	/// operation.
	pub LocalSecondaryIndexes: Option<SecondaryIndexesCapacityMap>,
	/// The amount of throughput consumed on the table affected by the operation.
	pub Table: Option<Capacity>,
}

/// An item collection is too large. This exception is only returned for tables
/// that have one or more local secondary indexes.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ItemCollectionSizeLimitExceededException {
	/// The total size of an item collection has exceeded the maximum limit of 10
	/// gigabytes.
	pub message: Option<ErrorMessage>,
}

/// Represents a request to perform a _DeleteItem_ operation on an item.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteRequest {
	/// A map of attribute name to attribute values, representing the primary key of
	/// the item to delete. All of the table's primary key attributes must be
	/// specified, and their data types must match those of the table's key schema.
	pub Key: Key,
}

pub type BooleanAttributeValue = bool;
pub type IndexName = String;
pub type ProjectionExpression = String;
pub type AttributeDefinitions = Vec<AttributeDefinition>;
pub type PositiveIntegerObject = i32;
/// Represents a global secondary index to be deleted from an existing table.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteGlobalSecondaryIndexAction {
	/// The name of the global secondary index to be deleted.
	pub IndexName: IndexName,
}

/// Represents the properties of a table.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct TableDescription {
	/// The Amazon Resource Name (ARN) that uniquely identifies the table.
	pub TableArn: Option<String>,
	/// Represents one or more local secondary indexes on the table. Each index is
	/// scoped to a given hash key value. Tables with one or more local secondary
	/// indexes are subject to an item collection size limit, where the amount of data
	/// within a given item collection cannot exceed 10 GB. Each element is composed
	/// of:
	///   * _IndexName_ \- The name of the local secondary index.
	///   * _KeySchema_ \- Specifies the complete index key schema. The attribute names in the key schema must be between 1 and 255 characters (inclusive). The key schema must begin with the same hash key attribute as the table.
	///   * _Projection_ \- Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:
	///     * _ProjectionType_ \- One of the following:
	///       * `KEYS_ONLY` \- Only the index and primary keys are projected into the index.
	///       * `INCLUDE` \- Only the specified table attributes are projected into the index. The list of projected attributes are in _NonKeyAttributes_.
	///       * `ALL` \- All of the table attributes are projected into the index.
	///     * _NonKeyAttributes_ \- A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in _NonKeyAttributes_, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.
	///   * _IndexSizeBytes_ \- Represents the total size of the index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.
	///   * _ItemCount_ \- Represents the number of items in the index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value.
	/// If the table is in the `DELETING` state, no information about indexes will be
	/// returned.
	pub LocalSecondaryIndexes: Option<LocalSecondaryIndexDescriptionList>,
	/// An array of _AttributeDefinition_ objects. Each of these objects describes one
	/// attribute in the table and index key schema.
	/// Each _AttributeDefinition_ object in this array is composed of:
	///   * _AttributeName_ \- The name of the attribute.
	///   * _AttributeType_ \- The data type for the attribute.
	pub AttributeDefinitions: Option<AttributeDefinitions>,
	/// The global secondary indexes, if any, on the table. Each index is scoped to a
	/// given hash key value. Each element is composed of:
	///   * _Backfilling_ \- If true, then the index is currently in the backfilling phase. Backfilling occurs only when a new global secondary index is added to the table; it is the process by which DynamoDB populates the new index with data from the table. (This attribute does not appear for indexes that were created during a _CreateTable_ operation.)
	///   * _IndexName_ \- The name of the global secondary index.
	///   * _IndexSizeBytes_ \- The total size of the global secondary index, in bytes. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value. 
	///   * _IndexStatus_ \- The current status of the global secondary index:
	///     * _CREATING_ \- The index is being created.
	///     * _UPDATING_ \- The index is being updated.
	///     * _DELETING_ \- The index is being deleted.
	///     * _ACTIVE_ \- The index is ready for use.
	///   * _ItemCount_ \- The number of items in the global secondary index. DynamoDB updates this value approximately every six hours. Recent changes might not be reflected in this value. 
	///   * _KeySchema_ \- Specifies the complete index key schema. The attribute names in the key schema must be between 1 and 255 characters (inclusive). The key schema must begin with the same hash key attribute as the table.
	///   * _Projection_ \- Specifies attributes that are copied (projected) from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected. Each attribute specification is composed of:
	///     * _ProjectionType_ \- One of the following:
	///       * `KEYS_ONLY` \- Only the index and primary keys are projected into the index.
	///       * `INCLUDE` \- Only the specified table attributes are projected into the index. The list of projected attributes are in _NonKeyAttributes_.
	///       * `ALL` \- All of the table attributes are projected into the index.
	///     * _NonKeyAttributes_ \- A list of one or more non-key attribute names that are projected into the secondary index. The total count of attributes provided in _NonKeyAttributes_, summed across all of the secondary indexes, must not exceed 20. If you project the same attribute into two different indexes, this counts as two distinct attributes when determining the total.
	///   * _ProvisionedThroughput_ \- The provisioned throughput settings for the global secondary index, consisting of read and write capacity units, along with data about increases and decreases. 
	/// If the table is in the `DELETING` state, no information about indexes will be
	/// returned.
	pub GlobalSecondaryIndexes: Option<GlobalSecondaryIndexDescriptionList>,
	/// The provisioned throughput settings for the table, consisting of read and
	/// write capacity units, along with data about increases and decreases.
	pub ProvisionedThroughput: Option<ProvisionedThroughputDescription>,
	/// The total size of the specified table, in bytes. DynamoDB updates this value
	/// approximately every six hours. Recent changes might not be reflected in this
	/// value.
	pub TableSizeBytes: Option<Long>,
	/// The name of the table.
	pub TableName: Option<TableName>,
	/// The current state of the table:
	///   * _CREATING_ \- The table is being created.
	///   * _UPDATING_ \- The table is being updated.
	///   * _DELETING_ \- The table is being deleted.
	///   * _ACTIVE_ \- The table is ready for use.
	pub TableStatus: Option<TableStatus>,
	/// The current DynamoDB Streams configuration for the table.
	pub StreamSpecification: Option<StreamSpecification>,
	/// A timestamp, in ISO 8601 format, for this stream.
	/// Note that _LatestStreamLabel_ is not a unique identifier for the stream,
	/// because it is possible that a stream from another table might have the same
	/// timestamp. However, the combination of the following three elements is
	/// guaranteed to be unique:
	///   * the AWS customer ID.
	///   * the table name.
	///   * the _StreamLabel_.
	pub LatestStreamLabel: Option<String>,
	/// The primary key structure for the table. Each _KeySchemaElement_ consists of:
	///   * _AttributeName_ \- The name of the attribute.
	///   * _KeyType_ \- The key type for the attribute. Can be either `HASH` or `RANGE`.
	/// For more information about primary keys, see [Primary Key](http://docs.aws.ama
	/// zon.com/amazondynamodb/latest/developerguide/DataModel.html#DataModelPrimaryKe
	/// y) in the _Amazon DynamoDB Developer Guide_.
	pub KeySchema: Option<KeySchema>,
	/// The number of items in the specified table. DynamoDB updates this value
	/// approximately every six hours. Recent changes might not be reflected in this
	/// value.
	pub ItemCount: Option<Long>,
	/// The date and time when the table was created, in [UNIX epoch
	/// time](http://www.epochconverter.com/) format.
	pub CreationDateTime: Option<Date>,
	/// The Amazon Resource Name (ARN) that uniquely identifies the latest stream for
	/// this table.
	pub LatestStreamArn: Option<StreamArn>,
}

pub type ItemCollectionSizeEstimateBound = f64;
/// Represents an operation to perform - either _DeleteItem_ or _PutItem_. You can
/// only request one of these operations, not both, in a single _WriteRequest_. If
/// you do need to perform both of these operations, you will need to provide two
/// separate _WriteRequest_ objects.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct WriteRequest {
	/// A request to perform a _PutItem_ operation.
	pub PutRequest: Option<PutRequest>,
	/// A request to perform a _DeleteItem_ operation.
	pub DeleteRequest: Option<DeleteRequest>,
}

pub struct DynamoDBClient<'a> {
	creds: Box<AWSCredentialsProvider + 'a>,
	region: &'a Region
}

impl<'a> DynamoDBClient<'a> { 
	pub fn new<P: AWSCredentialsProvider + 'a>(creds: P, region: &'a Region) -> DynamoDBClient<'a> {
		DynamoDBClient { creds: Box::new(creds), region: region }
	}
	/// Modifies the provisioned throughput settings, global secondary indexes, or
	/// DynamoDB Streams settings for a given table.
	/// You can only perform one of the following operations at once:
	///   * Modify the provisioned throughput settings of the table.
	///   * Enable or disable Streams on the table.
	///   * Remove a global secondary index from the table.
	///   * Create a new global secondary index on the table. Once the index begins backfilling, you can use _UpdateTable_ to perform other operations.
	/// _UpdateTable_ is an asynchronous operation; while it is executing, the table
	/// status changes from `ACTIVE` to `UPDATING`. While it is `UPDATING`, you cannot
	/// issue another _UpdateTable_ request. When the table returns to the `ACTIVE`
	/// state, the _UpdateTable_ operation is complete.
	pub fn update_table(&mut self, input: &UpdateTableInput) -> Result<UpdateTableOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.UpdateTable");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: UpdateTableOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// The _DeleteTable_ operation deletes a table and all of its items. After a
	/// _DeleteTable_ request, the specified table is in the `DELETING` state until
	/// DynamoDB completes the deletion. If the table is in the `ACTIVE` state, you
	/// can delete it. If a table is in `CREATING` or `UPDATING` states, then DynamoDB
	/// returns a _ResourceInUseException_. If the specified table does not exist,
	/// DynamoDB returns a _ResourceNotFoundException_. If table is already in the
	/// `DELETING` state, no error is returned.
	/// DynamoDB might continue to accept data read and write operations, such as
	/// _GetItem_ and _PutItem_, on a table in the `DELETING` state until the table
	/// deletion is complete.
	/// When you delete a table, any indexes on that table are also deleted.
	/// If you have DynamoDB Streams enabled on the table, then the corresponding
	/// stream on that table goes into the `DISABLED` state, and the stream is
	/// automatically deleted after 24 hours.
	/// Use the _DescribeTable_ API to check the status of the table.
	pub fn delete_table(&mut self, input: &DeleteTableInput) -> Result<DeleteTableOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.DeleteTable");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: DeleteTableOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// The _BatchGetItem_ operation returns the attributes of one or more items from
	/// one or more tables. You identify requested items by primary key.
	/// A single operation can retrieve up to 16 MB of data, which can contain as many
	/// as 100 items. _BatchGetItem_ will return a partial result if the response size
	/// limit is exceeded, the table's provisioned throughput is exceeded, or an
	/// internal processing failure occurs. If a partial result is returned, the
	/// operation returns a value for _UnprocessedKeys_. You can use this value to
	/// retry the operation starting with the next item to get.
	/// If you request more than 100 items _BatchGetItem_ will return a
	/// _ValidationException_ with the message "Too many items requested for the
	/// BatchGetItem call".
	/// For example, if you ask to retrieve 100 items, but each individual item is 300
	/// KB in size, the system returns 52 items (so as not to exceed the 16 MB limit).
	/// It also returns an appropriate _UnprocessedKeys_ value so you can get the next
	/// page of results. If desired, your application can include its own logic to
	/// assemble the pages of results into one data set.
	/// If _none_ of the items can be processed due to insufficient provisioned
	/// throughput on all of the tables in the request, then _BatchGetItem_ will
	/// return a _ProvisionedThroughputExceededException_. If _at least one_ of the
	/// items is successfully processed, then _BatchGetItem_ completes successfully,
	/// while returning the keys of the unread items in _UnprocessedKeys_.
	/// If DynamoDB returns any unprocessed items, you should retry the batch
	/// operation on those items. However, _we strongly recommend that you use an
	/// exponential backoff algorithm_. If you retry the batch operation immediately,
	/// the underlying read or write requests can still fail due to throttling on the
	/// individual tables. If you delay the batch operation using exponential backoff,
	/// the individual requests in the batch are much more likely to succeed.
	/// For more information, see [Batch Operations and Error Handling](http://docs.aw
	/// s.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOper
	/// ations) in the _Amazon DynamoDB Developer Guide_.
	/// By default, _BatchGetItem_ performs eventually consistent reads on every table
	/// in the request. If you want strongly consistent reads instead, you can set
	/// _ConsistentRead_ to `true` for any or all tables.
	/// In order to minimize response latency, _BatchGetItem_ retrieves items in
	/// parallel.
	/// When designing your application, keep in mind that DynamoDB does not return
	/// attributes in any particular order. To help parse the response by item,
	/// include the primary key values for the items in your request in the
	/// _AttributesToGet_ parameter.
	/// If a requested item does not exist, it is not returned in the result. Requests
	/// for nonexistent items consume the minimum read capacity units according to the
	/// type of read. For more information, see [Capacity Units Calculations](http://d
	/// ocs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html
	/// #CapacityUnitCalculations) in the _Amazon DynamoDB Developer Guide_.
	pub fn batch_get_item(&mut self, input: &BatchGetItemInput) -> Result<BatchGetItemOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.BatchGetItem");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: BatchGetItemOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// The _Scan_ operation returns one or more items and item attributes by
	/// accessing every item in a table or a secondary index. To have DynamoDB return
	/// fewer items, you can provide a _ScanFilter_ operation.
	/// If the total number of scanned items exceeds the maximum data set size limit
	/// of 1 MB, the scan stops and results are returned to the user as a
	/// _LastEvaluatedKey_ value to continue the scan in a subsequent operation. The
	/// results also include the number of items exceeding the limit. A scan can
	/// result in no table data meeting the filter criteria.
	/// By default, _Scan_ operations proceed sequentially; however, for faster
	/// performance on a large table or secondary index, applications can request a
	/// parallel _Scan_ operation by providing the _Segment_ and _TotalSegments_
	/// parameters. For more information, see [Parallel Scan](http://docs.aws.amazon.c
	/// om/amazondynamodb/latest/developerguide/QueryAndScan.html#QueryAndScanParallel
	/// Scan) in the _Amazon DynamoDB Developer Guide_.
	/// By default, _Scan_ uses eventually consistent reads when acessing the data in
	/// the table or local secondary index. However, you can use strongly consistent
	/// reads instead by setting the _ConsistentRead_ parameter to _true_.
	pub fn scan(&mut self, input: &ScanInput) -> Result<ScanOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.Scan");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ScanOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Returns information about the table, including the current status of the
	/// table, when it was created, the primary key schema, and any indexes on the
	/// table.
	/// If you issue a DescribeTable request immediately after a CreateTable request,
	/// DynamoDB might return a ResourceNotFoundException. This is because
	/// DescribeTable uses an eventually consistent query, and the metadata for your
	/// table might not be available at that moment. Wait for a few seconds, and then
	/// try the DescribeTable request again.
	pub fn describe_table(&mut self, input: &DescribeTableInput) -> Result<DescribeTableOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.DescribeTable");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: DescribeTableOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// The _BatchWriteItem_ operation puts or deletes multiple items in one or more
	/// tables. A single call to _BatchWriteItem_ can write up to 16 MB of data, which
	/// can comprise as many as 25 put or delete requests. Individual items to be
	/// written can be as large as 400 KB.
	/// _BatchWriteItem_ cannot update items. To update items, use the _UpdateItem_
	/// API.
	/// The individual _PutItem_ and _DeleteItem_ operations specified in
	/// _BatchWriteItem_ are atomic; however _BatchWriteItem_ as a whole is not. If
	/// any requested operations fail because the table's provisioned throughput is
	/// exceeded or an internal processing failure occurs, the failed operations are
	/// returned in the _UnprocessedItems_ response parameter. You can investigate and
	/// optionally resend the requests. Typically, you would call _BatchWriteItem_ in
	/// a loop. Each iteration would check for unprocessed items and submit a new
	/// _BatchWriteItem_ request with those unprocessed items until all items have
	/// been processed.
	/// Note that if _none_ of the items can be processed due to insufficient
	/// provisioned throughput on all of the tables in the request, then
	/// _BatchWriteItem_ will return a _ProvisionedThroughputExceededException_.
	/// If DynamoDB returns any unprocessed items, you should retry the batch
	/// operation on those items. However, _we strongly recommend that you use an
	/// exponential backoff algorithm_. If you retry the batch operation immediately,
	/// the underlying read or write requests can still fail due to throttling on the
	/// individual tables. If you delay the batch operation using exponential backoff,
	/// the individual requests in the batch are much more likely to succeed.
	/// For more information, see [Batch Operations and Error Handling](http://docs.aw
	/// s.amazon.com/amazondynamodb/latest/developerguide/ErrorHandling.html#BatchOper
	/// ations) in the _Amazon DynamoDB Developer Guide_.
	/// With _BatchWriteItem_, you can efficiently write or delete large amounts of
	/// data, such as from Amazon Elastic MapReduce (EMR), or copy data from another
	/// database into DynamoDB. In order to improve performance with these large-scale
	/// operations, _BatchWriteItem_ does not behave in the same way as individual
	/// _PutItem_ and _DeleteItem_ calls would. For example, you cannot specify
	/// conditions on individual put and delete requests, and _BatchWriteItem_ does
	/// not return deleted items in the response.
	/// If you use a programming language that supports concurrency, you can use
	/// threads to write items in parallel. Your application must include the
	/// necessary logic to manage the threads. With languages that don't support
	/// threading, you must update or delete the specified items one at a time. In
	/// both situations, _BatchWriteItem_ provides an alternative where the API
	/// performs the specified put and delete operations in parallel, giving you the
	/// power of the thread pool approach without having to introduce complexity into
	/// your application.
	/// Parallel processing reduces latency, but each specified put and delete request
	/// consumes the same number of write capacity units whether it is processed in
	/// parallel or not. Delete operations on nonexistent items consume one write
	/// capacity unit.
	/// If one or more of the following is true, DynamoDB rejects the entire batch
	/// write operation:
	///   * One or more tables specified in the _BatchWriteItem_ request does not exist.
	///   * Primary key attributes specified on an item in the request do not match those in the corresponding table's primary key schema.
	///   * You try to perform multiple operations on the same item in the same _BatchWriteItem_ request. For example, you cannot put and delete the same item in the same _BatchWriteItem_ request. 
	///   * There are more than 25 requests in the batch.
	///   * Any individual item in a batch exceeds 400 KB.
	///   * The total request size exceeds 16 MB.
	pub fn batch_write_item(&mut self, input: &BatchWriteItemInput) -> Result<BatchWriteItemOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.BatchWriteItem");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: BatchWriteItemOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// The _CreateTable_ operation adds a new table to your account. In an AWS
	/// account, table names must be unique within each region. That is, you can have
	/// two tables with same name if you create the tables in different regions.
	/// _CreateTable_ is an asynchronous operation. Upon receiving a _CreateTable_
	/// request, DynamoDB immediately returns a response with a _TableStatus_ of
	/// `CREATING`. After the table is created, DynamoDB sets the _TableStatus_ to
	/// `ACTIVE`. You can perform read and write operations only on an `ACTIVE` table.
	/// You can optionally define secondary indexes on the new table, as part of the
	/// _CreateTable_ operation. If you want to create multiple tables with secondary
	/// indexes on them, you must create the tables sequentially. Only one table with
	/// secondary indexes can be in the `CREATING` state at any given time.
	/// You can use the _DescribeTable_ API to check the table status.
	pub fn create_table(&mut self, input: &CreateTableInput) -> Result<CreateTableOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.CreateTable");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: CreateTableOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Returns an array of table names associated with the current account and
	/// endpoint. The output from _ListTables_ is paginated, with each page returning
	/// a maximum of 100 table names.
	pub fn list_tables(&mut self, input: &ListTablesInput) -> Result<ListTablesOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.ListTables");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ListTablesOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// The _GetItem_ operation returns a set of attributes for the item with the
	/// given primary key. If there is no matching item, _GetItem_ does not return any
	/// data.
	/// _GetItem_ provides an eventually consistent read by default. If your
	/// application requires a strongly consistent read, set _ConsistentRead_ to
	/// `true`. Although a strongly consistent read might take more time than an
	/// eventually consistent read, it always returns the last updated value.
	pub fn get_item(&mut self, input: &GetItemInput) -> Result<GetItemOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.GetItem");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: GetItemOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// A _Query_ operation uses the primary key of a table or a secondary index to
	/// directly access items from that table or index.
	/// Use the _KeyConditionExpression_ parameter to provide a specific hash key
	/// value. The _Query_ operation will return all of the items from the table or
	/// index with that hash key value. You can optionally narrow the scope of the
	/// _Query_ operation by specifying a range key value and a comparison operator in
	/// _KeyConditionExpression_. You can use the _ScanIndexForward_ parameter to get
	/// results in forward or reverse order, by range key or by index key.
	/// Queries that do not return results consume the minimum number of read capacity
	/// units for that type of read operation.
	/// If the total number of items meeting the query criteria exceeds the result set
	/// size limit of 1 MB, the query stops and results are returned to the user with
	/// the _LastEvaluatedKey_ element to continue the query in a subsequent
	/// operation. Unlike a _Scan_ operation, a _Query_ operation never returns both
	/// an empty result set and a _LastEvaluatedKey_ value. _LastEvaluatedKey_ is only
	/// provided if the results exceed 1 MB, or if you have used the _Limit_
	/// parameter.
	/// You can query a table, a local secondary index, or a global secondary index.
	/// For a query on a table or on a local secondary index, you can set the
	/// _ConsistentRead_ parameter to `true` and obtain a strongly consistent result.
	/// Global secondary indexes support eventually consistent reads only, so do not
	/// specify _ConsistentRead_ when querying a global secondary index.
	pub fn query(&mut self, input: &QueryInput) -> Result<QueryOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.Query");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: QueryOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Creates a new item, or replaces an old item with a new item. If an item that
	/// has the same primary key as the new item already exists in the specified
	/// table, the new item completely replaces the existing item. You can perform a
	/// conditional put operation (add a new item if one with the specified primary
	/// key doesn't exist), or replace an existing item if it has certain attribute
	/// values.
	/// In addition to putting an item, you can also return the item's attribute
	/// values in the same operation, using the _ReturnValues_ parameter.
	/// When you add an item, the primary key attribute(s) are the only required
	/// attributes. Attribute values cannot be null. String and Binary type attributes
	/// must have lengths greater than zero. Set type attributes cannot be empty.
	/// Requests with empty values will be rejected with a _ValidationException_
	/// exception.
	/// You can request that _PutItem_ return either a copy of the original item
	/// (before the update) or a copy of the updated item (after the update). For more
	/// information, see the _ReturnValues_ description below.
	/// To prevent a new item from replacing an existing item, use a conditional put
	/// operation with _ComparisonOperator_ set to `NULL` for the primary key
	/// attribute, or attributes.
	/// For more information about using this API, see [Working with Items](http://doc
	/// s.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithItems.html)
	/// in the _Amazon DynamoDB Developer Guide_.
	pub fn put_item(&mut self, input: &PutItemInput) -> Result<PutItemOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.PutItem");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: PutItemOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Edits an existing item's attributes, or adds a new item to the table if it
	/// does not already exist. You can put, delete, or add attribute values. You can
	/// also perform a conditional update on an existing item (insert a new attribute
	/// name-value pair if it doesn't exist, or replace an existing name-value pair if
	/// it has certain expected attribute values). If conditions are specified and the
	/// item does not exist, then the operation fails and a new item is not created.
	/// You can also return the item's attribute values in the same _UpdateItem_
	/// operation using the _ReturnValues_ parameter.
	pub fn update_item(&mut self, input: &UpdateItemInput) -> Result<UpdateItemOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.UpdateItem");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: UpdateItemOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Deletes a single item in a table by primary key. You can perform a conditional
	/// delete operation that deletes the item if it exists, or if it has an expected
	/// attribute value.
	/// In addition to deleting an item, you can also return the item's attribute
	/// values in the same operation, using the _ReturnValues_ parameter.
	/// Unless you specify conditions, the _DeleteItem_ is an idempotent operation;
	/// running it multiple times on the same item or attribute does _not_ result in
	/// an error response.
	/// Conditional deletes are useful for deleting items only if specific conditions
	/// are met. If those conditions are met, DynamoDB performs the delete. Otherwise,
	/// the item is not deleted.
	pub fn delete_item(&mut self, input: &DeleteItemInput) -> Result<DeleteItemOutput> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "dynamodb", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "DynamoDB_20120810.DeleteItem");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: DeleteItemOutput = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
}

