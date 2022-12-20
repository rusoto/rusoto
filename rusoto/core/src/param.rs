//! Parameters for talking to query-based AWS services.
//!
//! Key-value pairs for AWS query requests.
//!
//! Supports optional parameters for calling SQS and ETS.

use std::collections::BTreeMap;
/// Parameters for HTTP Request stored as a `BTreeMap`
pub type Params = BTreeMap<String, Option<String>>;

/// Key:value pair for an service parameter.
pub trait ServiceParams {
    /// Add a new parameter with a key and val
    ///
    /// * `key` - The key of the parameter to add.
    /// * `val` - The value of the parameter to add.
    fn put<T: ToParam>(&mut self, key: &str, val: T);
    /// Add a new parameter with a key
    fn put_key(&mut self, key: &str);
}

impl ServiceParams for Params {
    fn put<T: ToParam>(&mut self, key: &str, val: T) {
        self.insert(key.to_owned(), Some(val.to_param()));
    }

    fn put_key(&mut self, key: &str) {
        self.insert(key.to_owned(), None);
    }
}

/// Trait for implementing type to parameter conversion
pub trait ToParam {
    /// Renders this parameter to a String
    fn to_param(&self) -> String;
}

// The implementations below should be kept in sync with the types defined in rusoto_codegen/generator/mod.rs

macro_rules! to_string_param {
    ( $($t:ty)* ) => {
        $(
            impl ToParam for $t {
                fn to_param(&self) -> String {
                    self.to_string()
                }
            }
        )*
    }
}

to_string_param!( u8 bool f32 f64 i64 );

impl<'a, T> ToParam for &'a T
where
    T: ToParam + ?Sized,
{
    /// Converts a generic type to a parameter
    fn to_param(&self) -> String {
        ToParam::to_param(&**self)
    }
}

impl ToParam for str {
    /// Converts a str to a paramter
    fn to_param(&self) -> String {
        String::from(self)
    }
}

impl ToParam for String {
    /// Converts a String to a parameter
    fn to_param(&self) -> String {
        self.clone()
    }
}
