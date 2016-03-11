use botocore::{Service, Shape};

pub use self::json::JsonGenerator;

mod json;

pub trait Generator {
    fn generate(mut self, service: &Service) -> String;

    fn source(&mut self) -> &mut String;

    fn append<S>(&mut self, source_fragment: S) where S: AsRef<str> {
        self.source().push_str(&format!("{}\n", source_fragment.as_ref()));
    }

    fn append_client_header(&mut self, service: &Service) {
        self.append(format!("
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
            type_name = service.client_type_name(),
        ));
    }

    fn append_closing_brace(&mut self) {
        self.append("}");
    }

    fn append_list(&mut self, name: &str, shape: &Shape) {
        self.append(format!("pub type {} = Vec<{}>;", name, shape.member()));
    }

    fn append_map(&mut self, name: &str, shape: &Shape) {
        self.append(format!(
            "pub type {} = ::std::collections::HashMap<{}, {}>;",
            name,
            shape.key(),
            shape.value(),
        ));
    }

    fn append_primitive_type(&mut self, name: &str, shape_type: &str) {
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

        self.append(format!("pub type {} = {};", name, primitive_type));
    }

    fn append_shapes(&mut self, service: &Service) {
        for (name, shape) in service.shapes.iter() {
            if name == "String" {
                continue;
            }

            match &shape.shape_type[..] {
                "structure" => self.append_struct(name, shape),
                "map" => self.append_map(name, shape),
                "list" => self.append_list(name, shape),
                shape_type => self.append_primitive_type(name, shape_type),
            };
        }
    }

    fn append_struct(&mut self, name: &String, shape: &Shape) {
        self.append("#[derive(Debug, Default, Deserialize, Serialize)]");

        if shape.members.is_none() || shape.members.as_ref().unwrap().is_empty() {
            self.append(format!("pub struct {};", name));
        } else {
            self.append(&format!("pub struct {} {{", name,));
            self.append_struct_fields(shape);
            self.append_closing_brace();
        }

    }

    fn append_struct_fields(&mut self, shape: &Shape) {
        for (member_name, member) in shape.members.as_ref().unwrap().iter() {
            if let Some(ref docs) = member.documentation {
                self.append(&format!("/// {}", docs));
            }

            if shape.required(member_name) {
                self.append(&format!("pub {}: {},",  member_name, member.shape));
            } else if member_name == "type" {
                self.append(&format!("pub aws_{}: Option<{}>,",  member_name, member.shape));
            } else {
                self.append(&format!("pub {}: Option<{}>,",  member_name, member.shape));
            }
        }
    }
}
