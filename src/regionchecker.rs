
// TODO: add tests
// Valid Values:
// [ us-west-1 | us-west-2 | eu-west-1 | eu-central-1 | ap-southeast-1 | ap-northeast-1 | ap-southeast-2 | sa-east-1 ]
pub fn region_is_valid(region: &str) -> bool {
    match region {
        "us-west-1" => return true,
        "us-west-2" => return true,
        "us-east-1" => return true, // not sure why this isn't valid, you have to not pass in the constraint to get us-east-1.  We'll handle it later.
        "eu-west-1" => return true,
        "eu-central-1" => return true,
        "ap-southeast-1" => return true,
        "ap-northeast-1" => return true,
        "ap-southeast-2" => return true,
        "sa-east-1" => return true,
        _ => return false
    }
}
