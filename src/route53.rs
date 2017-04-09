//! AWS Route 53
//!
//! If you're using the service, you're probably looking for [Route53Client](struct.Route53Client.html).

use region;

include!(concat!(env!("OUT_DIR"), "/route53.rs"));
