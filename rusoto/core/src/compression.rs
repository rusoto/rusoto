use std::rc::Rc;
use std::sync::Arc;
use crate::signature::{SignedRequest, SignedRequestPayload};
use bytes::Bytes;
use std::io::Write;

/// Trait for implementing request payload compression
pub trait CompressRequestPayload {
    fn compress(&self, request: &mut SignedRequest);
}

impl<C: CompressRequestPayload> CompressRequestPayload for Rc<C> {
    fn compress(&self, request: &mut SignedRequest) {
        C::compress(&*self, request)
    }
}

impl<C: CompressRequestPayload> CompressRequestPayload for Arc<C> {
    fn compress(&self, request: &mut SignedRequest) {
        C::compress(&*self, request)
    }
}

// GzipCompressor uses libflate::gzip internally to compress request payloads
pub struct GzipCompressor {
    // Min payload size for compression
    pub min_payload_size_for_compression: Option<usize>,
}

impl GzipCompressor {
    pub fn new() -> Self {
        return GzipCompressor {
            min_payload_size_for_compression: Some(1024),
        };
    }
}

impl CompressRequestPayload for GzipCompressor {
    fn compress(&self, request: &mut SignedRequest) {
        match request.payload {
            None => return,
            Some(SignedRequestPayload::Buffer(ref payload)) => {
                if self.min_payload_size_for_compression.is_some() {
                    if payload.len() < self.min_payload_size_for_compression.unwrap() {
                        return;
                    }
                }
                let mut encoder = libflate::gzip::Encoder::new(Vec::<u8>::new()).unwrap();
                encoder.write_all(payload).unwrap();
                let payload_compressed = encoder.finish().into_result().unwrap();

                // Don't compress if payload length isn't decreased
                if payload.len() < payload_compressed.len() {
                    return;
                }
                let payload_compressed = Bytes::from(payload_compressed);
                request.payload = Some(SignedRequestPayload::Buffer(payload_compressed));
            }
            Some(SignedRequestPayload::Stream(ref stream)) => {
                // TODO what to do for streams?
                let _len = stream.size_hint().unwrap_or_else(|| 0);
                return;
            }
        };
        request.add_header("Content-Encoding", "gzip");
    }
}
