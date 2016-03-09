#![cfg(feature = "ecs")]

extern crate rusoto;

use rusoto::ecs::{ECSClient, ECSError, ListClustersRequest};
use rusoto::credentials::ChainProvider;
use rusoto::regions::Region;

#[test]
fn main() {
    let credentials = ChainProvider::new().unwrap();
    let region = Region::UsEast1;
    let mut ecs = ECSClient::new(
        credentials,
        &region
    );

    // http://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ListClusters.html
    match ecs.list_clusters(&ListClustersRequest::default()) {
        Ok(clusters) => {
            for arn in clusters.clusterArns.unwrap_or(vec![]) {
                println!("arn -> {:?}", arn);
            }
        },
        Err(err) => {
            panic!("Error listing container instances {:#?}", err);
        }
    }

    match ecs.list_clusters(
        &ListClustersRequest {
            nextToken: Some("bogus".to_owned()), ..Default::default()
        }) {
        Ok(_) => panic!("this should have been an InvalidParameterException ECSError"),
        Err(err) => {
            assert_eq!(err,  ECSError {
                __type: "InvalidParameterException".to_owned(),
                message: Some("Invalid token bogus".to_owned())
            })
        }
    }
}
