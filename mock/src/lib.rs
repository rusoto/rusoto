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
//! 
//! # Note regarding XML-based AWS Services
//! 
//! Some AWS services such as SQS and STS return responses in XML format rather than JSON.
//! 
//! In these cases, parsing may fail with `ParseError("Expected StartElement got None")`.
//! 
//! Valid XML examples can be found in the API documentation for an API call such as
//! [this one for STS AssumeRole](https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html#API_AssumeRole_Example_1_Response).
//! 
//! These can be used to create a `MockRequestDispatcher` like so:
//! 
//! ```rust
//! MockRequestDispatcher::default().with_body(r#"
//! <AssumeRoleResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
//!   ... etc
//! </AssumeRoleResponse>
//! "#)
//! ```
#![deny(missing_docs)]
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
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

/// Returns sequential mock API responses consuming a collection of MockRequestDispatch
pub struct MultipleMockRequestDispatcher<I>
where
    I: Iterator<Item = MockRequestDispatcher>,
{
    iterator: Mutex<I>,
}

impl<I> MultipleMockRequestDispatcher<I>
where
    I: Iterator<Item = MockRequestDispatcher>,
{
    /// Constructs a new MultipleMockRequestDispatcher with a collection of MockRequestDispatch as input
    pub fn new<C>(collection: C) -> MultipleMockRequestDispatcher<I>
    where
        C: IntoIterator<Item = MockRequestDispatcher>,
        C: IntoIterator<IntoIter = I>,
    {
        MultipleMockRequestDispatcher {
            iterator: Mutex::new(collection.into_iter()),
        }
    }
}

impl<I> DispatchSignedRequest for MultipleMockRequestDispatcher<I>
where
    I: Iterator<Item = MockRequestDispatcher>,
{
    fn dispatch(
        &self,
        request: SignedRequest,
        _timeout: Option<Duration>,
    ) -> rusoto_core::request::DispatchSignedRequestFuture {
        self.iterator
            .lock()
            .unwrap()
            .next()
            .expect("Ran out of mock responses to return from MultipleMockRequestDispatcher")
            .dispatch(request, _timeout)
    }
}
