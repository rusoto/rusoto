//! ## List Tables
//!
//! The following code shows a simple example of using Rusoto's DynamoDB API to
//! list the names of all tables in a database.
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

fn main() {
    let client = DynamoDbClient::simple(Region::UsEast1);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(&list_tables_input).sync() {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    println!("Tables in database:");

                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                },
                None => println!("No tables in database!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}
