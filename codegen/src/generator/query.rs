use inflector::Inflector;
use botocore::{Operation, Service, Shape, ShapeType};
use std::collections::HashSet;

use super::xml_response_parser::{generate_response_parser, generate_deserializer};
use super::{GenerateProtocol, error_type_name, generate_field_name};

pub struct QueryGenerator {
    deserialized_types: HashSet<String>,
    serialized_types: HashSet<String>
}

impl QueryGenerator {
    pub fn new(service: &Service) -> QueryGenerator {
        let mut deserialized_types:HashSet<String> = HashSet::new();
        let mut serialized_types:HashSet<String> = HashSet::new();
        for operation in service.operations.values() {
            if let Some(ref output) = operation.output {
                recurse_find_shapes(service, &mut deserialized_types, &output.shape);
            }
            if let Some(ref input) = operation.input {
                recurse_find_shapes(service, &mut serialized_types, &input.shape);
            }
        }

        println!("{:#?}", deserialized_types);
        println!("{:#?}", serialized_types);

        QueryGenerator {
            deserialized_types: deserialized_types,
            serialized_types: serialized_types
        }
    }
}

fn recurse_find_shapes(service: &Service, types: &mut HashSet<String>, shape_name: &str) {
    types.insert(shape_name.to_owned());
    let shape = service.shapes.get(shape_name).expect("Shape type missing from service definition");
    match shape.shape_type {
        ShapeType::Structure => {
            if let Some(ref members) = shape.members {
                for (_, member) in members {
                    //let member_shape = service.shapes.get(member.shape).expect("Shape type missing from service definition");
                    recurse_find_shapes(service, types, &member.shape);
                }
            }
        },
        ShapeType::Map => {
            recurse_find_shapes(service, types, shape.key_type());
            recurse_find_shapes(service, types, shape.value_type());
        },
        ShapeType::List => {
            recurse_find_shapes(service, types, shape.member_type());
        }
        _ => {}
    }
}

impl GenerateProtocol for QueryGenerator {
    fn generate_methods(&self, service: &Service) -> String {
        service.operations.iter().map(|(operation_name, operation)| {
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
                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {{
                        StatusCode::Ok => {{
                            {parse_response}
                        }}
                        _ => {{
                            Err({error_type}::from_body(&response.body))
                        }}
                    }}
                }}
                ",
                api_version = &service.metadata.api_version,
                documentation = generate_documentation(operation),
                error_type = error_type_name(operation_name),
                http_method = &operation.http.method,
                endpoint_prefix = &service.metadata.endpoint_prefix,
                parse_response = generate_response_parser(service, operation),
                method_signature = generate_method_signature(operation_name, operation),
                operation_name = &operation.name,
                request_uri = &operation.http.request_uri,
                serialize_input = generate_method_input_serialization(operation)
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, _: &Service) -> String {
        "use std::str::FromStr;
        use xml::EventReader;
        use xml::reader::ParserConfig;
        use xml::reader::events::XmlEvent;
        use std::collections::HashMap;
        use param::{Params, ServiceParams};
        use signature::SignedRequest;
        use xmlutil::{Next, Peek, XmlParseError, XmlResponse};
        use xmlutil::{characters, end_element, peek_at_name, start_element, skip_tree};
        use xmlerror::*;

        enum DeserializerNext {
            Close,
            Skip,
            Element(String),
        }
        ".to_owned()
    }

    fn generate_struct_attributes(&self, _struct_name: &str) -> String {
        "#[derive(Debug, Default)]".to_owned()
    }

    fn generate_support_types(&self, name: &str, shape: &Shape, service: &Service) -> Option<String> {
        let mut struct_collector = String::new();

        if self.serialized_types.contains(name) {
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
            serializer_body = generate_serializer_body(shape))
            );
        }

        if self.deserialized_types.contains(name) {
            struct_collector.push_str(&generate_deserializer(name, shape, service));
        }
        Some(struct_collector)
    }

    fn timestamp_type(&self) -> &'static str {
        "String"
    }
}

fn generate_documentation(operation: &Operation) -> String {
    match operation.documentation {
        Some(ref docs) => {
            format!("#[doc=\"{}\"]",
                    docs.replace("\\", "\\\\").replace("\"", "\\\""))
        }
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

fn generate_serializer_body(shape: &Shape) -> String {
    match shape.shape_type {
        ShapeType::List => generate_list_serializer(shape),
        ShapeType::Map => generate_map_serializer(shape),
        ShapeType::Structure => generate_struct_serializer(shape),
        _ => generate_primitive_serializer(shape),
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
