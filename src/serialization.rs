use std::error::Error;
use std::fmt::{Formatter, Result as FmtResult};

use rustc_serialize::base64::{FromBase64, STANDARD, ToBase64};
use serde::{Deserializer, Serializer};
use serde::de::{Error as SerdeError, Visitor};

pub trait SerdeBlob: Sized {
    fn deserialize_blob<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer;

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer;
}

struct BlobVisitor;

impl Visitor for BlobVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a blob")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: SerdeError {
        v.from_base64().map_err(|err| SerdeError::custom(err.description()))
    }
}

impl SerdeBlob for Vec<u8> {
    fn deserialize_blob<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where D: Deserializer {
        deserializer.deserialize(BlobVisitor)
    }

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.as_slice().to_base64(STANDARD))
    }
}

struct OptionalBlobVisitor;

impl Visitor for OptionalBlobVisitor {
    type Value = Option<Vec<u8>>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a blob")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> where E: SerdeError {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where D: Deserializer {
        Ok(Some(try!(SerdeBlob::deserialize_blob(deserializer))))
    }
}

impl SerdeBlob for Option<Vec<u8>> {
    fn deserialize_blob<D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where D: Deserializer {
        deserializer.deserialize_option(OptionalBlobVisitor)
    }

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            Some(ref vec) => serializer.serialize_some(vec),
            None => serializer.serialize_none(),
        }
    }
}
