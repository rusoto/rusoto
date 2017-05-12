// This file provides a custom deserializer for `String` and
// `BTreeMap<String, V>`, which function identically to the built-in serde
// deserializers, except for the fact that `String`s deserialized with these
// deserializers get their first letter capitalized.
//
// This is necessary for proper deserialization of `Shape` names from their
// botocore definitions in certain edge cases. Particularly, IAM shape names use
// "camelCase" instead of "CamelCase".
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::fmt::{Formatter, Result as FmtResult};

use serde::{Deserialize, Deserializer};
use serde::de::{Error as SerdeError, Visitor, MapAccess};

use generator::capitalize_first;

pub trait ShapeName<'de>: Sized {
    fn deserialize_shape_name<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>;
}

struct ShapeNameVisitor;

impl<'de> Visitor<'de> for ShapeNameVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a shape name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: SerdeError {
        Ok(capitalize_first(v))
    }
}

impl <'de> ShapeName<'de> for String {
    fn deserialize_shape_name<D>(deserializer: D) -> Result<String, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_string(ShapeNameVisitor)
    }
}

pub trait ShapesMap<'de>: Sized {
    fn deserialize_shapes_map<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>;
}

struct ShapesMapVisitor<V> {
    marker: PhantomData<BTreeMap<String, V>>,
}

impl<V> ShapesMapVisitor<V> {
    pub fn new() -> Self {
        ShapesMapVisitor { marker: PhantomData }
    }
}

impl<'de, V> Visitor<'de> for ShapesMapVisitor<V>
    where V: Deserialize<'de> {
    type Value = BTreeMap<String, V>;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "a shapes map")
    }

    fn visit_unit<E>(self) -> Result<BTreeMap<String, V>, E>
        where E: SerdeError {
        Ok(BTreeMap::new())
    }

    fn visit_map<Visitor>(self, mut visitor: Visitor) -> Result<Self::Value, Visitor::Error>
        where Visitor: MapAccess<'de> {
        let mut values = BTreeMap::new();

        while let Some((key, value)) = try!(visitor.next_entry()) {
            let key: String = key;
            values.insert(capitalize_first(key.as_str()), value);
        }

        Ok(values)
    }
}

impl<'de, V> ShapesMap<'de> for BTreeMap<String, V>
    where String: Deserialize<'de>,
          V: Deserialize<'de> {
    fn deserialize_shapes_map<D>(deserializer: D) -> Result<BTreeMap<String, V>, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_map(ShapesMapVisitor::new())
    }
}
