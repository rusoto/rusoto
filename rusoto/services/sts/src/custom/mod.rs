mod credential;

pub use self::credential::{
    NewAwsCredsForStsCreds, StsAssumeRoleSessionCredentialsProvider, StsSessionCredentialsProvider,
    StsWebIdentityFederationSessionCredentialsProvider,
};

mod web_identity;
pub use self::web_identity::*;
