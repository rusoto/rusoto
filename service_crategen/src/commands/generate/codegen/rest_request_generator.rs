use ::Service;
use botocore::Operation;
use inflector::Inflector;

// Add request headers for any shape members marked as headers
pub fn generate_headers(service: &Service, operation: &Operation) -> Option<String> {
    if operation.input.is_none() {
        return None;
    }

    let shape = service.get_shape(&operation.input.as_ref().unwrap().shape).unwrap();

    Some(shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            if member.location.is_none() {
                return None;
            }
            match &member.location.as_ref().unwrap()[..] {
                "header" => {
                    if shape.required(member_name) {
                        Some(format!("request.add_header(\"{location_name}\", \
                                      &input.{field_name});",
                                     location_name = member.location_name.as_ref().unwrap(),
                                     field_name = member_name.to_snake_case()))
                    } else {
                        Some(format!("
                        if let Some(ref {field_name}) = \
                                      input.{field_name} {{
                            \
                                      request.add_header(\"{location_name}\", \
                                      &{field_name}.to_string());
                        }}",
                                     location_name = member.location_name.as_ref().unwrap(),
                                     field_name = member_name.to_snake_case()))
                    }
                }
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join("\n"))
}