extern crate rusoto;
extern crate serde;

#[cfg(feature = "dynamodb")]
pub mod dynamodb;
#[cfg(feature = "kms")]
pub mod kms;
#[cfg(feature = "sqs")]
pub mod sqs;
