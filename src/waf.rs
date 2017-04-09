//! AWS Web Application Firewall
//!
//! If you're using the service, you're probably looking for [WafClient](struct.WafClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/waf.rs"));
