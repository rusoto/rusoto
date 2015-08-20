// Valid Values:
// [ us-west-1 | us-west-2 | eu-west-1 | eu-central-1 | ap-southeast-1 | ap-northeast-1 | ap-southeast-2 | sa-east-1 ]
pub fn region_is_valid(region: &str) -> bool {
    match region {
        "us-west-1" => return true,
        "us-west-2" => return true,
        "us-east-1" => return true,
        "eu-west-1" => return true,
        "eu-central-1" => return true,
        "ap-southeast-1" => return true,
        "ap-northeast-1" => return true,
        "ap-southeast-2" => return true,
        "sa-east-1" => return true,
        _ => return false
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid_regions_are_valid() {
        let valid_regions = vec!["us-west-1", "us-west-2", "eu-west-1", "eu-central-1", "ap-southeast-1", "ap-northeast-1", "ap-southeast-2", "sa-east-1"];
        for region in valid_regions.iter() {
            match region_is_valid(region) {
                true => continue,
                false => panic!("valid region returned as not valid"),
            }
        }
    }

    #[test]
	fn invalid_regions_are_invalid() {
        let valid_regions = vec!["us-west-100", "the-moon", "Jupiter"];
        for region in valid_regions.iter() {
            match region_is_valid(region) {
                true => panic!("invalid region returned as valid, broken!"),
                false => continue,
            }
        }
    }
}
