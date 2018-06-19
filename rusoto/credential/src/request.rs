use std::io::{Error as IoError};
use std::io::ErrorKind::InvalidData;
use std::mem;
use std::time::Duration;

use futures::{Async, Future, Poll, Stream};
use futures::future::{Either, Select2};
use futures::stream::Concat2;
use hyper::{Client as HyperClient, Body, Request};
use hyper::client::{ResponseFuture as HyperFutureResponse, HttpConnector};
use tokio_core::reactor::{Handle, Timeout};

use super::CredentialsError;

/// A future that will resolve to an `HttpResponse`.
pub struct HttpClientFuture(ClientFutureInner);

enum RequestFuture {
    Waiting(HyperFutureResponse),
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
    Request(Select2<RequestFuture, Timeout>),
    Error(String)
}

impl Future for HttpClientFuture {
    type Item = String;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0 {
            ClientFutureInner::Error(ref message) =>
                Err(CredentialsError { message: message.clone() }),
            ClientFutureInner::Request(ref mut select_future) => {
                match select_future.poll() {
                    Err(Either::A((err, _))) =>
                        Err(err),
                    Err(Either::B((io_err, _))) =>
                        Err(io_err.into()),
                    Ok(Async::NotReady) =>
                        Ok(Async::NotReady),
                    Ok(Async::Ready(Either::A((body, _)))) =>
                        Ok(Async::Ready(body)),
                    Ok(Async::Ready(Either::B(((), _)))) =>
                        Err(CredentialsError { message: "Request timed out".into() })
                }
            }
        }
    }
}

/// Http client for use in a credentials provider.
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: HyperClient<HttpConnector>,
    handle: Handle
}

impl HttpClient {
    /// Create an http client.
    pub fn new(handle: &Handle) -> HttpClient {
        HttpClient {
            inner: HyperClient::new(),
            handle: handle.clone()
        }
    }

    pub fn request(&self, request: Request<Body>, timeout: Duration) -> HttpClientFuture {
        let request_future = RequestFuture::Waiting(self.inner.request(request));

        let inner = match Timeout::new(timeout, &self.handle) {
            Err(err) => ClientFutureInner::Error(format!("Error creating timeout future {}", err)),
            Ok(timeout_future) => {
                let future = request_future.select2(timeout_future);
                ClientFutureInner::Request(future)
            }
        };

        HttpClientFuture(inner)
    }
}
