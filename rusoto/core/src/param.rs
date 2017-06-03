//! Parameters for talking to query-based AWS services.
//!
//! Key-value pairs for AWS query requests.
//!
//! Supports optional parameters for calling SQS and ETS.

use std::collections::BTreeMap;
pub type Params = BTreeMap<String, Option<String>>;

/// Key:value pair for an service parameter.
pub trait ServiceParams {
    fn put<T: ToParam>(&mut self, key: &str, val: T);
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

pub trait ToParam {
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

impl<'a, T> ToParam for &'a T where T: ToParam + ?Sized {
    fn to_param(&self) -> String {
        ToParam::to_param(&**self)
    }
}

impl ToParam for str {
    fn to_param(&self) -> String {
        String::from(self)
    }
}

impl ToParam for String {
    fn to_param(&self) -> String {
        self.clone()
    }
}

impl<T> ToParam for Vec<T> where T: ToParam {
    fn to_param(&self) -> String {
        self.iter().map(|x| x.to_param()).collect::<Vec<_>>().join(",")
    }
}
