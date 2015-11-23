//! AWS Regions and helper functions
//!
//! Mostly used for translating the Region enum to a string AWS accepts.
//!
//! EG: UsEast1 to "us-east-1"

use std::str::FromStr;

/// AWS Region
#[derive(Debug,PartialEq)]
pub enum Region {
    UsEast1,
    UsWest1,
    UsWest2,
    EuWest1,
    EuCentral1,
    ApSoutheast1,
    ApNortheast1,
    ApSoutheast2,
    SaEast1,
}

#[derive(Debug,PartialEq)]
pub struct ParseRegionError;

impl FromStr for Region {
    type Err = ParseRegionError;

    fn from_str(s: &str) -> Result<Region, ParseRegionError> {
        match s {
            "us-east-1" => Ok(Region::UsEast1),
            "us-west-1" => Ok(Region::UsWest1),
            "us-west-2" => Ok(Region::UsWest2),
            "eu-west-1" => Ok(Region::EuWest1),
            "eu-central-1" => Ok(Region::EuCentral1),
            "ap-southeast-1" => Ok(Region::ApSoutheast1),
            "ap-northeast-1" => Ok(Region::ApNortheast1),
            "ap-southeast-2" => Ok(Region::ApSoutheast2),
            "sa-east-1" => Ok(Region::SaEast1),
            _ => Err(ParseRegionError)
        }
    }
}

/// Translates region enum into AWS format.  EG: us-east-1
pub fn region_in_aws_format(region: &Region) -> String {
    match region {
        &Region::UsEast1 => "us-east-1".to_string(),
        &Region::UsWest1 => "us-west-1".to_string(),
        &Region::UsWest2 => "us-west-2".to_string(),
        &Region::EuWest1 => "eu-west-1".to_string(),
        &Region::EuCentral1 => "eu-central-1".to_string(),
        &Region::ApSoutheast1 => "ap-southeast-1".to_string(),
        &Region::ApNortheast1 => "ap-northeast-1".to_string(),
        &Region::ApSoutheast2 => "ap-southeast-2".to_string(),
        &Region::SaEast1 => "sa-east-1".to_string(),
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    use std::str::FromStr;

    #[test]
    fn from_str_for_region() {
        assert_eq!(FromStr::from_str("us-east-1"), Ok(Region::UsEast1));
        assert_eq!(FromStr::from_str("us-west-1"), Ok(Region::UsWest1));
        assert_eq!(FromStr::from_str("us-west-2"), Ok(Region::UsWest2));
        assert_eq!(FromStr::from_str("eu-west-1"), Ok(Region::EuWest1));
        assert_eq!(FromStr::from_str("eu-central-1"), Ok(Region::EuCentral1));
    }

	#[test]
	fn regions_correctly_map_to_aws_strings() {
        let mut region = Region::UsEast1;
        if region_in_aws_format(&region) != "us-east-1" {
            panic!("Couldn't map us-east-1 enum right.");
        }
        region = Region::UsWest1;
        if region_in_aws_format(&region) != "us-west-1" {
            panic!("Couldn't map us-west-1 enum right.");
        }
        region = Region::UsWest2;
        if region_in_aws_format(&region) != "us-west-2" {
            panic!("Couldn't map us-west-2 enum right.");
        }
        region = Region::EuWest1;
        if region_in_aws_format(&region) != "eu-west-1" {
            panic!("Couldn't map eu-west-1 enum right.");
        }
        region = Region::EuCentral1;
        if region_in_aws_format(&region) != "eu-central-1" {
            panic!("Couldn't map eu-central-1 enum right.");
        }
        region = Region::ApSoutheast1;
        if region_in_aws_format(&region) != "ap-southeast-1" {
            panic!("Couldn't map ap-southeast-1 enum right.");
        }
        region = Region::ApNortheast1;
        if region_in_aws_format(&region) != "ap-northeast-1" {
            panic!("Couldn't map ap-northeast-1 enum right.");
        }
        region = Region::ApSoutheast2;
        if region_in_aws_format(&region) != "ap-southeast-2" {
            panic!("Couldn't map ap-southeast-2 enum right.");
        }
        region = Region::SaEast1;
        if region_in_aws_format(&region) != "sa-east-1" {
            panic!("Couldn't map sa-east-1 enum right.");
        }
    }
}
