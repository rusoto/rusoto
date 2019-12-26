#![cfg(feature = "logs")]
#![deny(warnings)]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_logs;

use rusoto_core::credential::DefaultCredentialsProvider;
use rusoto_core::encoding::ContentEncoding;
use rusoto_core::encoding::DEFAULT_GZIP_COMPRESSION_LEVEL;
use rusoto_core::{Client, HttpClient, Region, RusotoError};
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, CreateLogGroupError, CreateLogGroupRequest,
    CreateLogStreamError, CreateLogStreamRequest, DeleteLogGroupRequest, DescribeLogStreamsRequest,
    GetLogEventsRequest, GetLogEventsResponse, InputLogEvent, PutLogEventsRequest,
};
use std::fs;
use std::time::SystemTime;
use std::{thread, time};

async fn rusoto_logs_test_executor(client: CloudWatchLogsClient, log_group: &str, log_stream: &str) {
    let _ = env_logger::try_init();

    // First, create the log group
    let mut create_log_group_req: CreateLogGroupRequest = Default::default();
    create_log_group_req.log_group_name = log_group.to_string();
    client
        .create_log_group(create_log_group_req)
        .await
        .unwrap_or_else(|e| {
            match e {
                RusotoError::Service(CreateLogGroupError::ResourceAlreadyExists(err)) => {
                    warn!("CreateLogGroupError::ResourceAlreadyExists: {}", err);
                    // It's fine, continue
                }
                err => {
                    panic!("Failed to create log group: {}", err);
                }
            }
        });

    // Create the log stream for that log group
    let create_log_stream_req = CreateLogStreamRequest {
        log_group_name: log_group.to_string(),
        log_stream_name: log_stream.to_string(),
    };
    client
        .create_log_stream(create_log_stream_req)
        .await
        .unwrap_or_else(|e| {
            match e {
                RusotoError::Service(CreateLogStreamError::ResourceAlreadyExists(err)) => {
                    warn!("CreateLogStreamError::ResourceAlreadyExists: {}", err);
                    // It's fine, continue
                }
                err => {
                    panic!("Failed to create log group: {}", err);
                }
            }
        });

    // Describe log stream to fetch sequence token
    let mut desc_streams_req: DescribeLogStreamsRequest = Default::default();
    desc_streams_req.log_group_name = log_group.to_owned();
    desc_streams_req.log_stream_name_prefix = Some(log_stream.to_owned());
    let streams_resp = client.describe_log_streams(desc_streams_req).await;
    let log_streams = streams_resp.unwrap().log_streams.unwrap();
    let stream = &log_streams
        .iter()
        .find(|s| s.log_stream_name == Some(log_stream.to_string()))
        .unwrap();
    let upload_sequence_token = stream.upload_sequence_token.clone();

    // Prepare event data with a huge log message
    let data = fs::read_to_string("tests/sample-data/sagan.log").expect("Unable to read file");
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let input_event = InputLogEvent {
        message: data,
        timestamp: now,
    };
    let mut input_events = Vec::<InputLogEvent>::new();
    input_events.push(input_event.clone());

    // Now put the log event
    let put_log_req = PutLogEventsRequest {
        log_events: input_events.clone(),
        log_group_name: log_group.to_owned(),
        log_stream_name: log_stream.to_owned(),
        sequence_token: upload_sequence_token,
    };
    client
        .put_log_events(put_log_req.clone())
        .await
        .unwrap_or_else(|e| panic!("Failed on put log events: {:?}", e));

    // Get log events back to verify if it successfully reached
    let mut get_log_req: GetLogEventsRequest = Default::default();
    get_log_req.log_group_name = log_group.to_owned();
    get_log_req.log_stream_name = log_stream.to_owned();
    // Restrict timestamp to fetch only our event
    get_log_req.start_time = Some(input_event.timestamp - 1);
    get_log_req.end_time = Some(input_event.timestamp + 1);

    // It takes time for Cloudwatch to organize events. Retry until it's ready.
    let retry_count = 5;
    let mut i = 0;
    let get_log_resp: Option<GetLogEventsResponse> = loop {
        if i == retry_count {
            break None;
        }
        info!(
            "Trying to get log events: {} out of {} times",
            i, retry_count
        );
        let get_log_resp = client
            .get_log_events(get_log_req.clone())
            .await
            .unwrap_or_else(|e| panic!("Failed on get log events: {}", e));
        if (get_log_resp.events).as_ref().unwrap().len() != 0_usize {
            break Some(get_log_resp);
        } else {
            i = i + 1;
            // sleep for one second before calling again
            thread::sleep(time::Duration::from_secs(1));
        }
    };

    if get_log_resp.is_none() {
        panic!("Failed to get log events for {} times", retry_count);
    }

    // Assert that our event is there
    let output_event = &get_log_resp.unwrap().events.unwrap()[0];
    assert_eq!(
        output_event.message.as_ref().unwrap(),
        &input_events[0].message
    );
    assert_eq!(output_event.timestamp.unwrap(), input_events[0].timestamp);

    // Delete log group. This will clear all resources so we would not leak anything.
    let del_log_group_req = DeleteLogGroupRequest {
        log_group_name: log_group.to_owned(),
    };
    client
        .delete_log_group(del_log_group_req)
        .await
        .unwrap_or_else(|e| panic!("Failed to delete log group:/n{}", e));
}

#[tokio::test]
async fn should_put_log_events() {
    let http_client = HttpClient::new().expect("failed to create request dispatcher");
    let creds_provider =
        DefaultCredentialsProvider::new().expect("failed to create default credentials provider");
    let client = CloudWatchLogsClient::new_with(http_client, creds_provider, Region::UsWest2);
    rusoto_logs_test_executor(client, "should_put_log_events", "should_put_log_events").await;
}

#[tokio::test]
async fn should_put_log_events_with_gzip_encoding() {
    let http_client = HttpClient::new().expect("failed to create request dispatcher");
    let creds_provider =
        DefaultCredentialsProvider::new().expect("failed to create default credentials provider");
    let encoding = ContentEncoding::Gzip(Some(1024), DEFAULT_GZIP_COMPRESSION_LEVEL);
    let client = CloudWatchLogsClient::new_with_client(
        // Use gzip encoding with client
        Client::new_with_encoding(creds_provider, http_client, encoding),
        Region::UsWest2,
    );
    rusoto_logs_test_executor(
        client,
        "should_put_log_events_with_encoding",
        "should_put_log_events_with_encoding",
    ).await;
}
