#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(
    feature = "nightly-testing",
    allow(
        cyclomatic_complexity,
        used_underscore_binding,
        ptr_arg,
        suspicious_else_formatting
    )
)]
#![allow(dead_code)]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]
#![deny(missing_docs)]

//! Rusoto is an [AWS](https://aws.amazon.com/) SDK for Rust.
//! A high level overview is available in `README.md` at <https://github.com/rusoto/rusoto>.

extern crate bytes;
extern crate futures;
extern crate hyper;
#[cfg(feature = "rustls")]
extern crate hyper_rustls as tls;
#[cfg(feature = "native-tls")]
extern crate hyper_tls as tls;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate base64;
pub extern crate rusoto_credential as credential;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate tokio;
extern crate tokio_timer;
extern crate xml;

mod client;
mod error;
mod future;
mod stream;

pub mod param;
#[doc(hidden)]
pub mod region;
pub mod request;
#[doc(hidden)]
pub mod signature;

#[doc(hidden)]
pub use crate::client::Client;
#[doc(hidden)]
pub mod encoding;
#[doc(hidden)]
pub mod proto;
#[doc(hidden)]
pub mod serialization;

pub use crate::credential::{CredentialsError, DefaultCredentialsProvider, ProvideAwsCredentials};
pub use crate::error::{AwsError, RusotoError, RusotoResult};
pub use crate::future::RusotoFuture;
pub use crate::region::Region;
pub use crate::request::{DispatchSignedRequest, HttpClient, HttpConfig, HttpDispatchError};
pub use crate::stream::ByteStream;
