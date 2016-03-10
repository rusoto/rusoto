use botocore::Service;
use super::Generator;

pub struct RestJsonGenerator<'a> {
    parent: &'a Generator<'a>,
}

impl<'a> RestJsonGenerator<'a> {
    pub fn new(parent: &'a Generator<'a>) -> Self {
        RestJsonGenerator {
            parent: parent,
        }
    }

    pub fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }
}
