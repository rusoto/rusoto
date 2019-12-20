use super::{AwsCredentials, CredentialsError, ProvideAwsCredentials};
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;

/// Object safe trait pendant to `ProvideAwsCredentials` trait allowing better composition
/// (abstraction) e.g. for chained providers.
///
/// # Examples
///
/// One can use the object safe representation to easily create credentials provider chains by
/// combining multiple `AwsCredentialsProvider`s using the `or` method.
///
/// ```rust
/// # use rusoto_credential::*;  
/// # use std::rc::Rc;
/// # use futures::future::Future;
/// let provider = (Box::new(EnvironmentProvider::default()) as Box<dyn AwsCredentialsProvider>).or(Box::new(ContainerProvider::default()));
/// # let _ = provider.credentials().wait();
/// # let _ = provider.fetch_credentials().wait();
/// ```
pub trait AwsCredentialsProvider {
    /// Produce a new `AwsCredentials` future.
    fn fetch_credentials(&self)
        -> Box<dyn Future<Item = AwsCredentials, Error = CredentialsError>>;
}

/// API to chain providers
pub trait WithFallback {
    /// Add fallback provider
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rusoto_credential::*;  
    /// # use std::rc::Rc;
    /// # use futures::future::Future;
    /// let env: Box<dyn AwsCredentialsProvider> = Box::new(EnvironmentProvider::default());
    /// let env_or_container = env.or(Box::new(ContainerProvider::default()));
    /// let env_or_container_or_ec2 = env_or_container.or(Box::new(InstanceMetadataProvider::new()));
    /// # let _ = env_or_container_or_ec2.credentials().wait();
    /// # let _ = env_or_container_or_ec2.fetch_credentials().wait();
    /// ```
    fn or(self, fallback: Self) -> Self;
}

impl WithFallback for Rc<dyn AwsCredentialsProvider> {
    fn or(self, fallback: Self) -> Self {
        Rc::new(AwsCredentialProviderChain::new(self, fallback))
    }
}

impl WithFallback for Arc<dyn AwsCredentialsProvider> {
    fn or(self, fallback: Self) -> Self {
        Arc::new(AwsCredentialProviderChain::new(
            Rc::new(self),
            Rc::new(fallback),
        ))
    }
}

impl WithFallback for Box<dyn AwsCredentialsProvider> {
    fn or(self, fallback: Self) -> Self {
        Box::new(AwsCredentialProviderChain::new(
            self.into(),
            fallback.into(),
        ))
    }
}

impl<P, F> AwsCredentialsProvider for P
where
    P: ProvideAwsCredentials<Future = F> + 'static,
    F: Future<Item = AwsCredentials, Error = CredentialsError> + 'static,
{
    fn fetch_credentials(
        &self,
    ) -> Box<dyn Future<Item = AwsCredentials, Error = CredentialsError>> {
        Box::new(P::credentials(&self))
    }
}

/// Provider chain
#[derive(Clone)]
pub struct AwsCredentialProviderChain {
    primary: Rc<dyn AwsCredentialsProvider>,
    fallback: Rc<dyn AwsCredentialsProvider>,
}

impl AwsCredentialProviderChain {
    /// Create new provider chain
    pub fn new(
        primary: Rc<dyn AwsCredentialsProvider>,
        fallback: Rc<dyn AwsCredentialsProvider>,
    ) -> Self {
        Self { primary, fallback }
    }

    /// Add provider to the end of the provider chain.
    pub fn push<P, F>(self, fallback: P) -> Self
    where
        P: ProvideAwsCredentials<Future = F> + Clone + 'static,
        F: Future<Item = AwsCredentials, Error = CredentialsError> + 'static,
    {
        Self {
            primary: Rc::new(self),
            fallback: Rc::new(fallback),
        }
    }
}

impl AwsCredentialsProvider for AwsCredentialProviderChain {
    fn fetch_credentials(
        &self,
    ) -> Box<dyn Future<Item = AwsCredentials, Error = CredentialsError>> {
        let fb = self.fallback.clone();
        Box::new(
            self.primary
                .fetch_credentials()
                .or_else(move |_| fb.fetch_credentials()),
        )
    }
}

/// Allow going from trait object to trait.
impl ProvideAwsCredentials for Rc<dyn AwsCredentialsProvider> {
    type Future = Box<dyn Future<Item = AwsCredentials, Error = CredentialsError>>;

    fn credentials(&self) -> Self::Future {
        // as_ref() is important as otherwise this would lead to a stack overflow, calling
        self.as_ref().fetch_credentials()
    }
}

impl ProvideAwsCredentials for Arc<dyn AwsCredentialsProvider> {
    type Future = Box<dyn Future<Item = AwsCredentials, Error = CredentialsError>>;

    fn credentials(&self) -> Self::Future {
        // as_ref() is important as otherwise this would lead to a stack overflow, calling
        self.as_ref().fetch_credentials()
    }
}

impl ProvideAwsCredentials for Box<dyn AwsCredentialsProvider> {
    type Future = Box<dyn Future<Item = AwsCredentials, Error = CredentialsError>>;

    fn credentials(&self) -> Self::Future {
        // as_ref() is important as otherwise this would lead to a stack overflow, calling
        self.as_ref().fetch_credentials()
    }
}

impl ProvideAwsCredentials for &dyn AwsCredentialsProvider {
    type Future = Box<dyn Future<Item = AwsCredentials, Error = CredentialsError>>;

    fn credentials(&self) -> Self::Future {
        // as_ref() is important as otherwise this would lead to a stack overflow, calling
        (*self).fetch_credentials()
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use crate::EnvironmentProvider;
    use futures::future::Future;
    use std::rc::Rc;
    use std::sync::Arc;

    fn by_obj(c: &dyn AwsCredentialsProvider) {
        let _ = c.fetch_credentials().wait();
    }

    fn by_obj_trait_ref<P>(c: &P)
    where
        P: AwsCredentialsProvider + ?Sized,
    {
        let _ = c.fetch_credentials().wait();
    }

    fn by_obj_trait<P>(c: &P)
    where
        P: AwsCredentialsProvider,
    {
        let _ = c.fetch_credentials().wait();
    }

    fn by_static_trait_ref<P, F>(c: &P)
    where
        P: ProvideAwsCredentials<Future = F>,
        F: Future<Item = AwsCredentials, Error = CredentialsError> + 'static,
    {
        let _ = c.credentials().wait();
    }

    fn by_static_trait<P, F>(c: P)
    where
        P: ProvideAwsCredentials<Future = F>,
        F: Future<Item = AwsCredentials, Error = CredentialsError> + 'static,
    {
        let _ = c.credentials().wait();
    }

    #[test]
    fn can_use_object_ref() {
        let provider: Box<dyn AwsCredentialsProvider> = Box::new(EnvironmentProvider::default());
        let obj_ref: &dyn AwsCredentialsProvider = &provider;
        by_obj(obj_ref);
        by_obj_trait_ref(obj_ref);
        by_static_trait_ref(&obj_ref);
        by_static_trait(obj_ref);
    }

    #[test]
    fn can_use_boxed() {
        let provider: Box<dyn AwsCredentialsProvider> = Box::new(EnvironmentProvider::default());
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);

        let provider: Box<EnvironmentProvider> = Box::new(EnvironmentProvider::default());
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);
    }

    #[test]
    fn can_use_rc() {
        let provider: Rc<dyn AwsCredentialsProvider> = Rc::new(EnvironmentProvider::default());
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);

        let provider: Rc<EnvironmentProvider> = Rc::new(EnvironmentProvider::default());
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);
    }

    #[test]
    fn can_use_arc() {
        let provider: Arc<dyn AwsCredentialsProvider> = Arc::new(EnvironmentProvider::default());
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);

        let provider: Arc<EnvironmentProvider> = Arc::new(EnvironmentProvider::default());
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);
    }

    #[test]
    fn can_chain_box() {
        let provider: Box<dyn AwsCredentialsProvider> = Box::new(EnvironmentProvider::default());
        let provider = provider.or(Box::new(ContainerProvider::new()));
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);
    }

    #[test]
    fn can_chain_rc() {
        let provider: Rc<dyn AwsCredentialsProvider> = Rc::new(EnvironmentProvider::default());
        let provider = provider.or(Rc::new(ContainerProvider::new()));
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);
    }

    #[test]
    fn can_chain_arc() {
        let provider: Arc<dyn AwsCredentialsProvider> = Arc::new(EnvironmentProvider::default());
        let provider = provider.or(Arc::new(ContainerProvider::new()));
        by_obj(&provider);
        by_obj_trait_ref(&provider);
        by_obj_trait(&provider);
        by_static_trait_ref(&provider);
        by_static_trait(provider);
    }
}
