mod credential;

pub use self::credential::{
    NewAwsCredsForStsCreds, StsAssumeRoleSessionCredentialsProvider, StsSessionCredentialsProvider,
    StsWebIdentityFederationSessionCredentialsProvider,
};
