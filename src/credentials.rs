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
use hyper::client::response::Response;

extern crate rustc_serialize;
use self::rustc_serialize::json::*;


#[derive(Clone, Debug)]
pub struct AWSCredentials {
    key: String,
    secret: String
}

#[derive(Debug)]
enum CredentialErr {
    NoEnvironmentVariables,
    NoCredentialsFile,
}

struct ProfileCredentialsError;

impl AWSCredentials {
    pub fn new(key:&str, secret:&str) -> AWSCredentials {
        AWSCredentials {
            key: key.to_string(),
            secret: secret.to_string()
        }
    }

    pub fn get_aws_access_key_id(&self) -> &str {
    	&self.key
    }

    pub fn get_aws_secret_key(&self) -> &str {
    	&self.secret
    }
}

// TODO: refactor get_credentials to return std::result
pub trait AWSCredentialsProvider {
    fn new() -> Self;
	fn get_credentials(&self) -> &AWSCredentials;
	fn refresh(&mut self);
}

pub struct EnvironmentCredentialsProvider {
    credentials: AWSCredentials
}

impl AWSCredentialsProvider for EnvironmentCredentialsProvider {
    fn new() -> EnvironmentCredentialsProvider {
        return EnvironmentCredentialsProvider {credentials: AWSCredentials{ key: "".to_string(), secret: "".to_string() } };
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

        self.credentials = AWSCredentials{key: env_key, secret: env_secret};
	}

	fn get_credentials(&self) -> &AWSCredentials {
		return &self.credentials;
	}
}

pub struct FileCredentialsProvider {
    credentials: AWSCredentials
}

impl AWSCredentialsProvider for FileCredentialsProvider {
    fn new() -> FileCredentialsProvider {
        return FileCredentialsProvider {credentials: AWSCredentials{ key: "".to_string(), secret: "".to_string() } };
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
                self.credentials = AWSCredentials{ key: "".to_string(), secret: "".to_string() };
                return;
            }
        }
        let credentials = get_credentials_from_file(profile_location);
        self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(), secret: credentials.get_aws_secret_key().to_string() };
    }

    fn get_credentials(&self) -> &AWSCredentials {
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
        return AWSCredentials{ key: "".to_string(), secret: "".to_string() };
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

    return AWSCredentials{ key: access_key.to_string(), secret: secret_key.to_string() };
}

// class for IAM role
pub struct IAMRoleCredentialsProvider {
    credentials: AWSCredentials
}

impl AWSCredentialsProvider for IAMRoleCredentialsProvider {
    fn new() -> IAMRoleCredentialsProvider {
        return IAMRoleCredentialsProvider {credentials: AWSCredentials{ key: "".to_string(), secret: "".to_string() } };
    }

	fn refresh(&mut self) {
        // call instance metadata to get iam role
        // curl http://169.254.169.254/latest/meta-data/iam/security-credentials/
        // sample result:
        // fooprofile

        // for "real" use: http://169.254.169.254/latest/meta-data/iam/security-credentials/
        let mut address : String = "http://localhost:8080/latest/meta-data/iam/security-credentials".to_string();
        println!("Checking {} for credentials.", address);
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
        println!("request made");

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

        // add body to location:
        address.push_str("/");
        address.push_str(&body);
        println!("Making request to {}", address);
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

        // println!("Response for iam role request: {}", body);

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
            Some(val) => access_key = val.to_string()
        };

        let mut secret_key = String::new();
        match json_object.find("SecretAccessKey") {
            None => {
                println!("Error finding AccessKeyId");
                return;
            }
            Some(val) => secret_key = val.to_string()
        };

        self.credentials = AWSCredentials{ key: access_key.to_string(), secret: secret_key.to_string() };
    }

    fn get_credentials(&self) -> &AWSCredentials {
		return &self.credentials;
	}
}

pub struct DefaultAWSCredentialsProviderChain {
    credentials: AWSCredentials
}

// Chain the providers:
impl DefaultAWSCredentialsProviderChain {
    pub fn new() -> DefaultAWSCredentialsProviderChain {
        return DefaultAWSCredentialsProviderChain {credentials: AWSCredentials{ key: "".to_string(), secret: "".to_string() } };
    }

    pub fn get_credentials(&self) -> &AWSCredentials {
        return &self.credentials;
    }

    // This is getting a bit out of control with nested if/else: should try doing something else for flow control.
    pub fn refresh(&mut self) {
        // fetch creds in order: env, file, IAM
        self.credentials = AWSCredentials{ key: "".to_string(), secret: "".to_string() };

        let mut env_provider = EnvironmentCredentialsProvider::new();
        env_provider.refresh();
        let credentials = env_provider.get_credentials();

        if creds_have_values(credentials) {
            println!("using creds from env: {}, {}", credentials.get_aws_access_key_id(), credentials.get_aws_secret_key());
            self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(), secret: credentials.get_aws_secret_key().to_string() };
            return;
        } else {
            // try the file provider
            let mut file_provider = FileCredentialsProvider::new();
            file_provider.refresh();
            let credentials = file_provider.get_credentials();
            println!("using creds from file: {}, {}", credentials.get_aws_access_key_id(), credentials.get_aws_secret_key());

            if creds_have_values(credentials) {
                self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(), secret: credentials.get_aws_secret_key().to_string() };
            } else {
                let mut iam_provider = IAMRoleCredentialsProvider::new();
                iam_provider.refresh();
                let credentials = iam_provider.get_credentials();
                if creds_have_values(credentials) {
                    println!("using creds from IAM: {}, {}", credentials.get_aws_access_key_id(), credentials.get_aws_secret_key());
                    self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(), secret: credentials.get_aws_secret_key().to_string() };
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
