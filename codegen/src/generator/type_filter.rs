use std::collections::HashSet;
use botocore::{Service, ShapeType};
use super::mutate_type_name;

pub fn filter_types(service: &Service) -> (HashSet<String>, HashSet<String>) {
    let mut deserialized_types: HashSet<String> = HashSet::new();
    let mut serialized_types: HashSet<String> = HashSet::new();
    for operation in service.operations.values() {
        if let Some(ref output) = operation.output {

            let output_shape = service.shapes.get(&output.shape).expect("Shape type missing from service definition");

            // output shapes with a payload blob don't get deserialized
            let streaming = match output_shape.payload {
                None => false,
                Some(ref payload_member) => {
                    let payload_shape = &service.shapes[payload_member];
                    payload_shape.shape_type == ShapeType::Blob || payload_shape.shape_type == ShapeType::String
                }
            };

            if !streaming {
                recurse_find_shapes(service, &mut deserialized_types, &output.shape);
            }
        }
        if let Some(ref input) = operation.input {
            recurse_find_shapes(service, &mut serialized_types, &input.shape);
        }
    }

    println!("----------------------\n{:#?}\n----------------------------\n", deserialized_types);

    (serialized_types, deserialized_types)
}

fn recurse_find_shapes(service: &Service, types: &mut HashSet<String>, shape_name: &str) {
    types.insert(mutate_type_name(shape_name).to_owned());
    let shape = service.shapes.get(shape_name).expect("Shape type missing from service definition");
    match shape.shape_type {
        ShapeType::Structure => {
            if let Some(ref members) = shape.members {
                for member in members.values() {
                    if member.location != Some("header".to_owned()) && !types.contains(&member.shape) {
                        recurse_find_shapes(service, types, &member.shape);
                    }
                }
            }
        }
        ShapeType::Map => {
            println!("{} -> {}", mutate_type_name(shape_name), mutate_type_name(shape.key_type()));
            recurse_find_shapes(service, types, shape.key_type());
            println!("{} -> {}", mutate_type_name(shape_name), mutate_type_name(shape.value_type()));
            recurse_find_shapes(service, types, shape.value_type());
        }
        ShapeType::List => {
            println!("{} -> {}", mutate_type_name(shape_name), mutate_type_name(shape.member_type()));
            recurse_find_shapes(service, types, shape.member_type());
        }
        _ => {}
    }
}
