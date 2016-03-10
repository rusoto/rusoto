use botocore::{Service, Shape};

pub use self::ec2::Ec2Generator;
pub use self::json::JsonGenerator;
pub use self::query::QueryGenerator;
pub use self::rest_json::RestJsonGenerator;
pub use self::rest_xml::RestXmlGenerator;

mod ec2;
mod json;
mod query;
mod rest_json;
mod rest_xml;

pub trait Generator {
    fn generate(&self) -> String;

    fn service(&self) -> &Service;

    fn client_type_name(&self) -> String {
        format!("{}Client", self.service_type_name())
    }

    fn error_type_name(&self) -> String {
        format!("{}Error", self.service_type_name())
    }

    fn generate_client_footer(&self) -> String {
        "}\n".to_owned()
    }

    fn generate_client_header(&self) -> String {
        format!("
pub struct {type_name}<'a> {{
    credential_provider: Box<ProvideAWSCredentials + 'a>,
    region: &'a Region,
}}

impl<'a> {type_name}<'a> {{
    pub fn new<P>(
        credential_provider: P,
        region: &'a Region,
    ) -> Self where P: ProvideAWSCredentials + 'a {{
        {type_name} {{
            credential_provider: Box::new(credential_provider),
            region: region,
        }}
    }}
",
            type_name = self.client_type_name()
        )
    }

    fn generate_list(&self, name: &str, shape: &Shape) -> String {
        format!("pub type {} = Vec<{}>;\n", name, shape.member())
    }

    fn generate_map(&self, name: &str, shape: &Shape) -> String {
        format!(
            "pub type {} = ::std::collections::HashMap<{}, {}>;\n",
            name,
            shape.key(),
            shape.value(),
        )
    }

    fn generate_primitive_type(&self, name: &str, shape_type: &str) -> String {
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

        format!("pub type {} = {};\n", name, primitive_type)
    }

    fn generate_shapes(&self) -> String {
        let mut source = String::new();

        for (name, shape) in self.service().shapes.iter() {
            if name == "String" {
                continue;
            }

            let shape_source = match &shape.shape_type[..] {
                "structure" => self.generate_struct(name, shape),
                "map" => self.generate_map(name, shape),
                "list" => self.generate_list(name, shape),
                shape_type => self.generate_primitive_type(name, shape_type),
            };

            source.push_str(&shape_source);
        }

        source
    }

    fn generate_struct(&self, name: &String, shape: &Shape) -> String {
        if shape.members.is_none() || shape.members.as_ref().unwrap().is_empty() {
            return format!("
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct {};
",
                name,
            );
        }

        format!("
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct {} {{
    {}
}}
",
            name,
            self.generate_struct_fields(shape),
        )
    }

    fn generate_struct_fields(&self, shape: &Shape) -> String {
        let mut source = String::new();

        for (member_name, member) in shape.members.as_ref().unwrap().iter() {
            if let Some(ref docs) = member.documentation {
                source.push_str(&format!("/// {}\n", docs));
            }

            if shape.required(member_name) {
                source.push_str(&format!("pub {}: {},\n",  member_name, member.shape));
            } else if member_name == "type" {
                source.push_str(&format!("pub aws_{}: Option<{}>,\n",  member_name, member.shape));
            } else {
                source.push_str(&format!("pub {}: Option<{}>,\n",  member_name, member.shape));
            }
        }

        source
    }

    fn service_type_name(&self) -> String {
        self.service().metadata.service_abbreviation.replace("Amazon ", "").replace(" ", "")
    }
}
