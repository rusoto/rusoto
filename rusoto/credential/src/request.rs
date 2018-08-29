use std::io::{Error as IoError};
use std::io::ErrorKind::InvalidData;
use std::mem;
use std::time::Duration;

use futures::{Async, Future, Poll, Stream};
use futures::stream::Concat2;
use hyper::{Client as HyperClient, Body, Request, Uri};
use hyper::client::{ResponseFuture as HyperResponseFuture, HttpConnector};
use tokio_timer::Timeout;

use super::CredentialsError;

/// A future that will resolve to an `HttpResponse`.
pub struct HttpClientFuture(ClientFutureInner);

enum RequestFuture {
    Waiting(HyperResponseFuture),
    Buffering(Concat2<Body>),
    Swapping
}

impl Future for RequestFuture {
    type Item = String;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match mem::replace(self, RequestFuture::Swapping) {
            RequestFuture::Waiting(mut hyper_future) => {
                match hyper_future.poll()? {
                    Async::NotReady => {
                        *self = RequestFuture::Waiting(hyper_future);
                        Ok(Async::NotReady)
                    },
                    Async::Ready(res) => {
                        if !res.status().is_success() {
                            Err(CredentialsError {
                                message: format!("Invalid Response Code: {}", res.status())
                            })
                        } else {
                            *self = RequestFuture::Buffering(res.into_body().concat2());
                            self.poll()
                        }
                    }
                }
            },
            RequestFuture::Buffering(mut concat_future) => {
                match concat_future.poll()? {
                    Async::NotReady => {
                        *self = RequestFuture::Buffering(concat_future);
                        Ok(Async::NotReady)
                    },
                    Async::Ready(body) => {
                        let string_body = String::from_utf8(body.to_vec())
                            .map_err(|_| IoError::new(InvalidData, "Non UTF-8 Data returned"))?;
                        Ok(Async::Ready(string_body))
                    }
                }
            },
            RequestFuture::Swapping => unreachable!()
        }
    }
}

enum ClientFutureInner {
    Request(Timeout<RequestFuture>),
    Error(String)
}

impl Future for HttpClientFuture {
    type Item = String;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0 {
            ClientFutureInner::Error(ref message) =>
                Err(CredentialsError { message: message.clone() }),
            ClientFutureInner::Request(ref mut deadline_future) => {
                match deadline_future.poll() {
                    Err(deadline_error) => {
                        Err(CredentialsError {
                            message: deadline_error.to_string()
                        })
                    }
                    Ok(Async::NotReady) =>
                        Ok(Async::NotReady),
                    Ok(Async::Ready(body)) =>
                        Ok(Async::Ready(body)),
                }
            }
        }
    }
}

/// Http client for use in a credentials provider.
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: HyperClient<HttpConnector>
}

impl HttpClient {
    /// Create an http client.
    pub fn new() -> HttpClient {
        HttpClient {
            inner: HyperClient::new(),
        }
    }

    pub fn get(&self, uri: Uri, timeout: Duration) -> HttpClientFuture {
        match Request::get(uri).body(Body::empty()) {
            Ok(request) => self.request(request, timeout),
            Err(err) => HttpClientFuture(ClientFutureInner::Error(err.to_string())),
        }
    }

    pub fn request(&self, req: Request<Body>, timeout: Duration) -> HttpClientFuture {
        let future = RequestFuture::Waiting(self.inner.request(req));
        let inner = ClientFutureInner::Request(Timeout::new(future, timeout));
        HttpClientFuture(inner)
    }
}
