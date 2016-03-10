use botocore::Service;
use super::Generator;

pub struct QueryGenerator<'a> {
    service: &'a Service,
}

impl<'a> QueryGenerator<'a> {
    pub fn new(service: &'a Service) -> Self {
        QueryGenerator {
            service: service,
        }
    }
}

impl<'a> Generator for QueryGenerator<'a> {
    fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }

    fn service(&self) -> &Service {
        self.service
    }
}
