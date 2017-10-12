//! AWS Regions and helper functions.
//!
//! Mostly used for translating the Region enum to a string AWS accepts.
//!
//! For example: `UsEast1` to "us-east-1"

use std;
use std::error::Error;
use std::str::FromStr;
use std::fmt::{Display, Error as FmtError, Formatter};
use serde::{de, Serialize, Serializer, Deserialize, Deserializer};

/// An AWS region.
/// `Custom` can be used to use a local or otherwise non-AWS endpoint.  This may be used for API-compatible services, such as DynamoDB Local or Ceph.
/// Example: `Region::Custom("http://localhost:8000".to_owned())` instead of `Region::UsEast1`.
/// `CnNorth1` is currently untested due to Rusoto maintainers not having access to AWS China.
#[derive(Clone, Debug, PartialEq)]
pub enum Region {
    /// Region that covers North-East part of America
    ApNortheast1,

    /// Region that covers North-East America
    ApNortheast2,

    /// Region that covers South America
    ApSouth1,

    /// Region that covers South-East America
    ApSoutheast1,

    /// Region that covers South-East America
    ApSoutheast2,

    /// Region that covers Central America
    CaCentral1,

    /// Region that covers Central Europe
    EuCentral1,

    /// Region that covers West Europe
    EuWest1,

    /// Region that covers West Europe
    EuWest2,

    /// Region that covers South-East Europe
    SaEast1,

    /// Region that covers East part of America
    UsEast1,

    /// Region that covers East part of America
    UsEast2,

    /// Region that covers West part of America
    UsWest1,

    /// Region that covers North-East part of America
    UsWest2,

    /// Region that covers North Canada
    CnNorth1,

    /// Specifies a custom region, such as a local Ceph target
    Custom(String)
}

/// An error produced when attempting to convert a `str` into a `Region` fails.
#[derive(Debug,PartialEq)]
pub struct ParseRegionError {
    message: String,
}

// Manually derived for lack of way to flatten Custom variant
// Related: https://github.com/serde-rs/serde/issues/119
impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
    {
        serializer.collect_str(&self.to_string())
    }
}

// Manually derived to deserialize both the standard 'us-east-1' kebab-case style
// as well as 'UsEast1' PascalCase style to faciliate migrating from older versions
impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let region_str = match *self {
            Region::ApNortheast1 => "ap-northeast-1",
            Region::ApNortheast2 => "ap-northeast-2",
            Region::ApSouth1 => "ap-south-1",
            Region::ApSoutheast1 => "ap-southeast-1",
            Region::ApSoutheast2 => "ap-southeast-2",
            Region::CaCentral1 => "ca-central-1",
            Region::EuCentral1 => "eu-central-1",
            Region::EuWest1 => "eu-west-1",
            Region::EuWest2 => "eu-west-2",
            Region::SaEast1 => "sa-east-1",
            Region::UsEast1 => "us-east-1",
            Region::UsEast2 => "us-east-2",
            Region::UsWest1 => "us-west-1",
            Region::UsWest2 => "us-west-2",
            Region::CnNorth1 => "cn-north-1",
            Region::Custom(ref hostname) => &hostname,
        };

        write!(f, "{}", region_str)
    }
}

impl FromStr for Region {
    type Err = ParseRegionError;

    fn from_str(s: &str) -> Result<Region, ParseRegionError> {
        let v : &str = &s.to_lowercase();
        match v {
            "ap-northeast-1" | "apnortheast1" => Ok(Region::ApNortheast1),
            "ap-northeast-2" | "apnortheast2" => Ok(Region::ApNortheast2),
            "ap-south-1" | "apsouth1" => Ok(Region::ApSouth1),
            "ap-southeast-1" | "apsoutheast1" => Ok(Region::ApSoutheast1),
            "ap-southeast-2" | "apsoutheast2" => Ok(Region::ApSoutheast2),
            "ca-central-1" | "cacentral1" => Ok(Region::CaCentral1),
            "eu-central-1" | "eucentral1" => Ok(Region::EuCentral1),
            "eu-west-1" | "euwest1" => Ok(Region::EuWest1),
            "eu-west-2" | "euwest2" => Ok(Region::EuWest2),
            "sa-east-1" | "saeast1" => Ok(Region::SaEast1),
            "us-east-1" | "useast1" => Ok(Region::UsEast1),
            "us-east-2" | "useast2" => Ok(Region::UsEast2),
            "us-west-1" | "uswest1" => Ok(Region::UsWest1),
            "us-west-2" | "uswest2" => Ok(Region::UsWest2),
            "cn-north-1" | "cnnorth1" => Ok(Region::CnNorth1),
            s => Err(ParseRegionError::new(s)),
        }
    }
}

impl ParseRegionError {
    /// Parses a region given as a string literal into a type `Region'
    pub fn new(input: &str) -> Self {
        ParseRegionError { message: format!("Not a valid AWS region: {}", input) }
    }
}

impl Error for ParseRegionError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for ParseRegionError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.message)
    }
}

/// Get the region from `AWS_DEFAULT_REGION` environment variable.
/// Uses us-east-1 if not set or value is malformed.
pub fn default_region() -> Region {
    match std::env::var("AWS_DEFAULT_REGION") {
        Ok(v) => {
            let slice : &str = &v;
            match Region::from_str(slice) {
                Ok(region) => region,
                Err(_) => Region::UsEast1,
            }
        },
        Err(_) => Region::UsEast1,
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;
    use self::serde_test::{Token, assert_tokens, assert_ser_tokens, assert_de_tokens};
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!("foo"
                       .parse::<Region>()
                       .err()
                       .expect("Parsing foo as a Region was not an error")
                       .to_string(),
                   "Not a valid AWS region: foo".to_owned());
        assert_eq!("ap-northeast-1".parse(), Ok(Region::ApNortheast1));
        assert_eq!("ap-northeast-2".parse(), Ok(Region::ApNortheast2));
        assert_eq!("ap-south-1".parse(), Ok(Region::ApSouth1));
        assert_eq!("ap-southeast-1".parse(), Ok(Region::ApSoutheast1));
        assert_eq!("ap-southeast-2".parse(), Ok(Region::ApSoutheast2));
        assert_eq!("ca-central-1".parse(), Ok(Region::CaCentral1));
        assert_eq!("eu-central-1".parse(), Ok(Region::EuCentral1));
        assert_eq!("eu-west-1".parse(), Ok(Region::EuWest1));
        assert_eq!("eu-west-2".parse(), Ok(Region::EuWest2));
        assert_eq!("sa-east-1".parse(), Ok(Region::SaEast1));
        assert_eq!("us-east-1".parse(), Ok(Region::UsEast1));
        assert_eq!("us-east-2".parse(), Ok(Region::UsEast2));
        assert_eq!("us-west-1".parse(), Ok(Region::UsWest1));
        assert_eq!("us-west-2".parse(), Ok(Region::UsWest2));
        assert_eq!("cn-north-1".parse(), Ok(Region::CnNorth1));
    }

    #[test]
    fn region_display() {
        assert_eq!(Region::ApNortheast1.to_string(),
                   "ap-northeast-1".to_owned());
        assert_eq!(Region::ApNortheast2.to_string(),
                   "ap-northeast-2".to_owned());
        assert_eq!(Region::ApSouth1.to_string(), "ap-south-1".to_owned());
        assert_eq!(Region::ApSoutheast1.to_string(),
                   "ap-southeast-1".to_owned());
        assert_eq!(Region::ApSoutheast2.to_string(),
                   "ap-southeast-2".to_owned());
        assert_eq!(Region::CaCentral1.to_string(), "ca-central-1".to_owned());
        assert_eq!(Region::EuCentral1.to_string(), "eu-central-1".to_owned());
        assert_eq!(Region::EuWest1.to_string(), "eu-west-1".to_owned());
        assert_eq!(Region::EuWest2.to_string(), "eu-west-2".to_owned());
        assert_eq!(Region::SaEast1.to_string(), "sa-east-1".to_owned());
        assert_eq!(Region::UsEast1.to_string(), "us-east-1".to_owned());
        assert_eq!(Region::UsEast2.to_string(), "us-east-2".to_owned());
        assert_eq!(Region::UsWest1.to_string(), "us-west-1".to_owned());
        assert_eq!(Region::UsWest2.to_string(), "us-west-2".to_owned());
        assert_eq!(Region::CnNorth1.to_string(), "cn-north-1".to_owned());
    }

    #[test]
    fn region_serialize_deserialize() {
        assert_tokens(&Region::ApNortheast1, &[Token::String("ap-northeast-1")]);
        assert_tokens(&Region::ApNortheast2, &[Token::String("ap-northeast-2")]);
        assert_tokens(&Region::ApSouth1, &[Token::String("ap-south-1")]);
        assert_tokens(&Region::ApSoutheast1, &[Token::String("ap-southeast-1")]);
        assert_tokens(&Region::ApSoutheast2, &[Token::String("ap-southeast-2")]);
        assert_tokens(&Region::CaCentral1, &[Token::String("ca-central-1")]);
        assert_tokens(&Region::EuCentral1, &[Token::String("eu-central-1")]);
        assert_tokens(&Region::EuWest1, &[Token::String("eu-west-1")]);
        assert_tokens(&Region::EuWest2, &[Token::String("eu-west-2")]);
        assert_tokens(&Region::SaEast1, &[Token::String("sa-east-1")]);
        assert_tokens(&Region::UsEast1, &[Token::String("us-east-1")]);
        assert_tokens(&Region::UsEast2, &[Token::String("us-east-2")]);
        assert_tokens(&Region::UsWest1, &[Token::String("us-west-1")]);
        assert_tokens(&Region::UsWest2, &[Token::String("us-west-2")]);
        assert_tokens(&Region::CnNorth1, &[Token::String("cn-north-1")]);
    }

    #[test]
    fn region_serialize_custom() {
        let custom_region = Region::Custom("http://localhost:8000".to_owned());
        assert_ser_tokens(&custom_region, &[Token::String("http://localhost:8000")]);
    }

    // Test we can still deserialize from the old style to ease migration
    #[test]
    fn region_deserialize_migration() {
        assert_de_tokens(&Region::ApNortheast1, &[Token::String("ApNortheast1")]);
        assert_de_tokens(&Region::ApNortheast2, &[Token::String("ApNortheast2")]);
        assert_de_tokens(&Region::ApSouth1, &[Token::String("ApSouth1")]);
        assert_de_tokens(&Region::ApSoutheast1, &[Token::String("ApSoutheast1")]);
        assert_de_tokens(&Region::ApSoutheast2, &[Token::String("ApSoutheast2")]);
        assert_de_tokens(&Region::CaCentral1, &[Token::String("CaCentral1")]);
        assert_de_tokens(&Region::EuCentral1, &[Token::String("EuCentral1")]);
        assert_de_tokens(&Region::EuWest1, &[Token::String("EuWest1")]);
        assert_de_tokens(&Region::EuWest2, &[Token::String("EuWest2")]);
        assert_de_tokens(&Region::SaEast1, &[Token::String("SaEast1")]);
        assert_de_tokens(&Region::UsEast1, &[Token::String("UsEast1")]);
        assert_de_tokens(&Region::UsEast2, &[Token::String("UsEast2")]);
        assert_de_tokens(&Region::UsWest1, &[Token::String("UsWest1")]);
        assert_de_tokens(&Region::UsWest2, &[Token::String("UsWest2")]);
        assert_de_tokens(&Region::CnNorth1, &[Token::String("CnNorth1")]);
    }
}
