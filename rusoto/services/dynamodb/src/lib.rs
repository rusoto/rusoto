
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
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p><fullname>Amazon DynamoDB</fullname> <p>Amazon DynamoDB is a fully managed NoSQL database service that provides fast and predictable performance with seamless scalability. DynamoDB lets you offload the administrative burdens of operating and scaling a distributed database, so that you don&#39;t have to worry about hardware provisioning, setup and configuration, replication, software patching, or cluster scaling.</p> <p>With DynamoDB, you can create database tables that can store and retrieve any amount of data, and serve any level of request traffic. You can scale up or scale down your tables&#39; throughput capacity without downtime or performance degradation, and use the AWS Management Console to monitor resource utilization and performance metrics.</p> <p>DynamoDB automatically spreads the data and traffic for your tables over a sufficient number of servers to handle your throughput and storage requirements, while maintaining consistent and fast performance. All of your data is stored on solid state disks (SSDs) and automatically replicated across multiple Availability Zones in an AWS region, providing built-in high availability and data durability. </p></p>
//!
//! If you're using the service, you're probably looking for [DynamoDbClient](struct.DynamoDbClient.html) and [DynamoDb](trait.DynamoDb.html).
//!
//! # Examples
//!
//! ## List Tables
//!
//! The following code shows a simple example of using Rusoto's DynamoDB API to
//! list the names of all tables in a database.
//!
//! ```rust,no_run
//! extern crate rusoto_core;
//! extern crate rusoto_dynamodb;
//! 
//! use rusoto_core::Region;
//! use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
//! 
//! fn main() {
//!     let client = DynamoDbClient::new(Region::UsEast1);
//!     let list_tables_input: ListTablesInput = Default::default();
//! 
//!     match client.list_tables(list_tables_input).sync() {
//!         Ok(output) => match output.table_names {
//!             Some(table_name_list) => {
//!                 println!("Tables in database:");
//! 
//!                 for table_name in table_name_list {
//!                     println!("{}", table_name);
//!                 }
//!             }
//!             None => println!("No tables in database!"),
//!         },
//!         Err(error) => {
//!             println!("Error: {:?}", error);
//!         }
//!     }
//! }
//! ```

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
