#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![cfg_attr(feature = "unstable", feature(custom_derive, plugin))]
#![cfg_attr(feature = "unstable", plugin(serde_macros))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(used_underscore_binding, ptr_arg))]
#![allow(dead_code)]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]

//! Rusoto is an [AWS](https://aws.amazon.com/) SDK for Rust.
//! A high level overview is available in `README.md` at https://github.com/rusoto/rusoto.

extern crate chrono;
extern crate hyper;
#[macro_use] extern crate log;
extern crate openssl;
extern crate regex;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate url;
extern crate xml;

pub use credential::{
    AwsCredentials,
    ChainProvider,
    EnvironmentProvider,
    IamProvider,
    ProfileProvider,
    ProvideAwsCredentials,
    DefaultCredentialsProvider,
    DefaultCredentialsProviderSync,
};
pub use region::{ParseRegionError, Region};
pub use request::{DispatchSignedRequest, HttpResponse, HttpDispatchError};

mod credential;
mod param;
mod region;
mod request;
mod xmlerror;
mod xmlutil;
mod serialization;
#[macro_use] mod signature;


#[cfg(feature = "acm")]
pub mod acm;
#[cfg(feature = "cloudhsm")]
pub mod cloudhsm;
#[cfg(feature = "cloudtrail")]
pub mod cloudtrail;
#[cfg(feature = "codecommit")]
pub mod codecommit;
#[cfg(feature = "codedeploy")]
pub mod codedeploy;
#[cfg(feature = "codepipeline")]
pub mod codepipeline;
#[cfg(feature = "cognito-identity")]
pub mod cognitoidentity;
#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "datapipeline")]
pub mod datapipeline;
#[cfg(feature = "devicefarm")]
pub mod devicefarm;
#[cfg(feature = "directconnect")]
pub mod directconnect;
#[cfg(feature = "ds")]
pub mod ds;
#[cfg(feature = "dynamodb")]
pub mod dynamodb;
#[cfg(feature = "dynamodbstreams")]
pub mod dynamodbstreams;
#[cfg(feature = "ec2")]
pub mod ec2;
#[cfg(feature = "ecr")]
pub mod ecr;
#[cfg(feature = "ecs")]
pub mod ecs;
#[cfg(feature = "emr")]
pub mod emr;
#[cfg(feature = "elastictranscoder")]
pub mod elastictranscoder;
#[cfg(feature = "events")]
pub mod events;
#[cfg(feature = "firehose")]
pub mod firehose;
#[cfg(feature = "inspector")]
pub mod inspector;
#[cfg(feature = "kinesis")]
pub mod kinesis;
#[cfg(feature = "kms")]
pub mod kms;
#[cfg(feature = "logs")]
pub mod logs;
#[cfg(feature = "machinelearning")]
pub mod machinelearning;
#[cfg(feature = "marketplacecommerceanalytics")]
pub mod marketplacecommerceanalytics;
#[cfg(feature = "opsworks")]
pub mod opsworks;
#[cfg(feature = "route53domains")]
pub mod route53domains;
#[cfg(feature = "s3")]
pub mod s3;
#[cfg(feature = "sqs")]
pub mod sqs;
#[cfg(feature = "ssm")]
pub mod ssm;
#[cfg(feature = "storagegateway")]
pub mod storagegateway;
#[cfg(feature = "swf")]
pub mod swf;
#[cfg(feature = "waf")]
pub mod waf;
#[cfg(feature = "workspaces")]
pub mod workspaces;

/*
#[cfg(feature = "gamelift")]
pub mod gamelift;
#[cfg(feature = "support")]
pub mod support;
*/

