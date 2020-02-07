#![cfg(feature = "glacier")]

extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_glacier;

use rusoto_core::Region;
use rusoto_glacier::{Glacier, GlacierClient, ListVaultsInput};

#[tokio::test]
async fn should_list_vaults() {
    let _ = env_logger::try_init();
    let client = GlacierClient::new(Region::UsWest2);
    // account id can be provided or use the account that signed the request with `-`.
    // http://docs.aws.amazon.com/amazonglacier/latest/dev/api-vaults-get.html
    let request = ListVaultsInput {
        account_id: "-".to_string(),
        ..Default::default()
    };

    let result = client.list_vaults(request).await.unwrap();
    println!("{:#?}", result);
}
