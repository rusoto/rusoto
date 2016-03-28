#[cfg(feature = "serde_macros")]
include!("botocore.in.rs");
#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/botocore.rs"));
