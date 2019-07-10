use std::rc::Rc;
use std::sync::Arc;
use crate::signature::SignedRequest;

pub struct CompressionOptions {
    pub min_payload_size_for_compression: Option<usize>,
}

/// Trait for implementing request payload compression
pub trait CompressRequestPayload {
    fn compress(&self, request: &mut SignedRequest, compression_options: &CompressionOptions);
    fn compression_options(&self) -> &Option<CompressionOptions>;
}

impl<C: CompressRequestPayload> CompressRequestPayload for Rc<C> {
    fn compress(&self, request: &mut SignedRequest, compression_options: &CompressionOptions) {
        C::compress(&*self, request, compression_options)
    }

    fn compression_options(&self) -> &Option<CompressionOptions> {
        C::compression_options(&*self)
    }
}

impl<C: CompressRequestPayload> CompressRequestPayload for Arc<C> {
    fn compress(&self, request: &mut SignedRequest, compression_options: &CompressionOptions) {
        C::compress(&*self, request, compression_options)
    }

    fn compression_options(&self) -> &Option<CompressionOptions> {
        C::compression_options(&*self)
    }
}
