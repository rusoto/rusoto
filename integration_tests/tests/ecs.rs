#![cfg(feature = "ecs")]

extern crate rusoto_core;
extern crate rusoto_ecs;

use rusoto_ecs::{Ecs, EcsClient, ListClustersRequest, ListClustersError};
use rusoto_core::Region;
use rusoto_core::DefaultCredentialsProvider;
use rusoto_core::request::HttpClient;

#[test]
fn main() {
    let cred_provider =  DefaultCredentialsProvider::new().unwrap();
    let http_provider = HttpClient::new_with_readbuff(Some(1024 * 1024 * 2)).unwrap();
    let ecs = EcsClient::new_with(http_provider, cred_provider, Region::UsEast1);

    // http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ListClusters.html
    match ecs.list_clusters(ListClustersRequest::default()).sync() {
        Ok(clusters) => {
            for arn in clusters.cluster_arns.unwrap_or(vec![]) {
                println!("arn -> {:?}", arn);
            }
        }
        Err(err) => {
            panic!("Error listing container instances {:#?}", err);
        }
    }

    match ecs.list_clusters(ListClustersRequest {
        next_token: Some("bogus".to_owned()),
        ..Default::default()
    }).sync() {
        Err(ListClustersError::InvalidParameter(msg)) => {
            assert!(msg.contains("Invalid token bogus"))
        }
        _ => panic!("this should have been an InvalidParameterException ECSError"),
    }
}
