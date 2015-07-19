extern crate regex;
use std::env::*;

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

//struct ProfileCredentialsError;

impl DefaultAWSCredentialsProviderChain {
		
	fn creds_from_env() -> Result<AWSCredentials, VarError> {
		let env_key = try!(var("AWS_ACCESS_KEY_ID"));
		let env_secret = try!(var("AWS_SECRET_KEY"));

		Ok(AWSCredentials { key: env_key, secret: env_secret }) 
	}
	/*
	fn creds_from_profile() -> Result<AWSCredentials, ProfileCredentialsError> {
			
		let file = match File::open("test_file.txt") {
			Ok(file) => file,
			Err(..) => panic!("room")
		};
		
		let reader = BufReader::new(&file);
		for line in reader.lines() {
			let _s = line.unwrap();
						
		}
	
		Err(ProfileCredentialsError)
	
	}
	*/
}