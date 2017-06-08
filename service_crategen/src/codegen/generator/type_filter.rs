use std::collections::BTreeSet;

use ::Service;
use codegen::botocore::{ShapeType, Shape};
use super::mutate_type_name;

pub fn filter_types(service: &Service) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut deserialized_types: BTreeSet<String> = BTreeSet::new();
    let mut serialized_types: BTreeSet<String> = BTreeSet::new();
    for operation in service.operations().values() {
        if let Some(ref output) = operation.output {

            let output_shape = service.get_shape(&output.shape)
                .expect("Shape type missing from service definition");

            if !can_skip_deserializer(service, output_shape) {
                recurse_find_shapes(service, &mut deserialized_types, &output.shape);
            }
        }
        if let Some(ref input) = operation.input {
            recurse_find_shapes(service, &mut serialized_types, &input.shape);
        }
    }

    (serialized_types, deserialized_types)
}

fn recurse_find_shapes(service: &Service, types: &mut BTreeSet<String>, shape_name: &str) {
    types.insert(mutate_type_name(shape_name).to_owned());
    let shape = service.get_shape(shape_name).expect("Shape type missing from service definition");
    match shape.shape_type {
        ShapeType::Structure => {
            if let Some(ref members) = shape.members {
                for member in members.values() {
                    if Some(true) != member.deprecated &&
                       member.location != Some("header".to_owned()) &&
                       member.location != Some("headers".to_owned()) &&
                       !types.contains(&member.shape) {
                        recurse_find_shapes(service, types, &member.shape);
                    }
                }
            }
        }
        ShapeType::Map => {
            recurse_find_shapes(service, types, shape.key_type());
            recurse_find_shapes(service, types, shape.value_type());
        }
        ShapeType::List => {
            recurse_find_shapes(service, types, shape.member_type());
        }
        _ => {}
    }
}

// output shapes with a payload blob don't get deserialized
// unless they have non-payload elements from the headers etc.
fn can_skip_deserializer(service: &Service, output_shape: &Shape) -> bool {
    match output_shape.payload {
        None => false,
        Some(ref payload_member) => {
            let payload_shape_type = &output_shape.members.as_ref().unwrap()[payload_member].shape;
            let payload_shape = service.get_shape(payload_shape_type).unwrap();

            let has_streaming_payload = payload_shape.shape_type == ShapeType::Blob ||
                                        payload_shape.shape_type == ShapeType::String;
            let mut has_other_members = false;

            for member in output_shape.members.as_ref().unwrap().values() {
                if member.location.is_some() {
                    has_other_members = true;
                    break;
                }
            }

            has_streaming_payload && !has_other_members
        }
    }
}
