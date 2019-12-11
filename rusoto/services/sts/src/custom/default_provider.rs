use crate::WebIdentityProvider;
use async_trait::async_trait;
use rusoto_core::credential::{
    AutoRefreshingProvider, AwsCredentialProviderChain,
    AwsCredentials, AwsCredentialsProvider, ContainerProvider, CredentialsError,
    EnvironmentProvider, InstanceMetadataProvider, ProfileProvider, ProvideAwsCredentials,
};
use std::sync::Arc;
use std::time::Duration;

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
#[derive(Clone)]
pub struct DefaultCredentialsProvider(Arc<dyn AwsCredentialsProvider>);

impl DefaultCredentialsProvider {
    pub fn new(profile_provider: Option<ProfileProvider>, timeout: Option<Duration>) -> Self {
        let provider = AwsCredentialProviderChain::new(
            Arc::new(EnvironmentProvider::default()),
            Arc::new(WebIdentityProvider::from_k8s_env()),
        );
        let provider = match profile_provider {
            Some(p) => provider.push(p),
            _ => provider,
        };
        let mut container_provider = ContainerProvider::new();
        let mut instance_provider = InstanceMetadataProvider::new();
        timeout.map(|t| {
            container_provider.set_timeout(t.clone());
            instance_provider.set_timeout(t);
        });
        let provider = provider.push(container_provider).push(instance_provider);
        Self(Arc::new(AutoRefreshingProvider::from(provider)))
    }

    /// Creates a potentially insecure default credential provider chain.
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
    /// # Warning
    ///
    /// This provider allows the [`credential_process`][credential_process] option, a method of
    /// sourcing credentials from an external process. This can potentially be dangerous, so proceed
    /// with caution. Other credential providers should be preferred if at all possible. If using this
    /// option, you should make sure that the config file is as locked down as possible using security
    /// best practices for your operating system.
    pub fn insecure() -> Self {
        Self::new(ProfileProvider::new().ok(), None)
    }

    /// Create secure default credential provider chain.
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
    /// This provider **does not** allow the [`credential_process`][credential_process] option
    /// in the AWS config file (`~/.aws/config`) and therefore the returned provider can be
    /// considered secure.
    pub fn secure() -> Self {
        let mut profile = ProfileProvider::new().ok();
        if let Some(p) = &mut profile {
            p.secure();
        }
        Self::new(profile, None)
    }
}

impl Default for DefaultCredentialsProvider {
    /// Create secure default credential provider chain.
    fn default() -> Self {
        Self::secure()
    }
}

#[async_trait]
impl ProvideAwsCredentials for DefaultCredentialsProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        self.0.credentials().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn api() {
        let provider = DefaultCredentialsProvider::default();
        provider.credentials().await;
    }
}
