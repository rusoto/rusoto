//! AWS Relational Database Service
//!
//! If you're using the service, you're probably looking for [RdsClient](struct.RdsClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/rds.rs"));
