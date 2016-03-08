//! Parameters for talking to SQS.
//!
//! Key-value pairs for SQS requests.
//!
//! Supports optional parameters for calling SQS.

use std::collections::BTreeMap;
pub type Params = BTreeMap<String, String>;

macro_rules! params {
	($($key:expr => $val:expr),*) => {
		{
			let mut params:Params = Params::new();
			$(
				params.insert($key.to_string(), $val.to_string());
			)*
			params
		}
	}
}

/// Key:value pair for an SQS parameter.
pub trait SQSParams {
	fn put(&mut self, key: &str, val: &str);
}

impl SQSParams for Params {
	fn put(&mut self, key: &str, val: &str) {
		self.insert(key.into(), val.into());
	}
}

/// Optional fields for SQS call
pub trait OptionalMap<T> {
	fn optional_put(&mut self, name: &str, value_opt: &Option<T>) ;

}

impl OptionalMap<String> for Params {
	fn optional_put(&mut self, name: &str, value_opt: &Option<String>) {
		if let &Some(ref value) = value_opt {
			self.insert(name.into(), value.to_string());
		}
	}
}

impl OptionalMap<i32> for Params {
	fn optional_put(&mut self, name: &str, value_opt: &Option<i32>) {
		if let &Some(value) = value_opt {
			self.insert(name.into(), value.to_string());
		}
	}
}
