/// Newtype (pattern) to protect secret credentials stored as Strings.
#[derive(Clone)]
pub struct Secret(String);

impl From<String> for Secret {
    fn from(s: String) -> Self {
        Secret(s)
    }
}

/// Allow dereferencing Secrets as &str.
///
/// ```rust
/// # use rusoto_credential::Secret;
/// assert_eq!(Secret::from("hello".to_string()).as_ref(), "hello");
/// ```
impl AsRef<str> for Secret {
    fn as_ref(&self) -> &str {
        return self.0.as_str();
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        //TODO Should we use (and depend on) https://docs.rs/zeroize/1.0.0/zeroize/ instead?
        let s = &mut self.0;
        unsafe {
            for c in s.as_bytes_mut() {
                core::ptr::write_volatile(c, 0);
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            }
        }
        s.clear();
    }
}

/// Secrets must not leak, so make sure they are not printed.
///
/// ```rust
/// # use rusoto_credential::Secret;
/// assert_eq!(format!("{:?}",Secret::from("hello".to_string())), "*******");
/// ```
impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*******")
    }
}

/// Secrets must not leak, so make sure they are not displayed.
///
/// ```rust
/// # use rusoto_credential::Secret;
/// assert_eq!(format!("{}",Secret::from("hello".to_string())), "*******");
/// ```
impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*******")
    }
}
