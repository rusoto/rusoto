#[allow(warnings)]
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
pub type AWSAccountIdType = String;
pub type AlgorithmSpec = String;
pub type AliasList = Vec<AliasListEntry>;
#[doc="<p>Contains information about an alias.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AliasListEntry {
    #[doc="<p>String that contains the key ARN.</p>"]
    #[serde(rename="AliasArn")]
    pub alias_arn: Option<ArnType>,
    #[doc="<p>String that contains the alias.</p>"]
    #[serde(rename="AliasName")]
    pub alias_name: Option<AliasNameType>,
    #[doc="<p>String that contains the key identifier referred to by the alias.</p>"]
    #[serde(rename="TargetKeyId")]
    pub target_key_id: Option<KeyIdType>,
}

pub type AliasNameType = String;
pub type ArnType = String;
pub type BooleanType = bool;
#[derive(Default,Debug,Clone,Serialize)]
pub struct CancelKeyDeletionRequest {
    #[doc="<p>The unique identifier for the customer master key (CMK) for which to cancel deletion.</p> <p>To specify this value, use the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: 1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> <li> <p>Key ARN: arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> </ul> <p>To obtain the unique key ID and key ARN for a given CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CancelKeyDeletionResponse {
    #[doc="<p>The unique identifier of the master key for which deletion is canceled.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
}

pub type CiphertextType = Vec<u8>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateAliasRequest {
    #[doc="<p>String that contains the display name. The name must start with the word \"alias\" followed by a forward slash (alias/). Aliases that begin with \"alias/AWS\" are reserved.</p>"]
    #[serde(rename="AliasName")]
    pub alias_name: AliasNameType,
    #[doc="<p>An identifier of the key for which you are creating the alias. This value cannot be another alias but can be a globally unique identifier or a fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="TargetKeyId")]
    pub target_key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateGrantRequest {
    #[doc="<p>A structure that you can use to allow certain operations in the grant only when the desired encryption context is present. For more information about encryption context, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html\">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="Constraints")]
    pub constraints: Option<GrantConstraints>,
    #[doc="<p>A list of grant tokens.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token\">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="GrantTokens")]
    pub grant_tokens: Option<GrantTokenList>,
    #[doc="<p>The principal that is given permission to perform the operations that the grant permits.</p> <p>To specify the principal, use the <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, IAM roles, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam\">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>"]
    #[serde(rename="GranteePrincipal")]
    pub grantee_principal: PrincipalIdType,
    #[doc="<p>The unique identifier for the customer master key (CMK) that the grant applies to.</p> <p>To specify this value, use the globally unique key ID or the Amazon Resource Name (ARN) of the key. Examples:</p> <ul> <li> <p>Globally unique key ID: 12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Key ARN: arn:aws:kms:us-west-2:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>A friendly name for identifying the grant. Use this value to prevent unintended creation of duplicate grants when retrying this request.</p> <p>When this value is absent, all <code>CreateGrant</code> requests result in a new grant with a unique <code>GrantId</code> even if all the supplied parameters are identical. This can result in unintended duplicates when you retry the <code>CreateGrant</code> request.</p> <p>When this value is present, you can retry a <code>CreateGrant</code> request with identical parameters; if the grant already exists, the original <code>GrantId</code> is returned without creating a new grant. Note that the returned grant token is unique with every <code>CreateGrant</code> request, even when a duplicate <code>GrantId</code> is returned. All grant tokens obtained in this way can be used interchangeably.</p>"]
    #[serde(rename="Name")]
    pub name: Option<GrantNameType>,
    #[doc="<p>A list of operations that the grant permits.</p>"]
    #[serde(rename="Operations")]
    pub operations: Option<GrantOperationList>,
    #[doc="<p>The principal that is given permission to retire the grant by using <a>RetireGrant</a> operation.</p> <p>To specify the principal, use the <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam\">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>"]
    #[serde(rename="RetiringPrincipal")]
    pub retiring_principal: Option<PrincipalIdType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateGrantResponse {
    #[doc="<p>The unique identifier for the grant.</p> <p>You can use the <code>GrantId</code> in a subsequent <a>RetireGrant</a> or <a>RevokeGrant</a> operation.</p>"]
    #[serde(rename="GrantId")]
    pub grant_id: Option<GrantIdType>,
    #[doc="<p>The grant token.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token\">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="GrantToken")]
    pub grant_token: Option<GrantTokenType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateKeyRequest {
    #[doc="<p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the likelihood that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam\">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you include a policy in the request and you intend to prevent the principal that is making the request from making a subsequent <a>PutKeyPolicy</a> request on the CMK.</p> <p>The default value is false.</p>"]
    #[serde(rename="BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<BooleanType>,
    #[doc="<p>A description of the CMK.</p> <p>Use a description that helps you decide whether the CMK is appropriate for a task.</p>"]
    #[serde(rename="Description")]
    pub description: Option<DescriptionType>,
    #[doc="<p>The intended use of the CMK.</p> <p>You can use CMKs only for symmetric encryption and decryption.</p>"]
    #[serde(rename="KeyUsage")]
    pub key_usage: Option<KeyUsageType>,
    #[doc="<p>The source of the CMK's key material.</p> <p>The default is <code>AWS_KMS</code>, which means AWS KMS creates the key material. When this parameter is set to <code>EXTERNAL</code>, the request creates a CMK without key material so that you can import key material from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html\">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK's <code>Origin</code> is immutable and is set when the CMK is created.</p>"]
    #[serde(rename="Origin")]
    pub origin: Option<OriginType>,
    #[doc="<p>The key policy to attach to the CMK.</p> <p>If you specify a policy and do not set <code>BypassPolicyLockoutSafetyCheck</code> to true, the policy must meet the following criteria:</p> <ul> <li> <p>It must allow the principal that is making the <code>CreateKey</code> request to make a subsequent <a>PutKeyPolicy</a> request on the CMK. This reduces the likelihood that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam\">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>The principals that are specified in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before specifying the new principal in a key policy because the new principal might not immediately be visible to AWS KMS. For more information, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency\">Changes that I make are not always immediately visible</a> in the <i>IAM User Guide</i>.</p> </li> </ul> <p>If you do not specify a policy, AWS KMS attaches a default key policy to the CMK. For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default\">Default Key Policy</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The policy size limit is 32 KiB (32768 bytes).</p>"]
    #[serde(rename="Policy")]
    pub policy: Option<PolicyType>,
    #[doc="<p>One or more tags. Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>Use this parameter to tag the CMK when it is created. Alternately, you can omit this parameter and instead tag the CMK after it is created using <a>TagResource</a>.</p>"]
    #[serde(rename="Tags")]
    pub tags: Option<TagList>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateKeyResponse {
    #[doc="<p>Metadata associated with the CMK.</p>"]
    #[serde(rename="KeyMetadata")]
    pub key_metadata: Option<KeyMetadata>,
}

pub type DataKeySpec = String;
pub type DateType = f64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DecryptRequest {
    #[doc="<p>Ciphertext to be decrypted. The blob includes metadata.</p>"]
    #[serde(rename="CiphertextBlob")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub ciphertext_blob: CiphertextType,
    #[doc="<p>The encryption context. If this was specified in the <a>Encrypt</a> function, it must be specified here or the decryption operation will fail. For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html\">Encryption Context</a>.</p>"]
    #[serde(rename="EncryptionContext")]
    pub encryption_context: Option<EncryptionContextType>,
    #[doc="<p>A list of grant tokens.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token\">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="GrantTokens")]
    pub grant_tokens: Option<GrantTokenList>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DecryptResponse {
    #[doc="<p>ARN of the key used to perform the decryption. This value is returned if no errors are encountered during the operation.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
    #[doc="<p>Decrypted plaintext data. This value may not be returned if the customer master key is not available or if you didn't have permission to use it.</p>"]
    #[serde(rename="Plaintext")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub plaintext: Option<PlaintextType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteAliasRequest {
    #[doc="<p>The alias to be deleted. The name must start with the word \"alias\" followed by a forward slash (alias/). Aliases that begin with \"alias/AWS\" are reserved.</p>"]
    #[serde(rename="AliasName")]
    pub alias_name: AliasNameType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteImportedKeyMaterialRequest {
    #[doc="<p>The identifier of the CMK whose key material to delete. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>A valid identifier is the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeKeyRequest {
    #[doc="<p>A list of grant tokens.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token\">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="GrantTokens")]
    pub grant_tokens: Option<GrantTokenList>,
    #[doc="<p>A unique identifier for the customer master key. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by \"alias/\".</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Alias Name Example - alias/MyAliasName</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeKeyResponse {
    #[doc="<p>Metadata associated with the key.</p>"]
    #[serde(rename="KeyMetadata")]
    pub key_metadata: Option<KeyMetadata>,
}

pub type DescriptionType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DisableKeyRequest {
    #[doc="<p>A unique identifier for the CMK.</p> <p>Use the CMK's unique identifier or its Amazon Resource Name (ARN). For example:</p> <ul> <li> <p>Unique ID: 1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> <li> <p>ARN: arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DisableKeyRotationRequest {
    #[doc="<p>A unique identifier for the customer master key. This value can be a globally unique identifier or the fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct EnableKeyRequest {
    #[doc="<p>A unique identifier for the customer master key. This value can be a globally unique identifier or the fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct EnableKeyRotationRequest {
    #[doc="<p>A unique identifier for the customer master key. This value can be a globally unique identifier or the fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct EncryptRequest {
    #[doc="<p>Name-value pair that specifies the encryption context to be used for authenticated encryption. If used here, the same value must be supplied to the <code>Decrypt</code> API or decryption will fail. For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html\">Encryption Context</a>.</p>"]
    #[serde(rename="EncryptionContext")]
    pub encryption_context: Option<EncryptionContextType>,
    #[doc="<p>A list of grant tokens.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token\">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="GrantTokens")]
    pub grant_tokens: Option<GrantTokenList>,
    #[doc="<p>A unique identifier for the customer master key. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by \"alias/\".</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Alias Name Example - alias/MyAliasName</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>Data to be encrypted.</p>"]
    #[serde(rename="Plaintext")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub plaintext: PlaintextType,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct EncryptResponse {
    #[doc="<p>The encrypted plaintext. If you are using the CLI, the value is Base64 encoded. Otherwise, it is not encoded.</p>"]
    #[serde(rename="CiphertextBlob")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub ciphertext_blob: Option<CiphertextType>,
    #[doc="<p>The ID of the key used during encryption.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
}

pub type EncryptionContextKey = String;
pub type EncryptionContextType = ::std::collections::HashMap<EncryptionContextKey,
                                                             EncryptionContextValue>;
pub type EncryptionContextValue = String;
pub type ErrorMessageType = String;
pub type ExpirationModelType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct GenerateDataKeyRequest {
    #[doc="<p>A set of key-value pairs that represents additional authenticated data.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html\">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="EncryptionContext")]
    pub encryption_context: Option<EncryptionContextType>,
    #[doc="<p>A list of grant tokens.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token\">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="GrantTokens")]
    pub grant_tokens: Option<GrantTokenList>,
    #[doc="<p>The identifier of the CMK under which to generate and encrypt the data encryption key.</p> <p>A valid identifier is the unique key ID or the Amazon Resource Name (ARN) of the CMK, or the alias name or ARN of an alias that refers to the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>CMK ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>The length of the data encryption key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>"]
    #[serde(rename="KeySpec")]
    pub key_spec: Option<DataKeySpec>,
    #[doc="<p>The length of the data encryption key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p>"]
    #[serde(rename="NumberOfBytes")]
    pub number_of_bytes: Option<NumberOfBytesType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GenerateDataKeyResponse {
    #[doc="<p>The encrypted data encryption key.</p>"]
    #[serde(rename="CiphertextBlob")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub ciphertext_blob: Option<CiphertextType>,
    #[doc="<p>The identifier of the CMK under which the data encryption key was generated and encrypted.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
    #[doc="<p>The data encryption key. Use this data key for local encryption and decryption, then remove it from memory as soon as possible.</p>"]
    #[serde(rename="Plaintext")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub plaintext: Option<PlaintextType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GenerateDataKeyWithoutPlaintextRequest {
    #[doc="<p>A set of key-value pairs that represents additional authenticated data.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html\">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="EncryptionContext")]
    pub encryption_context: Option<EncryptionContextType>,
    #[doc="<p>A list of grant tokens.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token\">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="GrantTokens")]
    pub grant_tokens: Option<GrantTokenList>,
    #[doc="<p>The identifier of the CMK under which to generate and encrypt the data encryption key.</p> <p>A valid identifier is the unique key ID or the Amazon Resource Name (ARN) of the CMK, or the alias name or ARN of an alias that refers to the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>CMK ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>The length of the data encryption key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>"]
    #[serde(rename="KeySpec")]
    pub key_spec: Option<DataKeySpec>,
    #[doc="<p>The length of the data encryption key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p>"]
    #[serde(rename="NumberOfBytes")]
    pub number_of_bytes: Option<NumberOfBytesType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    #[doc="<p>The encrypted data encryption key.</p>"]
    #[serde(rename="CiphertextBlob")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub ciphertext_blob: Option<CiphertextType>,
    #[doc="<p>The identifier of the CMK under which the data encryption key was generated and encrypted.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GenerateRandomRequest {
    #[doc="<p>The length of the byte string.</p>"]
    #[serde(rename="NumberOfBytes")]
    pub number_of_bytes: Option<NumberOfBytesType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GenerateRandomResponse {
    #[doc="<p>The random byte string.</p>"]
    #[serde(rename="Plaintext")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub plaintext: Option<PlaintextType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetKeyPolicyRequest {
    #[doc="<p>A unique identifier for the customer master key. This value can be a globally unique identifier or the fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>String that contains the name of the policy. Currently, this must be \"default\". Policy names can be discovered by calling <a>ListKeyPolicies</a>.</p>"]
    #[serde(rename="PolicyName")]
    pub policy_name: PolicyNameType,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetKeyPolicyResponse {
    #[doc="<p>A policy document in JSON format.</p>"]
    #[serde(rename="Policy")]
    pub policy: Option<PolicyType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetKeyRotationStatusRequest {
    #[doc="<p>A unique identifier for the customer master key. This value can be a globally unique identifier or the fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetKeyRotationStatusResponse {
    #[doc="<p>A Boolean value that specifies whether key rotation is enabled.</p>"]
    #[serde(rename="KeyRotationEnabled")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub key_rotation_enabled: Option<BooleanType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetParametersForImportRequest {
    #[doc="<p>The identifier of the CMK into which you will import key material. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>A valid identifier is the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>The algorithm you will use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>. For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-encrypt-key-material.html\">Encrypt the Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="WrappingAlgorithm")]
    pub wrapping_algorithm: AlgorithmSpec,
    #[doc="<p>The type of wrapping key (public key) to return in the response. Only 2048-bit RSA public keys are supported.</p>"]
    #[serde(rename="WrappingKeySpec")]
    pub wrapping_key_spec: WrappingKeySpec,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetParametersForImportResponse {
    #[doc="<p>The import token to send in a subsequent <a>ImportKeyMaterial</a> request.</p>"]
    #[serde(rename="ImportToken")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub import_token: Option<CiphertextType>,
    #[doc="<p>The identifier of the CMK to use in a subsequent <a>ImportKeyMaterial</a> request. This is the same CMK specified in the <code>GetParametersForImport</code> request.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
    #[doc="<p>The time at which the import token and public key are no longer valid. After this time, you cannot use them to make an <a>ImportKeyMaterial</a> request and you must send another <code>GetParametersForImport</code> request to retrieve new ones.</p>"]
    #[serde(rename="ParametersValidTo")]
    pub parameters_valid_to: Option<DateType>,
    #[doc="<p>The public key to use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>.</p>"]
    #[serde(rename="PublicKey")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub public_key: Option<PlaintextType>,
}

#[doc="<p>A structure that you can use to allow certain operations in the grant only when the desired encryption context is present. For more information about encryption context, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html\">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Grant constraints apply only to operations that accept encryption context as input. For example, the <code> <a>DescribeKey</a> </code> operation does not accept encryption context as input. A grant that allows the <code>DescribeKey</code> operation does so regardless of the grant constraints. In constrast, the <code> <a>Encrypt</a> </code> operation accepts encryption context as input. A grant that allows the <code>Encrypt</code> operation does so only when the encryption context of the <code>Encrypt</code> operation satisfies the grant constraints.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct GrantConstraints {
    #[doc="<p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows. When certain subsequent operations allowed by the grant include encryption context that matches this list, the grant allows the operation. Otherwise, the grant does not allow the operation.</p>"]
    #[serde(rename="EncryptionContextEquals")]
    pub encryption_context_equals: Option<EncryptionContextType>,
    #[doc="<p>A list of key-value pairs, all of which must be present in the encryption context of certain subsequent operations that the grant allows. When certain subsequent operations allowed by the grant include encryption context that matches this list or is a superset of this list, the grant allows the operation. Otherwise, the grant does not allow the operation.</p>"]
    #[serde(rename="EncryptionContextSubset")]
    pub encryption_context_subset: Option<EncryptionContextType>,
}

pub type GrantIdType = String;
pub type GrantList = Vec<GrantListEntry>;
#[doc="<p>Contains information about an entry in a list of grants.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GrantListEntry {
    #[doc="<p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>"]
    #[serde(rename="Constraints")]
    pub constraints: Option<GrantConstraints>,
    #[doc="<p>The date and time when the grant was created.</p>"]
    #[serde(rename="CreationDate")]
    pub creation_date: Option<DateType>,
    #[doc="<p>The unique identifier for the grant.</p>"]
    #[serde(rename="GrantId")]
    pub grant_id: Option<GrantIdType>,
    #[doc="<p>The principal that receives the grant's permissions.</p>"]
    #[serde(rename="GranteePrincipal")]
    pub grantee_principal: Option<PrincipalIdType>,
    #[doc="<p>The AWS account under which the grant was issued.</p>"]
    #[serde(rename="IssuingAccount")]
    pub issuing_account: Option<PrincipalIdType>,
    #[doc="<p>The unique identifier for the customer master key (CMK) to which the grant applies.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
    #[doc="<p>The friendly name that identifies the grant. If a name was provided in the <a>CreateGrant</a> request, that name is returned. Otherwise this value is null.</p>"]
    #[serde(rename="Name")]
    pub name: Option<GrantNameType>,
    #[doc="<p>The list of operations permitted by the grant.</p>"]
    #[serde(rename="Operations")]
    pub operations: Option<GrantOperationList>,
    #[doc="<p>The principal that can retire the grant.</p>"]
    #[serde(rename="RetiringPrincipal")]
    pub retiring_principal: Option<PrincipalIdType>,
}

pub type GrantNameType = String;
pub type GrantOperation = String;
pub type GrantOperationList = Vec<GrantOperation>;
pub type GrantTokenList = Vec<GrantTokenType>;
pub type GrantTokenType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ImportKeyMaterialRequest {
    #[doc="<p>The encrypted key material to import. It must be encrypted with the public key that you received in the response to a previous <a>GetParametersForImport</a> request, using the wrapping algorithm that you specified in that request.</p>"]
    #[serde(rename="EncryptedKeyMaterial")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub encrypted_key_material: CiphertextType,
    #[doc="<p>Specifies whether the key material expires. The default is <code>KEY_MATERIAL_EXPIRES</code>, in which case you must include the <code>ValidTo</code> parameter. When this parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>, you must omit the <code>ValidTo</code> parameter.</p>"]
    #[serde(rename="ExpirationModel")]
    pub expiration_model: Option<ExpirationModelType>,
    #[doc="<p>The import token that you received in the response to a previous <a>GetParametersForImport</a> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p>"]
    #[serde(rename="ImportToken")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub import_token: CiphertextType,
    #[doc="<p>The identifier of the CMK to import the key material into. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>A valid identifier is the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. You must omit this parameter when the <code>ExpirationModel</code> parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>. Otherwise it is required.</p>"]
    #[serde(rename="ValidTo")]
    pub valid_to: Option<DateType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ImportKeyMaterialResponse;

pub type KeyIdType = String;
pub type KeyList = Vec<KeyListEntry>;
#[doc="<p>Contains information about each entry in the key list.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct KeyListEntry {
    #[doc="<p>ARN of the key.</p>"]
    #[serde(rename="KeyArn")]
    pub key_arn: Option<ArnType>,
    #[doc="<p>Unique identifier of the key.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
}

#[doc="<p>Contains metadata about a customer master key (CMK).</p> <p>This data type is used as a response element for the <a>CreateKey</a> and <a>DescribeKey</a> operations.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct KeyMetadata {
    #[doc="<p>The twelve-digit account ID of the AWS account that owns the CMK.</p>"]
    #[serde(rename="AWSAccountId")]
    pub aws_account_id: Option<AWSAccountIdType>,
    #[doc="<p>The Amazon Resource Name (ARN) of the CMK. For examples, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms\">AWS Key Management Service (AWS KMS)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>"]
    #[serde(rename="Arn")]
    pub arn: Option<ArnType>,
    #[doc="<p>The date and time when the CMK was created.</p>"]
    #[serde(rename="CreationDate")]
    pub creation_date: Option<DateType>,
    #[doc="<p>The date and time after which AWS KMS deletes the CMK. This value is present only when <code>KeyState</code> is <code>PendingDeletion</code>, otherwise this value is omitted.</p>"]
    #[serde(rename="DeletionDate")]
    pub deletion_date: Option<DateType>,
    #[doc="<p>The description of the CMK.</p>"]
    #[serde(rename="Description")]
    pub description: Option<DescriptionType>,
    #[doc="<p>Specifies whether the CMK is enabled. When <code>KeyState</code> is <code>Enabled</code> this value is true, otherwise it is false.</p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub enabled: Option<BooleanType>,
    #[doc="<p>Specifies whether the CMK's key material expires. This value is present only when <code>Origin</code> is <code>EXTERNAL</code>, otherwise this value is omitted.</p>"]
    #[serde(rename="ExpirationModel")]
    pub expiration_model: Option<ExpirationModelType>,
    #[doc="<p>The globally unique identifier for the CMK.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>The state of the CMK.</p> <p>For more information about how key state affects the use of a CMK, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html\">How Key State Affects the Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="KeyState")]
    pub key_state: Option<KeyState>,
    #[doc="<p>The cryptographic operations for which you can use the CMK. Currently the only allowed value is <code>ENCRYPT_DECRYPT</code>, which means you can use the CMK for the <a>Encrypt</a> and <a>Decrypt</a> operations.</p>"]
    #[serde(rename="KeyUsage")]
    pub key_usage: Option<KeyUsageType>,
    #[doc="<p>The source of the CMK's key material. When this value is <code>AWS_KMS</code>, AWS KMS created the key material. When this value is <code>EXTERNAL</code>, the key material was imported from your existing key management infrastructure or the CMK lacks key material.</p>"]
    #[serde(rename="Origin")]
    pub origin: Option<OriginType>,
    #[doc="<p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. This value is present only for CMKs whose <code>Origin</code> is <code>EXTERNAL</code> and whose <code>ExpirationModel</code> is <code>KEY_MATERIAL_EXPIRES</code>, otherwise this value is omitted.</p>"]
    #[serde(rename="ValidTo")]
    pub valid_to: Option<DateType>,
}

pub type KeyState = String;
pub type KeyUsageType = String;
pub type LimitType = i64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListAliasesRequest {
    #[doc="<p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<LimitType>,
    #[doc="<p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>"]
    #[serde(rename="Marker")]
    pub marker: Option<MarkerType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListAliasesResponse {
    #[doc="<p>A list of key aliases in the user's account.</p>"]
    #[serde(rename="Aliases")]
    pub aliases: Option<AliasList>,
    #[doc="<p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="NextMarker")]
    pub next_marker: Option<MarkerType>,
    #[doc="<p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To retrieve more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="Truncated")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub truncated: Option<BooleanType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListGrantsRequest {
    #[doc="<p>A unique identifier for the customer master key. This value can be a globally unique identifier or the fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<LimitType>,
    #[doc="<p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>"]
    #[serde(rename="Marker")]
    pub marker: Option<MarkerType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListGrantsResponse {
    #[doc="<p>A list of grants.</p>"]
    #[serde(rename="Grants")]
    pub grants: Option<GrantList>,
    #[doc="<p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="NextMarker")]
    pub next_marker: Option<MarkerType>,
    #[doc="<p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To retrieve more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="Truncated")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub truncated: Option<BooleanType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListKeyPoliciesRequest {
    #[doc="<p>A unique identifier for the customer master key (CMK). You can use the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p> <p>Currently only 1 policy can be attached to a key.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<LimitType>,
    #[doc="<p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>"]
    #[serde(rename="Marker")]
    pub marker: Option<MarkerType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListKeyPoliciesResponse {
    #[doc="<p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="NextMarker")]
    pub next_marker: Option<MarkerType>,
    #[doc="<p>A list of policy names. Currently, there is only one policy and it is named \"Default\".</p>"]
    #[serde(rename="PolicyNames")]
    pub policy_names: Option<PolicyNameList>,
    #[doc="<p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To retrieve more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="Truncated")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub truncated: Option<BooleanType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListKeysRequest {
    #[doc="<p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<LimitType>,
    #[doc="<p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>"]
    #[serde(rename="Marker")]
    pub marker: Option<MarkerType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListKeysResponse {
    #[doc="<p>A list of keys.</p>"]
    #[serde(rename="Keys")]
    pub keys: Option<KeyList>,
    #[doc="<p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="NextMarker")]
    pub next_marker: Option<MarkerType>,
    #[doc="<p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To retrieve more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="Truncated")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub truncated: Option<BooleanType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListResourceTagsRequest {
    #[doc="<p>A unique identifier for the CMK whose tags you are listing. You can use the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 50, inclusive. If you do not include a value, it defaults to 50.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<LimitType>,
    #[doc="<p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p> <p>Do not attempt to construct this value. Use only the value of <code>NextMarker</code> from the truncated response you just received.</p>"]
    #[serde(rename="Marker")]
    pub marker: Option<MarkerType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListResourceTagsResponse {
    #[doc="<p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p> <p>Do not assume or infer any information from this value.</p>"]
    #[serde(rename="NextMarker")]
    pub next_marker: Option<MarkerType>,
    #[doc="<p>A list of tags. Each tag consists of a tag key and a tag value.</p>"]
    #[serde(rename="Tags")]
    pub tags: Option<TagList>,
    #[doc="<p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To retrieve more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>"]
    #[serde(rename="Truncated")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub truncated: Option<BooleanType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRetirableGrantsRequest {
    #[doc="<p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>"]
    #[serde(rename="Limit")]
    pub limit: Option<LimitType>,
    #[doc="<p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>"]
    #[serde(rename="Marker")]
    pub marker: Option<MarkerType>,
    #[doc="<p>The retiring principal for which to list grants.</p> <p>To specify the retiring principal, use the <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax for specifying a principal, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam\">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>Amazon Web Services General Reference</i>.</p>"]
    #[serde(rename="RetiringPrincipal")]
    pub retiring_principal: PrincipalIdType,
}

pub type MarkerType = String;
pub type NumberOfBytesType = i64;
pub type OriginType = String;
pub type PendingWindowInDaysType = i64;
pub type PlaintextType = Vec<u8>;
pub type PolicyNameList = Vec<PolicyNameType>;
pub type PolicyNameType = String;
pub type PolicyType = String;
pub type PrincipalIdType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutKeyPolicyRequest {
    #[doc="<p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the likelihood that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam\">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the CMK.</p> <p>The default value is false.</p>"]
    #[serde(rename="BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<BooleanType>,
    #[doc="<p>A unique identifier for the CMK.</p> <p>Use the CMK's unique identifier or its Amazon Resource Name (ARN). For example:</p> <ul> <li> <p>Unique ID: 1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> <li> <p>ARN: arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>The key policy to attach to the CMK.</p> <p>If you do not set <code>BypassPolicyLockoutSafetyCheck</code> to true, the policy must meet the following criteria:</p> <ul> <li> <p>It must allow the principal that is making the <code>PutKeyPolicy</code> request to make a subsequent <code>PutKeyPolicy</code> request on the CMK. This reduces the likelihood that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam\">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>The principals that are specified in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before specifying the new principal in a key policy because the new principal might not immediately be visible to AWS KMS. For more information, see <a href=\"http://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency\">Changes that I make are not always immediately visible</a> in the <i>IAM User Guide</i>.</p> </li> </ul> <p>The policy size limit is 32 KiB (32768 bytes).</p>"]
    #[serde(rename="Policy")]
    pub policy: PolicyType,
    #[doc="<p>The name of the key policy.</p> <p>This value must be <code>default</code>.</p>"]
    #[serde(rename="PolicyName")]
    pub policy_name: PolicyNameType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ReEncryptRequest {
    #[doc="<p>Ciphertext of the data to reencrypt.</p>"]
    #[serde(rename="CiphertextBlob")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub ciphertext_blob: CiphertextType,
    #[doc="<p>Encryption context to use when the data is reencrypted.</p>"]
    #[serde(rename="DestinationEncryptionContext")]
    pub destination_encryption_context: Option<EncryptionContextType>,
    #[doc="<p>A unique identifier for the CMK to use to reencrypt the data. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by \"alias/\".</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Alias ARN Example - arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Alias Name Example - alias/MyAliasName</p> </li> </ul>"]
    #[serde(rename="DestinationKeyId")]
    pub destination_key_id: KeyIdType,
    #[doc="<p>A list of grant tokens.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token\">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    #[serde(rename="GrantTokens")]
    pub grant_tokens: Option<GrantTokenList>,
    #[doc="<p>Encryption context used to encrypt and decrypt the data specified in the <code>CiphertextBlob</code> parameter.</p>"]
    #[serde(rename="SourceEncryptionContext")]
    pub source_encryption_context: Option<EncryptionContextType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReEncryptResponse {
    #[doc="<p>The reencrypted data.</p>"]
    #[serde(rename="CiphertextBlob")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub ciphertext_blob: Option<CiphertextType>,
    #[doc="<p>Unique identifier of the CMK used to reencrypt the data.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
    #[doc="<p>Unique identifier of the CMK used to originally encrypt the data.</p>"]
    #[serde(rename="SourceKeyId")]
    pub source_key_id: Option<KeyIdType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RetireGrantRequest {
    #[doc="<p>Unique identifier of the grant to retire. The grant ID is returned in the response to a <code>CreateGrant</code> operation.</p> <ul> <li> <p>Grant ID Example - 0123456789012345678901234567890123456789012345678901234567890123</p> </li> </ul>"]
    #[serde(rename="GrantId")]
    pub grant_id: Option<GrantIdType>,
    #[doc="<p>Token that identifies the grant to be retired.</p>"]
    #[serde(rename="GrantToken")]
    pub grant_token: Option<GrantTokenType>,
    #[doc="<p>The Amazon Resource Name of the CMK associated with the grant. Example:</p> <ul> <li> <p>arn:aws:kms:us-east-2:444455556666:key/1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RevokeGrantRequest {
    #[doc="<p>Identifier of the grant to be revoked.</p>"]
    #[serde(rename="GrantId")]
    pub grant_id: GrantIdType,
    #[doc="<p>A unique identifier for the customer master key associated with the grant. This value can be a globally unique identifier or the fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ScheduleKeyDeletionRequest {
    #[doc="<p>The unique identifier for the customer master key (CMK) to delete.</p> <p>To specify this value, use the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: 1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> <li> <p>Key ARN: arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</p> </li> </ul> <p>To obtain the unique key ID and key ARN for a given CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the customer master key (CMK).</p> <p>This value is optional. If you include a value, it must be between 7 and 30, inclusive. If you do not include a value, it defaults to 30.</p>"]
    #[serde(rename="PendingWindowInDays")]
    pub pending_window_in_days: Option<PendingWindowInDaysType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ScheduleKeyDeletionResponse {
    #[doc="<p>The date and time after which AWS KMS deletes the customer master key (CMK).</p>"]
    #[serde(rename="DeletionDate")]
    pub deletion_date: Option<DateType>,
    #[doc="<p>The unique identifier of the customer master key (CMK) for which deletion is scheduled.</p>"]
    #[serde(rename="KeyId")]
    pub key_id: Option<KeyIdType>,
}

#[doc="<p>A key-value pair. A tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Tag {
    #[doc="<p>The key of the tag.</p>"]
    #[serde(rename="TagKey")]
    pub tag_key: TagKeyType,
    #[doc="<p>The value of the tag.</p>"]
    #[serde(rename="TagValue")]
    pub tag_value: TagValueType,
}

pub type TagKeyList = Vec<TagKeyType>;
pub type TagKeyType = String;
pub type TagList = Vec<Tag>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct TagResourceRequest {
    #[doc="<p>A unique identifier for the CMK you are tagging. You can use the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>One or more tags. Each tag consists of a tag key and a tag value.</p>"]
    #[serde(rename="Tags")]
    pub tags: TagList,
}

pub type TagValueType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct UntagResourceRequest {
    #[doc="<p>A unique identifier for the CMK from which you are removing tags. You can use the unique key ID or the Amazon Resource Name (ARN) of the CMK. Examples:</p> <ul> <li> <p>Unique key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
    #[doc="<p>One or more tag keys. Specify only the tag keys, not the tag values.</p>"]
    #[serde(rename="TagKeys")]
    pub tag_keys: TagKeyList,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateAliasRequest {
    #[doc="<p>String that contains the name of the alias to be modified. The name must start with the word \"alias\" followed by a forward slash (alias/). Aliases that begin with \"alias/aws\" are reserved.</p>"]
    #[serde(rename="AliasName")]
    pub alias_name: AliasNameType,
    #[doc="<p>Unique identifier of the customer master key to be mapped to the alias. This value can be a globally unique identifier or the fully specified ARN of a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul> <p>You can call <a>ListAliases</a> to verify that the alias is mapped to the correct <code>TargetKeyId</code>.</p>"]
    #[serde(rename="TargetKeyId")]
    pub target_key_id: KeyIdType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateKeyDescriptionRequest {
    #[doc="<p>New description for the CMK.</p>"]
    #[serde(rename="Description")]
    pub description: DescriptionType,
    #[doc="<p>A unique identifier for the CMK. This value can be a globally unique identifier or the fully specified ARN to a key.</p> <ul> <li> <p>Key ARN Example - arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>Globally Unique Key ID Example - 12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
    #[serde(rename="KeyId")]
    pub key_id: KeyIdType,
}

pub type WrappingKeySpec = String;
/// Errors returned by CancelKeyDeletion
#[derive(Debug, PartialEq)]
pub enum CancelKeyDeletionError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CancelKeyDeletionError {
    pub fn from_body(body: &str) -> CancelKeyDeletionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        CancelKeyDeletionError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        CancelKeyDeletionError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        CancelKeyDeletionError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        CancelKeyDeletionError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CancelKeyDeletionError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelKeyDeletionError::Validation(error_message.to_string())
                    }
                    _ => CancelKeyDeletionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelKeyDeletionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelKeyDeletionError {
    fn from(err: serde_json::error::Error) -> CancelKeyDeletionError {
        CancelKeyDeletionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelKeyDeletionError {
    fn from(err: CredentialsError) -> CancelKeyDeletionError {
        CancelKeyDeletionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelKeyDeletionError {
    fn from(err: HttpDispatchError) -> CancelKeyDeletionError {
        CancelKeyDeletionError::HttpDispatch(err)
    }
}
impl fmt::Display for CancelKeyDeletionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelKeyDeletionError {
    fn description(&self) -> &str {
        match *self {
            CancelKeyDeletionError::DependencyTimeout(ref cause) => cause,
            CancelKeyDeletionError::InvalidArn(ref cause) => cause,
            CancelKeyDeletionError::KMSInternal(ref cause) => cause,
            CancelKeyDeletionError::KMSInvalidState(ref cause) => cause,
            CancelKeyDeletionError::NotFound(ref cause) => cause,
            CancelKeyDeletionError::Validation(ref cause) => cause,
            CancelKeyDeletionError::Credentials(ref err) => err.description(),
            CancelKeyDeletionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CancelKeyDeletionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    ///<p>The request was rejected because it attempted to create a resource that already exists.</p>
    AlreadyExists(String),
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified alias name is not valid.</p>
    InvalidAliasName(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateAliasError {
    pub fn from_body(body: &str) -> CreateAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyExistsException" => {
                        CreateAliasError::AlreadyExists(String::from(error_message))
                    }
                    "DependencyTimeoutException" => {
                        CreateAliasError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidAliasNameException" => {
                        CreateAliasError::InvalidAliasName(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        CreateAliasError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        CreateAliasError::KMSInvalidState(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateAliasError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => CreateAliasError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        CreateAliasError::Validation(error_message.to_string())
                    }
                    _ => CreateAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAliasError {
    fn from(err: serde_json::error::Error) -> CreateAliasError {
        CreateAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAliasError {
    fn from(err: CredentialsError) -> CreateAliasError {
        CreateAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAliasError {
    fn from(err: HttpDispatchError) -> CreateAliasError {
        CreateAliasError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAliasError {
    fn description(&self) -> &str {
        match *self {
            CreateAliasError::AlreadyExists(ref cause) => cause,
            CreateAliasError::DependencyTimeout(ref cause) => cause,
            CreateAliasError::InvalidAliasName(ref cause) => cause,
            CreateAliasError::KMSInternal(ref cause) => cause,
            CreateAliasError::KMSInvalidState(ref cause) => cause,
            CreateAliasError::LimitExceeded(ref cause) => cause,
            CreateAliasError::NotFound(ref cause) => cause,
            CreateAliasError::Validation(ref cause) => cause,
            CreateAliasError::Credentials(ref err) => err.description(),
            CreateAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGrant
#[derive(Debug, PartialEq)]
pub enum CreateGrantError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateGrantError {
    pub fn from_body(body: &str) -> CreateGrantError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        CreateGrantError::DependencyTimeout(String::from(error_message))
                    }
                    "DisabledException" => CreateGrantError::Disabled(String::from(error_message)),
                    "InvalidArnException" => {
                        CreateGrantError::InvalidArn(String::from(error_message))
                    }
                    "InvalidGrantTokenException" => {
                        CreateGrantError::InvalidGrantToken(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        CreateGrantError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        CreateGrantError::KMSInvalidState(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateGrantError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => CreateGrantError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        CreateGrantError::Validation(error_message.to_string())
                    }
                    _ => CreateGrantError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateGrantError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateGrantError {
    fn from(err: serde_json::error::Error) -> CreateGrantError {
        CreateGrantError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateGrantError {
    fn from(err: CredentialsError) -> CreateGrantError {
        CreateGrantError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateGrantError {
    fn from(err: HttpDispatchError) -> CreateGrantError {
        CreateGrantError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateGrantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGrantError {
    fn description(&self) -> &str {
        match *self {
            CreateGrantError::DependencyTimeout(ref cause) => cause,
            CreateGrantError::Disabled(ref cause) => cause,
            CreateGrantError::InvalidArn(ref cause) => cause,
            CreateGrantError::InvalidGrantToken(ref cause) => cause,
            CreateGrantError::KMSInternal(ref cause) => cause,
            CreateGrantError::KMSInvalidState(ref cause) => cause,
            CreateGrantError::LimitExceeded(ref cause) => cause,
            CreateGrantError::NotFound(ref cause) => cause,
            CreateGrantError::Validation(ref cause) => cause,
            CreateGrantError::Credentials(ref err) => err.description(),
            CreateGrantError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateGrantError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateKey
#[derive(Debug, PartialEq)]
pub enum CreateKeyError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    ///<p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    ///<p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
    ///<p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateKeyError {
    pub fn from_body(body: &str) -> CreateKeyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        CreateKeyError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        CreateKeyError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        CreateKeyError::KMSInternal(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateKeyError::LimitExceeded(String::from(error_message))
                    }
                    "MalformedPolicyDocumentException" => {
                        CreateKeyError::MalformedPolicyDocument(String::from(error_message))
                    }
                    "TagException" => CreateKeyError::Tag(String::from(error_message)),
                    "UnsupportedOperationException" => {
                        CreateKeyError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => CreateKeyError::Validation(error_message.to_string()),
                    _ => CreateKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateKeyError {
    fn from(err: serde_json::error::Error) -> CreateKeyError {
        CreateKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateKeyError {
    fn from(err: CredentialsError) -> CreateKeyError {
        CreateKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateKeyError {
    fn from(err: HttpDispatchError) -> CreateKeyError {
        CreateKeyError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateKeyError {
    fn description(&self) -> &str {
        match *self {
            CreateKeyError::DependencyTimeout(ref cause) => cause,
            CreateKeyError::InvalidArn(ref cause) => cause,
            CreateKeyError::KMSInternal(ref cause) => cause,
            CreateKeyError::LimitExceeded(ref cause) => cause,
            CreateKeyError::MalformedPolicyDocument(ref cause) => cause,
            CreateKeyError::Tag(ref cause) => cause,
            CreateKeyError::UnsupportedOperation(ref cause) => cause,
            CreateKeyError::Validation(ref cause) => cause,
            CreateKeyError::Credentials(ref err) => err.description(),
            CreateKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Decrypt
#[derive(Debug, PartialEq)]
pub enum DecryptError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    ///<p>The request was rejected because the specified ciphertext has been corrupted or is otherwise invalid.</p>
    InvalidCiphertext(String),
    ///<p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DecryptError {
    pub fn from_body(body: &str) -> DecryptError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        DecryptError::DependencyTimeout(String::from(error_message))
                    }
                    "DisabledException" => DecryptError::Disabled(String::from(error_message)),
                    "InvalidCiphertextException" => {
                        DecryptError::InvalidCiphertext(String::from(error_message))
                    }
                    "InvalidGrantTokenException" => {
                        DecryptError::InvalidGrantToken(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        DecryptError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        DecryptError::KMSInvalidState(String::from(error_message))
                    }
                    "KeyUnavailableException" => {
                        DecryptError::KeyUnavailable(String::from(error_message))
                    }
                    "NotFoundException" => DecryptError::NotFound(String::from(error_message)),
                    "ValidationException" => DecryptError::Validation(error_message.to_string()),
                    _ => DecryptError::Unknown(String::from(body)),
                }
            }
            Err(_) => DecryptError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DecryptError {
    fn from(err: serde_json::error::Error) -> DecryptError {
        DecryptError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DecryptError {
    fn from(err: CredentialsError) -> DecryptError {
        DecryptError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DecryptError {
    fn from(err: HttpDispatchError) -> DecryptError {
        DecryptError::HttpDispatch(err)
    }
}
impl fmt::Display for DecryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DecryptError {
    fn description(&self) -> &str {
        match *self {
            DecryptError::DependencyTimeout(ref cause) => cause,
            DecryptError::Disabled(ref cause) => cause,
            DecryptError::InvalidCiphertext(ref cause) => cause,
            DecryptError::InvalidGrantToken(ref cause) => cause,
            DecryptError::KMSInternal(ref cause) => cause,
            DecryptError::KMSInvalidState(ref cause) => cause,
            DecryptError::KeyUnavailable(ref cause) => cause,
            DecryptError::NotFound(ref cause) => cause,
            DecryptError::Validation(ref cause) => cause,
            DecryptError::Credentials(ref err) => err.description(),
            DecryptError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DecryptError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteAliasError {
    pub fn from_body(body: &str) -> DeleteAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        DeleteAliasError::DependencyTimeout(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        DeleteAliasError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        DeleteAliasError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => DeleteAliasError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        DeleteAliasError::Validation(error_message.to_string())
                    }
                    _ => DeleteAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAliasError {
    fn from(err: serde_json::error::Error) -> DeleteAliasError {
        DeleteAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAliasError {
    fn from(err: CredentialsError) -> DeleteAliasError {
        DeleteAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAliasError {
    fn from(err: HttpDispatchError) -> DeleteAliasError {
        DeleteAliasError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAliasError {
    fn description(&self) -> &str {
        match *self {
            DeleteAliasError::DependencyTimeout(ref cause) => cause,
            DeleteAliasError::KMSInternal(ref cause) => cause,
            DeleteAliasError::KMSInvalidState(ref cause) => cause,
            DeleteAliasError::NotFound(ref cause) => cause,
            DeleteAliasError::Validation(ref cause) => cause,
            DeleteAliasError::Credentials(ref err) => err.description(),
            DeleteAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteImportedKeyMaterial
#[derive(Debug, PartialEq)]
pub enum DeleteImportedKeyMaterialError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteImportedKeyMaterialError {
    pub fn from_body(body: &str) -> DeleteImportedKeyMaterialError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => DeleteImportedKeyMaterialError::DependencyTimeout(String::from(error_message)),
                    "InvalidArnException" => {
                        DeleteImportedKeyMaterialError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        DeleteImportedKeyMaterialError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        DeleteImportedKeyMaterialError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteImportedKeyMaterialError::NotFound(String::from(error_message))
                    }
                    "UnsupportedOperationException" => DeleteImportedKeyMaterialError::UnsupportedOperation(String::from(error_message)),
                    "ValidationException" => {
                        DeleteImportedKeyMaterialError::Validation(error_message.to_string())
                    }
                    _ => DeleteImportedKeyMaterialError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteImportedKeyMaterialError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteImportedKeyMaterialError {
    fn from(err: serde_json::error::Error) -> DeleteImportedKeyMaterialError {
        DeleteImportedKeyMaterialError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteImportedKeyMaterialError {
    fn from(err: CredentialsError) -> DeleteImportedKeyMaterialError {
        DeleteImportedKeyMaterialError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteImportedKeyMaterialError {
    fn from(err: HttpDispatchError) -> DeleteImportedKeyMaterialError {
        DeleteImportedKeyMaterialError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteImportedKeyMaterialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteImportedKeyMaterialError {
    fn description(&self) -> &str {
        match *self {
            DeleteImportedKeyMaterialError::DependencyTimeout(ref cause) => cause,
            DeleteImportedKeyMaterialError::InvalidArn(ref cause) => cause,
            DeleteImportedKeyMaterialError::KMSInternal(ref cause) => cause,
            DeleteImportedKeyMaterialError::KMSInvalidState(ref cause) => cause,
            DeleteImportedKeyMaterialError::NotFound(ref cause) => cause,
            DeleteImportedKeyMaterialError::UnsupportedOperation(ref cause) => cause,
            DeleteImportedKeyMaterialError::Validation(ref cause) => cause,
            DeleteImportedKeyMaterialError::Credentials(ref err) => err.description(),
            DeleteImportedKeyMaterialError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteImportedKeyMaterialError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeKey
#[derive(Debug, PartialEq)]
pub enum DescribeKeyError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeKeyError {
    pub fn from_body(body: &str) -> DescribeKeyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        DescribeKeyError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DescribeKeyError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        DescribeKeyError::KMSInternal(String::from(error_message))
                    }
                    "NotFoundException" => DescribeKeyError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        DescribeKeyError::Validation(error_message.to_string())
                    }
                    _ => DescribeKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeKeyError {
    fn from(err: serde_json::error::Error) -> DescribeKeyError {
        DescribeKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeKeyError {
    fn from(err: CredentialsError) -> DescribeKeyError {
        DescribeKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeKeyError {
    fn from(err: HttpDispatchError) -> DescribeKeyError {
        DescribeKeyError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeKeyError {
    fn description(&self) -> &str {
        match *self {
            DescribeKeyError::DependencyTimeout(ref cause) => cause,
            DescribeKeyError::InvalidArn(ref cause) => cause,
            DescribeKeyError::KMSInternal(ref cause) => cause,
            DescribeKeyError::NotFound(ref cause) => cause,
            DescribeKeyError::Validation(ref cause) => cause,
            DescribeKeyError::Credentials(ref err) => err.description(),
            DescribeKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableKey
#[derive(Debug, PartialEq)]
pub enum DisableKeyError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisableKeyError {
    pub fn from_body(body: &str) -> DisableKeyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        DisableKeyError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DisableKeyError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        DisableKeyError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        DisableKeyError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => DisableKeyError::NotFound(String::from(error_message)),
                    "ValidationException" => DisableKeyError::Validation(error_message.to_string()),
                    _ => DisableKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableKeyError {
    fn from(err: serde_json::error::Error) -> DisableKeyError {
        DisableKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableKeyError {
    fn from(err: CredentialsError) -> DisableKeyError {
        DisableKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableKeyError {
    fn from(err: HttpDispatchError) -> DisableKeyError {
        DisableKeyError::HttpDispatch(err)
    }
}
impl fmt::Display for DisableKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableKeyError {
    fn description(&self) -> &str {
        match *self {
            DisableKeyError::DependencyTimeout(ref cause) => cause,
            DisableKeyError::InvalidArn(ref cause) => cause,
            DisableKeyError::KMSInternal(ref cause) => cause,
            DisableKeyError::KMSInvalidState(ref cause) => cause,
            DisableKeyError::NotFound(ref cause) => cause,
            DisableKeyError::Validation(ref cause) => cause,
            DisableKeyError::Credentials(ref err) => err.description(),
            DisableKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableKeyRotation
#[derive(Debug, PartialEq)]
pub enum DisableKeyRotationError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisableKeyRotationError {
    pub fn from_body(body: &str) -> DisableKeyRotationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        DisableKeyRotationError::DependencyTimeout(String::from(error_message))
                    }
                    "DisabledException" => {
                        DisableKeyRotationError::Disabled(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        DisableKeyRotationError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        DisableKeyRotationError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        DisableKeyRotationError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DisableKeyRotationError::NotFound(String::from(error_message))
                    }
                    "UnsupportedOperationException" => {
                        DisableKeyRotationError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableKeyRotationError::Validation(error_message.to_string())
                    }
                    _ => DisableKeyRotationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableKeyRotationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableKeyRotationError {
    fn from(err: serde_json::error::Error) -> DisableKeyRotationError {
        DisableKeyRotationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableKeyRotationError {
    fn from(err: CredentialsError) -> DisableKeyRotationError {
        DisableKeyRotationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableKeyRotationError {
    fn from(err: HttpDispatchError) -> DisableKeyRotationError {
        DisableKeyRotationError::HttpDispatch(err)
    }
}
impl fmt::Display for DisableKeyRotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableKeyRotationError {
    fn description(&self) -> &str {
        match *self {
            DisableKeyRotationError::DependencyTimeout(ref cause) => cause,
            DisableKeyRotationError::Disabled(ref cause) => cause,
            DisableKeyRotationError::InvalidArn(ref cause) => cause,
            DisableKeyRotationError::KMSInternal(ref cause) => cause,
            DisableKeyRotationError::KMSInvalidState(ref cause) => cause,
            DisableKeyRotationError::NotFound(ref cause) => cause,
            DisableKeyRotationError::UnsupportedOperation(ref cause) => cause,
            DisableKeyRotationError::Validation(ref cause) => cause,
            DisableKeyRotationError::Credentials(ref err) => err.description(),
            DisableKeyRotationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisableKeyRotationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableKey
#[derive(Debug, PartialEq)]
pub enum EnableKeyError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EnableKeyError {
    pub fn from_body(body: &str) -> EnableKeyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        EnableKeyError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        EnableKeyError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        EnableKeyError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        EnableKeyError::KMSInvalidState(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        EnableKeyError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => EnableKeyError::NotFound(String::from(error_message)),
                    "ValidationException" => EnableKeyError::Validation(error_message.to_string()),
                    _ => EnableKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableKeyError {
    fn from(err: serde_json::error::Error) -> EnableKeyError {
        EnableKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableKeyError {
    fn from(err: CredentialsError) -> EnableKeyError {
        EnableKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableKeyError {
    fn from(err: HttpDispatchError) -> EnableKeyError {
        EnableKeyError::HttpDispatch(err)
    }
}
impl fmt::Display for EnableKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableKeyError {
    fn description(&self) -> &str {
        match *self {
            EnableKeyError::DependencyTimeout(ref cause) => cause,
            EnableKeyError::InvalidArn(ref cause) => cause,
            EnableKeyError::KMSInternal(ref cause) => cause,
            EnableKeyError::KMSInvalidState(ref cause) => cause,
            EnableKeyError::LimitExceeded(ref cause) => cause,
            EnableKeyError::NotFound(ref cause) => cause,
            EnableKeyError::Validation(ref cause) => cause,
            EnableKeyError::Credentials(ref err) => err.description(),
            EnableKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableKeyRotation
#[derive(Debug, PartialEq)]
pub enum EnableKeyRotationError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EnableKeyRotationError {
    pub fn from_body(body: &str) -> EnableKeyRotationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        EnableKeyRotationError::DependencyTimeout(String::from(error_message))
                    }
                    "DisabledException" => {
                        EnableKeyRotationError::Disabled(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        EnableKeyRotationError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        EnableKeyRotationError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        EnableKeyRotationError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        EnableKeyRotationError::NotFound(String::from(error_message))
                    }
                    "UnsupportedOperationException" => {
                        EnableKeyRotationError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableKeyRotationError::Validation(error_message.to_string())
                    }
                    _ => EnableKeyRotationError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableKeyRotationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableKeyRotationError {
    fn from(err: serde_json::error::Error) -> EnableKeyRotationError {
        EnableKeyRotationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableKeyRotationError {
    fn from(err: CredentialsError) -> EnableKeyRotationError {
        EnableKeyRotationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableKeyRotationError {
    fn from(err: HttpDispatchError) -> EnableKeyRotationError {
        EnableKeyRotationError::HttpDispatch(err)
    }
}
impl fmt::Display for EnableKeyRotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableKeyRotationError {
    fn description(&self) -> &str {
        match *self {
            EnableKeyRotationError::DependencyTimeout(ref cause) => cause,
            EnableKeyRotationError::Disabled(ref cause) => cause,
            EnableKeyRotationError::InvalidArn(ref cause) => cause,
            EnableKeyRotationError::KMSInternal(ref cause) => cause,
            EnableKeyRotationError::KMSInvalidState(ref cause) => cause,
            EnableKeyRotationError::NotFound(ref cause) => cause,
            EnableKeyRotationError::UnsupportedOperation(ref cause) => cause,
            EnableKeyRotationError::Validation(ref cause) => cause,
            EnableKeyRotationError::Credentials(ref err) => err.description(),
            EnableKeyRotationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            EnableKeyRotationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Encrypt
#[derive(Debug, PartialEq)]
pub enum EncryptError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    ///<p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    ///<p>The request was rejected because the specified <code>KeySpec</code> value is not valid.</p>
    InvalidKeyUsage(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EncryptError {
    pub fn from_body(body: &str) -> EncryptError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        EncryptError::DependencyTimeout(String::from(error_message))
                    }
                    "DisabledException" => EncryptError::Disabled(String::from(error_message)),
                    "InvalidGrantTokenException" => {
                        EncryptError::InvalidGrantToken(String::from(error_message))
                    }
                    "InvalidKeyUsageException" => {
                        EncryptError::InvalidKeyUsage(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        EncryptError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        EncryptError::KMSInvalidState(String::from(error_message))
                    }
                    "KeyUnavailableException" => {
                        EncryptError::KeyUnavailable(String::from(error_message))
                    }
                    "NotFoundException" => EncryptError::NotFound(String::from(error_message)),
                    "ValidationException" => EncryptError::Validation(error_message.to_string()),
                    _ => EncryptError::Unknown(String::from(body)),
                }
            }
            Err(_) => EncryptError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EncryptError {
    fn from(err: serde_json::error::Error) -> EncryptError {
        EncryptError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EncryptError {
    fn from(err: CredentialsError) -> EncryptError {
        EncryptError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EncryptError {
    fn from(err: HttpDispatchError) -> EncryptError {
        EncryptError::HttpDispatch(err)
    }
}
impl fmt::Display for EncryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EncryptError {
    fn description(&self) -> &str {
        match *self {
            EncryptError::DependencyTimeout(ref cause) => cause,
            EncryptError::Disabled(ref cause) => cause,
            EncryptError::InvalidGrantToken(ref cause) => cause,
            EncryptError::InvalidKeyUsage(ref cause) => cause,
            EncryptError::KMSInternal(ref cause) => cause,
            EncryptError::KMSInvalidState(ref cause) => cause,
            EncryptError::KeyUnavailable(ref cause) => cause,
            EncryptError::NotFound(ref cause) => cause,
            EncryptError::Validation(ref cause) => cause,
            EncryptError::Credentials(ref err) => err.description(),
            EncryptError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EncryptError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GenerateDataKey
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    ///<p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    ///<p>The request was rejected because the specified <code>KeySpec</code> value is not valid.</p>
    InvalidKeyUsage(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GenerateDataKeyError {
    pub fn from_body(body: &str) -> GenerateDataKeyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        GenerateDataKeyError::DependencyTimeout(String::from(error_message))
                    }
                    "DisabledException" => {
                        GenerateDataKeyError::Disabled(String::from(error_message))
                    }
                    "InvalidGrantTokenException" => {
                        GenerateDataKeyError::InvalidGrantToken(String::from(error_message))
                    }
                    "InvalidKeyUsageException" => {
                        GenerateDataKeyError::InvalidKeyUsage(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        GenerateDataKeyError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        GenerateDataKeyError::KMSInvalidState(String::from(error_message))
                    }
                    "KeyUnavailableException" => {
                        GenerateDataKeyError::KeyUnavailable(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GenerateDataKeyError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GenerateDataKeyError::Validation(error_message.to_string())
                    }
                    _ => GenerateDataKeyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GenerateDataKeyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GenerateDataKeyError {
    fn from(err: serde_json::error::Error) -> GenerateDataKeyError {
        GenerateDataKeyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GenerateDataKeyError {
    fn from(err: CredentialsError) -> GenerateDataKeyError {
        GenerateDataKeyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GenerateDataKeyError {
    fn from(err: HttpDispatchError) -> GenerateDataKeyError {
        GenerateDataKeyError::HttpDispatch(err)
    }
}
impl fmt::Display for GenerateDataKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateDataKeyError {
    fn description(&self) -> &str {
        match *self {
            GenerateDataKeyError::DependencyTimeout(ref cause) => cause,
            GenerateDataKeyError::Disabled(ref cause) => cause,
            GenerateDataKeyError::InvalidGrantToken(ref cause) => cause,
            GenerateDataKeyError::InvalidKeyUsage(ref cause) => cause,
            GenerateDataKeyError::KMSInternal(ref cause) => cause,
            GenerateDataKeyError::KMSInvalidState(ref cause) => cause,
            GenerateDataKeyError::KeyUnavailable(ref cause) => cause,
            GenerateDataKeyError::NotFound(ref cause) => cause,
            GenerateDataKeyError::Validation(ref cause) => cause,
            GenerateDataKeyError::Credentials(ref err) => err.description(),
            GenerateDataKeyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GenerateDataKeyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GenerateDataKeyWithoutPlaintext
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyWithoutPlaintextError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    ///<p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    ///<p>The request was rejected because the specified <code>KeySpec</code> value is not valid.</p>
    InvalidKeyUsage(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GenerateDataKeyWithoutPlaintextError {
    pub fn from_body(body: &str) -> GenerateDataKeyWithoutPlaintextError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => GenerateDataKeyWithoutPlaintextError::DependencyTimeout(String::from(error_message)),
                    "DisabledException" => {
                        GenerateDataKeyWithoutPlaintextError::Disabled(String::from(error_message))
                    }
                    "InvalidGrantTokenException" => GenerateDataKeyWithoutPlaintextError::InvalidGrantToken(String::from(error_message)),
                    "InvalidKeyUsageException" => GenerateDataKeyWithoutPlaintextError::InvalidKeyUsage(String::from(error_message)),
                    "KMSInternalException" => GenerateDataKeyWithoutPlaintextError::KMSInternal(String::from(error_message)),
                    "KMSInvalidStateException" => GenerateDataKeyWithoutPlaintextError::KMSInvalidState(String::from(error_message)),
                    "KeyUnavailableException" => GenerateDataKeyWithoutPlaintextError::KeyUnavailable(String::from(error_message)),
                    "NotFoundException" => {
                        GenerateDataKeyWithoutPlaintextError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GenerateDataKeyWithoutPlaintextError::Validation(error_message.to_string())
                    }
                    _ => GenerateDataKeyWithoutPlaintextError::Unknown(String::from(body)),
                }
            }
            Err(_) => GenerateDataKeyWithoutPlaintextError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GenerateDataKeyWithoutPlaintextError {
    fn from(err: serde_json::error::Error) -> GenerateDataKeyWithoutPlaintextError {
        GenerateDataKeyWithoutPlaintextError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GenerateDataKeyWithoutPlaintextError {
    fn from(err: CredentialsError) -> GenerateDataKeyWithoutPlaintextError {
        GenerateDataKeyWithoutPlaintextError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GenerateDataKeyWithoutPlaintextError {
    fn from(err: HttpDispatchError) -> GenerateDataKeyWithoutPlaintextError {
        GenerateDataKeyWithoutPlaintextError::HttpDispatch(err)
    }
}
impl fmt::Display for GenerateDataKeyWithoutPlaintextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateDataKeyWithoutPlaintextError {
    fn description(&self) -> &str {
        match *self {
            GenerateDataKeyWithoutPlaintextError::DependencyTimeout(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::Disabled(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::InvalidGrantToken(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::InvalidKeyUsage(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::KMSInternal(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::KMSInvalidState(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::KeyUnavailable(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::NotFound(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::Validation(ref cause) => cause,
            GenerateDataKeyWithoutPlaintextError::Credentials(ref err) => err.description(),
            GenerateDataKeyWithoutPlaintextError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GenerateDataKeyWithoutPlaintextError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GenerateRandom
#[derive(Debug, PartialEq)]
pub enum GenerateRandomError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GenerateRandomError {
    pub fn from_body(body: &str) -> GenerateRandomError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        GenerateRandomError::DependencyTimeout(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        GenerateRandomError::KMSInternal(String::from(error_message))
                    }
                    "ValidationException" => {
                        GenerateRandomError::Validation(error_message.to_string())
                    }
                    _ => GenerateRandomError::Unknown(String::from(body)),
                }
            }
            Err(_) => GenerateRandomError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GenerateRandomError {
    fn from(err: serde_json::error::Error) -> GenerateRandomError {
        GenerateRandomError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GenerateRandomError {
    fn from(err: CredentialsError) -> GenerateRandomError {
        GenerateRandomError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GenerateRandomError {
    fn from(err: HttpDispatchError) -> GenerateRandomError {
        GenerateRandomError::HttpDispatch(err)
    }
}
impl fmt::Display for GenerateRandomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateRandomError {
    fn description(&self) -> &str {
        match *self {
            GenerateRandomError::DependencyTimeout(ref cause) => cause,
            GenerateRandomError::KMSInternal(ref cause) => cause,
            GenerateRandomError::Validation(ref cause) => cause,
            GenerateRandomError::Credentials(ref err) => err.description(),
            GenerateRandomError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GenerateRandomError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetKeyPolicy
#[derive(Debug, PartialEq)]
pub enum GetKeyPolicyError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetKeyPolicyError {
    pub fn from_body(body: &str) -> GetKeyPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        GetKeyPolicyError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        GetKeyPolicyError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        GetKeyPolicyError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        GetKeyPolicyError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => GetKeyPolicyError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        GetKeyPolicyError::Validation(error_message.to_string())
                    }
                    _ => GetKeyPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetKeyPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetKeyPolicyError {
    fn from(err: serde_json::error::Error) -> GetKeyPolicyError {
        GetKeyPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetKeyPolicyError {
    fn from(err: CredentialsError) -> GetKeyPolicyError {
        GetKeyPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetKeyPolicyError {
    fn from(err: HttpDispatchError) -> GetKeyPolicyError {
        GetKeyPolicyError::HttpDispatch(err)
    }
}
impl fmt::Display for GetKeyPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetKeyPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetKeyPolicyError::DependencyTimeout(ref cause) => cause,
            GetKeyPolicyError::InvalidArn(ref cause) => cause,
            GetKeyPolicyError::KMSInternal(ref cause) => cause,
            GetKeyPolicyError::KMSInvalidState(ref cause) => cause,
            GetKeyPolicyError::NotFound(ref cause) => cause,
            GetKeyPolicyError::Validation(ref cause) => cause,
            GetKeyPolicyError::Credentials(ref err) => err.description(),
            GetKeyPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetKeyPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetKeyRotationStatus
#[derive(Debug, PartialEq)]
pub enum GetKeyRotationStatusError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetKeyRotationStatusError {
    pub fn from_body(body: &str) -> GetKeyRotationStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        GetKeyRotationStatusError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        GetKeyRotationStatusError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        GetKeyRotationStatusError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        GetKeyRotationStatusError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetKeyRotationStatusError::NotFound(String::from(error_message))
                    }
                    "UnsupportedOperationException" => {
                        GetKeyRotationStatusError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetKeyRotationStatusError::Validation(error_message.to_string())
                    }
                    _ => GetKeyRotationStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetKeyRotationStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetKeyRotationStatusError {
    fn from(err: serde_json::error::Error) -> GetKeyRotationStatusError {
        GetKeyRotationStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetKeyRotationStatusError {
    fn from(err: CredentialsError) -> GetKeyRotationStatusError {
        GetKeyRotationStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetKeyRotationStatusError {
    fn from(err: HttpDispatchError) -> GetKeyRotationStatusError {
        GetKeyRotationStatusError::HttpDispatch(err)
    }
}
impl fmt::Display for GetKeyRotationStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetKeyRotationStatusError {
    fn description(&self) -> &str {
        match *self {
            GetKeyRotationStatusError::DependencyTimeout(ref cause) => cause,
            GetKeyRotationStatusError::InvalidArn(ref cause) => cause,
            GetKeyRotationStatusError::KMSInternal(ref cause) => cause,
            GetKeyRotationStatusError::KMSInvalidState(ref cause) => cause,
            GetKeyRotationStatusError::NotFound(ref cause) => cause,
            GetKeyRotationStatusError::UnsupportedOperation(ref cause) => cause,
            GetKeyRotationStatusError::Validation(ref cause) => cause,
            GetKeyRotationStatusError::Credentials(ref err) => err.description(),
            GetKeyRotationStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetKeyRotationStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetParametersForImport
#[derive(Debug, PartialEq)]
pub enum GetParametersForImportError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetParametersForImportError {
    pub fn from_body(body: &str) -> GetParametersForImportError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        GetParametersForImportError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        GetParametersForImportError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        GetParametersForImportError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        GetParametersForImportError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetParametersForImportError::NotFound(String::from(error_message))
                    }
                    "UnsupportedOperationException" => GetParametersForImportError::UnsupportedOperation(String::from(error_message)),
                    "ValidationException" => {
                        GetParametersForImportError::Validation(error_message.to_string())
                    }
                    _ => GetParametersForImportError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetParametersForImportError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetParametersForImportError {
    fn from(err: serde_json::error::Error) -> GetParametersForImportError {
        GetParametersForImportError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetParametersForImportError {
    fn from(err: CredentialsError) -> GetParametersForImportError {
        GetParametersForImportError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetParametersForImportError {
    fn from(err: HttpDispatchError) -> GetParametersForImportError {
        GetParametersForImportError::HttpDispatch(err)
    }
}
impl fmt::Display for GetParametersForImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetParametersForImportError {
    fn description(&self) -> &str {
        match *self {
            GetParametersForImportError::DependencyTimeout(ref cause) => cause,
            GetParametersForImportError::InvalidArn(ref cause) => cause,
            GetParametersForImportError::KMSInternal(ref cause) => cause,
            GetParametersForImportError::KMSInvalidState(ref cause) => cause,
            GetParametersForImportError::NotFound(ref cause) => cause,
            GetParametersForImportError::UnsupportedOperation(ref cause) => cause,
            GetParametersForImportError::Validation(ref cause) => cause,
            GetParametersForImportError::Credentials(ref err) => err.description(),
            GetParametersForImportError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetParametersForImportError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportKeyMaterial
#[derive(Debug, PartialEq)]
pub enum ImportKeyMaterialError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the provided import token is expired. Use <a>GetParametersForImport</a> to retrieve a new import token and public key, use the new public key to encrypt the key material, and then try the request again.</p>
    ExpiredImportToken(String),
    ///<p>The request was rejected because the provided key material is invalid or is not the same key material that was previously imported into this customer master key (CMK).</p>
    IncorrectKeyMaterial(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because the specified ciphertext has been corrupted or is otherwise invalid.</p>
    InvalidCiphertext(String),
    ///<p>The request was rejected because the provided import token is invalid or is associated with a different customer master key (CMK).</p>
    InvalidImportToken(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ImportKeyMaterialError {
    pub fn from_body(body: &str) -> ImportKeyMaterialError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        ImportKeyMaterialError::DependencyTimeout(String::from(error_message))
                    }
                    "ExpiredImportTokenException" => {
                        ImportKeyMaterialError::ExpiredImportToken(String::from(error_message))
                    }
                    "IncorrectKeyMaterialException" => {
                        ImportKeyMaterialError::IncorrectKeyMaterial(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ImportKeyMaterialError::InvalidArn(String::from(error_message))
                    }
                    "InvalidCiphertextException" => {
                        ImportKeyMaterialError::InvalidCiphertext(String::from(error_message))
                    }
                    "InvalidImportTokenException" => {
                        ImportKeyMaterialError::InvalidImportToken(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ImportKeyMaterialError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        ImportKeyMaterialError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ImportKeyMaterialError::NotFound(String::from(error_message))
                    }
                    "UnsupportedOperationException" => {
                        ImportKeyMaterialError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        ImportKeyMaterialError::Validation(error_message.to_string())
                    }
                    _ => ImportKeyMaterialError::Unknown(String::from(body)),
                }
            }
            Err(_) => ImportKeyMaterialError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ImportKeyMaterialError {
    fn from(err: serde_json::error::Error) -> ImportKeyMaterialError {
        ImportKeyMaterialError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportKeyMaterialError {
    fn from(err: CredentialsError) -> ImportKeyMaterialError {
        ImportKeyMaterialError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportKeyMaterialError {
    fn from(err: HttpDispatchError) -> ImportKeyMaterialError {
        ImportKeyMaterialError::HttpDispatch(err)
    }
}
impl fmt::Display for ImportKeyMaterialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportKeyMaterialError {
    fn description(&self) -> &str {
        match *self {
            ImportKeyMaterialError::DependencyTimeout(ref cause) => cause,
            ImportKeyMaterialError::ExpiredImportToken(ref cause) => cause,
            ImportKeyMaterialError::IncorrectKeyMaterial(ref cause) => cause,
            ImportKeyMaterialError::InvalidArn(ref cause) => cause,
            ImportKeyMaterialError::InvalidCiphertext(ref cause) => cause,
            ImportKeyMaterialError::InvalidImportToken(ref cause) => cause,
            ImportKeyMaterialError::KMSInternal(ref cause) => cause,
            ImportKeyMaterialError::KMSInvalidState(ref cause) => cause,
            ImportKeyMaterialError::NotFound(ref cause) => cause,
            ImportKeyMaterialError::UnsupportedOperation(ref cause) => cause,
            ImportKeyMaterialError::Validation(ref cause) => cause,
            ImportKeyMaterialError::Credentials(ref err) => err.description(),
            ImportKeyMaterialError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ImportKeyMaterialError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListAliasesError {
    pub fn from_body(body: &str) -> ListAliasesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        ListAliasesError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidMarkerException" => {
                        ListAliasesError::InvalidMarker(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ListAliasesError::KMSInternal(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAliasesError::Validation(error_message.to_string())
                    }
                    _ => ListAliasesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAliasesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAliasesError {
    fn from(err: serde_json::error::Error) -> ListAliasesError {
        ListAliasesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAliasesError {
    fn from(err: CredentialsError) -> ListAliasesError {
        ListAliasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAliasesError {
    fn from(err: HttpDispatchError) -> ListAliasesError {
        ListAliasesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListAliasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAliasesError {
    fn description(&self) -> &str {
        match *self {
            ListAliasesError::DependencyTimeout(ref cause) => cause,
            ListAliasesError::InvalidMarker(ref cause) => cause,
            ListAliasesError::KMSInternal(ref cause) => cause,
            ListAliasesError::Validation(ref cause) => cause,
            ListAliasesError::Credentials(ref err) => err.description(),
            ListAliasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAliasesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGrants
#[derive(Debug, PartialEq)]
pub enum ListGrantsError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListGrantsError {
    pub fn from_body(body: &str) -> ListGrantsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        ListGrantsError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListGrantsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidMarkerException" => {
                        ListGrantsError::InvalidMarker(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ListGrantsError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        ListGrantsError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => ListGrantsError::NotFound(String::from(error_message)),
                    "ValidationException" => ListGrantsError::Validation(error_message.to_string()),
                    _ => ListGrantsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListGrantsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListGrantsError {
    fn from(err: serde_json::error::Error) -> ListGrantsError {
        ListGrantsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListGrantsError {
    fn from(err: CredentialsError) -> ListGrantsError {
        ListGrantsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListGrantsError {
    fn from(err: HttpDispatchError) -> ListGrantsError {
        ListGrantsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListGrantsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGrantsError {
    fn description(&self) -> &str {
        match *self {
            ListGrantsError::DependencyTimeout(ref cause) => cause,
            ListGrantsError::InvalidArn(ref cause) => cause,
            ListGrantsError::InvalidMarker(ref cause) => cause,
            ListGrantsError::KMSInternal(ref cause) => cause,
            ListGrantsError::KMSInvalidState(ref cause) => cause,
            ListGrantsError::NotFound(ref cause) => cause,
            ListGrantsError::Validation(ref cause) => cause,
            ListGrantsError::Credentials(ref err) => err.description(),
            ListGrantsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListGrantsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListKeyPolicies
#[derive(Debug, PartialEq)]
pub enum ListKeyPoliciesError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListKeyPoliciesError {
    pub fn from_body(body: &str) -> ListKeyPoliciesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        ListKeyPoliciesError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListKeyPoliciesError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ListKeyPoliciesError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        ListKeyPoliciesError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListKeyPoliciesError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListKeyPoliciesError::Validation(error_message.to_string())
                    }
                    _ => ListKeyPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListKeyPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListKeyPoliciesError {
    fn from(err: serde_json::error::Error) -> ListKeyPoliciesError {
        ListKeyPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListKeyPoliciesError {
    fn from(err: CredentialsError) -> ListKeyPoliciesError {
        ListKeyPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListKeyPoliciesError {
    fn from(err: HttpDispatchError) -> ListKeyPoliciesError {
        ListKeyPoliciesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListKeyPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListKeyPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListKeyPoliciesError::DependencyTimeout(ref cause) => cause,
            ListKeyPoliciesError::InvalidArn(ref cause) => cause,
            ListKeyPoliciesError::KMSInternal(ref cause) => cause,
            ListKeyPoliciesError::KMSInvalidState(ref cause) => cause,
            ListKeyPoliciesError::NotFound(ref cause) => cause,
            ListKeyPoliciesError::Validation(ref cause) => cause,
            ListKeyPoliciesError::Credentials(ref err) => err.description(),
            ListKeyPoliciesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListKeyPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListKeys
#[derive(Debug, PartialEq)]
pub enum ListKeysError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListKeysError {
    pub fn from_body(body: &str) -> ListKeysError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        ListKeysError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidMarkerException" => {
                        ListKeysError::InvalidMarker(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ListKeysError::KMSInternal(String::from(error_message))
                    }
                    "ValidationException" => ListKeysError::Validation(error_message.to_string()),
                    _ => ListKeysError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListKeysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListKeysError {
    fn from(err: serde_json::error::Error) -> ListKeysError {
        ListKeysError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListKeysError {
    fn from(err: CredentialsError) -> ListKeysError {
        ListKeysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListKeysError {
    fn from(err: HttpDispatchError) -> ListKeysError {
        ListKeysError::HttpDispatch(err)
    }
}
impl fmt::Display for ListKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListKeysError {
    fn description(&self) -> &str {
        match *self {
            ListKeysError::DependencyTimeout(ref cause) => cause,
            ListKeysError::InvalidMarker(ref cause) => cause,
            ListKeysError::KMSInternal(ref cause) => cause,
            ListKeysError::Validation(ref cause) => cause,
            ListKeysError::Credentials(ref err) => err.description(),
            ListKeysError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListKeysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceTags
#[derive(Debug, PartialEq)]
pub enum ListResourceTagsError {
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListResourceTagsError {
    pub fn from_body(body: &str) -> ListResourceTagsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArnException" => {
                        ListResourceTagsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidMarkerException" => {
                        ListResourceTagsError::InvalidMarker(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ListResourceTagsError::KMSInternal(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListResourceTagsError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListResourceTagsError::Validation(error_message.to_string())
                    }
                    _ => ListResourceTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListResourceTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListResourceTagsError {
    fn from(err: serde_json::error::Error) -> ListResourceTagsError {
        ListResourceTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourceTagsError {
    fn from(err: CredentialsError) -> ListResourceTagsError {
        ListResourceTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourceTagsError {
    fn from(err: HttpDispatchError) -> ListResourceTagsError {
        ListResourceTagsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListResourceTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceTagsError {
    fn description(&self) -> &str {
        match *self {
            ListResourceTagsError::InvalidArn(ref cause) => cause,
            ListResourceTagsError::InvalidMarker(ref cause) => cause,
            ListResourceTagsError::KMSInternal(ref cause) => cause,
            ListResourceTagsError::NotFound(ref cause) => cause,
            ListResourceTagsError::Validation(ref cause) => cause,
            ListResourceTagsError::Credentials(ref err) => err.description(),
            ListResourceTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListResourceTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRetirableGrants
#[derive(Debug, PartialEq)]
pub enum ListRetirableGrantsError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListRetirableGrantsError {
    pub fn from_body(body: &str) -> ListRetirableGrantsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        ListRetirableGrantsError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ListRetirableGrantsError::InvalidArn(String::from(error_message))
                    }
                    "InvalidMarkerException" => {
                        ListRetirableGrantsError::InvalidMarker(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ListRetirableGrantsError::KMSInternal(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListRetirableGrantsError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRetirableGrantsError::Validation(error_message.to_string())
                    }
                    _ => ListRetirableGrantsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRetirableGrantsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRetirableGrantsError {
    fn from(err: serde_json::error::Error) -> ListRetirableGrantsError {
        ListRetirableGrantsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRetirableGrantsError {
    fn from(err: CredentialsError) -> ListRetirableGrantsError {
        ListRetirableGrantsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRetirableGrantsError {
    fn from(err: HttpDispatchError) -> ListRetirableGrantsError {
        ListRetirableGrantsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListRetirableGrantsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRetirableGrantsError {
    fn description(&self) -> &str {
        match *self {
            ListRetirableGrantsError::DependencyTimeout(ref cause) => cause,
            ListRetirableGrantsError::InvalidArn(ref cause) => cause,
            ListRetirableGrantsError::InvalidMarker(ref cause) => cause,
            ListRetirableGrantsError::KMSInternal(ref cause) => cause,
            ListRetirableGrantsError::NotFound(ref cause) => cause,
            ListRetirableGrantsError::Validation(ref cause) => cause,
            ListRetirableGrantsError::Credentials(ref err) => err.description(),
            ListRetirableGrantsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListRetirableGrantsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutKeyPolicy
#[derive(Debug, PartialEq)]
pub enum PutKeyPolicyError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    ///<p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutKeyPolicyError {
    pub fn from_body(body: &str) -> PutKeyPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        PutKeyPolicyError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        PutKeyPolicyError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        PutKeyPolicyError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        PutKeyPolicyError::KMSInvalidState(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutKeyPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "MalformedPolicyDocumentException" => {
                        PutKeyPolicyError::MalformedPolicyDocument(String::from(error_message))
                    }
                    "NotFoundException" => PutKeyPolicyError::NotFound(String::from(error_message)),
                    "UnsupportedOperationException" => {
                        PutKeyPolicyError::UnsupportedOperation(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutKeyPolicyError::Validation(error_message.to_string())
                    }
                    _ => PutKeyPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutKeyPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutKeyPolicyError {
    fn from(err: serde_json::error::Error) -> PutKeyPolicyError {
        PutKeyPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutKeyPolicyError {
    fn from(err: CredentialsError) -> PutKeyPolicyError {
        PutKeyPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutKeyPolicyError {
    fn from(err: HttpDispatchError) -> PutKeyPolicyError {
        PutKeyPolicyError::HttpDispatch(err)
    }
}
impl fmt::Display for PutKeyPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutKeyPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutKeyPolicyError::DependencyTimeout(ref cause) => cause,
            PutKeyPolicyError::InvalidArn(ref cause) => cause,
            PutKeyPolicyError::KMSInternal(ref cause) => cause,
            PutKeyPolicyError::KMSInvalidState(ref cause) => cause,
            PutKeyPolicyError::LimitExceeded(ref cause) => cause,
            PutKeyPolicyError::MalformedPolicyDocument(ref cause) => cause,
            PutKeyPolicyError::NotFound(ref cause) => cause,
            PutKeyPolicyError::UnsupportedOperation(ref cause) => cause,
            PutKeyPolicyError::Validation(ref cause) => cause,
            PutKeyPolicyError::Credentials(ref err) => err.description(),
            PutKeyPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutKeyPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReEncrypt
#[derive(Debug, PartialEq)]
pub enum ReEncryptError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    ///<p>The request was rejected because the specified ciphertext has been corrupted or is otherwise invalid.</p>
    InvalidCiphertext(String),
    ///<p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    ///<p>The request was rejected because the specified <code>KeySpec</code> value is not valid.</p>
    InvalidKeyUsage(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ReEncryptError {
    pub fn from_body(body: &str) -> ReEncryptError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        ReEncryptError::DependencyTimeout(String::from(error_message))
                    }
                    "DisabledException" => ReEncryptError::Disabled(String::from(error_message)),
                    "InvalidCiphertextException" => {
                        ReEncryptError::InvalidCiphertext(String::from(error_message))
                    }
                    "InvalidGrantTokenException" => {
                        ReEncryptError::InvalidGrantToken(String::from(error_message))
                    }
                    "InvalidKeyUsageException" => {
                        ReEncryptError::InvalidKeyUsage(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ReEncryptError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        ReEncryptError::KMSInvalidState(String::from(error_message))
                    }
                    "KeyUnavailableException" => {
                        ReEncryptError::KeyUnavailable(String::from(error_message))
                    }
                    "NotFoundException" => ReEncryptError::NotFound(String::from(error_message)),
                    "ValidationException" => ReEncryptError::Validation(error_message.to_string()),
                    _ => ReEncryptError::Unknown(String::from(body)),
                }
            }
            Err(_) => ReEncryptError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ReEncryptError {
    fn from(err: serde_json::error::Error) -> ReEncryptError {
        ReEncryptError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ReEncryptError {
    fn from(err: CredentialsError) -> ReEncryptError {
        ReEncryptError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReEncryptError {
    fn from(err: HttpDispatchError) -> ReEncryptError {
        ReEncryptError::HttpDispatch(err)
    }
}
impl fmt::Display for ReEncryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReEncryptError {
    fn description(&self) -> &str {
        match *self {
            ReEncryptError::DependencyTimeout(ref cause) => cause,
            ReEncryptError::Disabled(ref cause) => cause,
            ReEncryptError::InvalidCiphertext(ref cause) => cause,
            ReEncryptError::InvalidGrantToken(ref cause) => cause,
            ReEncryptError::InvalidKeyUsage(ref cause) => cause,
            ReEncryptError::KMSInternal(ref cause) => cause,
            ReEncryptError::KMSInvalidState(ref cause) => cause,
            ReEncryptError::KeyUnavailable(ref cause) => cause,
            ReEncryptError::NotFound(ref cause) => cause,
            ReEncryptError::Validation(ref cause) => cause,
            ReEncryptError::Credentials(ref err) => err.description(),
            ReEncryptError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ReEncryptError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RetireGrant
#[derive(Debug, PartialEq)]
pub enum RetireGrantError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    ///<p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RetireGrantError {
    pub fn from_body(body: &str) -> RetireGrantError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        RetireGrantError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidGrantIdException" => {
                        RetireGrantError::InvalidGrantId(String::from(error_message))
                    }
                    "InvalidGrantTokenException" => {
                        RetireGrantError::InvalidGrantToken(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        RetireGrantError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        RetireGrantError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => RetireGrantError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        RetireGrantError::Validation(error_message.to_string())
                    }
                    _ => RetireGrantError::Unknown(String::from(body)),
                }
            }
            Err(_) => RetireGrantError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RetireGrantError {
    fn from(err: serde_json::error::Error) -> RetireGrantError {
        RetireGrantError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RetireGrantError {
    fn from(err: CredentialsError) -> RetireGrantError {
        RetireGrantError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RetireGrantError {
    fn from(err: HttpDispatchError) -> RetireGrantError {
        RetireGrantError::HttpDispatch(err)
    }
}
impl fmt::Display for RetireGrantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RetireGrantError {
    fn description(&self) -> &str {
        match *self {
            RetireGrantError::DependencyTimeout(ref cause) => cause,
            RetireGrantError::InvalidGrantId(ref cause) => cause,
            RetireGrantError::InvalidGrantToken(ref cause) => cause,
            RetireGrantError::KMSInternal(ref cause) => cause,
            RetireGrantError::KMSInvalidState(ref cause) => cause,
            RetireGrantError::NotFound(ref cause) => cause,
            RetireGrantError::Validation(ref cause) => cause,
            RetireGrantError::Credentials(ref err) => err.description(),
            RetireGrantError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RetireGrantError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RevokeGrant
#[derive(Debug, PartialEq)]
pub enum RevokeGrantError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RevokeGrantError {
    pub fn from_body(body: &str) -> RevokeGrantError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        RevokeGrantError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        RevokeGrantError::InvalidArn(String::from(error_message))
                    }
                    "InvalidGrantIdException" => {
                        RevokeGrantError::InvalidGrantId(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        RevokeGrantError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        RevokeGrantError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => RevokeGrantError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        RevokeGrantError::Validation(error_message.to_string())
                    }
                    _ => RevokeGrantError::Unknown(String::from(body)),
                }
            }
            Err(_) => RevokeGrantError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RevokeGrantError {
    fn from(err: serde_json::error::Error) -> RevokeGrantError {
        RevokeGrantError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RevokeGrantError {
    fn from(err: CredentialsError) -> RevokeGrantError {
        RevokeGrantError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RevokeGrantError {
    fn from(err: HttpDispatchError) -> RevokeGrantError {
        RevokeGrantError::HttpDispatch(err)
    }
}
impl fmt::Display for RevokeGrantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeGrantError {
    fn description(&self) -> &str {
        match *self {
            RevokeGrantError::DependencyTimeout(ref cause) => cause,
            RevokeGrantError::InvalidArn(ref cause) => cause,
            RevokeGrantError::InvalidGrantId(ref cause) => cause,
            RevokeGrantError::KMSInternal(ref cause) => cause,
            RevokeGrantError::KMSInvalidState(ref cause) => cause,
            RevokeGrantError::NotFound(ref cause) => cause,
            RevokeGrantError::Validation(ref cause) => cause,
            RevokeGrantError::Credentials(ref err) => err.description(),
            RevokeGrantError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RevokeGrantError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ScheduleKeyDeletion
#[derive(Debug, PartialEq)]
pub enum ScheduleKeyDeletionError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ScheduleKeyDeletionError {
    pub fn from_body(body: &str) -> ScheduleKeyDeletionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        ScheduleKeyDeletionError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        ScheduleKeyDeletionError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        ScheduleKeyDeletionError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        ScheduleKeyDeletionError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ScheduleKeyDeletionError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ScheduleKeyDeletionError::Validation(error_message.to_string())
                    }
                    _ => ScheduleKeyDeletionError::Unknown(String::from(body)),
                }
            }
            Err(_) => ScheduleKeyDeletionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ScheduleKeyDeletionError {
    fn from(err: serde_json::error::Error) -> ScheduleKeyDeletionError {
        ScheduleKeyDeletionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ScheduleKeyDeletionError {
    fn from(err: CredentialsError) -> ScheduleKeyDeletionError {
        ScheduleKeyDeletionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ScheduleKeyDeletionError {
    fn from(err: HttpDispatchError) -> ScheduleKeyDeletionError {
        ScheduleKeyDeletionError::HttpDispatch(err)
    }
}
impl fmt::Display for ScheduleKeyDeletionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ScheduleKeyDeletionError {
    fn description(&self) -> &str {
        match *self {
            ScheduleKeyDeletionError::DependencyTimeout(ref cause) => cause,
            ScheduleKeyDeletionError::InvalidArn(ref cause) => cause,
            ScheduleKeyDeletionError::KMSInternal(ref cause) => cause,
            ScheduleKeyDeletionError::KMSInvalidState(ref cause) => cause,
            ScheduleKeyDeletionError::NotFound(ref cause) => cause,
            ScheduleKeyDeletionError::Validation(ref cause) => cause,
            ScheduleKeyDeletionError::Credentials(ref err) => err.description(),
            ScheduleKeyDeletionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ScheduleKeyDeletionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArnException" => {
                        TagResourceError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        TagResourceError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        TagResourceError::KMSInvalidState(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        TagResourceError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => TagResourceError::NotFound(String::from(error_message)),
                    "TagException" => TagResourceError::Tag(String::from(error_message)),
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::InvalidArn(ref cause) => cause,
            TagResourceError::KMSInternal(ref cause) => cause,
            TagResourceError::KMSInvalidState(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::NotFound(ref cause) => cause,
            TagResourceError::Tag(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    ///<p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArnException" => {
                        UntagResourceError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        UntagResourceError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        UntagResourceError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UntagResourceError::NotFound(String::from(error_message))
                    }
                    "TagException" => UntagResourceError::Tag(String::from(error_message)),
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::InvalidArn(ref cause) => cause,
            UntagResourceError::KMSInternal(ref cause) => cause,
            UntagResourceError::KMSInvalidState(ref cause) => cause,
            UntagResourceError::NotFound(ref cause) => cause,
            UntagResourceError::Tag(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAlias
#[derive(Debug, PartialEq)]
pub enum UpdateAliasError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateAliasError {
    pub fn from_body(body: &str) -> UpdateAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        UpdateAliasError::DependencyTimeout(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        UpdateAliasError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        UpdateAliasError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => UpdateAliasError::NotFound(String::from(error_message)),
                    "ValidationException" => {
                        UpdateAliasError::Validation(error_message.to_string())
                    }
                    _ => UpdateAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAliasError {
    fn from(err: serde_json::error::Error) -> UpdateAliasError {
        UpdateAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAliasError {
    fn from(err: CredentialsError) -> UpdateAliasError {
        UpdateAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAliasError {
    fn from(err: HttpDispatchError) -> UpdateAliasError {
        UpdateAliasError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAliasError {
    fn description(&self) -> &str {
        match *self {
            UpdateAliasError::DependencyTimeout(ref cause) => cause,
            UpdateAliasError::KMSInternal(ref cause) => cause,
            UpdateAliasError::KMSInvalidState(ref cause) => cause,
            UpdateAliasError::NotFound(ref cause) => cause,
            UpdateAliasError::Validation(ref cause) => cause,
            UpdateAliasError::Credentials(ref err) => err.description(),
            UpdateAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateKeyDescription
#[derive(Debug, PartialEq)]
pub enum UpdateKeyDescriptionError {
    ///<p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    ///<p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    ///<p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    ///<p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    ///<p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateKeyDescriptionError {
    pub fn from_body(body: &str) -> UpdateKeyDescriptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        UpdateKeyDescriptionError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        UpdateKeyDescriptionError::InvalidArn(String::from(error_message))
                    }
                    "KMSInternalException" => {
                        UpdateKeyDescriptionError::KMSInternal(String::from(error_message))
                    }
                    "KMSInvalidStateException" => {
                        UpdateKeyDescriptionError::KMSInvalidState(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateKeyDescriptionError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateKeyDescriptionError::Validation(error_message.to_string())
                    }
                    _ => UpdateKeyDescriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateKeyDescriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateKeyDescriptionError {
    fn from(err: serde_json::error::Error) -> UpdateKeyDescriptionError {
        UpdateKeyDescriptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateKeyDescriptionError {
    fn from(err: CredentialsError) -> UpdateKeyDescriptionError {
        UpdateKeyDescriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateKeyDescriptionError {
    fn from(err: HttpDispatchError) -> UpdateKeyDescriptionError {
        UpdateKeyDescriptionError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateKeyDescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateKeyDescriptionError {
    fn description(&self) -> &str {
        match *self {
            UpdateKeyDescriptionError::DependencyTimeout(ref cause) => cause,
            UpdateKeyDescriptionError::InvalidArn(ref cause) => cause,
            UpdateKeyDescriptionError::KMSInternal(ref cause) => cause,
            UpdateKeyDescriptionError::KMSInvalidState(ref cause) => cause,
            UpdateKeyDescriptionError::NotFound(ref cause) => cause,
            UpdateKeyDescriptionError::Validation(ref cause) => cause,
            UpdateKeyDescriptionError::Credentials(ref err) => err.description(),
            UpdateKeyDescriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateKeyDescriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the KMS API. KMS clients implement this trait.
pub trait Kms {
    #[doc="<p>Cancels the deletion of a customer master key (CMK). When this operation is successful, the CMK is set to the <code>Disabled</code> state. To enable a CMK, use <a>EnableKey</a>.</p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html\">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn cancel_key_deletion(&self,
                           input: &CancelKeyDeletionRequest)
                           -> Result<CancelKeyDeletionResponse, CancelKeyDeletionError>;


    #[doc="<p>Creates a display name for a customer master key. An alias can be used to identify a key and should be unique. The console enforces a one-to-one mapping between the alias and a key. An alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). An alias must start with the word \"alias\" followed by a forward slash (alias/). An alias that begins with \"aws\" after the forward slash (alias/aws...) is reserved by Amazon Web Services (AWS).</p> <p>The alias and the key it is mapped to must be in the same AWS account and the same region.</p> <p>To map an alias to a different key, call <a>UpdateAlias</a>.</p>"]
    fn create_alias(&self, input: &CreateAliasRequest) -> Result<(), CreateAliasError>;


    #[doc="<p>Adds a grant to a key to specify who can use the key and under what conditions. Grants are alternate permission mechanisms to key policies.</p> <p>For more information about grants, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/grants.html\">Grants</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn create_grant(&self,
                    input: &CreateGrantRequest)
                    -> Result<CreateGrantResponse, CreateGrantError>;


    #[doc="<p>Creates a customer master key (CMK).</p> <p>You can use a CMK to encrypt small amounts of data (4 KiB or less) directly, but CMKs are more commonly used to encrypt data encryption keys (DEKs), which are used to encrypt raw data. For more information about DEKs and the difference between CMKs and DEKs, see the following:</p> <ul> <li> <p>The <a>GenerateDataKey</a> operation</p> </li> <li> <p> <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html\">AWS Key Management Service Concepts</a> in the <i>AWS Key Management Service Developer Guide</i> </p> </li> </ul>"]
    fn create_key(&self, input: &CreateKeyRequest) -> Result<CreateKeyResponse, CreateKeyError>;


    #[doc="<p>Decrypts ciphertext. Ciphertext is plaintext that has been previously encrypted by using any of the following functions:</p> <ul> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> </ul> <p>Note that if a caller has been granted access permissions to all keys (through, for example, IAM user policies that grant <code>Decrypt</code> permission on all resources), then ciphertext encrypted by using keys in other accounts where the key grants access to the caller can be decrypted. To remedy this, we recommend that you do not grant <code>Decrypt</code> access in an IAM user policy. Instead grant <code>Decrypt</code> access only in key policies. If you must grant <code>Decrypt</code> access in an IAM user policy, you should scope the resource to specific keys or to specific trusted accounts.</p>"]
    fn decrypt(&self, input: &DecryptRequest) -> Result<DecryptResponse, DecryptError>;


    #[doc="<p>Deletes the specified alias. To map an alias to a different key, call <a>UpdateAlias</a>.</p>"]
    fn delete_alias(&self, input: &DeleteAliasRequest) -> Result<(), DeleteAliasError>;


    #[doc="<p>Deletes key material that you previously imported and makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html\">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK's state. Otherwise, it changes the CMK's state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p>"]
    fn delete_imported_key_material(&self,
                                    input: &DeleteImportedKeyMaterialRequest)
                                    -> Result<(), DeleteImportedKeyMaterialError>;


    #[doc="<p>Provides detailed information about the specified customer master key.</p>"]
    fn describe_key(&self,
                    input: &DescribeKeyRequest)
                    -> Result<DescribeKeyResponse, DescribeKeyError>;


    #[doc="<p>Sets the state of a customer master key (CMK) to disabled, thereby preventing its use for cryptographic operations. For more information about how key state affects the use of a CMK, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html\">How Key State Affects the Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn disable_key(&self, input: &DisableKeyRequest) -> Result<(), DisableKeyError>;


    #[doc="<p>Disables rotation of the specified key.</p>"]
    fn disable_key_rotation(&self,
                            input: &DisableKeyRotationRequest)
                            -> Result<(), DisableKeyRotationError>;


    #[doc="<p>Marks a key as enabled, thereby permitting its use.</p>"]
    fn enable_key(&self, input: &EnableKeyRequest) -> Result<(), EnableKeyError>;


    #[doc="<p>Enables rotation of the specified customer master key.</p>"]
    fn enable_key_rotation(&self,
                           input: &EnableKeyRotationRequest)
                           -> Result<(), EnableKeyRotationError>;


    #[doc="<p>Encrypts plaintext into ciphertext by using a customer master key. The <code>Encrypt</code> function has two primary use cases:</p> <ul> <li> <p>You can encrypt up to 4 KB of arbitrary data such as an RSA key, a database password, or other sensitive customer information.</p> </li> <li> <p>If you are moving encrypted data from one region to another, you can use this API to encrypt in the new region the plaintext data key that was used to encrypt the data in the original region. This provides you with an encrypted copy of the data key that can be decrypted in the new region and used there to decrypt the encrypted data.</p> </li> </ul> <p>Unless you are moving encrypted data from one region to another, you don't use this function to encrypt a generated data key within a region. You retrieve data keys already encrypted by calling the <a>GenerateDataKey</a> or <a>GenerateDataKeyWithoutPlaintext</a> function. Data keys don't need to be encrypted again by calling <code>Encrypt</code>.</p> <p>If you want to encrypt data locally in your application, you can use the <code>GenerateDataKey</code> function to return a plaintext data encryption key and a copy of the key encrypted under the customer master key (CMK) of your choosing.</p>"]
    fn encrypt(&self, input: &EncryptRequest) -> Result<EncryptResponse, EncryptError>;


    #[doc="<p>Returns a data encryption key that you can use in your application to encrypt data locally.</p> <p>You must specify the customer master key (CMK) under which to generate the data key. You must also specify the length of the data key using either the <code>KeySpec</code> or <code>NumberOfBytes</code> field. You must specify one field or the other, but not both. For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use <code>KeySpec</code>.</p> <p>This operation returns a plaintext copy of the data key in the <code>Plaintext</code> field of the response, and an encrypted copy of the data key in the <code>CiphertextBlob</code> field. The data key is encrypted under the CMK specified in the <code>KeyId</code> field of the request.</p> <p>We recommend that you use the following pattern to encrypt data locally in your application:</p> <ol> <li> <p>Use this operation (<code>GenerateDataKey</code>) to retrieve a data encryption key.</p> </li> <li> <p>Use the plaintext data encryption key (returned in the <code>Plaintext</code> field of the response) to encrypt data locally, then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (returned in the <code>CiphertextBlob</code> field of the response) alongside the locally encrypted data.</p> </li> </ol> <p>To decrypt data locally:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key into a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data locally, then erase the plaintext data key from memory.</p> </li> </ol> <p>To return only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To return a random byte string that is cryptographically secure, use <a>GenerateRandom</a>.</p> <p>If you use the optional <code>EncryptionContext</code> field, you must store at least enough information to be able to reconstruct the full encryption context when you later send the ciphertext to the <a>Decrypt</a> operation. It is a good practice to choose an encryption context that you can reconstruct on the fly to better secure the ciphertext. For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html\">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn generate_data_key(&self,
                         input: &GenerateDataKeyRequest)
                         -> Result<GenerateDataKeyResponse, GenerateDataKeyError>;


    #[doc="<p>Returns a data encryption key encrypted under a customer master key (CMK). This operation is identical to <a>GenerateDataKey</a> but returns only the encrypted copy of the data key.</p> <p>This operation is useful in a system that has multiple components with different degrees of trust. For example, consider a system that stores encrypted data in containers. Each container stores the encrypted data and an encrypted copy of the data key. One component of the system, called the <i>control plane</i>, creates new containers. When it creates a new container, it uses this operation (<code>GenerateDataKeyWithoutPlaintext</code>) to get an encrypted data key and then stores it in the container. Later, a different component of the system, called the <i>data plane</i>, puts encrypted data into the containers. To do this, it passes the encrypted data key to the <a>Decrypt</a> operation, then uses the returned plaintext data key to encrypt data, and finally stores the encrypted data in the container. In this system, the control plane never sees the plaintext data key.</p>"]
    fn generate_data_key_without_plaintext
        (&self,
         input: &GenerateDataKeyWithoutPlaintextRequest)
         -> Result<GenerateDataKeyWithoutPlaintextResponse, GenerateDataKeyWithoutPlaintextError>;


    #[doc="<p>Returns a random byte string that is cryptographically secure.</p> <p>For more information about entropy and random number generation, see the <a href=\"https://d0.awsstatic.com/whitepapers/KMS-Cryptographic-Details.pdf\">AWS Key Management Service Cryptographic Details</a> whitepaper.</p>"]
    fn generate_random(&self,
                       input: &GenerateRandomRequest)
                       -> Result<GenerateRandomResponse, GenerateRandomError>;


    #[doc="<p>Retrieves a policy attached to the specified key.</p>"]
    fn get_key_policy(&self,
                      input: &GetKeyPolicyRequest)
                      -> Result<GetKeyPolicyResponse, GetKeyPolicyError>;


    #[doc="<p>Retrieves a Boolean value that indicates whether key rotation is enabled for the specified key.</p>"]
    fn get_key_rotation_status
        (&self,
         input: &GetKeyRotationStatusRequest)
         -> Result<GetKeyRotationStatusResponse, GetKeyRotationStatusError>;


    #[doc="<p>Returns the items you need in order to import key material into AWS KMS from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html\">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You must specify the key ID of the customer master key (CMK) into which you will import key material. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request. The public key and import token from the same response must be used together. These items are valid for 24 hours, after which they cannot be used for a subsequent <a>ImportKeyMaterial</a> request. To retrieve new ones, send another <code>GetParametersForImport</code> request.</p>"]
    fn get_parameters_for_import
        (&self,
         input: &GetParametersForImportRequest)
         -> Result<GetParametersForImportResponse, GetParametersForImportError>;


    #[doc="<p>Imports key material into an AWS KMS customer master key (CMK) from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html\">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You must specify the key ID of the CMK to import the key material into. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also send an import token and the encrypted key material. Send the import token that you received in the same <a>GetParametersForImport</a> response that contained the public key that you used to encrypt the key material. You must also specify whether the key material expires and if so, when. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. To use the CMK again, you can reimport the same key material. If you set an expiration date, you can change it only by reimporting the same key material and specifying a new expiration date.</p> <p>When this operation is successful, the specified CMK's key state changes to <code>Enabled</code>, and you can use the CMK.</p> <p>After you successfully import key material into a CMK, you can reimport the same key material into that CMK, but you cannot import different key material.</p>"]
    fn import_key_material(&self,
                           input: &ImportKeyMaterialRequest)
                           -> Result<ImportKeyMaterialResponse, ImportKeyMaterialError>;


    #[doc="<p>Lists all of the key aliases in the account.</p>"]
    fn list_aliases(&self,
                    input: &ListAliasesRequest)
                    -> Result<ListAliasesResponse, ListAliasesError>;


    #[doc="<p>List the grants for a specified key.</p>"]
    fn list_grants(&self,
                   input: &ListGrantsRequest)
                   -> Result<ListGrantsResponse, ListGrantsError>;


    #[doc="<p>Retrieves a list of policies attached to a key.</p>"]
    fn list_key_policies(&self,
                         input: &ListKeyPoliciesRequest)
                         -> Result<ListKeyPoliciesResponse, ListKeyPoliciesError>;


    #[doc="<p>Lists the customer master keys.</p>"]
    fn list_keys(&self, input: &ListKeysRequest) -> Result<ListKeysResponse, ListKeysError>;


    #[doc="<p>Returns a list of all tags for the specified customer master key (CMK).</p>"]
    fn list_resource_tags(&self,
                          input: &ListResourceTagsRequest)
                          -> Result<ListResourceTagsResponse, ListResourceTagsError>;


    #[doc="<p>Returns a list of all grants for which the grant's <code>RetiringPrincipal</code> matches the one specified.</p> <p>A typical use is to list all grants that you are able to retire. To retire a grant, use <a>RetireGrant</a>.</p>"]
    fn list_retirable_grants(&self,
                             input: &ListRetirableGrantsRequest)
                             -> Result<ListGrantsResponse, ListRetirableGrantsError>;


    #[doc="<p>Attaches a key policy to the specified customer master key (CMK).</p> <p>For more information about key policies, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html\">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn put_key_policy(&self, input: &PutKeyPolicyRequest) -> Result<(), PutKeyPolicyError>;


    #[doc="<p>Encrypts data on the server side with a new customer master key (CMK) without exposing the plaintext of the data on the client side. The data is first decrypted and then reencrypted. You can also use this operation to change the encryption context of a ciphertext.</p> <p>Unlike other operations, <code>ReEncrypt</code> is authorized twice, once as <code>ReEncryptFrom</code> on the source CMK and once as <code>ReEncryptTo</code> on the destination CMK. We recommend that you include the <code>\"kms:ReEncrypt*\"</code> permission in your <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html\">key policies</a> to permit reencryption from or to the CMK. This permission is automatically included in the key policy when you create a CMK through the console, but you must include it manually when you create a CMK programmatically or when you set a key policy with the <a>PutKeyPolicy</a> operation.</p>"]
    fn re_encrypt(&self, input: &ReEncryptRequest) -> Result<ReEncryptResponse, ReEncryptError>;


    #[doc="<p>Retires a grant. To clean up, you can retire a grant when you're done using it. You should revoke a grant when you intend to actively deny operations that depend on it. The following are permitted to call this API:</p> <ul> <li> <p>The AWS account (root user) under which the grant was created</p> </li> <li> <p>The <code>RetiringPrincipal</code>, if present in the grant</p> </li> <li> <p>The <code>GranteePrincipal</code>, if <code>RetireGrant</code> is an operation specified in the grant</p> </li> </ul> <p>You must identify the grant to retire by its grant token or by a combination of the grant ID and the Amazon Resource Name (ARN) of the customer master key (CMK). A grant token is a unique variable-length base64-encoded string. A grant ID is a 64 character unique identifier of a grant. The <a>CreateGrant</a> operation returns both.</p>"]
    fn retire_grant(&self, input: &RetireGrantRequest) -> Result<(), RetireGrantError>;


    #[doc="<p>Revokes a grant. You can revoke a grant to actively deny operations that depend on it.</p>"]
    fn revoke_grant(&self, input: &RevokeGrantRequest) -> Result<(), RevokeGrantError>;


    #[doc="<p>Schedules the deletion of a customer master key (CMK). You may provide a waiting period, specified in days, before deletion occurs. If you do not provide a waiting period, the default period of 30 days is used. When this operation is successful, the state of the CMK changes to <code>PendingDeletion</code>. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK and all AWS KMS data associated with it, including all aliases that refer to it.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is rendered unrecoverable. To restrict the use of a CMK without deleting it, use <a>DisableKey</a>.</p> </important> <p>For more information about scheduling a CMK for deletion, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html\">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn schedule_key_deletion(&self,
                             input: &ScheduleKeyDeletionRequest)
                             -> Result<ScheduleKeyDeletionResponse, ScheduleKeyDeletionError>;


    #[doc="<p>Adds or overwrites one or more tags for the specified customer master key (CMK). </p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>You cannot use the same tag key more than once per CMK. For example, consider a CMK with one tag whose tag key is <code>Purpose</code> and tag value is <code>Test</code>. If you send a <code>TagResource</code> request for this CMK with a tag key of <code>Purpose</code> and a tag value of <code>Prod</code>, it does not create a second tag. Instead, the original tag is overwritten with the new tag value.</p>"]
    fn tag_resource(&self, input: &TagResourceRequest) -> Result<(), TagResourceError>;


    #[doc="<p>Removes the specified tag or tags from the specified customer master key (CMK). </p> <p>To remove a tag, you specify the tag key for each tag to remove. You do not specify the tag value. To overwrite the tag value for an existing tag, use <a>TagResource</a>.</p>"]
    fn untag_resource(&self, input: &UntagResourceRequest) -> Result<(), UntagResourceError>;


    #[doc="<p>Updates an alias to map it to a different key.</p> <p>An alias is not a property of a key. Therefore, an alias can be mapped to and unmapped from an existing key without changing the properties of the key.</p> <p>An alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). An alias must start with the word \"alias\" followed by a forward slash (alias/). An alias that begins with \"aws\" after the forward slash (alias/aws...) is reserved by Amazon Web Services (AWS).</p> <p>The alias and the key it is mapped to must be in the same AWS account and the same region.</p>"]
    fn update_alias(&self, input: &UpdateAliasRequest) -> Result<(), UpdateAliasError>;


    #[doc="<p>Updates the description of a customer master key (CMK).</p>"]
    fn update_key_description(&self,
                              input: &UpdateKeyDescriptionRequest)
                              -> Result<(), UpdateKeyDescriptionError>;
}
/// A client for the KMS API.
pub struct KmsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> KmsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        KmsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Kms for KmsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Cancels the deletion of a customer master key (CMK). When this operation is successful, the CMK is set to the <code>Disabled</code> state. To enable a CMK, use <a>EnableKey</a>.</p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html\">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn cancel_key_deletion(&self,
                           input: &CancelKeyDeletionRequest)
                           -> Result<CancelKeyDeletionResponse, CancelKeyDeletionError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CancelKeyDeletion");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<CancelKeyDeletionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(CancelKeyDeletionError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Creates a display name for a customer master key. An alias can be used to identify a key and should be unique. The console enforces a one-to-one mapping between the alias and a key. An alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). An alias must start with the word \"alias\" followed by a forward slash (alias/). An alias that begins with \"aws\" after the forward slash (alias/aws...) is reserved by Amazon Web Services (AWS).</p> <p>The alias and the key it is mapped to must be in the same AWS account and the same region.</p> <p>To map an alias to a different key, call <a>UpdateAlias</a>.</p>"]
    fn create_alias(&self, input: &CreateAliasRequest) -> Result<(), CreateAliasError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(CreateAliasError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Adds a grant to a key to specify who can use the key and under what conditions. Grants are alternate permission mechanisms to key policies.</p> <p>For more information about grants, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/grants.html\">Grants</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn create_grant(&self,
                    input: &CreateGrantRequest)
                    -> Result<CreateGrantResponse, CreateGrantError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateGrant");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<CreateGrantResponse>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                       .unwrap())
        } else {
            Err(CreateGrantError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Creates a customer master key (CMK).</p> <p>You can use a CMK to encrypt small amounts of data (4 KiB or less) directly, but CMKs are more commonly used to encrypt data encryption keys (DEKs), which are used to encrypt raw data. For more information about DEKs and the difference between CMKs and DEKs, see the following:</p> <ul> <li> <p>The <a>GenerateDataKey</a> operation</p> </li> <li> <p> <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html\">AWS Key Management Service Concepts</a> in the <i>AWS Key Management Service Developer Guide</i> </p> </li> </ul>"]
    fn create_key(&self, input: &CreateKeyRequest) -> Result<CreateKeyResponse, CreateKeyError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateKey");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<CreateKeyResponse>(String::from_utf8_lossy(&response.body)
                                                             .as_ref())
                       .unwrap())
        } else {
            Err(CreateKeyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Decrypts ciphertext. Ciphertext is plaintext that has been previously encrypted by using any of the following functions:</p> <ul> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> </ul> <p>Note that if a caller has been granted access permissions to all keys (through, for example, IAM user policies that grant <code>Decrypt</code> permission on all resources), then ciphertext encrypted by using keys in other accounts where the key grants access to the caller can be decrypted. To remedy this, we recommend that you do not grant <code>Decrypt</code> access in an IAM user policy. Instead grant <code>Decrypt</code> access only in key policies. If you must grant <code>Decrypt</code> access in an IAM user policy, you should scope the resource to specific keys or to specific trusted accounts.</p>"]
    fn decrypt(&self, input: &DecryptRequest) -> Result<DecryptResponse, DecryptError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Decrypt");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DecryptResponse>(String::from_utf8_lossy(&response.body)
                                                           .as_ref())
                       .unwrap())
        } else {
            Err(DecryptError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Deletes the specified alias. To map an alias to a different key, call <a>UpdateAlias</a>.</p>"]
    fn delete_alias(&self, input: &DeleteAliasRequest) -> Result<(), DeleteAliasError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DeleteAliasError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Deletes key material that you previously imported and makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html\">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK's state. Otherwise, it changes the CMK's state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p>"]
    fn delete_imported_key_material(&self,
                                    input: &DeleteImportedKeyMaterialRequest)
                                    -> Result<(), DeleteImportedKeyMaterialError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteImportedKeyMaterial");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DeleteImportedKeyMaterialError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
        }
    }


    #[doc="<p>Provides detailed information about the specified customer master key.</p>"]
    fn describe_key(&self,
                    input: &DescribeKeyRequest)
                    -> Result<DescribeKeyResponse, DescribeKeyError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DescribeKey");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<DescribeKeyResponse>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                       .unwrap())
        } else {
            Err(DescribeKeyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Sets the state of a customer master key (CMK) to disabled, thereby preventing its use for cryptographic operations. For more information about how key state affects the use of a CMK, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html\">How Key State Affects the Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn disable_key(&self, input: &DisableKeyRequest) -> Result<(), DisableKeyError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisableKey");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DisableKeyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Disables rotation of the specified key.</p>"]
    fn disable_key_rotation(&self,
                            input: &DisableKeyRotationRequest)
                            -> Result<(), DisableKeyRotationError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisableKeyRotation");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(DisableKeyRotationError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
        }
    }


    #[doc="<p>Marks a key as enabled, thereby permitting its use.</p>"]
    fn enable_key(&self, input: &EnableKeyRequest) -> Result<(), EnableKeyError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.EnableKey");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(EnableKeyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Enables rotation of the specified customer master key.</p>"]
    fn enable_key_rotation(&self,
                           input: &EnableKeyRotationRequest)
                           -> Result<(), EnableKeyRotationError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.EnableKeyRotation");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(EnableKeyRotationError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Encrypts plaintext into ciphertext by using a customer master key. The <code>Encrypt</code> function has two primary use cases:</p> <ul> <li> <p>You can encrypt up to 4 KB of arbitrary data such as an RSA key, a database password, or other sensitive customer information.</p> </li> <li> <p>If you are moving encrypted data from one region to another, you can use this API to encrypt in the new region the plaintext data key that was used to encrypt the data in the original region. This provides you with an encrypted copy of the data key that can be decrypted in the new region and used there to decrypt the encrypted data.</p> </li> </ul> <p>Unless you are moving encrypted data from one region to another, you don't use this function to encrypt a generated data key within a region. You retrieve data keys already encrypted by calling the <a>GenerateDataKey</a> or <a>GenerateDataKeyWithoutPlaintext</a> function. Data keys don't need to be encrypted again by calling <code>Encrypt</code>.</p> <p>If you want to encrypt data locally in your application, you can use the <code>GenerateDataKey</code> function to return a plaintext data encryption key and a copy of the key encrypted under the customer master key (CMK) of your choosing.</p>"]
    fn encrypt(&self, input: &EncryptRequest) -> Result<EncryptResponse, EncryptError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Encrypt");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<EncryptResponse>(String::from_utf8_lossy(&response.body)
                                                           .as_ref())
                       .unwrap())
        } else {
            Err(EncryptError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Returns a data encryption key that you can use in your application to encrypt data locally.</p> <p>You must specify the customer master key (CMK) under which to generate the data key. You must also specify the length of the data key using either the <code>KeySpec</code> or <code>NumberOfBytes</code> field. You must specify one field or the other, but not both. For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use <code>KeySpec</code>.</p> <p>This operation returns a plaintext copy of the data key in the <code>Plaintext</code> field of the response, and an encrypted copy of the data key in the <code>CiphertextBlob</code> field. The data key is encrypted under the CMK specified in the <code>KeyId</code> field of the request.</p> <p>We recommend that you use the following pattern to encrypt data locally in your application:</p> <ol> <li> <p>Use this operation (<code>GenerateDataKey</code>) to retrieve a data encryption key.</p> </li> <li> <p>Use the plaintext data encryption key (returned in the <code>Plaintext</code> field of the response) to encrypt data locally, then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (returned in the <code>CiphertextBlob</code> field of the response) alongside the locally encrypted data.</p> </li> </ol> <p>To decrypt data locally:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key into a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data locally, then erase the plaintext data key from memory.</p> </li> </ol> <p>To return only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To return a random byte string that is cryptographically secure, use <a>GenerateRandom</a>.</p> <p>If you use the optional <code>EncryptionContext</code> field, you must store at least enough information to be able to reconstruct the full encryption context when you later send the ciphertext to the <a>Decrypt</a> operation. It is a good practice to choose an encryption context that you can reconstruct on the fly to better secure the ciphertext. For more information, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html\">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn generate_data_key(&self,
                         input: &GenerateDataKeyRequest)
                         -> Result<GenerateDataKeyResponse, GenerateDataKeyError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateDataKey");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GenerateDataKeyResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(GenerateDataKeyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Returns a data encryption key encrypted under a customer master key (CMK). This operation is identical to <a>GenerateDataKey</a> but returns only the encrypted copy of the data key.</p> <p>This operation is useful in a system that has multiple components with different degrees of trust. For example, consider a system that stores encrypted data in containers. Each container stores the encrypted data and an encrypted copy of the data key. One component of the system, called the <i>control plane</i>, creates new containers. When it creates a new container, it uses this operation (<code>GenerateDataKeyWithoutPlaintext</code>) to get an encrypted data key and then stores it in the container. Later, a different component of the system, called the <i>data plane</i>, puts encrypted data into the containers. To do this, it passes the encrypted data key to the <a>Decrypt</a> operation, then uses the returned plaintext data key to encrypt data, and finally stores the encrypted data in the container. In this system, the control plane never sees the plaintext data key.</p>"]
    fn generate_data_key_without_plaintext
        (&self,
         input: &GenerateDataKeyWithoutPlaintextRequest)
         -> Result<GenerateDataKeyWithoutPlaintextResponse, GenerateDataKeyWithoutPlaintextError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "TrentService.GenerateDataKeyWithoutPlaintext");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GenerateDataKeyWithoutPlaintextResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(GenerateDataKeyWithoutPlaintextError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Returns a random byte string that is cryptographically secure.</p> <p>For more information about entropy and random number generation, see the <a href=\"https://d0.awsstatic.com/whitepapers/KMS-Cryptographic-Details.pdf\">AWS Key Management Service Cryptographic Details</a> whitepaper.</p>"]
    fn generate_random(&self,
                       input: &GenerateRandomRequest)
                       -> Result<GenerateRandomResponse, GenerateRandomError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateRandom");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GenerateRandomResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(GenerateRandomError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Retrieves a policy attached to the specified key.</p>"]
    fn get_key_policy(&self,
                      input: &GetKeyPolicyRequest)
                      -> Result<GetKeyPolicyResponse, GetKeyPolicyError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetKeyPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GetKeyPolicyResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(GetKeyPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Retrieves a Boolean value that indicates whether key rotation is enabled for the specified key.</p>"]
    fn get_key_rotation_status
        (&self,
         input: &GetKeyRotationStatusRequest)
         -> Result<GetKeyRotationStatusResponse, GetKeyRotationStatusError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetKeyRotationStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GetKeyRotationStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(GetKeyRotationStatusError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
        }
    }


    #[doc="<p>Returns the items you need in order to import key material into AWS KMS from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html\">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You must specify the key ID of the customer master key (CMK) into which you will import key material. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request. The public key and import token from the same response must be used together. These items are valid for 24 hours, after which they cannot be used for a subsequent <a>ImportKeyMaterial</a> request. To retrieve new ones, send another <code>GetParametersForImport</code> request.</p>"]
    fn get_parameters_for_import
        (&self,
         input: &GetParametersForImportRequest)
         -> Result<GetParametersForImportResponse, GetParametersForImportError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetParametersForImport");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<GetParametersForImportResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(GetParametersForImportError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
        }
    }


    #[doc="<p>Imports key material into an AWS KMS customer master key (CMK) from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html\">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You must specify the key ID of the CMK to import the key material into. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also send an import token and the encrypted key material. Send the import token that you received in the same <a>GetParametersForImport</a> response that contained the public key that you used to encrypt the key material. You must also specify whether the key material expires and if so, when. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. To use the CMK again, you can reimport the same key material. If you set an expiration date, you can change it only by reimporting the same key material and specifying a new expiration date.</p> <p>When this operation is successful, the specified CMK's key state changes to <code>Enabled</code>, and you can use the CMK.</p> <p>After you successfully import key material into a CMK, you can reimport the same key material into that CMK, but you cannot import different key material.</p>"]
    fn import_key_material(&self,
                           input: &ImportKeyMaterialRequest)
                           -> Result<ImportKeyMaterialResponse, ImportKeyMaterialError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ImportKeyMaterial");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ImportKeyMaterialResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(ImportKeyMaterialError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Lists all of the key aliases in the account.</p>"]
    fn list_aliases(&self,
                    input: &ListAliasesRequest)
                    -> Result<ListAliasesResponse, ListAliasesError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListAliases");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ListAliasesResponse>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                       .unwrap())
        } else {
            Err(ListAliasesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>List the grants for a specified key.</p>"]
    fn list_grants(&self,
                   input: &ListGrantsRequest)
                   -> Result<ListGrantsResponse, ListGrantsError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListGrants");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ListGrantsResponse>(String::from_utf8_lossy(&response.body)
                                                              .as_ref())
                       .unwrap())
        } else {
            Err(ListGrantsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Retrieves a list of policies attached to a key.</p>"]
    fn list_key_policies(&self,
                         input: &ListKeyPoliciesRequest)
                         -> Result<ListKeyPoliciesResponse, ListKeyPoliciesError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListKeyPolicies");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ListKeyPoliciesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(ListKeyPoliciesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Lists the customer master keys.</p>"]
    fn list_keys(&self, input: &ListKeysRequest) -> Result<ListKeysResponse, ListKeysError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListKeys");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ListKeysResponse>(String::from_utf8_lossy(&response.body)
                                                            .as_ref())
                       .unwrap())
        } else {
            Err(ListKeysError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Returns a list of all tags for the specified customer master key (CMK).</p>"]
    fn list_resource_tags(&self,
                          input: &ListResourceTagsRequest)
                          -> Result<ListResourceTagsResponse, ListResourceTagsError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListResourceTags");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ListResourceTagsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(ListResourceTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Returns a list of all grants for which the grant's <code>RetiringPrincipal</code> matches the one specified.</p> <p>A typical use is to list all grants that you are able to retire. To retire a grant, use <a>RetireGrant</a>.</p>"]
    fn list_retirable_grants(&self,
                             input: &ListRetirableGrantsRequest)
                             -> Result<ListGrantsResponse, ListRetirableGrantsError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListRetirableGrants");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ListGrantsResponse>(String::from_utf8_lossy(&response.body)
                                                              .as_ref())
                       .unwrap())
        } else {
            Err(ListRetirableGrantsError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
        }
    }


    #[doc="<p>Attaches a key policy to the specified customer master key (CMK).</p> <p>For more information about key policies, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html\">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn put_key_policy(&self, input: &PutKeyPolicyRequest) -> Result<(), PutKeyPolicyError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.PutKeyPolicy");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(PutKeyPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Encrypts data on the server side with a new customer master key (CMK) without exposing the plaintext of the data on the client side. The data is first decrypted and then reencrypted. You can also use this operation to change the encryption context of a ciphertext.</p> <p>Unlike other operations, <code>ReEncrypt</code> is authorized twice, once as <code>ReEncryptFrom</code> on the source CMK and once as <code>ReEncryptTo</code> on the destination CMK. We recommend that you include the <code>\"kms:ReEncrypt*\"</code> permission in your <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html\">key policies</a> to permit reencryption from or to the CMK. This permission is automatically included in the key policy when you create a CMK through the console, but you must include it manually when you create a CMK programmatically or when you set a key policy with the <a>PutKeyPolicy</a> operation.</p>"]
    fn re_encrypt(&self, input: &ReEncryptRequest) -> Result<ReEncryptResponse, ReEncryptError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ReEncrypt");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ReEncryptResponse>(String::from_utf8_lossy(&response.body)
                                                             .as_ref())
                       .unwrap())
        } else {
            Err(ReEncryptError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Retires a grant. To clean up, you can retire a grant when you're done using it. You should revoke a grant when you intend to actively deny operations that depend on it. The following are permitted to call this API:</p> <ul> <li> <p>The AWS account (root user) under which the grant was created</p> </li> <li> <p>The <code>RetiringPrincipal</code>, if present in the grant</p> </li> <li> <p>The <code>GranteePrincipal</code>, if <code>RetireGrant</code> is an operation specified in the grant</p> </li> </ul> <p>You must identify the grant to retire by its grant token or by a combination of the grant ID and the Amazon Resource Name (ARN) of the customer master key (CMK). A grant token is a unique variable-length base64-encoded string. A grant ID is a 64 character unique identifier of a grant. The <a>CreateGrant</a> operation returns both.</p>"]
    fn retire_grant(&self, input: &RetireGrantRequest) -> Result<(), RetireGrantError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.RetireGrant");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(RetireGrantError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Revokes a grant. You can revoke a grant to actively deny operations that depend on it.</p>"]
    fn revoke_grant(&self, input: &RevokeGrantRequest) -> Result<(), RevokeGrantError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.RevokeGrant");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(RevokeGrantError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Schedules the deletion of a customer master key (CMK). You may provide a waiting period, specified in days, before deletion occurs. If you do not provide a waiting period, the default period of 30 days is used. When this operation is successful, the state of the CMK changes to <code>PendingDeletion</code>. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK and all AWS KMS data associated with it, including all aliases that refer to it.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is rendered unrecoverable. To restrict the use of a CMK without deleting it, use <a>DisableKey</a>.</p> </important> <p>For more information about scheduling a CMK for deletion, see <a href=\"http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html\">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>"]
    fn schedule_key_deletion(&self,
                             input: &ScheduleKeyDeletionRequest)
                             -> Result<ScheduleKeyDeletionResponse, ScheduleKeyDeletionError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ScheduleKeyDeletion");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(serde_json::from_str::<ScheduleKeyDeletionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
        } else {
            Err(ScheduleKeyDeletionError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
        }
    }


    #[doc="<p>Adds or overwrites one or more tags for the specified customer master key (CMK). </p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>You cannot use the same tag key more than once per CMK. For example, consider a CMK with one tag whose tag key is <code>Purpose</code> and tag value is <code>Test</code>. If you send a <code>TagResource</code> request for this CMK with a tag key of <code>Purpose</code> and a tag value of <code>Prod</code>, it does not create a second tag. Instead, the original tag is overwritten with the new tag value.</p>"]
    fn tag_resource(&self, input: &TagResourceRequest) -> Result<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.TagResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(TagResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Removes the specified tag or tags from the specified customer master key (CMK). </p> <p>To remove a tag, you specify the tag key for each tag to remove. You do not specify the tag value. To overwrite the tag value for an existing tag, use <a>TagResource</a>.</p>"]
    fn untag_resource(&self, input: &UntagResourceRequest) -> Result<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UntagResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(UntagResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Updates an alias to map it to a different key.</p> <p>An alias is not a property of a key. Therefore, an alias can be mapped to and unmapped from an existing key without changing the properties of the key.</p> <p>An alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). An alias must start with the word \"alias\" followed by a forward slash (alias/). An alias that begins with \"aws\" after the forward slash (alias/aws...) is reserved by Amazon Web Services (AWS).</p> <p>The alias and the key it is mapped to must be in the same AWS account and the same region.</p>"]
    fn update_alias(&self, input: &UpdateAliasRequest) -> Result<(), UpdateAliasError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateAlias");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(UpdateAliasError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
        }
    }


    #[doc="<p>Updates the description of a customer master key (CMK).</p>"]
    fn update_key_description(&self,
                              input: &UpdateKeyDescriptionRequest)
                              -> Result<(), UpdateKeyDescriptionError> {
        let mut request = SignedRequest::new("POST", "kms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateKeyDescription");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        if response.check_status(200) {
            Ok(())
        } else {
            Err(UpdateKeyDescriptionError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
