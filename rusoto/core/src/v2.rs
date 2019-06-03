//! TODO

use futures::{IntoFuture, Future};

use crate::Client;
use crate::Region;
use crate::{RusotoResult, RusotoFuture};
use crate::error::RusotoError;
use crate::request::HttpResponse;
use crate::signature::SignedRequest;

/// TODO
pub trait ServiceRequest {
    /// TODO
    type Output;
    /// TODO
    type Error;

    /// TODO
    fn dispatch(self, region: &Region, dispatcher: &impl Dispatcher) -> RusotoFuture<Self::Output, Self::Error>;
}

/// TODO
pub struct Request<T> {
    input: T,
    region: Region,
    client: Client,
}

impl<T: ServiceRequest> Request<T> {
    /// TODO
    pub fn new(input: T, region: Region, client: Client) -> Self {
        Request { input, region, client }
    }

    /// TODO
    pub fn send(self) -> RusotoFuture<T::Output, T::Error> {
        self.input.dispatch(&self.region, &self.client)
    }

    /// TODO
    pub fn sync(self) -> RusotoResult<T::Output, T::Error>
    where
        T::Output: Send + 'static,
        T::Error: Send + 'static,
    {
        self.send().sync()
    }
}

impl<T: ServiceRequest> IntoFuture for Request<T> {
    type Future = RusotoFuture<T::Output, T::Error>;
    type Item = T::Output;
    type Error = RusotoError<T::Error>;

    fn into_future(self) -> Self::Future {
        self.send()
    }
}

/// TODO
pub trait Dispatcher {
/// TODO
    fn dispatch<T, E>(
        &self,
        request: SignedRequest,
        response_handler: fn(HttpResponse) -> Box<dyn Future<Item = T, Error = RusotoError<E>> + Send>,
    ) -> RusotoFuture<T, E>;
}

impl Dispatcher for Client {
    fn dispatch<T, E>(
        &self,
        request: SignedRequest,
        response_handler: fn(HttpResponse) -> Box<dyn Future<Item = T, Error = RusotoError<E>> + Send>,
    ) -> RusotoFuture<T, E> {
        self.sign_and_dispatch(request, response_handler)
    }
}
