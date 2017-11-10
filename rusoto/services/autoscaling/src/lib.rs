
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

//! <fullname>Auto Scaling</fullname> <p>Auto Scaling is designed to automatically launch or terminate EC2 instances based on user-defined policies, schedules, and health checks. Use this service in conjunction with the Amazon CloudWatch and Elastic Load Balancing services.</p>
//!
//! If you're using the service, you're probably looking for [AutoscalingClient](struct.AutoscalingClient.html) and [Autoscaling](trait.Autoscaling.html).

extern crate futures;
extern crate hyper;
extern crate rusoto_core;
extern crate tokio_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            
