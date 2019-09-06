use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::{Stream, StreamExt};
use pin_project::pin_project;
use tokio::io::AsyncRead;

/// Stream of bytes.
#[pin_project]
pub struct ByteStream {
    size_hint: Option<usize>,
    #[pin]
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>,
}

impl ByteStream {
    /// Create a new `ByteStream` by wrapping a `futures` stream.
    pub fn new<S>(stream: S) -> ByteStream
    where
        S: Stream<Item = Result<Bytes, io::Error>> + Send + 'static,
    {
        ByteStream {
            size_hint: None,
            inner: stream.boxed(),
        }
    }

    pub(crate) fn size_hint(&self) -> Option<usize> {
        self.size_hint
    }

    /// Return an implementation of `AsyncRead` that uses async i/o to consume the stream.
    pub fn into_async_read(self) -> impl AsyncRead + Send {
        ImplAsyncRead::new(self.inner)
    }
}

impl From<Vec<u8>> for ByteStream {
    fn from(buf: Vec<u8>) -> ByteStream {
        ByteStream {
            size_hint: Some(buf.len()),
            inner: futures::stream::once(futures::future::ready(Ok(Bytes::from(buf)))).boxed(),
        }
    }
}

impl fmt::Debug for ByteStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<ByteStream size_hint={:?}>", self.size_hint)
    }
}

impl Stream for ByteStream {
    type Item = Result<Bytes, io::Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(cx)
    }
}

#[pin_project]
struct ImplAsyncRead {
    buffer: io::Cursor<Bytes>,
    #[pin]
    stream: futures::stream::Fuse<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>>,
}

impl ImplAsyncRead {
    fn new(stream: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>) -> Self {
        ImplAsyncRead {
            buffer: io::Cursor::new(Bytes::new()),
            stream: stream.fuse(),
        }
    }
}

impl AsyncRead for ImplAsyncRead {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        match futures::ready!(this.stream.poll_next(cx)) {
            None => Poll::Ready(Ok(0)),
            Some(Err(e)) => Poll::Ready(Err(e)),
            Some(Ok(bytes)) => {
                let n = bytes.len();
                this.buffer.extend(bytes);
                Poll::Ready(Ok(n))
            }
        }
    }
}

//#[test]
//fn test_async_read() {
//    use bytes::Bytes;
//    use std::io::Read;
//
//    let chunks = vec![Ok(Bytes::from_static(b"1234")), Ok(Bytes::from_static(b"5678"))];
//    let stream = ByteStream::new(futures::stream::iter(chunks));
//    let mut async_read = stream.into_async_read();
//
//    let mut buf = [0u8; 3];
//    assert_eq!(async_read.read(&mut buf).unwrap(), 3);
//    assert_eq!(&buf[..3], b"123");
//    assert_eq!(async_read.read(&mut buf).unwrap(), 1);
//    assert_eq!(&buf[..1], b"4");
//    assert_eq!(async_read.read(&mut buf).unwrap(), 3);
//    assert_eq!(&buf[..3], b"567");
//    assert_eq!(async_read.read(&mut buf).unwrap(), 1);
//    assert_eq!(&buf[..1], b"8");
//    assert_eq!(async_read.read(&mut buf).unwrap(), 0);
//}
