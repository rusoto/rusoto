//! Mock request dispatcher and credentials for unit testing services

extern crate chrono;
extern crate hyper;
extern crate rusoto_core;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use rusoto_core::{DispatchSignedRequest, HttpResponse, HttpDispatchError, SignedRequest};
use rusoto_core::credential::{ProvideAwsCredentials, CredentialsError, AwsCredentials};
use chrono::{Duration, UTC};
use hyper::status::StatusCode;

const ONE_DAY: i64 = 86400;

pub struct MockCredentialsProvider;

impl ProvideAwsCredentials for MockCredentialsProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        Ok(AwsCredentials::new("mock_key",
                               "mock_secret",
                               None,
                               UTC::now() + Duration::seconds(ONE_DAY)))
    }
}

pub struct MockRequestDispatcher {
    mock_response: HttpResponse,
    request_checker: Option<Box<Fn(&SignedRequest)>>,
}

impl MockRequestDispatcher {
    pub fn with_status(status: u16) -> MockRequestDispatcher {
        let response = HttpResponse {
            status: StatusCode::from_u16(status),
            body: b"".to_vec(),
            headers: HashMap::new(),
        };

        MockRequestDispatcher {
            mock_response: response,
            request_checker: None,
        }
    }

    pub fn with_body(mut self, body: &str) -> MockRequestDispatcher {
        self.mock_response.body = body.as_bytes().to_vec();
        self
    }

    pub fn with_request_checker<F>(mut self, checker: F) -> MockRequestDispatcher
        where F: Fn(&SignedRequest) + 'static {
        self.request_checker = Some(Box::new(checker));
        self
    }

    pub fn with_header<S1, S2>(mut self, key: S1, value: S2) -> MockRequestDispatcher
        where S1: Into<String>, S2: Into<String> {
        self.mock_response.headers.insert(key.into(), value.into());
        self
    }
}

impl DispatchSignedRequest for MockRequestDispatcher {
    fn dispatch(&self, request: &SignedRequest) -> Result<HttpResponse, HttpDispatchError> {
        if self.request_checker.is_some() {
            self.request_checker.as_ref().unwrap()(request);
        }
        Ok(self.mock_response.clone())
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