//! The AWS STS API.

#![allow(unused_imports)]

include!(concat!(env!("OUT_DIR"), "/sts.rs"));

pub use self::credential::{
    StsSessionCredentialsProvider,
    StsAssumeRoleSessionCredentialsProvider,
    StsWebIdentityFederationSessionCredentialsProvider,
    NewAwsCredsForStsCreds,
    };

mod credential {
    use std::error::Error;
    use std::fmt;

    use hyper::Client;
    use chrono::*;

    use ::{AwsCredentials, ProfileProvider, CredentialsError,
        ProvideAwsCredentials, DispatchSignedRequest};
    use super::{AssumeRoleRequest, AssumeRoleResponse, AssumeRoleError,
        AssumeRoleWithSAMLRequest, AssumeRoleWithSAMLResponse, AssumeRoleWithSAMLError,
        AssumeRoleWithWebIdentityRequest, AssumeRoleWithWebIdentityResponse, AssumeRoleWithWebIdentityError,
        DecodeAuthorizationMessageRequest, DecodeAuthorizationMessageResponse, DecodeAuthorizationMessageError,
        GetCallerIdentityRequest, GetCallerIdentityResponse, GetCallerIdentityError,
        GetFederationTokenRequest, GetFederationTokenResponse, GetFederationTokenError,
        GetSessionTokenRequest, GetSessionTokenResponse, GetSessionTokenError,
        StsClient};

    pub const DEFAULT_DURATION_SECONDS: i32 = 3600;
    pub const DEFAULT_ROLE_DURATION_SECONDS: i32 = 900;

    pub trait NewAwsCredsForStsCreds {
        fn new_for_credentials(sts_creds: ::sts::Credentials) -> Result<AwsCredentials, CredentialsError>;
    }

    impl NewAwsCredsForStsCreds for AwsCredentials {
        fn new_for_credentials(sts_creds: ::sts::Credentials) -> Result<AwsCredentials, CredentialsError> {
            let expires_at = try!(sts_creds.expiration.parse::<DateTime<UTC>>().map_err(CredentialsError::from));

            Ok(AwsCredentials::new(
                sts_creds.access_key_id, 
                sts_creds.secret_access_key, 
                Some(sts_creds.session_token), 
                expires_at))
        }
    }

    pub trait StsSessionCredentialsClient {
        fn assume_role(&self, input: &AssumeRoleRequest) -> Result<AssumeRoleResponse, AssumeRoleError>;

        fn assume_role_with_saml(&self, input: &AssumeRoleWithSAMLRequest) -> Result<AssumeRoleWithSAMLResponse, AssumeRoleWithSAMLError>;

        fn assume_role_with_web_identity(&self, input: &AssumeRoleWithWebIdentityRequest) -> Result<AssumeRoleWithWebIdentityResponse, AssumeRoleWithWebIdentityError>;

        fn decode_authorization_message(&self, input: &DecodeAuthorizationMessageRequest) -> Result<DecodeAuthorizationMessageResponse, DecodeAuthorizationMessageError>;

        fn get_caller_identity(&self, input: &GetCallerIdentityRequest) -> Result<GetCallerIdentityResponse, GetCallerIdentityError> ;

        fn get_federation_token(&self, input: &GetFederationTokenRequest) -> Result<GetFederationTokenResponse, GetFederationTokenError>;

        fn get_session_token(&self, input: &GetSessionTokenRequest) -> Result<GetSessionTokenResponse, GetSessionTokenError>;
    }

    impl <P,D> StsSessionCredentialsClient for StsClient<P,D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
        fn assume_role(&self, input: &AssumeRoleRequest) -> Result<AssumeRoleResponse, AssumeRoleError> {
            StsClient::assume_role(self, input)
        }

        fn assume_role_with_saml(&self, input: &AssumeRoleWithSAMLRequest) -> Result<AssumeRoleWithSAMLResponse, AssumeRoleWithSAMLError> {
            StsClient::assume_role_with_saml(self, input)
        }

        fn assume_role_with_web_identity(&self, input: &AssumeRoleWithWebIdentityRequest) -> Result<AssumeRoleWithWebIdentityResponse, AssumeRoleWithWebIdentityError> {
            StsClient::assume_role_with_web_identity(self, input)
        }

        fn decode_authorization_message(&self, input: &DecodeAuthorizationMessageRequest) -> Result<DecodeAuthorizationMessageResponse, DecodeAuthorizationMessageError> {
            StsClient::decode_authorization_message(self, input)
        }

        fn get_caller_identity(&self, input: &GetCallerIdentityRequest) -> Result<GetCallerIdentityResponse, GetCallerIdentityError> {
            StsClient::get_caller_identity(self, input)
        }

        fn get_federation_token(&self, input: &GetFederationTokenRequest) -> Result<GetFederationTokenResponse, GetFederationTokenError> {
            StsClient::get_federation_token(self, input)
        }

        fn get_session_token(&self, input: &GetSessionTokenRequest) -> Result<GetSessionTokenResponse, GetSessionTokenError> {
            StsClient::get_session_token(self, input)
        }
    }

    pub struct StsSessionCredentialsProvider {
        sts_client: Box<StsSessionCredentialsClient>,
        session_duration: Duration,
        mfa_serial: Option<String>,
        mfa_code: Option<String>,
    }

    impl StsSessionCredentialsProvider {
        pub fn new<P,D>(sts_client: StsClient<P,D>,
                duration: Option<Duration>,
                mfa_serial: Option<String>,
                ) -> StsSessionCredentialsProvider 
                where P: ProvideAwsCredentials + 'static, D: DispatchSignedRequest + 'static {
            StsSessionCredentialsProvider {
                sts_client: Box::new(sts_client),
                session_duration: duration.unwrap_or(Duration::seconds(DEFAULT_DURATION_SECONDS as i64)),
                mfa_serial: mfa_serial,
                mfa_code: None,
            }
        }

        pub fn set_mfa_code<S>(&mut self, code: S) where S: Into<String> {
            self.mfa_code = Some(code.into());
        }

        pub fn clear_mfa_code(&mut self) {
            self.mfa_code = None;
        }

        pub fn get_session_token(&self) -> Result<AwsCredentials, CredentialsError> {
            match self.sts_client.get_session_token(
                &GetSessionTokenRequest {
                    serial_number: self.mfa_serial.clone(),
                    token_code: self.mfa_code.clone(),
                    duration_seconds: Some(self.session_duration.num_seconds() as i32),
                    ..Default::default()
                }) {
                Ok(resp) => {
                    let creds = try!(resp.credentials.ok_or(CredentialsError::new("no credentials in response")));

                    AwsCredentials::new_for_credentials(creds)
                },
                err => 
                    Err(CredentialsError::new(format!("StsProvider get_session_token error: {:?}", err)))
            }
        }
    }

    impl ProvideAwsCredentials for StsSessionCredentialsProvider {
        fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
            self.get_session_token()
        }
    }

    pub struct StsAssumeRoleSessionCredentialsProvider {
        sts_client: Box<StsSessionCredentialsClient>,
        role_arn: String,
        session_name: String,
        external_id: Option<String>,
        session_duration: Duration,
        scope_down_policy: Option<String>,
    }

    impl StsAssumeRoleSessionCredentialsProvider {
        pub fn new<P,D>(sts_client: StsClient<P,D>,
                role_arn: String,
                session_name: String,
                external_id: Option<String>,
                session_duration: Option<Duration>,
                scope_down_policy: Option<String>,
                ) 
                -> StsAssumeRoleSessionCredentialsProvider 
                where P: ProvideAwsCredentials + 'static, D: DispatchSignedRequest + 'static {
            StsAssumeRoleSessionCredentialsProvider {
                sts_client: Box::new(sts_client),
                role_arn: role_arn,
                session_name: session_name,
                external_id: external_id,
                session_duration: session_duration.unwrap_or(Duration::seconds(DEFAULT_ROLE_DURATION_SECONDS as i64)),
                scope_down_policy: scope_down_policy,
            }
        }

        pub fn assume_role(&self) -> Result<AwsCredentials, CredentialsError> {
            match self.sts_client.assume_role(&AssumeRoleRequest {
                role_arn: self.role_arn.clone(),
                role_session_name: self.session_name.clone(),
                duration_seconds: Some(self.session_duration.num_seconds() as i32),
                external_id: self.external_id.clone(),
                policy: self.scope_down_policy.clone(),
                ..Default::default()
            }) {
                Err(err) =>
                    Err(CredentialsError::new(format!("Sts AssumeRoleError: {:?}", err))),
                Ok(resp) => {
                    let creds = try!(resp.credentials.ok_or(CredentialsError::new("no credentials in response")));
                    
                    AwsCredentials::new_for_credentials(creds)
                }
            }
        }
    }

    impl ProvideAwsCredentials for StsAssumeRoleSessionCredentialsProvider {
        fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
            self.assume_role()
        }
    }

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
        pub fn new<P,D>(sts_client: StsClient<P,D>,
                wif_token: String,
                wif_provider: Option<String>,
                role_arn: String,
                session_name: String,
                session_duration: Option<Duration>,
                policy: Option<String>,
                ) -> StsWebIdentityFederationSessionCredentialsProvider
                where P: ProvideAwsCredentials + 'static, D: DispatchSignedRequest + 'static {
            StsWebIdentityFederationSessionCredentialsProvider {
                sts_client: Box::new(sts_client),
                wif_token: wif_token,
                wif_provider: wif_provider,
                role_arn: role_arn,
                session_name: session_name,
                session_duration: session_duration.unwrap_or(Duration::seconds(DEFAULT_DURATION_SECONDS as i64)),
                scope_down_policy: policy,
            }
        }

        pub fn assume_role_with_web_identity(&self) -> Result<AwsCredentials, CredentialsError> {
            match self.sts_client.assume_role_with_web_identity(&AssumeRoleWithWebIdentityRequest {
                web_identity_token: self.wif_token.clone(),
                provider_id: self.wif_provider.clone(),
                role_arn: self.role_arn.clone(),
                role_session_name: self.session_name.clone(),
                duration_seconds: Some(self.session_duration.num_seconds() as i32),
                policy: self.scope_down_policy.clone(),
                ..Default::default()
            }) {
                Err(err) =>
                    Err(CredentialsError::new(format!("Sts AssumeRoleWithWebIdentityError: {:?}", err))),
                Ok(resp) => {
                    let creds = try!(resp.credentials.ok_or(CredentialsError::new("no credentials in response")));

                    let mut aws_creds = try!(AwsCredentials::new_for_credentials(creds));

                    if let Some(subject_from_wif) = resp.subject_from_web_identity_token {
                        aws_creds.claims_mut().insert(::claims::SUBJECT.to_owned(), subject_from_wif);
                    }

                    Ok(aws_creds)
                }
            }
        }
    }

    impl ProvideAwsCredentials for StsWebIdentityFederationSessionCredentialsProvider {
        fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
            self.assume_role_with_web_identity()
        }
    }
}