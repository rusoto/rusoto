#![cfg(feature = "xray")]

extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;

extern crate rusoto;

use rand::Rng;

use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};
use rusoto::xray::{XRayClient, PutTraceSegmentsRequest};

#[derive(Debug, Deserialize, Serialize)]
struct SegmentDocument {
    name: String,
    id: String,
    trace_id: String,
    start_time: f64,
    end_time: f64,
}

// See: http://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html
// Creates a segment two seconds in the past with a one second duration and a random ID.
fn segment_generator(name: String) -> String {
    let current_time = time::get_time();
    let mut rng = rand::thread_rng();
    let id = format!("{:8x}", rng.next_u64());
    let trace_id = format!("1-{:x}-{:016x}{:08x}", current_time.sec-2, rng.next_u64(), rng.next_u32());
    let segment = SegmentDocument {
        name: name,
        id: id,
        trace_id: trace_id,
        start_time: (current_time.sec-1) as f64,
        end_time: current_time.sec as f64,
    };
    serde_json::to_string(&segment).unwrap()
}

#[test]
fn put_trace_segments() {
    let service = "rusoto.xray_test".to_string();
    let segment = segment_generator(service);

    let client = XRayClient::new(default_tls_client().unwrap(), DefaultCredentialsProvider::new().unwrap(), Region::UsEast1);
    let request = PutTraceSegmentsRequest{trace_segment_documents: vec![segment]};
    let result = client.put_trace_segments(&request);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unprocessed_trace_segments.unwrap_or(Vec::new()).len(), 0);
}

