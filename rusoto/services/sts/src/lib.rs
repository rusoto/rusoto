
//! AWS Security Token Service
//!
//! If you're using the service, you're probably looking for [StsClient](struct.StsClient.html), [Sts](trait.Sts.html)
//! and [StsAssumeRoleSessionCredentialsProvider](struct.StsAssumeRoleSessionCredentialsProvider.html).
//!
//! # Example
//! ```rust,ignore
//! extern crate env_logger;
//! extern crate rusoto_core;
//! extern crate rusoto_ec2;
//! extern crate rusoto_sts;
//!
//! use std::default::Default;
//!
//! use rusoto_core::{DefaultCredentialsProvider, Region};
//! use rusoto_core::default_tls_client;
//!
//! use rusoto_ec2::{Ec2Client, Ec2};
//! use rusoto_sts::{StsClient, StsAssumeRoleSessionCredentialsProvider};
//!
//! fn main() {
//!     env_logger::init().unwrap();
//!
//!     let credentials = DefaultCredentialsProvider::new().unwrap();
//!     let sts = StsClient::new(default_tls_client().unwrap(), credentials, Region::EuWest1);
//!
//!     let provider = StsAssumeRoleSessionCredentialsProvider::new(
//!         sts,
//!         "arn:aws:iam::something:role/something".to_owned(),
//!         "default".to_owned(),
//!         None, None, None, None
//!     );
//!
//!     let client = Ec2Client::new(default_tls_client().unwrap(), provider, Region::UsEast1);
//!
//!     let sir_input = Default::default();
//!     println!("[*] requesting...");
//!     let x = client.describe_spot_instance_requests(&sir_input);
//!
//!     println!("{:?}", x);
//! }

extern crate chrono;
extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            