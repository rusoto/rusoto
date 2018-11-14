extern crate rusoto_mock;

use ::*;

use rusoto_core::Region;
use self::rusoto_mock::*;

#[test]
fn test_upload_multipart_part_response() {
    let mock = MockRequestDispatcher::with_status(204).with_header("x-amz-sha256-tree-hash", "42");
    let client = GlacierClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);

    let upload_part_copy_req = UploadMultipartPartInput {
        account_id: "foo".to_owned(),
        upload_id: "bar".to_owned(),
        vault_name: "baz".to_owned(),
        ..Default::default()
    };
    let result = client
        .upload_multipart_part(upload_part_copy_req)
        .sync()
        .expect("Should parse empty body");
    assert_eq!(result.checksum.unwrap(), "42", "Should handle checksum in response");
}
