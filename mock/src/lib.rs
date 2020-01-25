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
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use async_trait::async_trait;
use futures::FutureExt;
use http::{header::HeaderName, HeaderMap, StatusCode};
use rusoto_core::credential::{AwsCredentials, ProvideAwsCredentials};
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
use rusoto_core::{ByteStream, DispatchSignedRequest, HttpDispatchError};
use serde::Serialize;

/// Provides a set of credentials that always resolve
/// successfully
pub struct MockCredentialsProvider;

#[async_trait]
impl ProvideAwsCredentials for MockCredentialsProvider {
    async fn credentials(
        &self,
    ) -> Result<rusoto_core::credential::AwsCredentials, rusoto_core::credential::CredentialsError>
    {
        Ok(AwsCredentials::new("mock_key", "mock_secret", None, None))
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
    outcome: RequestOutcome,
    body: Vec<u8>,
    headers: HeaderMap<String>,
    request_checker: Option<Box<dyn Fn(&SignedRequest) + Send + Sync>>,
}

enum RequestOutcome {
    Performed(StatusCode),
    Failed(HttpDispatchError),
}

impl Default for RequestOutcome {
    fn default() -> RequestOutcome {
        RequestOutcome::Performed(StatusCode::default())
    }
}

impl MockRequestDispatcher {
    /// Returns a instance that mocks the status code that would
    /// be returned from AWS
    pub fn with_status(status: u16) -> MockRequestDispatcher {
        MockRequestDispatcher {
            outcome: RequestOutcome::Performed(StatusCode::from_u16(status).unwrap()),
            ..MockRequestDispatcher::default()
        }
    }

    /// Mocks the service request failing with a communications error
    pub fn with_dispatch_error(error: HttpDispatchError) -> MockRequestDispatcher {
        MockRequestDispatcher {
            outcome: RequestOutcome::Failed(error),
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
    where
        B: Serialize,
    {
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
        self.headers
            .insert(key.parse::<HeaderName>().unwrap(), value.into());
        self
    }
}

impl DispatchSignedRequest for MockRequestDispatcher {
    fn dispatch(
        &self,
        request: SignedRequest,
        _timeout: Option<Duration>,
    ) -> rusoto_core::request::DispatchSignedRequestFuture {
        if self.request_checker.is_some() {
            self.request_checker.as_ref().unwrap()(&request);
        }
        match self.outcome {
            RequestOutcome::Performed(ref status) => futures::future::ready(Ok(HttpResponse {
                status: *status,
                body: ByteStream::from(self.body.clone()),
                headers: self.headers.clone(),
            }))
            .boxed(),
            RequestOutcome::Failed(ref error) => futures::future::ready(Err(error.clone())).boxed(),
        }
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
            .unwrap_or_else(|_| panic!("Failed to read {:?}", file_name));

        mock_response
    }
}
