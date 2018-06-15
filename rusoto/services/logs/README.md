
# Rusoto CloudWatchLogs
Rust SDK for Amazon CloudWatch Logs

You may be looking for:

* [An overview of Rusoto][rusoto-overview]
* [AWS services supported by Rusoto][supported-aws-services]
* [API documentation][api-documentation]
* [Getting help with Rusoto][rusoto-help]

## Requirements

Rust stable or beta are required to use Rusoto. Nightly is tested, but not guaranteed to be supported. Older
versions _may_ be supported. The currently supported Rust versions can be found in the Rusoto project 
[`travis.yml`](https://github.com/rusoto/rusoto/blob/master/.travis.yml).

On Linux, OpenSSL is required.

## Installation

To use `rusoto_logs` in your application, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rusoto_logs = "0.32.0"
```
## Example

Here is a full working example using the library to send a single log event:

```toml
# Cargo.toml
[dependencies]
chrono = "0.4.3"
dotenv = "0.13"
rusoto_core = "0.32"
rusoto_logs = "0.32"
```

```rust
// src/main.rs
extern crate chrono;
extern crate dotenv;
extern crate rusoto_core;
extern crate rusoto_logs;

use dotenv::dotenv;
use chrono::{Utc};

use std::default::Default;
use rusoto_core::{EnvironmentProvider, Region};
use rusoto_core::reactor::RequestDispatcher;
use rusoto_logs::{CloudWatchLogs, CloudWatchLogsClient, DescribeLogStreamsRequest, InputLogEvent, PutLogEventsRequest};

fn main() {
    const LOG_GROUP_NAME: &'static str = "testing";
    const LOG_STREAM_NAME: &'static str = "testing";
    // AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY stored in a local .env file
    dotenv().ok();
    let request_dispatcher: RequestDispatcher = Default::default();

    // EnvironmentProvider automatically grabs the credentials from the environment
    let client = CloudWatchLogsClient::new(request_dispatcher, EnvironmentProvider, Region::UsEast2);
    let input_log_event = InputLogEvent {
        message: "Test log message".to_string(),
        timestamp: Utc::now().timestamp_millis(), // milliseconds epoch
    };

    // We need the log stream to get the sequence token
    let mut desc_streams_req: DescribeLogStreamsRequest = Default::default();
    desc_streams_req.log_group_name = LOG_GROUP_NAME.to_string();
    let streams_resp = client.describe_log_streams(&desc_streams_req).sync();
    let log_streams = streams_resp.unwrap().log_streams.unwrap();
    let stream = &log_streams[0]; // if we had more than one stream, of course, we would find it by name
    let sequence_token = stream.upload_sequence_token.clone();

    let put_log_events_request = PutLogEventsRequest {
        log_events: vec![input_log_event], // > 1 must sort by timestamp ASC
        log_group_name: LOG_GROUP_NAME.to_string(),
        log_stream_name: LOG_STREAM_NAME.to_string(),
        sequence_token: sequence_token,
    };

    let resp = client.put_log_events(&put_log_events_request).sync();
    println!("{:#?}", resp);
}
```

## Contributing

See [CONTRIBUTING][contributing].

## License

Rusoto is distributed under the terms of the MIT license.

See [LICENSE][license] for details.

[api-documentation]: https://rusoto.github.io/rusoto/rusoto/ "API documentation"
[license]: https://github.com/rusoto/rusoto/blob/master/LICENSE "MIT License"
[contributing]: https://github.com/rusoto/rusoto/blob/master/CONTRIBUTING.md "Contributing Guide"
[rusoto-help]: https://www.rusoto.org/help.html "Getting help with Rusoto"
[rusoto-overview]: https://www.rusoto.org/ "Rusoto overview"
[supported-aws-services]: https://www.rusoto.org/supported-aws-services.html "List of AWS services supported by Rusoto"
        
