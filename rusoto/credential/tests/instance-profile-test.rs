extern crate tokio_core;

use rusoto_credential::{InstanceMetadataProvider, ProvideAwsCredentials};
use tokio_core::reactor::Core;
use std::time::Duration;

#[test]
fn it_fetches_basic_role() {
  // set env vars to point to local provider
  let mut provider = InstanceMetadataProvider::new();
  provider.set_timeout(Duration::from_secs(5));

  let creds_future = provider.credentials();
  let mut core = Core::new().unwrap();
  let creds = match core.run(creds_future) {
    Ok(creds) => creds,
    Err(e) => panic!("Got error: {:?}", e),
  };

  assert_eq!(creds.aws_access_key_id(), "Access_key_id_value");
  assert_eq!(creds.aws_secret_access_key(), "Secret_access_key_value");
}