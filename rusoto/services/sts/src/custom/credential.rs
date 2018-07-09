use chrono::prelude::*;
use chrono::Duration;
use futures::{Async, Future, Poll};

use rusoto_core;

use rusoto_core::{CredentialsError, ProvideAwsCredentials, RusotoFuture};
use rusoto_core::credential::AwsCredentials;
use ::{AssumeRoleRequest, AssumeRoleResponse, AssumeRoleError,
    AssumeRoleWithSAMLRequest, AssumeRoleWithSAMLResponse, AssumeRoleWithSAMLError,
    AssumeRoleWithWebIdentityRequest, AssumeRoleWithWebIdentityResponse, AssumeRoleWithWebIdentityError,
    DecodeAuthorizationMessageRequest, DecodeAuthorizationMessageResponse, DecodeAuthorizationMessageError,
    GetCallerIdentityRequest, GetCallerIdentityResponse, GetCallerIdentityError,
    GetFederationTokenRequest, GetFederationTokenResponse, GetFederationTokenError,
    GetSessionTokenRequest, GetSessionTokenResponse, GetSessionTokenError, Sts,
    StsClient};
use rusoto_core::reactor::{CredentialsProvider, CredentialsProviderFuture};
use rusoto_core::credential::{ChainProvider, ChainProviderFuture,
                              EnvironmentProvider, EnvironmentProviderFuture,
                              InstanceMetadataProvider, InstanceMetadataProviderFuture,
                              ProfileProvider, ProfileProviderFuture,
                              ContainerProvider, ContainerProviderFuture};

pub const DEFAULT_DURATION_SECONDS: i32 = 3600;
pub const DEFAULT_ROLE_DURATION_SECONDS: i32 = 900;

/// Trait for conversions from STS Credentials to AWS Credentials.
pub trait NewAwsCredsForStsCreds {
    /// Creates an [AwsCredentials](../rusoto_credential/struct.AwsCredentials.html) from a [Credentials](struct.Credentials.html)
    /// Returns a [CredentialsError](../rusoto_credential/struct.CredentialsError.html) in case of an error.
    fn new_for_credentials(sts_creds: ::Credentials) -> Result<AwsCredentials, CredentialsError>;
}

impl NewAwsCredsForStsCreds for AwsCredentials {
    fn new_for_credentials(sts_creds: ::Credentials) -> Result<AwsCredentials, CredentialsError> {
        let expires_at = Some(try!(sts_creds.expiration.parse::<DateTime<Utc>>().map_err(CredentialsError::from)));

        Ok(AwsCredentials::new(
            sts_creds.access_key_id, 
            sts_creds.secret_access_key, 
            Some(sts_creds.session_token), 
            expires_at))
    }
}

// Trait that defines the STS Client API without any type parameters or assumptions about implementation.
// This is an internal type used to box the [StsClient](struct.StsClient.html) provided in the session token providers' constructors.
pub trait StsSessionCredentialsClient {
    fn assume_role(&self, input: AssumeRoleRequest) -> RusotoFuture<AssumeRoleResponse, AssumeRoleError>;

    fn assume_role_with_saml(&self, input: AssumeRoleWithSAMLRequest) -> RusotoFuture<AssumeRoleWithSAMLResponse, AssumeRoleWithSAMLError>;

    fn assume_role_with_web_identity(&self, input: AssumeRoleWithWebIdentityRequest) -> RusotoFuture<AssumeRoleWithWebIdentityResponse, AssumeRoleWithWebIdentityError>;

    fn decode_authorization_message(&self, input: DecodeAuthorizationMessageRequest) -> RusotoFuture<DecodeAuthorizationMessageResponse, DecodeAuthorizationMessageError>;

    fn get_caller_identity(&self, input: GetCallerIdentityRequest) -> RusotoFuture<GetCallerIdentityResponse, GetCallerIdentityError> ;

    fn get_federation_token(&self, input: GetFederationTokenRequest) -> RusotoFuture<GetFederationTokenResponse, GetFederationTokenError>;

    fn get_session_token(&self, input: GetSessionTokenRequest) -> RusotoFuture<GetSessionTokenResponse, GetSessionTokenError>;
}

impl<T> StsSessionCredentialsClient for T where T: Sts {
    fn assume_role(&self, input: AssumeRoleRequest) -> RusotoFuture<AssumeRoleResponse, AssumeRoleError> {
        T::assume_role(self, input)
    }

    fn assume_role_with_saml(&self, input: AssumeRoleWithSAMLRequest) -> RusotoFuture<AssumeRoleWithSAMLResponse, AssumeRoleWithSAMLError> {
        T::assume_role_with_saml(self, input)
    }

    fn assume_role_with_web_identity(&self, input: AssumeRoleWithWebIdentityRequest) -> RusotoFuture<AssumeRoleWithWebIdentityResponse, AssumeRoleWithWebIdentityError> {
        T::assume_role_with_web_identity(self, input)
    }

    fn decode_authorization_message(&self, input: DecodeAuthorizationMessageRequest) -> RusotoFuture<DecodeAuthorizationMessageResponse, DecodeAuthorizationMessageError> {
        T::decode_authorization_message(self, input)
    }

    fn get_caller_identity(&self, input: GetCallerIdentityRequest) -> RusotoFuture<GetCallerIdentityResponse, GetCallerIdentityError> {
        T::get_caller_identity(self, input)
    }

    fn get_federation_token(&self, input: GetFederationTokenRequest) -> RusotoFuture<GetFederationTokenResponse, GetFederationTokenError> {
        T::get_federation_token(self, input)
    }

    fn get_session_token(&self, input: GetSessionTokenRequest) -> RusotoFuture<GetSessionTokenResponse, GetSessionTokenError> {
        T::get_session_token(self, input)
    }
}

/// [AwsCredentials](../rusoto_credential/struct.AwsCredentials.html) provider that calls
/// `GetSessionToken` using the provided [StsClient](struct.StsClient.html).
/// To use with MFA, pass in the MFA serial number then set the MFA code.
/// You will need to ensure the provider has a valid code each time you
/// acquire a new STS token.
pub struct StsSessionCredentialsProvider {
    sts_client: Box<StsSessionCredentialsClient>,
    session_duration: Duration,
    mfa_serial: Option<String>,
    mfa_code: Option<String>,
}

impl StsSessionCredentialsProvider {
    /// Creates a new `StsSessionCredentialsProvider` with the given
    /// [StsClient](struct.StsClient.html) and session parameters.
    ///
    /// * `sts_client` - The [StsClient](struct.StsClient.html) to use to acquire session tokens.
    /// * `duration` - The duration of the session tokens. Default 1 hour.
    /// * `mfa_serial` - Optional MFA hardware device serial number or virtual device ARN. Set the MFA code with `set_mfa_code`.
    pub fn new(sts_client: StsClient,
            duration: Option<Duration>,
            mfa_serial: Option<String>,
            ) -> StsSessionCredentialsProvider 
    {
        StsSessionCredentialsProvider {
            sts_client: Box::new(sts_client),
            session_duration: duration.unwrap_or(Duration::seconds(DEFAULT_DURATION_SECONDS as i64)),
            mfa_serial: mfa_serial,
            mfa_code: None,
        }
    }

    /// Set the MFA code for use when acquiring session tokens.
    pub fn set_mfa_code<S>(&mut self, code: S) where S: Into<String> {
        self.mfa_code = Some(code.into());
    }

    /// Clear the MFA code.
    pub fn clear_mfa_code(&mut self) {
        self.mfa_code = None;
    }

    /// Calls `GetSessionToken` to get a session token from the STS Api.
    /// Optionally uses MFA if the MFA serial number and code are set.
    pub fn get_session_token(&self) -> StsSessionCredentialsProviderFuture {
        let request = GetSessionTokenRequest {
            serial_number: self.mfa_serial.clone(),
            token_code: self.mfa_code.clone(),
            duration_seconds: Some(self.session_duration.num_seconds() as i64),
            ..Default::default()
        };
        StsSessionCredentialsProviderFuture {
            inner: self.sts_client.get_session_token(request)
        }
    }
}

pub struct StsSessionCredentialsProviderFuture {
    inner: RusotoFuture<GetSessionTokenResponse, GetSessionTokenError>
}

impl Future for StsSessionCredentialsProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner.poll() {
            Ok(Async::Ready(resp)) => {
                let creds = try!(resp.credentials.ok_or(CredentialsError::new("no credentials in response")));

                Ok(Async::Ready(try!(AwsCredentials::new_for_credentials(creds))))
            },
            Ok(Async::NotReady) =>
                Ok(Async::NotReady),
            Err(err) =>
                Err(CredentialsError::new(format!("StsProvider get_session_token error: {:?}", err)))
        }
    }
}

impl ProvideAwsCredentials for StsSessionCredentialsProvider {
    type Future = StsSessionCredentialsProviderFuture;

    fn credentials(&self) -> Self::Future {
        self.get_session_token()
    }
}

/// [AwsCredentials](../rusoto_credential/struct.AwsCredentials.html) provider that calls
/// `AssumeRole` using the provided [StsClient](struct.StsClient.html).
/// To use with MFA, pass in the MFA serial number then set the MFA code.
/// You will need to ensure the provider has a valid code each time you
/// acquire a new STS token.
pub struct StsAssumeRoleSessionCredentialsProvider {
    sts_client: Box<StsSessionCredentialsClient>,
    role_arn: String,
    session_name: String,
    external_id: Option<String>,
    session_duration: Duration,
    scope_down_policy: Option<String>,
    mfa_serial: Option<String>,
    mfa_code: Option<String>,
}

impl StsAssumeRoleSessionCredentialsProvider {
    /// Creates a new `StsAssumeRoleSessionCredentialsProvider` with the given
    /// [StsClient](struct.StsClient.html) and session parameters.
    ///
    /// * `sts_client` - [StsClient](struct.StsClient.html) to use to acquire session tokens.
    /// * `role_arn` - The ARN of the role to assume.
    /// * `session_name` - An identifier for the assumed role session. Minimum length of 2. Maximum length of 64. Pattern: `[\w+=,.@-]*`
    /// * `external_id` - 
    /// * `session_duration` - Duration of session tokens. Default 1 hour.
    /// * `scope_down_policy` - Optional inline IAM policy in JSON format to further restrict the access granted to the negotiated session.
    /// * `mfa_serial` - Optional MFA hardware device serial number or virtual device ARN. Use `set_mfa_code` to set the MFA code.
    pub fn new(sts_client: StsClient,
            role_arn: String,
            session_name: String,
            external_id: Option<String>,
            session_duration: Option<Duration>,
            scope_down_policy: Option<String>,
            mfa_serial: Option<String>
            ) 
            -> StsAssumeRoleSessionCredentialsProvider 
    {
        StsAssumeRoleSessionCredentialsProvider {
            sts_client: Box::new(sts_client),
            role_arn: role_arn,
            session_name: session_name,
            external_id: external_id,
            session_duration: session_duration.unwrap_or(Duration::seconds(DEFAULT_ROLE_DURATION_SECONDS as i64)),
            scope_down_policy: scope_down_policy,
            mfa_serial: mfa_serial,
            mfa_code: None,
        }
    }

    /// Set the MFA code for use when acquiring session tokens.
    pub fn set_mfa_code<S>(&mut self, code: S) where S: Into<String> {
        self.mfa_code = Some(code.into());
    }

    /// Clear the MFA code.
    pub fn clear_mfa_code(&mut self) {
        self.mfa_code = None;
    }

    /// Calls `AssumeRole` to get a session token from the STS Api.
    /// Optionally uses MFA if the MFA serial number and code are set.
    pub fn assume_role(&self) -> StsAssumeRoleSessionCredentialsProviderFuture {
        let request = AssumeRoleRequest {
            role_arn: self.role_arn.clone(),
            role_session_name: self.session_name.clone(),
            duration_seconds: Some(self.session_duration.num_seconds() as i64),
            external_id: self.external_id.clone(),
            policy: self.scope_down_policy.clone(),
            serial_number: self.mfa_serial.clone(),
            token_code: self.mfa_code.clone(),
            ..Default::default()
        };
        StsAssumeRoleSessionCredentialsProviderFuture {
            inner: self.sts_client.assume_role(request)
        }
    }
}

pub struct StsAssumeRoleSessionCredentialsProviderFuture {
    inner: RusotoFuture<AssumeRoleResponse, AssumeRoleError>
}

impl Future for StsAssumeRoleSessionCredentialsProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner.poll() {
            Ok(Async::Ready(resp)) => {
                let creds = try!(resp.credentials.ok_or(CredentialsError::new("no credentials in response")));
                
                Ok(Async::Ready(try!(AwsCredentials::new_for_credentials(creds))))
            },
            Ok(Async::NotReady) =>
                Ok(Async::NotReady),
            Err(err) =>
                Err(CredentialsError::new(format!("Sts AssumeRoleError: {:?}", err)))
        }
    }
}

impl ProvideAwsCredentials for StsAssumeRoleSessionCredentialsProvider {
    type Future = StsAssumeRoleSessionCredentialsProviderFuture;

    fn credentials(&self) -> Self::Future {
        self.assume_role()
    }
}

/// [AwsCredentials](../rusoto_credential/struct.AwsCredentials.html) provider that calls
/// `AssumeRoleWithWebIdentity` using the provided [StsClient](struct.StsClient.html).
pub struct StsWebIdentityFederationSessionCredentialsProvider {
    sts_client: Box<StsSessionCredentialsClient>,
    wif_token: String,
    wif_provider: Option<String>,
    role_arn: String,
    session_name: String,
    session_duration: Duration,
    scope_down_policy: Option<String>,
}

impl StsWebIdentityFederationSessionCredentialsProvider {
    /// Creates a new `StsWebIdentityFederationSessionCredentialsProvider` with the given
    /// [StsClient](struct.StsClient.html) and session parameters.
    ///
    /// * `sts_client` - The [StsClient](struct.StsClient.html) to use to acquire session tokens.
    /// * `wif_token` - The OAuth 2.0 access token or OpenID Connect ID token that is provided by the identity provider.
    /// * `wif_provider` - The fully qualified host component of the domain name of the identity provider. Only for OAuth 2.0 access tokens. Do not include URL schemes and port numbers.
    /// * `role_arn` - The ARN of the role to assume.
    /// * `session_name` - An identifier for the assumed role session. Minimum length of 2. Maximum length of 64. Pattern: `[\w+=,.@-]*`
    /// * `session_duration` - Duration of session tokens. Default 1 hour.
    /// * `scope_down_policy` - Optional inline IAM policy in JSON format to further restrict the access granted to the negotiated session.
    pub fn new(sts_client: StsClient,
            wif_token: String,
            wif_provider: Option<String>,
            role_arn: String,
            session_name: String,
            session_duration: Option<Duration>,
            scope_down_policy: Option<String>
            ) -> StsWebIdentityFederationSessionCredentialsProvider
        {
        StsWebIdentityFederationSessionCredentialsProvider {
            sts_client: Box::new(sts_client),
            wif_token: wif_token,
            wif_provider: wif_provider,
            role_arn: role_arn,
            session_name: session_name,
            session_duration: session_duration.unwrap_or(Duration::seconds(DEFAULT_DURATION_SECONDS as i64)),
            scope_down_policy: scope_down_policy
        }
    }

    /// Calls `AssumeRoleWithWebIdentity` to get a session token from the STS Api.
    pub fn assume_role_with_web_identity(&self) -> StsWebIdentityFederationSessionCredentialsProviderFuture {
        let request = AssumeRoleWithWebIdentityRequest {
            web_identity_token: self.wif_token.clone(),
            provider_id: self.wif_provider.clone(),
            role_arn: self.role_arn.clone(),
            role_session_name: self.session_name.clone(),
            duration_seconds: Some(self.session_duration.num_seconds() as i64),
            policy: self.scope_down_policy.clone(),
            ..Default::default()
        };
        StsWebIdentityFederationSessionCredentialsProviderFuture {
            inner: self.sts_client.assume_role_with_web_identity(request)
        }
    }
}

pub struct StsWebIdentityFederationSessionCredentialsProviderFuture {
    inner: RusotoFuture<AssumeRoleWithWebIdentityResponse, AssumeRoleWithWebIdentityError>
}

impl Future for StsWebIdentityFederationSessionCredentialsProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner.poll() {
            Ok(Async::Ready(resp)) => {
                let creds = try!(resp.credentials.ok_or(CredentialsError::new("no credentials in response")));

                let mut aws_creds = try!(AwsCredentials::new_for_credentials(creds));

                if let Some(subject_from_wif) = resp.subject_from_web_identity_token {
                    aws_creds.claims_mut().insert(rusoto_core::credential::claims::SUBJECT.to_owned(), subject_from_wif);
                }

                if let Some(audience) = resp.audience {
                    aws_creds.claims_mut().insert(rusoto_core::credential::claims::AUDIENCE.to_owned(), audience);
                }

                if let Some(issuer) = resp.provider {
                    aws_creds.claims_mut().insert(rusoto_core::credential::claims::ISSUER.to_owned(), issuer);
                }

                Ok(Async::Ready(aws_creds))
            },
            Ok(Async::NotReady) =>
                Ok(Async::NotReady),
            Err(err) =>
                Err(CredentialsError::new(format!("Sts AssumeRoleWithWebIdentityError: {:?}", err))),
        }
    }
}

impl ProvideAwsCredentials for StsWebIdentityFederationSessionCredentialsProvider {
    type Future = StsWebIdentityFederationSessionCredentialsProviderFuture;

    fn credentials(&self) -> Self::Future {
        self.assume_role_with_web_identity()
    }
}

/// A wrapper that can hold arbitrary credential providers
pub enum GenericCredentialProvider {
    Chain(ChainProvider),
    Environment(EnvironmentProvider),
    InstanceMetadata(InstanceMetadataProvider),
    Profile(ProfileProvider),
    Container(ContainerProvider),
    Credentials(CredentialsProvider),
    StsSessionCredentials(StsSessionCredentialsProvider),
    StsAssumeRoleSessionCredentials(StsAssumeRoleSessionCredentialsProvider),
    StsWebIdentityFederationSessionCredentials(StsWebIdentityFederationSessionCredentialsProvider),
}

impl From<ChainProvider> for GenericCredentialProvider {
    fn from(provider: ChainProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::Chain(provider)
    }
}

impl From<EnvironmentProvider> for GenericCredentialProvider {
    fn from(provider: EnvironmentProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::Environment(provider)
    }
}

impl From<InstanceMetadataProvider> for GenericCredentialProvider {
    fn from(provider: InstanceMetadataProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::InstanceMetadata(provider)
    }
}

impl From<ProfileProvider> for GenericCredentialProvider {
    fn from(provider: ProfileProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::Profile(provider)
    }
}

impl From<ContainerProvider> for GenericCredentialProvider {
    fn from(provider: ContainerProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::Container(provider)
    }
}

impl From<CredentialsProvider> for GenericCredentialProvider {
    fn from(provider: CredentialsProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::Credentials(provider)
    }
}

impl From<StsSessionCredentialsProvider> for GenericCredentialProvider {
    fn from(provider: StsSessionCredentialsProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::StsSessionCredentials(provider)
    }
}

impl From<StsAssumeRoleSessionCredentialsProvider> for GenericCredentialProvider {
    fn from(provider: StsAssumeRoleSessionCredentialsProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::StsAssumeRoleSessionCredentials(provider)
    }
}

impl From<StsWebIdentityFederationSessionCredentialsProvider> for GenericCredentialProvider {
    fn from(provider: StsWebIdentityFederationSessionCredentialsProvider) -> GenericCredentialProvider {
        GenericCredentialProvider::StsWebIdentityFederationSessionCredentials(provider)
    }
}

impl ProvideAwsCredentials for GenericCredentialProvider {
    type Future = GenericCredentialProviderFuture;

    fn credentials(&self) -> Self::Future {
        use self::GenericCredentialProvider::*;
        match *self {
            Chain(ref p) => p.credentials().into(),
            Environment(ref p) => p.credentials().into(),
            InstanceMetadata(ref p) => p.credentials().into(),
            Profile(ref p) => p.credentials().into(),
            Container(ref p) => p.credentials().into(),
            Credentials(ref p) => p.credentials().into(),
            StsSessionCredentials(ref p) => p.credentials().into(),
            StsAssumeRoleSessionCredentials(ref p) => p.credentials().into(),
            StsWebIdentityFederationSessionCredentials(ref p) => p.credentials().into(),
        }
    }
}

pub enum GenericCredentialProviderFuture {
    Chain(ChainProviderFuture),
    Environment(EnvironmentProviderFuture),
    InstanceMetadata(InstanceMetadataProviderFuture),
    Profile(ProfileProviderFuture),
    Container(ContainerProviderFuture),
    Credentials(CredentialsProviderFuture),
    StsSessionCredentials(StsSessionCredentialsProviderFuture),
    StsAssumeRoleSessionCredentials(StsAssumeRoleSessionCredentialsProviderFuture),
    StsWebIdentityFederationSessionCredentials(StsWebIdentityFederationSessionCredentialsProviderFuture),
}

impl Future for GenericCredentialProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::GenericCredentialProviderFuture::*;
        match *self {
            Chain(ref mut f) => f.poll(),
            Environment(ref mut f) => f.poll(),
            InstanceMetadata(ref mut f) => f.poll(),
            Profile(ref mut f) => f.poll(),
            Container(ref mut f) => f.poll(),
            Credentials(ref mut f) => f.poll(),
            StsSessionCredentials(ref mut f) => f.poll(),
            StsAssumeRoleSessionCredentials(ref mut f) => f.poll(),
            StsWebIdentityFederationSessionCredentials(ref mut f) => f.poll(),
        }
    }
}

impl From<ChainProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: ChainProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::Chain(future)
    }
}

impl From<EnvironmentProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: EnvironmentProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::Environment(future)
    }
}

impl From<InstanceMetadataProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: InstanceMetadataProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::InstanceMetadata(future)
    }
}

impl From<ProfileProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: ProfileProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::Profile(future)
    }
}

impl From<ContainerProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: ContainerProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::Container(future)
    }
}

impl From<CredentialsProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: CredentialsProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::Credentials(future)
    }
}

impl From<StsSessionCredentialsProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: StsSessionCredentialsProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::StsSessionCredentials(future)
    }
}

impl From<StsAssumeRoleSessionCredentialsProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: StsAssumeRoleSessionCredentialsProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::StsAssumeRoleSessionCredentials(future)
    }
}

impl From<StsWebIdentityFederationSessionCredentialsProviderFuture> for GenericCredentialProviderFuture {
    fn from(future: StsWebIdentityFederationSessionCredentialsProviderFuture) -> GenericCredentialProviderFuture {
        GenericCredentialProviderFuture::StsWebIdentityFederationSessionCredentials(future)
    }
}
