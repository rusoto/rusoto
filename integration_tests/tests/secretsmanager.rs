#![cfg(feature = "secretsmanager")]

extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;

use rusoto_core::Region;
use rusoto_secretsmanager::{ListSecretsRequest, SecretsManager, SecretsManagerClient};

#[tokio::test]
async fn should_list_invitations() {
    let _ = env_logger::try_init();
    let client = SecretsManagerClient::new(Region::UsWest2);
    let request = ListSecretsRequest {
        ..Default::default()
    };

    let result = client.list_secrets(request).await;
    assert!(result.is_ok());
}
