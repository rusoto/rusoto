//! S3 bindings for Rust

#![allow(unused_variables, unused_mut, non_snake_case)]
use std::result;

use credentials::AWSCredentialsProvider;
use error::AWSError;
use regions::Region;
use signature::SignedRequest;
use xmlutil::{XmlResponseFromAws, XmlParseError, Next, Peek};
use xml::reader::EventReader;
use hyper::header::Headers;

// Probably can pull this enum out to a different file:
#[derive(Debug,PartialEq)]
pub enum ArgumentLocation {
	Header,
	Body,
	Headers,
	Querystring,
	Uri,
}

// include the code generated from the S3 botocore templates
include!(concat!(env!("OUT_DIR"), "/s3.rs"));

pub struct S3Error;

pub type Result<T> = result::Result<T, S3Error>;
