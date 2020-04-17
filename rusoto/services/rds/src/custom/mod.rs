use std::time::Duration;

use rusoto_core::region::Region;
use rusoto_credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_signature::signature::SignedRequest;

// TODO: should these be constants, perhaps they'd just be better inline?
const RDS_GENERATE_DB_AUTH_TOKEN_ACTION: &'static str = "connect";
const RDS_GENERATE_DB_AUTH_TOKEN_METHOD: &'static str = "GET";
const RDS_GENERATE_DB_AUTH_TOKEN_PATH: &'static str = "/";
const RDS_GENERATE_DB_AUTH_TOKEN_SERVICE: &'static str = "rds-db";

pub struct GenerateDbAuthTokenAttributes {
    pub region: Region,
    pub hostname: String,
    pub port: u16,
    pub username: String,
}

async fn generate_db_auth_token<P>(provider: P, attributes: GenerateDbAuthTokenAttributes) -> Result<String, CredentialsError>
where
    P: ProvideAwsCredentials + Send + Sync + 'static
{
    // TODO: should this use also have a configurable timeout. See: client.rs#sign_and_dispatch
    let credentials = provider.credentials().await?;

    let mut request = SignedRequest::new(RDS_GENERATE_DB_AUTH_TOKEN_METHOD, RDS_GENERATE_DB_AUTH_TOKEN_SERVICE, &attributes.region, RDS_GENERATE_DB_AUTH_TOKEN_PATH);

    let hostname = format!("{}:{}", attributes.hostname, attributes.port);
    request.set_hostname(Some(hostname));

    request.add_param("Action", RDS_GENERATE_DB_AUTH_TOKEN_ACTION);
    request.add_param("DBUser", &attributes.username);

    // TODO: should this expiration time be configurable
    let expires_in = Duration::from_secs(15 * 60);

    let signature = request.generate_presigned_url(&credentials, &expires_in, true);

    // remove the leading scheme
    // TODO: is there a more idiomatic way to remove the prefix?
    //       should we parse as a URL an then format it?
    //       `strip_prefix` is closer in behaviour to what we want, but is a
    //       nightly feature.
    let signature = signature.trim_start_matches("https://");

    Ok(signature.to_owned())
}

#[cfg(test)]
mod test {
    // TODO: how should we test this?
    // With a way to inject the current time, and static credentials this could
    // be deterministic.
}
