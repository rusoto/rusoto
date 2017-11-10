use futures::{Future, Poll};

/// TODO: docs
pub struct RusotoFuture<T, E>(Box<Future<Item=T, Error=E> + 'static>);

impl<T, E> RusotoFuture<T, E> {
    /// TODO: docs
    pub fn new<F>(future: F) -> RusotoFuture<T, E>
        where F: Future<Item=T, Error=E> + 'static
    {
        RusotoFuture(Box::new(future))
    }

    /// TODO: docs
    pub fn sync(self) -> Result<T, E> {
        self.wait()
    }
}

impl<T, E> Future for RusotoFuture<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}
