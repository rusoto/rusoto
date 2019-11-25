use crate::signature::SignedRequest;
#[cfg(feature = "encoding")]
use crate::signature::SignedRequestPayload;
#[cfg(feature = "encoding")]
use flate2::{write::GzEncoder, Compression};
#[cfg(feature = "encoding")]
use std::io::Write;

// Default compression level for gzip defined same as flate2
pub const DEFAULT_GZIP_COMPRESSION_LEVEL: u32 = 6;

#[derive(Debug, Clone)]
pub enum ContentEncoding {
    /// Indicates the identity function (i.e., no compression or modification)
    Identity,

    /// Gzip encoding uses flate2 library's GzEncoder internally to compress request payloads.
    /// Streaming requests are not supported yet.
    ///
    /// First parameter is for minimum payload. If request payload length is lesser than it
    /// no compression will be performed.
    ///
    /// Second parameter is the compression level for gzip on a scale of 0-9 where 0 means
    /// "no compression" and 9 means "take as long as you'd like".
    #[cfg(feature = "encoding")]
    Gzip(Option<usize>, u32),
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
                        let mut encoder = GzEncoder::new(Vec::<u8>::new(), Compression::new(*level));
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
