use std::fmt;
use std::io;

use bytes::Bytes;
use futures::{future, stream, Async, Future, Poll, Stream};
use tokio::io::AsyncRead;

/// Stream of bytes.
pub struct ByteStream {
    size_hint: Option<usize>,
    inner: Box<Stream<Item = Bytes, Error = io::Error> + Send>,
}

impl ByteStream {
    /// Create a new `ByteStream` by wrapping a `futures` stream.
    pub fn new<S>(stream: S) -> ByteStream
    where
        S: Stream<Item = Bytes, Error = io::Error> + Send + 'static,
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
            inner: Box::new(stream::once(Ok(Bytes::from(buf)))),
        }
    }
}

impl fmt::Debug for ByteStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<ByteStream size_hint={:?}>", self.size_hint)
    }
}

impl Stream for ByteStream {
    type Item = Bytes;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

struct ImplAsyncRead {
    buffer: io::Cursor<Bytes>,
    stream: stream::Fuse<Box<Stream<Item = Bytes, Error = io::Error> + Send>>,
}

impl ImplAsyncRead {
    fn new(stream: Box<Stream<Item = Bytes, Error = io::Error> + Send>) -> Self {
        ImplAsyncRead {
            buffer: io::Cursor::new(Bytes::new()),
            stream: stream.fuse(),
        }
    }
}

impl io::Read for ImplAsyncRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        loop {
            let n = self.buffer.read(buf)?;
            if n > 0 {
                return Ok(n);
            }
            match self.stream.poll()? {
                Async::NotReady => {
                    return Err(io::ErrorKind::WouldBlock.into());
                }
                Async::Ready(Some(buffer)) => {
                    self.buffer = io::Cursor::new(buffer);
                    continue;
                }
                Async::Ready(None) => {
                    return Ok(0);
                }
            }
        }
    }
}

impl AsyncRead for ImplAsyncRead {}

struct ImplBlockingRead {
    inner: ImplAsyncRead,
}

impl ImplBlockingRead {
    fn new(stream: Box<Stream<Item = Bytes, Error = io::Error> + Send>) -> Self {
        ImplBlockingRead {
            inner: ImplAsyncRead::new(stream),
        }
    }
}

impl io::Read for ImplBlockingRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        future::poll_fn(|| self.inner.poll_read(buf)).wait()
    }
}

#[test]
fn test_async_read() {
    use std::io::Read;
    use bytes::Bytes;

    let chunks = vec![Bytes::from_static(b"1234"), Bytes::from_static(b"5678")];
    let stream = ByteStream::new(stream::iter_ok(chunks));
    let mut async_read = stream.into_async_read();

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

#[test]
fn test_blocking_read() {
    use std::io::Read;
    use bytes::Bytes;

    let chunks = vec![Bytes::from_static(b"1234"), Bytes::from_static(b"5678")];
    let stream = ByteStream::new(stream::iter_ok(chunks));
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
