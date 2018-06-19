//! Mock request dispatcher and credentials for unit testing services

extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate rusoto_core;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::time::Duration;

use rusoto_core::{DispatchSignedRequest, HttpResponse, HttpDispatchError, SignedRequest};
use rusoto_core::credential::{ProvideAwsCredentials, CredentialsError, AwsCredentials};
use rusoto_core::request::Headers;
use futures::future::{FutureResult, ok};
use futures::stream::once;
use hyper::StatusCode;

pub struct MockCredentialsProvider;

impl ProvideAwsCredentials for MockCredentialsProvider {
    type Future = FutureResult<AwsCredentials, CredentialsError>;

    fn credentials(&self) -> Self::Future {
        ok(AwsCredentials::new("mock_key",
                               "mock_secret",
                               None,
                               None))
    }
}

pub struct MockRequestDispatcher {
    status: StatusCode,
    body: Vec<u8>,
    headers: HashMap<String, String>,
    request_checker: Option<Box<Fn(&SignedRequest)>>,
}

impl MockRequestDispatcher {
    pub fn with_status(status: u16) -> MockRequestDispatcher {
        MockRequestDispatcher {
            status: StatusCode::from_u16(status).unwrap(),
            body: b"".to_vec(),
            headers: HashMap::new(),
            request_checker: None,
        }
    }

    pub fn with_body(mut self, body: &str) -> MockRequestDispatcher {
        self.body = body.as_bytes().to_vec();
        self
    }

    pub fn with_request_checker<F>(mut self, checker: F) -> MockRequestDispatcher
        where F: Fn(&SignedRequest) + 'static {
        self.request_checker = Some(Box::new(checker));
        self
    }

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

pub trait ReadMockResponse {
    fn read_response(dir_name: &str, file_name: &str) -> String;
}

pub struct MockResponseReader;

impl ReadMockResponse for MockResponseReader {
    fn read_response(dir_name: &str, response_name: &str) -> String {
        let file_name = format!("{}/{}", dir_name, response_name);

        let mut input_file = File::open(&file_name).expect("couldn't find file");

        let mut mock_response = String::new();

        input_file.read_to_string(&mut mock_response).expect(&format!(
	        "Failed to read {:?}",
	        file_name,
	    ));

        mock_response
    }
}
