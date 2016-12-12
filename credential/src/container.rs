use std::env;
use std::io::Read;
use std::time::Duration as StdDuration;

use hyper::Client;
use hyper::header::Connection;
use serde_json::{self, Value};

use {
    AwsCredentials,
    CredentialsError,
    ProvideAwsCredentials,
    extract_string_value_from_json,
};

const AWS_CREDENTIALS_PROVIDER_IP: &'static str = "169.254.170.2";

/// Provides AWS credentials from a task's IAM role.
pub struct ContainerProvider;

impl ProvideAwsCredentials for ContainerProvider {
    // TODO: backoff and retry on failure.
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let aws_container_credentials_relative_uri = match env::var("AWS_CONTAINER_CREDENTIALS_RELATIVE_URI") {
            Ok(v) => v,
            Err(_) => return Err(CredentialsError::new("No AWS_CONTAINER_CREDENTIALS_RELATIVE_URI in environment")),
        };

        let address: String = format!("http://{}{}", AWS_CREDENTIALS_PROVIDER_IP, aws_container_credentials_relative_uri);

        let mut client = Client::new();
        client.set_read_timeout(Some(StdDuration::from_secs(15)));
        let mut response = match client.get(&address)
            .header(Connection::close()).send()
        {
            Ok(response) => response,
            Err(_) => return Err(CredentialsError::new("Couldn't connect to credentials provider")), // add why?
        };

        let mut body = String::new();
        if response.read_to_string(&mut body).is_err() {
            return Err(CredentialsError::new("Didn't get a parsable response body from the credentials provider"));
        }

        let credentials = try!(parse_credentials_response(&body));

        Ok(credentials)
    }
}

fn parse_credentials_response(response: &str) -> Result<AwsCredentials, CredentialsError> {
    let json_object: Value = match serde_json::from_str(response) {
        Ok(v) => v,
        Err(_) => return Err(CredentialsError::new("Couldn't parse credentials response body.")),
    };

    let access_key_id = try!(extract_string_value_from_json(&json_object, "AccessKeyId"));
    let secret_access_key = try!(extract_string_value_from_json(&json_object, "SecretAccessKey"));
    let token = try!(extract_string_value_from_json(&json_object, "Token"));
    let expiration = try!(extract_string_value_from_json(&json_object, "Expiration"));

    let expiration = try!(expiration.parse());

    Ok(AwsCredentials::new(
        access_key_id,
        secret_access_key,
        Some(token),
        expiration,
    ))
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::fs::{self, File};
    use std::path::Path;

    use super::parse_credentials_response;

    #[test]
    fn parse_iam_task_credentials_sample_response() {
        fn read_file_to_string(file_path: &Path) -> String {
            match fs::metadata(file_path) {
                Ok(metadata) => if !metadata.is_file() {
                    panic!("Couldn't open file");
                },
                Err(_) => panic!("Couldn't stat file"),
            };

            let mut file = File::open(file_path).unwrap();
            let mut result = String::new();
            file.read_to_string(&mut result).ok();

            result
        }

        let response = read_file_to_string(
            Path::new("tests/sample-data/iam_task_credentials_sample_response")
        );

        let credentials = parse_credentials_response(&response);

        assert!(credentials.is_ok());
        let credentials = credentials.unwrap();

        assert_eq!(credentials.aws_access_key_id(), "AKIAIOSFODNN7EXAMPLE");
        assert_eq!(credentials.aws_secret_access_key(), "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
    }
}
