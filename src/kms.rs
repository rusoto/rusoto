//! AWS Key Management Service
//!
//! If you're using the service, you're probably looking for [KmsClient](struct.KmsClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/kms.rs"));
