#![cfg(feature = "timestream-query")]

use rusoto_core::Region;
use rusoto_timestream_query::{TimestreamQuery, TimestreamQueryClient};

#[tokio::test]
async fn should_describe_endpoints() {
	let client = TimestreamQueryClient::new(Region::UsEast1);

    client.describe_endpoints().await.unwrap();
}
