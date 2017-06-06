use std::error::Error;
use std::fmt::{Formatter, Result as FmtResult};

use rustc_serialize::base64::{FromBase64, STANDARD, ToBase64};
use serde::{Deserializer, Serializer};
use serde::de::{Error as SerdeError, Visitor};

pub trait SerdeBlob: Sized {
    fn deserialize_blob<'de, D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>;

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer;
}

struct BlobVisitor;

impl<'de> Visitor<'de> for BlobVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a blob")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: SerdeError {
        v.from_base64().map_err(|err| SerdeError::custom(err.description()))
    }
}

impl SerdeBlob for Vec<u8> {
    fn deserialize_blob<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_any(BlobVisitor)
    }

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        serializer.serialize_str(&self.as_slice().to_base64(STANDARD))
    }
}

struct OptionalBlobVisitor;

impl<'de> Visitor<'de> for OptionalBlobVisitor {
    type Value = Option<Vec<u8>>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a blob")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
        where E: SerdeError {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de> {
        Ok(Some(try!(SerdeBlob::deserialize_blob(deserializer))))
    }
}

impl SerdeBlob for Option<Vec<u8>> {
    fn deserialize_blob<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_option(OptionalBlobVisitor)
    }

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        match *self {
            Some(ref vec) => SerdeBlob::serialize_blob(vec, serializer),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde;
    extern crate serde_json;

    use super::SerdeBlob;

    #[test]
    fn serialize_optional_blob_when_none() {
        let blob: Option<Vec<u8>> = None;
        let serialized = serialize_blob_helper(blob);

        assert_eq!("null", serialized);
    }

    #[test]
    fn serialize_optional_blob_when_empty() {
        let blob = Some(vec![]);
        let serialized = serialize_blob_helper(blob);

        assert_eq!("\"\"", serialized);
    }

    #[test]
    fn serialize_optional_blob_when_has_content() {
        let blob = Some("hello world!".as_bytes().to_vec());
        let serialized = serialize_blob_helper(blob);

        assert_eq!("\"aGVsbG8gd29ybGQh\"", serialized);
    }

    #[test]
    fn deserialize_optional_blob_when_null() {
        let deserialized: Option<Vec<u8>> = deserialize_blob_helper("null").unwrap();

        assert!(deserialized.is_none());
    }

    #[test]
    fn deserialize_optional_blob_when_empty() {
        let deserialized: Option<Vec<u8>> = deserialize_blob_helper("\"\"").unwrap();

        assert_eq!(Some(vec![]), deserialized);
    }

    #[test]
    fn deserialize_optional_blob_when_has_content() {
        let deserialized: Option<Vec<u8>> = deserialize_blob_helper("\"aGVsbG8gd29ybGQh\"").unwrap();

        assert_eq!(Some("hello world!".as_bytes().to_vec()), deserialized);
    }

    #[test]
    fn serialize_blob_when_empty() {
        let blob = vec![];
        let serialized = serialize_blob_helper(blob);

        assert_eq!("\"\"", serialized);
    }

    #[test]
    fn serialize_blob_when_has_content() {
        let blob = "hello world!".as_bytes().to_vec();
        let serialized = serialize_blob_helper(blob);

        assert_eq!("\"aGVsbG8gd29ybGQh\"", serialized);
    }

    #[test]
    fn deserialize_blob_when_null() {
        let deserialized: Result<Vec<u8>, _> = deserialize_blob_helper("null");

        assert!(deserialized.is_err());
    }

    #[test]
    fn deserialize_blob_when_empty() {
        let deserialized: Vec<u8> = deserialize_blob_helper("\"\"").unwrap();

        assert_eq!(Vec::<u8>::new(), deserialized);
    }

    #[test]
    fn deserialize_blob_when_has_content() {
        let deserialized: Vec<u8> = deserialize_blob_helper("\"aGVsbG8gd29ybGQh\"").unwrap();

        assert_eq!("hello world!".as_bytes().to_vec(), deserialized);
    }

    fn serialize_blob_helper<B: SerdeBlob>(blob: B) -> String {
        let mut serialized_data = Vec::new();
        {
            let mut json_serializer = serde_json::Serializer::new(&mut serialized_data);
            blob.serialize_blob(&mut json_serializer).unwrap();
        }
        String::from_utf8_lossy(&serialized_data).into_owned()
    }

    fn deserialize_blob_helper<B: SerdeBlob>(s: &str) -> Result<B, <&mut serde_json::de::Deserializer<serde_json::de::StrRead> as serde::Deserializer>::Error> {
        let reader = serde_json::de::StrRead::new(s);
        let mut deserializer = serde_json::de::Deserializer::new(reader);
        B::deserialize_blob(&mut deserializer)
    }
}