use botocore::Service;
use super::Generator;

pub struct Ec2Generator<'a> {
    service: &'a Service,
}

impl<'a> Ec2Generator<'a> {
    pub fn new(service: &'a Service) -> Self {
        Ec2Generator {
            service: service,
        }
    }
}

impl<'a> Generator for Ec2Generator<'a> {
    fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }

    fn service(&self) -> &Service {
        self.service
    }
}
