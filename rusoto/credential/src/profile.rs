//! The Credentials Provider for Credentials stored in a profile inside of a Credentials file.

use regex::Regex;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::env::{home_dir, var as env_var};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use {AwsCredentials, CredentialsError, ProvideAwsCredentials, in_ten_minutes};

/// Provides AWS credentials from a profile in a credentials file.
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
        let profile_location = match env_var("AWS_SHARED_CREDENTIALS_FILE") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                match ProfileProvider::default_profile_location() {
                    Ok(path) => path,
                    Err(err) => return Err(err),
                }
            }
        };

        Ok(ProfileProvider {
            file_path: profile_location,
            profile: "default".to_owned(),
        })
    }

    /// Default credentials file location:
    /// `~/.aws/credentials` (Linux/Mac)
    /// `%USERPROFILE%\.aws\credentials` (Windows)
    fn default_profile_location() -> Result<PathBuf, CredentialsError> {
        match home_dir() {
            Some(home_path) => {
                let mut credentials_path = PathBuf::from(".aws");

                credentials_path.push("credentials");

                Ok(home_path.join(credentials_path))
            }
            None => Err(CredentialsError::new("The environment variable HOME must be set.")),
        }
    }

    /// Create a new `ProfileProvider` for the credentials file at the given path, using
    /// the given profile.
    pub fn with_configuration<F, P>(file_path: F, profile: P) -> ProfileProvider
    where F: Into<PathBuf>, P: Into<String> {
        ProfileProvider {
            file_path: file_path.into(),
            profile: profile.into(),
        }
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
    pub fn set_file_path<F>(&mut self, file_path: F) where F: Into<PathBuf> {
        self.file_path = file_path.into();
    }

    /// Set the profile name.
    pub fn set_profile<P>(&mut self, profile: P) where P: Into<String> {
        self.profile = profile.into();
    }
}

impl ProvideAwsCredentials for ProfileProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        parse_credentials_file(self.file_path()).and_then(|mut profiles| {
            profiles.remove(self.profile()).ok_or_else(|| CredentialsError::new("profile not found"))
        })
    }
}

/// Parses a Credentials file into a Map of <ProfileName, AwsCredentials>
fn parse_credentials_file(file_path: &Path) -> Result<HashMap<String, AwsCredentials>, CredentialsError> {
    match fs::metadata(file_path) {
        Err(_) => return Err(
            CredentialsError::new(
                format!("Couldn't stat credentials file: [ {:?} ]. Non existant, or no permission.", file_path)
            )
        ),
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(CredentialsError::new(format!("Credentials file: [ {:?} ] is not a file.", file_path)));
            }
        }
    };

    let file = try!(File::open(file_path));

    let profile_regex = Regex::new(r"^\[([^\]]+)\]$").expect("Failed to compile regex");
    let mut profiles: HashMap<String, AwsCredentials> = HashMap::new();
    let mut access_key: Option<String> = None;
    let mut secret_key: Option<String> = None;
    let mut token: Option<String> = None;
    let mut profile_name: Option<String> = None;

    let file_lines = BufReader::new(&file);
    for (line_no, line) in file_lines.lines().enumerate() {
        let unwrapped_line: String = line.expect(&format!("Failed to read credentials file, line: {}", line_no));

        // skip empty lines
        if unwrapped_line.is_empty() {
            continue;
        }

        // skip comments
        if unwrapped_line.starts_with('#') {
            continue;
        }

        // handle the opening of named profile blocks
        if profile_regex.is_match(&unwrapped_line) {
            if profile_name.is_some() && access_key.is_some() && secret_key.is_some() {
                let creds = AwsCredentials::new(access_key.unwrap(), secret_key.unwrap(), token, in_ten_minutes());
                profiles.insert(profile_name.unwrap(), creds);
            }

            access_key = None;
            secret_key = None;
            token = None;

            let caps = profile_regex.captures(&unwrapped_line).unwrap();
            profile_name = Some(caps.get(1).unwrap().as_str().to_string());
            continue;
        }

        // otherwise look for key=value pairs we care about
        let lower_case_line = unwrapped_line.to_ascii_lowercase().to_string();

        if lower_case_line.contains("aws_access_key_id") &&
            access_key.is_none()
        {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                access_key = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_secret_access_key") &&
            secret_key.is_none()
        {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                secret_key = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_session_token") &&
            token.is_none()
        {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                token = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_security_token")
        {
            if token.is_none() {
                let v: Vec<&str> = unwrapped_line.split('=').collect();
                if !v.is_empty() {
                    token = Some(v[1].trim_matches(' ').to_string());
                }
            }
        } else if lower_case_line.contains("region") ||
            lower_case_line.contains("output")
        {
            // Not Supported here, but valid according to: http://docs.aws.amazon.com/cli/latest/userguide/cli-chap-getting-started.html#cli-config-files
            continue;
        } else {
            return Err(CredentialsError::new(format!("Invalid AWS Config line: [ {} ]", lower_case_line)))
        }

    }

    if profile_name.is_some() && access_key.is_some() && secret_key.is_some() {
        let creds = AwsCredentials::new(access_key.unwrap(), secret_key.unwrap(), token, in_ten_minutes());
        profiles.insert(profile_name.unwrap(), creds);
    }

    if profiles.is_empty() {
        return Err(CredentialsError::new("No credentials found."));
    }

    Ok(profiles)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::Path;

    use {CredentialsError, ProvideAwsCredentials};
    use super::*;

    #[test]
    fn parse_credentials_file_default_profile() {
        let result = super::parse_credentials_file(
            Path::new("tests/sample-data/default_profile_credentials")
        );
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles.get("default").expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

    #[test]
    fn parse_credentials_file_multiple_profiles() {
        let result = super::parse_credentials_file(
            Path::new("tests/sample-data/multiple_profile_credentials")
        );
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 2);

        let foo_profile = profiles.get("foo").expect("No foo profile in multiple_profile_credentials");
        assert_eq!(foo_profile.aws_access_key_id(), "foo_access_key");
        assert_eq!(foo_profile.aws_secret_access_key(), "foo_secret_key");

        let bar_profile = profiles.get("bar").expect("No bar profile in multiple_profile_credentials");
        assert_eq!(bar_profile.aws_access_key_id(), "bar_access_key");
        assert_eq!(bar_profile.aws_secret_access_key(), "bar_secret_key");

    }

    #[test]
    fn parse_all_values_credentials_file() {
        let result = super::parse_credentials_file(
            Path::new("tests/sample-data/full_profile_credentials")
        );
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles.get("default").expect("No default profile in full_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

    #[test]
    fn profile_provider_happy_path() {
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foo",
        );
        let result = provider.credentials();

        assert!(result.is_ok());

        let creds = result.ok().unwrap();
        assert_eq!(creds.aws_access_key_id(), "foo_access_key");
        assert_eq!(creds.aws_secret_access_key(), "foo_secret_key");
    }

    #[test]
    fn profile_provider_via_environment_variable() {
        let credentials_path = "tests/sample-data/default_profile_credentials";
        env::set_var("AWS_SHARED_CREDENTIALS_FILE", credentials_path);
        let result = ProfileProvider::new();
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.file_path().to_str().unwrap(), credentials_path);
        env::remove_var("AWS_SHARED_CREDENTIALS_FILE");
    }

    #[test]
    fn profile_provider_bad_profile() {
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "not_a_profile",
        );
        let result = provider.credentials();

        assert!(result.is_err());
        assert_eq!(result.err(), Some(CredentialsError::new("profile not found")));
    }

    #[test]
    fn profile_provider_profile_name() {
       let mut provider = ProfileProvider::new().unwrap();
       assert_eq!("default", provider.profile());
       provider.set_profile("foo");
       assert_eq!("foo", provider.profile());
    }

    #[test]
    fn existing_file_no_credentials() {
        let result = super::parse_credentials_file(Path::new("tests/sample-data/no_credentials"));
        assert_eq!(result.err(), Some(CredentialsError::new("No credentials found.")))
    }

    #[test]
    fn parse_credentials_bad_path() {
        let result = super::parse_credentials_file(Path::new("/bad/file/path"));
        assert_eq!(result.err(), Some(CredentialsError::new("Couldn\'t stat credentials file: [ \"/bad/file/path\" ]. Non existant, or no permission.")));
    }

    #[test]
    fn parse_credentials_directory_path() {
        let result = super::parse_credentials_file(Path::new("tests/"));
        assert_eq!(result.err(), Some(CredentialsError::new("Credentials file: [ \"tests/\" ] is not a file.")));
    }

    #[test]
    fn parse_credentials_invalid() {
        let result = super::parse_credentials_file(Path::new("tests/sample-data/invalid_profile_credentials"));
        assert_eq!(result.err(), Some(CredentialsError::new("Invalid AWS Config line: [ some_super_awesome_value = baz ]")));
    }
}
