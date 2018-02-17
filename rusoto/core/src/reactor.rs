//! Default implementations for `DispatchSignedRequest` and `ProvideAwsCredentials` backed by
//! an implicit event loop.
//!
//! The purpose of this module is to enable the use of Rusoto without requiring any knowledge about
//! `tokio`.
//!
//! It does this by providing two simple structs, `CredentialsProvider` and `RequestDispatcher`,
//! which can be plugged into any AWS client exposed by Rusoto.
//!
//! These implementations are backed by an implicit event loop running on a background thread, so no
//! manual event loop setup is required.
//!
//! The implicit event loop is crated lazily on first use of these implementations, meaning that advanced
//! users who wish to interface more directly with tokio can do so by using the otherwise provided
//! implementations without causing any extra background threads or event loops to be spawned.

use std::io::{Result as IoResult};
use std::rc::Rc;
use std::time::Duration;
use std::thread;

use futures::{Async, Future, Poll, Stream};
use futures::future::{Either, ok};
use futures::sync::{mpsc, oneshot};
use tokio_core::reactor::{Core, Handle, Remote};

use super::{
    SignedRequest, HttpResponse, HttpDispatchError, DispatchSignedRequest,
    AwsCredentials, CredentialsError, ProvideAwsCredentials,
    HttpClient, TlsError, DefaultCredentialsProvider
};

lazy_static! {
    static ref DEFAULT_REACTOR: Reactor = {
        Reactor::spawn().expect("failed to spawn default reactor")
    };
}

struct Reactor {
    remote: Remote
}

impl Reactor {
    fn spawn() -> IoResult<Reactor> {
        let (init_tx, init_rx) = oneshot::channel();

        thread::spawn(move || {
            let mut core = match Core::new() {
                Ok(core) => {
                    if let Err(_) = init_tx.send(Ok(core.remote())) {
                        panic!("failed to send init back to caller");
                    }
                    core
                },
                Err(err) => {
                    if let Err(_) = init_tx.send(Err(err)) {
                        panic!("failed to send init back to caller");
                    }
                    return;
                }
            };

            loop {
                core.turn(None);
            }
        });

        let remote = init_rx.wait()
            .expect("failed to initiate reactor")?;
        Ok(Reactor { remote: remote })
    }
}

/// A request dispatcher backed by an implicit event loop.
pub struct RequestDispatcher {
    sender: mpsc::UnboundedSender<((SignedRequest, Option<Duration>), oneshot::Sender<Result<HttpResponse, HttpDispatchError>>)>
}

impl Default for RequestDispatcher {
    fn default() -> RequestDispatcher {
        DEFAULT_REACTOR.default_request_dispatcher()
            .expect("failed to create default request dispatcher")
    }
}

/// Future returned from `RequestDispatcher`.
pub struct RequestDispatcherFuture {
    receiver: oneshot::Receiver<Result<HttpResponse, HttpDispatchError>>
}

impl Future for RequestDispatcherFuture {
    type Item = HttpResponse;
    type Error = HttpDispatchError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.receiver.poll().expect("failed to retrieve response from reactor") {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(result) => result.map(Async::Ready)
        }
    }
}

impl DispatchSignedRequest for RequestDispatcher {
    type Future = RequestDispatcherFuture;

    fn dispatch(&self, signed_request: SignedRequest, timeout: Option<Duration>) -> Self::Future {
        let (tx, rx) = oneshot::channel();
        if let Some(err) = self.sender.unbounded_send(((signed_request, timeout), tx)).err() {
            panic!("failed to send request to reactor: {}", err);
        }
        RequestDispatcherFuture {
            receiver: rx
        }
    }
}

/// A credentials provider backed by an implicit event loop.
pub struct CredentialsProvider {
    sender: mpsc::UnboundedSender<((), oneshot::Sender<Result<AwsCredentials, CredentialsError>>)>
}

impl Default for CredentialsProvider {
    fn default() -> CredentialsProvider {
        DEFAULT_REACTOR.default_credentials_provider()
            .expect("failed to create default credentials provider")
    }
}

/// Future returned from `CredentialsProvider`.
pub struct CredentialsProviderFuture {
    receiver: oneshot::Receiver<Result<AwsCredentials, CredentialsError>>
}

impl Future for CredentialsProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.receiver.poll().expect("failed to retrieve response from reactor") {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(result) => result.map(Async::Ready)
        }
    }
}

impl ProvideAwsCredentials for CredentialsProvider {
    type Future = CredentialsProviderFuture;

    fn credentials(&self) -> Self::Future {
        let (tx, rx) = oneshot::channel();
        if let Some(err) = self.sender.unbounded_send(((), tx)).err() {
            panic!("failed to send request to reactor: {}", err);
        }
        CredentialsProviderFuture {
            receiver: rx
        }
    }
}

impl Reactor {
    fn default_request_dispatcher(&self) -> Result<RequestDispatcher, TlsError> {
        self.new_request_dispatcher(|handle| HttpClient::new(&handle))
    }

    fn default_credentials_provider(&self) -> Result<CredentialsProvider, CredentialsError> {
        self.new_credentials_provider(|handle| DefaultCredentialsProvider::new(&handle))
    }

    fn new_request_dispatcher<D: DispatchSignedRequest + 'static, E: Send + 'static, F: FnOnce(Handle) -> Result<D, E> + Send + 'static>(&self, make_dispatcher: F) -> Result<RequestDispatcher, E> {
        self.new_responder(|handle| {
            make_dispatcher(handle).map(|dispatcher| move |(request, timeout)| { dispatcher.dispatch(request, timeout) })
        }).map(|sender| RequestDispatcher { sender: sender })
    }

    fn new_credentials_provider<P: ProvideAwsCredentials + 'static, E: Send + 'static, F: FnOnce(Handle) -> Result<P, E> + Send + 'static>(&self, make_provider: F) -> Result<CredentialsProvider, E> {
        self.new_responder(|handle| {
            make_provider(handle).map(|provider| move |()| { provider.credentials() })
        }).map(|sender| CredentialsProvider { sender: sender })
    }

    // This is the guts of the reactor mechanism. It takes a `make_responder` (`F`) function which
    // will be passed the `Handle` to the background event loop, and is supposed to return a "responder".
    //
    // A "responder" (`G`) is just another function which can be called multiple times with a request (`T`),
    // and then responds with a future (`U`). The item and error types of that future are required to be `Send`,
    // so that they can be moved across thread boundaries.
    //
    // The `new_responder` function then creates a channel, and spawns a new execution on the background event loop
    // which reads requests from the channel, and calls the responder function with the request. It will then drive
    // the future to completion, and when ready, send the result back to the caller.
    fn new_responder<T, U, E, F, G>(&self, make_responder: F) -> Result<mpsc::UnboundedSender<(T, oneshot::Sender<Result<U::Item, U::Error>>)>, E>
        where F: FnOnce(Handle) -> Result<G, E> + Send + 'static,
              G: Fn(T) -> U + 'static,
              E: Send + 'static,
              T: Send + 'static,
              U: Future + 'static,
              U::Item: Send + 'static,
              U::Error: Send + 'static
    {
        let (init_tx, init_rx) = oneshot::channel();

        self.remote.spawn(move |handle_ref| {
            let (tx, rx) = mpsc::unbounded::<(T, oneshot::Sender<Result<U::Item, U::Error>>)>();

            let responder = match make_responder(handle_ref.clone()) {
                Ok(responder) => {
                    if let Err(_) = init_tx.send(Ok(tx)) {
                        panic!("failed to send back reactor");
                    }
                    Rc::new(responder)
                },
                Err(err) => {
                    if let Err(_) = init_tx.send(Err(err)) {
                        panic!("failed to send back reactor");
                    }
                    return Either::A(ok(()));
                }
            };

            let handle = handle_ref.clone();
            Either::B(rx.for_each(move |(request, response_sender)| {
                let responder = responder.clone();
                handle.spawn_fn(move || {
                    ((responder)(request)).then(move |result| {
                        response_sender.send(result).ok();
                        Ok(())
                    })
                });
                Ok(())
            }))
        });

        init_rx.wait().expect("failed to initiate reactor") 
    }

}
