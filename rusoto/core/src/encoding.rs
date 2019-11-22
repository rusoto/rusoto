use crate::signature::SignedRequest;
#[cfg(feature = "encoding")]
use crate::signature::SignedRequestPayload;
#[cfg(feature = "encoding")]
use flate2::{write::GzEncoder, Compression};
#[cfg(feature = "encoding")]
use std::io::Write;

#[derive(Debug, Clone)]
pub enum ContentEncoding {
    /// Indicates the identity function (i.e., no compression or modification)
    Identity,

    /// GzipCompressor uses flate2 library's GzEncoder internally to compress request payloads.
    /// Streaming requests are not supported yet.
    ///
    /// First parameter is for minimum payload. If request payload is lesser than it no compression
    /// will happen.
    ///
    /// Second parameter is Compression level for Gzip
    #[cfg(feature = "encoding")]
    Gzip(Option<usize>, Compression),
}

impl Default for ContentEncoding {
    fn default() -> Self {
        ContentEncoding::Identity
    }
}

impl ContentEncoding {
    #[allow(warnings)] // Ignore request is not being used
    pub fn encode(&self, request: &mut SignedRequest) {
        match self {
            ContentEncoding::Identity => {
                // no compression or modification
                return;
            }
            #[cfg(feature = "encoding")]
            ContentEncoding::Gzip(min_payload_size, level) => {
                match request.payload {
                    None => return,
                    Some(SignedRequestPayload::Buffer(ref payload)) => {
                        if let Some(min_payload_size) = &min_payload_size {
                            if payload.len() < *min_payload_size {
                                return;
                            }
                        }
                        let mut encoder = GzEncoder::new(Vec::<u8>::new(), *level);
                        encoder
                            .write_all(payload)
                            .expect("Request payload was not written to encoder.");
                        let payload_compressed =
                            encoder.finish().expect("Failed to finish compression.");

                        // Don't compress if payload length isn't decreased
                        if payload.len() < payload_compressed.len() {
                            return;
                        }
                        let payload_compressed = bytes::Bytes::from(payload_compressed);
                        request.payload = Some(SignedRequestPayload::Buffer(payload_compressed));
                    }
                    Some(SignedRequestPayload::Stream(ref stream)) => {
                        // Stream compression is not supported yet
                        return;
                    }
                };
                request.add_header("Content-Encoding", "gzip");
            }
        }
    }
}
