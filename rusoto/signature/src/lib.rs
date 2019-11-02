//! AWS API request signatures.
//!
//! Follows [AWS Signature 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html)
//! algorithm.
//!
//! If needed, the request will be re-issued to a temporary redirect endpoint.  This can happen with
//! newly created S3 buckets not in us-standard/us-east-1.
//!
//! Please note that this module does not expect URIs to already be encoded.
//!
#![cfg_attr(not(feature = "unstable"), deny(warnings))]
pub extern crate rusoto_credential as credential;
pub mod region;
pub mod sign;
pub mod signature;

#[cfg(feature = "futures-01")]
pub mod stream;
pub use region::Region;
pub use sign::Sign;
pub use signature::SignedRequest;
#[cfg(feature = "futures-01")]
pub use signature::SignedRequestPayload;
#[cfg(feature = "futures-01")]
pub use stream::ByteStream;
