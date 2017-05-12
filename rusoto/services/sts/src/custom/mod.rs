mod credential;

pub use self::credential::{
    StsSessionCredentialsProvider,
    StsAssumeRoleSessionCredentialsProvider,
    StsWebIdentityFederationSessionCredentialsProvider,
    NewAwsCredsForStsCreds,
};