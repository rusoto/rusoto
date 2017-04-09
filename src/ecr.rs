//! Amazon EC2 Container Registry
//!
//! If you're using the service, you're probably looking for [EcrClient](struct.EcrClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/ecr.rs"));
