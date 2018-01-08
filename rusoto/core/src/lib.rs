#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity, used_underscore_binding, ptr_arg, suspicious_else_formatting))]
#![allow(dead_code)]
#![deny(missing_docs)]

//! Rusoto is an [AWS](https://aws.amazon.com/) SDK for Rust.
//! A high level overview is available in `README.md` at https://github.com/rusoto/rusoto.
//!
//! # Example
//!
//! The following code shows a simple example of using Rusoto's DynamoDB API to
//! list the names of all tables in a database.
//!
//! ```rust,no_run
//! extern crate rusoto_core;
//! extern crate rusoto_dynamodb;
//!
//! use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
//! use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
//!
//! fn main() {
//!     let dispatcher = default_tls_client().unwrap();
//!     let provider = DefaultCredentialsProvider::new().unwrap();
//!     let client = DynamoDbClient::new(dispatcher, provider, Region::UsEast1);
//!     let list_tables_input: ListTablesInput = Default::default();
//!
//!     match client.list_tables(&list_tables_input) {
//!         Ok(output) => {
//!             match output.table_names {
//!                 Some(table_name_list) => {
//!                     println!("Tables in database:");
//!
//!                     for table_name in table_name_list {
//!                         println!("{}", table_name);
//!                     }
//!                 },
//!                 None => println!("No tables in database!"),
//!             }
//!         },
//!         Err(error) => {
//!             println!("Error: {:?}", error);
//!         },
//!     }
//! }

extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate hex;
extern crate hmac;
extern crate base64;
pub extern crate rusoto_credential as credential;
extern crate serde;
extern crate sha2;
extern crate time;
extern crate url;
extern crate unicase;
extern crate xml;

pub mod param;
pub mod region;
pub mod request;
pub mod signature;

#[doc(hidden)]
pub mod serialization;
#[doc(hidden)]
pub mod xmlerror;
#[doc(hidden)]
pub mod xmlutil;

pub use credential::{AwsCredentials, ChainProvider, ContainerProvider, CredentialsError,
                     EnvironmentProvider, InstanceMetadataProvider, ProfileProvider,
                     ProvideAwsCredentials, DefaultCredentialsProvider,
                     DefaultCredentialsProviderSync, claims,
                     AutoRefreshingProviderSync, AutoRefreshingProvider,
                     BaseAutoRefreshingProvider};

pub use region::{ParseRegionError, Region, default_region};
pub use request::{DispatchSignedRequest, HttpResponse, HttpDispatchError, TlsError, default_tls_client};
pub use signature::SignedRequest;
