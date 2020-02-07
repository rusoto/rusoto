#![cfg(feature = "rds")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_core::Region;
use rusoto_rds::{
    CreateOptionGroupMessage, DeleteOptionGroupMessage, DescribeDBClustersMessage, Rds, RdsClient,
    Tag,
};

#[tokio::test]
async fn should_describe_db_clusters() {
    let _ = env_logger::try_init();
    let client = RdsClient::new(Region::UsEast1);
    let request = DescribeDBClustersMessage::default();

    let result = client.describe_db_clusters(request).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn should_create_destroy_options_group() {
    let _ = env_logger::try_init();
    let client = RdsClient::new(Region::UsEast1);

    let mut tags: Vec<Tag> = Vec::new();
    let tag = Tag {
        key: Some("rkey".to_string()),
        value: Some("rvalue".to_string()),
    };
    tags.push(tag);

    let create_opt_group_request = CreateOptionGroupMessage {
        engine_name: "mysql".to_string(),
        major_engine_version: "5.6".to_string(),
        option_group_description: "rusotogroup".to_string(),
        option_group_name: "rusotogroup".to_string(),
        tags: Some(tags),
    };

    let result = client.create_option_group(create_opt_group_request).await;
    println!("{:#?}", result);
    assert!(result.is_ok());

    // Check ListTagsForResource to ensure tags got applied
    // (They do not appear in the Console, so there's probably a bug)

    let delete_opt_group_req = DeleteOptionGroupMessage {
        option_group_name: "rusotogroup".to_string(),
    };
    let delete_result = client.delete_option_group(delete_opt_group_req).await;
    println!("{:#?}", delete_result);
    assert!(delete_result.is_ok());
}
