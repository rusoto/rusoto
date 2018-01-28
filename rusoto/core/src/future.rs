use futures::{Future, Poll};

/// Future that is returned from all rusoto service APIs.
pub struct RusotoFuture<T, E> {
    inner: Box<Future<Item=T, Error=E> + 'static>
}

impl<T, E> RusotoFuture<T, E> {
    #[doc(hidden)]
    pub fn new<F>(future: F) -> RusotoFuture<T, E>
        where F: Future<Item=T, Error=E> + 'static
    {
        RusotoFuture { inner: Box::new(future) }
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
