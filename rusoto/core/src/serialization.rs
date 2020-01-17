use std::fmt::{Formatter, Result as FmtResult};

use base64;
use bytes::Bytes;
use serde::de::{Error as SerdeError, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait SerdeBlob: Sized {
    fn deserialize_blob<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

struct BlobVisitor;

impl<'de> Visitor<'de> for BlobVisitor {
    type Value = Bytes;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a blob")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        base64::decode(v)
            .map(Bytes::from)
            .map_err(|err| SerdeError::custom(err.to_string()))
    }
}

impl SerdeBlob for Bytes {
    fn deserialize_blob<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BlobVisitor)
    }

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(self.as_ref()))
    }
}

struct OptionalBlobVisitor;

impl<'de> Visitor<'de> for OptionalBlobVisitor {
    type Value = Option<Bytes>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a blob")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(SerdeBlob::deserialize_blob(deserializer)?))
    }
}

impl SerdeBlob for Option<Bytes> {
    fn deserialize_blob<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionalBlobVisitor)
    }

    fn serialize_blob<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Some(ref vec) => SerdeBlob::serialize_blob(vec, serializer),
            None => serializer.serialize_none(),
        }
    }
}

struct DeserializeWrapper<T>(T);

impl<'de, T> Deserialize<'de> for DeserializeWrapper<T>
where
    T: SerdeBlob,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DeserializeWrapper(T::deserialize_blob(deserializer)?))
    }
}

struct SerializeWrapper<'a, T>(&'a T);

impl<'a, T> Serialize for SerializeWrapper<'a, T>
where
    T: SerdeBlob,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        T::serialize_blob(self.0, serializer)
    }
}

pub trait SerdeBlobList: Sized {
    fn deserialize_blob_list<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;

    fn serialize_blob_list<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

struct BlobListVisitor;

impl<'de> Visitor<'de> for BlobListVisitor {
    type Value = Vec<Bytes>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a list of blobs")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut blob_list = Vec::new();
        while let Some(DeserializeWrapper(blob)) = seq.next_element()? {
            blob_list.push(blob);
        }
        Ok(blob_list)
    }
}

impl SerdeBlobList for Vec<Bytes> {
    fn deserialize_blob_list<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BlobListVisitor)
    }

    fn serialize_blob_list<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for blob in self.iter() {
            seq.serialize_element(&SerializeWrapper(blob))?;
        }
        seq.end()
    }
}

struct OptionalBlobListVisitor;

impl<'de> Visitor<'de> for OptionalBlobListVisitor {
    type Value = Option<Vec<Bytes>>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "an optional blob list")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(SerdeBlobList::deserialize_blob_list(deserializer)?))
    }
}

impl SerdeBlobList for Option<Vec<Bytes>> {
    fn deserialize_blob_list<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionalBlobListVisitor)
    }

    fn serialize_blob_list<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Some(ref list) => SerdeBlobList::serialize_blob_list(list, serializer),
            None => serializer.serialize_none(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde;
    extern crate serde_json;

    use super::{SerdeBlob, SerdeBlobList};
    use bytes::Bytes;

    #[test]
    fn serialize_optional_blob_when_none() {
        let blob: Option<Bytes> = None;
        let serialized = serialize_blob_helper(blob);

        assert_eq!("null", serialized);
    }

    #[test]
    fn serialize_optional_blob_when_empty() {
        let blob = Some(Bytes::new());
        let serialized = serialize_blob_helper(blob);

        assert_eq!("\"\"", serialized);
    }

    #[test]
    fn serialize_optional_blob_when_has_content() {
        let blob = Some(Bytes::from_static(b"hello world!"));
        let serialized = serialize_blob_helper(blob);

        assert_eq!("\"aGVsbG8gd29ybGQh\"", serialized);
    }

    #[test]
    fn deserialize_optional_blob_when_null() {
        let deserialized: Option<Bytes> = deserialize_blob_helper("null").unwrap();

        assert!(deserialized.is_none());
    }

    #[test]
    fn deserialize_optional_blob_when_empty() {
        let deserialized: Option<Bytes> = deserialize_blob_helper("\"\"").unwrap();

        assert_eq!(Some(Bytes::new()), deserialized);
    }

    #[test]
    fn deserialize_optional_blob_when_has_content() {
        let deserialized: Option<Bytes> = deserialize_blob_helper("\"aGVsbG8gd29ybGQh\"").unwrap();

        assert_eq!(Some(Bytes::from_static(b"hello world!")), deserialized);
    }

    #[test]
    fn serialize_blob_when_empty() {
        let blob = Bytes::new();
        let serialized = serialize_blob_helper(blob);

        assert_eq!("\"\"", serialized);
    }

    #[test]
    fn serialize_blob_when_has_content() {
        let blob = Bytes::from_static(b"hello world!");
        let serialized = serialize_blob_helper(blob);

        assert_eq!("\"aGVsbG8gd29ybGQh\"", serialized);
    }

    #[test]
    fn deserialize_blob_when_null() {
        let deserialized: Result<Bytes, _> = deserialize_blob_helper("null");

        assert!(deserialized.is_err());
    }

    #[test]
    fn deserialize_blob_when_empty() {
        let deserialized: Bytes = deserialize_blob_helper("\"\"").unwrap();

        assert_eq!(Bytes::new(), deserialized);
    }

    #[test]
    fn deserialize_blob_when_has_content() {
        let deserialized: Bytes = deserialize_blob_helper("\"aGVsbG8gd29ybGQh\"").unwrap();

        assert_eq!(Bytes::from_static(b"hello world!"), deserialized);
    }

    #[test]
    fn serialize_blob_list_when_empty() {
        let blob_list = vec![];
        let serialized = serialize_blob_list_helper(blob_list);

        assert_eq!("[]", serialized);
    }

    #[test]
    fn serialize_blob_list_when_has_content() {
        let blob_list = vec![
            Bytes::from_static(b"Sunny"),
            Bytes::from_static(b"Rainy"),
            Bytes::from_static(b"Snowy"),
        ];
        let serialized = serialize_blob_list_helper(blob_list);

        assert_eq!("[\"U3Vubnk=\",\"UmFpbnk=\",\"U25vd3k=\"]", serialized);
    }

    #[test]
    fn deserialize_blob_list_when_empty() {
        let deserialized: Vec<Bytes> = deserialize_blob_list_helper("[]").unwrap();

        assert_eq!(Vec::<Bytes>::new(), deserialized);
    }

    #[test]
    fn deserialize_blob_list_when_has_content() {
        let deserialized: Vec<Bytes> =
            deserialize_blob_list_helper("[\"U3Vubnk=\",\"UmFpbnk=\",\"U25vd3k=\"]").unwrap();

        assert_eq!(
            vec![
                Bytes::from_static(b"Sunny"),
                Bytes::from_static(b"Rainy"),
                Bytes::from_static(b"Snowy")
            ],
            deserialized
        );
    }

    fn serialize_blob_helper<B: SerdeBlob>(blob: B) -> String {
        let mut serialized_data = Vec::new();
        {
            let mut json_serializer = serde_json::Serializer::new(&mut serialized_data);
            blob.serialize_blob(&mut json_serializer).unwrap();
        }
        String::from_utf8_lossy(&serialized_data).into_owned()
    }

    fn deserialize_blob_helper<B: SerdeBlob>(
        s: &str,
    ) -> Result<
        B,
        <&mut serde_json::de::Deserializer<serde_json::de::StrRead> as serde::Deserializer>::Error,
    > {
        let reader = serde_json::de::StrRead::new(s);
        let mut deserializer = serde_json::de::Deserializer::new(reader);
        B::deserialize_blob(&mut deserializer)
    }

    fn serialize_blob_list_helper<B: SerdeBlobList>(blob_list: B) -> String {
        let mut serialized_data = Vec::new();
        {
            let mut json_serializer = serde_json::Serializer::new(&mut serialized_data);
            blob_list.serialize_blob_list(&mut json_serializer).unwrap();
        }
        String::from_utf8_lossy(&serialized_data).into_owned()
    }

    fn deserialize_blob_list_helper<B: SerdeBlobList>(
        s: &str,
    ) -> Result<
        B,
        <&mut serde_json::de::Deserializer<serde_json::de::StrRead> as serde::Deserializer>::Error,
    > {
        let reader = serde_json::de::StrRead::new(s);
        let mut deserializer = serde_json::de::Deserializer::new(reader);
        B::deserialize_blob_list(&mut deserializer)
    }
}
