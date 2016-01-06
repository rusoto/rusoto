pub struct KMSHelper<'a> {
	client: KMSClient<'a>
}

impl<'a> KMSHelper<'a> {
    /// Creates a new KMS helper
    pub fn new<P: AWSCredentialsProvider + 'a>(credentials: P, region:&'a Region) -> KMSHelper<'a> {
	KMSHelper { client: KMSClient::new(credentials, region) }
    }

    pub fn list_keys(&mut self) -> Result<ListKeysResponse> {
        let mut req = ListKeysRequest::default();
        self.client.list_keys(&req)
    }

}

#[derive(Debug, Deserialize)]
pub struct KMSError {
	__type: String,
	message: String
}

impl From<AWSError> for KMSError {
	fn from(err: AWSError) -> KMSError {
		let AWSError(message) = err;
		KMSError { __type: "Unknown".to_string(), message: message.to_string() }
	}
}

pub type Result<T> = result::Result<T, KMSError>;

fn parse_error(body: &str) -> KMSError {
	if let Ok(decoded) = from_str::<KMSError>(&body) {
		decoded
	} else {
		KMSError { __type: "DecodeError".to_string(), message: body.to_string() }
	}
}
