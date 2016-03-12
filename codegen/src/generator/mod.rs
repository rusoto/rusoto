use botocore::{Service, Shape};

pub use self::json::JsonGenerator;

mod json;

pub trait GenerateProtocol {
    fn generate_methods(&self, service: &Service) -> String;
}

pub fn generate<P>(service: &Service, protocol_generator: P) -> String where P: GenerateProtocol {
    format!("use std::io::Read;
use std::result;

use serde_json;

use credentials::ProvideAWSCredentials;
use error::AWSError;
use regions::Region;
use signature::SignedRequest;

#[derive(Debug, Default, Deserialize)]
pub struct {error_type_name} {{
    __type: String,
    message: String,
}}

pub type Result<T> = result::Result<T, {error_type_name}>;

impl From<AWSError> for {error_type_name} {{
    fn from(err: AWSError) -> Self {{
        let AWSError(message) = err;

        {error_type_name} {{
            __type: \"Unknown\".to_string(),
            message: message.to_string(),
        }}
    }}
}}

fn parse_error(body: &str) -> {error_type_name} {{
    if let Ok(decoded) = serde_json::from_str::<{error_type_name}>(&body) {{
        decoded
    }} else {{
        {error_type_name} {{
            __type: \"DecodeError\".to_string(),
            message: body.to_string(),
        }}
    }}
}}

{types}

{client}",
        client = generate_client(service, protocol_generator),
        error_type_name = service.error_type_name(),
        types = generate_types(service),
    )
}

fn generate_client<P>(service: &Service, protocol_generator: P) -> String
where P: GenerateProtocol {
    format!("
pub struct {type_name}<'a> {{
    credentials_provider: Box<ProvideAWSCredentials + 'a>,
    region: &'a Region,
}}

impl<'a> {type_name}<'a> {{
    pub fn new<P>(
        credentials_provider: P,
        region: &'a Region,
    ) -> Self where P: ProvideAWSCredentials + 'a {{
        {type_name} {{
            credentials_provider: Box::new(credentials_provider),
            region: region,
        }}
    }}

    {methods}
}}",
        methods = protocol_generator.generate_methods(service),
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
        "double" => "f64",
        "float" => "f32",
        "integer" => "i32",
        "long" => "i64",
        "string" => "String",
        "timestamp" => "f64",
        primitive_type => panic!("Unknown primitive type: {}", primitive_type),
    };

    format!("pub type {} = {};", name, primitive_type)
}

fn generate_types(service: &Service) -> String {
    service.shapes.iter().filter_map(|(name, shape)| {
        if name == "String" {
            return None;
        }

        match &shape.shape_type[..] {
            "structure" => Some(generate_struct(name, shape)),
            "map" => Some(generate_map(name, shape)),
            "list" => Some(generate_list(name, shape)),
            shape_type => Some(generate_primitive_type(name, shape_type)),
        }
    }).collect::<Vec<String>>().join("\n")
}

fn generate_struct(name: &String, shape: &Shape) -> String {
    if shape.members.is_none() || shape.members.as_ref().unwrap().is_empty() {
        format!("#[derive(Debug, Default, Deserialize, Serialize)]\npub struct {};", name)
    } else {
        format!("#[derive(Debug, Default, Deserialize, Serialize)]
pub struct {name} {{
{struct_fields}
}}",
            name = name,
            struct_fields = generate_struct_fields(shape),
        )
    }

}

fn generate_struct_fields(shape: &Shape) -> String {
    shape.members.as_ref().unwrap().iter().map(|(member_name, member)| {
        let mut lines = Vec::with_capacity(2);

        if let Some(ref docs) = member.documentation {
            lines.push(format!("#[doc=\"{}\"]", docs.replace("\"", "\\\"")));
        }

        if shape.required(member_name) {
            lines.push(format!("pub {}: {},",  member_name, member.shape));
        } else if member_name == "type" {
            lines.push(format!("pub aws_{}: Option<{}>,",  member_name, member.shape));
        } else {
            lines.push(format!("pub {}: Option<{}>,",  member_name, member.shape));
        }

        lines.join("\n")
    }).collect::<Vec<String>>().join("\n")
}
