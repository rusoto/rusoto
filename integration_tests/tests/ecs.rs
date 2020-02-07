#![cfg(feature = "ecs")]

extern crate rusoto_core;
extern crate rusoto_ecs;

use rusoto_core::request::{HttpClient, HttpConfig};
use rusoto_core::{credential::DefaultCredentialsProvider, Region, RusotoError};
use rusoto_ecs::{Ecs, EcsClient, ListClustersError, ListClustersRequest};

#[tokio::test]
async fn main() {
    // EcsClient configuration demonstrates setting the hyper read_buf_size option
    // to 2MB:
    let cred_provider = DefaultCredentialsProvider::new().unwrap();
    let mut http_config_with_bigger_buffer = HttpConfig::new();
    http_config_with_bigger_buffer.read_buf_size(1024 * 1024 * 2);
    let http_provider = HttpClient::new_with_config(http_config_with_bigger_buffer).unwrap();

    let ecs = EcsClient::new_with(http_provider, cred_provider, Region::UsEast1);

    // http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ListClusters.html
    match ecs.list_clusters(ListClustersRequest::default()).await {
        Ok(clusters) => {
            for arn in clusters.cluster_arns.unwrap_or(vec![]) {
                println!("arn -> {:?}", arn);
            }
        }
        Err(err) => {
            panic!("Error listing container instances {:#?}", err);
        }
    }

    match ecs
        .list_clusters(ListClustersRequest {
            next_token: Some("bogus".to_owned()),
            ..Default::default()
        })
        .await
    {
        Err(RusotoError::Service(ListClustersError::InvalidParameter(msg))) => {
            assert!(msg.contains("Invalid token bogus"))
        }
        _ => panic!("this should have been an InvalidParameterException ECSError"),
    }
}
