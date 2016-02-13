//! KMS bindings for Rust
//!
//! Check
//! [Helper](http://dualspark.github.io/rusoto/rusoto/kms/struct.KMSHelper.html)
//! for convenience functions.
//!

#![allow(unused_variables, unused_mut, non_snake_case)]
use std::result;

use credentials::AWSCredentialsProvider;
use error::AWSError;
use regions::Region;
use signature::SignedRequest;

// include the code generated from the KMS botocore templates
include!(concat!(env!("OUT_DIR"), "/kms.rs"));

// include the code generated from the helpers
include!(concat!(env!("OUT_DIR"), "/kms_helpers.rs"));
