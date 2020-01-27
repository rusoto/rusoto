mod credential;

pub use self::credential::{
    NewAwsCredsForStsCreds, StsAssumeRoleSessionCredentialsProvider, StsSessionCredentialsProvider,
    StsWebIdentityFederationSessionCredentialsProvider,
};

//mod default_provider;
mod web_identity;
//pub use self::default_provider::*;
pub use self::web_identity::*;
