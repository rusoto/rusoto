use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::{executor::block_on, future::{self, ok}, stream, Stream, StreamExt};
use tokio::{io::AsyncRead};

/// Stream of bytes.
pub struct ByteStream {
    size_hint: Option<usize>,
    inner: Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin + 'static>,
}

impl ByteStream {
    /// Create a new `ByteStream` by wrapping a `futures` stream.
    pub fn new<S>(stream: S) -> ByteStream
    where
        S: Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin + 'static,
    {
        ByteStream {
            size_hint: None,
            inner: Box::new(stream),
        }
    }

    pub(crate) fn size_hint(&self) -> Option<usize> {
        self.size_hint
    }

    /// Return an implementation of `AsyncRead` that uses async i/o to consume the stream.
    pub fn into_async_read(self) -> impl AsyncRead + Send {
        ImplAsyncRead::new(self.inner)
    }

    /// Return an implementation of `Read` that uses blocking i/o to consume the stream.
    pub fn into_blocking_read(self) -> impl io::Read + Send {
        ImplBlockingRead::new(self.inner)
    }
}

impl From<Vec<u8>> for ByteStream {
    fn from(buf: Vec<u8>) -> ByteStream {
        ByteStream {
            size_hint: Some(buf.len()),
            inner: Box::new(stream::once(ok(Bytes::from(buf)))),
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

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.get_mut().inner).poll_next(cx)
    }
}

struct ImplAsyncRead {
    buffer: io::Cursor<Bytes>,
    stream: stream::Fuse<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin>>,
}

impl ImplAsyncRead {
    fn new(stream: Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin>) -> Self {
        ImplAsyncRead {
            buffer: io::Cursor::new(Bytes::new()),
            stream: stream.fuse(),
        }
    }
}

impl AsyncRead for ImplAsyncRead {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        if buf.is_empty() {
            return Poll::Ready(Ok(0));
        }
        let &mut Self { ref mut buffer, ref mut stream } = self.get_mut();
        let mut stream = stream;
        loop {
            use std::io::Read;

            let n = buffer.read(buf)?;
            if n > 0 {
                return Poll::Ready(Ok(n));
            }
            match Pin::new(&mut stream).poll_next(cx)? {
                Poll::Pending => {
                    return Poll::Ready(Err(io::ErrorKind::WouldBlock.into()));
                }
                Poll::Ready(Some(buf)) => {
                    *buffer = io::Cursor::new(buf);
                    continue;
                }
                Poll::Ready(None) => {
                    return Poll::Ready(Ok(0));
                }
            }
        }
    }
}

struct ImplBlockingRead {
    inner: ImplAsyncRead,
}

impl ImplBlockingRead {
    fn new(stream: Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin>) -> Self {
        ImplBlockingRead {
            inner: ImplAsyncRead::new(stream),
        }
    }
}

impl io::Read for ImplBlockingRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        block_on(future::poll_fn(|cx| Pin::new(&mut self.inner).poll_read(cx, buf)))
    }
}

#[tokio::test]
async fn test_async_read() {
    use bytes::Bytes;
    use tokio::io::AsyncReadExt;

    let chunks = vec![Bytes::from_static(b"1234"), Bytes::from_static(b"5678")];
    let stream = ByteStream::new(stream::iter(chunks.into_iter().map(Ok)));
    let mut async_read = stream.into_async_read();

    let mut buf = [0u8; 3];
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 3);
    assert_eq!(&buf[..3], b"123");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 1);
    assert_eq!(&buf[..1], b"4");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 3);
    assert_eq!(&buf[..3], b"567");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 1);
    assert_eq!(&buf[..1], b"8");
    assert_eq!(async_read.read(&mut buf).await.unwrap(), 0);
}

#[test]
fn test_blocking_read() {
    use bytes::Bytes;
    use std::io::Read;

    let chunks = vec![Bytes::from_static(b"1234"), Bytes::from_static(b"5678")];
    let stream = ByteStream::new(stream::iter(chunks.into_iter().map(Ok)));
    let mut async_read = stream.into_blocking_read();

    let mut buf = [0u8; 3];
    assert_eq!(async_read.read(&mut buf).unwrap(), 3);
    assert_eq!(&buf[..3], b"123");
    assert_eq!(async_read.read(&mut buf).unwrap(), 1);
    assert_eq!(&buf[..1], b"4");
    assert_eq!(async_read.read(&mut buf).unwrap(), 3);
    assert_eq!(&buf[..3], b"567");
    assert_eq!(async_read.read(&mut buf).unwrap(), 1);
    assert_eq!(&buf[..1], b"8");
    assert_eq!(async_read.read(&mut buf).unwrap(), 0);
}
