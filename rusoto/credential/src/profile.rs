//! The Credentials Provider for Credentials stored in a profile inside of a Credentials file.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use serde::Deserialize;
use tokio::process::Command;

use crate::{
    config::{default_credentials_location, default_profile_name, ConfigFile, CredentialsFile},
    AwsCredentials, CredentialsError, ProvideAwsCredentials,
};

/// Provides AWS credentials from a profile in a credentials file, or from a credential process.
///
/// # Warning
///
/// This provider allows the [`credential_process`][credential_process] option, a method of
/// sourcing credentials from an external process. This can potentially be dangerous, so proceed
/// with caution. Other credential providers should be preferred if at all possible. If using this
/// option, you should make sure that the config file is as locked down as possible using security
/// best practices for your operating system.
///
/// [credential_process]: https://docs.aws.amazon.com/cli/latest/topic/config-vars.html#sourcing-credentials-from-external-processes
#[derive(Clone, Debug)]
pub struct ProfileProvider {
    /// The File Path the Credentials File is located at.
    file_path: PathBuf,
    /// The Profile Path to parse out of the Credentials File.
    profile: String,
}

impl ProfileProvider {
    /// Create a new `ProfileProvider` for the default credentials file path and profile name.
    pub fn new() -> Result<ProfileProvider, CredentialsError> {
        let profile_location = default_credentials_location()?;
        Ok(ProfileProvider::with_default_configuration(
            profile_location,
        ))
    }

    /// Create a new `ProfileProvider` for the credentials file at the given path, using
    /// the given profile.
    pub fn with_configuration<F, P>(file_path: F, profile: P) -> ProfileProvider
    where
        F: Into<PathBuf>,
        P: Into<String>,
    {
        ProfileProvider {
            file_path: file_path.into(),
            profile: profile.into(),
        }
    }

    /// Create a new `ProfileProvider` for the credentials file at the given path, using
    /// the profile name from environment variable ```AWS_PROFILE``` or fall-back to ```"default"```
    /// if ```AWS_PROFILE``` is not set.
    pub fn with_default_configuration<F>(file_path: F) -> ProfileProvider
    where
        F: Into<PathBuf>,
    {
        ProfileProvider::with_configuration(file_path, default_profile_name())
    }

    /// Create a new `ProfileProvider` for the default credentials file path using
    /// the given profile.
    pub fn with_default_credentials<P>(profile: P) -> Result<ProfileProvider, CredentialsError>
    where
        P: Into<String>,
    {
        let profile_location = default_credentials_location()?;
        Ok(ProfileProvider {
            file_path: profile_location,
            profile: profile.into(),
        })
    }

    /// Attempts to resolve a region value associated with the current default profile from
    /// `~/.aws/config` or the file associated with the `AWS_CONFIG_FILE` environment variable.
    /// As these fields do not require a region field to be defined, an `Option` type is returned
    ///
    /// For a the full region resolution chain, use the `Default` impl for `rusoto_core::Region`
    pub fn region() -> Result<Option<String>, CredentialsError> {
        Ok(ConfigFile::new_default()?
            .default_profile()
            .and_then(|profile| profile.region())
            .map(std::borrow::ToOwned::to_owned))
    }

    /// Attempts to resolve the region value associated with the current `ProfileProvider`s
    /// config file path (`ProfileProvider.file_path`) and profile (`ProfileProvider.profile`).
    /// As these fields do not require a region field to be defined, an `Option` type is returned
    pub fn region_from_profile(&self) -> Result<Option<String>, CredentialsError> {
        Ok(CredentialsFile::new(&self.file_path)?
            .profile(&self.profile)
            .and_then(|profile| profile.region())
            .map(std::borrow::ToOwned::to_owned))
    }

    /// Get a reference to the credentials file path.
    pub fn file_path(&self) -> &Path {
        self.file_path.as_ref()
    }

    /// Get a reference to the profile name.
    pub fn profile(&self) -> &str {
        &self.profile
    }

    /// Set the credentials file path.
    pub fn set_file_path<F>(&mut self, file_path: F)
    where
        F: Into<PathBuf>,
    {
        self.file_path = file_path.into();
    }

    /// Set the profile name.
    pub fn set_profile<P>(&mut self, profile: P)
    where
        P: Into<String>,
    {
        self.profile = profile.into();
    }
}

#[async_trait]
impl ProvideAwsCredentials for ProfileProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        match ConfigFile::new_default()?
            .default_profile()
            .and_then(|profile| profile.credential_process())
            .map(std::borrow::ToOwned::to_owned)
        {
            Some(command) => {
                // credential_process is set, create the future
                let mut command = parse_command_str(&command)?;
                let output = command.output().await.map_err(|e| {
                    CredentialsError::new(format!("Credential process failed: {:?}", e))
                })?;
                if output.status.success() {
                    parse_credential_process_output(&output.stdout)
                } else {
                    Err(CredentialsError::new(format!(
                        "Credential process failed with {}: {}",
                        output.status,
                        String::from_utf8_lossy(&output.stderr)
                    )))
                }
            }
            None => {
                // credential_process is not set, parse the credentials file
                CredentialsFile::new(self.file_path()).and_then(|credentials| {
                    credentials
                        .profile(self.profile())
                        .ok_or_else(|| CredentialsError::new("Profile not found"))
                        .and_then(|profile| {
                            Ok(AwsCredentials::new(
                                profile.aws_access_key_id().ok_or_else(|| {
                                    CredentialsError::new("Profile is missing `aws_access_key_id`")
                                })?,
                                profile.aws_secret_access_key().ok_or_else(|| {
                                    CredentialsError::new(
                                        "Profile is missing `aws_secret_access_key`",
                                    )
                                })?,
                                profile
                                    .aws_session_token()
                                    .or_else(|| profile.aws_security_token())
                                    .map(std::borrow::ToOwned::to_owned),
                                None,
                            ))
                        })
                })
            }
        }
    }
}

#[derive(Deserialize)]
struct CredentialProcessOutput {
    #[serde(flatten)]
    creds: AwsCredentials,
    #[serde(rename = "Version")]
    version: u8,
}

fn parse_credential_process_output(v: &[u8]) -> Result<AwsCredentials, CredentialsError> {
    let output: CredentialProcessOutput = serde_json::from_slice(v)?;
    if output.version == 1 {
        Ok(output.creds)
    } else {
        Err(CredentialsError::new(format!(
            "Unsupported version '{}' for credential process provider, supported versions: 1",
            output.version
        )))
    }
}

fn parse_command_str(s: &str) -> Result<Command, CredentialsError> {
    let args = shlex::split(s)
        .ok_or_else(|| CredentialsError::new("Unable to parse credential_process value."))?;
    let mut iter = args.iter();
    let mut command = Command::new(
        iter.next()
            .ok_or_else(|| CredentialsError::new("credential_process value is empty."))?,
    );
    command.args(iter);
    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{
            default_profile_name,
            test_utils::{
                with_env_config_file, with_env_profile, with_env_shared_credentials_file,
            },
        },
        test_utils::lock_env,
        CredentialsError, ProvideAwsCredentials,
    };

    #[tokio::test]
    async fn profile_provider_happy_path() {
        let _guard = lock_env();
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foo",
        );
        let result = provider.credentials().await;

        assert!(result.is_ok());

        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "foo_access_key");
        assert_eq!(creds.aws_secret_access_key(), "foo_secret_key");
    }

    #[test]
    fn profile_provider_via_environment_variable() {
        let _guard = lock_env();
        let credentials_path = "tests/sample-data/default_profile_credentials";
        let _credentials_guard = with_env_shared_credentials_file(Some(credentials_path));
        let result = ProfileProvider::new();
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.file_path().to_str().unwrap(), credentials_path);
    }

    #[tokio::test]
    async fn profile_provider_profile_name_via_environment_variable() {
        let _guard = lock_env();
        let credentials_path = "tests/sample-data/multiple_profile_credentials";
        let _credentials_guard = with_env_shared_credentials_file(Some(credentials_path));
        let _profile_guard = with_env_profile(Some("bar"));
        let result = ProfileProvider::new();
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.file_path().to_str().unwrap(), credentials_path);
        let creds = provider.credentials().await;
        assert_eq!(creds.unwrap().aws_access_key_id(), "bar_access_key");
    }

    #[tokio::test]
    async fn profile_provider_bad_profile() {
        let _guard = lock_env();
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "not_a_profile",
        );
        let result = provider.credentials().await;

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(CredentialsError::new("Profile not found"))
        );
    }

    #[tokio::test]
    async fn profile_provider_credential_process() {
        let _guard = lock_env();
        let _config_guard =
            with_env_config_file(Some("tests/sample-data/credential_process_config"));
        let provider = ProfileProvider::new().unwrap();
        let result = provider.credentials().await;

        assert!(result.is_ok());

        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "baz_access_key");
        assert_eq!(creds.aws_secret_access_key(), "baz_secret_key");
        assert_eq!(
            creds.token().as_ref().expect("session token not parsed"),
            "baz_session_token"
        );
        assert!(creds.expires_at().is_some());
    }

    #[test]
    fn profile_provider_profile_name() {
        let _guard = lock_env();
        let mut provider = ProfileProvider::new().unwrap();
        assert_eq!(default_profile_name(), provider.profile());
        provider.set_profile("foo");
        assert_eq!("foo", provider.profile());
    }

    #[test]
    fn region_from_profile() {
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foo",
        );
        let maybe_region = provider.region_from_profile().unwrap();

        assert_eq!(maybe_region, Some("us-east-3".to_string()));
    }

    #[test]
    fn region_from_profile_missing_profile() {
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foobar",
        );
        let maybe_region = provider.region_from_profile().unwrap();

        assert_eq!(maybe_region, None);
    }
}
