#![cfg(feature = "ecs")]

extern crate rusoto;

use rusoto::ecs::{EcsClient, ListClustersRequest};
use rusoto::{AwsError, ChainProvider, Region};

#[test]
fn main() {
    let credentials = ChainProvider::new().unwrap();

    let ecs = EcsClient::new(credentials, Region::UsEast1);

    // http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ListClusters.html
    match ecs.list_clusters(&ListClustersRequest::default()) {
        Ok(clusters) => {
            for arn in clusters.cluster_arns.unwrap_or(vec![]) {
                println!("arn -> {:?}", arn);
            }
        },
        Err(err) => {
            panic!("Error listing container instances {:#?}", err);
        }
    }

    match ecs.list_clusters(
        &ListClustersRequest {
            next_token: Some("bogus".to_owned()), ..Default::default()
        }) {
        Ok(_) => panic!("this should have been an InvalidParameterException ECSError"),
        Err(err) => {
            assert_eq!(err, AwsError::new("InvalidParameterException: Invalid token bogus"))
        }
    }
}
