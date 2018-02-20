#[cfg(test)]
mod tests {
    extern crate rusoto_mock;

    pub use generated::*;
    use self::rusoto_mock::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_and_extract_valid_cloudfront_list_distributions() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudfront-list-distributions.xml",
            );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client = CloudFrontClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListDistributionsRequest::default();
        let result = client.list_distributions(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
        assert!(result.unwrap().distribution_list.is_some(), "Should have a distribution list");
    }
}
