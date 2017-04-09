//! Amazon Identity and Access Management
//!
//! If you're using the service, you're probably looking for [IamClient](struct.IamClient.html).

#![cfg_attr(feature = "nightly-testing", allow(while_let_loop))]

use region;

include!(concat!(env!("OUT_DIR"), "/iam.rs"));
