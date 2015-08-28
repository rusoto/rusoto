use std::env::*;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use hyper::Client;
use hyper::header::Connection;
use error::*;
use regex::Regex;


extern crate rustc_serialize;
use self::rustc_serialize::json::*;

extern crate chrono;
use self::chrono::*;


#[derive(Clone, Debug)]
pub struct AWSCredentials {
    key: String,
    secret: String,
    token: Option<String>,
    expires_at: DateTime<UTC>
}

impl AWSCredentials {
    pub fn new<K, S>(key:K, secret:S, token:Option<String>, expires_at:DateTime<UTC>) -> AWSCredentials where K:Into<String>, S:Into<String> {
        AWSCredentials {
            key: key.into(),
            secret: secret.into(),
            token: token,
            expires_at: expires_at,
        }
    }

    pub fn get_aws_access_key_id(&self) -> &str {
    	&self.key
    }

    pub fn get_aws_secret_key(&self) -> &str {
    	&self.secret
    }

    pub fn get_expires_at(&self) -> &DateTime<UTC> {
        &self.expires_at
    }

    pub fn get_token(&self) -> &Option<String> {
        &self.token
    }

    fn credentials_are_expired(&self) -> bool {
        //println!("Seeing if creds of {:?} are expired compared to {:?}", self.expires_at, UTC::now() + Duration::seconds(20));
        // This is a rough hack to hopefully avoid someone requesting creds then sitting on them
        // before issuing the request:
        if self.expires_at < UTC::now() + Duration::seconds(20) {
            return true;
        }
        return false;
    }
}

pub trait AWSCredentialsProvider {
    fn new() -> Self;
	fn get_credentials(&mut self) -> Result<&AWSCredentials, &str>;
}

pub struct EnvironmentCredentialsProvider {
    credentials: Option<AWSCredentials>
}

impl AWSCredentialsProvider for EnvironmentCredentialsProvider {
    fn new() -> EnvironmentCredentialsProvider {
        EnvironmentCredentialsProvider { credentials: None }
    }

	fn get_credentials(&mut self) -> Result<&AWSCredentials, &str> {
        if self.credentials.is_none() || self.credentials.as_ref().unwrap().credentials_are_expired() {
           self.credentials = Some(try!(get_credentials_from_environment()));
        } 
        Ok(self.credentials.as_ref().unwrap())
	}
}

fn get_credentials_from_environment<'a>() -> Result<AWSCredentials, &'a str> {
    let env_key = match var("AWS_ACCESS_KEY_ID") {
        Ok(val) => val,
        Err(_) => return Err("No AWS_ACCESS_KEY_ID in environment")
    };
    let env_secret = match var("AWS_SECRET_ACCESS_KEY") {
        Ok(val) => val,
        Err(_) => return Err("No AWS_SECRET_ACCESS_KEY in environment")
    };

    if env_key.is_empty() || env_secret.is_empty() {
        return Err("Couldn't find either AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY or both in environment.");
    }

    Ok(AWSCredentials::new(env_key, env_secret, None, in_ten_minutes()))
}

pub struct ProfileCredentialsProvider {
    profile: String,
    file_name: String,
    credentials: Option<AWSCredentials>
}

impl ProfileCredentialsProvider {
    pub fn with_configuration(profile: &str, file_name: &str) -> ProfileCredentialsProvider {
        ProfileCredentialsProvider { credentials: None, profile: profile.to_string(), file_name: file_name.to_string() }
    }
}

impl AWSCredentialsProvider for ProfileCredentialsProvider {
    fn new() -> ProfileCredentialsProvider {
        // Default credentials file location:
        // ~/.aws/credentials (Linux/Mac)
        // %USERPROFILE%\.aws\credentials  (Windows)
        let mut profile_location;
        match env::home_dir() {
            Some(ref p) => profile_location = p.display().to_string() + "/.aws/credentials",
            None => panic!("Couldn't get your home dir.")
        }

        ProfileCredentialsProvider { credentials: None, profile: "default".to_string(), file_name: profile_location }
    }

    fn get_credentials(&mut self) -> Result<&AWSCredentials, &str> {
        if self.credentials.is_none() || self.credentials.as_ref().unwrap().credentials_are_expired() {           
            match parse_credentials_file(&self.file_name) {
                Ok(mut profiles) => { 
                    let default_profile = profiles.remove(&self.profile);
                    if default_profile.is_none() {
                        return Err("profile not found");
                    }
                    self.credentials = default_profile;
                },
                Err(_) => { return Err("Parse error"); }
            };
       }
       Ok(self.credentials.as_ref().unwrap())
   }
}

fn parse_credentials_file(file_with_path: &str) -> Result<HashMap<String, AWSCredentials>, AWSError> {
    let path = Path::new(&file_with_path);
    let display = path.display();

    match fs::metadata(&path) {
        Err(_) => return Err(AWSError::new("Couldn't stat credentials file.")),
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(AWSError::new("Couldn't open file."));
            }
        }
    };

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(opened_file) => opened_file,
    };

    let profile_regex = Regex::new(r"^\[([^\]]+)\]$").unwrap();
    let mut profiles: HashMap<String, AWSCredentials> = HashMap::new();
    let mut access_key: Option<String> = None;
    let mut secret_key: Option<String> = None;    
    let mut profile_name: Option<String> = None;

    let file_lines = BufReader::new(&file);
    for line in file_lines.lines() {

        let unwrapped_line : String = line.unwrap();

        // skip comments
        if unwrapped_line.starts_with('#') {
            continue;
        }

        // handle the opening of named profile blocks
        if profile_regex.is_match(&unwrapped_line) {

            if profile_name.is_some() && access_key.is_some() && secret_key.is_some() {
                let creds = AWSCredentials::new(access_key.unwrap(), secret_key.unwrap(), None, in_ten_minutes());
                profiles.insert(profile_name.unwrap(), creds);
            }

            access_key = None;
            secret_key = None;

            let caps = profile_regex.captures(&unwrapped_line).unwrap();            
            profile_name = Some(caps.at(1).unwrap().to_string());
            continue;
        }

        // otherwise look for key=value pairs we care about
        let lower_case_line = unwrapped_line.to_ascii_lowercase().to_string();

        if lower_case_line.contains("aws_access_key_id") {
            if access_key.is_none() {
                let v: Vec<&str> = unwrapped_line.split("=").collect();
                if v.len() > 0 {                  
                    access_key = Some(v[1].trim_matches(' ').to_string());
                }
            }
        } else if lower_case_line.contains("aws_secret_access_key") {
            if secret_key.is_none() {
                let v: Vec<&str> = unwrapped_line.split("=").collect();
                if v.len() > 0 {                  
                    secret_key = Some(v[1].trim_matches(' ').to_string());
                }
            }
        }

        // we could potentially explode here to indicate that the file is invalid

    }

    if profile_name.is_some() && access_key.is_some() && secret_key.is_some() {
        let creds = AWSCredentials::new(access_key.unwrap(), secret_key.unwrap(), None, in_ten_minutes());
        profiles.insert(profile_name.unwrap(), creds);
    }

    if profiles.is_empty() {
        return Err(AWSError::new("No credentials found."));
    } 

    Ok(profiles)
}

pub struct IAMRoleCredentialsProvider {
    credentials: Option<AWSCredentials>
}

impl AWSCredentialsProvider for IAMRoleCredentialsProvider {
    fn new() -> IAMRoleCredentialsProvider {
        IAMRoleCredentialsProvider { credentials: None }
    }

    fn get_credentials(&mut self) -> Result<&AWSCredentials, &str> {
        if self.credentials.is_none() || self.credentials.as_ref().unwrap().credentials_are_expired() {
            // TODO: backoff and retry on failure.

            //println!("Calling IAM metadata");
            // for "real" use: http://169.254.169.254/latest/meta-data/iam/security-credentials/
            let mut address : String = "http://169.254.169.254/latest/meta-data/iam/security-credentials".to_string();
            let client = Client::new();
            let mut response;
            match client.get(&address)
                .header(Connection::close()).send() {
                    Err(_) => return Err("Couldn't connect to metadata service"), // add Why?
                    Ok(received_response) => response = received_response
                };

            let mut body = String::new();
            match response.read_to_string(&mut body) {
                Err(_) => return Err("Didn't get a parsable response body from metadata service"),
                Ok(_) => (),
            };

            address.push_str("/");
            address.push_str(&body);
            body = String::new();
            match client.get(&address)
                .header(Connection::close()).send() {
                    Err(_) => return Err("Didn't get a parseable response body from instance role details"),
                    Ok(received_response) => response = received_response
                };

            match response.read_to_string(&mut body) {
                Err(_) => return Err("Had issues with reading iam role response: {}"),
                Ok(_) => (),
            };

            let json_object : Json;
            match Json::from_str(&body) {
                Err(_) => return Err("Couldn't parse metadata response body."),
                Ok(val) => json_object = val
            };

            let mut access_key;
            match json_object.find("AccessKeyId") {
                None => return Err("Couldn't find AccessKeyId in response."),
                Some(val) => access_key = val.to_string().replace("\"", "")
            };

            let mut secret_key;
            match json_object.find("SecretAccessKey") {
                None => return Err("Couldn't find SecretAccessKey in response."),
                Some(val) => secret_key = val.to_string().replace("\"", "")
            };

            let mut expiration;
            match json_object.find("Expiration") {
                None => return Err("Couldn't find Expiration in response."),
                Some(val) => expiration = val.to_string().replace("\"", "")
            };

            let mut expiration_time;
            match expiration.parse::<DateTime<UTC>>() {
                Err(why) => panic!("Kabloey on parse: {}", why),
                Ok(val) => expiration_time = val
            };

            let mut token_from_response;
            match json_object.find("Token") {
                None => return Err("Couldn't find Token in response."),
                Some(val) => token_from_response = val.to_string().replace("\"", "")
            };

            self.credentials = Some(AWSCredentials::new(access_key, secret_key, Some(token_from_response.to_string()), expiration_time));
        }

		Ok(&self.credentials.as_ref().unwrap())
	}
}

#[derive(Debug, Clone)]
pub struct DefaultAWSCredentialsProviderChain {
    credentials: Option<AWSCredentials>
}

// Chain the providers:
impl DefaultAWSCredentialsProviderChain {
    pub fn new() -> DefaultAWSCredentialsProviderChain {
        DefaultAWSCredentialsProviderChain { credentials: None }
    }

    pub fn get_credentials(&mut self) -> Result<&AWSCredentials, AWSError> {
        if self.credentials.is_none() || self.credentials.as_ref().unwrap().credentials_are_expired() {
            // fetch creds in order: env, file, IAM

            if let Ok(creds) = EnvironmentCredentialsProvider::new().get_credentials() {
                //println!("Found creds in env");
                self.credentials = Some(creds.clone());
            } else if let Ok(creds) = ProfileCredentialsProvider::new().get_credentials() {             
                //println!("Found creds in file");
                self.credentials = Some(creds.clone());             
            } else if let Ok(creds) = IAMRoleCredentialsProvider::new().get_credentials() {
                //println!("Found creds via iam");
                self.credentials = Some(creds.clone());
            } else {
               panic!("Couldn't find AWS credentials in environment, default credential file location or IAM role.")
            }
        }
        Ok(self.credentials.as_ref().unwrap())
    }
}

fn in_ten_minutes() -> DateTime<UTC> {
    UTC::now() + Duration::seconds(600)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::fs::File;
    use error::*;
    use regex::*;

    #[test]
    fn parse_credentials_file_default_profile() {
        let result = super::parse_credentials_file("tests/sample-data/default_profile_credentials");
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles.get("default").unwrap();
        assert_eq!(default_profile.get_aws_access_key_id(), "foo");
        assert_eq!(default_profile.get_aws_secret_key(), "bar");        
    }

    #[test]
    fn parse_credentials_file_multiple_profiles() {
        let result = super::parse_credentials_file("tests/sample-data/multiple_profile_credentials");
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 2);

        let foo_profile = profiles.get("foo").unwrap();
        assert_eq!(foo_profile.get_aws_access_key_id(), "foo_access_key");
        assert_eq!(foo_profile.get_aws_secret_key(), "foo_secret_key");   

        let bar_profile = profiles.get("bar").unwrap();
        assert_eq!(bar_profile.get_aws_access_key_id(), "bar_access_key");
        assert_eq!(bar_profile.get_aws_secret_key(), "bar_secret_key");   

    }

    #[test]
    fn profile_credentials_provider_happy_path() {
        let mut provider = ProfileCredentialsProvider::with_configuration("foo","tests/sample-data/multiple_profile_credentials");
        let result = provider.get_credentials();

        assert!(result.is_ok());

        let creds = result.ok().unwrap();
        assert_eq!(creds.get_aws_access_key_id(), "foo_access_key");
        assert_eq!(creds.get_aws_secret_key(), "foo_secret_key");
     }

    #[test]
    fn profile_credentials_provider_bad_profile() {
        let mut provider = ProfileCredentialsProvider::with_configuration("not_a_profile","tests/sample-data/multiple_profile_credentials");
        let result = provider.get_credentials();

        assert!(result.is_err());
        assert_eq!(result.err(), Some("profile not found"));
     }

    #[test]
    fn existing_file_no_credentials() {
        let result = super::parse_credentials_file("tests/sample-data/no_credentials");
        assert_eq!(result.err(), Some(AWSError::new("No credentials found.")))
    }

    #[test]
    fn parse_credentials_bad_path() {
        let result = super::parse_credentials_file("/bad/file/path");
        assert_eq!(result.err(), Some(AWSError::new("Couldn't stat credentials file.")));
    }

    #[test]
    fn parse_credentials_directory_path() {
        let result = super::parse_credentials_file("tests/");
        assert_eq!(result.err(), Some(AWSError::new("Couldn't open file.")));
    }

}

