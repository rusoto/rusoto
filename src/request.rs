//! AWS API requests.
//!
//! Wraps the Hyper library to send PUT, POST, DELETE and GET requests.

use std::io::Read;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;

use hyper::Client;
use hyper::Error as HyperError;
use hyper::header::Headers;
use hyper::method::Method;

use log::LogLevel::Debug;

use signature::SignedRequest;

pub struct HttpResponse {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>
}

#[derive(Debug, PartialEq)]
pub struct HttpDispatchError {
    message: String
}

impl Error for HttpDispatchError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for HttpDispatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<HyperError> for HttpDispatchError {
    fn from(err: HyperError) -> HttpDispatchError {
        HttpDispatchError { message: err.description().to_string() }
    }
}

impl From<IoError> for HttpDispatchError {
    fn from(err: IoError) -> HttpDispatchError {
        HttpDispatchError { message: err.description().to_string() }
    }
}

pub trait DispatchSignedRequest {
    fn dispatch(&self, request: &SignedRequest) -> Result<HttpResponse, HttpDispatchError>;
}

impl DispatchSignedRequest for Client {
    fn dispatch(&self, request: &SignedRequest) -> Result<HttpResponse, HttpDispatchError> {
        let hyper_method = match request.method().as_ref() {
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "GET" => Method::Get,
            v @ _ => return Err(HttpDispatchError { message: format!("Unsupported HTTP verb {}", v) })

        };

        // translate the headers map to a format Hyper likes
        let mut hyper_headers = Headers::new();
        for h in request.headers().iter() {
            hyper_headers.set_raw(h.0.to_owned(), h.1.to_owned());
        }

        let mut final_uri = format!("https://{}{}", request.hostname(), request.path());
        if !request.canonical_query_string().is_empty() {
            final_uri = final_uri + &format!("?{}", request.canonical_query_string());
        }

        if log_enabled!(Debug) {
            let payload = request.payload().map(|mut payload_bytes| {
                let mut payload_string = String::new();

                payload_bytes.read_to_string(&mut payload_string)
                    .map(|_| payload_string)
                    .unwrap_or_else(|_| String::from("<non-UTF-8 data>"))
            });

            debug!("Full request: \n method: {}\n final_uri: {}\n payload: {:?}\nHeaders:\n", hyper_method, final_uri, payload);
            for h in hyper_headers.iter() {
                debug!("{}:{}", h.name(), h.value_string());
            }
        }

        let mut hyper_response = match request.payload() {
            None => try!(self.request(hyper_method, &final_uri).headers(hyper_headers).body("").send()),
            Some(payload_contents) => try!(self.request(hyper_method, &final_uri).headers(hyper_headers).body(payload_contents).send()),
        };

        let mut body = String::new();
        try!(hyper_response.read_to_string(&mut body));

        if log_enabled!(Debug) {
            debug!("Response body:\n{}", body);
        }

        let mut headers: HashMap<String, String> = HashMap::new();

        for header in hyper_response.headers.iter() {
            headers.insert(header.name().to_string(), header.value_string());
        }

        Ok(HttpResponse {
            status: hyper_response.status.to_u16(),
            body: body,
            headers: headers
        })
        
    }
}
