use std::env::*;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::ascii::AsciiExt;
use hyper::Client;
use hyper::header::Connection;

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
    pub fn new(key:&str, secret:&str, token:Option<String>, expires_at:DateTime<UTC>) -> AWSCredentials {
        AWSCredentials {
            key: key.to_string(),
            secret: secret.to_string(),
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
        // expired by default
        return EnvironmentCredentialsProvider { credentials: None }
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

    Ok(AWSCredentials{key: env_key, secret: env_secret,
       token: None, expires_at: UTC::now() + Duration::seconds(600)})
}

pub struct FileCredentialsProvider {
    credentials: AWSCredentials
}

impl AWSCredentialsProvider for FileCredentialsProvider {
    fn new() -> FileCredentialsProvider {
        return FileCredentialsProvider {credentials: AWSCredentials{ key: "".to_string(),
            secret: "".to_string(), token: None, expires_at: UTC::now() - Duration::seconds(600) } };
    }

    fn get_credentials(&mut self) -> Result<&AWSCredentials, &str> {
        if self.credentials.credentials_are_expired() {
            // Default credentials file location:
            // ~/.aws/credentials (Linux/Mac)
            // %USERPROFILE%\.aws\credentials  (Windows)
            let mut profile_location;
            match env::home_dir() {
                Some(ref p) => profile_location = p.display().to_string() + "/.aws/credentials",
                None => return Err("Couldn't get your home dir.")
            }
            match get_credentials_from_file(profile_location) {
                Ok(creds) => {
                    self.credentials = AWSCredentials{ key: creds.get_aws_access_key_id().to_string(),
                        secret: creds.get_aws_secret_key().to_string(), token: None,
                        expires_at: UTC::now() + Duration::seconds(600) };
                }
                Err(_) => return Err("Couldn't get creds from file.")
            }
       }
       Ok(&self.credentials)
   }
}

// Finds and uses the first "aws_access_key_id" and "aws_secret_access_key" in the file.
fn get_credentials_from_file<'a>(file_with_path: String) -> Result<AWSCredentials, &'a str> {
    //println!("Looking for credentials file at {}", file_with_path);
    let path = Path::new(&file_with_path);
    let display = path.display();

    match fs::metadata(&path) {
        Err(_) => return Err("Couldn't stat credentials file."),
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err("Couldn't open file.")
            }
        }
    };

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(opened_file) => opened_file,
    };

    let mut access_key = String::new();
    let mut secret_key = String::new();
    let file_lines = BufReader::new(&file);
    for line in file_lines.lines() {
        let unwrapped_line : String = line.unwrap();
        if unwrapped_line.starts_with('#') {
            continue;
        }

        let lower_case_line = unwrapped_line.to_ascii_lowercase().to_string();

        if lower_case_line.contains("aws_access_key_id") {
            if access_key.is_empty() {
                let v: Vec<&str> = unwrapped_line.split("=").collect();
                if v.len() == 0 {
                    access_key = "".to_string();
                } else {
                    access_key = v[1].trim_matches(' ').to_string();
                }
            }
        } else if lower_case_line.contains("aws_secret_access_key") {
            if secret_key.is_empty() {
                let v: Vec<&str> = unwrapped_line.split("=").collect();
                if v.len() == 0 {
                    secret_key = "".to_string();
                } else {
                    secret_key = v[1].trim_matches(' ').to_string();
                }
            }
        }
    }

    if access_key.is_empty() || secret_key.is_empty() {
        return Err("Couldn't find either aws_access_key_id, aws_secret_access_key or both in profile file.");
    }

    return Ok(AWSCredentials{ key: access_key.to_string(), secret: secret_key.to_string(),
         token:None, expires_at: UTC::now() + Duration::seconds(600) });
}

pub struct IAMRoleCredentialsProvider {
    credentials: AWSCredentials
}

impl AWSCredentialsProvider for IAMRoleCredentialsProvider {
    fn new() -> IAMRoleCredentialsProvider {
        return IAMRoleCredentialsProvider {credentials: AWSCredentials{ key: "".to_string(),
        secret: "".to_string(), token: None, expires_at: UTC::now() - Duration::seconds(600) } };
    }

    fn get_credentials(&mut self) -> Result<&AWSCredentials, &str> {
        if self.credentials.credentials_are_expired() {
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

            self.credentials = AWSCredentials{ key: access_key.to_string(),
                secret: secret_key.to_string(),  token: Some(token_from_response.to_string()), expires_at: expiration_time };
        }

		Ok(&self.credentials)
	}
}

#[derive(Debug, Clone)]
pub struct DefaultAWSCredentialsProviderChain {
    credentials: AWSCredentials
}

// Chain the providers:
impl DefaultAWSCredentialsProviderChain {
    pub fn new() -> DefaultAWSCredentialsProviderChain {
        return DefaultAWSCredentialsProviderChain {credentials: AWSCredentials{ key: "".to_string(),
            secret: "".to_string(), token: None, expires_at: UTC::now() - Duration::seconds(600) } };
    }

    pub fn get_credentials(&mut self) -> &AWSCredentials {
        if self.credentials.credentials_are_expired() {
            // fetch creds in order: env, file, IAM
            let mut env_provider = EnvironmentCredentialsProvider::new();
            match env_provider.get_credentials() {
                Ok(creds) => {
                    //println!("Found creds in env");
                    self.credentials = AWSCredentials{ key: creds.get_aws_access_key_id().to_string(),
                        secret: creds.get_aws_secret_key().to_string(), token: None,
                        expires_at: UTC::now() + Duration::seconds(600) };
                    return &self.credentials;
                }
                Err(_) => () //println!("Whiffed on env")
            }

            let mut file_provider = FileCredentialsProvider::new();
            match file_provider.get_credentials() {
                Ok(creds) => {
                    //println!("Found creds in file");
                    self.credentials = AWSCredentials{ key: creds.get_aws_access_key_id().to_string(),
                        secret: creds.get_aws_secret_key().to_string(), token: None,
                        expires_at: UTC::now() + Duration::seconds(600) };
                    return &self.credentials;
                }
                Err(_) => () //println!("Whiffed on file")
            }

            let mut iam_provider = IAMRoleCredentialsProvider::new();
            match iam_provider.get_credentials() {
                Ok(creds) => {
                    //println!("Found creds via iam");
                    self.credentials = AWSCredentials{ key: creds.get_aws_access_key_id().to_string(),
                        secret: creds.get_aws_secret_key().to_string(), token: creds.get_token().clone(),
                        expires_at: UTC::now() + Duration::seconds(600) };
                    return &self.credentials;
                }
                Err(_) => panic!("Couldn't find AWS credentials in environment, default credential file location or IAM role.")
            }
        }
        return &self.credentials;
    }
}

