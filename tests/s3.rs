#![cfg(feature = "s3")]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;

#[macro_use]
extern crate rusoto;

use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::{S3Error, S3Helper};

#[test]
fn all_s3_tests() {
    let _ = env_logger::init();
    info!("s3 integration tests starting up.");
    let mut s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), Region::UsWest2);

    match s3_list_buckets_tests(&mut s3) {
        Ok(_) => { info!("Everything worked for S3 list buckets."); },
        Err(err) => { info!("Got error in s3 list buckets: {}", err); }
    }
}

fn s3_list_buckets_tests(s3: &mut S3Helper<DefaultCredentialsProvider>) -> Result<(), S3Error> {
    let response = try!(s3.list_buckets());
    info!("Got list of buckets: {:?}", response);
    for q in response.buckets {
        info!("Existing bucket: {:?}", q.name);
    }

    Ok(())
}
