use std::io::Read;

use reqwest;
use serde_json::{Value, from_str};
use retry;

use {AwsCredentials, CredentialsError, ProvideAwsCredentials};

/// Provides AWS credentials from a resource's IAM role.
#[derive(Debug)]
pub struct InstanceMetadataProvider;

impl ProvideAwsCredentials for InstanceMetadataProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let mut address : String = "http://169.254.169.254/latest/meta-data/iam/security-credentials".to_string();
        let mut response =
            match retry::retry_exponentially(5, 7f64, || { reqwest::get(&address) },
            |response_attempt| {
                match response_attempt.as_ref() {
                    Ok(response_returned) => response_returned.status().is_success(),
                    Err(_) => false,
                }
            })
        {
            Ok(response_from_try) => {
                match response_from_try {
                    Ok(resp) => resp,
                    Err(err) => return Err(CredentialsError::new(&format!("Couldn't connect to credentials provider: {}", err))),
                }
            }
            Err(error) => return Err(CredentialsError::new(&format!("Couldn't connect to credentials provider: {}", error))),
        };

        let mut body = String::new();
        if response.read_to_string(&mut body).is_err() {
            return Err(CredentialsError::new("Didn't get a parsable response body from metadata service"));
        }

        address.push_str("/");
        address.push_str(&body);
        body = String::new();
        match reqwest::get(&address) {
                Err(_) => return Err(CredentialsError::new("Didn't get a parseable response body from instance role details")),
                Ok(received_response) => response = received_response
            };

        if response.read_to_string(&mut body).is_err() {
            return Err(CredentialsError::new("Had issues with reading iam role response: {}"));
        }

        let json_object: Value;
        match from_str(&body) {
            Err(_) => return Err(CredentialsError::new("Couldn't parse metadata response body.")),
            Ok(val) => json_object = val
        };

        let access_key;
        match json_object.get("AccessKeyId") {
            None => return Err(CredentialsError::new("Couldn't find AccessKeyId in response.")),
            Some(val) => access_key = val.as_str().expect("AccessKeyId value was not a string").to_owned().replace("\"", "")
        };

        let secret_key;
        match json_object.get("SecretAccessKey") {
            None => return Err(CredentialsError::new("Couldn't find SecretAccessKey in response.")),
            Some(val) => secret_key = val.as_str().expect("SecretAccessKey value was not a string").to_owned().replace("\"", "")
        };

        let expiration;
        match json_object.get("Expiration") {
            None => return Err(CredentialsError::new("Couldn't find Expiration in response.")),
            Some(val) => expiration = val.as_str().expect("Expiration value was not a string").to_owned().replace("\"", "")
        };

        let expiration_time = try!(expiration.parse());

        let token_from_response;
        match json_object.get("Token") {
            None => return Err(CredentialsError::new("Couldn't find Token in response.")),
            Some(val) => token_from_response = val.as_str().expect("Token value was not a string").to_owned().replace("\"", "")
        };

        Ok(AwsCredentials::new(access_key, secret_key, Some(token_from_response), expiration_time))
    }
}
