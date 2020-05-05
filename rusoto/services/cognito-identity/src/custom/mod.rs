
//! The Credentials provider from Cognito.

use rusoto_core::Region;
use async_trait::async_trait;

use rusoto_core::credential::{
    AwsCredentials, 
    CredentialsError,
    ProvideAwsCredentials,
    StaticProvider
};

use crate::generated::{
    GetCredentialsForIdentityInput,
    CognitoIdentityClient,
    CognitoIdentity
};

use chrono::{
    offset::Utc,
    DateTime,
    NaiveDateTime
};

use std::collections::HashMap;

/// Provides AWS credentials from aws Cognito.
///
/// For further information about Cognito Identities check https://docs.aws.amazon.com/cognito/index.html
///
/// # Example
///
/// ```rust
/// use rusoto_core::Region;
/// use rusoto_cognito_identity::CognitoProvider;
/// let provider = CognitoProvider::builder()
///     .identity_id("IDENTITY_POOL_ID".to_string())
///     .region(Region::EuCentral1)
///     .login("graph.facebook.com".to_string(), "FBTOKEN".to_string())
///     .build();
/// 
/// ```

/// <p>The Cognito credential provider.</p>
#[derive(Debug, Clone)]
pub struct CognitoProvider {
    /// <p>A unique identifier in the format REGION:GUID.</p>
    identity_id: String,
    /// <p>The region of the identity pool.</p>
    region: Region,
    /// <p>A set of optional name-value pairs that map provider names to provider tokens. The name-value pair will follow the syntax "provider_name": "provider_user_identifier".</p> <p>Logins should not be specified when trying to get credentials for an unauthenticated identity.</p> <p>The Logins parameter is required when using identities associated with external identity providers such as FaceBook. For examples of <code>Logins</code> maps, see the code examples in the <a href="http://docs.aws.amazon.com/cognito/latest/developerguide/external-identity-providers.html">External Identity Providers</a> section of the Amazon Cognito Developer Guide.</p>
    logins: Option<HashMap<String, String>>,
    /// <p>The Amazon Resource Name (ARN) of the role to be assumed when multiple roles were received in the token from the identity provider. For example, a SAML-based identity provider. This parameter is optional for identity providers that do not support role customization.</p>
    custom_role_arn: Option<String>
}

/// <p>A builder for the Cognito credential provider.</p>
#[derive(Default)]
pub struct CognitoProviderBuilder {
    identity_id: Option<String>,
    region: Option<Region>,
    logins: Option<HashMap<String, String>>,
    custom_role_arn: Option<String>
}

impl CognitoProviderBuilder {
    /// <p>Build the provider.</p>
    pub fn build(self) -> CognitoProvider { 
        CognitoProvider {
            identity_id: self.identity_id.expect("no identity id provided"),
            region: self.region.unwrap_or(Region::default()),
            logins: self.logins,
            custom_role_arn: self.custom_role_arn
        }
    }

    /// <p>Set the identity id.</p>
    pub fn identity_id(mut self, identity_id: String)-> Self {
        self.identity_id = Some(identity_id);
        self
    }

    /// <p>Set the region.</p>
    pub fn region(mut self, region: Region)-> Self {
        self.region = Some(region);
        self
    }

    /// <p>Set the custom role arn.</p>
    pub fn custom_role_arn(mut self, arn: String)-> Self {
        self.custom_role_arn = Some(arn);
        self
    }

    /// <p>Add a pair provider/token.</p>
    pub fn login(mut self, provider: String, token: String)-> Self {
        if self.logins == None {
            self.logins = Some(HashMap::new());
        }
        self.logins.as_mut().unwrap().insert(provider, token);
        self
    }
}

impl CognitoProvider {
    /// Get a builder.
    pub fn builder() -> CognitoProviderBuilder {
        CognitoProviderBuilder::default()
    }
}

#[async_trait]
impl ProvideAwsCredentials for CognitoProvider {

    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let client = CognitoIdentityClient::new_with(
            rusoto_core::request::HttpClient::new().map_err(|e| CredentialsError::new(format!("{:?}", e)))?,
            StaticProvider::from(AwsCredentials::default()),
            self.region.clone()
        );
        let input = GetCredentialsForIdentityInput {
            identity_id: self.identity_id.clone(),
            logins: self.logins.clone(),
            custom_role_arn: self.custom_role_arn.clone(),
            ..Default::default()
        };

        let resp = client.get_credentials_for_identity(input).await.map_err(|e| CredentialsError::new(format!("{:?}", e)))?;
        
        let creds = resp.credentials.ok_or(CredentialsError::new("no credentials were found in the response"))?;
        Ok(
            AwsCredentials::new(
                creds.access_key_id.ok_or(CredentialsError::new("no access key id was found in the response"))?, 
                creds.secret_key.ok_or(CredentialsError::new("no secret key was found in the response"))?, 
                creds.session_token, 
                creds.expiration.map(|x| DateTime::from_utc(NaiveDateTime::from_timestamp(x as i64, 0), Utc)) 
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    #[should_panic(expected = "no identity id provided")]
    fn builder_empty() {
        CognitoProvider::builder().build();
    }

    #[test]
    #[should_panic(expected = "no identity id provided")]
    fn builder_no_identity_id() {
        CognitoProvider::builder()
        .login("provider".to_string(), "token".to_string())
        .build();
    }

    #[test]
    fn builder_simple() {
        let provider = CognitoProvider::builder().identity_id("id_id".to_string()).build();
        assert_eq!(provider.identity_id, "id_id");
        assert_eq!(provider.region, Region::default());
        assert_eq!(provider.logins, None);
        assert_eq!(provider.custom_role_arn, None);
    }

    #[test]
    fn builder_complete() {
        let provider = CognitoProvider::builder()
            .identity_id("id_id".to_string())
            .region(Region::EuCentral1)
            .login("provider".to_string(), "token".to_string())
            .custom_role_arn("arn".to_string())
            .build();
        assert_eq!(provider.identity_id, "id_id");
        assert_eq!(provider.region, Region::EuCentral1);
        assert!(provider.custom_role_arn.is_some());
        assert_eq!(provider.custom_role_arn.unwrap(), "arn");
        assert!(provider.logins.is_some());
        let logins = provider.logins.unwrap();
        assert_eq!(logins.len(), 1);
        assert!(logins.get("provider").is_some());
        assert_eq!(logins.get("provider").unwrap(), "token");
    }

    #[test]
    fn builder_two_providers() {
        let provider = CognitoProvider::builder()
            .identity_id("id_id".to_string())
            .region(Region::EuCentral1)
            .login("provider1".to_string(), "token1".to_string())
            .login("provider2".to_string(), "token2".to_string())
            .build();
        assert_eq!(provider.identity_id, "id_id");
        assert_eq!(provider.region, Region::EuCentral1);
        assert!(provider.logins.is_some());
        let logins = provider.logins.unwrap();
        assert_eq!(logins.len(), 2);
        assert!(logins.get("provider1").is_some());
        assert_eq!(logins.get("provider1").unwrap(), "token1");
        assert!(logins.get("provider2").is_some());
        assert_eq!(logins.get("provider2").unwrap(), "token2");
        assert_eq!(provider.custom_role_arn, None);
    }
}

