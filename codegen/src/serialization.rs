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

use serde::{Deserialize, Deserializer, Error as SerdeError};
use serde::de::{Visitor, MapVisitor};

use generator::capitalize_first;

pub trait ShapeName: Sized {
    fn deserialize_shape_name<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer;
}

struct ShapeNameVisitor;

impl Visitor for ShapeNameVisitor {
    type Value = String;

    fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E>
        where E: SerdeError {
        Ok(capitalize_first(v))
    }
}

impl ShapeName for String {
    fn deserialize_shape_name<D>(deserializer: &mut D) -> Result<String, D::Error>
        where D: Deserializer {
        deserializer.deserialize_string(ShapeNameVisitor)
    }
}

pub trait ShapesMap: Sized {
    fn deserialize_shapes_map<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer;
}

struct ShapesMapVisitor<V> {
    marker: PhantomData<BTreeMap<String, V>>,
}

impl<V> ShapesMapVisitor<V> {
    pub fn new() -> Self {
        ShapesMapVisitor { marker: PhantomData }
    }
}

impl<V> Visitor for ShapesMapVisitor<V>
    where V: Deserialize {
    type Value = BTreeMap<String, V>;

    fn visit_unit<E>(&mut self) -> Result<BTreeMap<String, V>, E>
        where E: SerdeError {
        Ok(BTreeMap::new())
    }

    fn visit_map<Visitor>(&mut self, mut visitor: Visitor) -> Result<Self::Value, Visitor::Error>
        where Visitor: MapVisitor {
        let mut values = BTreeMap::new();

        while let Some((key, value)) = try!(visitor.visit()) {
            let key: String = key;
            values.insert(capitalize_first(key.as_str()), value);
        }

        try!(visitor.end());

        Ok(values)
    }
}

impl<V> ShapesMap for BTreeMap<String, V>
    where String: Deserialize,
          V: Deserialize {
    fn deserialize_shapes_map<D>(deserializer: &mut D) -> Result<BTreeMap<String, V>, D::Error>
        where D: Deserializer {
        deserializer.deserialize(ShapesMapVisitor::new())
    }
}
