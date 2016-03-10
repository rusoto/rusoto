use botocore::Service;

pub struct RestXmlGenerator<'a> {
    service: &'a Service,
}

impl<'a> RestXmlGenerator<'a> {
    pub fn new(service: &'a Service) -> Self {
        RestXmlGenerator {
            service: service,
        }
    }

    pub fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }
}
