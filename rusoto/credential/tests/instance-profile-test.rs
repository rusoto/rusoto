use rusoto_credential::{InstanceMetadataProvider, ProvideAwsCredentials};
use std::time::Duration;

// This test is marked ignored because it requires special setup.
// It's run with the `credential_integration_test` Makefile target.
#[tokio::test]
#[ignore]
async fn it_fetches_basic_role() {
    // set env vars to point to local provider
    let mut provider = InstanceMetadataProvider::new();
    provider.set_timeout(Duration::from_secs(5));
    provider.set_ip_addr_with_port("127.0.0.1", "8080");

    let creds = provider.credentials().await.expect("credentials");

    assert_eq!(creds.aws_access_key_id(), "Access_key_id_value");
    assert_eq!(creds.aws_secret_access_key(), "Secret_access_key_value");
    assert_eq!(creds.token().as_ref(), Some(&"AAAAA".to_string()));
    let dt = match creds.expires_at().as_ref() {
        Some(d) => d.to_string(),
        None => panic!("Expiration should be present"),
    };
    assert_eq!(dt, "2015-08-04 06:32:37 UTC");
}
