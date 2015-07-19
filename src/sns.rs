//! SNS bindings for Rust
#![allow(unused_variables, unused_mut, non_camel_case_types, unused_imports)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use xmlutil::*;
use std::str::FromStr;

// include the code generated from the SNS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/sns.rs"));

