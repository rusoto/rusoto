#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![allow(dead_code)]

extern crate time;
extern crate xml;
extern crate hyper;
extern crate openssl;
extern crate url;
extern crate rustc_serialize as serialize;

#[macro_use] pub mod params;
#[macro_use] pub mod signature;
pub mod credentials;
pub mod error;
pub mod sqs;
pub mod s3;
pub mod xmlutil;
