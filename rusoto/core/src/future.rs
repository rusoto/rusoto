use std::time::Duration;

use futures::{Future, Poll, Async};
use futures::sync::oneshot::spawn;
use tokio::runtime::Runtime;

use super::credential::CredentialsError;
use super::client::{TimeoutFuture, SignAndDispatchError};
use super::request::{HttpResponse, HttpDispatchError};

lazy_static! {
    static ref FALLBACK_RUNTIME: Runtime = Runtime::new().unwrap();
}

/// Future that is returned from all rusoto service APIs.
pub struct RusotoFuture<T, E> {
    state: Option<RusotoFutureState<T, E>>
}

pub fn new<T, E>(
        future: Box<TimeoutFuture<Item=HttpResponse, Error=SignAndDispatchError> + Send>,
        handler: fn(HttpResponse) -> Box<Future<Item=T, Error=E> + Send>
    ) -> RusotoFuture<T, E>
{
    RusotoFuture { state: Some(RusotoFutureState::SignAndDispatch { future, handler }) }
}

impl<T, E> RusotoFuture<T, E> {
    /// Set the timeout on the future to the provided duration.
    ///
    /// Unlike `set_timeout` this method can be easily chained:
    ///
    /// ```rust,ignore
    /// let future = s3.head_object(...)
    ///     .with_timeout(Duration::from_secs(10));
    /// ```
    ///
    /// This is only guaranteed to take effect when called before the future
    /// is polled for the first time.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.set_timeout(timeout);
        self
    }

    /// Set the timeout on the future to the provided duration.
    ///
    /// This is only guaranteed to take effect when called before the future
    /// is polled for the first time.
    pub fn set_timeout(&mut self, timeout: Duration) {
        if let Some(RusotoFutureState::SignAndDispatch { ref mut future, .. }) = self.state {
            future.set_timeout(timeout);
        }
    }

    /// Clear the timeout on the future.
    ///
    /// This is only guaranteed to take effect when called before the future
    /// is polled for the first time.
    pub fn clear_timeout(&mut self) {
        if let Some(RusotoFutureState::SignAndDispatch { ref mut future, .. }) = self.state {
            future.clear_timeout();
        }
    }

    /// Blocks the current thread until the future has resolved.
    ///
    /// This is meant to provide a simple way for non-async consumers
    /// to work with rusoto.
    pub fn sync(self) -> Result<T, E>
        where T: Send + 'static,
              E: From<CredentialsError> + From<HttpDispatchError> + Send + 'static
    {
        spawn(self, &FALLBACK_RUNTIME.executor()).wait()
    }
}

impl<T, E> Future for RusotoFuture<T, E>
    where E: From<CredentialsError> + From<HttpDispatchError>
{
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<T, E> {
        match self.state.take().unwrap() {
            RusotoFutureState::SignAndDispatch { mut future, handler } => {
                match future.poll() {
                    Err(SignAndDispatchError::Credentials(err)) => Err(err.into()),
                    Err(SignAndDispatchError::Dispatch(err)) => Err(err.into()),
                    Ok(Async::Ready(response)) => {
                        self.state = Some(RusotoFutureState::RunningResponseHandler(handler(response)));
                        self.poll()
                    },
                    Ok(Async::NotReady) => {
                        self.state = Some(RusotoFutureState::SignAndDispatch { future, handler });
                        Ok(Async::NotReady)
                    }
                }
            },
            RusotoFutureState::RunningResponseHandler(mut future) => {
                match future.poll()? {
                    Async::Ready(value) => Ok(Async::Ready(value)),
                    Async::NotReady => {
                        self.state = Some(RusotoFutureState::RunningResponseHandler(future));
                        Ok(Async::NotReady)
                    }
                }
            }
        }
    }
}

enum RusotoFutureState<T, E> {
    SignAndDispatch {
        future: Box<TimeoutFuture<Item=HttpResponse, Error=SignAndDispatchError> + Send>,
        handler: fn(HttpResponse) -> Box<Future<Item=T, Error=E> + Send>
    },
    RunningResponseHandler(Box<Future<Item=T, Error=E> + Send>)
}

#[test]
fn rusoto_future_is_send() {
    fn is_send<T: Send>() {}

    is_send::<RusotoFuture<(), ()>>();
}
