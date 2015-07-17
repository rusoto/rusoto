use credentials::*;
use error::*;

pub trait AWSRequest<R> {
	fn execute(&self, creds: &AWSCredentials, region: &str) -> Result<R, AWSError>;
}