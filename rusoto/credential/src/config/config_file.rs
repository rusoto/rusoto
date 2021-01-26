use std::path::Path;

use ini::{Ini, Properties};

use crate::CredentialsError;

use super::{default_config_location, default_profile_name, try_parse_ini};

pub struct ConfigFile {
    ini: Ini,
}

fn try_parse_config_ini<L>(location: L) -> Result<Ini, CredentialsError>
where
    L: AsRef<Path>,
{
    try_parse_ini(location).map_err(|e| {
        CredentialsError::new(format!("An error occurred parsing the config file: {}", e))
    })
}

impl ConfigFile {
    pub fn new<L>(location: L) -> Result<Self, CredentialsError>
    where
        L: AsRef<Path>,
    {
        let ini = try_parse_config_ini(location)?;
        Ok(ConfigFile { ini })
    }

    pub fn new_default() -> Result<Self, CredentialsError> {
        let location = default_config_location()?;
        Self::new(&location)
    }

    pub fn profile(&self, profile_name: &str) -> Option<ConfigProfile<'_>> {
        self.ini
            .section(Some(profile_name))
            .or_else(|| self.ini.section(Some(&format!("profile {}", profile_name))))
            .map(ConfigProfile::from)
    }

    pub fn default_profile(&self) -> Option<ConfigProfile<'_>> {
        self.profile(&default_profile_name())
    }
}

pub struct ConfigProfile<'a> {
    properties: &'a Properties,
}

impl<'a> From<&'a Properties> for ConfigProfile<'a> {
    fn from(properties: &'a Properties) -> Self {
        ConfigProfile { properties }
    }
}

impl<'a> ConfigProfile<'a> {
    pub fn region(&self) -> Option<&'a str> {
        self.properties.get("region")
    }

    pub fn credential_process(&self) -> Option<&'a str> {
        self.properties.get("credential_process")
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn parse_config_file_default_profile() {
        let result = ConfigFile::new(Path::new("tests/sample-data/default_config"));
        assert!(result.is_ok());
        let config = result.unwrap();
        let default_profile = config
            .default_profile()
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.region(), Some("us-east-2"));
    }

    #[test]
    fn parse_config_file_multiple_profiles() {
        let result = ConfigFile::new(Path::new("tests/sample-data/multiple_profile_config"));
        assert!(result.is_ok());

        let config = result.unwrap();

        let foo_profile = config
            .profile("foo")
            .expect("No foo profile in multiple_profile_credentials");
        assert_eq!(foo_profile.region(), Some("us-east-3"));

        let bar_profile = config
            .profile("bar")
            .expect("No bar profile in multiple_profile_credentials");
        assert_eq!(bar_profile.region(), Some("us-east-4"));
    }

    #[test]
    fn parse_config_file_credential_process() {
        let result = ConfigFile::new(Path::new("tests/sample-data/credential_process_config"));
        assert!(result.is_ok());
        let config = result.unwrap();
        let default_profile = config
            .default_profile()
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.region(), Some("us-east-2"));
        assert_eq!(
            default_profile.credential_process(),
            Some("cat tests/sample-data/credential_process_sample_response")
        );
    }
}
