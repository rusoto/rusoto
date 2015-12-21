use std::collections::HashMap;
use std::error::Error;
use rustc_serialize::json;
use std::io::Read;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListRetirableGrantsRequest {
	/// Use this parameter only when paginating results and only in a subsequent
	/// request after you've received a response with truncated results. Set it to the
	/// value of `NextMarker` from the response you just received.
	pub Marker: Option<MarkerType>,
	/// When paginating results, specify the maximum number of items to return in the
	/// response. If additional items exist beyond the number you specify, the
	/// `Truncated` element in the response is set to true.
	/// This value is optional. If you include a value, it must be between 1 and 100,
	/// inclusive. If you do not include a value, it defaults to 50.
	pub Limit: Option<LimitType>,
	/// The retiring principal for which to list grants.
	/// To specify the retiring principal, use the [Amazon Resource Name
	/// (ARN)](http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-
	/// namespaces.html) of an AWS principal. Valid AWS principals include AWS
	/// accounts (root), IAM users, federated users, and assumed role users. For
	/// examples of the ARN syntax for specifying a principal, go to [AWS Identity and
	/// Access Management (IAM)](http://docs.aws.amazon.com/general/latest/gr/aws-
	/// arns-and-namespaces.html#arn-syntax-iam) in the Example ARNs section of the
	/// _Amazon Web Services General Reference_.
	pub RetiringPrincipal: PrincipalIdType,
}

/// The system timed out while trying to fulfill the request. The request can be
/// retried.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DependencyTimeoutException {
	pub message: Option<ErrorMessageType>,
}

/// The request was rejected because a specified parameter is not supported.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct UnsupportedOperationException {
	pub message: Option<ErrorMessageType>,
}

/// The request was rejected because a specified ARN was not valid.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct InvalidArnException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct EnableKeyRotationRequest {
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
}

pub type LimitType = i32;
/// The request was rejected because the specified KeySpec parameter is not valid.
/// The currently supported value is ENCRYPT/DECRYPT.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct InvalidKeyUsageException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateKeyRequest {
	/// Policy to attach to the key. This is required and delegates back to the
	/// account. The key is the root of trust. The policy size limit is 32 KiB (32768
	/// bytes).
	pub Policy: Option<PolicyType>,
	/// Specifies the intended use of the key. Currently this defaults to
	/// ENCRYPT/DECRYPT, and only symmetric encryption and decryption are supported.
	pub KeyUsage: Option<KeyUsageType>,
	/// Description of the key. We recommend that you choose a description that helps
	/// your customer decide whether the key is appropriate for a task.
	pub Description: Option<DescriptionType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetKeyPolicyResponse {
	/// A policy document in JSON format.
	pub Policy: Option<PolicyType>,
}

pub type DataKeySpec = String;
pub type EncryptionContextValue = String;
pub type KeyIdType = String;
/// The request was rejected because the specified key was marked as disabled.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DisabledException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GenerateDataKeyWithoutPlaintextResponse {
	/// System generated unique identifier of the key to be used to decrypt the
	/// encrypted copy of the data key.
	pub KeyId: Option<KeyIdType>,
	/// Ciphertext that contains the wrapped data key. You must store the blob and
	/// encryption context so that the key can be used in a future decrypt operation.
	/// If you are using the CLI, the value is Base64 encoded. Otherwise, it is not
	/// encoded.
	pub CiphertextBlob: Option<CiphertextType>,
}

pub type AWSAccountIdType = String;
pub type NumberOfBytesType = i32;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DescribeKeyRequest {
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier, a fully specified ARN to either an alias or a key, or an
	/// alias name prefixed by "alias/".
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	///   * Alias Name Example - alias/MyAliasName
	pub KeyId: KeyIdType,
	/// A list of grant tokens.
	/// For more information, go to [Grant Tokens](http://docs.aws.amazon.com/kms/late
	/// st/developerguide/concepts.html#grant_token) in the _AWS Key Management
	/// Service Developer Guide_.
	pub GrantTokens: Option<GrantTokenList>,
}

pub type KeyState = String;
/// The request was rejected because the specified `GrantId` is not valid.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct InvalidGrantIdException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct EnableKeyRequest {
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetKeyRotationStatusRequest {
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct UpdateAliasRequest {
	/// Unique identifier of the customer master key to be mapped to the alias. This
	/// value can be a globally unique identifier or the fully specified ARN of a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	/// You can call ListAliases to verify that the alias is mapped to the correct
	/// `TargetKeyId`.
	pub TargetKeyId: KeyIdType,
	/// String that contains the name of the alias to be modified. The name must start
	/// with the word "alias" followed by a forward slash (alias/). Aliases that begin
	/// with "alias/aws" are reserved.
	pub AliasName: AliasNameType,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GenerateDataKeyRequest {
	/// Integer that contains the number of bytes to generate. Common values are 128,
	/// 256, 512, and 1024. 1024 is the current limit. We recommend that you use the
	/// `KeySpec` parameter instead.
	pub NumberOfBytes: Option<NumberOfBytesType>,
	/// Name/value pair that contains additional data to be authenticated during the
	/// encryption and decryption processes that use the key. This value is logged by
	/// AWS CloudTrail to provide context around the data encrypted by the key.
	pub EncryptionContext: Option<EncryptionContextType>,
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier, a fully specified ARN to either an alias or a key, or an
	/// alias name prefixed by "alias/".
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	///   * Alias Name Example - alias/MyAliasName
	pub KeyId: KeyIdType,
	/// A list of grant tokens.
	/// For more information, go to [Grant Tokens](http://docs.aws.amazon.com/kms/late
	/// st/developerguide/concepts.html#grant_token) in the _AWS Key Management
	/// Service Developer Guide_.
	pub GrantTokens: Option<GrantTokenList>,
	/// Value that identifies the encryption algorithm and key size to generate a data
	/// key for. Currently this can be AES_128 or AES_256.
	pub KeySpec: Option<DataKeySpec>,
}

/// The request was rejected because the state of the specified resource is not
/// valid for this request.
/// For more information about how key state affects the use of a customer master
/// key (CMK), go to [How Key State Affects the Use of a Customer Master
/// Key](http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html) in
/// the _AWS Key Management Service Developer Guide_.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct KMSInvalidStateException {
	pub message: Option<ErrorMessageType>,
}

pub type CiphertextType = Vec<u8>;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GenerateDataKeyWithoutPlaintextRequest {
	/// Integer that contains the number of bytes to generate. Common values are 128,
	/// 256, 512, 1024 and so on. We recommend that you use the `KeySpec` parameter
	/// instead.
	pub NumberOfBytes: Option<NumberOfBytesType>,
	/// Name:value pair that contains additional data to be authenticated during the
	/// encryption and decryption processes.
	pub EncryptionContext: Option<EncryptionContextType>,
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier, a fully specified ARN to either an alias or a key, or an
	/// alias name prefixed by "alias/".
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	///   * Alias Name Example - alias/MyAliasName
	pub KeyId: KeyIdType,
	/// A list of grant tokens.
	/// For more information, go to [Grant Tokens](http://docs.aws.amazon.com/kms/late
	/// st/developerguide/concepts.html#grant_token) in the _AWS Key Management
	/// Service Developer Guide_.
	pub GrantTokens: Option<GrantTokenList>,
	/// Value that identifies the encryption algorithm and key size. Currently this
	/// can be AES_128 or AES_256.
	pub KeySpec: Option<DataKeySpec>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ScheduleKeyDeletionResponse {
	/// The unique identifier of the customer master key (CMK) for which deletion is
	/// scheduled.
	pub KeyId: Option<KeyIdType>,
	/// The date and time after which AWS KMS deletes the customer master key (CMK).
	pub DeletionDate: Option<DateType>,
}

pub type PrincipalIdType = String;
/// The request was rejected because the specified entity or resource could not be
/// found.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct NotFoundException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GenerateRandomResponse {
	/// Plaintext that contains the unpredictable byte string.
	pub Plaintext: Option<PlaintextType>,
}

pub type AliasList = Vec<AliasListEntry>;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ReEncryptRequest {
	/// Encryption context used to encrypt and decrypt the data specified in the
	/// `CiphertextBlob` parameter.
	pub SourceEncryptionContext: Option<EncryptionContextType>,
	/// A list of grant tokens.
	/// For more information, go to [Grant Tokens](http://docs.aws.amazon.com/kms/late
	/// st/developerguide/concepts.html#grant_token) in the _AWS Key Management
	/// Service Developer Guide_.
	pub GrantTokens: Option<GrantTokenList>,
	/// Encryption context to be used when the data is re-encrypted.
	pub DestinationEncryptionContext: Option<EncryptionContextType>,
	/// A unique identifier for the customer master key used to re-encrypt the data.
	/// This value can be a globally unique identifier, a fully specified ARN to
	/// either an alias or a key, or an alias name prefixed by "alias/".
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	///   * Alias Name Example - alias/MyAliasName
	pub DestinationKeyId: KeyIdType,
	/// Ciphertext of the data to re-encrypt.
	pub CiphertextBlob: CiphertextType,
}

/// The request was rejected because a grant token provided as part of the request
/// is invalid.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct InvalidGrantTokenException {
	pub message: Option<ErrorMessageType>,
}

pub type GrantNameType = String;
/// The request was rejected because a limit was exceeded. For more information,
/// see [Limits](http://docs.aws.amazon.com/kms/latest/developerguide/limits.html)
/// in the _AWS Key Management Service Developer Guide_.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct LimitExceededException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListAliasesResponse {
	/// A flag that indicates whether there are more items in the list. If your
	/// results were truncated, you can use the `Marker` parameter to make a
	/// subsequent pagination request to retrieve more items in the list.
	pub Truncated: Option<BooleanType>,
	/// When `Truncated` is true, this value is present and contains the value to use
	/// for the `Marker` parameter in a subsequent pagination request.
	pub NextMarker: Option<MarkerType>,
	/// A list of key aliases in the user's account.
	pub Aliases: Option<AliasList>,
}

/// The request was rejected because the marker that specifies where pagination
/// should next begin is not valid.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct InvalidMarkerException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListKeyPoliciesRequest {
	/// Use this parameter only when paginating results and only in a subsequent
	/// request after you've received a response with truncated results. Set it to the
	/// value of `NextMarker` from the response you just received.
	pub Marker: Option<MarkerType>,
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier, a fully specified ARN to either an alias or a key, or an
	/// alias name prefixed by "alias/".
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	///   * Alias Name Example - alias/MyAliasName
	pub KeyId: KeyIdType,
	/// When paginating results, specify the maximum number of items to return in the
	/// response. If additional items exist beyond the number you specify, the
	/// `Truncated` element in the response is set to true.
	/// This value is optional. If you include a value, it must be between 1 and 1000,
	/// inclusive. If you do not include a value, it defaults to 100.
	/// Currently only 1 policy can be attached to a key.
	pub Limit: Option<LimitType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteAliasRequest {
	/// The alias to be deleted. The name must start with the word "alias" followed by
	/// a forward slash (alias/). Aliases that begin with "alias/AWS" are reserved.
	pub AliasName: AliasNameType,
}

/// The request was rejected because the specified policy is not syntactically or
/// semantically correct.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MalformedPolicyDocumentException {
	pub message: Option<ErrorMessageType>,
}

pub type DateType = f64;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DecryptResponse {
	/// Decrypted plaintext data. This value may not be returned if the customer
	/// master key is not available or if you didn't have permission to use it.
	pub Plaintext: Option<PlaintextType>,
	/// ARN of the key used to perform the decryption. This value is returned if no
	/// errors are encountered during the operation.
	pub KeyId: Option<KeyIdType>,
}

/// A structure for specifying the conditions under which the operations permitted
/// by the grant are allowed.
/// You can use this structure to allow the operations permitted by the grant only
/// when a specified encryption context is present. For more information about
/// encryption context, see [Encryption
/// Context](http://docs.aws.amazon.com/kms/latest/developerguide/encrypt-
/// context.html) in the _AWS Key Management Service Developer Guide_.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GrantConstraints {
	/// Contains a list of key-value pairs, a subset of which must be present in the
	/// encryption context of a subsequent operation permitted by the grant. When a
	/// subsequent operation permitted by the grant includes an encryption context
	/// that matches this list or is a subset of this list, the grant allows the
	/// operation. Otherwise, the operation is not allowed.
	pub EncryptionContextSubset: Option<EncryptionContextType>,
	/// Contains a list of key-value pairs that must be present in the encryption
	/// context of a subsequent operation permitted by the grant. When a subsequent
	/// operation permitted by the grant includes an encryption context that matches
	/// this list, the grant allows the operation. Otherwise, the operation is not
	/// allowed.
	pub EncryptionContextEquals: Option<EncryptionContextType>,
}

pub type GrantIdType = String;
pub type GrantList = Vec<GrantListEntry>;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct RetireGrantRequest {
	/// Token that identifies the grant to be retired.
	pub GrantToken: Option<GrantTokenType>,
	/// A unique identifier for the customer master key associated with the grant.
	/// This value can be a globally unique identifier or a fully specified ARN of the
	/// key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: Option<KeyIdType>,
	/// Unique identifier of the grant to be retired. The grant ID is returned by the
	/// `CreateGrant` function.
	///   * Grant ID Example - 0123456789012345678901234567890123456789012345678901234567890123
	pub GrantId: Option<GrantIdType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListKeysResponse {
	/// A list of keys.
	pub Keys: Option<KeyList>,
	/// When `Truncated` is true, this value is present and contains the value to use
	/// for the `Marker` parameter in a subsequent pagination request.
	pub NextMarker: Option<MarkerType>,
	/// A flag that indicates whether there are more items in the list. If your
	/// results were truncated, you can use the `Marker` parameter to make a
	/// subsequent pagination request to retrieve more items in the list.
	pub Truncated: Option<BooleanType>,
}

pub type GrantTokenList = Vec<GrantTokenType>;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct EncryptResponse {
	/// The ID of the key used during encryption.
	pub KeyId: Option<KeyIdType>,
	/// The encrypted plaintext. If you are using the CLI, the value is Base64
	/// encoded. Otherwise, it is not encoded.
	pub CiphertextBlob: Option<CiphertextType>,
}

pub type PlaintextType = Vec<u8>;
/// The request was rejected because it attempted to create a resource that
/// already exists.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct AlreadyExistsException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GenerateDataKeyResponse {
	/// Plaintext that contains the data key. Use this for encryption and decryption
	/// and then remove it from memory as soon as possible.
	pub Plaintext: Option<PlaintextType>,
	/// System generated unique identifier of the key to be used to decrypt the
	/// encrypted copy of the data key.
	pub KeyId: Option<KeyIdType>,
	/// Ciphertext that contains the encrypted data key. You must store the blob and
	/// enough information to reconstruct the encryption context so that the data
	/// encrypted by using the key can later be decrypted. You must provide both the
	/// ciphertext blob and the encryption context to the Decrypt API to recover the
	/// plaintext data key and decrypt the object.
	/// If you are using the CLI, the value is Base64 encoded. Otherwise, it is not
	/// encoded.
	pub CiphertextBlob: Option<CiphertextType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListKeyPoliciesResponse {
	/// A list of policy names. Currently, there is only one policy and it is named
	/// "Default".
	pub PolicyNames: Option<PolicyNameList>,
	/// When `Truncated` is true, this value is present and contains the value to use
	/// for the `Marker` parameter in a subsequent pagination request.
	pub NextMarker: Option<MarkerType>,
	/// A flag that indicates whether there are more items in the list. If your
	/// results were truncated, you can use the `Marker` parameter to make a
	/// subsequent pagination request to retrieve more items in the list.
	pub Truncated: Option<BooleanType>,
}

pub type PolicyNameList = Vec<PolicyNameType>;
pub type GrantOperationList = Vec<GrantOperation>;
pub type PendingWindowInDaysType = i32;
pub type BooleanType = bool;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateKeyResponse {
	/// Metadata associated with the key.
	pub KeyMetadata: Option<KeyMetadata>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ReEncryptResponse {
	/// Unique identifier of the key used to originally encrypt the data.
	pub SourceKeyId: Option<KeyIdType>,
	/// Unique identifier of the key used to re-encrypt the data.
	pub KeyId: Option<KeyIdType>,
	/// The re-encrypted data. If you are using the CLI, the value is Base64 encoded.
	/// Otherwise, it is not encoded.
	pub CiphertextBlob: Option<CiphertextType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateAliasRequest {
	/// An identifier of the key for which you are creating the alias. This value
	/// cannot be another alias but can be a globally unique identifier or a fully
	/// specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub TargetKeyId: KeyIdType,
	/// String that contains the display name. The name must start with the word
	/// "alias" followed by a forward slash (alias/). Aliases that begin with
	/// "alias/AWS" are reserved.
	pub AliasName: AliasNameType,
}

/// The request was rejected because the key was not available. The request can be
/// retried.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct KeyUnavailableException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CancelKeyDeletionRequest {
	/// The unique identifier for the customer master key (CMK) for which to cancel
	/// deletion.
	/// To specify this value, use the unique key ID or the Amazon Resource Name (ARN)
	/// of the CMK. Examples:
	///   * Unique key ID: 1234abcd-12ab-34cd-56ef-1234567890ab
	///   * Key ARN: arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab
	/// To obtain the unique key ID and key ARN for a given CMK, use ListKeys or
	/// DescribeKey.
	pub KeyId: KeyIdType,
}

pub type GrantOperation = String;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetKeyRotationStatusResponse {
	/// A Boolean value that specifies whether key rotation is enabled.
	pub KeyRotationEnabled: Option<BooleanType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct EncryptRequest {
	/// Data to be encrypted.
	pub Plaintext: PlaintextType,
	/// Name/value pair that specifies the encryption context to be used for
	/// authenticated encryption. If used here, the same value must be supplied to the
	/// `Decrypt` API or decryption will fail. For more information, see [Encryption
	/// Context](http://docs.aws.amazon.com/kms/latest/developerguide/encrypt-
	/// context.html).
	pub EncryptionContext: Option<EncryptionContextType>,
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier, a fully specified ARN to either an alias or a key, or an
	/// alias name prefixed by "alias/".
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	///   * Alias Name Example - alias/MyAliasName
	pub KeyId: KeyIdType,
	/// A list of grant tokens.
	/// For more information, go to [Grant Tokens](http://docs.aws.amazon.com/kms/late
	/// st/developerguide/concepts.html#grant_token) in the _AWS Key Management
	/// Service Developer Guide_.
	pub GrantTokens: Option<GrantTokenList>,
}

/// Contains information about each entry in the key list.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct KeyListEntry {
	/// ARN of the key.
	pub KeyArn: Option<ArnType>,
	/// Unique identifier of the key.
	pub KeyId: Option<KeyIdType>,
}

pub type EncryptionContextType = HashMap<EncryptionContextKey,EncryptionContextValue>;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListKeysRequest {
	/// Use this parameter only when paginating results and only in a subsequent
	/// request after you've received a response with truncated results. Set it to the
	/// value of `NextMarker` from the response you just received.
	pub Marker: Option<MarkerType>,
	/// When paginating results, specify the maximum number of items to return in the
	/// response. If additional items exist beyond the number you specify, the
	/// `Truncated` element in the response is set to true.
	/// This value is optional. If you include a value, it must be between 1 and 1000,
	/// inclusive. If you do not include a value, it defaults to 100.
	pub Limit: Option<LimitType>,
}

/// Contains metadata about a customer master key (CMK).
/// This data type is used as a response element for the CreateKey and DescribeKey
/// operations.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct KeyMetadata {
	/// The globally unique identifier for the key.
	pub KeyId: KeyIdType,
	/// The friendly description of the key.
	pub Description: Option<DescriptionType>,
	/// The date and time after which AWS KMS deletes the customer master key (CMK).
	/// This value is present only when `KeyState` is `PendingDeletion`, otherwise
	/// this value is null.
	pub DeletionDate: Option<DateType>,
	/// Specifies whether the key is enabled. When `KeyState` is `Enabled` this value
	/// is true, otherwise it is false.
	pub Enabled: Option<BooleanType>,
	/// The cryptographic operations for which you can use the key. Currently the only
	/// allowed value is `ENCRYPT_DECRYPT`, which means you can use the key for the
	/// Encrypt and Decrypt operations.
	pub KeyUsage: Option<KeyUsageType>,
	/// The state of the customer master key (CMK).
	/// For more information about how key state affects the use of a CMK, go to [How
	/// Key State Affects the Use of a Customer Master
	/// Key](http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html) in
	/// the _AWS Key Management Service Developer Guide_.
	pub KeyState: Option<KeyState>,
	/// The date and time when the key was created.
	pub CreationDate: Option<DateType>,
	/// The Amazon Resource Name (ARN) of the key. For examples, see [AWS Key
	/// Management Service (AWS KMS)](http://docs.aws.amazon.com/general/latest/gr
	/// /aws-arns-and-namespaces.html#arn-syntax-kms) in the Example ARNs section of
	/// the _AWS General Reference_.
	pub Arn: Option<ArnType>,
	/// The twelve-digit account ID of the AWS account that owns the key.
	pub AWSAccountId: Option<AWSAccountIdType>,
}

pub type DescriptionType = String;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListGrantsRequest {
	/// Use this parameter only when paginating results and only in a subsequent
	/// request after you've received a response with truncated results. Set it to the
	/// value of `NextMarker` from the response you just received.
	pub Marker: Option<MarkerType>,
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
	/// When paginating results, specify the maximum number of items to return in the
	/// response. If additional items exist beyond the number you specify, the
	/// `Truncated` element in the response is set to true.
	/// This value is optional. If you include a value, it must be between 1 and 100,
	/// inclusive. If you do not include a value, it defaults to 50.
	pub Limit: Option<LimitType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DecryptRequest {
	/// The encryption context. If this was specified in the Encrypt function, it must
	/// be specified here or the decryption operation will fail. For more information,
	/// see [Encryption Context](http://docs.aws.amazon.com/kms/latest/developerguide
	/// /encrypt-context.html).
	pub EncryptionContext: Option<EncryptionContextType>,
	/// A list of grant tokens.
	/// For more information, go to [Grant Tokens](http://docs.aws.amazon.com/kms/late
	/// st/developerguide/concepts.html#grant_token) in the _AWS Key Management
	/// Service Developer Guide_.
	pub GrantTokens: Option<GrantTokenList>,
	/// Ciphertext to be decrypted. The blob includes metadata.
	pub CiphertextBlob: CiphertextType,
}

pub type PolicyType = String;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CancelKeyDeletionResponse {
	/// The unique identifier of the master key for which deletion is canceled.
	pub KeyId: Option<KeyIdType>,
}

pub type KeyList = Vec<KeyListEntry>;
/// Contains information about an alias.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct AliasListEntry {
	/// String that contains the key ARN.
	pub AliasArn: Option<ArnType>,
	/// String that contains the alias.
	pub AliasName: Option<AliasNameType>,
	/// String that contains the key identifier pointed to by the alias.
	pub TargetKeyId: Option<KeyIdType>,
}

pub type GrantTokenType = String;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetKeyPolicyRequest {
	/// String that contains the name of the policy. Currently, this must be
	/// "default". Policy names can be discovered by calling ListKeyPolicies.
	pub PolicyName: PolicyNameType,
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
}

pub type ErrorMessageType = String;
pub type MarkerType = String;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct UpdateKeyDescriptionRequest {
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
	/// New description for the key.
	pub Description: DescriptionType,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutKeyPolicyRequest {
	/// The policy to attach to the key. This is required and delegates back to the
	/// account. The key is the root of trust. The policy size limit is 32 KiB (32768
	/// bytes).
	pub Policy: PolicyType,
	/// Name of the policy to be attached. Currently, the only supported name is
	/// "default".
	pub PolicyName: PolicyNameType,
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct RevokeGrantRequest {
	/// A unique identifier for the customer master key associated with the grant.
	/// This value can be a globally unique identifier or the fully specified ARN to a
	/// key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
	/// Identifier of the grant to be revoked.
	pub GrantId: GrantIdType,
}

pub type ArnType = String;
/// The request was rejected because the specified alias name is not valid.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct InvalidAliasNameException {
	pub message: Option<ErrorMessageType>,
}

pub type PolicyNameType = String;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GenerateRandomRequest {
	/// Integer that contains the number of bytes to generate. Common values are 128,
	/// 256, 512, 1024 and so on. The current limit is 1024 bytes.
	pub NumberOfBytes: Option<NumberOfBytesType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ScheduleKeyDeletionRequest {
	/// The waiting period, specified in number of days. After the waiting period
	/// ends, AWS KMS deletes the customer master key (CMK).
	/// This value is optional. If you include a value, it must be between 7 and 30,
	/// inclusive. If you do not include a value, it defaults to 30.
	pub PendingWindowInDays: Option<PendingWindowInDaysType>,
	/// The unique identifier for the customer master key (CMK) to delete.
	/// To specify this value, use the unique key ID or the Amazon Resource Name (ARN)
	/// of the CMK. Examples:
	///   * Unique key ID: 1234abcd-12ab-34cd-56ef-1234567890ab
	///   * Key ARN: arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab
	/// To obtain the unique key ID and key ARN for a given CMK, use ListKeys or
	/// DescribeKey.
	pub KeyId: KeyIdType,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DisableKeyRequest {
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateGrantRequest {
	/// A list of operations that the grant permits. The list can contain any
	/// combination of one or more of the following values:
	///   * Decrypt
	///   * Encrypt
	///   * GenerateDataKey
	///   * GenerateDataKeyWithoutPlaintext
	///   * ReEncryptFrom
	///   * ReEncryptTo
	///   * CreateGrant
	///   * RetireGrant
	pub Operations: Option<GrantOperationList>,
	/// The unique identifier for the customer master key (CMK) that the grant applies
	/// to.
	/// To specify this value, use the globally unique key ID or the Amazon Resource
	/// Name (ARN) of the key. Examples:
	///   * Globally unique key ID: 12345678-1234-1234-1234-123456789012
	///   * Key ARN: arn:aws:kms:us-west-2:123456789012:key/12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
	/// A friendly name for identifying the grant. Use this value to prevent
	/// unintended creation of duplicate grants when retrying this request.
	/// When this value is absent, all `CreateGrant` requests result in a new grant
	/// with a unique `GrantId` even if all the supplied parameters are identical.
	/// This can result in unintended duplicates when you retry the `CreateGrant`
	/// request.
	/// When this value is present, you can retry a `CreateGrant` request with
	/// identical parameters; if the grant already exists, the original `GrantId` is
	/// returned without creating a new grant. Note that the returned grant token is
	/// unique with every `CreateGrant` request, even when a duplicate `GrantId` is
	/// returned. All grant tokens obtained in this way can be used interchangeably.
	pub Name: Option<GrantNameType>,
	/// The principal that is given permission to retire the grant by using
	/// RetireGrant operation.
	/// To specify the principal, use the [Amazon Resource Name
	/// (ARN)](http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-
	/// namespaces.html) of an AWS principal. Valid AWS principals include AWS
	/// accounts (root), IAM users, federated users, and assumed role users. For
	/// examples of the ARN syntax to use for specifying a principal, see [AWS
	/// Identity and Access Management
	/// (IAM)](http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-
	/// namespaces.html#arn-syntax-iam) in the Example ARNs section of the _AWS
	/// General Reference_.
	pub RetiringPrincipal: Option<PrincipalIdType>,
	/// The principal that is given permission to perform the operations that the
	/// grant permits.
	/// To specify the principal, use the [Amazon Resource Name
	/// (ARN)](http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-
	/// namespaces.html) of an AWS principal. Valid AWS principals include AWS
	/// accounts (root), IAM users, federated users, and assumed role users. For
	/// examples of the ARN syntax to use for specifying a principal, see [AWS
	/// Identity and Access Management
	/// (IAM)](http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-
	/// namespaces.html#arn-syntax-iam) in the Example ARNs section of the _AWS
	/// General Reference_.
	pub GranteePrincipal: PrincipalIdType,
	/// A list of grant tokens.
	/// For more information, go to [Grant Tokens](http://docs.aws.amazon.com/kms/late
	/// st/developerguide/concepts.html#grant_token) in the _AWS Key Management
	/// Service Developer Guide_.
	pub GrantTokens: Option<GrantTokenList>,
	/// The conditions under which the operations permitted by the grant are allowed.
	/// You can use this value to allow the operations permitted by the grant only
	/// when a specified encryption context is present. For more information, see
	/// [Encryption Context](http://docs.aws.amazon.com/kms/latest/developerguide
	/// /encrypt-context.html) in the _AWS Key Management Service Developer Guide_.
	pub Constraints: Option<GrantConstraints>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListAliasesRequest {
	/// Use this parameter only when paginating results and only in a subsequent
	/// request after you've received a response with truncated results. Set it to the
	/// value of `NextMarker` from the response you just received.
	pub Marker: Option<MarkerType>,
	/// When paginating results, specify the maximum number of items to return in the
	/// response. If additional items exist beyond the number you specify, the
	/// `Truncated` element in the response is set to true.
	/// This value is optional. If you include a value, it must be between 1 and 100,
	/// inclusive. If you do not include a value, it defaults to 50.
	pub Limit: Option<LimitType>,
}

/// The request was rejected because the specified ciphertext has been corrupted
/// or is otherwise invalid.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct InvalidCiphertextException {
	pub message: Option<ErrorMessageType>,
}

pub type KeyUsageType = String;
/// Contains information about an entry in a list of grants.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GrantListEntry {
	/// The list of operations permitted by the grant.
	pub Operations: Option<GrantOperationList>,
	/// The unique identifier for the customer master key (CMK) to which the grant
	/// applies.
	pub KeyId: Option<KeyIdType>,
	/// The friendly name that identifies the grant. If a name was provided in the
	/// CreateGrant request, that name is returned. Otherwise this value is null.
	pub Name: Option<GrantNameType>,
	/// The principal that can retire the grant.
	pub RetiringPrincipal: Option<PrincipalIdType>,
	/// The principal that receives the grant's permissions.
	pub GranteePrincipal: Option<PrincipalIdType>,
	/// The unique identifier for the grant.
	pub GrantId: Option<GrantIdType>,
	/// The AWS account under which the grant was issued.
	pub IssuingAccount: Option<PrincipalIdType>,
	/// The date and time when the grant was created.
	pub CreationDate: Option<DateType>,
	/// The conditions under which the grant's operations are allowed.
	pub Constraints: Option<GrantConstraints>,
}

pub type EncryptionContextKey = String;
pub type AliasNameType = String;
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateGrantResponse {
	/// The grant token.
	/// For more information about using grant tokens, see [Grant Tokens](http://docs.
	/// aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token) in the
	/// _AWS Key Management Service Developer Guide_.
	pub GrantToken: Option<GrantTokenType>,
	/// The unique identifier for the grant.
	/// You can use the `GrantId` in a subsequent RetireGrant or RevokeGrant
	/// operation.
	pub GrantId: Option<GrantIdType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ListGrantsResponse {
	/// When `Truncated` is true, this value is present and contains the value to use
	/// for the `Marker` parameter in a subsequent pagination request.
	pub NextMarker: Option<MarkerType>,
	/// A list of grants.
	pub Grants: Option<GrantList>,
	/// A flag that indicates whether there are more items in the list. If your
	/// results were truncated, you can use the `Marker` parameter to make a
	/// subsequent pagination request to retrieve more items in the list.
	pub Truncated: Option<BooleanType>,
}

/// The request was rejected because an internal exception occurred. The request
/// can be retried.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct KMSInternalException {
	pub message: Option<ErrorMessageType>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DescribeKeyResponse {
	/// Metadata associated with the key.
	pub KeyMetadata: Option<KeyMetadata>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DisableKeyRotationRequest {
	/// A unique identifier for the customer master key. This value can be a globally
	/// unique identifier or the fully specified ARN to a key.
	///   * Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012
	///   * Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012
	pub KeyId: KeyIdType,
}

pub struct KMSClient<'a> {
	creds: Box<AWSCredentialsProvider + 'a>,
	region: &'a Region
}

impl<'a> KMSClient<'a> { 
	pub fn new<P: AWSCredentialsProvider + 'a>(creds: P, region: &'a Region) -> KMSClient<'a> {
		KMSClient { creds: Box::new(creds), region: region }
	}
	/// Encrypts plaintext into ciphertext by using a customer master key. The
	/// `Encrypt` function has two primary use cases:
	///   * You can encrypt up to 4 KB of arbitrary data such as an RSA key, a database password, or other sensitive customer information.
	///   * If you are moving encrypted data from one region to another, you can use this API to encrypt in the new region the plaintext data key that was used to encrypt the data in the original region. This provides you with an encrypted copy of the data key that can be decrypted in the new region and used there to decrypt the encrypted data. 
	/// Unless you are moving encrypted data from one region to another, you don't use
	/// this function to encrypt a generated data key within a region. You retrieve
	/// data keys already encrypted by calling the GenerateDataKey or
	/// GenerateDataKeyWithoutPlaintext function. Data keys don't need to be encrypted
	/// again by calling `Encrypt`.
	/// If you want to encrypt data locally in your application, you can use the
	/// `GenerateDataKey` function to return a plaintext data encryption key and a
	/// copy of the key encrypted under the customer master key (CMK) of your
	/// choosing.
	pub fn encrypt(&mut self, input: &EncryptRequest) -> Result<EncryptResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.Encrypt");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: EncryptResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Updates an alias to map it to a different key.
	/// An alias is not a property of a key. Therefore, an alias can be mapped to and
	/// unmapped from an existing key without changing the properties of the key.
	/// An alias name can contain only alphanumeric characters, forward slashes (/),
	/// underscores (_), and dashes (-). An alias must start with the word "alias"
	/// followed by a forward slash (alias/). An alias that begins with "aws" after
	/// the forward slash (alias/aws...) is reserved by Amazon Web Services (AWS).
	/// The alias and the key it is mapped to must be in the same AWS account and the
	/// same region.
	pub fn update_alias(&mut self, input: &UpdateAliasRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.UpdateAlias");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Sets the state of a master key to disabled, thereby preventing its use for
	/// cryptographic operations. For more information about how key state affects the
	/// use of a master key, go to [How Key State Affects the Use of a Customer Master
	/// Key](http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html) in
	/// the _AWS Key Management Service Developer Guide_.
	pub fn disable_key(&mut self, input: &DisableKeyRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.DisableKey");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Returns a data key encrypted by a customer master key without the plaintext
	/// copy of that key. Otherwise, this API functions exactly like GenerateDataKey.
	/// You can use this API to, for example, satisfy an audit requirement that an
	/// encrypted key be made available without exposing the plaintext copy of that
	/// key.
	pub fn generate_data_key_without_plaintext(&mut self, input: &GenerateDataKeyWithoutPlaintextRequest) -> Result<GenerateDataKeyWithoutPlaintextResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.GenerateDataKeyWithoutPlaintext");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: GenerateDataKeyWithoutPlaintextResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Generates a data key that you can use in your application to locally encrypt
	/// data. This call returns a plaintext version of the key in the `Plaintext`
	/// field of the response object and an encrypted copy of the key in the
	/// `CiphertextBlob` field. The key is encrypted by using the master key specified
	/// by the `KeyId` field. To decrypt the encrypted key, pass it to the `Decrypt`
	/// API.
	/// We recommend that you use the following pattern to locally encrypt data: call
	/// the `GenerateDataKey` API, use the key returned in the `Plaintext` response
	/// field to locally encrypt data, and then erase the plaintext data key from
	/// memory. Store the encrypted data key (contained in the `CiphertextBlob` field)
	/// alongside of the locally encrypted data.
	/// You should not call the `Encrypt` function to re-encrypt your data keys within
	/// a region. `GenerateDataKey` always returns the data key encrypted and tied to
	/// the customer master key that will be used to decrypt it. There is no need to
	/// decrypt it twice.
	/// If you decide to use the optional `EncryptionContext` parameter, you must also
	/// store the context in full or at least store enough information along with the
	/// encrypted data to be able to reconstruct the context when submitting the
	/// ciphertext to the `Decrypt` API. It is a good practice to choose a context
	/// that you can reconstruct on the fly to better secure the ciphertext. For more
	/// information about how this parameter is used, see [Encryption
	/// Context](http://docs.aws.amazon.com/kms/latest/developerguide/encrypt-
	/// context.html).
	/// To decrypt data, pass the encrypted data key to the `Decrypt` API. `Decrypt`
	/// uses the associated master key to decrypt the encrypted data key and returns
	/// it as plaintext. Use the plaintext data key to locally decrypt your data and
	/// then erase the key from memory. You must specify the encryption context, if
	/// any, that you specified when you generated the key. The encryption context is
	/// logged by CloudTrail, and you can use this log to help track the use of
	/// particular data.
	pub fn generate_data_key(&mut self, input: &GenerateDataKeyRequest) -> Result<GenerateDataKeyResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.GenerateDataKey");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: GenerateDataKeyResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Lists all of the key aliases in the account.
	pub fn list_aliases(&mut self, input: &ListAliasesRequest) -> Result<ListAliasesResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.ListAliases");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ListAliasesResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Cancels the deletion of a customer master key (CMK). When this operation is
	/// successful, the CMK is set to the `Disabled` state. To enable a CMK, use
	/// EnableKey.
	/// For more information about scheduling and canceling deletion of a CMK, go to
	/// [Deleting Customer Master
	/// Keys](http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html)
	/// in the _AWS Key Management Service Developer Guide_.
	pub fn cancel_key_deletion(&mut self, input: &CancelKeyDeletionRequest) -> Result<CancelKeyDeletionResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.CancelKeyDeletion");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: CancelKeyDeletionResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Retrieves a Boolean value that indicates whether key rotation is enabled for
	/// the specified key.
	pub fn get_key_rotation_status(&mut self, input: &GetKeyRotationStatusRequest) -> Result<GetKeyRotationStatusResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.GetKeyRotationStatus");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: GetKeyRotationStatusResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Retrieves a list of policies attached to a key.
	pub fn list_key_policies(&mut self, input: &ListKeyPoliciesRequest) -> Result<ListKeyPoliciesResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.ListKeyPolicies");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ListKeyPoliciesResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Deletes the specified alias. To map an alias to a different key, call
	/// UpdateAlias.
	pub fn delete_alias(&mut self, input: &DeleteAliasRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.DeleteAlias");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Retires a grant. You can retire a grant when you're done using it to clean up.
	/// You should revoke a grant when you intend to actively deny operations that
	/// depend on it. The following are permitted to call this API:
	///   * The account that created the grant
	///   * The `RetiringPrincipal`, if present
	///   * The `GranteePrincipal`, if `RetireGrant` is a grantee operation
	/// The grant to retire must be identified by its grant token or by a combination
	/// of the key ARN and the grant ID. A grant token is a unique variable-length
	/// base64-encoded string. A grant ID is a 64 character unique identifier of a
	/// grant. Both are returned by the `CreateGrant` function.
	pub fn retire_grant(&mut self, input: &RetireGrantRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.RetireGrant");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Provides detailed information about the specified customer master key.
	pub fn describe_key(&mut self, input: &DescribeKeyRequest) -> Result<DescribeKeyResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.DescribeKey");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: DescribeKeyResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Lists the customer master keys.
	pub fn list_keys(&mut self, input: &ListKeysRequest) -> Result<ListKeysResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.ListKeys");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ListKeysResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Generates an unpredictable byte string.
	pub fn generate_random(&mut self, input: &GenerateRandomRequest) -> Result<GenerateRandomResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.GenerateRandom");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: GenerateRandomResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Retrieves a policy attached to the specified key.
	pub fn get_key_policy(&mut self, input: &GetKeyPolicyRequest) -> Result<GetKeyPolicyResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.GetKeyPolicy");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: GetKeyPolicyResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Adds a grant to a key to specify who can use the key and under what
	/// conditions. Grants are alternate permission mechanisms to key policies.
	/// For more information about grants, see
	/// [Grants](http://docs.aws.amazon.com/kms/latest/developerguide/grants.html) in
	/// the _AWS Key Management Service Developer Guide_.
	pub fn create_grant(&mut self, input: &CreateGrantRequest) -> Result<CreateGrantResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.CreateGrant");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: CreateGrantResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Creates a customer master key. Customer master keys can be used to encrypt
	/// small amounts of data (less than 4K) directly, but they are most commonly used
	/// to encrypt or envelope data keys that are then used to encrypt customer data.
	/// For more information about data keys, see GenerateDataKey and
	/// GenerateDataKeyWithoutPlaintext.
	pub fn create_key(&mut self, input: &CreateKeyRequest) -> Result<CreateKeyResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.CreateKey");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: CreateKeyResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Encrypts data on the server side with a new customer master key without
	/// exposing the plaintext of the data on the client side. The data is first
	/// decrypted and then encrypted. This operation can also be used to change the
	/// encryption context of a ciphertext.
	/// Unlike other actions, `ReEncrypt` is authorized twice - once as
	/// `ReEncryptFrom` on the source key and once as `ReEncryptTo` on the destination
	/// key. We therefore recommend that you include the `"action":"kms:ReEncrypt*"`
	/// statement in your key policies to permit re-encryption from or to the key. The
	/// statement is included automatically when you authorize use of the key through
	/// the console but must be included manually when you set a policy by using the
	/// PutKeyPolicy function.
	pub fn re_encrypt(&mut self, input: &ReEncryptRequest) -> Result<ReEncryptResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.ReEncrypt");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ReEncryptResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Disables rotation of the specified key.
	pub fn disable_key_rotation(&mut self, input: &DisableKeyRotationRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.DisableKeyRotation");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Returns a list of all grants for which the grant's `RetiringPrincipal` matches
	/// the one specified.
	/// A typical use is to list all grants that you are able to retire. To retire a
	/// grant, use RetireGrant.
	pub fn list_retirable_grants(&mut self, input: &ListRetirableGrantsRequest) -> Result<ListGrantsResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.ListRetirableGrants");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ListGrantsResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Schedules the deletion of a customer master key (CMK). You may provide a
	/// waiting period, specified in days, before deletion occurs. If you do not
	/// provide a waiting period, the default period of 30 days is used. When this
	/// operation is successful, the state of the CMK changes to `PendingDeletion`.
	/// Before the waiting period ends, you can use CancelKeyDeletion to cancel the
	/// deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK
	/// and all AWS KMS data associated with it, including all aliases that point to
	/// it.
	/// Deleting a CMK is a destructive and potentially dangerous operation. When a
	/// CMK is deleted, all data that was encrypted under the CMK is rendered
	/// unrecoverable. To restrict the use of a CMK without deleting it, use
	/// DisableKey.
	/// For more information about scheduling a CMK for deletion, go to [Deleting
	/// Customer Master Keys](http://docs.aws.amazon.com/kms/latest/developerguide
	/// /deleting-keys.html) in the _AWS Key Management Service Developer Guide_.
	pub fn schedule_key_deletion(&mut self, input: &ScheduleKeyDeletionRequest) -> Result<ScheduleKeyDeletionResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.ScheduleKeyDeletion");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ScheduleKeyDeletionResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Creates a display name for a customer master key. An alias can be used to
	/// identify a key and should be unique. The console enforces a one-to-one mapping
	/// between the alias and a key. An alias name can contain only alphanumeric
	/// characters, forward slashes (/), underscores (_), and dashes (-). An alias
	/// must start with the word "alias" followed by a forward slash (alias/). An
	/// alias that begins with "aws" after the forward slash (alias/aws...) is
	/// reserved by Amazon Web Services (AWS).
	/// The alias and the key it is mapped to must be in the same AWS account and the
	/// same region.
	/// To map an alias to a different key, call UpdateAlias.
	pub fn create_alias(&mut self, input: &CreateAliasRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.CreateAlias");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Enables rotation of the specified customer master key.
	pub fn enable_key_rotation(&mut self, input: &EnableKeyRotationRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.EnableKeyRotation");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// List the grants for a specified key.
	pub fn list_grants(&mut self, input: &ListGrantsRequest) -> Result<ListGrantsResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.ListGrants");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: ListGrantsResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Marks a key as enabled, thereby permitting its use.
	pub fn enable_key(&mut self, input: &EnableKeyRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.EnableKey");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Decrypts ciphertext. Ciphertext is plaintext that has been previously
	/// encrypted by using any of the following functions:
	///   * GenerateDataKey
	///   * GenerateDataKeyWithoutPlaintext
	///   * Encrypt
	/// Note that if a caller has been granted access permissions to all keys
	/// (through, for example, IAM user policies that grant `Decrypt` permission on
	/// all resources), then ciphertext encrypted by using keys in other accounts
	/// where the key grants access to the caller can be decrypted. To remedy this, we
	/// recommend that you do not grant `Decrypt` access in an IAM user policy.
	/// Instead grant `Decrypt` access only in key policies. If you must grant
	/// `Decrypt` access in an IAM user policy, you should scope the resource to
	/// specific keys or to specific trusted accounts.
	pub fn decrypt(&mut self, input: &DecryptRequest) -> Result<DecryptResponse> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.Decrypt");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				let decoded: DecryptResponse = json::decode(&body).unwrap();
				Ok(decoded)
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Updates the description of a key.
	pub fn update_key_description(&mut self, input: &UpdateKeyDescriptionRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.UpdateKeyDescription");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Revokes a grant. You can revoke a grant to actively deny operations that
	/// depend on it.
	pub fn revoke_grant(&mut self, input: &RevokeGrantRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.RevokeGrant");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
	/// Attaches a policy to the specified key.
	pub fn put_key_policy(&mut self, input: &PutKeyPolicyRequest) -> Result<()> {
		let encoded = json::encode(&input).unwrap();
		let mut request = SignedRequest::new("POST", "kms", &self.region, "/");
		request.set_content_type("application/x-amz-json-1.0".to_string());
		request.add_header("x-amz-target", "TrentService.PutKeyPolicy");
		request.set_payload(Some(encoded.as_bytes()));
		let mut result = request.sign_and_execute(try!(self.creds.get_credentials()));
		let status = result.status.to_u16();
		let mut body = String::new();
		result.read_to_string(&mut body).unwrap();
		match status {
			200 => { 
				Ok(())
			}
			_ => {
				Err(parse_error(&body))
			}
		}
	}
}

