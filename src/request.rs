//! AWS API requests.
//!
//! Wraps the Hyper library to send PUT, POST, DELETE and GET requests.

extern crate lazy_static;

use std::env;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;

use hyper::{Client, Uri, Body};
use hyper::Error as HyperError;
use hyper::client::Request;
use hyper::header::UserAgent;
use hyper::status::StatusCode;
use hyper::Method;
use hyper_tls::HttpsConnector;

use futures::{Future, future, Stream};
use tokio_core::reactor::Handle;

use log::LogLevel::Debug;

use signature::SignedRequest;

// Pulls in the statically generated rustc version.
include!(concat!(env!("OUT_DIR"), "/user_agent_vars.rs"));

// Use a lazy static to cache the default User-Agent header
// because it never changes once it's been computed.
lazy_static! {
    static ref DEFAULT_USER_AGENT: Vec<Vec<u8>> = vec![format!("rusoto/{} rust/{} {}",
            env!("CARGO_PKG_VERSION"), RUST_VERSION, env::consts::OS).as_bytes().to_vec()];
}


#[derive(Clone)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub struct HttpDispatchError {
    message: String,
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

type AwsFuture = Box<Future<Item = HttpResponse, Error = HttpDispatchError>>;

pub trait DispatchSignedRequest {
    fn dispatch(&self, request: &SignedRequest) -> AwsFuture;
}

impl DispatchSignedRequest for Client<HttpsConnector> {
    fn dispatch(&self, request: &SignedRequest) -> AwsFuture {
        let hyper_method = match request.method().as_ref() {
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            v => return Box::new(future::err(HttpDispatchError { message: format!("Unsupported HTTP verb {}", v) })),
        };

        let mut final_uri = format!("https://{}{}", request.hostname(), request.canonical_path());
        if !request.canonical_query_string().is_empty() {
            final_uri = final_uri + &format!("?{}", request.canonical_query_string());
        }

        if log_enabled!(Debug) {
            let payload = match &request.payload {
                &Some(ref payload_bytes) => {
                    String::from_utf8(payload_bytes.to_owned())
                        .unwrap_or_else(|_| String::from("<non-UTF-8 data>"))
                }
                _ => "".to_owned(),
            };

            debug!("Full request: \n method: {}\n final_uri: {}\n payload: {}\nHeaders:\n",
                   hyper_method,
                   final_uri,
                   payload);

            for h in request.headers().iter() {
                debug!("{}:{:?}", h.0, h.1);
            }
        }

        let hyper_uri = match Uri::from_str(&final_uri) {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::err(HttpDispatchError { message: format!("Failed to parse URI {}", err) }))
        };

        let mut hyper_request = Request::new(hyper_method, hyper_uri);

        // translate the headers map to a format Hyper likes
        for h in request.headers().iter() {
            hyper_request
                .headers_mut()
                .set_raw(h.0.to_owned(), h.1.to_owned());
        }

        // Add a default user-agent header if one is not already present.
        if !hyper_request.headers().has::<UserAgent>() {
            hyper_request.headers_mut().set_raw("user-agent".to_owned(), DEFAULT_USER_AGENT.clone());
        }

        // TODO: remove this in favor of the logs above?
        // debug!("Full request: {:?}", hyper_request);

        // TODO: remove this clone()
        match request.payload {
            None => (),
            Some(ref payload_contents) => hyper_request.set_body(Body::from(payload_contents.clone()))
        };

        let hyper_response = self.request(hyper_request);

        let converted = hyper_response
            .and_then(|res| {
                let mut headers: HashMap<String, String> = HashMap::new();
                let status = res.status().to_owned();

                for header in res.headers().iter() {
                    headers.insert(header.name().to_string(), header.value_string());
                }

                res.body()
                    .fold(Vec::new(), |mut buffer, chunk| {
                        buffer.extend_from_slice(chunk.as_ref());
                        future::ok::<_, HyperError>(buffer)
                    })
                    .and_then(move |body| {
                        if log_enabled!(Debug) {
                            debug!("Response body:\n{:?}", body);
                        }

                        future::ok(HttpResponse {
                            status: status,
                            body: body,
                            headers: headers
                        })
                    })
            })
            .map_err(|hyper_err| HttpDispatchError { message: hyper_err.description().to_string() });

        Box::new(converted)
    }
}

#[derive(Debug, PartialEq)]
pub struct TlsError {
    message: String,
}

impl Error for TlsError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for TlsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Helper method for creating an http client with tls.
/// Makes a `hyper` client with NativeTls for HTTPS support.
pub fn default_tls_client(threads: usize, handle: &Handle) -> Result<Client<HttpsConnector>, TlsError> {
    // TODO: refactor not to return a Result and return a Client<HttpsConnector> instead?
    let connector = HttpsConnector::new(threads, handle);

    let client = Client::configure()
        .connector(connector)
        .build(handle);

    // FIXME: new client doesn't support redirect policy
    // client.set_redirect_policy(RedirectPolicy::FollowNone);

    Ok(client)
}
