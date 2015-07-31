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
		let env_creds = DefaultAWSCredentialsProviderChain::creds_from_env();
		if env_creds.is_ok() {
			self.credentials = env_creds.unwrap();
			return;
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

	fn creds_from_env() -> Result<AWSCredentials, VarError> {
		let env_key = try!(var("AWS_ACCESS_KEY_ID"));
		let env_secret = try!(var("AWS_SECRET_KEY"));

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

        return  Ok(AWSCredentials{ key: profile_key, secret: secret_key });

		// Err(ProfileCredentialsError)

	}

    // IAM role
}
