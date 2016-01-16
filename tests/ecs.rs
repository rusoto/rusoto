#![cfg(feature = "ecs")]

extern crate rusoto;

use rusoto::ecs::{ECSClient, ListClustersRequest};
use rusoto::credentials::DefaultAWSCredentialsProviderChain;
use rusoto::regions::Region;

#[test]
fn main() {
    let credentials = DefaultAWSCredentialsProviderChain::new();
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
            println!("Error listing container instances {:#?}", err);
        }
    }
}
