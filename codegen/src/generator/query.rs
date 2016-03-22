use inflector::Inflector;

use botocore::{Member, Service, Shape};
use super::GenerateProtocol;

pub struct QueryGenerator;

impl GenerateProtocol for QueryGenerator {
    fn generate_methods(&self, service: &Service) -> String {
        service.operations.values().map(|_operation| {
            format!(
                ""
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, _service: &Service) -> String {
        "use std::collections::HashMap;
        use std::str::{FromStr, from_utf8};

        use xml::EventReader;

        use credentials::ProvideAWSCredentials;
        use error::AWSError;
        use params::{Params, SQSParams};
        use regions::Region;
        use signature::SignedRequest;
        use xmlutil::{Next, Peek, XmlParseError, XmlResponseFromAws};
        use xmlutil::{characters, end_element, peek_at_name, start_element};
        ".to_owned()
    }

    fn generate_struct_attributes(&self) -> String {
        "#[derive(Debug, Default)]".to_owned()
    }

    fn generate_support_types(&self, name: &str, shape: &Shape) -> Option<String> {
        Some(format!(
            "/// Deserializes {name} from XML.
            struct {name}Deserializer;
            impl {name}Deserializer {{
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<{name}, XmlParseError> {{
                    {deserializer_body}
                }}
            }}

            /// Serialize {name} contents to a `SignedRequest`.
            struct {name}Serializer;
            impl {name}Serializer {{
                {serializer_signature} {{
                    {serializer_body}
                }}
            }}
            ",
            deserializer_body = generate_deserializer_body(name, shape),
            name = name,
            serializer_body = generate_serializer_body(shape),
            serializer_signature = generate_serializer_signature(name, shape),
        ))
    }
}

fn generate_deserializer_body(name: &str, shape: &Shape) -> String {
    match &shape.shape_type[..] {
        "list" => generate_list_deserializer(shape),
        "map" => generate_map_deserializer(shape),
        "structure" => generate_struct_deserializer(name, shape),
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
        "string" => "try!(characters(stack))",
        "timestamp" => "try!(characters(stack))",
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

fn generate_struct_deserializer(name: &str, shape: &Shape) -> String {
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

        try!(end_element ( tag_name , stack ));

        Ok(obj)
        ",
        name = name,
        struct_field_deserializers = generate_struct_field_deserializers(shape),
    )
}

fn generate_struct_field_deserializers(shape: &Shape) -> String {
    shape.members.as_ref().unwrap().iter().map(|(member_name, member)| {
        format!(
            "\"{member_name}\" => {{
                obj.{field_name} = {parse_expression};
                continue;
            }}",
            field_name = member_name.to_snake_case(),
            parse_expression = generate_struct_field_parse_expression(shape, member_name, member),
            member_name = member_name,
        )
    }).collect::<Vec<String>>().join("\n")
}

fn generate_struct_field_parse_expression(
    shape: &Shape,
    member_name: &String,
    member: &Member,
) -> String {
    let expression = format!(
        "try!({name}Deserializer::deserialize(\"{member_name}\", stack))",
        name = member.shape,
        member_name = member_name,
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
    let key = format!(\"{{}}.{{}}\", name, index);
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
                tag_name = member.tag_name(),
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
