use botocore::Service;

pub struct QueryGenerator<'a> {
    service: &'a Service,
}

impl<'a> QueryGenerator<'a> {
    pub fn new(service: &'a Service) -> Self {
        QueryGenerator {
            service: service,
        }
    }

    pub fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }
}
