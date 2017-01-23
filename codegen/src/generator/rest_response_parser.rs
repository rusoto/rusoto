use inflector::Inflector;
use botocore::{Service, Operation, Shape, ShapeType};

pub fn generate_response_headers_parser(service: &Service, operation: &Operation) -> Option<String> {
    // nothing to do if there's no output type
    if operation.output.is_none() {
        return None;
    }

    let shape = &service.shapes[&operation.output.as_ref().unwrap().shape];
    let members = shape.members.as_ref().unwrap();

    let parser_pieces = members.iter()
        .filter_map(|(member_name, member)| {
            if member.location.is_none() || member.location.as_ref().unwrap() != "header" {
                return None;
            }

            let member_shape_name = &member.shape;
            let member_shape = &service.shapes[member_shape_name];

            if shape.required(member_name) {
                Some(format!("let value = response.headers.get(\"{location_name}\").unwrap().to_owned();
                              result.{field_name} = {primitive_parser};",
                             location_name = member.location_name.as_ref().unwrap(),
                             field_name = member_name.to_snake_case(),
                             primitive_parser = generate_header_primitive_parser(&member_shape)))
            } else {
                Some(format!("if let Some({field_name}) = response.headers.get(\"{location_name}\") {{
                                let value = {field_name}.to_owned();
                                result.{field_name} = Some({primitive_parser})
                              }};",
                             location_name = member.location_name.as_ref().unwrap(),
                             field_name = member_name.to_snake_case(),
                             primitive_parser = generate_header_primitive_parser(&member_shape)))
            }

        })
        .collect::<Vec<String>>();

    if !parser_pieces.is_empty() {
        Some(parser_pieces.join("\n"))
    } else {
        None
    }
}

/// Parse a primitive type from the response headers
fn generate_header_primitive_parser(shape: &Shape) -> String {
    let statement = match shape.shape_type {
        ShapeType::String | ShapeType::Timestamp => "value",
        ShapeType::Integer => "i32::from_str(&value).unwrap()",
        ShapeType::Long => "i64::from_str(&value).unwrap()",
        ShapeType::Double => "f64::from_str(&value).unwrap()",
        ShapeType::Float => "f32::from_str(&value).unwrap()",
        ShapeType::Boolean => "bool::from_str(&value).unwrap()",
        _ => panic!("Unknown primitive shape type"),
    };

    statement.to_string()
}