//! DynamoDB helper types.

#![allow(unused_variables, unused_mut, non_snake_case)]
use std::str;

use rusoto_core::{Region, RusotoResult};
use rusoto_dynamodb::*;

// Make getting values out of nested Option objects less verbose.
macro_rules! try_opt {
    ($expr:expr) => (match $expr {
        ::std::option::Option::Some(ref val) => val,
        ::std::option::Option::None => return None
    })
}

pub struct DynamoDbHelper {
    client: DynamoDbClient,
}

impl DynamoDbHelper {
    pub fn new(region: Region) -> DynamoDbHelper {
        DynamoDbHelper { client: DynamoDbClient::new(region) }
    }

    pub async fn list_tables(&mut self) -> RusotoResult<ListTablesOutput, ListTablesError> {
        let mut req = ListTablesInput::default();
        self.client.list_tables(req).await
    }

    pub async fn create_table(&mut self, input: CreateTableInput) -> RusotoResult<CreateTableOutput, CreateTableError> {
        self.client.create_table(input).await
    }

    pub async fn describe_table(&mut self, name: &str) -> RusotoResult<DescribeTableOutput, DescribeTableError> {
        let mut input = DescribeTableInput::default();
        input.table_name = String::from(name);
        self.client.describe_table(input).await
    }

    pub async fn delete_table(&mut self, name: &str) -> RusotoResult<DeleteTableOutput, DeleteTableError> {
        let mut input = DeleteTableInput::default();
        input.table_name = String::from(name);
        self.client.delete_table(input).await
    }

    pub async fn put_item(&mut self, input: PutItemInput) -> RusotoResult<PutItemOutput, PutItemError> {
        self.client.put_item(input).await
    }

    pub async fn get_item(&mut self, input: GetItemInput) -> RusotoResult<GetItemOutput, GetItemError> {
        self.client.get_item(input).await
    }
}

pub trait PutItemInputHelper {
    fn new() -> PutItemInput;
}

impl PutItemInputHelper for PutItemInput {
    fn new() -> PutItemInput {
        PutItemInput::default()
    }
}

pub trait CreateTableInputHelper {
    fn new() -> CreateTableInput;
    fn with_name(self, table_name: &str) -> CreateTableInput;
    fn with_provisioned_capacity(self, write_capacity_units: i64, read_capacity_units: i64) -> CreateTableInput;
    fn with_attributes(self, attributes: Vec<AttributeDefinition>) -> CreateTableInput;
    fn with_key_schema(self, key_schema: Vec<KeySchemaElement>) -> CreateTableInput;
    fn add_attribute<N: Into<String>, T: Into<String>>(self,
                                                       name: N,
                                                       attr_type: T)
                                                       -> CreateTableInput;
}

impl CreateTableInputHelper for CreateTableInput {
    fn new() -> CreateTableInput {
        CreateTableInput::default()
    }

    fn with_name(mut self, table_name: &str) -> CreateTableInput {
        self.table_name = String::from(table_name);
        self
    }

    fn with_provisioned_capacity(mut self, write_capacity_units: i64, read_capacity_units: i64) -> CreateTableInput {
        self.provisioned_throughput = Some(ProvisionedThroughput{
            read_capacity_units,
            write_capacity_units,
        });
        self
    }

    fn with_attributes(mut self, attributes: Vec<AttributeDefinition>) -> CreateTableInput {
        self.attribute_definitions = attributes;
        self
    }

    fn with_key_schema(mut self, key_schema: Vec<KeySchemaElement>) -> CreateTableInput {
        self.key_schema = key_schema;
        self
    }

    fn add_attribute<N: Into<String>, T: Into<String>>(mut self,
                                                       name: N,
                                                       attr_type: T)
                                                       -> CreateTableInput {
        self.attribute_definitions.push(AttributeDefinition {
            attribute_name: name.into(),
            attribute_type: attr_type.into(),
        });
        self
    }
}

pub trait DescribeTableOutputHelper {
    fn get_status(&self) -> Option<String>;
}

impl DescribeTableOutputHelper for DescribeTableOutput {
    fn get_status(&self) -> Option<String> {
        let table = try_opt!(self.table);
        Some(try_opt!(table.table_status).to_string())
    }
}




#[macro_export]
macro_rules! attributes {
    ($($val:expr => $attr_type:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(AttributeDefinition { attribute_name: String::from($val), attribute_type: String::from($attr_type) });
            )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! key_schema {
    ($($name:expr => $key_type:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(KeySchemaElement { key_type: String::from($key_type), attribute_name: String::from($name) });
            )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! val {
	(B => $val:expr) => (
	    {
	    	let mut attr = AttributeValue::default();
	    	attr.b = Some($val);
	    	attr
	    }
	);
	(S => $val:expr) => (
	    {
			let mut attr = AttributeValue::default();
			attr.s = Some($val.to_string());
			attr
		}
	);
	(N => $val:expr) => (
	    {
	    	let mut attr = AttributeValue::default();
	    	attr.n = Some($val.to_string());
	    	attr
	    }
	);
}

// TODO: make a macro from this?
pub fn get_str_from_attribute(attr: &AttributeValue) -> Option<&str> {
    match attr.b {
        None => (),
        Some(ref blob_attribute) => return Some(str::from_utf8(blob_attribute).unwrap()),
    }

    match attr.s {
        None => (),
        Some(ref string_attribute) => return Some(string_attribute),
    }

    match attr.n {
        None => (),
        Some(ref number_attribute) => return Some(number_attribute),
    }

    return None;
}
