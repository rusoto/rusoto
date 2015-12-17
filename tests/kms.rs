#![cfg(feature = "kms")]

extern crate rusoto;

use std::default::Default;

use rusoto::credentials::DefaultAWSCredentialsProviderChain;
use rusoto::kms::{
    CreateKeyRequest,
    DecryptRequest,
    GenerateDataKeyRequest,
    KMSClient,
    ScheduleKeyDeletionRequest,
};
use rusoto::regions::Region;

#[test]
fn test_kms() {
    let creds = DefaultAWSCredentialsProviderChain::new();
    let region = Region::UsWest2;

    let mut client = KMSClient::new(creds, &region);

    let create_key_request = CreateKeyRequest {
        Description: Some("Test key for the Rusoto integration test suite".to_owned()),
        ..Default::default()
    };

    let key_id = client.create_key(&create_key_request).expect(
        "Failed to create master key."
    ).KeyMetadata.expect(
        "CreateKeyResponse didn't have any metadata. \
        You might need to remove the master key manually."
    ).KeyId;

    let generate_data_key_request = GenerateDataKeyRequest {
        KeyId: key_id.clone(),
        KeySpec: Some("AES_256".to_owned()),
        ..Default::default()
    };

    let data_key_response = client.generate_data_key(&generate_data_key_request).expect(
        "Failed to create data key. \
        You might need to remove the master key manually."
    );

    let data_key = data_key_response.Plaintext.expect(
        "GenerateDataKeyResponse didn't have plaintext. \
        You might need to remove the master key manually."
    );

    let encrypted_data_key = data_key_response.CiphertextBlob.expect(
        "GenerateDataKeyResponse didn't have ciphertext. \
        You might need to remove the master key manually."
    );

    let decrypt_request = DecryptRequest {
        CiphertextBlob: encrypted_data_key,
        ..Default::default()
    };

    let decrypt_response = client.decrypt(&decrypt_request).expect(
        "Failed to decrypt data key. \
        You might need to remove the master key manually."
    );

    let data_key_from_decryption = decrypt_response.KeyId.expect(
        "DecryptResponse didn't have key id. \
        You might need to remove the master key manually."
    );

    assert!(
        data_key == data_key_from_decryption.into_bytes(),
        "Decrypted data key didn't match original data key! \
        You might need to remove the master key manually."
    );

    let schedule_key_deletion_request = ScheduleKeyDeletionRequest {
        PendingWindowInDays: Some(7),
        KeyId: key_id,
    };

    client.schedule_key_deletion(&schedule_key_deletion_request).expect(
        "Failed to schedule the test master key for deletion! \
        You might need to remove the master key manually."
    );
}
