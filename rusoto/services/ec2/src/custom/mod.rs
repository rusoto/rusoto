/// `Filter` structure is a bit verbose to set up, and since it is used in many places
/// `filter!` macro provides an ergononic way of creating it.
/// `filter!` expects its arguments to be `ToString`.
/// The first argument is the name field of the `Filter`, while the rest of them (if any) become the values.
///
/// `Filter` structure is used in various `Ec2Client::describe_*()` calls.
///
/// For example:
/// ```no_run
/// # use std::error::Error;
/// # use rusoto_ec2::{filter, DescribeAvailabilityZonesRequest, Ec2, Ec2Client};
/// # use rusoto_core::Region;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let filter = filter!("region-name", "us-east-1");
/// let input = DescribeAvailabilityZonesRequest {
///     filters: Some(vec![filter]),
///     ..DescribeAvailabilityZonesRequest::default()
/// };
///
/// let output = Ec2Client::new(Region::default())
///     .describe_availability_zones(input)
///     .await?;
/// #
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use rusoto_ec2::filter;
/// let empty = filter!("empty");
/// let direct = rusoto_ec2::Filter {
///     name: Some(String::from("empty")),
///     values: Some(vec![])
/// };
///
/// assert_eq!(empty, direct);
///
/// let single = filter!("owner", "bozo");
/// let direct = rusoto_ec2::Filter {
///     name: Some(String::from("owner")),
///     values: Some(vec![String::from("bozo")])
/// };
///
/// assert_eq!(single, direct);
///
/// let list = filter!("groups", "sg-xxxxx", "sg-yyyyy");
/// let direct = rusoto_ec2::Filter {
///     name: Some(String::from("groups")),
///     values: Some(
///         vec![
///             String::from("sg-xxxxx"),
///             String::from("sg-yyyyy"),
///         ])
/// };
///
/// assert_eq!(list, direct);
/// ```

#[macro_export]
macro_rules! filter {
    ($key:expr) => {
        $crate::Filter {
            name: Some($key.to_string()),
            values: Some(vec![])
        }
    };
    ($key:expr, $($value:expr),+) => {
        $crate::Filter {
            name: Some($key.to_string()),
            values: Some(vec![$($value.to_string()),+])
        }
    }
}
