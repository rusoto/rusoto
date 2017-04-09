//! AWS Lambda
//!
//! If you're using the service, you're probably looking for [LambdaClient](struct.LambdaClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/lambda.rs"));
