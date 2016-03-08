//! The AWS SQS API.

#![allow(unused_variables, unused_mut)]

use std::str::FromStr;

use xml::EventReader;

use credentials::AWSCredentialsProvider;
use error::AWSError;
use params::{Params, SQSParams};
use regions::Region;
use signature::SignedRequest;
use xmlutil::{Next, Peek, XmlParseError, XmlResponseFromAws};
use xmlutil::{characters, end_element, peek_at_name, start_element};

// include the code generated from the SQS botocore templates
include!(concat!(env!("OUT_DIR"), "/sqs.rs"));

