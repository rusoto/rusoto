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
// use hyper::client::response::Response;

extern crate rustc_serialize;
use self::rustc_serialize::json::*;

extern crate chrono;
use self::chrono::*;


#[derive(Clone, Debug)]
pub struct AWSCredentials {
    key: String,
    secret: String,
    token: String,
    expires_at: DateTime<UTC>
}

impl AWSCredentials {
    pub fn new(key:&str, secret:&str, token:&str, expires_at:DateTime<UTC>) -> AWSCredentials {
        AWSCredentials {
            key: key.to_string(),
            secret: secret.to_string(),
            token: token.to_string(),
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

    pub fn get_token(&self) -> &str {
        &self.token
    }

    fn credentials_are_expired(&self) -> bool {
        println!("Seeing if creds of {:?} are expired compared to {:?}", self.expires_at, UTC::now() + Duration::seconds(20));
        // This is a rough hack to hopefully avoid someone requesting creds then sitting on them
        // before issuing the request:
        if self.expires_at < UTC::now() + Duration::seconds(20) {
            return true;
        }
        return false;
    }
}

// TODO: refactor get_credentials to return std::result
pub trait AWSCredentialsProvider {
    fn new() -> Self;
	fn get_credentials(&mut self) -> &AWSCredentials;
	fn refresh(&mut self);
}

pub struct EnvironmentCredentialsProvider {
    credentials: AWSCredentials
}

impl AWSCredentialsProvider for EnvironmentCredentialsProvider {
    fn new() -> EnvironmentCredentialsProvider {
        return EnvironmentCredentialsProvider {credentials: AWSCredentials{ key: "".to_string(),
            secret: "".to_string(), token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600) } };
    }

	fn refresh(&mut self) {
        let env_key = match var("AWS_ACCESS_KEY_ID") {
            Ok(val) => val,
            Err(_) => "".to_string()
        };
        let env_secret = match var("AWS_SECRET_ACCESS_KEY") {
            Ok(val) => val,
            Err(_) => "".to_string()
        };

        self.credentials = AWSCredentials{key: env_key, secret: env_secret,
             token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600)};
	}

	fn get_credentials(&mut self) -> &AWSCredentials {
		return &self.credentials;
	}
}

pub struct FileCredentialsProvider {
    credentials: AWSCredentials
}

impl AWSCredentialsProvider for FileCredentialsProvider {
    fn new() -> FileCredentialsProvider {
        return FileCredentialsProvider {credentials: AWSCredentials{ key: "".to_string(),
            secret: "".to_string(), token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600) } };
    }

	fn refresh(&mut self) {
        // Default credentials file location:
        // ~/.aws/credentials (Linux/Mac)
        // %USERPROFILE%\.aws\credentials  (Windows)
        let mut profile_location = String::new();
        match env::home_dir() {
            Some(ref p) => profile_location = p.display().to_string() + "/.aws/credentials",
            None => {
                println!("Couldn't get your home dir.");
                self.credentials = AWSCredentials{ key: "".to_string(), secret: "".to_string(),
                     token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600) };
                return;
            }
        }
        let credentials = get_credentials_from_file(profile_location);
        self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(),
            secret: credentials.get_aws_secret_key().to_string(), token: "".to_string(),
            expires_at: UTC::now() + Duration::seconds(600) };
    }

    fn get_credentials(&mut self) -> &AWSCredentials {
		return &self.credentials;
	}
}

// Finds and uses the first "aws_access_key_id" and "aws_secret_access_key" in the file.
fn get_credentials_from_file(file_with_path: String) -> AWSCredentials {
    println!("Looking for credentials file at {}", file_with_path);
    let path = Path::new(&file_with_path);
    let display = path.display();

    let mut found_file = false;

    match fs::metadata(&path) {
        Err(why) => println!("Couldn't get metadata for file: {}", why),
        Ok(metadata) => found_file = metadata.is_file()
    };

    // bail early.  Should be converted to a return type.
    if !found_file {
        return AWSCredentials{ key: "".to_string(), secret: "".to_string(),
             token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600) };
    }

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(opened_file) => opened_file,
    };

    let mut access_key = String::new();
    let mut secret_key = String::new();
    let file_lines = BufReader::new(&file);
    for line in file_lines.lines() {
        let unwrapped_line : String = line.unwrap();
        // Skip line if it starts with a comment ('#')
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

    return AWSCredentials{ key: access_key.to_string(), secret: secret_key.to_string(),
         token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600) };
}

// class for IAM role
pub struct IAMRoleCredentialsProvider {
    credentials: AWSCredentials
}

impl AWSCredentialsProvider for IAMRoleCredentialsProvider {
    fn new() -> IAMRoleCredentialsProvider {
        return IAMRoleCredentialsProvider {credentials: AWSCredentials{ key: "".to_string(),
        secret: "".to_string(), token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600) } };
    }

	fn refresh(&mut self) {
        // call instance metadata to get iam role
        // for "real" use: http://169.254.169.254/latest/meta-data/iam/security-credentials/
        let mut address : String = "http://169.254.169.254/latest/meta-data/iam/security-credentials".to_string();
        let client = Client::new();
        let mut response;
        match client.get(&address)
            .header(Connection::close()).send() {
                Err(why) => {
                    println!("boo, request failed: {}", why);
                    return;
                }
                Ok(received_response) => response = received_response
            };

        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Err(why) => println!("Had issues with reading response: {}", why),
            Ok(_) => (),
        };

        // use results to make another call:
        // curl http://169.254.169.254/latest/meta-data/iam/security-credentials/fooprofile

        // sample results:
        // {
        //   "Code" : "Success",
        //   "LastUpdated" : "2015-08-04T00:09:23Z",
        //   "Type" : "AWS-HMAC",
        //   "AccessKeyId" : "AAAAAA",
        //   "SecretAccessKey" : "AAAAA",
        //   "Token" : "AAAAA",
        //   "Expiration" : "2015-08-04T06:32:37Z"
        // }
        //  let format = "%Y-%m-%d %T.%f";

        address.push_str("/");
        address.push_str(&body);
        body = String::new();
        match client.get(&address)
            .header(Connection::close()).send() {
                Err(_) => return,
                Ok(received_response) => response = received_response
            };

        match response.read_to_string(&mut body) {
            Err(why) => println!("Had issues with reading iam role response: {}", why),
            Ok(_) => (),
        };

        let json_object : Json;
        match Json::from_str(&body) {
            Err(why) => {
                println!("Error: {}", why);
                return;
            }
            Ok(val) => json_object = val
        };

        let mut access_key = String::new();
        match json_object.find("AccessKeyId") {
            None => {
                println!("Error finding AccessKeyId");
                return;
            }
            Some(val) => access_key = val.to_string().replace("\"", "")
        };

        let mut secret_key = String::new();
        match json_object.find("SecretAccessKey") {
            None => {
                println!("Error finding SecretAccessKey");
                return;
            }
            Some(val) => secret_key = val.to_string().replace("\"", "")
        };

        let mut expiration = String::new();
        match json_object.find("Expiration") {
            None => {
                println!("Error finding Expiration");
                return;
            }
            Some(val) => expiration = val.to_string().replace("\"", "")
        };

        let mut expiration_time;
        match expiration.parse::<DateTime<UTC>>() {
            Err(why) => panic!("Kabloey on parse: {}", why),
            Ok(val) => expiration_time = val
        };

        let mut token_from_response = String::new();
        match json_object.find("Token") {
            None => {
                println!("Error finding Token");
                return;
            }
            Some(val) => token_from_response = val.to_string().replace("\"", "")
        };

        self.credentials = AWSCredentials{ key: access_key.to_string(),
            secret: secret_key.to_string(),  token: token_from_response.to_string(), expires_at: expiration_time };
    }

    // This seems a bit convoluted: try to reused expired instead of making a new var.
    fn get_credentials(&mut self) -> &AWSCredentials {
        let expired = &self.credentials.credentials_are_expired();
        if *expired {
            println!("Creds are expired, refreshing.");
            &self.refresh();
            let new_expired = &self.credentials.credentials_are_expired();
            if *new_expired {
                panic!("Credentials were expired and couldn't fetch fresh ones.");
            }
        }
		return &self.credentials;
	}
}

pub struct DefaultAWSCredentialsProviderChain {
    credentials: AWSCredentials
}

// Chain the providers:
impl DefaultAWSCredentialsProviderChain {
    pub fn new() -> DefaultAWSCredentialsProviderChain {
        return DefaultAWSCredentialsProviderChain {credentials: AWSCredentials{ key: "".to_string(),
            secret: "".to_string(), token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600) } };
    }

    pub fn get_credentials(&mut self) -> &AWSCredentials {
        return &self.credentials;
    }

    // This is getting a bit out of control with nested if/else: should try doing something else for flow control.
    pub fn refresh(&mut self) {
        // fetch creds in order: env, file, IAM
        self.credentials = AWSCredentials{ key: "".to_string(), secret: "".to_string(),
            token: "".to_string(), expires_at: UTC::now() + Duration::seconds(600) };

        let mut env_provider = EnvironmentCredentialsProvider::new();
        env_provider.refresh();
        let credentials = env_provider.get_credentials();

        if creds_have_values(credentials) {
            println!("using creds from env: {}, {}", credentials.get_aws_access_key_id(), credentials.get_aws_secret_key());
            self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(),
                secret: credentials.get_aws_secret_key().to_string(), token: "".to_string(),
                expires_at: UTC::now() + Duration::seconds(600) };
            return;
        } else {
            // try the file provider
            let mut file_provider = FileCredentialsProvider::new();
            file_provider.refresh();
            let credentials = file_provider.get_credentials();
            println!("using creds from file: {}, {}", credentials.get_aws_access_key_id(), credentials.get_aws_secret_key());

            if creds_have_values(credentials) {
                self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(),
                    secret: credentials.get_aws_secret_key().to_string(), token: "".to_string(),
                    expires_at: UTC::now() + Duration::seconds(600) };
            } else {
                let mut iam_provider = IAMRoleCredentialsProvider::new();
                iam_provider.refresh();
                let credentials = iam_provider.get_credentials();
                if creds_have_values(credentials) {
                    println!("using creds from IAM: {}, {}, {}", credentials.get_aws_access_key_id(),
                        credentials.get_aws_secret_key(), credentials.get_token());
                    self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(),
                        secret: credentials.get_aws_secret_key().to_string(), token: credentials.get_token().to_string(),
                        expires_at: UTC::now() + Duration::seconds(600) };
                    return;
                } else {
                    // We're out of options
                    panic!("Couldn't find AWS credentials in environment, default credential file location or IAM role.");
                }
            }
        }
    }
}

fn creds_have_values(creds: &AWSCredentials) -> bool {
    if creds.get_aws_access_key_id().len() > 0 && creds.get_aws_secret_key().len() > 0 {
        return true;
    }
    return false;
}
