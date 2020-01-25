//! KMS helper types.

use rusoto_core::{Region, RusotoResult};
use rusoto_kms::{
    Kms,
    KmsClient,
    ListKeysError,
    ListKeysRequest,
    ListKeysResponse,
};

pub struct KmsHelper {
    client: KmsClient,
}

impl KmsHelper {
    /// Create a new KMS helper.
    pub fn new(region: Region) -> KmsHelper {
        KmsHelper {
            client: KmsClient::new(region)
        }
    }

    pub async fn list_keys(&mut self) -> RusotoResult<ListKeysResponse, ListKeysError> {
        let req = ListKeysRequest::default();
        self.client.list_keys(req).await
    }
}
