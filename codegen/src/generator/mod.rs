use inflector::Inflector;

use botocore::{Service, Shape};

use self::json::JsonGenerator;
use self::query::QueryGenerator;

mod json;
mod query;

pub trait GenerateProtocol {
    fn generate_methods(&self, service: &Service) -> String;

    fn generate_prelude(&self, service: &Service) -> String;

    fn generate_struct_attributes(&self) -> String;

    fn generate_support_types(&self, _name: &str, _shape: &Shape) -> Option<String> {
        None
    }
}

pub fn generate_source(service: &Service) -> String {
    match &service.metadata.protocol[..] {
        "json" => generate(service, JsonGenerator),
        "query" => generate(service, QueryGenerator),
        protocol => panic!("Unknown protocol {}", protocol),
    }
}

fn generate<P>(service: &Service, protocol_generator: P) -> String where P: GenerateProtocol {
    format!(
        "{prelude}

        {types}

        {client}",
        client = generate_client(service, &protocol_generator),
        prelude = &protocol_generator.generate_prelude(service),
        types = generate_types(service, &protocol_generator),
    )
}

fn generate_client<P>(service: &Service, protocol_generator: &P) -> String
where P: GenerateProtocol {
    format!(
        "/// A client for the {service_name} API.
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
        }}
        ",
        methods = protocol_generator.generate_methods(service),
        service_name = &service.metadata.service_abbreviation,
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

fn generate_types<P>(service: &Service, protocol_generator: &P) -> String
where P: GenerateProtocol {
    service.shapes.iter().filter_map(|(name, shape)| {
        if name == "String" {
            return protocol_generator.generate_support_types(name, shape);
        }

        let mut parts = Vec::with_capacity(3);

        if let Some(ref docs) = shape.documentation {
            parts.push(format!("#[doc=\"{}\"]", docs.replace("\"", "\\\"")));
        }

        match &shape.shape_type[..] {
            "structure" => parts.push(generate_struct(name, shape, protocol_generator)),
            "map" => parts.push(generate_map(name, shape)),
            "list" => parts.push(generate_list(name, shape)),
            shape_type => parts.push(generate_primitive_type(name, shape_type)),
        }

        if let Some(support_types) = protocol_generator.generate_support_types(name, shape) {
            parts.push(support_types);
        }

        Some(parts.join("\n"))
    }).collect::<Vec<String>>().join("\n")
}

fn generate_struct<P>(name: &String, shape: &Shape, protocol_generator: &P) -> String
where P: GenerateProtocol {
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
            struct_fields = generate_struct_fields(shape),
        )
    }

}

fn generate_struct_fields(shape: &Shape) -> String {
    shape.members.as_ref().unwrap().iter().map(|(member_name, member)| {
        let mut lines = Vec::with_capacity(2);
        let name = member_name.to_snake_case();

        if let Some(ref docs) = member.documentation {
            lines.push(format!("#[doc=\"{}\"]", docs.replace("\"", "\\\"")));
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
