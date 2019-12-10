//! The Credentials Provider for Credentials stored in a profile inside of a Credentials file.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use dirs::home_dir;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use tokio::process::Command;

use crate::{non_empty_env_var, AwsCredentials, CredentialsError, ProvideAwsCredentials};

const AWS_CONFIG_FILE: &str = "AWS_CONFIG_FILE";
const AWS_PROFILE: &str = "AWS_PROFILE";
const AWS_SHARED_CREDENTIALS_FILE: &str = "AWS_SHARED_CREDENTIALS_FILE";
const DEFAULT: &str = "default";
const REGION: &str = "region";

lazy_static! {
    static ref PROFILE_REGEX: Regex =
        Regex::new(r"^\[(profile )?([^\]]+)\]$").expect("Failed to compile regex");
}

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
    /// If `true` disable [`credential_process`][credential_process] option, making sure not to
    /// call any external process.
    ///
    /// [credential_process]: https://docs.aws.amazon.com/cli/latest/topic/config-vars.html#sourcing-credentials-from-external-processes
    secure: bool,
}

impl ProfileProvider {
    /// Create a new `ProfileProvider` for the default credentials file path and profile name.
    pub fn new() -> Result<ProfileProvider, CredentialsError> {
        let profile_location = ProfileProvider::default_profile_location()?;
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
            secure: false,
        }
    }

    /// Create a new `ProfileProvider` for the credentials file at the given path, using
    /// the profile name from environment variable ```AWS_PROFILE``` or fall-back to ```"default"```
    /// if ```AWS_PROFILE``` is not set.
    pub fn with_default_configuration<F>(file_path: F) -> ProfileProvider
    where
        F: Into<PathBuf>,
    {
        ProfileProvider::with_configuration(file_path, ProfileProvider::default_profile_name())
    }

    /// Attempts to resolve a region value associated with the current profile from
    /// `~/.aws/config` or the file associated with the `AWS_CONFIG_FILE` environment variable.
    /// As these fields do not require a region field to be defined, an `Option` type is returned
    ///
    /// For a the ful region resolution chain, use the `Default` impl for `rusoto_core::Region`
    pub fn region() -> Result<Option<String>, CredentialsError> {
        let location = ProfileProvider::default_config_location();
        location.map(|location| {
            parse_config_file(&location).and_then(|config| {
                config
                    .get(&ProfileProvider::default_profile_name())
                    .and_then(|props| props.get(REGION))
                    .map(std::borrow::ToOwned::to_owned)
            })
        })
    }

    /// Default config file location:
    /// 1: if set and not empty, use the value from environment variable ```AWS_CONFIG_FILE```
    /// 2. otherwise return `~/.aws/config` (Linux/Mac) resp. `%USERPROFILE%\.aws\config` (Windows)
    fn default_config_location() -> Result<PathBuf, CredentialsError> {
        let env = non_empty_env_var(AWS_CONFIG_FILE);
        match env {
            Some(path) => Ok(PathBuf::from(path)),
            None => ProfileProvider::hardcoded_config_location(),
        }
    }

    fn hardcoded_config_location() -> Result<PathBuf, CredentialsError> {
        match home_dir() {
            Some(mut home_path) => {
                home_path.push(".aws");
                home_path.push("config");
                Ok(home_path)
            }
            None => Err(CredentialsError::new("Failed to determine home directory.")),
        }
    }

    /// Default credentials file location:
    /// 1. if set and not empty, use value from environment variable ```AWS_SHARED_CREDENTIALS_FILE```
    /// 2. otherwise return `~/.aws/credentials` (Linux/Mac) resp. `%USERPROFILE%\.aws\credentials` (Windows)
    fn default_profile_location() -> Result<PathBuf, CredentialsError> {
        let env = non_empty_env_var(AWS_SHARED_CREDENTIALS_FILE);
        match env {
            Some(path) => Ok(PathBuf::from(path)),
            None => ProfileProvider::hardcoded_profile_location(),
        }
    }

    fn hardcoded_profile_location() -> Result<PathBuf, CredentialsError> {
        match home_dir() {
            Some(mut home_path) => {
                home_path.push(".aws");
                home_path.push("credentials");
                Ok(home_path)
            }
            None => Err(CredentialsError::new("Failed to determine home directory.")),
        }
    }

    /// Get the default profile name:
    /// 1. if set and not empty, use value from environment variable ```AWS_PROFILE```
    /// 2. otherwise return ```"default"```
    /// see https://docs.aws.amazon.com/sdk-for-java/v1/developer-guide/credentials.html.
    fn default_profile_name() -> String {
        non_empty_env_var(AWS_PROFILE).unwrap_or_else(|| DEFAULT.to_owned())
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

    /// Returns `true` if this provider instance will not execute any external processes to obtain
    /// credentials.
    pub fn is_secure(&self) -> bool {
        self.secure
    }

    /// Ensure this provider instance does not call any external processes to obtain credentials.
    pub fn secure(&mut self) {
        self.secure = true;
    }
}

#[async_trait]
impl ProvideAwsCredentials for ProfileProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        match ProfileProvider::default_config_location().map(|location| {
            parse_config_file(&location).and_then(|config| {
                config
                    .get(&self.profile)
                    .and_then(|props| props.get("credential_process"))
                    .map(std::borrow::ToOwned::to_owned)
            })
        }) {
            Ok(Some(ref command)) if !self.is_secure() => {
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
            Ok(Some(_)) | Ok(None) => {
                // credential_process is not set, parse the credentials file
                parse_credentials_file(self.file_path()).and_then(|mut profiles| {
                    profiles
                        .remove(self.profile())
                        .ok_or_else(|| CredentialsError::new("profile not found"))
                })
            }
            Err(err) => Err(err),
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

fn parse_config_file<P>(file_path: P) -> Option<HashMap<String, HashMap<String, String>>>
where
    P: AsRef<Path>,
{
    if !file_path.as_ref().exists() || !file_path.as_ref().is_file() {
        return None;
    }

    let file = File::open(file_path).expect("expected file");
    let file_lines = BufReader::new(&file);
    let result: (HashMap<String, HashMap<String, String>>, Option<String>) = file_lines
        .lines()
        .filter_map(|line| {
            line.ok()
                .map(|l| l.trim_matches(' ').to_owned())
                .into_iter()
                .find(|l| !l.starts_with('#') && !l.is_empty())
        })
        .fold(Default::default(), |(mut result, profile), line| {
            if PROFILE_REGEX.is_match(&line) {
                let caps = PROFILE_REGEX.captures(&line).unwrap();
                let next_profile = caps.get(2).map(|value| value.as_str().to_string());
                (result, next_profile)
            } else {
                match &line
                    .splitn(2, '=')
                    .map(|value| value.trim_matches(' '))
                    .collect::<Vec<&str>>()[..]
                {
                    [key, value] if !key.is_empty() && !value.is_empty() => {
                        if let Some(current) = profile.clone() {
                            let values = result.entry(current).or_insert_with(HashMap::new);
                            (*values).insert(key.to_string(), value.to_string());
                        }
                        (result, profile)
                    }
                    _ => (result, profile),
                }
            }
        });
    Some(result.0)
}

/// Parses a Credentials file into a Map of <`ProfileName`, `AwsCredentials`>
fn parse_credentials_file<P>(
    file_path: P,
) -> Result<HashMap<String, AwsCredentials>, CredentialsError>
where
    P: AsRef<Path>,
{
    let profiles: HashMap<_, _> = parse_config_file(&file_path)
        .map(|data| {
            Ok(data
                .into_iter()
                .filter_map(|(profile, properties)| {
                    if let (Some(key), Some(secret)) = (
                        properties.get("aws_access_key_id"),
                        properties.get("aws_secret_access_key"),
                    ) {
                        Some((
                            profile,
                            AwsCredentials::new(
                                key,
                                secret,
                                properties
                                    .get("aws_session_token")
                                    .or_else(|| properties.get("aws_security_token"))
                                    .map(String::to_owned),
                                None,
                            ),
                        ))
                    } else {
                        None
                    }
                })
                .collect())
        })
        .unwrap_or_else(|| {
            Err(CredentialsError::new(format!(
                "Invalid credentials file: [ {} ].",
                file_path.as_ref().display()
            )))
        })?;

    if profiles.is_empty() {
        Err(CredentialsError::new("No credentials found."))
    } else {
        Ok(profiles)
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

    use std::env;
    use std::path::Path;

    use super::*;
    use crate::test_utils::lock_env;
    use crate::{CredentialsError, ProvideAwsCredentials};

    #[test]
    fn parse_config_file_default_profile() {
        let result = super::parse_config_file(Path::new("tests/sample-data/default_config"));
        assert!(result.is_some());
        let profiles = result.unwrap();
        assert_eq!(profiles.len(), 1);
        let default_profile = profiles
            .get(DEFAULT)
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.get(REGION), Some(&"us-east-2".to_string()));
        assert_eq!(default_profile.get("output"), Some(&"json".to_string()));
    }

    #[test]
    fn parse_config_file_multiple_profiles() {
        let result =
            super::parse_config_file(Path::new("tests/sample-data/multiple_profile_config"));
        assert!(result.is_some());

        let profiles = result.unwrap();
        assert_eq!(profiles.len(), 3);

        let foo_profile = profiles
            .get("foo")
            .expect("No foo profile in multiple_profile_credentials");
        assert_eq!(foo_profile.get(REGION), Some(&"us-east-3".to_string()));
        assert_eq!(foo_profile.get("output"), Some(&"json".to_string()));

        let bar_profile = profiles
            .get("bar")
            .expect("No bar profile in multiple_profile_credentials");
        assert_eq!(bar_profile.get(REGION), Some(&"us-east-4".to_string()));
        assert_eq!(bar_profile.get("output"), Some(&"json".to_string()));
        assert_eq!(bar_profile.get("# comments"), None);
    }

    #[test]
    fn parse_config_file_credential_process() {
        let result =
            super::parse_config_file(Path::new("tests/sample-data/credential_process_config"));
        assert!(result.is_some());
        let profiles = result.unwrap();
        assert_eq!(profiles.len(), 2);
        let default_profile = profiles
            .get(DEFAULT)
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.get(REGION), Some(&"us-east-2".to_string()));
        assert_eq!(
            default_profile.get("credential_process"),
            Some(&"cat tests/sample-data/credential_process_sample_response".to_string())
        );
    }

    #[test]
    fn parse_credentials_file_default_profile() {
        let result = super::parse_credentials_file(Path::new(
            "tests/sample-data/default_profile_credentials",
        ));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles
            .get(DEFAULT)
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

    #[test]
    fn parse_credentials_file_multiple_profiles() {
        let result = super::parse_credentials_file(Path::new(
            "tests/sample-data/multiple_profile_credentials",
        ));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 2);

        let foo_profile = profiles
            .get("foo")
            .expect("No foo profile in multiple_profile_credentials");
        assert_eq!(foo_profile.aws_access_key_id(), "foo_access_key");
        assert_eq!(foo_profile.aws_secret_access_key(), "foo_secret_key");

        let bar_profile = profiles
            .get("bar")
            .expect("No bar profile in multiple_profile_credentials");
        assert_eq!(bar_profile.aws_access_key_id(), "bar_access_key");
        assert_eq!(bar_profile.aws_secret_access_key(), "bar_secret_key");
    }

    #[test]
    fn parse_all_values_credentials_file() {
        let result =
            super::parse_credentials_file(Path::new("tests/sample-data/full_profile_credentials"));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles
            .get(DEFAULT)
            .expect("No default profile in full_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

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
        env::set_var(AWS_SHARED_CREDENTIALS_FILE, credentials_path);
        let result = ProfileProvider::new();
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.file_path().to_str().unwrap(), credentials_path);
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
    }

    #[tokio::test]
    async fn profile_provider_profile_name_via_environment_variable() {
        let _guard = lock_env();
        let credentials_path = "tests/sample-data/multiple_profile_credentials";
        env::set_var(AWS_SHARED_CREDENTIALS_FILE, credentials_path);
        env::set_var(AWS_PROFILE, "bar");
        let result = ProfileProvider::new();
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.file_path().to_str().unwrap(), credentials_path);
        let creds = provider.credentials().await;
        assert_eq!(creds.unwrap().aws_access_key_id(), "bar_access_key");
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
        env::remove_var(AWS_PROFILE);
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
            Some(CredentialsError::new("profile not found"))
        );
    }

    #[tokio::test]
    async fn profile_provider_credential_process() {
        let _guard = lock_env();
        env::set_var(
            AWS_CONFIG_FILE,
            "tests/sample-data/credential_process_config",
        );
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

        env::remove_var(AWS_CONFIG_FILE);
    }

    #[tokio::test]
    async fn profile_provider_credential_process_foo() {
        let _guard = lock_env();
        env::set_var(
            AWS_CONFIG_FILE,
            "tests/sample-data/credential_process_config",
        );
        let mut provider = ProfileProvider::new().unwrap();
        provider.set_profile("foo");
        let result = provider.credentials().await;

        assert!(result.is_ok());

        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "foo_access_key");
        assert_eq!(creds.aws_secret_access_key(), "foo_secret_key");
        assert_eq!(
            creds.token().as_ref().expect("session token not parsed"),
            "foo_session_token"
        );
        assert!(creds.expires_at().is_some());

        env::remove_var(AWS_CONFIG_FILE);
    }

    #[test]
    fn profile_provider_profile_name() {
        let _guard = lock_env();
        let mut provider = ProfileProvider::new().unwrap();
        assert_eq!(DEFAULT, provider.profile());
        provider.set_profile("foo");
        assert_eq!("foo", provider.profile());
    }

    #[test]
    fn existing_file_no_credentials() {
        let result = super::parse_credentials_file(Path::new("tests/sample-data/no_credentials"));
        assert_eq!(
            result.err(),
            Some(CredentialsError::new("No credentials found."))
        )
    }

    #[test]
    fn parse_credentials_bad_path() {
        let result = super::parse_credentials_file("/bad/file/path");
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(
                "Invalid credentials file: [ /bad/file/path ].",
            ))
        );
    }

    #[test]
    fn parse_credentials_directory_path() {
        let result = super::parse_credentials_file("tests/");
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(
                "Invalid credentials file: [ tests/ ].",
            ))
        );
    }

    #[test]
    fn parse_credentials_unrecognized_field() {
        let result = super::parse_credentials_file(Path::new(
            "tests/sample-data/unrecognized_field_profile_credentials",
        ));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles
            .get(DEFAULT)
            .expect("No default profile in full_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

    #[test]
    fn default_profile_name_from_env_var() {
        let _guard = lock_env();
        env::set_var(AWS_PROFILE, "bar");
        assert_eq!("bar", ProfileProvider::default_profile_name());
        env::remove_var(AWS_PROFILE);
    }

    #[test]
    fn default_profile_name_from_empty_env_var() {
        let _guard = lock_env();
        env::set_var(AWS_PROFILE, "");
        assert_eq!(DEFAULT, ProfileProvider::default_profile_name());
        env::remove_var(AWS_PROFILE);
    }

    #[test]
    fn default_profile_name() {
        let _guard = lock_env();
        env::remove_var(AWS_PROFILE);
        assert_eq!(DEFAULT, ProfileProvider::default_profile_name());
    }

    #[test]
    fn default_profile_location_from_env_var() {
        let _guard = lock_env();
        env::set_var(AWS_SHARED_CREDENTIALS_FILE, "bar");
        assert_eq!(
            Ok(PathBuf::from("bar")),
            ProfileProvider::default_profile_location()
        );
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
    }

    #[test]
    fn default_profile_location_from_empty_env_var() {
        let _guard = lock_env();
        env::set_var(AWS_SHARED_CREDENTIALS_FILE, "");
        assert_eq!(
            ProfileProvider::hardcoded_profile_location(),
            ProfileProvider::default_profile_location()
        );
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
    }

    #[test]
    fn default_profile_location() {
        let _guard = lock_env();
        env::remove_var(AWS_SHARED_CREDENTIALS_FILE);
        assert_eq!(
            ProfileProvider::hardcoded_profile_location(),
            ProfileProvider::default_profile_location()
        );
    }
}
