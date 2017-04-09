//! The CloudFront API.
//!
//! If you're using the service, you're probably looking for [CloudFrontClient](struct.CloudFrontClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/cloudfront.rs"));
