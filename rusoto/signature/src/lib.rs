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
#![cfg_attr(not(feature = "unstable"), allow(clippy::type_complexity))]
pub extern crate rusoto_credential as credential;
pub mod region;
pub mod signature;
pub mod stream;
pub use region::Region;
pub use signature::{SignedRequest, SignedRequestPayload};
pub use stream::ByteStream;
