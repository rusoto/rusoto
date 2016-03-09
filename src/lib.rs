#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![allow(dead_code)]

//! # Rusoto
//!
//! Rusoto is an AWS SDK for Rust.
//! A high level overview is available in `README.md` at https://github.com/rusoto/rusoto.

extern crate time;
extern crate xml;
extern crate hyper;
extern crate openssl;
extern crate url;
extern crate rustc_serialize;
extern crate regex;
extern crate crypto;
extern crate chrono;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate log;

#[macro_use] pub mod params;
#[macro_use] pub mod signature;
pub mod credentials;
pub mod error;
pub mod xmlutil;
pub mod regions;
pub mod request;

#[cfg(feature = "dynamodb")]
pub mod dynamodb;
#[cfg(feature = "ecs")]
pub mod ecs;
#[cfg(feature = "kms")]
pub mod kms;
#[cfg(feature = "s3")]
pub mod s3;
#[cfg(feature = "sqs")]
pub mod sqs;
