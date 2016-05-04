#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(used_underscore_binding, ptr_arg))]
#![allow(dead_code)]
#![cfg_attr(not(feature = "nightly"), deny(warnings))]

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
};
pub use error::{AwsError, AwsResult};
pub use region::{ParseRegionError, Region};

mod credential;
mod error;
mod param;
mod region;
mod request;
mod xmlutil;
mod serialization;
#[macro_use] mod signature;

#[cfg(feature = "dynamodb")]
pub mod dynamodb;
#[cfg(feature = "ecs")]
pub mod ecs;
#[cfg(feature = "ets")]
pub mod ets;
#[cfg(feature = "kms")]
pub mod kms;
#[cfg(feature = "s3")]
pub mod s3;
#[cfg(feature = "sqs")]
pub mod sqs;
