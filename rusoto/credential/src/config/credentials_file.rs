use std::path::Path;

use ini::{Ini, Properties};

use crate::CredentialsError;

use super::try_parse_ini;

fn try_parse_credentials_ini<L>(location: L) -> Result<Ini, CredentialsError>
where
    L: AsRef<Path>,
{
    try_parse_ini(location).map_err(|e| {
        CredentialsError::new(format!(
            "An error occurred while parsing the credentials file: {}",
            e
        ))
    })
}

/// The AWS [credentials] file. Located at `~/.aws/credentials` by default, its location can be overriden with the
/// `AWS_SHARED_CREDENTIALS_FILE` environment variable.
///
/// [credentials]: https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html
pub struct CredentialsFile {
    ini: Ini,
}

impl CredentialsFile {
    /// Parses the credentials file at the given location.
    pub fn new<L>(location: L) -> Result<Self, CredentialsError>
    where
        L: AsRef<Path>,
    {
        let ini = try_parse_credentials_ini(location)?;
        Ok(CredentialsFile { ini })
    }

    /// Returns the profile with the given name.
    pub fn profile(&self, profile_name: &str) -> Option<CredentialsProfile<'_>> {
        self.ini
            .section(Some(profile_name))
            .map(CredentialsProfile::from)
    }
}

/// A profile defined in the AWS [credentials] file.
pub struct CredentialsProfile<'a> {
    properties: &'a Properties,
}

impl<'a> From<&'a Properties> for CredentialsProfile<'a> {
    fn from(properties: &'a Properties) -> Self {
        CredentialsProfile { properties }
    }
}

impl<'a> CredentialsProfile<'a> {
    /// Returns the region property of this profile.
    pub fn region(&self) -> Option<&'a str> {
        self.properties.get("region")
    }

    /// Returns the aws_access_key_id property of this profile.
    pub fn aws_access_key_id(&self) -> Option<&'a str> {
        self.properties.get("aws_access_key_id")
    }

    /// Returns the aws_secret_access_key property of this profile.
    pub fn aws_secret_access_key(&self) -> Option<&'a str> {
        self.properties.get("aws_secret_access_key")
    }

    /// Returns the aws_session_token property of this profile.
    pub fn aws_session_token(&self) -> Option<&'a str> {
        self.properties.get("aws_session_token")
    }

    /// Returns the aws_security_token property of this profile.
    pub fn aws_security_token(&self) -> Option<&'a str> {
        self.properties.get("aws_security_token")
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::{config::default_profile_name, CredentialsError};

    #[test]
    fn parse_credentials_file_default_profile() {
        let result =
            CredentialsFile::new(Path::new("tests/sample-data/default_profile_credentials"));
        assert!(result.is_ok());

        let credentials = result.ok().unwrap();

        let default_profile = credentials
            .profile(&default_profile_name())
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), Some("foo"));
        assert_eq!(default_profile.aws_secret_access_key(), Some("bar"));
    }

    #[test]
    fn parse_credentials_file_multiple_profiles() {
        let result =
            CredentialsFile::new(Path::new("tests/sample-data/multiple_profile_credentials"));
        assert!(result.is_ok());

        let credentials = result.ok().unwrap();

        let foo_profile = credentials
            .profile("foo")
            .expect("No foo profile in multiple_profile_credentials");
        assert_eq!(foo_profile.aws_access_key_id(), Some("foo_access_key"));
        assert_eq!(foo_profile.aws_secret_access_key(), Some("foo_secret_key"));

        let bar_profile = credentials
            .profile("bar")
            .expect("No bar profile in multiple_profile_credentials");
        assert_eq!(bar_profile.aws_access_key_id(), Some("bar_access_key"));
        assert_eq!(bar_profile.aws_secret_access_key(), Some("bar_secret_key"));
    }

    #[test]
    fn parse_all_values_credentials_file() {
        let result = CredentialsFile::new(Path::new("tests/sample-data/full_profile_credentials"));
        assert!(result.is_ok());

        let credentials = result.ok().unwrap();

        let default_profile = credentials
            .profile(&default_profile_name())
            .expect("No default profile in full_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), Some("foo"));
        assert_eq!(default_profile.aws_secret_access_key(), Some("bar"));
    }

    #[test]
    fn parse_credentials_bad_path() {
        let result = CredentialsFile::new(Path::new("/bad/file/path"));
        assert!(
            result.err().unwrap().to_string().starts_with("An error occurred while parsing the credentials file: Couldn\'t stat file [ \"/bad/file/path\" ]: "),
        );
    }

    #[test]
    fn parse_credentials_directory_path() {
        let result = CredentialsFile::new(Path::new("tests/"));
        assert_eq!(
            result.err(),
            Some(CredentialsError::new(
                "An error occurred while parsing the credentials file: [ \"tests/\" ] is not a file.",
            ))
        );
    }

    #[test]
    fn parse_credentials_unrecognized_field() {
        let result = CredentialsFile::new(Path::new(
            "tests/sample-data/unrecognized_field_profile_credentials",
        ));
        assert!(result.is_ok());

        let credentials = result.ok().unwrap();

        let default_profile = credentials
            .profile(&default_profile_name())
            .expect("No default profile in full_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), Some("foo"));
        assert_eq!(default_profile.aws_secret_access_key(), Some("bar"));
    }
}
