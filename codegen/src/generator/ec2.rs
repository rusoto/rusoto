use botocore::Service;
use super::Generator;

pub struct Ec2Generator<'a> {
    parent: &'a Generator<'a>,
}

impl<'a> Ec2Generator<'a> {
    pub fn new(parent: &'a Generator<'a>) -> Self {
        Ec2Generator {
            parent: parent,
        }
    }

    pub fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }
}
