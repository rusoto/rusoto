//! AWS Certificate Manager
//!
//! If you're using the service, you're probably looking for [AcmClient](struct.AcmClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/acm.rs"));
