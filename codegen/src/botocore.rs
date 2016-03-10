#[cfg(feature = "serde_macros")]
include!("botocore_types.rs");
#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/botocore_types.rs"));
