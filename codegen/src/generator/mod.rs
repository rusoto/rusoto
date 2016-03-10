use botocore::Service;
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
            "ec2" => Ec2Generator::new(self.service).generate(),
            "json" => JsonGenerator::new(self.service).generate(),
            "query" => QueryGenerator::new(self.service).generate(),
            "rest-json" => RestJsonGenerator::new(self.service).generate(),
            "rest-xml" => RestXmlGenerator::new(self.service).generate(),
            protocol => panic!("Unknown protocol {}", protocol),
        }
    }
}
