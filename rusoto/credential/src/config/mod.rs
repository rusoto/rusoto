use std::{
    fs,
    path::{Path, PathBuf},
};

use dirs_next::home_dir;
use ini::Ini;

use crate::{non_empty_env_var, CredentialsError};

mod config_file;
mod credentials_file;

pub use config_file::{ConfigFile, ConfigProfile};
pub use credentials_file::{CredentialsFile, CredentialsProfile};

const AWS_PROFILE: &str = "AWS_PROFILE";
const AWS_CONFIG_FILE: &str = "AWS_CONFIG_FILE";
const AWS_SHARED_CREDENTIALS_FILE: &str = "AWS_SHARED_CREDENTIALS_FILE";
const DEFAULT_PROFILE: &str = "default";

/// Default config file location:
/// 1: if set and not empty, use the value from environment variable ```AWS_CONFIG_FILE```
/// 2. otherwise return `~/.aws/config` (Linux/Mac) resp. `%USERPROFILE%\.aws\config` (Windows)
pub fn default_config_location() -> Result<PathBuf, CredentialsError> {
    let env = non_empty_env_var(AWS_CONFIG_FILE);
    match env {
        Some(path) => Ok(PathBuf::from(path)),
        None => hardcoded_location("config"),
    }
}

/// Default credentials file location:
/// 1. if set and not empty, use value from environment variable ```AWS_SHARED_CREDENTIALS_FILE```
/// 2. otherwise return `~/.aws/credentials` (Linux/Mac) resp. `%USERPROFILE%\.aws\credentials` (Windows)
pub fn default_credentials_location() -> Result<PathBuf, CredentialsError> {
    let env = non_empty_env_var(AWS_SHARED_CREDENTIALS_FILE);
    match env {
        Some(path) => Ok(PathBuf::from(path)),
        None => hardcoded_location("credentials"),
    }
}

fn hardcoded_location(filename: &str) -> Result<PathBuf, CredentialsError> {
    match home_dir() {
        Some(mut home_path) => {
            home_path.push(".aws");
            home_path.push(filename);
            Ok(home_path)
        }
        None => Err(CredentialsError::new("Failed to determine home directory.")),
    }
}

/// Get the default profile name:
/// 1. if set and not empty, use value from environment variable ```AWS_PROFILE```
/// 2. otherwise return ```"default"```
/// see https://docs.aws.amazon.com/sdk-for-java/v1/developer-guide/credentials.html.
pub fn default_profile_name() -> String {
    non_empty_env_var(AWS_PROFILE).unwrap_or_else(|| DEFAULT_PROFILE.into())
}

pub(self) fn try_parse_ini<L>(location: L) -> Result<Ini, CredentialsError>
where
    L: AsRef<Path>,
{
    match fs::metadata(location.as_ref()) {
        Err(err) => {
            return Err(CredentialsError::new(format!(
                "Couldn't stat file [ {:?} ]: {}",
                location.as_ref(),
                err
            )));
        }
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(CredentialsError::new(format!(
                    "[ {:?} ] is not a file.",
                    location.as_ref()
                )));
            }
        }
    };

    let ini = Ini::load_from_file(location).map_err(|err| {
        CredentialsError::new(format!("An error occurred while parsing the file: {}", err))
    })?;

    Ok(ini)
}

#[cfg(test)]
pub mod test_utils {
    use super::{AWS_CONFIG_FILE, AWS_PROFILE, AWS_SHARED_CREDENTIALS_FILE};
    use std::env;

    #[derive(Debug)]
    pub struct VarGuard {
        variable: String,
        previous_value: Option<String>,
    }

    impl VarGuard {
        fn new(variable: String, value: Option<String>) -> VarGuard {
            let previous_value = env::var(&variable).map(Some).unwrap_or(None);
            if let Some(value) = value {
                env::set_var(&variable, value);
            } else {
                env::remove_var(&variable);
            }
            VarGuard {
                variable,
                previous_value,
            }
        }
    }

    impl Drop for VarGuard {
        fn drop(&mut self) {
            if let Some(value) = self.previous_value.take() {
                env::set_var(&self.variable, value);
            } else {
                env::remove_var(&self.variable)
            }
        }
    }

    pub fn with_env_profile(value: Option<&str>) -> VarGuard {
        VarGuard::new(AWS_PROFILE.to_string(), value.map(ToString::to_string))
    }

    pub fn with_env_config_file(value: Option<&str>) -> VarGuard {
        VarGuard::new(AWS_CONFIG_FILE.to_string(), value.map(ToString::to_string))
    }

    pub fn with_env_shared_credentials_file(value: Option<&str>) -> VarGuard {
        VarGuard::new(
            AWS_SHARED_CREDENTIALS_FILE.to_string(),
            value.map(ToString::to_string),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::lock_env;
    use test_utils::{with_env_profile, with_env_shared_credentials_file};

    #[test]
    fn default_profile_name_from_env_var() {
        let _guard = lock_env();
        let _profile_guard = with_env_profile(Some("bar"));
        assert_eq!("bar", default_profile_name());
    }

    #[test]
    fn default_profile_name_from_empty_env_var() {
        let _guard = lock_env();
        let _profile_guard = with_env_profile(Some(""));
        assert_eq!(DEFAULT_PROFILE, default_profile_name());
    }

    #[test]
    fn default_profile_name_with_no_env_var() {
        let _guard = lock_env();
        let _profile_guard = with_env_profile(None);
        assert_eq!(DEFAULT_PROFILE, default_profile_name());
    }

    #[test]
    fn default_profile_location_from_env_var() {
        let _guard = lock_env();
        let _credentials_guard = with_env_shared_credentials_file(Some("bar"));
        assert_eq!(Ok(PathBuf::from("bar")), default_credentials_location());
    }

    #[test]
    fn default_profile_location_from_empty_env_var() {
        let _guard = lock_env();
        let _credentials_guard = with_env_shared_credentials_file(Some(""));
        assert_eq!(
            hardcoded_location("credentials"),
            default_credentials_location()
        );
    }

    #[test]
    fn default_profile_location_with_no_env_var() {
        let _guard = lock_env();
        let _credentials_guard = with_env_shared_credentials_file(None);
        assert_eq!(
            hardcoded_location("credentials"),
            default_credentials_location()
        );
    }
}
