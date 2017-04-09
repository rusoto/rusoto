//! AWS CloudHSM
//!
//! If you're using the service, you're probably looking for [CloudHsmClient](struct.CloudHsmClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/cloudhsm.rs"));
