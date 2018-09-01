#![doc(html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png")]
#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity, used_underscore_binding, ptr_arg, suspicious_else_formatting))]
#![allow(dead_code)]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]
#![deny(missing_docs)]

//! Rusoto is an [AWS](https://aws.amazon.com/) SDK for Rust.
//! A high level overview is available in `README.md` at <https://github.com/rusoto/rusoto>.

extern crate futures;
extern crate hyper;
#[cfg(feature = "native-tls")]
extern crate hyper_tls as tls;
#[cfg(feature = "rustls")]
extern crate hyper_rustls as tls;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate md5;
extern crate hex;
extern crate hmac;
extern crate base64;
pub extern crate rusoto_credential as credential;
extern crate serde;
extern crate sha2;
extern crate time;
extern crate tokio;
extern crate tokio_timer;
extern crate url;
extern crate xml;

mod future;
mod client;

pub mod param;
pub mod region;
pub mod request;
pub mod signature;

#[doc(hidden)]
pub use client::Client;
#[doc(hidden)]
pub mod serialization;
#[doc(hidden)]
pub mod xmlerror;
#[doc(hidden)]
pub mod xmlutil;

pub use credential::{ProvideAwsCredentials, DefaultCredentialsProvider, CredentialsError};
pub use request::{DispatchSignedRequest, HttpClient, HttpDispatchError};
pub use region::Region;
pub use future::RusotoFuture;
