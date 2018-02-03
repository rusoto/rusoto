//! Credential Claims module.
//!
//! Contains constants used as keys into the [`AwsCredentials`](../struct.AwsCredentials.html) claims map
//! returned by the STS client's federated assume role APIs.

/// Key used in the claims map for the `Subject` claim.
///
/// The `Subject` is the unique user identifier that is returned by the identity provider.
/// For SAML this is the `NameID` element in the `Subject` element of the SAML assertion.
/// For OpenID Connect this field contains the value returned by the identity provider
/// as the token's `sub` claim.
pub const SUBJECT: &str = "sub";

/// Key used in the claims map for the `Audience` claim.
///
/// The intended audience (also known as client ID) of the web identity token.
/// This is traditionally the client identifier issued to the application that requested the web identity token.
/// For OpenID Connect this field contains the value of the `aud` claim.
/// For SAML this is the value of the `Recipient` attribute of the `SubjectConfirmationData`
/// element of the SAML assertion.
pub const AUDIENCE: &str = "aud";

/// Key used in the claims map for the `Issuer` claim.
///
/// For OpenID Connect ID Tokens this contains the value of the `iss` field.
/// For OAuth 2.0 access tokens, this contains the value of the `ProviderId` parameter
/// that was passed in the `AssumeRoleWithWebIdentity` request.
/// For SAML this is the value of the Issuer element of the SAML assertion.
pub const ISSUER: &str = "iss";
