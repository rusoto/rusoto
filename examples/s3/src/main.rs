//!
//! This is a very basic example of using the s3 API of rusoto.
//!
//! There are several existing crates demonstrating behaviour such as
//! multipart uploads and range fetches.

use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use structopt::StructOpt;
use tokio::prelude::*;

#[derive(StructOpt)]
enum Command {
    ListBuckets,
    GetObject {
        #[structopt(long)]
        bucket: String,
        #[structopt(long)]
        key: String,
    },
}

#[derive(StructOpt)]
#[structopt(about = "a tiny subset of aws s3api")]
struct S3Api {
    #[structopt(long)]
    region: Region,

    #[structopt(subcommand)]
    command: Command,
}

type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Rough equivalent to aws s3api list-buckets
async fn aws_s3api_list_buckets(s3: &S3Client) -> Result<(), AnyError> {
    let response = s3.list_buckets().await?;
    serde_json::to_writer_pretty(std::io::stdout(), &response)?;
    Ok(())
}

/// Rough equivalent to aws s3api get-object
async fn aws_s3api_get_object(s3: &S3Client, bucket: String, key: String) -> Result<(), AnyError> {
    let request = GetObjectRequest {
        bucket,
        key,
        ..GetObjectRequest::default()
    };
    let response = s3.get_object(request).await?;

    // Get an AsyncRead object from the body.
    let mut body = response.body.unwrap().into_async_read();

    // Use AsyncReadExt to read to the end.
    // Note: This is suboptimal for large transfers.
    let mut buf = Vec::new();
    body.read_to_end(&mut buf).await?;

    println!("body={}", std::str::from_utf8(buf.as_ref()).unwrap());
    Ok(())
}

// Example command lines:
//
// cargo run -- --region eu-west-2 list-buckets
// cargo run -- --region eu-west-2 get-object --bucket my-bucket --key my-file

#[tokio::main]
async fn main() {
    let opt = S3Api::from_args();
    let s3 = S3Client::new(Region::from(opt.region));
    if let Err(err) = match opt.command {
        Command::ListBuckets => aws_s3api_list_buckets(&s3).await,
        Command::GetObject {
            bucket,
            key,
        } => aws_s3api_get_object(&s3, bucket, key).await,
    } {
        println!("error={}", err);
    }
}
