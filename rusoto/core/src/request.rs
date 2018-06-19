//! AWS API requests.
//!
//! Wraps the `hyper` library to send PUT, POST, DELETE and GET requests.

//extern crate lazy_static;

use std::env;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use std::io;
use std::collections::hash_map::{self, HashMap};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use std::mem;

use futures::{Async, Future, Poll, Stream};
use futures::future::{Either, Select2};
use hyper::client::connect::Connect;
use hyper::{Client as HyperClient, Request as HyperRequest, Response as HyperResponse, Body as HyperBody};
use hyper::client::ResponseFuture as HyperResponseFuture;
use hyper::Error as HyperError;
use hyper::StatusCode;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::{Handle, Timeout};

use log::Level::Debug;

use signature::{SignedRequest, SignedRequestPayload};

// Pulls in the statically generated rustc version.
include!(concat!(env!("OUT_DIR"), "/user_agent_vars.rs"));

// Use a lazy static to cache the default User-Agent header
// because it never changes once it's been computed.
lazy_static! {
    static ref DEFAULT_USER_AGENT: Vec<Vec<u8>> = vec![format!("rusoto/{} rust/{} {}",
            env!("CARGO_PKG_VERSION"), RUST_VERSION, env::consts::OS).as_bytes().to_vec()];
}

/// HTTP headers
#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    /// Create Headers from iterator
    pub fn new<'a, T>(headers: T) -> Self
        where T: IntoIterator<Item = (&'a str, String)>
    {
        Headers (
            headers.into_iter().map(|(k, v)| {
                (k.to_ascii_lowercase(), v)
            }).collect()
        )
    }

    /// Get value for HTTP header
    pub fn get(&self, name: &str) -> Option<&str> {
        self.0.get(&name.to_lowercase()).map(|n| n.as_str())
    }

    /// Create iterator over HTTP headers
    ///
    /// Header names are normalized to lowercase.
    pub fn iter(&self) -> HeaderIter {
        HeaderIter(self.0.iter())
    }
}

/// Iterator returned by `Headers::iter`
pub struct HeaderIter<'a>(hash_map::Iter<'a, String, String>);

impl<'a> Iterator for HeaderIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

/// Stores the response from a HTTP request.
pub struct HttpResponse {
    /// Status code of HTTP Request
    pub status: StatusCode,
    /// Contents of Response
    pub body: Box<Stream<Item=Vec<u8>, Error=io::Error> + Send>,
    /// Response headers
    pub headers: Headers,
}

/// Stores the buffered response from a HTTP request.
pub struct BufferedHttpResponse {
    /// Status code of HTTP Request
    pub status: StatusCode,
    /// Contents of Response
    pub body: Vec<u8>,
    /// Response headers
    pub headers: Headers
}

/// Future returned from `HttpResponse::buffer`.
pub struct BufferedHttpResponseFuture {
    status: StatusCode,
    headers: HashMap<String, String>,
    future: ::futures::stream::Concat2<Box<Stream<Item=Vec<u8>, Error=io::Error> + Send>>
}

impl Future for BufferedHttpResponseFuture {
    type Item = BufferedHttpResponse;
    type Error = HttpDispatchError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll().map_err(|err| err.into()).map(|async| async.map(|body| {
            BufferedHttpResponse {
                status: self.status,
                headers: Headers(mem::replace(&mut self.headers, HashMap::new())),
                body: body
            }
        }))
    }
}

impl HttpResponse {
    /// Buffer the full response body in memory, resulting in a `BufferedHttpResponse`. 
    pub fn buffer(self) -> BufferedHttpResponseFuture {
        BufferedHttpResponseFuture {
            status: self.status,
            headers: self.headers.0,
            future: self.body.concat2()
        }
    }

    fn from_hyper(hyper_response: HyperResponse<HyperBody>) -> HttpResponse {
        let status = hyper_response.status();
        let headers = Headers::new(hyper_response.headers().iter().map(|h| (
                    h.0.as_str(), String::from_utf8_lossy(h.1.as_bytes()).into_owned())));
        let body = hyper_response.into_body()
            .map(|chunk| chunk.as_ref().to_vec())
            .map_err(|err| {
                io::Error::new(io::ErrorKind::Other, err)
            });

        HttpResponse {
            status: status,
            headers: headers,
            body: Box::new(body),
        }
    }
}


#[derive(Debug, PartialEq)]
/// An error produced when invalid request types are sent.
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

/// Trait for implementing HTTP Request/Response
pub trait DispatchSignedRequest {
    /// The future response value.
    type Future: Future<Item=HttpResponse, Error=HttpDispatchError> + 'static;
    /// Dispatch Request, and then return a Response
    fn dispatch(&self, request: SignedRequest, timeout: Option<Duration>) -> Self::Future;
}

impl<D: DispatchSignedRequest> DispatchSignedRequest for Rc<D> {
    type Future = D::Future;
    fn dispatch(&self, request: SignedRequest, timeout: Option<Duration>) -> Self::Future {
        D::dispatch(&*self, request, timeout)
    }
}

impl<D: DispatchSignedRequest> DispatchSignedRequest for Arc<D> {
    type Future = D::Future;
    fn dispatch(&self, request: SignedRequest, timeout: Option<Duration>) -> Self::Future {
        D::dispatch(&*self, request, timeout)
    }
}

/// A future that will resolve to an `HttpResponse`.
pub struct HttpClientFuture(ClientFutureInner);

enum ClientFutureInner {
    Hyper(HyperResponseFuture),
    HyperWithTimeout(Select2<HyperResponseFuture, Timeout>),
    Error(String)
}

impl Future for HttpClientFuture {
    type Item = HttpResponse;
    type Error = HttpDispatchError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0 {
            ClientFutureInner::Error(ref message) =>
                Err(HttpDispatchError { message: message.clone() }),
            ClientFutureInner::Hyper(ref mut hyper_future) =>
                Ok(hyper_future.poll()?.map(HttpResponse::from_hyper)),
            ClientFutureInner::HyperWithTimeout(ref mut select_future) => {
                match select_future.poll() {
                    Err(Either::A((hyper_err, _))) =>
                        Err(hyper_err.into()),
                    Err(Either::B((io_err, _))) =>
                        Err(io_err.into()),
                    Ok(Async::NotReady) =>
                        Ok(Async::NotReady),
                    Ok(Async::Ready(Either::A((hyper_res, _)))) =>
                        Ok(Async::Ready(HttpResponse::from_hyper(hyper_res))),
                    Ok(Async::Ready(Either::B(((), _)))) =>
                        Err(HttpDispatchError { message: "Request timed out".into() })
                }
            }
        }
    }
}

struct HttpClientPayload {
    inner: SignedRequestPayload
}

impl Stream for HttpClientPayload {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.inner {
            SignedRequestPayload::Buffer(ref mut buffer) => {
                if buffer.len() == 0 {
                    Ok(Async::Ready(None))
                } else {
                    Ok(Async::Ready(Some(buffer.split_off(0))))
                }
            },
            SignedRequestPayload::Stream(_, ref mut stream) =>
                Ok(stream.poll()?)
        }
    }
}

/// Http client for use with AWS services.
pub struct HttpClient<C = HttpsConnector<HttpConnector>> {
    inner: HyperClient<C>,
    handle: Handle,
}

impl HttpClient {
    /// Create a tls-enabled http client.
    pub fn new(handle: &Handle) -> Result<Self, TlsError> {
        let connector = match HttpsConnector::new(4) {
            Ok(connector) => connector,
            Err(tls_error) => {
                return Err(TlsError {
                    message: format!("Couldn't create NativeTlsClient: {}", tls_error),
                })
            }
        };

        Ok(Self::from_connector(connector, handle))
    }
}

impl<C> HttpClient<C>
where
    C: Connect + 'static,
{
    /// Allows for a custom connector to be used with the HttpClient
    pub fn from_connector(connector: C, handle: &Handle) -> Self {
        let inner = HyperClient::builder()
            .build(connector);

        HttpClient {
            inner,
            handle: handle.clone(),
        }
    }

}

impl<C: Connect + 'static> DispatchSignedRequest for HttpClient<C> {
    type Future = HttpClientFuture;

    fn dispatch(&self, request: SignedRequest, timeout: Option<Duration>) -> Self::Future {


        let mut final_uri = format!("{}://{}{}", request.scheme(), request.hostname(), request.canonical_path());
        if !request.canonical_query_string().is_empty() {
            final_uri = final_uri + &format!("?{}", request.canonical_query_string());
        }

        if log_enabled!(Debug) {
            let payload = match request.payload {
                Some(SignedRequestPayload::Buffer(ref payload_bytes)) => {
                    String::from_utf8(payload_bytes.to_owned())
                        .unwrap_or_else(|_| String::from("<non-UTF-8 data>"))
                },
                Some(SignedRequestPayload::Stream(len, _)) =>
                    format!("<stream len_hint={:?}>", len),
                None => "".to_owned(),
            };

            debug!("Full request: \n method: {}\n final_uri: {}\n payload: {}\nHeaders:\n",
                   request.method(),
                   final_uri,
                   payload);
            for h in request.headers().iter() {
                debug!("{}:{:?}", h.0, h.1);
            }
        }

        let mut hyper_request = HyperRequest::builder();

        hyper_request.method(request.method());
        hyper_request.uri(final_uri);

        for h in request.headers().iter() {
            for vh in h.1 {
                hyper_request.header(h.0.as_ref() as &str, vh as &[u8]);
            }
        }

        let mut dispatch_body = move |b| {

            let hyper_request = match hyper_request.body(b) {
                Ok(v) => v,
                Err(e) => return HttpClientFuture(ClientFutureInner::Error(format!("{}", e))),
            };

            let inner = match timeout {
                None => ClientFutureInner::Hyper(self.inner.request(hyper_request)),
                Some(duration) => {
                    match Timeout::new(duration, &self.handle) {
                        Err(err) => ClientFutureInner::Error(format!("Error creating timeout future {}", err)),
                        Ok(timeout_future) => {
                            let future = self.inner.request(hyper_request).select2(timeout_future);
                            ClientFutureInner::HyperWithTimeout(future)
                        }
                    }
                }
            };

            HttpClientFuture(inner)
        };

        // workaround for https://github.com/hyperium/hyper/issues/1373
        match request.payload {
            Some(body) => dispatch_body(HyperBody::wrap_stream(HttpClientPayload{inner:  body})),
            None       => dispatch_body(HyperBody::empty()),
        }
    }

}

#[derive(Debug, PartialEq)]
/// An error produced when the user has an invalid TLS client
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

#[cfg(test)]
mod tests {
    use super::*;
    use Region;
    use signature::SignedRequest;

    #[test]
    fn custom_region_http() {
        let a_region = Region::Custom {
            endpoint: "http://localhost".to_owned(),
            name: "eu-west-3".to_owned(),
        };
        let request = SignedRequest::new("POST", "sqs", &a_region, "/");
        assert_eq!("http", request.scheme());
        assert_eq!("localhost", request.hostname());
    }

    #[test]
    fn custom_region_https() {
        let a_region = Region::Custom {
            endpoint: "https://localhost".to_owned(),
            name: "eu-west-3".to_owned(),
        };
        let request = SignedRequest::new("POST", "sqs", &a_region, "/");
        assert_eq!("https", request.scheme());
        assert_eq!("localhost", request.hostname());
    }

    #[test]
    fn custom_region_with_port() {
        let a_region = Region::Custom {
            endpoint: "https://localhost:8000".to_owned(),
            name: "eu-west-3".to_owned(),
        };
        let request = SignedRequest::new("POST", "sqs", &a_region, "/");
        assert_eq!("https", request.scheme());
        assert_eq!("localhost:8000", request.hostname());
    }

    #[test]
    fn custom_region_no_scheme() {
        let a_region = Region::Custom {
            endpoint: "localhost".to_owned(),
            name: "eu-west-3".to_owned(),
        };
        let request = SignedRequest::new("POST", "sqs", &a_region, "/");
        assert_eq!("https", request.scheme());
        assert_eq!("localhost", request.hostname());
    }

    #[test]
    fn headers() {
        let input = &[
            ("amazon-style-header-name", "SomeRandomValue"),
            ("Minio-Style-Header-Name", "AnotherValue"),
            ("RaNDOm-styLe-HeAdEr-NAme", "yet again another value")
        ];
        let headers = Headers::new(input.iter().map(|&(k, v)| (k, v.to_string())));

        assert_eq!(headers.get("Amazon-Style-Header-Name").unwrap(), "SomeRandomValue");
        assert_eq!(headers.get("Minio-Style-Header-Name").unwrap(), "AnotherValue");
        assert_eq!(headers.get("random-style-header-name").unwrap(), "yet again another value");
        assert!(headers.get("No-Such-Header").is_none());

        let mut output: Vec<_> = headers.iter().collect();
        output.sort();
        assert_eq!(
            output,
            &[("amazon-style-header-name", "SomeRandomValue"),
              ("minio-style-header-name", "AnotherValue"),
              ("random-style-header-name", "yet again another value")]
        );
    }
}
