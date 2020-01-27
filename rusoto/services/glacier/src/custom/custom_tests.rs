extern crate rusoto_mock;

use crate::generated::*;

use self::rusoto_mock::*;
use rusoto_core::Region;

#[tokio::test]
async fn test_initiate_multipart_part_response() {
    let mock = MockRequestDispatcher::with_status(201)
        .with_header("Location", "/111122223333/vaults/examplevault/multipart-uploads/OW2fM5iVylEpFEMM9_HpKowRapC3vn5sSL39_396UW9zLFUWVrnRHaPjUJddQ5OxSHVXjYtrN47NBZ-khxOjyEXAMPLE")
        .with_header("x-amz-multipart-upload-id", "OW2fM5iVylEpFEMM9_HpKowRapC3vn5sSL39_396UW9zLFUWVrnRHaPjUJddQ5OxSHVXjYtrN47NBZ-khxOjyEXAMPLE");
    let client = GlacierClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);

    let initiate_multipart_upload_req = InitiateMultipartUploadInput {
        account_id: "111122223333".to_owned(),
        vault_name: "examplevault".to_owned(),
        ..Default::default()
    };
    let result = client
        .initiate_multipart_upload(initiate_multipart_upload_req)
        .await
        .expect("Should parse empty body");
    assert_eq!(result.location.unwrap(), "/111122223333/vaults/examplevault/multipart-uploads/OW2fM5iVylEpFEMM9_HpKowRapC3vn5sSL39_396UW9zLFUWVrnRHaPjUJddQ5OxSHVXjYtrN47NBZ-khxOjyEXAMPLE", "Should see location in response");
    assert_eq!(result.upload_id.unwrap(), "OW2fM5iVylEpFEMM9_HpKowRapC3vn5sSL39_396UW9zLFUWVrnRHaPjUJddQ5OxSHVXjYtrN47NBZ-khxOjyEXAMPLE", "Should see upload id in response");
}

#[tokio::test]
async fn test_upload_multipart_part_response() {
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
        .await
        .expect("Should parse empty body");
    assert_eq!(
        result.checksum.unwrap(),
        "42",
        "Should handle checksum in response"
    );
}
