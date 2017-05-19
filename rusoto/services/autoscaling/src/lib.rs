
//! Auto Scaling
//!
//! If you're using the service, you're probably looking for [AutoscalingClient](struct.AutoscalingClient.html) and [Autoscaling](trait.Autoscaling.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            