#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity, used_underscore_binding, ptr_arg, suspicious_else_formatting))]
#![allow(dead_code)]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]

extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate ring;
extern crate rustc_serialize;
pub extern crate rusoto_credential as credential;
extern crate serde;
extern crate time;
extern crate url;
extern crate xml;

pub mod param;
pub mod region;
pub mod request;
pub mod signature;

#[doc(hidden)]
pub mod serialization;
#[doc(hidden)]
pub mod xmlerror;
#[doc(hidden)]
pub mod xmlutil;

pub use credential::{AwsCredentials, ChainProvider, ContainerProvider, CredentialsError,
                     EnvironmentProvider, InstanceMetadataProvider, ProfileProvider,
                     ProvideAwsCredentials, DefaultCredentialsProvider,
                     DefaultCredentialsProviderSync, claims,
                     AutoRefreshingProviderSync, AutoRefreshingProvider,
                     BaseAutoRefreshingProvider};

pub use region::{ParseRegionError, Region, default_region};
pub use request::{DispatchSignedRequest, HttpResponse, HttpDispatchError, TlsError, default_tls_client};
pub use signature::SignedRequest;
