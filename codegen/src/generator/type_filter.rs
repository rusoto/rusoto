use std::collections::HashSet;
use botocore::{Service, ShapeType};
use super::mutate_type_name;

pub fn filter_types(service: &Service) -> (HashSet<String>, HashSet<String>) {
    let mut deserialized_types: HashSet<String> = HashSet::new();
    let mut serialized_types: HashSet<String> = HashSet::new();
    for operation in service.operations.values() {
        if let Some(ref output) = operation.output {
            recurse_find_shapes(service, &mut deserialized_types, &output.shape);
        }
        if let Some(ref input) = operation.input {
            recurse_find_shapes(service, &mut serialized_types, &input.shape);
        }
    }

    (serialized_types, deserialized_types)
}

fn recurse_find_shapes(service: &Service, types: &mut HashSet<String>, shape_name: &str) {
    types.insert(mutate_type_name(shape_name).to_owned());
    let shape = service.shapes.get(shape_name).expect("Shape type missing from service definition");
    match shape.shape_type {
        ShapeType::Structure => {
            if let Some(ref members) = shape.members {
                for member in members.values() {
                    if !types.contains(&member.shape) {
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
