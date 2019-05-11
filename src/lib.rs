#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity, used_underscore_binding, ptr_arg, suspicious_else_formatting))]
#![allow(dead_code)]

//! Rusoto is an [AWS](https://aws.amazon.com/) SDK for Rust.
//!
//! This crate for Rusoto is deprecated. It will receive no bugfixes or updates.
//!
//! Please migrate to Rusoto 0.25.0 or later.  See the [0.25.0 release notes](https://github.com/rusoto/rusoto/releases/tag/rusoto-v0.25.0) for more information including migration help.
