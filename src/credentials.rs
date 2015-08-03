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
        return EnvironmentCredentialsProvider {credentials: AWSCredentials{ key: "a".to_string(), secret: "b".to_string() } };
    }

	fn refresh(&mut self) {
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
            panic!("Couldn't find credentials in environment.");
            // return Err(CredentialErr::NoEnvironmentVariables);
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
        return FileCredentialsProvider {credentials: AWSCredentials{ key: "a".to_string(), secret: "b".to_string() } };
    }

	fn refresh(&mut self) {
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

        let profile_key = String::from("ACCESS_KEY");
        let secret_key = String::from("SECRET_KEY");

        self.credentials = AWSCredentials{ key: profile_key, secret: secret_key };
    }

    fn get_credentials(&self) -> &AWSCredentials {
		return &self.credentials;
	}
}

// class for IAM role
pub struct IAMRoleCredentialsProvider {
    credentials: AWSCredentials
}

pub struct DefaultAWSCredentialsProviderChain {
    credentials: AWSCredentials
}

impl DefaultAWSCredentialsProviderChain {
    pub fn new() -> DefaultAWSCredentialsProviderChain {
        return DefaultAWSCredentialsProviderChain {credentials: AWSCredentials{ key: "a".to_string(), secret: "b".to_string() } };
    }

    pub fn get_credentials(&self) -> &AWSCredentials {
        return &self.credentials;
    }

    pub fn refresh(&mut self) {
        // fetch creds in order: env, file, IAM
        // pretend we got credentials here
        self.credentials = AWSCredentials{ key: "a2".to_string(), secret: "b2".to_string() };

        let mut env_provider = EnvironmentCredentialsProvider::new();
        env_provider.refresh();
        let credentials = env_provider.get_credentials();
        println!("creds from env: {}, {}", credentials.get_aws_access_key_id(), credentials.get_aws_secret_key());
        // if not blank, use it to populate self.credentials

        if credentials.get_aws_access_key_id().len() <= 0 || credentials.get_aws_secret_key().len() <= 0 {
            panic!("Couldn't find credentials in environment.");
            // return Err(CredentialErr::NoEnvironmentVariables);
        };
        self.credentials = AWSCredentials{ key: credentials.get_aws_access_key_id().to_string(), secret: credentials.get_aws_secret_key().to_string() }; //credentials.clone();
    }
}



// From http://blogs.aws.amazon.com/security/post/Tx3D6U6WSFGOK2H/A-New-and-Standardized-Way-to-Manage-Credentials-in-the-AWS-SDKs
// 1. environment variables
// 2. central credentials file (named profile is supplied, otherwise default)
// 3. IAM role (if running on an EC2 instance)
// impl DefaultAWSCredentialsProviderChain {
//
// }
