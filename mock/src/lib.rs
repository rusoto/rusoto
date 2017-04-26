//! Mock request dispatcher and credentials for unit testing services

extern crate chrono;
extern crate hyper;
extern crate rusoto_core;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use rusoto_core::{DispatchSignedRequest, HttpResponse, HttpDispatchError, SignedRequest};
use rusoto_core::credential::{ProvideAwsCredentials, CredentialsError, AwsCredentials};
use chrono::{Duration, Utc};
use hyper::status::StatusCode;

const ONE_DAY: i64 = 86400;

pub struct MockCredentialsProvider;

impl ProvideAwsCredentials for MockCredentialsProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        Ok(AwsCredentials::new("mock_key",
                               "mock_secret",
                               None,
                               Utc::now() + Duration::seconds(ONE_DAY)))
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
            status: StatusCode::from_u16(status),
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
    fn dispatch(&self, request: &SignedRequest) -> Result<HttpResponse, HttpDispatchError> {
        if self.request_checker.is_some() {
            self.request_checker.as_ref().unwrap()(request);
        }
        Ok(HttpResponse {
            status: self.status,
            body: self.body.clone(),
            headers: self.headers.clone(),
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
