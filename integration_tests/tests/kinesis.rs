#![cfg(feature = "kinesis")]

extern crate rusoto_core;
extern crate rusoto_kinesis;

use std::cell::RefCell;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::time::Duration;

use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::FutureExt;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tokio::time::{delay_for, timeout};

use rusoto_core::Region;
use rusoto_kinesis::{Kinesis, KinesisClient, ListStreamsInput};

#[async_trait]
trait AsyncCleanup {
    async fn cleanup(&mut self);
}

async fn ensure_cleanup<C, B, BF, R>(ctx: &RefCell<C>, block: B) -> R
where
    C: AsyncCleanup,
    BF: Future<Output = R>,
    B: FnOnce() -> BF,
{
    let result = AssertUnwindSafe(block()).catch_unwind().await;
    ctx.borrow_mut().cleanup().await;
    match result {
        Ok(r) => r,
        Err(p) => panic!(p),
    }
}

#[derive(Clone)]
struct TestKinesisStream {
    client: KinesisClient,
    arn: String,
    name: String,
}

impl TestKinesisStream {
    async fn new(client: &KinesisClient) -> TestKinesisStream {
        let uid: String = thread_rng()
            .sample_iter(&Alphanumeric {})
            .take(10)
            .collect();
        let name = format!("rusoto-test-{}", uid);

        println!("Creating a test kinesis stream {}...", name);
        client
            .create_stream(rusoto_kinesis::CreateStreamInput {
                shard_count: 1,
                stream_name: name.clone(),
            })
            .await
            .unwrap();

        let describe_result = client
            .describe_stream(rusoto_kinesis::DescribeStreamInput {
                stream_name: name.clone(),

                ..Default::default()
            })
            .await
            .unwrap();
        let arn = describe_result.stream_description.stream_arn;

        TestKinesisStream {
            client: client.clone(),
            arn,
            name,
        }
    }
}

#[async_trait]
impl AsyncCleanup for TestKinesisStream {
    async fn cleanup(&mut self) {
        self.client
            .delete_stream(rusoto_kinesis::DeleteStreamInput {
                enforce_consumer_deletion: Some(true),
                stream_name: self.name.clone(),
            })
            .await
            .unwrap();
        println!("Test kinesis stream {} deleted", self.name);
    }
}

#[tokio::test]
async fn should_list_streams() {
    let client = KinesisClient::new(Region::UsEast1);
    let request = ListStreamsInput::default();

    client.list_streams(request).await.unwrap();
}

#[tokio::test]
async fn should_listen_for_shard_events() {
    let client = KinesisClient::new(Region::default());
    let test_stream = RefCell::new(TestKinesisStream::new(&client).await);
    ensure_cleanup(&test_stream, || async {
        let stream = test_stream.borrow();

        loop {
            let steam_desc_result = client
                .describe_stream(rusoto_kinesis::DescribeStreamInput {
                    stream_name: stream.name.clone(),

                    ..Default::default()
                })
                .await
                .unwrap();

            if steam_desc_result.stream_description.stream_status == "CREATING" {
                println!("Stream {} still initializing, waiting...", stream.name);
                delay_for(std::time::Duration::from_secs(2)).await;
                continue;
            } else {
                break;
            }
        }

        if let Ok(kms_id) = std::env::var("KMS_ID") {
            client
                .start_stream_encryption(rusoto_kinesis::StartStreamEncryptionInput {
                    encryption_type: "KMS".to_string(),
                    key_id: kms_id,
                    stream_name: stream.name.clone(),
                })
                .await
                .unwrap();
        }

        tokio::time::delay_for(Duration::from_secs(10)).await;

        let consumer_result = client
            .register_stream_consumer(rusoto_kinesis::RegisterStreamConsumerInput {
                consumer_name: "test-consumer".to_string(),
                stream_arn: stream.arn.clone(),
            })
            .await
            .unwrap();

        loop {
            let consumer_desc_result = client
                .describe_stream_consumer(rusoto_kinesis::DescribeStreamConsumerInput {
                    consumer_arn: Some(consumer_result.consumer.consumer_arn.clone()),
                    stream_arn: Some(stream.arn.clone()),

                    ..Default::default()
                })
                .await
                .unwrap();

            if consumer_desc_result.consumer_description.consumer_status == "CREATING" {
                println!(
                    "Consumer for stream {} still initializing, waiting...",
                    stream.name
                );
                delay_for(std::time::Duration::from_secs(2)).await;
                continue;
            } else {
                break;
            }
        }

        let shards_result = client
            .list_shards(rusoto_kinesis::ListShardsInput {
                stream_name: Some(stream.name.clone()),

                ..Default::default()
            })
            .await
            .unwrap();
        let shard_id = shards_result.shards.unwrap()[0].shard_id.clone();

        let mut event_stream = client
            .subscribe_to_shard(rusoto_kinesis::SubscribeToShardInput {
                consumer_arn: consumer_result.consumer.consumer_arn,
                shard_id: shard_id.clone(),
                starting_position: rusoto_kinesis::StartingPosition {
                    sequence_number: None,
                    timestamp: None,
                    type_: "TRIM_HORIZON".to_string(),
                },
            })
            .await
            .unwrap()
            .event_stream;

        let mut events: Vec<rusoto_kinesis::Record> = Vec::new();

        for i in 0i32..5 {
            let message = format!("Record {}", i);

            let result = client
                .put_record(rusoto_kinesis::PutRecordInput {
                    data: bytes::Bytes::copy_from_slice(message.as_bytes()),
                    partition_key: "hello".to_string(),
                    stream_name: stream.name.clone(),

                    ..Default::default()
                })
                .await
                .unwrap();

            eprintln!("Put record result: {:?}", result);
        }

        let read_events_future = async {
            while let Some(item) = event_stream.next().await {
                let payload = item.unwrap();
                println!("Got event from the event stream: {:?}", payload);

                match payload {
                    rusoto_kinesis::SubscribeToShardEventStreamItem::SubscribeToShardEvent(e) => {
                        events.extend(e.records);
                    }
                    _ => {}
                }
            }
        };

        timeout(Duration::from_secs(10), read_events_future)
            .await
            .unwrap_err();

        println!("Events: {:?}", events);
        assert_eq!(events.len(), 5);
    })
    .await;
    println!("About to exit the upper test fn");
}
