use inflector::Inflector;

use botocore::{Member, Operation, Service, Shape, ShapeType};
use super::GenerateProtocol;
use super::generate_field_name;
use super::error_type_name;

pub struct QueryGenerator;

impl GenerateProtocol for QueryGenerator {
    fn generate_methods(&self, service: &Service) -> String {
        service.operations.iter().map(|(operation_name, operation)| {
            let xml_tag = &operation.output_shape_or_wrapper_or("()");

            format!(
                "
                {documentation}
                {method_signature} {{
                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, \"{request_uri}\");
                    let mut params = Params::new();

                    params.put(\"Action\", \"{operation_name}\");
                    params.put(\"Version\", \"{api_version}\");
                    {serialize_input}
                    request.set_params(params);

                    request.sign(&try!(self.credentials_provider.credentials()));
                    let result = try!(self.dispatcher.dispatch(&request));

                    {xml_stack_loader}

                    match result.status {{
                        StatusCode::Ok => {{
                            {method_return_value}
                        }}
                        _ => {{
                            Err({error_type}::from_body(&result.body))
                        }}
                    }}
                }}
                ",
                api_version = &service.metadata.api_version,
                documentation = generate_documentation(operation),
                error_type = error_type_name(operation_name),
                http_method = &operation.http.method,
                endpoint_prefix = &service.metadata.endpoint_prefix,
                method_return_value = generate_method_return_value(operation),
                method_signature = generate_method_signature(operation_name, operation),
                operation_name = &operation.name,
                xml_stack_loader = generate_xml_stack_loader(xml_tag),
                request_uri = &operation.http.request_uri,
                serialize_input = generate_method_input_serialization(operation)
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, _: &Service) -> String {
        "use std::str::FromStr;
        use xml::EventReader;
        use xml::reader::ParserConfig;

        use param::{Params, ServiceParams};
        use signature::SignedRequest;
        use xmlutil::{Next, Peek, XmlParseError, XmlResponse};
        use xmlutil::{characters, end_element, peek_at_name, start_element};
        use xmlerror::*;
        ".to_owned()
    }

    fn generate_struct_attributes(&self, _struct_name: &str) -> String {
        "#[derive(Debug, Default, Clone)]".to_owned()
    }

    fn generate_support_types(&self, name: &str, shape: &Shape, service: &Service) -> Option<String> {
        let mut struct_collector = String::new();
        let serializer = generate_serializer_body(name, shape);

        if serializer.is_some() {
            struct_collector.push_str(&format!("
            /// Serialize `{name}` contents to a `SignedRequest`.
            struct {name}Serializer;
            impl {name}Serializer {{
                {serializer_signature} {{
                    {serializer_body}
                }}
            }}
            ",
            name = name,
            serializer_signature = generate_serializer_signature(name, shape),
            serializer_body = serializer.unwrap())
            );
        }
        struct_collector.push_str(&format!(
            "/// Deserializes `{name}` from XML.
            struct {name}Deserializer;
            impl {name}Deserializer {{
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<{name}, XmlParseError> {{
                    {deserializer_body}
                }}
            }}
            ",
            deserializer_body = generate_deserializer_body(name, shape, service),
            name = name,
        ));
        Some(struct_collector)
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

        let output_type = &operation.output.as_ref().unwrap().shape;
        let tag_name = operation.wrapper().unwrap_or(output_type);

        format!(
            "Ok(try!({output_type}Deserializer::deserialize(\"{tag_name}\", &mut stack)))",
            tag_name = tag_name,
            output_type = output_type
        )
    } else {
        "Ok(())".to_owned()
    }
}

fn generate_method_signature(operation_name: &str, operation: &Operation) -> String {
    if operation.input.is_some() {
        format!(
            "pub fn {operation_name}(&self, input: &{input_type}) -> Result<{output_type}, {error_type}>",
            input_type = operation.input.as_ref().unwrap().shape,
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = error_type_name(operation_name),
        )
    } else {
        format!(
            "pub fn {operation_name}(&self) -> Result<{output_type}, {error_type}>",
            operation_name = operation.name.to_snake_case(),
            output_type = &operation.output_shape_or("()"),
            error_type = error_type_name(operation_name),
        )
    }
}

fn generate_xml_stack_loader(output_type: &str) -> String {
    if output_type == "()" {
        "".to_owned()
    } else {
        format!(
            "let mut reader = EventReader::with_config(
                result.body.as_bytes(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.events().peekable());

            // Look through the stack for the `StartElement` `XmlEvent` for the
            // `{output_type}`. This is necessary so that we being deserializing at the
            // correct tag in the XML. This loop continues until we either encounter an
            // error or the end of the stack.
            while let Ok(name) = peek_at_name(&mut stack) {{
                if name == \"{output_type}\" || stack.peek() == None {{
                    break;
                }}

                stack.next();
            }}",
            output_type = output_type,
        )
    }
}

fn generate_deserializer_body(name: &str, shape: &Shape, service: &Service) -> String {
    match shape.shape_type {
        ShapeType::List => generate_list_deserializer(shape),
        ShapeType::Map => generate_map_deserializer(shape),
        ShapeType::Structure => generate_struct_deserializer(name, shape, service),
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
        member_name = shape.member_type()
    )
}

fn generate_map_deserializer(shape: &Shape) -> String {
    let key = shape.key.as_ref().unwrap();
    let value = shape.value.as_ref().unwrap();

    let element_tag_name;
    let entry_start_element;
    let entry_end_element;
    if let Some(true) = shape.flattened {
        element_tag_name = "tag_name";
        entry_start_element = "";
        entry_end_element = "";
    } else {
        element_tag_name = "\"entry\"";
        entry_start_element = "try!(start_element(tag_name, stack));";
        entry_end_element = "try!(end_element(tag_name, stack));";
    };

    format!(
        "
        let mut obj = ::std::collections::HashMap::new();

        {entry_start_element}
        while try!(peek_at_name(stack)) == {element_tag_name} {{
            try!(start_element({element_tag_name}, stack));
            let key = try!({key_type_name}Deserializer::deserialize(\"{key_tag_name}\", stack));
            let value = try!({value_type_name}Deserializer::deserialize(\"{value_tag_name}\", stack));
            obj.insert(key, value);
            try!(end_element({element_tag_name}, stack));
        }}
        {entry_end_element}

        Ok(obj)
        ",
        element_tag_name = element_tag_name,
        entry_start_element = entry_start_element,
        entry_end_element = entry_end_element,
        key_tag_name = key.location_name.clone().unwrap_or("key".to_string()),
        key_type_name = key.shape,
        value_tag_name = value.location_name.clone().unwrap_or("value".to_string()),
        value_type_name = value.shape,
    )
}

fn generate_primitive_deserializer(shape: &Shape) -> String {
    let statement =  match shape.shape_type {
        ShapeType::String | ShapeType::Timestamp => "try!(characters(stack))",
        ShapeType::Integer => "i32::from_str(try!(characters(stack)).as_ref()).unwrap()",
        ShapeType::Double => "f64::from_str(try!(characters(stack)).as_ref()).unwrap()",
        ShapeType::Blob => "try!(characters(stack)).into_bytes()",
        ShapeType::Boolean => "bool::from_str(try!(characters(stack)).as_ref()).unwrap()",
        shape_type => panic!("Unknown primitive shape type: {:?}", shape_type),
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
        let maybe_child_shape = service.shape_for_member(member);
        let mut location_name = member_name.to_string();
        let member_loc_name = if member.location_name.is_some() {
            member.location_name.clone().unwrap().to_string()
        } else {
            "".to_string()
        };
        let mut enter_wrapper_expression = String::new();
        let mut leave_wrapper_expression = String::new();
        let mut wrapper_location_name: Option<&str> = None;

        let parse_expression_location_name = if let Some(child_shape) = maybe_child_shape {
            if child_shape.flattened.is_some() {
                if let Some(ref child_member) = child_shape.member {
                    if let Some(ref loc_name) = child_member.location_name {
                        location_name = loc_name.to_string();
                        Some(&location_name[..])
                    } else {
                        None
                    }
                } else {
                    // assumes we'll only hit this case if a location_name is provided
                    Some(&member_loc_name[..])
                }
            } else if child_shape.shape_type == ShapeType::List {
                wrapper_location_name = Some(member_name);

                enter_wrapper_expression = format!("try!(start_element(\"{}\", stack))", member_name);
                leave_wrapper_expression = format!("try!(end_element(\"{}\", stack))", member_name);

                child_shape.location_name.as_ref().map(|s| &s[..]).or_else(|| Some("member"))
            } else {
                None
            }
        } else {
            None
        };

        let parse_expression = generate_struct_field_parse_expression(shape, member_name, member, parse_expression_location_name);
        format!(
            "\"{wrapper_location_name}\" => {{
                {enter_wrapper_expression};
                obj.{field_name} = {parse_expression};
                {leave_wrapper_expression};
                continue;
            }}",
            field_name = generate_field_name(member_name),
            parse_expression = parse_expression,
            wrapper_location_name = wrapper_location_name.or_else(|| parse_expression_location_name).unwrap_or(&location_name),
            enter_wrapper_expression = enter_wrapper_expression,
            leave_wrapper_expression = leave_wrapper_expression,
        )

    }).collect::<Vec<String>>().join("\n")
}

fn generate_struct_field_parse_expression(
    shape: &Shape,
    member_name: &str,
    member: &Member,
    location_name: Option<&str>,
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

fn generate_serializer_body(name: &str,shape: &Shape) -> Option<String> {
    // Don't need to send "Response" objects, don't make the code for their serializers
    if name.ends_with("Response") {
        return None;
    }
    match shape.shape_type {
        ShapeType::List => Some(generate_list_serializer(shape)),
        ShapeType::Map => Some(generate_map_serializer(shape)),
        ShapeType::Structure => Some(generate_struct_serializer(shape)),
        _ => Some(generate_primitive_serializer(shape)),
    }
}

fn generate_serializer_signature(name: &str, shape: &Shape) -> String {
    if shape.shape_type == ShapeType::Structure && shape.members.as_ref().unwrap().is_empty() {
        format!("fn serialize(_params: &mut Params, name: &str, _obj: &{})", name)
    } else {
        format!("fn serialize(params: &mut Params, name: &str, obj: &{})", name)
    }
}

fn generate_list_serializer(shape: &Shape) -> String {
    // List format is different for CloudWatch: http://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_PutMetricData.html
    format!("for (index, element) in obj.iter().enumerate() {{
                let key = format!(\"{{}}.member.{{}}\", name, index+1);
                {name}Serializer::serialize(params, &key, element);
            }}",
            name = shape.member_type(),
    )
}

fn generate_map_serializer(shape: &Shape) -> String {
    format!(
        "for (index, (key, value)) in obj.iter().enumerate() {{
            let prefix = format!(\"{{}}.{{}}\", name, index+1);
            {key_type}Serializer::serialize(
                params,
                &format!(\"{{}}.{{}}\", prefix, \"{key_name}\"),
                key,
            );
            {value_type}Serializer::serialize(
                params,
                &format!(\"{{}}.{{}}\", prefix, \"{value_name}\"),
                value,
            );
        }}
        ",
        key_type = shape.key_type(),
        value_type = shape.value_type(),
        key_name = shape.key_name(),
        value_name = shape.value_name()
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
                field_name = generate_field_name(member_name),
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
                }}",
                field_name = generate_field_name(member_name),
                member_shape_name = member.shape,
                tag_name = member.location_name.clone().unwrap_or(member_name.to_owned())
            )
        }
    }).collect::<Vec<String>>().join("\n")
}

fn generate_primitive_serializer(shape: &Shape) -> String {
    let expression = match shape.shape_type {
        ShapeType::String | ShapeType::Timestamp => "obj",
        ShapeType::Integer | ShapeType::Double | ShapeType::Boolean => "&obj.to_string()",
        ShapeType::Blob => "::std::str::from_utf8(obj).unwrap()",
        shape_type => panic!("Unknown primitive shape type: {:?}", shape_type),
    };

    format!("params.put(name, {});", expression)
}
