use std::io;
use std::fmt;

use tokio::io::AsyncRead;
use futures::{Stream, stream, Async, Poll};

/// Stream of bytes.
pub struct ByteStream {
    size_hint: Option<usize>,
    inner: Box<Stream<Item = Vec<u8>, Error = io::Error> + Send>,
}

impl ByteStream {
    /// Create a new `ByteStreamy` by wrapping a `futures` stream.
    pub fn new<S>(stream: S) -> ByteStream
    where
        S: Stream<Item = Vec<u8>, Error = io::Error> + Send + 'static,
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
        ImplAsyncRead {
            buffer: io::Cursor::new(Vec::new()),
            stream: self.inner.fuse(),
        }
    }
}

impl From<Vec<u8>> for ByteStream {
    fn from(buf: Vec<u8>) -> ByteStream {
        ByteStream {
            size_hint: Some(buf.len()),
            inner: Box::new(stream::once(Ok(buf))),
        }
    }
}

impl fmt::Debug for ByteStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<ByteStream size_hint={:?}>", self.size_hint)
    }
}

impl Stream for ByteStream {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

struct ImplAsyncRead {
    buffer: io::Cursor<Vec<u8>>,
    stream: stream::Fuse<Box<Stream<Item = Vec<u8>, Error = io::Error> + Send>>,
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
                },
                Async::Ready(Some(buffer)) => {
                    self.buffer = io::Cursor::new(buffer);
                    continue;
                },
                Async::Ready(None) => {
                    return Ok(0);
                }
            }
        }
    }
}

impl AsyncRead for ImplAsyncRead {}

#[test]
fn test_async_read() {
    use std::io::Read;

    let chunks = vec![b"1234".to_vec(), b"5678".to_vec()];
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
