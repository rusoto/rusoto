use botocore::Service;
use super::Generator;

pub struct QueryGenerator {
    source: String,
}

impl QueryGenerator {
    pub fn new() -> Self {
        QueryGenerator {
            source: String::new(),
        }
    }
}

impl Generator for QueryGenerator {
    fn generate(mut self, service: &Service) -> String {
        self.source
    }

    fn source(&mut self) -> &mut String {
        &mut self.source
    }
}
