//! Parameters for talking to SQS.
//!
//! Key-value pairs for SQS requests.
//!
//! Supports optional parameters for calling SQS.

use std::collections::BTreeMap;
pub type Params = BTreeMap<String, String>;

/// Key:value pair for an SQS parameter.
pub trait SQSParams {
	fn put(&mut self, key: &str, val: &str);
}

impl SQSParams for Params {
	fn put(&mut self, key: &str, val: &str) {
		self.insert(key.into(), val.into());
	}
}
