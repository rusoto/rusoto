use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::{future, stream, Stream, StreamExt};
use pin_project::pin_project;
use tokio::io::AsyncRead;

/// Stream of bytes.
#[pin_project]
pub struct ByteStream {
    size_hint: Option<usize>,
    #[pin]
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + 'static>>,
}

impl ByteStream {
    /// Create a new `ByteStream` by wrapping a `futures` stream.
    pub fn new<S>(stream: S) -> ByteStream
    where
        S: Stream<Item = Result<Bytes, io::Error>> + Send + Sync + 'static,
    {
        ByteStream {
            size_hint: None,
            inner: Box::pin(stream),
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
            inner: Box::pin(stream::once(async move { Ok(Bytes::from(buf)) })),
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

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(cx)
    }
}

#[pin_project]
struct ImplAsyncRead {
    buffer: io::Cursor<Bytes>,
    #[pin]
    stream: stream::Fuse<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync>>>,
}

impl ImplAsyncRead {
    fn new(stream: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync>>) -> Self {
        ImplAsyncRead {
            buffer: io::Cursor::new(Bytes::new()),
            stream: stream.fuse(),
        }
    }
}

// impl io::Read for ImplAsyncRead {
// fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//     if buf.is_empty() {
//         return Ok(0);
//     }
//     let waker = futures::task::noop_waker();
//     let mut cx = Context::from_waker(&waker);
//     loop {
//         let n = self.buffer.read(buf)?;
//         if n > 0 {
//             return Ok(n);
//         }
//         match self.stream.poll_next_unpin(&mut cx)? {
//             Poll::Pending => {
//                 return Err(io::ErrorKind::WouldBlock.into());
//             }
//             Poll::Ready(Some(buffer)) => {
//                 self.buffer = io::Cursor::new(buffer);
//                 continue;
//             }
//             Poll::Ready(None) => {
//                 return Ok(0);
//             }
//         }
//     }
// }
// }

impl tokio::io::AsyncRead for ImplAsyncRead {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut this = self.get_mut();
        if buf.is_empty() {
            return Poll::Ready(Ok(0));
        }
        loop {
            let n = std::io::Read::read(&mut this.buffer, buf)?;
            if n > 0 {
                return Poll::Ready(Ok(n));
            }
            match Pin::new(&mut this.stream).poll_next(cx)? {
                Poll::Pending => {
                    return Poll::Pending;
                }
                Poll::Ready(Some(buffer)) => {
                    this.buffer = io::Cursor::new(buffer);
                    continue;
                }
                Poll::Ready(None) => {
                    return Poll::Ready(Ok(0));
                }
            }
        }
    }
}

#[pin_project]
struct ImplBlockingRead {
    #[pin]
    inner: ImplAsyncRead,
}

impl ImplBlockingRead {
    fn new(stream: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync>>) -> Self {
        ImplBlockingRead {
            inner: ImplAsyncRead::new(stream),
        }
    }
}

impl io::Read for ImplBlockingRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(future::poll_fn(|cx| {
            tokio::io::AsyncRead::poll_read(Pin::new(&mut self.inner), cx, buf)
        }))
    }
}

#[tokio::test]
async fn test_async_read() {
    use bytes::Bytes;
    use tokio::io::AsyncReadExt;

    let chunks = vec![
        Ok(Bytes::from_static(b"1234")),
        Ok(Bytes::from_static(b"5678")),
    ];
    let stream = ByteStream::new(stream::iter(chunks));
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

    let chunks = vec![
        Ok(Bytes::from_static(b"1234")),
        Ok(Bytes::from_static(b"5678")),
    ];
    let stream = ByteStream::new(stream::iter(chunks));
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
