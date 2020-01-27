//use crate::WebIdentityProvider;
//use futures::{Future, Poll};
//use rusoto_core::credential::{
//    AutoRefreshingProvider, AutoRefreshingProviderFuture, AwsCredentialProviderChain,
//    AwsCredentials, AwsCredentialsProvider, ContainerProvider, CredentialsError,
//    EnvironmentProvider, InstanceMetadataProvider, ProfileProvider, ProvideAwsCredentials,
//};
//use std::rc::Rc;
//use std::time::Duration;

/// Default credential provider.
///
/// Checks multiple sources for credentials, and uses `AutoRefreshingProvider` to refreshes the
/// credentials automatically when they expire. Currently the following providers are probed in
/// order, taking the credentials from the first provider that returns without an error:
///
/// - EnvironmentProvider
/// - WebIdentityProvider (Kubernets)
/// - ProfileProvider (optional)
/// - ContainerProvider (ECS/Fargate)
/// - InstanceMetadataProvider
///
/// [credential_process]: https://docs.aws.amazon.com/cli/latest/topic/config-vars.html#sourcing-credentials-from-external-processes

//#[derive(Clone)]
//pub struct DefaultCredentialsProvider(AutoRefreshingProvider<Rc<dyn AwsCredentialsProvider>>);
//
//impl DefaultCredentialsProvider {
//    pub fn new(profile_provider: Option<ProfileProvider>, timeout: Option<Duration>) -> Self {
//        let provider = AwsCredentialProviderChain::new(
//            Rc::new(EnvironmentProvider::default()),
//            Rc::new(WebIdentityProvider::from_k8s_env()),
//        );
//        let provider = match profile_provider {
//            Some(p) => provider.push(p),
//            _ => provider,
//        };
//        let mut container_provider = ContainerProvider::new();
//        let mut instance_provider = InstanceMetadataProvider::new();
//        timeout.map(|t| {
//            container_provider.set_timeout(t.clone());
//            instance_provider.set_timeout(t);
//        });
//        let provider = provider.push(container_provider).push(instance_provider);
//        Self(AutoRefreshingProvider::from(Rc::new(provider)))
//    }
//
//    /// Creates a potentially insecure default credential provider chain.
//    ///
//    /// Checks multiple sources for credentials, and uses `AutoRefreshingProvider` to refreshes the
//    /// credentials automatically when they expire. Currently the following providers are probed in
//    /// order, taking the credentials from the first provider that returns without an error:
//    ///
//    /// - EnvironmentProvider
//    /// - WebIdentityProvider (Kubernets)
//    /// - ProfileProvider (optional)
//    /// - ContainerProvider (ECS/Fargate)
//    /// - InstanceMetadataProvider
//    ///
//    /// # Warning
//    ///
//    /// This provider allows the [`credential_process`][credential_process] option, a method of
//    /// sourcing credentials from an external process. This can potentially be dangerous, so proceed
//    /// with caution. Other credential providers should be preferred if at all possible. If using this
//    /// option, you should make sure that the config file is as locked down as possible using security
//    /// best practices for your operating system.
//    pub fn insecure() -> Self {
//        Self::new(ProfileProvider::new().ok(), None)
//    }
//
//    /// Create secure default credential provider chain.
//    ///
//    /// Checks multiple sources for credentials, and uses `AutoRefreshingProvider` to refreshes the
//    /// credentials automatically when they expire. Currently the following providers are probed in
//    /// order, taking the credentials from the first provider that returns without an error:
//    ///
//    /// - EnvironmentProvider
//    /// - WebIdentityProvider (Kubernets)
//    /// - ProfileProvider (optional)
//    /// - ContainerProvider (ECS/Fargate)
//    /// - InstanceMetadataProvider
//    ///
//    /// This provider **does not** allow the [`credential_process`][credential_process] option
//    /// in the AWS config file (`~/.aws/config`) and therefore the returned provider can be
//    /// considered secure.
//    pub fn secure() -> Self {
//        let mut profile = ProfileProvider::new().ok();
//        if let Some(p) = &mut profile {
//            p.secure();
//        }
//        Self::new(profile, None)
//    }
//}
//
//impl Default for DefaultCredentialsProvider {
//    /// Create secure default credential provider chain.
//    fn default() -> Self {
//        Self::secure()
//    }
//}
//
//impl ProvideAwsCredentials for DefaultCredentialsProvider {
//    type Future = DefaultCredentialsProviderFuture;
//
//    fn credentials(&self) -> Self::Future {
//        let inner = self.0.credentials();
//        DefaultCredentialsProviderFuture(inner)
//    }
//}
//
///// Future returned from `DefaultCredentialsProvider`.
//pub struct DefaultCredentialsProviderFuture(
//    AutoRefreshingProviderFuture<Rc<dyn AwsCredentialsProvider>>,
//);
//
//impl Future for DefaultCredentialsProviderFuture {
//    type Item = AwsCredentials;
//    type Error = CredentialsError;
//
//    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//        self.0.poll()
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn api() {
//        let provider = DefaultCredentialsProvider::default();
//        let future = provider.credentials();
//        let _result = future.wait();
//    }
//}
