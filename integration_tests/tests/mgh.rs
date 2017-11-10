#![cfg(feature = "mgh")]

extern crate rusoto_core;
extern crate rusoto_mgh;

use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};
use rusoto_mgh::{MigrationHub, MigrationHubClient, ListMigrationTasksRequest};

#[test]
fn should_list_migration_tasks() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = MigrationHubClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = ListMigrationTasksRequest::default();

    let result = client.list_migration_tasks(&request).sync().unwrap();
    println!("Results: {:?}", result);
}