#![cfg(feature = "ec2")]

extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::{Region, RusotoError};
use rusoto_ec2::{CreateSnapshotRequest, DescribeInstancesRequest, Ec2, Ec2Client};
use rusoto_ec2::{CreateTagsRequest, Tag};

use std::str;

#[tokio::test]
async fn main() {
    let ec2 = Ec2Client::new(Region::UsEast1);

    let mut req = DescribeInstancesRequest::default();
    req.instance_ids = Some(vec!["i-00000000".into(), "i-00000001".into()]);
    match ec2.describe_instances(req).await {
        Ok(_) => {
            panic!("DescribeInstances should fail");
        }
        Err(error) => {
            match error {
                RusotoError::Unknown(ref e) => {
                    assert!(str::from_utf8(&e.body).unwrap().contains("<Message>The instance IDs 'i-00000000, i-00000001' do not exist</Message>"), "Missing error message");
                }
                _ => {
                    panic!("Should have a typed error from EC2");
                }
            }
        }
    }
}

// Issue 383
#[tokio::test]
#[ignore]
#[should_panic(
    expected = "<Message>Request would have succeeded, but DryRun flag is set.</Message>"
)]
async fn dry_run() {
    let ec2 = Ec2Client::new(Region::UsEast1);
    let req = CreateSnapshotRequest {
        volume_id: "v-00000001".into(),
        dry_run: Some(true),
        ..Default::default()
    };
    let _ = ec2.create_snapshot(req).await.unwrap();
}

// Issue 387
#[tokio::test]
#[ignore]
#[should_panic(expected = "<Code>InvalidID</Code>")]
async fn query_serialization_name() {
    let ec2 = Ec2Client::new(Region::UsEast1);
    let req = CreateTagsRequest {
        dry_run: None,
        resources: vec!["v-00000001".into()],
        tags: vec![Tag {
            key: Some("key".into()),
            value: Some("val".into()),
        }],
    };
    let _ = ec2.create_tags(req).await.unwrap();
}
