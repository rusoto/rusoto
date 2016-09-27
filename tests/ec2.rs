#![cfg(feature = "ec2")]

extern crate rusoto;

use rusoto::ec2::{Ec2Client, CreateSnapshotRequest, DescribeInstancesRequest};
use rusoto::ec2::{CreateTagsRequest, Tag};
use rusoto::{DefaultCredentialsProvider, Region};
use std::error::Error;

#[test]
fn main() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let ec2 = Ec2Client::new(credentials, Region::UsEast1);

    let mut req = DescribeInstancesRequest::default();
    req.instance_ids = Some(vec!["i-00000000".into(), "i-00000001".into()]);
    match ec2.describe_instances(&req) {
        Ok(_) => {
            panic!("DescribeInstances should fail");
        },
        Err(error) => {
            assert!(error.description().contains("<Message>The instance IDs 'i-00000000, i-00000001' do not exist</Message>"), "Missing error message");
        }
    }

}

// Issue 383
#[test]
#[should_panic(expected="<Message>Request would have succeeded, but DryRun flag is set.</Message>")]
fn dry_run() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let ec2 = Ec2Client::new(credentials, Region::UsEast1);
    let req = CreateSnapshotRequest {
        volume_id: "v-00000001".into(),
        description: None,
        dry_run: Some(true)
    };
    let _ = ec2.create_snapshot(&req) .unwrap();
}

// Issue 387
#[test]
#[should_panic(expected="<Code>InvalidID</Code>")]
fn query_serialization_name() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let ec2 = Ec2Client::new(credentials, Region::UsEast1);
    let req = CreateTagsRequest {
        dry_run: None,
        resources: vec!["v-00000001".into()],
        tags: vec![Tag {
            key: Some("key".into()),
            value: Some("val".into())
        }]
    };
    let _ = ec2.create_tags(&req) .unwrap();
}
