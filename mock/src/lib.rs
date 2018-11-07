//! Mock request dispatcher and credentials for unit testing rusoto AWS service clients
//!
//! All rusoto generated service clients come with a constuctor function named `new_with` which
//! accepts three arguments
//!
//! * A `rusoto_core::DispatchSignedRequest` implementation
//! * A `rusoto_core::credential::ProvideAwsCredentials` implementation
//! * A `rusoto_core::Region`
//!
//! This crate provides mock implementations to satisfy the first two.
//!
//! # Example
//!
//! The following is an example for the `rusoto_s3` crate but should
//! work for all service crates just the same. The code is commented
//! out for illustration but also to avoid a cyclic dependency in this crate.
//!
//! ```rust
//! extern crate rusoto_mock;
//! // extern crate rusoto_s3;
//!
//! use rusoto_mock::{MockCredentialsProvider, MockRequestDispatcher, MockResponseReader};
//!
//! fn main() {
//!    // let s3 = rusoto_s3::S3Client::new_with(
//!    //   MockRequestDispatcher::default().with_body(
//!    //      MockResponseReader::read_response("test-data", "s3-response.json")
//!    //   ),
//!    //   MockCredentialsProvider,
//!    //   Default::default()
//!    // );
//! }
//! ```
#![deny(missing_docs)]
extern crate chrono;
extern crate futures;
extern crate http;
extern crate rusoto_core;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use futures::future::{ok, FutureResult};
use futures::stream::once;
use http::{HttpTryFrom, StatusCode};
use rusoto_core::credential::{AwsCredentials, CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::{Headers, HttpResponse};
use rusoto_core::signature::SignedRequest;
use rusoto_core::{DispatchSignedRequest, HttpDispatchError};
use serde::Serialize

/// Provides a set of credentials that always resolve
/// successfully
pub struct MockCredentialsProvider;

impl ProvideAwsCredentials for MockCredentialsProvider {
    type Future = FutureResult<AwsCredentials, CredentialsError>;

    fn credentials(&self) -> Self::Future {
        ok(AwsCredentials::new("mock_key", "mock_secret", None, None))
    }
}

/// Composes mock API responses
///
/// A Default is provided which returns an successful response with an empty body
///
/// These can be constructed using either a Default implementation or
///  with the [with_status](method.with_status) function.
///
#[derive(Default)]
pub struct MockRequestDispatcher {
    status: StatusCode,
    body: Vec<u8>,
    headers: HashMap<String, String>,
    request_checker: Option<Box<Fn(&SignedRequest) + Send + Sync>>,
}

impl MockRequestDispatcher {
    /// Returns a instance that mocks the status code that would
    /// be returned from AWS
    pub fn with_status(status: u16) -> MockRequestDispatcher {
        MockRequestDispatcher {
            status: StatusCode::try_from(status).unwrap(),
            ..MockRequestDispatcher::default()
        }
    }

    /// Mocks the service response body what would be
    /// returned from AWS
    pub fn with_body(mut self, body: &str) -> MockRequestDispatcher {
        self.body = body.as_bytes().to_vec();
        self
    }

    /// Mocks the json serialized response body what would be
    /// returned from AWS
    pub fn with_json_body<B>(mut self, body: B) -> MockRequestDispatcher
        where B: Serialize {
        self.body = serde_json::to_vec(&body).expect("failed to deserialize into json");
        self
    }

    /// Mocks the signed request checking applied to a request before sending
    /// to AWS
    pub fn with_request_checker<F>(mut self, checker: F) -> MockRequestDispatcher
    where
        F: Fn(&SignedRequest) + Send + Sync + 'static,
    {
        self.request_checker = Some(Box::new(checker));
        self
    }

    /// Mocks a single service header that would be returned from AWS
    pub fn with_header(mut self, key: &str, value: &str) -> MockRequestDispatcher {
        self.headers.insert(key.into(), value.into());
        self
    }
}

impl DispatchSignedRequest for MockRequestDispatcher {
    type Future = FutureResult<HttpResponse, HttpDispatchError>;

    fn dispatch(&self, request: SignedRequest, _timeout: Option<Duration>) -> Self::Future {
        if self.request_checker.is_some() {
            self.request_checker.as_ref().unwrap()(&request);
        }
        ok(HttpResponse {
            status: self.status,
            body: Box::new(once(Ok(self.body.clone()))),
            headers: Headers::new(self.headers.iter().map(|(k, v)| (k.as_ref(), v.to_owned()))),
        })
    }
}

/// An interface for producing response body content
pub trait ReadMockResponse {
    /// Return a response body string for a given directory and file name
    fn read_response(dir_name: &str, file_name: &str) -> String;
}

/// Reads response body content from disk
pub struct MockResponseReader;

impl ReadMockResponse for MockResponseReader {
    fn read_response(dir_name: &str, response_name: &str) -> String {
        let file_name = format!("{}/{}", dir_name, response_name);

        let mut input_file = File::open(&file_name).expect("couldn't find file");

        let mut mock_response = String::new();

        input_file
            .read_to_string(&mut mock_response)
            .expect(&format!("Failed to read {:?}", file_name,));

        mock_response
    }
}
