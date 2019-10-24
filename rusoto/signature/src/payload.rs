// todo: move to hyper::Payload
use bytes::Bytes;
use futures::{Async, Poll, Stream};
use hyper::body::Payload;
use std::io;

use crate::SignedRequestPayload;

pub struct Body {
    inner: Option<SignedRequestPayload>,
}

impl Body {
    pub fn new(inner: Option<SignedRequestPayload>) -> Self {
        Self { inner }
    }
}

impl Payload for Body {
    type Data = io::Cursor<Bytes>;
    type Error = io::Error;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        match self.inner {
            None => Ok(Async::Ready(None)),
            Some(SignedRequestPayload::Buffer(ref mut buffer)) => {
                if buffer.is_empty() {
                    Ok(Async::Ready(None))
                } else {
                    Ok(Async::Ready(Some(io::Cursor::new(buffer.split_off(0)))))
                }
            }
            Some(SignedRequestPayload::Stream(ref mut stream)) => match stream.poll()? {
                Async::NotReady => Ok(Async::NotReady),
                Async::Ready(None) => Ok(Async::Ready(None)),
                Async::Ready(Some(buffer)) => Ok(Async::Ready(Some(io::Cursor::new(buffer)))),
            },
        }
    }

    fn is_end_stream(&self) -> bool {
        match self.inner {
            None => true,
            Some(SignedRequestPayload::Buffer(ref buffer)) => buffer.is_empty(),
            Some(SignedRequestPayload::Stream(_)) => false,
        }
    }

    fn content_length(&self) -> Option<u64> {
        match self.inner {
            None => Some(0),
            Some(SignedRequestPayload::Buffer(ref buffer)) => Some(buffer.len() as u64),
            Some(SignedRequestPayload::Stream(ref stream)) => stream.size_hint().map(|s| s as u64),
        }
    }
}
