use std::time::Duration;

use futures::{Future, Poll};

use super::client_inner::ClientInnerFuture;

/// Future that is returned from all rusoto service APIs.
pub struct RusotoFuture<T, E> {
    inner: Box<ClientInnerFuture<Item=T, Error=E> + Send>
}

impl<T, E> RusotoFuture<T, E> {
    #[doc(hidden)]
    pub fn new<F>(future: F) -> RusotoFuture<T, E>
        where F: ClientInnerFuture<Item=T, Error=E> + 'static + Send
    {
        RusotoFuture { inner: Box::new(future) }
    }

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
        self.inner.set_timeout(timeout);
    }

    /// Clear the timeout on the future.
    ///
    /// This is only guaranteed to take effect when called before the future
    /// is polled for the first time.
    pub fn clear_timeout(&mut self) {
        self.inner.clear_timeout();
    }

    /// Blocks the current thread until the future has resolved.
    ///
    /// This is meant to provide a simple way for non-async consumers
    /// to work with rusoto.
    pub fn sync(self) -> Result<T, E> {
        self.wait()
    }
}

impl<T, E> Future for RusotoFuture<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}
