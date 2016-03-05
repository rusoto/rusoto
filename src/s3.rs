//! S3 bindings for Rust
//!
//! Not all functions are yet implemented.  Check [S3Helper](http://dualspark.github.io/rusoto/rusoto/s3/struct.S3Helper.html)
//! for implemented functions and convenience functions.
//!

#![allow(unused_variables, unused_mut)]

include!(concat!(env!("OUT_DIR"), "/s3.rs"));
