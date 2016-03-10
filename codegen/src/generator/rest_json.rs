use botocore::Service;
use super::Generator;

pub struct RestJsonGenerator<'a> {
    service: &'a Service,
}

impl<'a> RestJsonGenerator<'a> {
    pub fn new(service: &'a Service) -> Self {
        RestJsonGenerator {
            service: service,
        }
    }
}

impl<'a> Generator for RestJsonGenerator<'a> {
    fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }

    fn service(&self) -> &Service {
        self.service
    }
}
