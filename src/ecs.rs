//! [ECS](https://aws.amazon.com/ecs/) bindings for Rust
//!
//! To get started, see the docs for [ECSClient](struct.ECSClient.html)

#![allow(non_snake_case)]

use credentials::AWSCredentialsProvider;
use regions::Region;
use signature::SignedRequest;

include!(concat!(env!("OUT_DIR"), "/ecs.rs"));

include!(concat!(env!("OUT_DIR"), "/ecs_helpers.rs"));
