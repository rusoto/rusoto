//! ## Put Log Events
//!
//! The following code shows a simple example of using Rusoto's CloudWatchLogs
//! API to send a single log event to the 'testing' log stream in the 'testing'
//! log group.
use chrono::Utc;

use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogStreamsRequest, InputLogEvent,
    PutLogEventsRequest,
};
use std::default::Default;

#[tokio::main]
async fn main() {
    const LOG_GROUP_NAME: &str = "testing";
    const LOG_STREAM_NAME: &str = "testing";

    let client = CloudWatchLogsClient::new(Region::UsEast2);

    let input_log_event = InputLogEvent {
        message: "Test log message".to_string(),
        timestamp: Utc::now().timestamp_millis(), // milliseconds epoch
    };

    // We need the log stream to get the sequence token
    let mut desc_streams_req: DescribeLogStreamsRequest = Default::default();
    desc_streams_req.log_group_name = LOG_GROUP_NAME.to_string();
    let streams_resp = client.describe_log_streams(desc_streams_req).await;
    let log_streams = streams_resp.unwrap().log_streams.unwrap();
    let stream = &log_streams
        .iter()
        .find(|s| s.log_stream_name == Some(LOG_STREAM_NAME.to_string()))
        .unwrap();
    let sequence_token = stream.upload_sequence_token.clone();

    let put_log_events_request = PutLogEventsRequest {
        log_events: vec![input_log_event], // > 1 must sort by timestamp ASC
        log_group_name: LOG_GROUP_NAME.to_string(),
        log_stream_name: LOG_STREAM_NAME.to_string(),
        sequence_token,
    };

    let resp = client.put_log_events(put_log_events_request).await;
    println!("{:#?}", resp);
}
