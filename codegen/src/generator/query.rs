use botocore::Service;
use super::Generator;

pub struct QueryGenerator<'a> {
    parent: &'a Generator<'a>,
}

impl<'a> QueryGenerator<'a> {
    pub fn new(parent: &'a Generator<'a>) -> Self {
        QueryGenerator {
            parent: parent,
        }
    }

    pub fn generate(&self) -> String {
        "Unimplemented!".to_owned()
    }
}
