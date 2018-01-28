use std::sync::Arc;
use std::marker::PhantomData;

use futures::{Async, Future, Poll};

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
    pub fn sign_and_dispatch<E>(&self, request: SignedRequest) -> SignAndDispatch<P, D, E> {
        SignAndDispatch {
            credentials_provider: self.credentials_provider.clone(),
            dispatcher: self.dispatcher.clone(),
            state: SignAndDispatchState::Lazy { request: request },
            _phan: PhantomData
        }
    }
}

pub struct SignAndDispatch<P: ProvideAwsCredentials, D: DispatchSignedRequest, E> {
    credentials_provider: Arc<P>,
    dispatcher: Arc<D>,
    state: SignAndDispatchState<P, D>,
    _phan: PhantomData<E>
}

enum SignAndDispatchState<P: ProvideAwsCredentials, D: DispatchSignedRequest> {
    Lazy { request: SignedRequest },
    FetchingCredentials { future: P::Future, request: SignedRequest },
    Dispatching { future: D::Future },
    Swapping
}

impl<P, D, E> Future for SignAndDispatch<P, D, E>
    where P: ProvideAwsCredentials,
        D: DispatchSignedRequest,
        E: From<HttpDispatchError> + From<CredentialsError>
{
    type Item = HttpResponse;
    type Error = E;

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
                        let future = self.dispatcher.dispatch(request);
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
                    Async::Ready(response) =>
                        Ok(Async::Ready(response))
                }
            },
            SignAndDispatchState::Swapping => unreachable!()
        }
    }
}
