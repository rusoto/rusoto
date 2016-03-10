use botocore::Service;

pub struct Ec2Generator<'a> {
    service: &'a Service,
}

impl<'a> Ec2Generator<'a> {
    pub fn new(service: &'a Service) -> Self {
        Ec2Generator {
            service: service,
        }
    }

    pub fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }
}
