//! Amazon Kinesis

include!(concat!(env!("OUT_DIR"), "/kinesis.rs"));

pub struct KinesisHelper<P> where P: ProvideAwsCredentials{
	client: KinesisClient<P>
}

impl<P: ProvideAwsCredentials> KinesisHelper<P> {
	pub fn new(credentials: P, region: Region) -> KinesisHelper<P> {
        KinesisHelper { client: KinesisClient::new(credentials, region) }
    }

    pub fn describe_stream(&mut self, stream_name: &str) -> Result<DescribeStreamOutput, DescribeStreamError> {
    	let ds_input = DescribeStreamInput {
    		exclusive_start_shard_id: None,
    		limit: None,
    		stream_name: String::from(stream_name)
    	};

    	self.client.describe_stream(&ds_input)

    }

    /// Create a stream idempotently.  i.e., don't fail if the stream already exists
	pub fn create_stream(&mut self, stream_name: &str, shard_count: i32) -> Result<(), CreateStreamError> {
		let cs_input = CreateStreamInput {
			shard_count: shard_count,
			stream_name: String::from(stream_name)
		};

		match self.client.create_stream(&cs_input) {
			Ok(_) | Err(CreateStreamError::ResourceInUseException(_)) => Ok(()),
			Err(e) => Err(e)
		}
	}

	/// Delete a stream idempotently.  i.e., don't fail if the stream doesn't exist
	pub fn delete_stream(&mut self, stream_name: &str) -> Result<(), DeleteStreamError> {
		let ds_input = DeleteStreamInput { stream_name: String::from(stream_name) };

		match self.client.delete_stream(&ds_input) {
			Ok(_) | Err(DeleteStreamError::ResourceNotFoundException(_)) => Ok(()),
			Err(e) => Err(e)
		}
	}
}