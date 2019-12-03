// moved to rusoto_signature
pub use rusoto_signature::region::*;

/// Convenience trait for implementing by service clients.
/// Most of then are associated with a specific region, and it allows consumers to check what this region is.
pub trait GetRegion {
    /// Return the region this service is associated with
    fn region(&self) -> &Region;
}
