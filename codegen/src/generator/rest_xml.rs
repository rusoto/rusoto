use botocore::Service;
use super::Generator;

pub struct RestXmlGenerator<'a> {
    parent: &'a Generator<'a>,
}

impl<'a> RestXmlGenerator<'a> {
    pub fn new(parent: &'a Generator<'a>) -> Self {
        RestXmlGenerator {
            parent: parent,
        }
    }

    pub fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }
}
