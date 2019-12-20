#![cfg(feature = "mgh")]

extern crate rusoto_core;
extern crate rusoto_mgh;

use rusoto_core::Region;
use rusoto_mgh::{ListMigrationTasksRequest, MigrationHub, MigrationHubClient};

#[tokio::test]
async fn should_list_migration_tasks() {
    let client = MigrationHubClient::new(Region::UsWest2);
    let request = ListMigrationTasksRequest::default();

    let result = client.list_migration_tasks(request).await.unwrap();
    println!("Results: {:?}", result);
}
