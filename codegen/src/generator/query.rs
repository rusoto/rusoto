use inflector::Inflector;

use botocore::{Member, Operation, Service, Shape};
use super::GenerateProtocol;

pub struct QueryGenerator;

impl GenerateProtocol for QueryGenerator {
    fn generate_methods(&self, service: &Service) -> String {
        service.operations.values().map(|operation| {
            format!(
                "
                {documentation}
                {method_signature} {{
                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, \"{request_uri}\");
                    let mut params = Params::new();

                    params.put(\"Action\", \"{operation_name}\");
                    {serialize_input}
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let result = try!(self.dispatcher.dispatch(&request));

                    let mut reader = EventReader::from_str(&result.body);
                    let mut stack = XmlResponse::new(reader.events().peekable());

                    let _start_document = stack.next();
                    let _response_envelope = stack.next();

                    match result.status {{
                        200 => {{
                            {method_return_value}
                        }}
                        _ => {{
                            Err({error_type}::from_body(&result.body))
                        }}
                    }}

                }}
                ",
                documentation = generate_documentation(operation),
                error_type = operation.error_type_name(),
                http_method = &operation.http.method,
                endpoint_prefix = &service.metadata.endpoint_prefix,
                method_return_value = generate_method_return_value(operation),
                method_signature = generate_method_signature(operation),
                operation_name = &operation.name,
                request_uri = &operation.http.request_uri,
                serialize_input = generate_method_input_serialization(operation),
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, _: &Service) -> String {
        "use std::collections::HashMap;
        use std::str::{FromStr, from_utf8};
        use xml::EventReader;

        use param::{Params, ServiceParams};
        use signature::SignedRequest;
        use xmlutil::{Next, Peek, XmlParseError, XmlResponse};
        use xmlutil::{characters, end_element, peek_at_name, start_element};
        use xmlerror::*;
        ".to_owned()
    }

    fn generate_struct_attributes(&self) -> String {
        "#[derive(Debug, Default, Clone)]".to_owned()
    }

    fn generate_support_types(&self, name: &str, shape: &Shape, service: &Service) -> Option<String> {
        Some(format!(
            "/// Deserializes `{name}` from XML.
            struct {name}Deserializer;
            impl {name}Deserializer {{
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<{name}, XmlParseError> {{
                    {deserializer_body}
                }}
            }}

            /// Serialize `{name}` contents to a `SignedRequest`.
            struct {name}Serializer;
            impl {name}Serializer {{
                {serializer_signature} {{
                    {serializer_body}
                }}
            }}
            ",
            deserializer_body = generate_deserializer_body(name, shape, service),
            name = name,
            serializer_body = generate_serializer_body(shape),
            serializer_signature = generate_serializer_signature(name, shape),
        ))
    }

    fn timestamp_type(&self) -> &'static str {
        "String"
    }
}

fn generate_documentation(operation: &Operation) -> String {
    match operation.documentation {
        Some(ref docs) => format!("#[doc=\"{}\"]", docs.replace("\"", "\\\"")),
        None => "".to_owned(),
    }
}

fn generate_method_input_serialization(operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "{input_type}Serializer::serialize(&mut params, \"\", &input);",
            input_type = operation.input.as_ref().unwrap().shape,
        )
    } else {
        String::new()
    }
}

fn generate_method_return_value(operation: &Operation) -> String {
    if operation.output.is_some() {
        format!(
            "Ok(try!({output_type}Deserializer::deserialize(\"{output_type}\", &mut stack)))",
            output_type = &operation.output.as_ref().unwrap().shape,
        )
    } else {
        "Ok(())".to_owned()
    }
}

fn generate_method_signature(operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "pub fn {operation_name}(&self, input: &{input_type}) -> Result<{output_type}, {error_type}>",
            input_type = operation.input.as_ref().unwrap().shape,
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = operation.error_type_name(),
        )
    } else {
        format!(
            "pub fn {operation_name}(&self) -> Result<{output_type}, {error_type}>",
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = operation.error_type_name(),
        )
    }
}

fn generate_deserializer_body(name: &str, shape: &Shape, service: &Service) -> String {
    match &shape.shape_type[..] {
        "list" => generate_list_deserializer(shape),
        "map" => generate_map_deserializer(shape),
        "structure" => generate_struct_deserializer(name, shape, service),
        _ => generate_primitive_deserializer(shape),
    }
}

fn generate_list_deserializer(shape: &Shape) -> String {
    format!(
        "
        let mut obj = vec![];

        while try!(peek_at_name(stack)) == tag_name {{
            obj.push(try!({member_name}Deserializer::deserialize(tag_name, stack)));
        }}

        Ok(obj)
        ",
        member_name = shape.member()
    )
}

fn generate_map_deserializer(shape: &Shape) -> String {
    let key = shape.key.as_ref().unwrap();
    let value = shape.value.as_ref().unwrap();

    format!(
        "
        let mut obj = HashMap::new();

        while try!(peek_at_name(stack)) == tag_name {{
            try!(start_element(tag_name, stack));
            let key = try!({key_type_name}Deserializer::deserialize(\"{key_tag_name}\", stack));
            let value = try!({value_type_name}Deserializer::deserialize(\"{value_tag_name}\", stack));
            obj.insert(key, value);
            try!(end_element(tag_name, stack));
        }}

        Ok(obj)
        ",
        key_tag_name = key.tag_name(),
        key_type_name = key.shape,
        value_tag_name = value.tag_name(),
        value_type_name = value.shape,
    )
}

fn generate_primitive_deserializer(shape: &Shape) -> String {
    let statement =  match &shape.shape_type[..] {
        "string" | "timestamp" => "try!(characters(stack))",
        "integer" => "i32::from_str(try!(characters(stack)).as_ref()).unwrap()",
        "double" => "f32::from_str(try!(characters(stack)).as_ref()).unwrap()",
        "blob" => "try!(characters(stack)).into_bytes()",
        "boolean" => "bool::from_str(try!(characters(stack)).as_ref()).unwrap()",
        shape_type => panic!("Unknown primitive shape type: {}", shape_type),
    };

    format!(
        "try!(start_element(tag_name, stack));
        let obj = {statement};
        try!(end_element(tag_name, stack));

        Ok(obj)
        ",
        statement = statement,
    )
}

fn generate_struct_deserializer(name: &str, shape: &Shape, service: &Service) -> String {
    if shape.members.as_ref().unwrap().is_empty() {
        return format!(
            "try!(start_element(tag_name, stack));

            let obj = {name}::default();

            try!(end_element(tag_name, stack));

            Ok(obj)
            ",
            name = name,
        );
    }

    format!(
        "try!(start_element(tag_name, stack));

        let mut obj = {name}::default();

        loop {{
            match &try!(peek_at_name(stack))[..] {{
                {struct_field_deserializers}
                _ => break,
            }}
        }}

        try!(end_element(tag_name, stack));

        Ok(obj)
        ",
        name = name,
        struct_field_deserializers = generate_struct_field_deserializers(shape, service),
    )
}

fn generate_struct_field_deserializers(shape: &Shape, service: &Service) -> String {
    shape.members.as_ref().unwrap().iter().map(|(member_name, member)| {
        // look up member.shape in all_shapes.  use that shape.member.location_name
        let mut location_name = member_name.to_string();

        let parse_expression_location_name = if let Some(ref child_shape) = service.shape_for_member(member) {
            if child_shape.flattened.is_some() {
                if let Some(ref child_member) = child_shape.member {
                    if let Some(ref loc_name) = child_member.location_name {
                        location_name = loc_name.to_string();
                        Some(&location_name)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        let parse_expression = generate_struct_field_parse_expression(shape, member_name, member, parse_expression_location_name);
        format!(
            "\"{location_name}\" => {{
                obj.{field_name} = {parse_expression};
                continue;
            }}",
            field_name = member_name.to_snake_case(),
            parse_expression = parse_expression,
            location_name = location_name,
        )

    }).collect::<Vec<String>>().join("\n")
}

fn generate_struct_field_parse_expression(
    shape: &Shape,
    member_name: &str,
    member: &Member,
    location_name: Option<&String>,
) -> String {

    let location_to_use = match location_name {
        Some(loc) => loc.to_string(),
        None => member_name.to_string(),
    };
    let expression = format!(
        "try!({name}Deserializer::deserialize(\"{location}\", stack))",
        name = member.shape,
        location = location_to_use,
    );

    if shape.required(member_name) {
        expression
    } else {
        format!("Some({})", expression)
    }
}

fn generate_serializer_body(shape: &Shape) -> String {
    match &shape.shape_type[..] {
        "list" => generate_list_serializer(shape),
        "map" => generate_map_serializer(shape),
        "structure" => generate_struct_serializer(shape),
        _ => generate_primitive_serializer(shape),
    }
}

fn generate_serializer_signature(name: &str, shape: &Shape) -> String {
    if &shape.shape_type[..] == "structure" && shape.members.as_ref().unwrap().is_empty() {
        format!("fn serialize(_params: &mut Params, name: &str, _obj: &{})", name)
    } else {
        format!("fn serialize(params: &mut Params, name: &str, obj: &{})", name)
    }
}

fn generate_list_serializer(shape: &Shape) -> String {
    format!(
        "for (index, element) in obj.iter().enumerate() {{
    // Lists are one-based, see example here: http://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SendMessage.html
    let key = format!(\"{{}}.{{}}\", name, index+1);
    {name}Serializer::serialize(params, &key, element);
}}
        ",
        name = shape.member(),
    )
}

fn generate_map_serializer(shape: &Shape) -> String {
    format!(
        "for (index, (key, value)) in obj.iter().enumerate() {{
    let prefix = format!(\"{{}}.{{}}\", name, index);
    {key_name}Serializer::serialize(
        params,
        &format!(\"{{}}.{{}}\", prefix, \"{key_name}\"),
        key,
    );
    {value_name}Serializer::serialize(
        params,
        &format!(\"{{}}.{{}}\", prefix, \"{value_name}\"),
        value,
    );
}}
        ",
        key_name = shape.key(),
        value_name = shape.value(),
    )
}

fn generate_struct_serializer(shape: &Shape) -> String {
    format!(
        "let mut prefix = name.to_string();
if prefix != \"\" {{
    prefix.push_str(\".\");
}}

{struct_field_serializers}
        ",
        struct_field_serializers = generate_struct_field_serializers(shape),
    )
}

fn generate_struct_field_serializers(shape: &Shape) -> String {
    shape.members.as_ref().unwrap().iter().map(|(member_name, member)| {
        if shape.required(member_name) {
            format!(
                "{member_shape_name}Serializer::serialize(
    params,
    &format!(\"{{}}{{}}\", prefix, \"{tag_name}\"),
    &obj.{field_name},
);
                ",
                field_name = member_name.to_snake_case(),
                member_shape_name = member.shape,
                tag_name = member_name,
            )
        } else {
            format!(
                "if let Some(ref field_value) = obj.{field_name} {{
    {member_shape_name}Serializer::serialize(
        params,
        &format!(\"{{}}{{}}\", prefix, \"{tag_name}\"),
        field_value,
    );
}}
                ",
                field_name = member_name.to_snake_case(),
                member_shape_name = member.shape,
                tag_name = member.tag_name(),
            )
        }
    }).collect::<Vec<String>>().join("\n")
}

fn generate_primitive_serializer(shape: &Shape) -> String {
    let expression = match &shape.shape_type[..] {
        "string" | "timestamp" => "obj",
        "integer" | "double" | "boolean" => "&obj.to_string()",
        "blob" => "from_utf8(obj).unwrap()",
        shape_type => panic!("Unknown primitive shape type: {}", shape_type),
    };

    format!("params.put(name, {});", expression)
}
