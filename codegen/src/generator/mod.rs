use inflector::Inflector;

use botocore::{Service, Shape, Operation};

use self::json::JsonGenerator;
use self::query::QueryGenerator;
use self::rest_json::RestJsonGenerator;

use std::collections::HashMap;

mod json;
mod query;
mod rest_json;

pub trait GenerateProtocol {
    fn generate_methods(&self, service: &Service) -> String;

    fn generate_prelude(&self, service: &Service) -> String;

    fn generate_struct_attributes(&self) -> String;

    fn generate_support_types(&self, _name: &str, _shape: &Shape, _service: &Service)
        -> Option<String> {
        None
    }
}

pub fn generate_source(service: &Service) -> String {
    match &service.metadata.protocol[..] {
        "json" => generate(service, JsonGenerator),
        "query" => generate(service, QueryGenerator),
        "rest-json" => generate(service, RestJsonGenerator),
        protocol => panic!("Unknown protocol {}", protocol),
    }
}

fn generate<P>(service: &Service, protocol_generator: P) -> String where P: GenerateProtocol {
    format!(
        "{prelude}

        {types}
        {error_types}

        {client}",
        client = generate_client(service, &protocol_generator),
        prelude = &protocol_generator.generate_prelude(service),
        types = generate_types(service, &protocol_generator),
        error_types = generate_error_types(service).unwrap_or("".to_string()),
    )
}

fn generate_client<P>(service: &Service, protocol_generator: &P) -> String
where P: GenerateProtocol {
    format!(
        "/// A client for the {service_name} API.
        pub struct {type_name}<P> where P: ProvideAwsCredentials {{
            credentials_provider: P,
            region: Region,
        }}

        impl<P> {type_name}<P> where P: ProvideAwsCredentials {{
            pub fn new(credentials_provider: P, region: Region) -> Self {{
                {type_name} {{
                    credentials_provider: credentials_provider,
                    region: region,
                }}
            }}

            {methods}
        }}
        ",
        methods = protocol_generator.generate_methods(service),
        service_name = match &service.metadata.service_abbreviation {
            &Some(ref service_abbreviation) => service_abbreviation.as_str(),
            &None => {
                match service.metadata.endpoint_prefix {
                    ref x if x == "elastictranscoder" => "Amazon Elastic Transcoder",
                    _ => panic!("Unable to determine service abbreviation"),
                }
            },
        },
        type_name = service.client_type_name(),
    )
}

fn generate_list(name: &str, shape: &Shape) -> String {
    format!("pub type {} = Vec<{}>;", name, shape.member())
}

fn generate_map(name: &str, shape: &Shape) -> String {
    format!(
        "pub type {} = ::std::collections::HashMap<{}, {}>;",
        name,
        shape.key(),
        shape.value(),
    )
}

fn generate_primitive_type(name: &str, shape_type: &str) -> String {
    let primitive_type = match shape_type {
        "blob" => "Vec<u8>",
        "boolean" => "bool",
        "double" | "timestamp" => "f64",
        "float" => "f32",
        "integer" => "i32",
        "long" => "i64",
        "string" => "String",
        primitive_type => panic!("Unknown primitive type: {}", primitive_type),
    };

    format!("pub type {} = {};", name, primitive_type)
}

fn generate_types<P>(service: &Service, protocol_generator: &P) -> String
where P: GenerateProtocol {
    service.shapes.iter().filter_map(|(name, shape)| {
        if name == "String" {
            return protocol_generator.generate_support_types(name, shape, &service);
        }

        if shape.exception() && service.typed_errors() {
            return None;
        }

        let mut parts = Vec::with_capacity(3);

        if let Some(ref docs) = shape.documentation {
            parts.push(format!("#[doc=\"{}\"]", docs.replace("\"", "\\\"")));
        }

        match &shape.shape_type[..] {
            "structure" => parts.push(generate_struct(service, name, shape, protocol_generator)),
            "map" => parts.push(generate_map(name, shape)),
            "list" => parts.push(generate_list(name, shape)),
            shape_type => parts.push(generate_primitive_type(name, shape_type)),
        }

        if let Some(support_types) = protocol_generator.generate_support_types(name, shape, &service) {
            parts.push(support_types);
        }

        Some(parts.join("\n"))
    }).collect::<Vec<String>>().join("\n")
}

fn generate_error_types(service: &Service) -> Option<String> {
    if service.typed_errors() {

        // grab error type documentation for use with error enums in generated code
        // botocore presents errors as structs.  we filter those out in generate_types.
        let mut error_documentation = HashMap::new();

        for (name, shape) in service.shapes.iter() {
            if shape.exception() && shape.documentation.is_some() {
                error_documentation.insert(name, shape.documentation.as_ref().unwrap());
            }
        }

       Some(service.operations.iter()
        .filter_map(|(_, operation)| generate_error_type(operation, &error_documentation) )
        .collect::<Vec<String>>()
        .join("\n")
        )
    } else {
       None
    }
}

fn generate_struct<P>(
    service: &Service,
    name: &str,
    shape: &Shape,
    protocol_generator: &P,
) -> String where P: GenerateProtocol {
    if shape.members.is_none() || shape.members.as_ref().unwrap().is_empty() {
        format!(
            "{attributes}
            pub struct {name};
            ",
            attributes = protocol_generator.generate_struct_attributes(),
            name = name,
        )
    } else {
        format!(
            "{attributes}
            pub struct {name} {{
                {struct_fields}
            }}
            ",
            attributes = protocol_generator.generate_struct_attributes(),
            name = name,
            struct_fields = generate_struct_fields(service, shape),
        )
    }

}

fn generate_struct_fields(service: &Service, shape: &Shape) -> String {
    shape.members.as_ref().unwrap().iter().map(|(member_name, member)| {
        let mut lines = Vec::with_capacity(4);
        let name = member_name.to_snake_case();

        if let Some(ref docs) = member.documentation {
            lines.push(format!("#[doc=\"{}\"]", docs.replace("\"", "\\\"")));
        }

        lines.push("#[allow(unused_attributes)]".to_owned());
        lines.push(format!("#[serde(rename=\"{}\")]", member_name));

        if let Some(shape_type) = service.shape_type_for_member(member) {
            if shape_type == "blob" {
                lines.push(
                    "#[serde(
                        deserialize_with=\"::serialization::SerdeBlob::deserialize_blob\",
                        serialize_with=\"::serialization::SerdeBlob::serialize_blob\",
                    )]".to_owned()
                );
            }
        }

        if shape.required(member_name) {
            lines.push(format!("pub {}: {},",  name, member.shape));
        } else if name == "type" {
            lines.push(format!("pub aws_{}: Option<{}>,",  name, member.shape));
        } else {
            lines.push(format!("pub {}: Option<{}>,",  name, member.shape));
        }

        lines.join("\n")
    }).collect::<Vec<String>>().join("\n")
}

impl Operation {
    pub fn error_type_name(&self) -> String {
        format!("{}Error", self.name)
    }
}

pub fn generate_error_type(operation: &Operation, error_documentation: &HashMap<&String, &String>) -> Option<String> {

    let error_type_name = operation.error_type_name();

    Some(format!("
        #[derive(Debug, PartialEq)]
        pub enum {type_name} {{
            {error_types}
        }}

        impl {type_name} {{
            pub fn from_body(body: &str) -> {type_name} {{
                match from_str::<Value>(body) {{
                    Ok(json) => {{
                        let error_type: &str = match json.find(\"__type\") {{
                            Some(error_type) => error_type.as_string().unwrap_or(\"Unknown\"),
                            None => \"Unknown\",
                        }};

                        match error_type {{
                            {type_matchers}
                        }}
                    }},
                    Err(_) => {type_name}::Unknown(String::from(body))
                }}
            }}
        }}
        impl From<AwsError> for {type_name} {{
            fn from(err: AwsError) -> {type_name} {{
                {type_name}::Unknown(err.message)
            }}
        }}
        impl fmt::Display for {type_name} {{
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
                write!(f, \"{{}}\", self.description())
            }}
        }}
        impl Error for {type_name} {{
            fn description(&self) -> &str {{
               match *self {{
                   {description_matchers}
               }}
           }}
       }}
       ",
       type_name = error_type_name,
       error_types = generate_error_enum_types(operation, error_documentation).unwrap_or(String::from("")),
       type_matchers = generate_error_type_matchers(operation).unwrap_or(String::from("")),
       description_matchers = generate_error_description_matchers(operation).unwrap_or(String::from(""))))
    }

fn generate_error_enum_types(operation: &Operation, error_documentation: &HashMap<&String, &String>) -> Option<String> {
    let mut enum_types: Vec<String> = Vec::new();

    if operation.errors.is_some() {
        for error in operation.errors.as_ref().unwrap().iter() {
            enum_types.push(format!("\n///{}\n{}(String)",
                error_documentation.get(&error.shape).unwrap_or(&&String::from("")),
                error.idiomatic_error_name()));
        }
    }

    enum_types.push("/// A validation error occurred.  Details from AWS are provided.\nValidation(String)".to_string());
    enum_types.push("/// An unknown error occurred.  The raw HTTP response is provided.\nUnknown(String)".to_string());
    Some(enum_types.join(","))
}

fn generate_error_type_matchers(operation: &Operation) -> Option<String> {
    let mut type_matchers: Vec<String> = Vec::new();
    let error_type = operation.error_type_name();

    if operation.errors.is_some() {
        for error in operation.errors.as_ref().unwrap().iter() {
            type_matchers.push(format!("\"{error_shape}\" => {error_type}::{error_name}(String::from(body))",
                error_shape = error.shape,
                error_type = error_type,
                error_name = error.idiomatic_error_name()))
        }
    }

   type_matchers.push(format!("\"Validation\" => {error_type}::Validation(String::from(body))", error_type = error_type));
   type_matchers.push(format!("_ => {error_type}::Unknown(String::from(body))",  error_type = error_type));
   Some(type_matchers.join(","))
}

fn generate_error_description_matchers(operation: &Operation) -> Option<String> {
    let mut type_matchers: Vec<String> = Vec::new();
    let error_type = operation.error_type_name();

    if operation.errors.is_some() {
        for error in operation.errors.as_ref().unwrap().iter() {
            type_matchers.push(format!("{error_type}::{error_shape}(ref cause) => cause",
                error_type = operation.error_type_name(),
                error_shape = error.idiomatic_error_name()))
        }
    }

   type_matchers.push(format!("{error_type}::Validation(ref cause) => cause", error_type = error_type));
   type_matchers.push(format!("{error_type}::Unknown(ref cause) => cause", error_type = error_type));
   Some(type_matchers.join(","))
}

