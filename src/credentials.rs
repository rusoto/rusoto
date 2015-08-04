use std::env::*;
use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::ascii::AsciiExt;

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

// class for environment
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

// class for file based
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

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
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
// TODO: implement
pub struct IAMRoleCredentialsProvider {
    credentials: AWSCredentials
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
                // or try IAM role
                panic!("Couldn't find credentials in env or file.  IAM roles not yet supported.");
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
