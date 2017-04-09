//! Amazon CloudWatch Logs
//!
//! If you're using the service, you're probably looking for [CloudWatchLogsClient](struct.CloudWatchLogsClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/logs.rs"));
