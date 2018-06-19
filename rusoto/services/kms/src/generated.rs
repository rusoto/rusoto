// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Contains information about an alias.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AliasListEntry {
    /// <p>String that contains the key ARN.</p>
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// <p>String that contains the alias.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>String that contains the key identifier referred to by the alias.</p>
    #[serde(rename = "TargetKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelKeyDeletionRequest {
    /// <p>The unique identifier for the customer master key (CMK) for which to cancel deletion.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelKeyDeletionResponse {
    /// <p>The unique identifier of the master key for which deletion is canceled.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAliasRequest {
    /// <p>String that contains the display name. The name must start with the word "alias" followed by a forward slash (alias/). Aliases that begin with "alias/AWS" are reserved.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>Identifies the CMK for which you are creating the alias. This value cannot be an alias.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGrantRequest {
    /// <p>A structure that you can use to allow certain operations in the grant only when the desired encryption context is present. For more information about encryption context, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "Constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The principal that is given permission to perform the operations that the grant permits.</p> <p>To specify the principal, use the <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, IAM roles, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "GranteePrincipal")]
    pub grantee_principal: String,
    /// <p>The unique identifier for the customer master key (CMK) that the grant applies to.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>A friendly name for identifying the grant. Use this value to prevent unintended creation of duplicate grants when retrying this request.</p> <p>When this value is absent, all <code>CreateGrant</code> requests result in a new grant with a unique <code>GrantId</code> even if all the supplied parameters are identical. This can result in unintended duplicates when you retry the <code>CreateGrant</code> request.</p> <p>When this value is present, you can retry a <code>CreateGrant</code> request with identical parameters; if the grant already exists, the original <code>GrantId</code> is returned without creating a new grant. Note that the returned grant token is unique with every <code>CreateGrant</code> request, even when a duplicate <code>GrantId</code> is returned. All grant tokens obtained in this way can be used interchangeably.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of operations that the grant permits.</p>
    #[serde(rename = "Operations")]
    pub operations: Vec<String>,
    /// <p>The principal that is given permission to retire the grant by using <a>RetireGrant</a> operation.</p> <p>To specify the principal, use the <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "RetiringPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateGrantResponse {
    /// <p>The unique identifier for the grant.</p> <p>You can use the <code>GrantId</code> in a subsequent <a>RetireGrant</a> or <a>RevokeGrant</a> operation.</p>
    #[serde(rename = "GrantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>The grant token.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateKeyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you include a policy in the request and you intend to prevent the principal that is making the request from making a subsequent <a>PutKeyPolicy</a> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>A description of the CMK.</p> <p>Use a description that helps you decide whether the CMK is appropriate for a task.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The intended use of the CMK.</p> <p>You can use CMKs only for symmetric encryption and decryption.</p>
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>The source of the CMK's key material.</p> <p>The default is <code>AWS_KMS</code>, which means AWS KMS creates the key material. When this parameter is set to <code>EXTERNAL</code>, the request creates a CMK without key material so that you can import key material from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK's <code>Origin</code> is immutable and is set when the CMK is created.</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// <p>The key policy to attach to the CMK.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul> <li> <p>If you don't set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must allow the principal that is making the <code>CreateKey</code> request to make a subsequent <a>PutKeyPolicy</a> request on the CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> </ul> <p>If you do not provide a key policy, AWS KMS attaches a default key policy to the CMK. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default">Default Key Policy</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The key policy size limit is 32 kilobytes (32768 bytes).</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>One or more tags. Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>Use this parameter to tag the CMK when it is created. Alternately, you can omit this parameter and instead tag the CMK after it is created using <a>TagResource</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateKeyResponse {
    /// <p>Metadata associated with the CMK.</p>
    #[serde(rename = "KeyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DecryptRequest {
    /// <p>Ciphertext to be decrypted. The blob includes metadata.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: Vec<u8>,
    /// <p>The encryption context. If this was specified in the <a>Encrypt</a> function, it must be specified here or the decryption operation will fail. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html">Encryption Context</a>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DecryptResponse {
    /// <p>ARN of the key used to perform the decryption. This value is returned if no errors are encountered during the operation.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Decrypted plaintext data. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub plaintext: Option<Vec<u8>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAliasRequest {
    /// <p>The alias to be deleted. The name must start with the word "alias" followed by a forward slash (alias/). Aliases that begin with "alias/aws" are reserved.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteImportedKeyMaterialRequest {
    /// <p>The identifier of the CMK whose key material to delete. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeKeyRequest {
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with "alias/". To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeKeyResponse {
    /// <p>Metadata associated with the key.</p>
    #[serde(rename = "KeyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableKeyRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableKeyRotationRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableKeyRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableKeyRotationRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EncryptRequest {
    /// <p>Name-value pair that specifies the encryption context to be used for authenticated encryption. If used here, the same value must be supplied to the <code>Decrypt</code> API or decryption will fail. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html">Encryption Context</a>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with "alias/". To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Data to be encrypted.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub plaintext: Vec<u8>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EncryptResponse {
    /// <p>The encrypted plaintext. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: Option<Vec<u8>>,
    /// <p>The ID of the key used during encryption.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateDataKeyRequest {
    /// <p>A set of key-value pairs that represents additional authenticated data.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The identifier of the CMK under which to generate and encrypt the data encryption key.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with "alias/". To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The length of the data encryption key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// <p>The length of the data encryption key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GenerateDataKeyResponse {
    /// <p>The encrypted data encryption key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: Option<Vec<u8>>,
    /// <p>The identifier of the CMK under which the data encryption key was generated and encrypted.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The data encryption key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded. Use this data key for local encryption and decryption, then remove it from memory as soon as possible.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub plaintext: Option<Vec<u8>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateDataKeyWithoutPlaintextRequest {
    /// <p>A set of key-value pairs that represents additional authenticated data.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The identifier of the customer master key (CMK) under which to generate and encrypt the data encryption key.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with "alias/". To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The length of the data encryption key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// <p>The length of the data encryption key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    /// <p>The encrypted data encryption key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: Option<Vec<u8>>,
    /// <p>The identifier of the CMK under which the data encryption key was generated and encrypted.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateRandomRequest {
    /// <p>The length of the byte string.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GenerateRandomResponse {
    /// <p>The random byte string. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub plaintext: Option<Vec<u8>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetKeyPolicyRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Specifies the name of the key policy. The only valid name is <code>default</code>. To get the names of key policies, use <a>ListKeyPolicies</a>.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetKeyPolicyResponse {
    /// <p>A key policy document in JSON format.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetKeyRotationStatusRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetKeyRotationStatusResponse {
    /// <p>A Boolean value that specifies whether key rotation is enabled.</p>
    #[serde(rename = "KeyRotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParametersForImportRequest {
    /// <p>The identifier of the CMK into which you will import key material. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The algorithm you will use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-encrypt-key-material.html">Encrypt the Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "WrappingAlgorithm")]
    pub wrapping_algorithm: String,
    /// <p>The type of wrapping key (public key) to return in the response. Only 2048-bit RSA public keys are supported.</p>
    #[serde(rename = "WrappingKeySpec")]
    pub wrapping_key_spec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetParametersForImportResponse {
    /// <p>The import token to send in a subsequent <a>ImportKeyMaterial</a> request.</p>
    #[serde(rename = "ImportToken")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub import_token: Option<Vec<u8>>,
    /// <p>The identifier of the CMK to use in a subsequent <a>ImportKeyMaterial</a> request. This is the same CMK specified in the <code>GetParametersForImport</code> request.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The time at which the import token and public key are no longer valid. After this time, you cannot use them to make an <a>ImportKeyMaterial</a> request and you must send another <code>GetParametersForImport</code> request to get new ones.</p>
    #[serde(rename = "ParametersValidTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_valid_to: Option<f64>,
    /// <p>The public key to use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>.</p>
    #[serde(rename = "PublicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub public_key: Option<Vec<u8>>,
}

/// <p>A structure that you can use to allow certain operations in the grant only when the desired encryption context is present. For more information about encryption context, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Grant constraints apply only to operations that accept encryption context as input. For example, the <code> <a>DescribeKey</a> </code> operation does not accept encryption context as input. A grant that allows the <code>DescribeKey</code> operation does so regardless of the grant constraints. In constrast, the <code> <a>Encrypt</a> </code> operation accepts encryption context as input. A grant that allows the <code>Encrypt</code> operation does so only when the encryption context of the <code>Encrypt</code> operation satisfies the grant constraints.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrantConstraints {
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows. When certain subsequent operations allowed by the grant include encryption context that matches this list, the grant allows the operation. Otherwise, the grant does not allow the operation.</p>
    #[serde(rename = "EncryptionContextEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_equals: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of key-value pairs, all of which must be present in the encryption context of certain subsequent operations that the grant allows. When certain subsequent operations allowed by the grant include encryption context that matches this list or is a superset of this list, the grant allows the operation. Otherwise, the grant does not allow the operation.</p>
    #[serde(rename = "EncryptionContextSubset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_subset: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Contains information about an entry in a list of grants.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GrantListEntry {
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>
    #[serde(rename = "Constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    /// <p>The date and time when the grant was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The unique identifier for the grant.</p>
    #[serde(rename = "GrantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>The principal that receives the grant's permissions.</p>
    #[serde(rename = "GranteePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_principal: Option<String>,
    /// <p>The AWS account under which the grant was issued.</p>
    #[serde(rename = "IssuingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_account: Option<String>,
    /// <p>The unique identifier for the customer master key (CMK) to which the grant applies.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The friendly name that identifies the grant. If a name was provided in the <a>CreateGrant</a> request, that name is returned. Otherwise this value is null.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of operations permitted by the grant.</p>
    #[serde(rename = "Operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    /// <p>The principal that can retire the grant.</p>
    #[serde(rename = "RetiringPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportKeyMaterialRequest {
    /// <p>The encrypted key material to import. It must be encrypted with the public key that you received in the response to a previous <a>GetParametersForImport</a> request, using the wrapping algorithm that you specified in that request.</p>
    #[serde(rename = "EncryptedKeyMaterial")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub encrypted_key_material: Vec<u8>,
    /// <p>Specifies whether the key material expires. The default is <code>KEY_MATERIAL_EXPIRES</code>, in which case you must include the <code>ValidTo</code> parameter. When this parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>, you must omit the <code>ValidTo</code> parameter.</p>
    #[serde(rename = "ExpirationModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    /// <p>The import token that you received in the response to a previous <a>GetParametersForImport</a> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p>
    #[serde(rename = "ImportToken")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub import_token: Vec<u8>,
    /// <p>The identifier of the CMK to import the key material into. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. You must omit this parameter when the <code>ExpirationModel</code> parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>. Otherwise it is required.</p>
    #[serde(rename = "ValidTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ImportKeyMaterialResponse {}

/// <p>Contains information about each entry in the key list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KeyListEntry {
    /// <p>ARN of the key.</p>
    #[serde(rename = "KeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
    /// <p>Unique identifier of the key.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

/// <p>Contains metadata about a customer master key (CMK).</p> <p>This data type is used as a response element for the <a>CreateKey</a> and <a>DescribeKey</a> operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KeyMetadata {
    /// <p>The twelve-digit account ID of the AWS account that owns the CMK.</p>
    #[serde(rename = "AWSAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the CMK. For examples, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">AWS Key Management Service (AWS KMS)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time when the CMK was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date and time after which AWS KMS deletes the CMK. This value is present only when <code>KeyState</code> is <code>PendingDeletion</code>, otherwise this value is omitted.</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The description of the CMK.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specifies whether the CMK is enabled. When <code>KeyState</code> is <code>Enabled</code> this value is true, otherwise it is false.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies whether the CMK's key material expires. This value is present only when <code>Origin</code> is <code>EXTERNAL</code>, otherwise this value is omitted.</p>
    #[serde(rename = "ExpirationModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    /// <p>The globally unique identifier for the CMK.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The CMK's manager. CMKs are either customer-managed or AWS-managed. For more information about the difference, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KeyManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,
    /// <p>The state of the CMK.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KeyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// <p>The cryptographic operations for which you can use the CMK. Currently the only allowed value is <code>ENCRYPT_DECRYPT</code>, which means you can use the CMK for the <a>Encrypt</a> and <a>Decrypt</a> operations.</p>
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>The source of the CMK's key material. When this value is <code>AWS_KMS</code>, AWS KMS created the key material. When this value is <code>EXTERNAL</code>, the key material was imported from your existing key management infrastructure or the CMK lacks key material.</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// <p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. This value is present only for CMKs whose <code>Origin</code> is <code>EXTERNAL</code> and whose <code>ExpirationModel</code> is <code>KEY_MATERIAL_EXPIRES</code>, otherwise this value is omitted.</p>
    #[serde(rename = "ValidTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAliasesRequest {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAliasesResponse {
    /// <p>A list of aliases.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AliasListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGrantsRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListGrantsResponse {
    /// <p>A list of grants.</p>
    #[serde(rename = "Grants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<GrantListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListKeyPoliciesRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p> <p>Currently only 1 policy can be attached to a key.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListKeyPoliciesResponse {
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of key policy names. Currently, there is only one key policy per CMK and it is always named <code>default</code>.</p>
    #[serde(rename = "PolicyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListKeysRequest {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListKeysResponse {
    /// <p>A list of customer master keys (CMKs).</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<KeyListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceTagsRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 50, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p> <p>Do not attempt to construct this value. Use only the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListResourceTagsResponse {
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p> <p>Do not assume or infer any information from this value.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of tags. Each tag consists of a tag key and a tag value.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRetirableGrantsRequest {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The retiring principal for which to list grants.</p> <p>To specify the retiring principal, use the <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax for specifying a principal, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "RetiringPrincipal")]
    pub retiring_principal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutKeyPolicyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The key policy to attach to the CMK.</p> <p>The key policy must meet the following criteria:</p> <ul> <li> <p>If you don't set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must allow the principal that is making the <code>PutKeyPolicy</code> request to make a subsequent <code>PutKeyPolicy</code> request on the CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> </ul> <p>The key policy size limit is 32 kilobytes (32768 bytes).</p>
    #[serde(rename = "Policy")]
    pub policy: String,
    /// <p>The name of the key policy. The only valid value is <code>default</code>.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReEncryptRequest {
    /// <p>Ciphertext of the data to reencrypt.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: Vec<u8>,
    /// <p>Encryption context to use when the data is reencrypted.</p>
    #[serde(rename = "DestinationEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique identifier for the CMK that is used to reencrypt the data.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with "alias/". To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "DestinationKeyId")]
    pub destination_key_id: String,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Encryption context used to encrypt and decrypt the data specified in the <code>CiphertextBlob</code> parameter.</p>
    #[serde(rename = "SourceEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_context: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReEncryptResponse {
    /// <p>The reencrypted data. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: Option<Vec<u8>>,
    /// <p>Unique identifier of the CMK used to reencrypt the data.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Unique identifier of the CMK used to originally encrypt the data.</p>
    #[serde(rename = "SourceKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RetireGrantRequest {
    /// <p><p>Unique identifier of the grant to retire. The grant ID is returned in the response to a <code>CreateGrant</code> operation.</p> <ul> <li> <p>Grant ID Example - 0123456789012345678901234567890123456789012345678901234567890123</p> </li> </ul></p>
    #[serde(rename = "GrantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>Token that identifies the grant to be retired.</p>
    #[serde(rename = "GrantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the CMK associated with the grant. </p> <p>For example: <code>arn:aws:kms:us-east-2:444455556666:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RevokeGrantRequest {
    /// <p>Identifier of the grant to be revoked.</p>
    #[serde(rename = "GrantId")]
    pub grant_id: String,
    /// <p>A unique identifier for the customer master key associated with the grant.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleKeyDeletionRequest {
    /// <p>The unique identifier of the customer master key (CMK) to delete.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the customer master key (CMK).</p> <p>This value is optional. If you include a value, it must be between 7 and 30, inclusive. If you do not include a value, it defaults to 30.</p>
    #[serde(rename = "PendingWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_window_in_days: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScheduleKeyDeletionResponse {
    /// <p>The date and time after which AWS KMS deletes the customer master key (CMK).</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The unique identifier of the customer master key (CMK) for which deletion is scheduled.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

/// <p>A key-value pair. A tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "TagValue")]
    pub tag_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>A unique identifier for the CMK you are tagging.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>One or more tags. Each tag consists of a tag key and a tag value.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>A unique identifier for the CMK from which you are removing tags.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>One or more tag keys. Specify only the tag keys, not the tag values.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAliasRequest {
    /// <p>String that contains the name of the alias to be modified. The name must start with the word "alias" followed by a forward slash (alias/). Aliases that begin with "alias/aws" are reserved.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>Unique identifier of the customer master key to be mapped to the alias.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> <p>To verify that the alias is mapped to the correct CMK, use <a>ListAliases</a>.</p>
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateKeyDescriptionRequest {
    /// <p>New description for the CMK.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

/// Errors returned by CancelKeyDeletion
#[derive(Debug, PartialEq)]
pub enum CancelKeyDeletionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for CancelKeyDeletionError {
    fn from(err: io::Error) -> CancelKeyDeletionError {
        CancelKeyDeletionError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The request was rejected because it attempted to create a resource that already exists.</p>
    AlreadyExists(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified alias name is not valid.</p>
    InvalidAliasName(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for CreateAliasError {
    fn from(err: io::Error) -> CreateAliasError {
        CreateAliasError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for CreateGrantError {
    fn from(err: io::Error) -> CreateGrantError {
        CreateGrantError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for CreateKeyError {
    fn from(err: io::Error) -> CreateKeyError {
        CreateKeyError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DecryptError {
    fn from(err: io::Error) -> DecryptError {
        DecryptError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DeleteAliasError {
    fn from(err: io::Error) -> DeleteAliasError {
        DeleteAliasError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        DeleteImportedKeyMaterialError::DependencyTimeout(String::from(
                            error_message,
                        ))
                    }
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
                    "UnsupportedOperationException" => {
                        DeleteImportedKeyMaterialError::UnsupportedOperation(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for DeleteImportedKeyMaterialError {
    fn from(err: io::Error) -> DeleteImportedKeyMaterialError {
        DeleteImportedKeyMaterialError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DescribeKeyError {
    fn from(err: io::Error) -> DescribeKeyError {
        DescribeKeyError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DisableKeyError {
    fn from(err: io::Error) -> DisableKeyError {
        DisableKeyError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for DisableKeyRotationError {
    fn from(err: io::Error) -> DisableKeyRotationError {
        DisableKeyRotationError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for EnableKeyError {
    fn from(err: io::Error) -> EnableKeyError {
        EnableKeyError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for EnableKeyRotationError {
    fn from(err: io::Error) -> EnableKeyRotationError {
        EnableKeyRotationError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because the specified <code>KeySpec</code> value is not valid.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for EncryptError {
    fn from(err: io::Error) -> EncryptError {
        EncryptError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because the specified <code>KeySpec</code> value is not valid.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for GenerateDataKeyError {
    fn from(err: io::Error) -> GenerateDataKeyError {
        GenerateDataKeyError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because the specified <code>KeySpec</code> value is not valid.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        GenerateDataKeyWithoutPlaintextError::DependencyTimeout(String::from(
                            error_message,
                        ))
                    }
                    "DisabledException" => {
                        GenerateDataKeyWithoutPlaintextError::Disabled(String::from(error_message))
                    }
                    "InvalidGrantTokenException" => {
                        GenerateDataKeyWithoutPlaintextError::InvalidGrantToken(String::from(
                            error_message,
                        ))
                    }
                    "InvalidKeyUsageException" => {
                        GenerateDataKeyWithoutPlaintextError::InvalidKeyUsage(String::from(
                            error_message,
                        ))
                    }
                    "KMSInternalException" => GenerateDataKeyWithoutPlaintextError::KMSInternal(
                        String::from(error_message),
                    ),
                    "KMSInvalidStateException" => {
                        GenerateDataKeyWithoutPlaintextError::KMSInvalidState(String::from(
                            error_message,
                        ))
                    }
                    "KeyUnavailableException" => {
                        GenerateDataKeyWithoutPlaintextError::KeyUnavailable(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for GenerateDataKeyWithoutPlaintextError {
    fn from(err: io::Error) -> GenerateDataKeyWithoutPlaintextError {
        GenerateDataKeyWithoutPlaintextError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for GenerateRandomError {
    fn from(err: io::Error) -> GenerateRandomError {
        GenerateRandomError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for GetKeyPolicyError {
    fn from(err: io::Error) -> GetKeyPolicyError {
        GetKeyPolicyError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for GetKeyRotationStatusError {
    fn from(err: io::Error) -> GetKeyRotationStatusError {
        GetKeyRotationStatusError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                    "UnsupportedOperationException" => {
                        GetParametersForImportError::UnsupportedOperation(String::from(
                            error_message,
                        ))
                    }
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
impl From<io::Error> for GetParametersForImportError {
    fn from(err: io::Error) -> GetParametersForImportError {
        GetParametersForImportError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the provided import token is expired. Use <a>GetParametersForImport</a> to get a new import token and public key, use the new public key to encrypt the key material, and then try the request again.</p>
    ExpiredImportToken(String),
    /// <p>The request was rejected because the provided key material is invalid or is not the same key material that was previously imported into this customer master key (CMK).</p>
    IncorrectKeyMaterial(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the provided import token is invalid or is associated with a different customer master key (CMK).</p>
    InvalidImportToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ImportKeyMaterialError {
    fn from(err: io::Error) -> ImportKeyMaterialError {
        ImportKeyMaterialError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListAliasesError {
    fn from(err: io::Error) -> ListAliasesError {
        ListAliasesError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListGrantsError {
    fn from(err: io::Error) -> ListGrantsError {
        ListGrantsError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListKeyPoliciesError {
    fn from(err: io::Error) -> ListKeyPoliciesError {
        ListKeyPoliciesError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListKeysError {
    fn from(err: io::Error) -> ListKeysError {
        ListKeysError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListResourceTagsError {
    fn from(err: io::Error) -> ListResourceTagsError {
        ListResourceTagsError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ListRetirableGrantsError {
    fn from(err: io::Error) -> ListRetirableGrantsError {
        ListRetirableGrantsError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for PutKeyPolicyError {
    fn from(err: io::Error) -> PutKeyPolicyError {
        PutKeyPolicyError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because the specified <code>KeySpec</code> value is not valid.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ReEncryptError {
    fn from(err: io::Error) -> ReEncryptError {
        ReEncryptError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DependencyTimeoutException" => {
                        RetireGrantError::DependencyTimeout(String::from(error_message))
                    }
                    "InvalidArnException" => {
                        RetireGrantError::InvalidArn(String::from(error_message))
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
impl From<io::Error> for RetireGrantError {
    fn from(err: io::Error) -> RetireGrantError {
        RetireGrantError::HttpDispatch(HttpDispatchError::from(err))
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
            RetireGrantError::InvalidArn(ref cause) => cause,
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for RevokeGrantError {
    fn from(err: io::Error) -> RevokeGrantError {
        RevokeGrantError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for ScheduleKeyDeletionError {
    fn from(err: io::Error) -> ScheduleKeyDeletionError {
        ScheduleKeyDeletionError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for UpdateAliasError {
    fn from(err: io::Error) -> UpdateAliasError {
        UpdateAliasError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN was not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
impl From<io::Error> for UpdateKeyDescriptionError {
    fn from(err: io::Error) -> UpdateKeyDescriptionError {
        UpdateKeyDescriptionError::HttpDispatch(HttpDispatchError::from(err))
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
    /// <p>Cancels the deletion of a customer master key (CMK). When this operation is successful, the CMK is set to the <code>Disabled</code> state. To enable a CMK, use <a>EnableKey</a>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn cancel_key_deletion(
        &self,
        input: CancelKeyDeletionRequest,
    ) -> RusotoFuture<CancelKeyDeletionResponse, CancelKeyDeletionError>;

    /// <p>Creates a display name for a customer master key (CMK). You can use an alias to identify a CMK in selected operations, such as <a>Encrypt</a> and <a>GenerateDataKey</a>. </p> <p>Each CMK can have multiple aliases, but each alias points to only one CMK. The alias name must be unique in the AWS account and region. To simplify code that runs in multiple regions, use the same alias name, but point it to a different CMK in each region. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation.</p> <p>An alias must start with the word <code>alias</code> followed by a forward slash (<code>alias/</code>). The alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). Alias names cannot begin with <code>aws</code>; that alias name prefix is reserved by Amazon Web Services (AWS).</p> <p>The alias and the CMK it is mapped to must be in the same AWS account and the same region. You cannot perform this operation on an alias in a different AWS account.</p> <p>To map an existing alias to a different CMK, call <a>UpdateAlias</a>.</p>
    fn create_alias(&self, input: CreateAliasRequest) -> RusotoFuture<(), CreateAliasError>;

    /// <p>Adds a grant to a customer master key (CMK). The grant specifies who can use the CMK and under what conditions. When setting permissions, grants are an alternative to key policies. </p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the KeyId parameter. For more information about grants, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> RusotoFuture<CreateGrantResponse, CreateGrantError>;

    /// <p>Creates a customer master key (CMK) in the caller's AWS account.</p> <p>You can use a CMK to encrypt small amounts of data (4 KiB or less) directly, but CMKs are more commonly used to encrypt data encryption keys (DEKs), which are used to encrypt raw data. For more information about DEKs and the difference between CMKs and DEKs, see the following:</p> <ul> <li> <p>The <a>GenerateDataKey</a> operation</p> </li> <li> <p> <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html">AWS Key Management Service Concepts</a> in the <i>AWS Key Management Service Developer Guide</i> </p> </li> </ul> <p>You cannot use this operation to create a CMK in a different AWS account.</p>
    fn create_key(
        &self,
        input: CreateKeyRequest,
    ) -> RusotoFuture<CreateKeyResponse, CreateKeyError>;

    /// <p>Decrypts ciphertext. Ciphertext is plaintext that has been previously encrypted by using any of the following operations:</p> <ul> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> </ul> <p>Note that if a caller has been granted access permissions to all keys (through, for example, IAM user policies that grant <code>Decrypt</code> permission on all resources), then ciphertext encrypted by using keys in other accounts where the key grants access to the caller can be decrypted. To remedy this, we recommend that you do not grant <code>Decrypt</code> access in an IAM user policy. Instead grant <code>Decrypt</code> access only in key policies. If you must grant <code>Decrypt</code> access in an IAM user policy, you should scope the resource to specific keys or to specific trusted accounts.</p>
    fn decrypt(&self, input: DecryptRequest) -> RusotoFuture<DecryptResponse, DecryptError>;

    /// <p>Deletes the specified alias. You cannot perform this operation on an alias in a different AWS account. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation. </p> <p>Each CMK can have multiple aliases. To change the alias of a CMK, use <a>DeleteAlias</a> to delete the current alias and <a>CreateAlias</a> to create a new alias. To associate an existing alias with a different customer master key (CMK), call <a>UpdateAlias</a>.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError>;

    /// <p>Deletes key material that you previously imported. This operation makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK's state. Otherwise, it changes the CMK's state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p>
    fn delete_imported_key_material(
        &self,
        input: DeleteImportedKeyMaterialRequest,
    ) -> RusotoFuture<(), DeleteImportedKeyMaterialError>;

    /// <p>Provides detailed information about the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn describe_key(
        &self,
        input: DescribeKeyRequest,
    ) -> RusotoFuture<DescribeKeyResponse, DescribeKeyError>;

    /// <p>Sets the state of a customer master key (CMK) to disabled, thereby preventing its use for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key(&self, input: DisableKeyRequest) -> RusotoFuture<(), DisableKeyError>;

    /// <p>Disables automatic rotation of the key material for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn disable_key_rotation(
        &self,
        input: DisableKeyRotationRequest,
    ) -> RusotoFuture<(), DisableKeyRotationError>;

    /// <p>Sets the state of a customer master key (CMK) to enabled, thereby permitting its use for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p>
    fn enable_key(&self, input: EnableKeyRequest) -> RusotoFuture<(), EnableKeyError>;

    /// <p>Enables automatic rotation of the key material for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn enable_key_rotation(
        &self,
        input: EnableKeyRotationRequest,
    ) -> RusotoFuture<(), EnableKeyRotationError>;

    /// <p>Encrypts plaintext into ciphertext by using a customer master key (CMK). The <code>Encrypt</code> operation has two primary use cases:</p> <ul> <li> <p>You can encrypt up to 4 kilobytes (4096 bytes) of arbitrary data such as an RSA key, a database password, or other sensitive information.</p> </li> <li> <p>To move encrypted data from one AWS region to another, you can use this operation to encrypt in the new region the plaintext data key that was used to encrypt the data in the original region. This provides you with an encrypted copy of the data key that can be decrypted in the new region and used there to decrypt the encrypted data.</p> </li> </ul> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p> <p>Unless you are moving encrypted data from one region to another, you don't use this operation to encrypt a generated data key within a region. To get data keys that are already encrypted, call the <a>GenerateDataKey</a> or <a>GenerateDataKeyWithoutPlaintext</a> operation. Data keys don't need to be encrypted again by calling <code>Encrypt</code>.</p> <p>To encrypt data locally in your application, use the <a>GenerateDataKey</a> operation to return a plaintext data encryption key and a copy of the key encrypted under the CMK of your choosing.</p>
    fn encrypt(&self, input: EncryptRequest) -> RusotoFuture<EncryptResponse, EncryptError>;

    /// <p>Returns a data encryption key that you can use in your application to encrypt data locally. </p> <p>You must specify the customer master key (CMK) under which to generate the data key. You must also specify the length of the data key using either the <code>KeySpec</code> or <code>NumberOfBytes</code> field. You must specify one field or the other, but not both. For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use <code>KeySpec</code>. To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p> <p>This operation returns a plaintext copy of the data key in the <code>Plaintext</code> field of the response, and an encrypted copy of the data key in the <code>CiphertextBlob</code> field. The data key is encrypted under the CMK specified in the <code>KeyId</code> field of the request. </p> <p>We recommend that you use the following pattern to encrypt data locally in your application:</p> <ol> <li> <p>Use this operation (<code>GenerateDataKey</code>) to get a data encryption key.</p> </li> <li> <p>Use the plaintext data encryption key (returned in the <code>Plaintext</code> field of the response) to encrypt data locally, then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (returned in the <code>CiphertextBlob</code> field of the response) alongside the locally encrypted data.</p> </li> </ol> <p>To decrypt data locally:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key into a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data locally, then erase the plaintext data key from memory.</p> </li> </ol> <p>To return only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To return a random byte string that is cryptographically secure, use <a>GenerateRandom</a>.</p> <p>If you use the optional <code>EncryptionContext</code> field, you must store at least enough information to be able to reconstruct the full encryption context when you later send the ciphertext to the <a>Decrypt</a> operation. It is a good practice to choose an encryption context that you can reconstruct on the fly to better secure the ciphertext. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key(
        &self,
        input: GenerateDataKeyRequest,
    ) -> RusotoFuture<GenerateDataKeyResponse, GenerateDataKeyError>;

    /// <p>Returns a data encryption key encrypted under a customer master key (CMK). This operation is identical to <a>GenerateDataKey</a> but returns only the encrypted copy of the data key. </p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p> <p>This operation is useful in a system that has multiple components with different degrees of trust. For example, consider a system that stores encrypted data in containers. Each container stores the encrypted data and an encrypted copy of the data key. One component of the system, called the <i>control plane</i>, creates new containers. When it creates a new container, it uses this operation (<code>GenerateDataKeyWithoutPlaintext</code>) to get an encrypted data key and then stores it in the container. Later, a different component of the system, called the <i>data plane</i>, puts encrypted data into the containers. To do this, it passes the encrypted data key to the <a>Decrypt</a> operation, then uses the returned plaintext data key to encrypt data, and finally stores the encrypted data in the container. In this system, the control plane never sees the plaintext data key.</p>
    fn generate_data_key_without_plaintext(
        &self,
        input: GenerateDataKeyWithoutPlaintextRequest,
    ) -> RusotoFuture<GenerateDataKeyWithoutPlaintextResponse, GenerateDataKeyWithoutPlaintextError>;

    /// <p>Returns a random byte string that is cryptographically secure.</p> <p>For more information about entropy and random number generation, see the <a href="https://d0.awsstatic.com/whitepapers/KMS-Cryptographic-Details.pdf">AWS Key Management Service Cryptographic Details</a> whitepaper.</p>
    fn generate_random(
        &self,
        input: GenerateRandomRequest,
    ) -> RusotoFuture<GenerateRandomResponse, GenerateRandomError>;

    /// <p>Gets a key policy attached to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn get_key_policy(
        &self,
        input: GetKeyPolicyRequest,
    ) -> RusotoFuture<GetKeyPolicyResponse, GetKeyPolicyError>;

    /// <p>Gets a Boolean value that indicates whether automatic rotation of the key material is enabled for the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the KeyId parameter.</p>
    fn get_key_rotation_status(
        &self,
        input: GetKeyRotationStatusRequest,
    ) -> RusotoFuture<GetKeyRotationStatusResponse, GetKeyRotationStatusError>;

    /// <p>Returns the items you need in order to import key material into AWS KMS from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You must specify the key ID of the customer master key (CMK) into which you will import key material. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material. You cannot perform this operation on a CMK in a different AWS account.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request. The public key and import token from the same response must be used together. These items are valid for 24 hours. When they expire, they cannot be used for a subsequent <a>ImportKeyMaterial</a> request. To get new ones, send another <code>GetParametersForImport</code> request.</p>
    fn get_parameters_for_import(
        &self,
        input: GetParametersForImportRequest,
    ) -> RusotoFuture<GetParametersForImportResponse, GetParametersForImportError>;

    /// <p>Imports key material into an existing AWS KMS customer master key (CMK) that was created without key material. You cannot perform this operation on a CMK in a different AWS account. For more information about creating CMKs with no key material and then importing key material, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Before using this operation, call <a>GetParametersForImport</a>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p> <p>When calling this operation, you must specify the following values:</p> <ul> <li> <p>The key ID or key ARN of a CMK with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a CMK with no key material, call <a>CreateKey</a> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a CMK, call <a>DescribeKey</a>.)</p> </li> <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <a>GetParametersForImport</a>.</p> </li> <li> <p>The import token that <a>GetParametersForImport</a> returned. This token and the public key used to encrypt the key material must have come from the same response.</p> </li> <li> <p>Whether the key material expires and if so, when. If you set an expiration date, you can change it only by reimporting the same key material and specifying a new expiration date. If the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. To use the CMK again, you must reimport the same key material.</p> </li> </ul> <p>When this operation is successful, the CMK's key state changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the CMK. After you successfully import key material into a CMK, you can reimport the same key material into that CMK, but you cannot import different key material.</p>
    fn import_key_material(
        &self,
        input: ImportKeyMaterialRequest,
    ) -> RusotoFuture<ImportKeyMaterialResponse, ImportKeyMaterialError>;

    /// <p>Gets a list of all aliases in the caller's AWS account and region. You cannot list aliases in other accounts. For more information about aliases, see <a>CreateAlias</a>.</p> <p>The response might include several aliases that do not have a <code>TargetKeyId</code> field because they are not associated with a CMK. These are predefined aliases that are reserved for CMKs managed by AWS services. If an alias is not associated with a CMK, the alias does not count against the <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html#aliases-limit">alias limit</a> for your account.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError>;

    /// <p>Gets a list of all grants for the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the KeyId parameter.</p>
    fn list_grants(
        &self,
        input: ListGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListGrantsError>;

    /// <p>Gets the names of the key policies that are attached to a customer master key (CMK). This operation is designed to get policy names that you can use in a <a>GetKeyPolicy</a> operation. However, the only valid policy name is <code>default</code>. You cannot perform this operation on a CMK in a different AWS account.</p>
    fn list_key_policies(
        &self,
        input: ListKeyPoliciesRequest,
    ) -> RusotoFuture<ListKeyPoliciesResponse, ListKeyPoliciesError>;

    /// <p>Gets a list of all customer master keys (CMKs) in the caller's AWS account and region.</p>
    fn list_keys(&self, input: ListKeysRequest) -> RusotoFuture<ListKeysResponse, ListKeysError>;

    /// <p>Returns a list of all tags for the specified customer master key (CMK).</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p>
    fn list_resource_tags(
        &self,
        input: ListResourceTagsRequest,
    ) -> RusotoFuture<ListResourceTagsResponse, ListResourceTagsError>;

    /// <p>Returns a list of all grants for which the grant's <code>RetiringPrincipal</code> matches the one specified.</p> <p>A typical use is to list all grants that you are able to retire. To retire a grant, use <a>RetireGrant</a>.</p>
    fn list_retirable_grants(
        &self,
        input: ListRetirableGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListRetirableGrantsError>;

    /// <p>Attaches a key policy to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about key policies, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn put_key_policy(&self, input: PutKeyPolicyRequest) -> RusotoFuture<(), PutKeyPolicyError>;

    /// <p>Encrypts data on the server side with a new customer master key (CMK) without exposing the plaintext of the data on the client side. The data is first decrypted and then reencrypted. You can also use this operation to change the encryption context of a ciphertext. </p> <p>You can reencrypt data using CMKs in different AWS accounts.</p> <p>Unlike other operations, <code>ReEncrypt</code> is authorized twice, once as <code>ReEncryptFrom</code> on the source CMK and once as <code>ReEncryptTo</code> on the destination CMK. We recommend that you include the <code>"kms:ReEncrypt*"</code> permission in your <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policies</a> to permit reencryption from or to the CMK. This permission is automatically included in the key policy when you create a CMK through the console, but you must include it manually when you create a CMK programmatically or when you set a key policy with the <a>PutKeyPolicy</a> operation.</p>
    fn re_encrypt(
        &self,
        input: ReEncryptRequest,
    ) -> RusotoFuture<ReEncryptResponse, ReEncryptError>;

    /// <p>Retires a grant. To clean up, you can retire a grant when you're done using it. You should revoke a grant when you intend to actively deny operations that depend on it. The following are permitted to call this API:</p> <ul> <li> <p>The AWS account (root user) under which the grant was created</p> </li> <li> <p>The <code>RetiringPrincipal</code>, if present in the grant</p> </li> <li> <p>The <code>GranteePrincipal</code>, if <code>RetireGrant</code> is an operation specified in the grant</p> </li> </ul> <p>You must identify the grant to retire by its grant token or by a combination of the grant ID and the Amazon Resource Name (ARN) of the customer master key (CMK). A grant token is a unique variable-length base64-encoded string. A grant ID is a 64 character unique identifier of a grant. The <a>CreateGrant</a> operation returns both.</p>
    fn retire_grant(&self, input: RetireGrantRequest) -> RusotoFuture<(), RetireGrantError>;

    /// <p>Revokes the specified grant for the specified customer master key (CMK). You can revoke a grant to actively deny operations that depend on it.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the KeyId parameter.</p>
    fn revoke_grant(&self, input: RevokeGrantRequest) -> RusotoFuture<(), RevokeGrantError>;

    /// <p>Schedules the deletion of a customer master key (CMK). You may provide a waiting period, specified in days, before deletion occurs. If you do not provide a waiting period, the default period of 30 days is used. When this operation is successful, the state of the CMK changes to <code>PendingDeletion</code>. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK and all AWS KMS data associated with it, including all aliases that refer to it.</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is rendered unrecoverable. To restrict the use of a CMK without deleting it, use <a>DisableKey</a>.</p> </important> <p>For more information about scheduling a CMK for deletion, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn schedule_key_deletion(
        &self,
        input: ScheduleKeyDeletionRequest,
    ) -> RusotoFuture<ScheduleKeyDeletionResponse, ScheduleKeyDeletionError>;

    /// <p>Adds or overwrites one or more tags for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>You cannot use the same tag key more than once per CMK. For example, consider a CMK with one tag whose tag key is <code>Purpose</code> and tag value is <code>Test</code>. If you send a <code>TagResource</code> request for this CMK with a tag key of <code>Purpose</code> and a tag value of <code>Prod</code>, it does not create a second tag. Instead, the original tag is overwritten with the new tag value.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Removes the specified tag or tags from the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>To remove a tag, you specify the tag key for each tag to remove. You do not specify the tag value. To overwrite the tag value for an existing tag, use <a>TagResource</a>.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Associates an existing alias with a different customer master key (CMK). Each CMK can have multiple aliases, but the aliases must be unique within the account and region. You cannot perform this operation on an alias in a different AWS account.</p> <p>This operation works only on existing aliases. To change the alias of a CMK to a new value, use <a>CreateAlias</a> to create a new alias and <a>DeleteAlias</a> to delete the old alias.</p> <p>Because an alias is not a property of a CMK, you can create, update, and delete the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs in the account, use the <a>ListAliases</a> operation. </p> <p>An alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). An alias must start with the word <code>alias</code> followed by a forward slash (<code>alias/</code>). The alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). Alias names cannot begin with <code>aws</code>; that alias name prefix is reserved by Amazon Web Services (AWS).</p>
    fn update_alias(&self, input: UpdateAliasRequest) -> RusotoFuture<(), UpdateAliasError>;

    /// <p>Updates the description of a customer master key (CMK). To see the decription of a CMK, use <a>DescribeKey</a>. </p> <p>You cannot perform this operation on a CMK in a different AWS account.</p>
    fn update_key_description(
        &self,
        input: UpdateKeyDescriptionRequest,
    ) -> RusotoFuture<(), UpdateKeyDescriptionError>;
}
/// A client for the KMS API.
pub struct KmsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl KmsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> KmsClient {
        KmsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> KmsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        KmsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Kms for KmsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Cancels the deletion of a customer master key (CMK). When this operation is successful, the CMK is set to the <code>Disabled</code> state. To enable a CMK, use <a>EnableKey</a>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn cancel_key_deletion(
        &self,
        input: CancelKeyDeletionRequest,
    ) -> RusotoFuture<CancelKeyDeletionResponse, CancelKeyDeletionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CancelKeyDeletion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CancelKeyDeletionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelKeyDeletionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a display name for a customer master key (CMK). You can use an alias to identify a CMK in selected operations, such as <a>Encrypt</a> and <a>GenerateDataKey</a>. </p> <p>Each CMK can have multiple aliases, but each alias points to only one CMK. The alias name must be unique in the AWS account and region. To simplify code that runs in multiple regions, use the same alias name, but point it to a different CMK in each region. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation.</p> <p>An alias must start with the word <code>alias</code> followed by a forward slash (<code>alias/</code>). The alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). Alias names cannot begin with <code>aws</code>; that alias name prefix is reserved by Amazon Web Services (AWS).</p> <p>The alias and the CMK it is mapped to must be in the same AWS account and the same region. You cannot perform this operation on an alias in a different AWS account.</p> <p>To map an existing alias to a different CMK, call <a>UpdateAlias</a>.</p>
    fn create_alias(&self, input: CreateAliasRequest) -> RusotoFuture<(), CreateAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAliasError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds a grant to a customer master key (CMK). The grant specifies who can use the CMK and under what conditions. When setting permissions, grants are an alternative to key policies. </p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the KeyId parameter. For more information about grants, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> RusotoFuture<CreateGrantResponse, CreateGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateGrantResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateGrantError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a customer master key (CMK) in the caller's AWS account.</p> <p>You can use a CMK to encrypt small amounts of data (4 KiB or less) directly, but CMKs are more commonly used to encrypt data encryption keys (DEKs), which are used to encrypt raw data. For more information about DEKs and the difference between CMKs and DEKs, see the following:</p> <ul> <li> <p>The <a>GenerateDataKey</a> operation</p> </li> <li> <p> <a href="http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html">AWS Key Management Service Concepts</a> in the <i>AWS Key Management Service Developer Guide</i> </p> </li> </ul> <p>You cannot use this operation to create a CMK in a different AWS account.</p>
    fn create_key(
        &self,
        input: CreateKeyRequest,
    ) -> RusotoFuture<CreateKeyResponse, CreateKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateKeyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Decrypts ciphertext. Ciphertext is plaintext that has been previously encrypted by using any of the following operations:</p> <ul> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> </ul> <p>Note that if a caller has been granted access permissions to all keys (through, for example, IAM user policies that grant <code>Decrypt</code> permission on all resources), then ciphertext encrypted by using keys in other accounts where the key grants access to the caller can be decrypted. To remedy this, we recommend that you do not grant <code>Decrypt</code> access in an IAM user policy. Instead grant <code>Decrypt</code> access only in key policies. If you must grant <code>Decrypt</code> access in an IAM user policy, you should scope the resource to specific keys or to specific trusted accounts.</p>
    fn decrypt(&self, input: DecryptRequest) -> RusotoFuture<DecryptResponse, DecryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Decrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DecryptResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DecryptError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified alias. You cannot perform this operation on an alias in a different AWS account. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation. </p> <p>Each CMK can have multiple aliases. To change the alias of a CMK, use <a>DeleteAlias</a> to delete the current alias and <a>CreateAlias</a> to create a new alias. To associate an existing alias with a different customer master key (CMK), call <a>UpdateAlias</a>.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAliasError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes key material that you previously imported. This operation makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK's state. Otherwise, it changes the CMK's state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p>
    fn delete_imported_key_material(
        &self,
        input: DeleteImportedKeyMaterialRequest,
    ) -> RusotoFuture<(), DeleteImportedKeyMaterialError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteImportedKeyMaterial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteImportedKeyMaterialError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides detailed information about the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn describe_key(
        &self,
        input: DescribeKeyRequest,
    ) -> RusotoFuture<DescribeKeyResponse, DescribeKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DescribeKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeKeyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the state of a customer master key (CMK) to disabled, thereby preventing its use for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about how key state affects the use of a CMK, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key(&self, input: DisableKeyRequest) -> RusotoFuture<(), DisableKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisableKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disables automatic rotation of the key material for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn disable_key_rotation(
        &self,
        input: DisableKeyRotationRequest,
    ) -> RusotoFuture<(), DisableKeyRotationError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisableKeyRotation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DisableKeyRotationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Sets the state of a customer master key (CMK) to enabled, thereby permitting its use for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p>
    fn enable_key(&self, input: EnableKeyRequest) -> RusotoFuture<(), EnableKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.EnableKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Enables automatic rotation of the key material for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn enable_key_rotation(
        &self,
        input: EnableKeyRotationRequest,
    ) -> RusotoFuture<(), EnableKeyRotationError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.EnableKeyRotation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EnableKeyRotationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Encrypts plaintext into ciphertext by using a customer master key (CMK). The <code>Encrypt</code> operation has two primary use cases:</p> <ul> <li> <p>You can encrypt up to 4 kilobytes (4096 bytes) of arbitrary data such as an RSA key, a database password, or other sensitive information.</p> </li> <li> <p>To move encrypted data from one AWS region to another, you can use this operation to encrypt in the new region the plaintext data key that was used to encrypt the data in the original region. This provides you with an encrypted copy of the data key that can be decrypted in the new region and used there to decrypt the encrypted data.</p> </li> </ul> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p> <p>Unless you are moving encrypted data from one region to another, you don't use this operation to encrypt a generated data key within a region. To get data keys that are already encrypted, call the <a>GenerateDataKey</a> or <a>GenerateDataKeyWithoutPlaintext</a> operation. Data keys don't need to be encrypted again by calling <code>Encrypt</code>.</p> <p>To encrypt data locally in your application, use the <a>GenerateDataKey</a> operation to return a plaintext data encryption key and a copy of the key encrypted under the CMK of your choosing.</p>
    fn encrypt(&self, input: EncryptRequest) -> RusotoFuture<EncryptResponse, EncryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Encrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<EncryptResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(EncryptError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a data encryption key that you can use in your application to encrypt data locally. </p> <p>You must specify the customer master key (CMK) under which to generate the data key. You must also specify the length of the data key using either the <code>KeySpec</code> or <code>NumberOfBytes</code> field. You must specify one field or the other, but not both. For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use <code>KeySpec</code>. To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p> <p>This operation returns a plaintext copy of the data key in the <code>Plaintext</code> field of the response, and an encrypted copy of the data key in the <code>CiphertextBlob</code> field. The data key is encrypted under the CMK specified in the <code>KeyId</code> field of the request. </p> <p>We recommend that you use the following pattern to encrypt data locally in your application:</p> <ol> <li> <p>Use this operation (<code>GenerateDataKey</code>) to get a data encryption key.</p> </li> <li> <p>Use the plaintext data encryption key (returned in the <code>Plaintext</code> field of the response) to encrypt data locally, then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (returned in the <code>CiphertextBlob</code> field of the response) alongside the locally encrypted data.</p> </li> </ol> <p>To decrypt data locally:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key into a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data locally, then erase the plaintext data key from memory.</p> </li> </ol> <p>To return only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To return a random byte string that is cryptographically secure, use <a>GenerateRandom</a>.</p> <p>If you use the optional <code>EncryptionContext</code> field, you must store at least enough information to be able to reconstruct the full encryption context when you later send the ciphertext to the <a>Decrypt</a> operation. It is a good practice to choose an encryption context that you can reconstruct on the fly to better secure the ciphertext. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key(
        &self,
        input: GenerateDataKeyRequest,
    ) -> RusotoFuture<GenerateDataKeyResponse, GenerateDataKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateDataKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GenerateDataKeyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GenerateDataKeyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a data encryption key encrypted under a customer master key (CMK). This operation is identical to <a>GenerateDataKey</a> but returns only the encrypted copy of the data key. </p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p> <p>This operation is useful in a system that has multiple components with different degrees of trust. For example, consider a system that stores encrypted data in containers. Each container stores the encrypted data and an encrypted copy of the data key. One component of the system, called the <i>control plane</i>, creates new containers. When it creates a new container, it uses this operation (<code>GenerateDataKeyWithoutPlaintext</code>) to get an encrypted data key and then stores it in the container. Later, a different component of the system, called the <i>data plane</i>, puts encrypted data into the containers. To do this, it passes the encrypted data key to the <a>Decrypt</a> operation, then uses the returned plaintext data key to encrypt data, and finally stores the encrypted data in the container. In this system, the control plane never sees the plaintext data key.</p>
    fn generate_data_key_without_plaintext(
        &self,
        input: GenerateDataKeyWithoutPlaintextRequest,
    ) -> RusotoFuture<GenerateDataKeyWithoutPlaintextResponse, GenerateDataKeyWithoutPlaintextError>
    {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "TrentService.GenerateDataKeyWithoutPlaintext",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GenerateDataKeyWithoutPlaintextResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GenerateDataKeyWithoutPlaintextError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a random byte string that is cryptographically secure.</p> <p>For more information about entropy and random number generation, see the <a href="https://d0.awsstatic.com/whitepapers/KMS-Cryptographic-Details.pdf">AWS Key Management Service Cryptographic Details</a> whitepaper.</p>
    fn generate_random(
        &self,
        input: GenerateRandomRequest,
    ) -> RusotoFuture<GenerateRandomResponse, GenerateRandomError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateRandom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GenerateRandomResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GenerateRandomError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a key policy attached to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn get_key_policy(
        &self,
        input: GetKeyPolicyRequest,
    ) -> RusotoFuture<GetKeyPolicyResponse, GetKeyPolicyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetKeyPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetKeyPolicyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetKeyPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a Boolean value that indicates whether automatic rotation of the key material is enabled for the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the KeyId parameter.</p>
    fn get_key_rotation_status(
        &self,
        input: GetKeyRotationStatusRequest,
    ) -> RusotoFuture<GetKeyRotationStatusResponse, GetKeyRotationStatusError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetKeyRotationStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetKeyRotationStatusResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetKeyRotationStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the items you need in order to import key material into AWS KMS from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You must specify the key ID of the customer master key (CMK) into which you will import key material. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material. You cannot perform this operation on a CMK in a different AWS account.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request. The public key and import token from the same response must be used together. These items are valid for 24 hours. When they expire, they cannot be used for a subsequent <a>ImportKeyMaterial</a> request. To get new ones, send another <code>GetParametersForImport</code> request.</p>
    fn get_parameters_for_import(
        &self,
        input: GetParametersForImportRequest,
    ) -> RusotoFuture<GetParametersForImportResponse, GetParametersForImportError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetParametersForImport");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetParametersForImportResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetParametersForImportError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Imports key material into an existing AWS KMS customer master key (CMK) that was created without key material. You cannot perform this operation on a CMK in a different AWS account. For more information about creating CMKs with no key material and then importing key material, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Before using this operation, call <a>GetParametersForImport</a>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p> <p>When calling this operation, you must specify the following values:</p> <ul> <li> <p>The key ID or key ARN of a CMK with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a CMK with no key material, call <a>CreateKey</a> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a CMK, call <a>DescribeKey</a>.)</p> </li> <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <a>GetParametersForImport</a>.</p> </li> <li> <p>The import token that <a>GetParametersForImport</a> returned. This token and the public key used to encrypt the key material must have come from the same response.</p> </li> <li> <p>Whether the key material expires and if so, when. If you set an expiration date, you can change it only by reimporting the same key material and specifying a new expiration date. If the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. To use the CMK again, you must reimport the same key material.</p> </li> </ul> <p>When this operation is successful, the CMK's key state changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the CMK. After you successfully import key material into a CMK, you can reimport the same key material into that CMK, but you cannot import different key material.</p>
    fn import_key_material(
        &self,
        input: ImportKeyMaterialRequest,
    ) -> RusotoFuture<ImportKeyMaterialResponse, ImportKeyMaterialError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ImportKeyMaterial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ImportKeyMaterialResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ImportKeyMaterialError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of all aliases in the caller's AWS account and region. You cannot list aliases in other accounts. For more information about aliases, see <a>CreateAlias</a>.</p> <p>The response might include several aliases that do not have a <code>TargetKeyId</code> field because they are not associated with a CMK. These are predefined aliases that are reserved for CMKs managed by AWS services. If an alias is not associated with a CMK, the alias does not count against the <a href="http://docs.aws.amazon.com/kms/latest/developerguide/limits.html#aliases-limit">alias limit</a> for your account.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListAliases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAliasesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAliasesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of all grants for the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the KeyId parameter.</p>
    fn list_grants(
        &self,
        input: ListGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListGrantsError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListGrantsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListGrantsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the names of the key policies that are attached to a customer master key (CMK). This operation is designed to get policy names that you can use in a <a>GetKeyPolicy</a> operation. However, the only valid policy name is <code>default</code>. You cannot perform this operation on a CMK in a different AWS account.</p>
    fn list_key_policies(
        &self,
        input: ListKeyPoliciesRequest,
    ) -> RusotoFuture<ListKeyPoliciesResponse, ListKeyPoliciesError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListKeyPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListKeyPoliciesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListKeyPoliciesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of all customer master keys (CMKs) in the caller's AWS account and region.</p>
    fn list_keys(&self, input: ListKeysRequest) -> RusotoFuture<ListKeysResponse, ListKeysError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListKeys");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListKeysResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListKeysError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of all tags for the specified customer master key (CMK).</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p>
    fn list_resource_tags(
        &self,
        input: ListResourceTagsRequest,
    ) -> RusotoFuture<ListResourceTagsResponse, ListResourceTagsError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListResourceTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListResourceTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListResourceTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of all grants for which the grant's <code>RetiringPrincipal</code> matches the one specified.</p> <p>A typical use is to list all grants that you are able to retire. To retire a grant, use <a>RetireGrant</a>.</p>
    fn list_retirable_grants(
        &self,
        input: ListRetirableGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListRetirableGrantsError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListRetirableGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListGrantsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListRetirableGrantsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Attaches a key policy to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about key policies, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn put_key_policy(&self, input: PutKeyPolicyRequest) -> RusotoFuture<(), PutKeyPolicyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.PutKeyPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutKeyPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Encrypts data on the server side with a new customer master key (CMK) without exposing the plaintext of the data on the client side. The data is first decrypted and then reencrypted. You can also use this operation to change the encryption context of a ciphertext. </p> <p>You can reencrypt data using CMKs in different AWS accounts.</p> <p>Unlike other operations, <code>ReEncrypt</code> is authorized twice, once as <code>ReEncryptFrom</code> on the source CMK and once as <code>ReEncryptTo</code> on the destination CMK. We recommend that you include the <code>"kms:ReEncrypt*"</code> permission in your <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policies</a> to permit reencryption from or to the CMK. This permission is automatically included in the key policy when you create a CMK through the console, but you must include it manually when you create a CMK programmatically or when you set a key policy with the <a>PutKeyPolicy</a> operation.</p>
    fn re_encrypt(
        &self,
        input: ReEncryptRequest,
    ) -> RusotoFuture<ReEncryptResponse, ReEncryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ReEncrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ReEncryptResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ReEncryptError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retires a grant. To clean up, you can retire a grant when you're done using it. You should revoke a grant when you intend to actively deny operations that depend on it. The following are permitted to call this API:</p> <ul> <li> <p>The AWS account (root user) under which the grant was created</p> </li> <li> <p>The <code>RetiringPrincipal</code>, if present in the grant</p> </li> <li> <p>The <code>GranteePrincipal</code>, if <code>RetireGrant</code> is an operation specified in the grant</p> </li> </ul> <p>You must identify the grant to retire by its grant token or by a combination of the grant ID and the Amazon Resource Name (ARN) of the customer master key (CMK). A grant token is a unique variable-length base64-encoded string. A grant ID is a 64 character unique identifier of a grant. The <a>CreateGrant</a> operation returns both.</p>
    fn retire_grant(&self, input: RetireGrantRequest) -> RusotoFuture<(), RetireGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.RetireGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RetireGrantError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Revokes the specified grant for the specified customer master key (CMK). You can revoke a grant to actively deny operations that depend on it.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the KeyId parameter.</p>
    fn revoke_grant(&self, input: RevokeGrantRequest) -> RusotoFuture<(), RevokeGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.RevokeGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RevokeGrantError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Schedules the deletion of a customer master key (CMK). You may provide a waiting period, specified in days, before deletion occurs. If you do not provide a waiting period, the default period of 30 days is used. When this operation is successful, the state of the CMK changes to <code>PendingDeletion</code>. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK and all AWS KMS data associated with it, including all aliases that refer to it.</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is rendered unrecoverable. To restrict the use of a CMK without deleting it, use <a>DisableKey</a>.</p> </important> <p>For more information about scheduling a CMK for deletion, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn schedule_key_deletion(
        &self,
        input: ScheduleKeyDeletionRequest,
    ) -> RusotoFuture<ScheduleKeyDeletionResponse, ScheduleKeyDeletionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ScheduleKeyDeletion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ScheduleKeyDeletionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ScheduleKeyDeletionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds or overwrites one or more tags for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>You cannot use the same tag key more than once per CMK. For example, consider a CMK with one tag whose tag key is <code>Purpose</code> and tag value is <code>Test</code>. If you send a <code>TagResource</code> request for this CMK with a tag key of <code>Purpose</code> and a tag value of <code>Prod</code>, it does not create a second tag. Instead, the original tag is overwritten with the new tag value.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the specified tag or tags from the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>To remove a tag, you specify the tag key for each tag to remove. You do not specify the tag value. To overwrite the tag value for an existing tag, use <a>TagResource</a>.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Associates an existing alias with a different customer master key (CMK). Each CMK can have multiple aliases, but the aliases must be unique within the account and region. You cannot perform this operation on an alias in a different AWS account.</p> <p>This operation works only on existing aliases. To change the alias of a CMK to a new value, use <a>CreateAlias</a> to create a new alias and <a>DeleteAlias</a> to delete the old alias.</p> <p>Because an alias is not a property of a CMK, you can create, update, and delete the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs in the account, use the <a>ListAliases</a> operation. </p> <p>An alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). An alias must start with the word <code>alias</code> followed by a forward slash (<code>alias/</code>). The alias name can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). Alias names cannot begin with <code>aws</code>; that alias name prefix is reserved by Amazon Web Services (AWS).</p>
    fn update_alias(&self, input: UpdateAliasRequest) -> RusotoFuture<(), UpdateAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAliasError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the description of a customer master key (CMK). To see the decription of a CMK, use <a>DescribeKey</a>. </p> <p>You cannot perform this operation on a CMK in a different AWS account.</p>
    fn update_key_description(
        &self,
        input: UpdateKeyDescriptionRequest,
    ) -> RusotoFuture<(), UpdateKeyDescriptionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateKeyDescription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(future::ok(::std::mem::drop(response)))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateKeyDescriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
