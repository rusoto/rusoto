use botocore::Service;

pub struct RestJsonGenerator<'a> {
    service: &'a Service,
}

impl<'a> RestJsonGenerator<'a> {
    pub fn new(service: &'a Service) -> Self {
        RestJsonGenerator {
            service: service,
        }
    }

    pub fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }
}
