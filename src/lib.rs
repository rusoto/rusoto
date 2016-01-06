#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![allow(dead_code)]

//! # Rusoto
//!
//! Rusoto is a Rust SDK for [Amazon Web Services](http://aws.amazon.com/).  It follows best practices
//! for [AWS Credentials](https://blogs.aws.amazon.com/security/post/Tx3D6U6WSFGOK2H/A-New-and-Standardized-Way-to-Manage-Credentials-in-the-AWS-SDKs).
//!
//! The library interacts with AWS in the same fashion as Python's [boto3](https://github.com/boto/boto3) SDK.
//!
//! ## Credentials
//!
//! Credentials are sourced from environment variables, AWS credentials file and IAM instance profiles,
//! in that order.  IAM instance profile credentials are refreshed automatically as needed.
//!
//! ## Supported services
//!
//! * DynamoDB
//! * KMS
//! * S3
//! * SQS
//!
//! ## Requests and request signing
//!
//! Rusoto uses [AWS Signature 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html)
//! to sign requests.

extern crate time;
extern crate xml;
extern crate hyper;
extern crate openssl;
extern crate url;
extern crate rustc_serialize;
extern crate regex;
extern crate crypto;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;

#[macro_use] pub mod params;
#[macro_use] pub mod signature;
pub mod credentials;
pub mod dynamodb;
pub mod error;
pub mod kms;
pub mod sqs;
pub mod s3;
pub mod xmlutil;
pub mod regions;
pub mod request;
