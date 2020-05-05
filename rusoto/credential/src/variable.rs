use std::convert::From;
use std::env::{var, VarError};
use std::fmt;
use std::sync::Arc;

/// Variable is an abstraction over parameters to credential providers, allowing to abstract on
/// how (source) and when (time) parameter values are resolved. A lot of credentials providers
/// use external information sources such as environment variables or files to obtain parameter
/// values needed to produce AWS credentials.
///
/// # Information Sources
///
/// - In memory values (static)
/// - Environment variables (dynamic)
/// - Files (dynamic)
/// - ...
///
/// # Resolving Behaviour
///
/// - Static variables always resolve to the same value.
/// - Dynamic variables can resolve to different values over time.
///
/// Most prominent examples for dynamic variables are parameters which read their value from
/// environment variables or files.
pub enum Variable<T, E = super::CredentialsError> {
    /// Static variable always resolving to the same given value.
    Static(T),
    /// Dynamic variable can resolve to different values over time.
    Dynamic(Arc<dyn Fn() -> Result<T, E> + Send + Sync>),
    /// Fallback try variables in given order returning the value of the first variable that
    /// does resolve.
    Fallback(Box<Variable<T, E>>, Box<Variable<T, E>>),
}

impl<T, E> From<T> for Variable<T, E>
where
    T: Clone,
{
    fn from(value: T) -> Self {
        Variable::with_value(value)
    }
}

impl<E> From<&str> for Variable<String, E> {
    fn from(value: &str) -> Self {
        Variable::with_value(value.to_owned())
    }
}

/*
impl<T, E, V> From<&V> for Variable<T, E>
where
    T: Clone + std::borrow::Borrow<V>,
    V: ToOwned<Owned = T> + ?Sized,
{
    fn from(value: &V) -> Self {
        Variable::with_value(value.to_owned())
    }
}*/

impl<T: fmt::Debug, E> fmt::Debug for Variable<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static(t) => write!(f, "Variable::Static({:?})", t),
            Self::Dynamic(_) => write!(f, "Variable::Dynamic(...)"),
            Self::Fallback(a, b) => write!(f, "Variable::Fallback({:?}, {:?})", a, b),
        }
    }
}

/// Custom Clone implementation as type parameter E doesn't have to be cloneable.
impl<T: Clone, E> Clone for Variable<T, E> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(t) => Self::Static(t.clone()),
            Self::Dynamic(f) => Self::Dynamic(f.clone()),
            Self::Fallback(a, b) => Self::Fallback(a.clone(), b.clone()),
        }
    }
}

impl<T: Clone, E> Variable<T, E> {
    /// Variable which statically resolves to a provided (in-memory) value.
    pub fn with_value<V: Into<T>>(value: V) -> Self {
        Self::Static(value.into())
    }

    /// Resolve the variable's value.
    pub fn resolve(&self) -> Result<T, E> {
        match self {
            Self::Static(t) => Ok(t.clone()),
            Self::Dynamic(f) => f(),
            Self::Fallback(a, b) => a.resolve().or_else(|_| b.resolve()),
        }
    }

    /// Combine this Variable with a fallback Variable. Resolving the variable's value will be
    /// done lazily, stopping on the first Variable that successfuly resolves.
    ///
    /// # Example Usage
    ///
    /// ```rust
    /// # use rusoto_credential::Variable;
    /// let primary: Variable<String> = Variable::from_env_var("AWS_SECRET_ACCESS_KEY");
    /// let fallback = Variable::from_env_var("AWS_SECRET_KEY");
    /// let aws_secret_access_key = primary.or(fallback);
    /// ```
    pub fn or(self, other: Variable<T, E>) -> Self {
        Self::Fallback(Box::new(self), Box::new(other))
    }
}

impl<T: 'static, E: 'static> Variable<T, E> {
    /// Variable which dynamically resolves to the value returned from the provided closure. Use
    /// this constructor function to create dynamically resolving Variables with custom logic.
    pub fn dynamic(f: impl Fn() -> Result<T, E> + Send + Sync + 'static) -> Self {
        Self::Dynamic(Arc::new(f))
    }
}

impl<T, E> Variable<T, E>
where
    T: From<String> + 'static,
    E: From<VarError> + 'static,
{
    /// Variable which dynamically resolves to the value of a given environment variable.
    pub fn from_env_var<K: AsRef<std::ffi::OsStr>>(key: K) -> Self {
        let tmp = key.as_ref().to_os_string();
        Self::dynamic(move || match var(&tmp) {
            Ok(ref v) if !v.trim().is_empty() => Ok(T::from(v.trim().to_string())),
            Ok(_) => Err(E::from(VarError::NotPresent)),
            Err(e) => Err(E::from(e)),
        })
    }
}

impl<T, E> Variable<Option<T>, E>
where
    T: From<String> + 'static,
    E: From<VarError> + 'static,
{
    /// Variable which dynamically resolves to the value of a given environment variable.
    pub fn from_env_var_optional<K: AsRef<std::ffi::OsStr>>(key: K) -> Self {
        let tmp = key.as_ref().to_os_string();
        Self::dynamic(move || match var(&tmp) {
            Ok(ref v) if !v.trim().is_empty() => Ok(Some(T::from(v.trim().to_string()))),
            Ok(_) | Err(VarError::NotPresent) => Ok(None),
            Err(e) => Err(E::from(e)),
        })
    }
}

impl<T, E> Variable<T, E>
where
    T: From<String> + 'static,
    E: From<std::io::Error> + From<std::string::FromUtf8Error> + 'static,
{
    /// Variable which dynamically resolves to the value of an UTF-8 encoded text file
    /// (removing all leading and trailing whitespaces.
    pub fn from_text_file<K: AsRef<std::path::Path>>(file: K) -> Self {
        use std::fs::read;
        let tmp = file.as_ref().to_path_buf();
        Self::dynamic(move || {
            Ok(T::from(
                String::from_utf8(read(&tmp)?)?.as_str().trim().to_string(),
            ))
        })
    }
}

impl<T, E> Variable<T, E>
where
    T: From<Vec<u8>> + 'static,
    E: From<std::io::Error> + 'static,
{
    /// Variable which dynamically resolves to the value of a binary file.
    pub fn from_binary_file<K: AsRef<std::path::Path>>(file: K) -> Self {
        use std::fs::read;
        let tmp = file.as_ref().to_path_buf();
        Self::dynamic(move || Ok(T::from(read(&tmp)?)))
    }
}

#[cfg(test)]
mod test {
    use super::super::CredentialsError;
    use super::*;
    use crate::test_utils::lock_env;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn api_ergonomics() {
        let _tmp: Variable<String> = "".to_string().into();
        let _tmp: Variable<String> = "".into();
        let _tmp: Variable<u32> = 1.into();
    }

    #[test]
    fn is_send() {
        fn _send<T: Send>(_t: T) {}
        let var = Variable::<i32, ()>::with_value(1);
        _send(var);
    }

    #[test]
    fn is_sync() {
        fn _sync<T: Sync>(_t: T) {}
        let var = Variable::<i32, ()>::with_value(1);
        _sync(var);
    }

    #[test]
    fn from_value() {
        let var = Variable::<i32, ()>::with_value(1);
        assert_eq!(var.resolve(), Ok(1));
        assert_eq!(var.resolve(), Ok(1));
    }

    #[test]
    fn dynamic() {
        fn xx() -> Result<i32, ()> {
            Ok(1)
        }
        let var = Variable::<i32, ()>::dynamic(|| Ok(1));
        assert_eq!(var.resolve(), Ok(1));
        assert_eq!(var.resolve(), Ok(1));
        let var = Variable::<i32, ()>::dynamic(xx);
        assert_eq!(var.resolve(), Ok(1));
        assert_eq!(var.resolve(), Ok(1));
    }

    #[test]
    fn from_env_var() {
        let _guard = lock_env();
        const VALUE: &str = "E6591691_C658_4C63_A7CF_C822D8FFC15B";
        std::env::set_var(VALUE, VALUE);
        let var = Variable::<String, CredentialsError>::from_env_var(VALUE);
        assert_eq!(var.resolve(), Ok(VALUE.to_string()));
        assert_eq!(var.resolve(), Ok(VALUE.to_string()));
        std::env::remove_var(VALUE);
        assert_eq!(var.resolve().is_ok(), false);
    }

    #[test]
    fn from_empty_env_var() {
        let _guard = lock_env();
        const VALUE: &str = "90E839DA_2254_4416_8295_AB82BD44D822";
        std::env::set_var(VALUE, "");
        let var = Variable::<String>::from_env_var(VALUE);
        assert_eq!(var.resolve().is_ok(), false);
        std::env::remove_var(VALUE);
    }

    #[test]
    fn from_text_file() -> Result<(), CredentialsError> {
        const VALUE: &str = "value";
        let mut file = NamedTempFile::new()?;
        writeln!(file, "{}", VALUE)?;
        let var = Variable::<String>::from_text_file(file.path());
        assert_eq!(var.resolve(), Ok(VALUE.to_string()));
        assert_eq!(var.resolve(), Ok(VALUE.to_string()));
        Ok(())
    }

    #[test]
    fn from_binary_file() -> Result<(), CredentialsError> {
        const VALUE: &[u8] = b"value";
        let mut file = NamedTempFile::new()?;
        file.write(VALUE)?;
        let var = Variable::<Vec<u8>>::from_binary_file(file.path());
        assert_eq!(var.resolve().as_ref().map(|v| v.as_slice()), Ok(VALUE));
        assert_eq!(var.resolve().as_ref().map(|v| v.as_slice()), Ok(VALUE));
        Ok(())
    }

    #[test]
    fn or() {
        let a = Variable::<i32, ()>::with_value(1);
        let b = Variable::<i32, ()>::with_value(2);
        assert_eq!(a.or(b).resolve(), Ok(1));
        let a = Variable::<i32, VarError>::dynamic(|| Err(VarError::NotPresent));
        let b = Variable::<i32, VarError>::with_value(2);
        assert_eq!(a.or(b).resolve(), Ok(2));
        let a = Variable::<i32, VarError>::dynamic(|| Err(VarError::NotPresent));
        let b = Variable::<i32, VarError>::dynamic(|| Err(VarError::NotPresent));
        let c = Variable::<i32, VarError>::with_value(3);
        assert_eq!(a.or(b).or(c).resolve(), Ok(3));
        let a = Variable::<i32, VarError>::dynamic(|| Err(VarError::NotPresent));
        let b = Variable::<i32, VarError>::dynamic(|| Err(VarError::NotPresent));
        assert_eq!(a.or(b).resolve(), Err(VarError::NotPresent));
    }
}
