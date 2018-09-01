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
use hyper::client::{ResponseFuture as HyperResponseFuture};
use hyper::client::connect::Connect;
use hyper::{Client as HyperClient, Body, Request as HyperRequest, Response as HyperResponse};
use hyper::Error as HyperError;
use hyper::header::{HeaderMap as HyperHeaders, HeaderValue as HyperHeaderValue, HeaderName as HyperHeaderName};
use hyper::body::Payload;
use hyper::StatusCode;
use hyper::Method;
use hyper::client::HttpConnector;
use tls::HttpsConnector;
use tokio_timer::Timeout;

use log::Level::Debug;

use signature::{SignedRequest, SignedRequestPayload};

// Pulls in the statically generated rustc version.
include!(concat!(env!("OUT_DIR"), "/user_agent_vars.rs"));

// Use a lazy static to cache the default User-Agent header
// because it never changes once it's been computed.
lazy_static! {
    static ref DEFAULT_USER_AGENT: String = format!("rusoto/{} rust/{} {}",
            env!("CARGO_PKG_VERSION"), RUST_VERSION, env::consts::OS);
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

    fn from_hyper(hyper_response: HyperResponse<Body>) -> HttpResponse {
        let status = hyper_response.status();
        let headers = Headers::new(hyper_response.headers().iter().map(|(h, v)| {
            let value_string = v.to_str().unwrap().to_owned();
            (h.as_str(), value_string)
        }));
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
        HttpDispatchError { message: err.to_string() }
    }
}

impl From<IoError> for HttpDispatchError {
    fn from(err: IoError) -> HttpDispatchError {
        HttpDispatchError { message: err.to_string() }
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
    HyperWithTimeout(Timeout<HyperResponseFuture>),
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
            ClientFutureInner::HyperWithTimeout(ref mut deadline_future) => {
                match deadline_future.poll() {
                    Err(deadline_err) => {
                        if deadline_err.is_elapsed() {
                            Err(HttpDispatchError { message: "Request timed out".into() })
                        } else if deadline_err.is_inner() {
                            Err(deadline_err.into_inner().unwrap().into())
                        } else {
                            Err(HttpDispatchError { message: format!("deadline error: {}", deadline_err) })
                        }
                    },
                    Ok(Async::NotReady) =>
                        Ok(Async::NotReady),
                    Ok(Async::Ready(hyper_res)) =>
                        Ok(Async::Ready(HttpResponse::from_hyper(hyper_res))),
                }
            }
        }
    }
}

struct HttpClientPayload {
    inner: Option<SignedRequestPayload>
}

impl Payload for HttpClientPayload {
    type Data = io::Cursor<Vec<u8>>;
    type Error = io::Error;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        match self.inner {
            None => Ok(Async::Ready(None)),
            Some(SignedRequestPayload::Buffer(ref mut buffer)) => {
                if buffer.len() == 0 {
                    Ok(Async::Ready(None))
                } else {
                    Ok(Async::Ready(Some(io::Cursor::new(buffer.split_off(0)))))
                }
            },
            Some(SignedRequestPayload::Stream(_, ref mut stream)) => {
                match stream.poll()? {
                    Async::NotReady => Ok(Async::NotReady),
                    Async::Ready(None) => Ok(Async::Ready(None)),
                    Async::Ready(Some(buffer)) => Ok(Async::Ready(Some(io::Cursor::new(buffer))))
                }
            }
        }
    }

    fn is_end_stream(&self) -> bool {
        match self.inner {
            None => true,
            Some(SignedRequestPayload::Buffer(ref buffer)) => buffer.len() == 0,
            Some(SignedRequestPayload::Stream(_, _)) => false
        }
    }

    fn content_length(&self) -> Option<u64> {
        match self.inner {
            None => Some(0),
            Some(SignedRequestPayload::Buffer(ref buffer)) =>
                Some(buffer.len() as u64),
            Some(SignedRequestPayload::Stream(hint, _)) =>
                hint.map(|s| s as u64)
        }
    }
}

/// Http client for use with AWS services.
pub struct HttpClient<C = HttpsConnector<HttpConnector>> {
    inner: HyperClient<C, HttpClientPayload>
}

impl HttpClient {
    /// Create a tls-enabled http client.
    pub fn new() -> Result<Self, TlsError> {
        #[cfg(feature="native-tls")]
        let connector = match HttpsConnector::new(4) {
            Ok(connector) => connector,
            Err(tls_error) => {
                return Err(TlsError {
                    message: format!("Couldn't create NativeTlsClient: {}", tls_error),
                })
            }
        };

        #[cfg(feature="rustls")]
        let connector = HttpsConnector::new(4);

        Ok(Self::from_connector(connector))
    }
}

impl<C> HttpClient<C>
where
    C: Connect,
    C::Future: 'static
{
    /// Allows for a custom connector to be used with the HttpClient
    pub fn from_connector(connector: C) -> Self {
        let inner = HyperClient::builder()
            .build(connector);

        HttpClient {
            inner
        }
    }
}

impl<C> DispatchSignedRequest for HttpClient<C>
    where C: Connect + 'static,
          C::Future: 'static
{
    type Future = HttpClientFuture;

    fn dispatch(&self, request: SignedRequest, timeout: Option<Duration>) -> Self::Future {
        let hyper_method = match request.method().as_ref() {
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            v => {
                return HttpClientFuture(ClientFutureInner::Error(format!("Unsupported HTTP verb {}", v)))
            }
        };

        // translate the headers map to a format Hyper likes
        let mut hyper_headers = HyperHeaders::new();
        for h in request.headers().iter() {
            let header_name = match h.0.parse::<HyperHeaderName>() {
                Ok(name) => name,
                Err(err) => {
                    return HttpClientFuture(ClientFutureInner::Error(format!("error parsing header name: {}", err)));
                }
            };
            for v in h.1.iter() {
                let header_value = match HyperHeaderValue::from_bytes(v) {
                    Ok(value) => value,
                    Err(err) => {
                        return HttpClientFuture(ClientFutureInner::Error(format!("error parsing header value: {}", err)));
                    }
                };
                hyper_headers.append(&header_name, header_value);
            }
        }

        // Add a default user-agent header if one is not already present.
        if !hyper_headers.contains_key("user-agent") {
            hyper_headers.insert("user-agent", DEFAULT_USER_AGENT.parse().unwrap());
        }

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
                   hyper_method,
                   final_uri,
                   payload);
            for (h, v) in hyper_headers.iter() {
                debug!("{}:{:?}", h.as_str(), v);
            }
        }

        let mut http_request_builder = HyperRequest::builder();
        http_request_builder.method(hyper_method);
        http_request_builder.uri(final_uri);

        let body = HttpClientPayload { inner: request.payload };
        let mut http_request = match http_request_builder.body(body) {
            Ok(request) => request,
            Err(err) => {
                return HttpClientFuture(ClientFutureInner::Error(format!("error building request: {}", err)));
            }
        };

        *http_request.headers_mut() = hyper_headers;

        let inner = match timeout {
            None => ClientFutureInner::Hyper(self.inner.request(http_request)),
            Some(duration) => {
                let future = Timeout::new(self.inner.request(http_request), duration);
                ClientFutureInner::HyperWithTimeout(future)
            }
        };

        HttpClientFuture(inner)
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
    fn http_client_is_send_and_sync() {
        fn is_send_and_sync<T: Send + Sync>() {}

        is_send_and_sync::<HttpClient>();
    }

    #[test]
    fn http_client_future_is_send() {
        fn is_send<T: Send>() {}

        is_send::<HttpClientFuture>();
    }

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

    #[test]
    fn from_io_error_preserves_error_message() {
        let io_error = ::std::io::Error::new(::std::io::ErrorKind::Other, "my error message");
        let error = HttpDispatchError::from(io_error);
        assert_eq!(error.to_string(), "my error message")
    }
}
