use botocore::Service;
use super::Generator;

pub struct RestXmlGenerator<'a> {
    service: &'a Service,
}

impl<'a> RestXmlGenerator<'a> {
    pub fn new(service: &'a Service) -> Self {
        RestXmlGenerator {
            service: service,
        }
    }
}

impl<'a> Generator for RestXmlGenerator<'a> {
    fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }

    fn service(&self) -> &Service {
        self.service
    }
}
