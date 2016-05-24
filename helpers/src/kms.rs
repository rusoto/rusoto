//! KMS helper types.

use rusoto::{AwsResult, ProvideAwsCredentials, Region};
use rusoto::kms::{
    KmsClient,
    ListKeysRequest,
    ListKeysResponse,
};

pub struct KmsHelper<P> where P: ProvideAwsCredentials {
	client: KmsClient<P>
}

impl <P: ProvideAwsCredentials> KmsHelper<P> {
    /// Create a new KMS helper.
    pub fn new(credentials: P, region: Region) -> KmsHelper<P> {
        KmsHelper {
            client: KmsClient::new(credentials, region)
        }
    }

    pub fn list_keys(&mut self) -> AwsResult<ListKeysResponse> {
        let req = ListKeysRequest::default();
        self.client.list_keys(&req)
    }
}
