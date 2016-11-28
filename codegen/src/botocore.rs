#[cfg(feature = "serde_derive")]
include!("botocore.in.rs");
#[cfg(not(feature = "serde_derive"))]
include!(concat!(env!("OUT_DIR"), "/botocore.rs"));
