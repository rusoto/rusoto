use botocore::{
    Service,
    Shape,
};
use self::ec2::Ec2Generator;
use self::json::JsonGenerator;
use self::query::QueryGenerator;
use self::rest_json::RestJsonGenerator;
use self::rest_xml::RestXmlGenerator;

mod ec2;
mod json;
mod query;
mod rest_json;
mod rest_xml;

pub struct Generator<'a> {
    service: &'a Service,
}

impl<'a> Generator<'a> {
    pub fn new(service: &'a Service) -> Self {
        Generator {
            service: service,
        }
    }

    pub fn generate(&self) -> String {
        match &self.service.metadata.protocol[..] {
            "ec2" => Ec2Generator::new(self).generate(),
            "json" => JsonGenerator::new(self).generate(),
            "query" => QueryGenerator::new(self).generate(),
            "rest-json" => RestJsonGenerator::new(self).generate(),
            "rest-xml" => RestXmlGenerator::new(self).generate(),
            protocol => panic!("Unknown protocol {}", protocol),
        }
    }

    fn generate_list(&self, shape: &Shape) -> String {
        format!(
            "Vec<{}>",
            shape.member(),
        )
    }

    fn generate_map(&self, shape: &Shape) -> String {
        format!(
            "::std::collections::HashMap<{}, {}>",
            shape.key(),
            shape.value(),
        )
    }

    fn generate_primitive_type(&self, shape_type: &str) -> String {
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

        primitive_type.to_owned()
    }

    pub fn generate_shapes(&self) -> String {
        let mut source = String::new();

        for (name, shape) in self.service.shapes.iter() {
            if name == "String" {
                continue;
            }

            let shape_source = match &shape.shape_type[..] {
                "structure" => self.generate_struct(name, shape),
                "map" => self.generate_map(shape),
                "list" => self.generate_list(shape),
                shape_type => self.generate_primitive_type(shape_type),
            };

            source.push_str(&shape_source);
        }

        source
    }

    fn generate_struct(&self, name: &String, shape: &Shape) -> String {
        let mut source = "#[derive(Debug, Default, Deserialize, Serialize)]\n".to_owned();

        if shape.members.is_empty() {
            source.push_str(&format!("pub struct {};\n", name));

            return source;
        }

        source.push_str(&format!("pub struct {} {{\n", name));

        for (member_name, member) in shape.members.iter() {
            if let Some(ref docs) = member.documentation {
                source.push_str(&format!("/// {}\n", docs));
            }
        }

        source.push_str("}\n");

        source
    }

    pub fn error_type_name(&self) -> String {
        format!("{}Error", self.service.metadata.service_abbreviation)
    }
}
