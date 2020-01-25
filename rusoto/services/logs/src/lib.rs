
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p><p>You can use Amazon CloudWatch Logs to monitor, store, and access your log files from Amazon EC2 instances, AWS CloudTrail, or other sources. You can then retrieve the associated log data from CloudWatch Logs using the CloudWatch console, CloudWatch Logs commands in the AWS CLI, CloudWatch Logs API, or CloudWatch Logs SDK.</p> <p>You can use CloudWatch Logs to:</p> <ul> <li> <p> <b>Monitor logs from EC2 instances in real-time</b>: You can use CloudWatch Logs to monitor applications and systems using log data. For example, CloudWatch Logs can track the number of errors that occur in your application logs and send you a notification whenever the rate of errors exceeds a threshold that you specify. CloudWatch Logs uses your log data for monitoring; so, no code changes are required. For example, you can monitor application logs for specific literal terms (such as &quot;NullReferenceException&quot;) or count the number of occurrences of a literal term at a particular position in log data (such as &quot;404&quot; status codes in an Apache access log). When the term you are searching for is found, CloudWatch Logs reports the data to a CloudWatch metric that you specify.</p> </li> <li> <p> <b>Monitor AWS CloudTrail logged events</b>: You can create alarms in CloudWatch and receive notifications of particular API activity as captured by CloudTrail and use the notification to perform troubleshooting.</p> </li> <li> <p> <b>Archive log data</b>: You can use CloudWatch Logs to store your log data in highly durable storage. You can change the log retention setting so that any log events older than this setting are automatically deleted. The CloudWatch Logs agent makes it easy to quickly send both rotated and non-rotated log data off of a host and into the log service. You can then access the raw log data when you need it.</p> </li> </ul></p>
//!
//! If you're using the service, you're probably looking for [CloudWatchLogsClient](struct.CloudWatchLogsClient.html) and [CloudWatchLogs](trait.CloudWatchLogs.html).
//!
//! # Examples
//!
//! ## Put Log Events
//!
//! The following code shows a simple example of using Rusoto's CloudWatchLogs
//! API to send a single log event to the 'testing' log stream in the 'testing'
//! log group.
//!
//! ```rust,no_run
//! use chrono::Utc;
//! 
//! use rusoto_core::Region;
//! use rusoto_logs::{
//!     CloudWatchLogs, CloudWatchLogsClient, DescribeLogStreamsRequest, InputLogEvent,
//!     PutLogEventsRequest,
//! };
//! use std::default::Default;
//! 
//! #[tokio::main]
//! async fn main() {
//!     const LOG_GROUP_NAME: &str = "testing";
//!     const LOG_STREAM_NAME: &str = "testing";
//! 
//!     let client = CloudWatchLogsClient::new(Region::UsEast2);
//! 
//!     let input_log_event = InputLogEvent {
//!         message: "Test log message".to_string(),
//!         timestamp: Utc::now().timestamp_millis(), // milliseconds epoch
//!     };
//! 
//!     // We need the log stream to get the sequence token
//!     let mut desc_streams_req: DescribeLogStreamsRequest = Default::default();
//!     desc_streams_req.log_group_name = LOG_GROUP_NAME.to_string();
//!     let streams_resp = client.describe_log_streams(desc_streams_req).await;
//!     let log_streams = streams_resp.unwrap().log_streams.unwrap();
//!     let stream = &log_streams
//!         .iter()
//!         .find(|s| s.log_stream_name == Some(LOG_STREAM_NAME.to_string()))
//!         .unwrap();
//!     let sequence_token = stream.upload_sequence_token.clone();
//! 
//!     let put_log_events_request = PutLogEventsRequest {
//!         log_events: vec![input_log_event], // > 1 must sort by timestamp ASC
//!         log_group_name: LOG_GROUP_NAME.to_string(),
//!         log_stream_name: LOG_STREAM_NAME.to_string(),
//!         sequence_token,
//!     };
//! 
//!     let resp = client.put_log_events(put_log_events_request).await;
//!     println!("{:#?}", resp);
//! }
//! ```

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
