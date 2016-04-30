extern crate rusoto;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate time;

use rusoto::kinesis::*;
use rusoto::{ChainProvider, Region};
use time::get_time;
use std::time::Duration;
use std::thread::sleep;

fn setup() -> KinesisHelper<ChainProvider> {
	let _ = env_logger::init();
	KinesisHelper::new(ChainProvider::new().unwrap(), Region::UsEast1)
}

#[test]
fn should_do_basic_stream_crud() {
	
	let mut kinesis = setup();
	let stream_name = &format!("rusoto_{}", get_time().sec);

	// create a new stream
	let create_result = kinesis.create_stream(stream_name, 1);
	assert!(create_result.is_ok());
	info!("Created kinesis stream {}", stream_name);
	
	// wait for it to become active
	let five_seconds = Duration::new(5, 0);
	let mut active = false;	

	info!("Waiting for stream to become active");
	while !active {		
		sleep(five_seconds);
		let status = kinesis.describe_stream(stream_name).unwrap().stream_description.stream_status;
		info!("\tstream_status: {}", status);
		active = status == "ACTIVE";
	}

	info!("Done");
	info!("{:#?}", kinesis.describe_stream(stream_name).unwrap());

	let delete_result = kinesis.delete_stream(stream_name);
	assert!(delete_result.is_ok());
	info!("Deleted stream {}", stream_name);
}
