#![cfg(feature = "dynamodb")]

#[macro_use]
extern crate rusoto;
extern crate time;

use std::thread;

use time::get_time;

use rusoto::ChainProvider;
use rusoto::dynamodb::{
    AttributeDefinition,
    AttributeValue,
    CreateTableInput,
    DynamoDBError,
    DynamoDBHelper,
    GetItemInput,
    GetItemOutput,
    Key,
    KeySchemaElement,
    PutItemInput,
    PutItemInputAttributeMap,
    get_str_from_attribute,
};
use rusoto::Region;

#[test]
fn main() {
    let creds = ChainProvider::new().unwrap();
    let region = Region::UsWest2;

    let mut dynamodb = DynamoDBHelper::new(creds, &region);

    match dynamo_list_tables_tests(&mut dynamodb) {
        Ok(_) => {
            println!("List tables OK");
        }
        Err(err) => {
            println!("Error getting table list: {:#?}", err);
        }
    }

    let table_name = &format!("test_table_{}", get_time().sec);

    match dynamo_create_table_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Issued create table command for {}", table_name);
        }
        Err(err) => {
            println!("Error creating table {:#?}", err);
        }
    }

    match dynamo_describe_wait_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Table {} is now active", table_name);
        }
        Err(err) => {
            println!("Error waiting for table to become active {:#?}", err);
        }
    }

    let mut item = Key::default();
    item.insert("string".to_string(), val!(S => "foo"));
    item.insert("number".to_string(), val!(N => "1234"));

    match dynamo_get_item_test(&mut dynamodb, &table_name, item) {
        Ok(item_from_dynamo) => {
            println!("Got item back from Dynamo");
            match item_from_dynamo.Item {
                None => println!("nothing received from Dynamo, item may not exist"),
                Some(attributes_map) => {
                    for (column_name, value) in attributes_map {
                        println!("found column name '{}' with value of '{}'", column_name, get_str_from_attribute(&value).unwrap());
                    }
                },
            }
        },
        Err(err) => {
            println!("Error retrieving object: {:?}", err);
        }
    }

    match dynamo_put_item_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Put item to {}", table_name);
        }
        Err(err) => {
            println!("Error putting item to table {:#?}", err);
        }
    }

    println!("Trying the dynamo get again");
    item = Key::default();
    item.insert("string".to_string(), val!(S => "foo"));
    item.insert("number".to_string(), val!(N => "1234"));
    match dynamo_get_item_test(&mut dynamodb, &table_name, item) {
        Ok(item_from_dynamo) => {
            println!("Got item back from Dynamo");
            match item_from_dynamo.Item {
                None => println!("nothing received from Dynamo, item may not exist"),
                Some(attributes_map) => {
                    for (column_name, value) in attributes_map {
                        println!("found column name '{}' with value of '{}'", column_name, get_str_from_attribute(&value).unwrap());
                    }
                },
            }
        },
        Err(err) => {
            println!("Error retrieving object: {:?}", err);
        }
    }

    match dynamo_delete_table_test(&mut dynamodb, &table_name) {
        Ok(_) => {
            println!("Deleted table {}", table_name);
        }
        Err(err) => {
            println!("Error deleting DynamoDB table {:#?}", err);
        }
    }

}

fn dynamo_list_tables_tests(dynamodb: &mut DynamoDBHelper) -> Result<(), DynamoDBError> {
    let response = try!(dynamodb.list_tables());
    println!("{:#?}", response);
    Ok(())
}

fn dynamo_create_table_test(dynamodb: &mut DynamoDBHelper,
                            table_name: &str)
                            -> Result<(), DynamoDBError> {
    println!("Creating table {} ", table_name);

    let input = CreateTableInput::new()
                        .with_name(table_name)
                        .with_write_capacity(1)
                        .with_read_capacity(1)
                        .with_attributes(attributes!("string" => "S", "number" => "N"))
                        .with_key_schema(key_schema!("string" => "HASH", "number" => "RANGE"));

    let _result = try!(dynamodb.create_table(&input));
    Ok(())
}

fn dynamo_put_item_test(dynamodb: &mut DynamoDBHelper, table_name: &str) -> Result<(), DynamoDBError> {
    let mut input = PutItemInput::default();

    let mut item = PutItemInputAttributeMap::default();
    item.insert("string".to_string(), val!(S => "foo"));
    item.insert("number".to_string(), val!(N => "1234"));

    input.Item = item;
    input.TableName = table_name.to_string();

    try!(dynamodb.put_item(&input));

    Ok(())
}

fn dynamo_get_item_test(dynamodb: &mut DynamoDBHelper, table_name: &str, item_key: Key) -> Result<GetItemOutput, DynamoDBError> {
    let mut item_request = GetItemInput::default();
    item_request.Key = item_key;
    item_request.TableName = table_name.to_string();

    match dynamodb.get_item(&item_request) {
        Err(why) => Err(why),
        Ok(output) => Ok(output),
    }
}

fn dynamo_describe_wait_test(dynamodb: &mut DynamoDBHelper,
                             table_name: &str)
                             -> Result<(), DynamoDBError> {

    loop {
        let result = try!(dynamodb.describe_table(table_name));

        if let Some(ref status) = result.get_status() {
            if status == "ACTIVE" {
                break;
            } else {
                println!("\t{} not ready - {}", table_name, status);
                thread::sleep_ms(1000);
            }
        }

    }
    Ok(())
}

fn dynamo_delete_table_test(dynamodb: &mut DynamoDBHelper,
                            table_name: &str)
                            -> Result<(), DynamoDBError> {
    let _result = try!(dynamodb.delete_table(table_name));
    Ok(())
}
