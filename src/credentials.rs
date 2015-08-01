extern crate regex;
use std::env::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

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

pub trait AWSCredentialsProvider {
	fn get_credentials(&self) -> &AWSCredentials;
	fn refresh(&mut self);
}

pub struct EnvironmentCredentialsProvider;

pub struct DefaultAWSCredentialsProviderChain {
	credentials: AWSCredentials
}

impl AWSCredentialsProvider for DefaultAWSCredentialsProviderChain {
	fn refresh(&mut self) {
		let env_creds = DefaultAWSCredentialsProviderChain::get_credentials();
        match env_creds {
            Ok(creds) => {
                self.credentials = creds;
	            return;
            }
            Err(_) => panic!("Couldn't open credentials anywhere."),
        }

		//let file_creds = DefaultAWSCredentialsProviderChain::creds_from_profile();
	}

	fn get_credentials(&self) -> &AWSCredentials {
		&self.credentials
	}

}

struct ProfileCredentialsError;

// From http://blogs.aws.amazon.com/security/post/Tx3D6U6WSFGOK2H/A-New-and-Standardized-Way-to-Manage-Credentials-in-the-AWS-SDKs
// 1. environment variables
// 2. central credentials file (named profile is supplied, otherwise default)
// 3. IAM role (if running on an EC2 instance)
impl DefaultAWSCredentialsProviderChain {

    fn get_credentials() -> Result<AWSCredentials, CredentialErr> {
        let env_return = match DefaultAWSCredentialsProviderChain::creds_from_env() {
            Ok(creds) => creds,
            Err(_) => AWSCredentials {key: "".to_string(), secret: "".to_string()},
        };
        if env_return.get_aws_secret_key().len() > 0 && env_return.get_aws_access_key_id().len() > 0 {
            return Ok(env_return)
        }

        let file_return = match DefaultAWSCredentialsProviderChain::creds_from_profile() {
            Ok(creds) => creds,
            Err(_) => AWSCredentials {key: "".to_string(), secret: "".to_string()},
        };
        if file_return.get_aws_secret_key().len() > 0 && file_return.get_aws_access_key_id().len() > 0 {
            return Ok(file_return)
        }
        panic!("Couldn't find any credentials to use.");
    }

	fn creds_from_env() -> Result<AWSCredentials, CredentialErr> {
        let env_key = match var("AWS_ACCESS_KEY_ID") {
            Ok(val) => val,
            Err(_) => {println!("couldn't find access key");
                "".to_string() }
        };
        let env_secret = match var("AWS_SECRET_KEY") {
            Ok(val) => val,
            Err(_) => {println!("couldn't find secret key");
                "".to_string() }
        };

        if env_key.len() <= 0 || env_secret.len() <= 0 {
            return Err(CredentialErr::NoEnvironmentVariables);
        }
		Ok(AWSCredentials { key: env_key, secret: env_secret })
	}

	fn creds_from_profile() -> Result<AWSCredentials, ProfileCredentialsError> {
        let path = Path::new("sample-credentials");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       Error::description(&why)),
            Ok(file) => file,
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       Error::description(&why)),
            Ok(_) => {},
        }

        let profile_key = String::from("foo");
        let secret_key = String::from("bar");

        return Ok(AWSCredentials{ key: profile_key, secret: secret_key });

		// Err(ProfileCredentialsError)
	}

    // IAM role
}
