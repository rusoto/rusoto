extern crate rusoto_codegen;

use rusoto_codegen::{Generator, Service};

fn main() {
    let services = [
        Service::new("dynamodb", "DynamoDBClient", "2012-08-10"),
        Service::new("kms", "KMSClient", "2014-11-01"),
        Service::new("sqs", "SQSClient", "2012-11-05"),
    ];

    let generator = match Generator::new() {
        Ok(generator) => generator,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    if let Err(error) = generator.generate(&services) {
        println!("Error: {}", error);
    }
}
