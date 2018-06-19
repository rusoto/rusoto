use std::sync::Arc;
use std::time::Duration;

use futures::{Async, IntoFuture, Future, Poll};

use super::{CredentialsError, HttpResponse, HttpDispatchError, SignedRequest, ProvideAwsCredentials, DispatchSignedRequest};

/// Re-usable logic for all clients.
pub struct ClientInner<P, D> {
    credentials_provider: Arc<P>,
    dispatcher: Arc<D>
}

impl<P, D> ClientInner<P, D> {
    /// Create from a credentials provider and request dispatcher.
    pub fn new(credentials_provider: P, dispatcher: D) -> Self {
        ClientInner {
            credentials_provider: Arc::new(credentials_provider),
            dispatcher: Arc::new(dispatcher)
        }
    }
}

impl<P: ProvideAwsCredentials, D: DispatchSignedRequest> ClientInner<P, D> {
    /// Fetch credentials, sign the request and dispatch it.
    pub fn sign_and_dispatch<F: IntoFuture>(&self, request: SignedRequest, response_handler: fn(HttpResponse) -> F) -> SignAndDispatch<P, D, F> {
        SignAndDispatch {
            credentials_provider: self.credentials_provider.clone(),
            dispatcher: self.dispatcher.clone(),
            response_handler: response_handler,
            state: SignAndDispatchState::Lazy { request: request },
            timeout: None
        }
    }
}

pub trait ClientInnerFuture: Future {
    fn set_timeout(&mut self, timeout: Duration);
    fn clear_timeout(&mut self);
}

pub struct SignAndDispatch<P: ProvideAwsCredentials, D: DispatchSignedRequest, F: IntoFuture> {
    credentials_provider: Arc<P>,
    dispatcher: Arc<D>,
    response_handler: fn(HttpResponse) -> F,
    state: SignAndDispatchState<P, D, F>,
    timeout: Option<Duration>,
}

impl<P, D, F> ClientInnerFuture for SignAndDispatch<P, D, F>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest,
          F: IntoFuture,
          F::Error: From<HttpDispatchError> + From<CredentialsError>
{
    fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }

    fn clear_timeout(&mut self) {
        self.timeout = None;
    }
}

unsafe impl<P, D, F> Send for SignAndDispatch<P, D, F>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest,
          F: IntoFuture,
          F::Error: From<HttpDispatchError> + From<CredentialsError>
{}

enum SignAndDispatchState<P: ProvideAwsCredentials, D: DispatchSignedRequest, F: IntoFuture> {
    Lazy { request: SignedRequest },
    FetchingCredentials { future: P::Future, request: SignedRequest },
    Dispatching { future: D::Future },
    RunningResponseHandler { future: F::Future },
    Swapping
}

impl<P, D, F> Future for SignAndDispatch<P, D, F>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest,
          F: IntoFuture,
          F::Error: From<HttpDispatchError> + From<CredentialsError>
{
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match ::std::mem::replace(&mut self.state, SignAndDispatchState::Swapping) {
            SignAndDispatchState::Lazy { request } => {
                let future = self.credentials_provider.credentials();
                self.state = SignAndDispatchState::FetchingCredentials {
                    future: future,
                    request: request
                };
                self.poll()
            },
            SignAndDispatchState::FetchingCredentials { mut future, mut request } => {
                match future.poll()? {
                    Async::NotReady => {
                        self.state = SignAndDispatchState::FetchingCredentials {
                            future: future,
                            request: request
                        };
                        Ok(Async::NotReady)
                    },
                    Async::Ready(credentials) => {
                        request.sign_with_plus(&credentials, true);
                        let future = self.dispatcher.dispatch(request, self.timeout);
                        self.state = SignAndDispatchState::Dispatching {
                            future: future
                        };
                        self.poll()
                    }
                }
            },
            SignAndDispatchState::Dispatching { mut future } => {
                match future.poll()? {
                    Async::NotReady => {
                        self.state = SignAndDispatchState::Dispatching {
                            future: future
                        };
                        Ok(Async::NotReady)
                    },
                    Async::Ready(response) => {
                        let future = (self.response_handler)(response).into_future();
                        self.state = SignAndDispatchState::RunningResponseHandler{
                            future: future
                        };
                        self.poll()
                    }
                }
            },
            SignAndDispatchState::RunningResponseHandler { mut future } => {
                match future.poll()? {
                    Async::NotReady => {
                        self.state = SignAndDispatchState::RunningResponseHandler {
                            future: future
                        };
                        Ok(Async::NotReady)
                    },
                    Async::Ready(item) => Ok(Async::Ready(item))
                }
            },
            SignAndDispatchState::Swapping => unreachable!()
        }
    }
}
