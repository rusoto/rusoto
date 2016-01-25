//! S3 bindings for Rust

#![allow(unused_variables, unused_mut, non_snake_case)]
use std::result;

use credentials::AWSCredentialsProvider;
use error::AWSError;
use regions::Region;
use signature::SignedRequest;
use xmlutil::XmlResponseFromAws;
use xml::reader::EventReader;

// include the code generated from the S3 botocore templates
include!(concat!(env!("OUT_DIR"), "/s3.rs"));

pub struct S3Error;

pub type Result<T> = result::Result<T, S3Error>;
