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

use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use futures::{FutureExt, TryFutureExt};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Contains information about an alias.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AliasListEntry {
    /// <p>String that contains the key ARN.</p>
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// <p>String that contains the alias. This value begins with <code>alias/</code>.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct CancelKeyDeletionResponse {
    /// <p>The unique identifier of the master key for which deletion is canceled.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConnectCustomKeyStoreRequest {
    /// <p>Enter the key store ID of the custom key store that you want to connect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConnectCustomKeyStoreResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAliasRequest {
    /// <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for AWS managed CMKs.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>Identifies the CMK to which the alias refers. Specify the key ID or the Amazon Resource Name (ARN) of the CMK. You cannot specify another alias. For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html#find-cmk-id-arn">Finding the Key ID and ARN</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCustomKeyStoreRequest {
    /// <p>Identifies the AWS CloudHSM cluster for the custom key store. Enter the cluster ID of any active AWS CloudHSM cluster that is not already associated with a custom key store. To find the cluster ID, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    #[serde(rename = "CloudHsmClusterId")]
    pub cloud_hsm_cluster_id: String,
    /// <p>Specifies a friendly name for the custom key store. The name must be unique in your AWS account.</p>
    #[serde(rename = "CustomKeyStoreName")]
    pub custom_key_store_name: String,
    /// <p>Enter the password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU) account</a> in the specified AWS CloudHSM cluster. AWS KMS logs into the cluster as this user to manage key material on your behalf.</p> <p>This parameter tells AWS KMS the <code>kmsuser</code> account password; it does not change the password in the AWS CloudHSM cluster.</p>
    #[serde(rename = "KeyStorePassword")]
    pub key_store_password: String,
    /// <p>Enter the content of the trust anchor certificate for the cluster. This is the content of the <code>customerCA.crt</code> file that you created when you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html">initialized the cluster</a>.</p>
    #[serde(rename = "TrustAnchorCertificate")]
    pub trust_anchor_certificate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCustomKeyStoreResponse {
    /// <p>A unique identifier for the new custom key store.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGrantRequest {
    /// <p>Allows a cryptographic operation only when the encryption context matches or includes the encryption context specified in this structure. For more information about encryption context, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    #[serde(rename = "Constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The principal that is given permission to perform the operations that the grant permits.</p> <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, IAM roles, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "GranteePrincipal")]
    pub grantee_principal: String,
    /// <p>The unique identifier for the customer master key (CMK) that the grant applies to.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>A friendly name for identifying the grant. Use this value to prevent the unintended creation of duplicate grants when retrying this request.</p> <p>When this value is absent, all <code>CreateGrant</code> requests result in a new grant with a unique <code>GrantId</code> even if all the supplied parameters are identical. This can result in unintended duplicates when you retry the <code>CreateGrant</code> request.</p> <p>When this value is present, you can retry a <code>CreateGrant</code> request with identical parameters; if the grant already exists, the original <code>GrantId</code> is returned without creating a new grant. Note that the returned grant token is unique with every <code>CreateGrant</code> request, even when a duplicate <code>GrantId</code> is returned. All grant tokens obtained in this way can be used interchangeably.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of operations that the grant permits.</p>
    #[serde(rename = "Operations")]
    pub operations: Vec<String>,
    /// <p>The principal that is given permission to retire the grant by using <a>RetireGrant</a> operation.</p> <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "RetiringPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateGrantResponse {
    /// <p>The unique identifier for the grant.</p> <p>You can use the <code>GrantId</code> in a subsequent <a>RetireGrant</a> or <a>RevokeGrant</a> operation.</p>
    #[serde(rename = "GrantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>The grant token.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateKeyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </important> <p>Use this parameter only when you include a policy in the request and you intend to prevent the principal that is making the request from making a subsequent <a>PutKeyPolicy</a> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>Creates the CMK in the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> and the key material in its associated AWS CloudHSM cluster. To create a CMK in a custom key store, you must also specify the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs, each in a different Availability Zone in the Region.</p> <p>To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>The response includes the custom key store ID and the ID of the AWS CloudHSM cluster.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>A description of the CMK.</p> <p>Use a description that helps you decide whether the CMK is appropriate for a task.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The cryptographic operations for which you can use the CMK. The only valid value is <code>ENCRYPT_DECRYPT</code>, which means you can use the CMK to encrypt and decrypt data.</p>
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>The source of the key material for the CMK. You cannot change the origin after you create the CMK.</p> <p>The default is <code>AWS_KMS</code>, which means AWS KMS creates the key material in its own key store.</p> <p>When the parameter value is <code>EXTERNAL</code>, AWS KMS creates a CMK without key material so that you can import key material from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the parameter value is <code>AWS_CLOUDHSM</code>, AWS KMS creates the CMK in an AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> and creates its key material in the associated AWS CloudHSM cluster. You must also use the <code>CustomKeyStoreId</code> parameter to identify the custom key store.</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// <p>The key policy to attach to the CMK.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul> <li> <p>If you don't set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must allow the principal that is making the <code>CreateKey</code> request to make a subsequent <a>PutKeyPolicy</a> request on the CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> </ul> <p>If you do not provide a key policy, AWS KMS attaches a default key policy to the CMK. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default">Default Key Policy</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The key policy size limit is 32 kilobytes (32768 bytes).</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>One or more tags. Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>Use this parameter to tag the CMK when it is created. Alternately, you can omit this parameter and instead tag the CMK after it is created using <a>TagResource</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateKeyResponse {
    /// <p>Metadata associated with the CMK.</p>
    #[serde(rename = "KeyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

/// <p>Contains information about each custom key store in the custom key store list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CustomKeyStoresListEntry {
    /// <p>A unique identifier for the AWS CloudHSM cluster that is associated with the custom key store.</p>
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p>Describes the connection error. Valid values are:</p> <ul> <li> <p> <code>CLUSTER_NOT_FOUND</code> - AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID.</p> </li> <li> <p> <code>INSUFFICIENT_CLOUDHSM_HSMS</code> - The associated AWS CloudHSM cluster does not contain any active HSMs. To connect a custom key store to its AWS CloudHSM cluster, the cluster must contain at least one active HSM.</p> </li> <li> <p> <code>INTERNAL_ERROR</code> - AWS KMS could not complete the request due to an internal error. Retry the request. For <code>ConnectCustomKeyStore</code> requests, disconnect the custom key store before trying to connect again.</p> </li> <li> <p> <code>INVALID_CREDENTIALS</code> - AWS KMS does not have the correct password for the <code>kmsuser</code> crypto user in the AWS CloudHSM cluster.</p> </li> <li> <p> <code>NETWORK_ERRORS</code> - Network errors are preventing AWS KMS from connecting to the custom key store.</p> </li> <li> <p> <code>USER_LOCKED_OUT</code> - The <code>kmsuser</code> CU account is locked out of the associated AWS CloudHSM cluster due to too many failed password attempts. Before you can connect your custom key store to its AWS CloudHSM cluster, you must change the <code>kmsuser</code> account password and update the password value for the custom key store.</p> </li> </ul> <p>For help with connection failures, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting Custom Key Stores</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "ConnectionErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_error_code: Option<String>,
    /// <p>Indicates whether the custom key store is connected to its AWS CloudHSM cluster.</p> <p>You can create and use CMKs in your custom key stores only when its connection state is <code>CONNECTED</code>.</p> <p>The value is <code>DISCONNECTED</code> if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If the value is <code>CONNECTED</code> but you are having trouble using the custom key store, make sure that its associated AWS CloudHSM cluster is active and contains at least one active HSM.</p> <p>A value of <code>FAILED</code> indicates that an attempt to connect was unsuccessful. For help resolving a connection failure, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "ConnectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The date and time when the custom key store was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique identifier for the custom key store.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>The user-specified friendly name for the custom key store.</p>
    #[serde(rename = "CustomKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_name: Option<String>,
    /// <p>The trust anchor certificate of the associated AWS CloudHSM cluster. When you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html#sign-csr">initialize the cluster</a>, you create this certificate and save it in the <code>customerCA.crt</code> file.</p>
    #[serde(rename = "TrustAnchorCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor_certificate: Option<String>,
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
    pub ciphertext_blob: bytes::Bytes,
    /// <p>The encryption context. If this was specified in the <a>Encrypt</a> function, it must be specified here or the decryption operation will fail. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAliasRequest {
    /// <p>The alias to be deleted. The alias name must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCustomKeyStoreRequest {
    /// <p>Enter the ID of the custom key store you want to delete. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteCustomKeyStoreResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteImportedKeyMaterialRequest {
    /// <p>Identifies the CMK from which you are deleting imported key material. The <code>Origin</code> of the CMK must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCustomKeyStoresRequest {
    /// <p>Gets only information about the specified custom key store. Enter the key store ID.</p> <p>By default, this operation gets information about all custom key stores in the account and region. To limit the output to a particular custom key store, you can use either the <code>CustomKeyStoreId</code> or <code>CustomKeyStoreName</code> parameter, but not both.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>Gets only information about the specified custom key store. Enter the friendly name of the custom key store.</p> <p>By default, this operation gets information about all custom key stores in the account and region. To limit the output to a particular custom key store, you can use either the <code>CustomKeyStoreId</code> or <code>CustomKeyStoreName</code> parameter, but not both.</p>
    #[serde(rename = "CustomKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_name: Option<String>,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCustomKeyStoresResponse {
    /// <p>Contains metadata about each custom key store.</p>
    #[serde(rename = "CustomKeyStores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_stores: Option<Vec<CustomKeyStoresListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeKeyRequest {
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Describes the specified customer master key (CMK). </p> <p>If you specify a predefined AWS alias (an AWS alias with no key ID), KMS associates the alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a> and returns its <code>KeyId</code> and <code>Arn</code> in the response.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
pub struct DisconnectCustomKeyStoreRequest {
    /// <p>Enter the ID of the custom key store you want to disconnect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisconnectCustomKeyStoreResponse {}

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
    /// <p>Name-value pair that specifies the encryption context to be used for authenticated encryption. If used here, the same value must be supplied to the <code>Decrypt</code> API or decryption will fail. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Data to be encrypted.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub plaintext: bytes::Bytes,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EncryptResponse {
    /// <p>The encrypted plaintext. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The ID of the key used during encryption.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateDataKeyRequest {
    /// <p>A set of key-value pairs that represents additional authenticated data.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>An identifier for the CMK that encrypts the data key.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// <p>The length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GenerateDataKeyResponse {
    /// <p>The encrypted copy of the data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The identifier of the CMK that encrypted the data key.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The plaintext data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded. Use this data key to encrypt your data outside of KMS. Then, remove it from memory as soon as possible.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateDataKeyWithoutPlaintextRequest {
    /// <p>A set of key-value pairs that represents additional authenticated data.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The identifier of the customer master key (CMK) that encrypts the data key.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// <p>The length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    /// <p>The encrypted data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The identifier of the CMK that encrypted the data key.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateRandomRequest {
    /// <p>Generates the random byte string in the AWS CloudHSM cluster that is associated with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>The length of the byte string.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GenerateRandomResponse {
    /// <p>The random byte string. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The algorithm you will use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-encrypt-key-material.html">Encrypt the Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "WrappingAlgorithm")]
    pub wrapping_algorithm: String,
    /// <p>The type of wrapping key (public key) to return in the response. Only 2048-bit RSA public keys are supported.</p>
    #[serde(rename = "WrappingKeySpec")]
    pub wrapping_key_spec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetParametersForImportResponse {
    /// <p>The import token to send in a subsequent <a>ImportKeyMaterial</a> request.</p>
    #[serde(rename = "ImportToken")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_token: Option<bytes::Bytes>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
}

/// <p><p>Use this structure to allow cryptographic operations in the grant only when the operation request includes the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">encryption context</a>.</p> <p>AWS KMS applies the grant constraints only when the grant allows a cryptographic operation that accepts an encryption context as input, such as the following.</p> <ul> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>ReEncrypt</a> </p> </li> </ul> <p>AWS KMS does not apply the grant constraints to other operations, such as <a>DescribeKey</a> or <a>ScheduleKeyDeletion</a>.</p> <important> <p>In a cryptographic operation, the encryption context in the decryption operation must be an exact, case-sensitive match for the keys and values in the encryption context of the encryption operation. Only the order of the pairs can vary.</p> <p>However, in a grant constraint, the key in each key-value pair is not case sensitive, but the value is case sensitive.</p> <p>To avoid confusion, do not use multiple encryption context pairs that differ only by case. To require a fully case-sensitive encryption context, use the <code>kms:EncryptionContext:</code> and <code>kms:EncryptionContextKeys</code> conditions in an IAM or key policy. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/policy-conditions.html#conditions-kms-encryption-context">kms:EncryptionContext:</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </important></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrantConstraints {
    /// <p>A list of key-value pairs that must match the encryption context in the cryptographic operation request. The grant allows the operation only when the encryption context in the request is the same as the encryption context specified in this constraint.</p>
    #[serde(rename = "EncryptionContextEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_equals: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of key-value pairs that must be included in the encryption context of the cryptographic operation request. The grant allows the cryptographic operation only when the encryption context in the request includes the key-value pairs specified in this constraint, although it can include additional key-value pairs.</p>
    #[serde(rename = "EncryptionContextSubset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_subset: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Contains information about an entry in a list of grants.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    pub encrypted_key_material: bytes::Bytes,
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
    pub import_token: bytes::Bytes,
    /// <p>The identifier of the CMK to import the key material into. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. You must omit this parameter when the <code>ExpirationModel</code> parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>. Otherwise it is required.</p>
    #[serde(rename = "ValidTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportKeyMaterialResponse {}

/// <p>Contains information about each entry in the key list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct KeyMetadata {
    /// <p>The twelve-digit account ID of the AWS account that owns the CMK.</p>
    #[serde(rename = "AWSAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the CMK. For examples, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">AWS Key Management Service (AWS KMS)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The cluster ID of the AWS CloudHSM cluster that contains the key material for the CMK. When you create a CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, AWS KMS creates the key material for the CMK in the associated AWS CloudHSM cluster. This value is present only when the CMK is created in a custom key store.</p>
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p>The date and time when the CMK was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique identifier for the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that contains the CMK. This value is present only when the CMK is created in a custom key store.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>The date and time after which AWS KMS deletes the CMK. This value is present only when <code>KeyState</code> is <code>PendingDeletion</code>.</p>
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
    /// <p>The manager of the CMK. CMKs in your AWS account are either customer managed or AWS managed. For more information about the difference, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KeyManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,
    /// <p>The state of the CMK.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KeyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// <p>The cryptographic operations for which you can use the CMK. The only valid value is <code>ENCRYPT_DECRYPT</code>, which means you can use the CMK to encrypt and decrypt data.</p>
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>The source of the CMK's key material. When this value is <code>AWS_KMS</code>, AWS KMS created the key material. When this value is <code>EXTERNAL</code>, the key material was imported from your existing key management infrastructure or the CMK lacks key material. When this value is <code>AWS_CLOUDHSM</code>, the key material was created in the AWS CloudHSM cluster associated with a custom key store.</p>
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
    /// <p>Lists only aliases that refer to the specified CMK. The value of this parameter can be the ID or Amazon Resource Name (ARN) of a CMK in the caller's account and region. You cannot use an alias name or alias ARN in this value.</p> <p>This parameter is optional. If you omit it, <code>ListAliases</code> returns all aliases in the account and region.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListAliasesResponse {
    /// <p>A list of aliases.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AliasListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListGrantsResponse {
    /// <p>A list of grants.</p>
    #[serde(rename = "Grants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<GrantListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListKeyPoliciesRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p> <p>Only one policy can be attached to a key.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListKeyPoliciesResponse {
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of key policy names. The only valid value is <code>default</code>.</p>
    #[serde(rename = "PolicyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListKeysResponse {
    /// <p>A list of customer master keys (CMKs).</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<KeyListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListResourceTagsResponse {
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p> <p>Do not assume or infer any information from this value.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of tags. Each tag consists of a tag key and a tag value.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
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
    /// <p>The retiring principal for which to list grants.</p> <p>To specify the retiring principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "RetiringPrincipal")]
    pub retiring_principal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutKeyPolicyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The key policy to attach to the CMK.</p> <p>The key policy must meet the following criteria:</p> <ul> <li> <p>If you don't set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must allow the principal that is making the <code>PutKeyPolicy</code> request to make a subsequent <code>PutKeyPolicy</code> request on the CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> </ul> <p>The key policy size limit is 32 kilobytes (32768 bytes).</p>
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
    pub ciphertext_blob: bytes::Bytes,
    /// <p>Encryption context to use when the data is reencrypted.</p>
    #[serde(rename = "DestinationEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique identifier for the CMK that is used to reencrypt the data.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "DestinationKeyId")]
    pub destination_key_id: String,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Encryption context used to encrypt and decrypt the data specified in the <code>CiphertextBlob</code> parameter.</p>
    #[serde(rename = "SourceEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_context: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReEncryptResponse {
    /// <p>The reencrypted data. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>A key-value pair. A tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
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
    /// <p>Specifies the name of the alias to change. This value must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>Unique identifier of the customer master key (CMK) to be mapped to the alias. When the update operation completes, the alias will point to this CMK.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> <p>To verify that the alias is mapped to the correct CMK, use <a>ListAliases</a>.</p>
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCustomKeyStoreRequest {
    /// <p>Associates the custom key store with a related AWS CloudHSM cluster. </p> <p>Enter the cluster ID of the cluster that you used to create the custom key store or a cluster that shares a backup history and has the same cluster certificate as the original cluster. You cannot use this parameter to associate a custom key store with an unrelated cluster. In addition, the replacement cluster must <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">fulfill the requirements</a> for a cluster associated with a custom key store. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p>Identifies the custom key store that you want to update. Enter the ID of the custom key store. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    pub custom_key_store_id: String,
    /// <p>Enter the current password of the <code>kmsuser</code> crypto user (CU) in the AWS CloudHSM cluster that is associated with the custom key store.</p> <p>This parameter tells AWS KMS the current password of the <code>kmsuser</code> crypto user (CU). It does not set or change the password of any users in the AWS CloudHSM cluster.</p>
    #[serde(rename = "KeyStorePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_store_password: Option<String>,
    /// <p>Changes the friendly name of the custom key store to the value that you specify. The custom key store name must be unique in the AWS account.</p>
    #[serde(rename = "NewCustomKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_custom_key_store_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateCustomKeyStoreResponse {}

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
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CancelKeyDeletionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelKeyDeletionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CancelKeyDeletionError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CancelKeyDeletionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CancelKeyDeletionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CancelKeyDeletionError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CancelKeyDeletionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ConnectCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum ConnectCustomKeyStoreError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl ConnectCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConnectCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ConnectCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ConnectCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConnectCustomKeyStoreError {
    fn description(&self) -> &str {
        match *self {
            ConnectCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => cause,
            ConnectCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => cause,
            ConnectCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => cause,
            ConnectCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => cause,
            ConnectCustomKeyStoreError::KMSInternal(ref cause) => cause,
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
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateAliasError::AlreadyExists(err.msg))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateAliasError::DependencyTimeout(err.msg))
                }
                "InvalidAliasNameException" => {
                    return RusotoError::Service(CreateAliasError::InvalidAliasName(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CreateAliasError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAliasError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum CreateCustomKeyStoreError {
    /// <p>The request was rejected because the specified AWS CloudHSM cluster is already associated with a custom key store or it shares a backup history with a cluster that is associated with a custom key store. Each custom key store must be associated with a different AWS CloudHSM cluster.</p> <p>Clusters that share a backup history have the same cluster certificate. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    CloudHsmClusterInUse(String),
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p>The request was rejected because AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID. Retry the request with a different cluster ID.</p>
    CloudHsmClusterNotFound(String),
    /// <p>The request was rejected because the specified custom key store name is already assigned to another custom key store in the account. Try again with a custom key store name that is unique in the account.</p>
    CustomKeyStoreNameInUse(String),
    /// <p>The request was rejected because the trust anchor certificate in the request is not the trust anchor certificate for the specified AWS CloudHSM cluster.</p> <p>When you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html#sign-csr">initialize the cluster</a>, you create the trust anchor certificate and save it in the <code>customerCA.crt</code> file.</p>
    IncorrectTrustAnchor(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl CreateCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInUseException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::CloudHsmClusterInUse(
                        err.msg,
                    ))
                }
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CloudHsmClusterNotFoundException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterNotFound(err.msg),
                    )
                }
                "CustomKeyStoreNameInUseException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CustomKeyStoreNameInUse(err.msg),
                    )
                }
                "IncorrectTrustAnchorException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::IncorrectTrustAnchor(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCustomKeyStoreError {
    fn description(&self) -> &str {
        match *self {
            CreateCustomKeyStoreError::CloudHsmClusterInUse(ref cause) => cause,
            CreateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => cause,
            CreateCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => cause,
            CreateCustomKeyStoreError::CloudHsmClusterNotFound(ref cause) => cause,
            CreateCustomKeyStoreError::CustomKeyStoreNameInUse(ref cause) => cause,
            CreateCustomKeyStoreError::IncorrectTrustAnchor(ref cause) => cause,
            CreateCustomKeyStoreError::KMSInternal(ref cause) => cause,
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
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CreateGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateGrantError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(CreateGrantError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(CreateGrantError::InvalidGrantToken(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CreateGrantError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGrantError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by CreateKey
#[derive(Debug, PartialEq)]
pub enum CreateKeyError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl CreateKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        CreateKeyError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(CreateKeyError::CustomKeyStoreInvalidState(
                        err.msg,
                    ))
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(CreateKeyError::CustomKeyStoreNotFound(err.msg))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateKeyError::KMSInternal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateKeyError::LimitExceeded(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(CreateKeyError::MalformedPolicyDocument(err.msg))
                }
                "TagException" => return RusotoError::Service(CreateKeyError::Tag(err.msg)),
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateKeyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateKeyError::CloudHsmClusterInvalidConfiguration(ref cause) => cause,
            CreateKeyError::CustomKeyStoreInvalidState(ref cause) => cause,
            CreateKeyError::CustomKeyStoreNotFound(ref cause) => cause,
            CreateKeyError::DependencyTimeout(ref cause) => cause,
            CreateKeyError::InvalidArn(ref cause) => cause,
            CreateKeyError::KMSInternal(ref cause) => cause,
            CreateKeyError::LimitExceeded(ref cause) => cause,
            CreateKeyError::MalformedPolicyDocument(ref cause) => cause,
            CreateKeyError::Tag(ref cause) => cause,
            CreateKeyError::UnsupportedOperation(ref cause) => cause,
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
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DecryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DecryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DecryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(DecryptError::Disabled(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(DecryptError::InvalidCiphertext(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(DecryptError::InvalidGrantToken(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DecryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DecryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(DecryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DecryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DeleteAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DeleteAliasError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DeleteAliasError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum DeleteCustomKeyStoreError {
    /// <p>The request was rejected because the custom key store contains AWS KMS customer master keys (CMKs). After verifying that you do not need to use the CMKs, use the <a>ScheduleKeyDeletion</a> operation to delete the CMKs. After they are deleted, you can delete the custom key store.</p>
    CustomKeyStoreHasCMKs(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DeleteCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreHasCMKsException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::CustomKeyStoreHasCMKs(
                        err.msg,
                    ))
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        DeleteCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCustomKeyStoreError {
    fn description(&self) -> &str {
        match *self {
            DeleteCustomKeyStoreError::CustomKeyStoreHasCMKs(ref cause) => cause,
            DeleteCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => cause,
            DeleteCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => cause,
            DeleteCustomKeyStoreError::KMSInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteImportedKeyMaterial
#[derive(Debug, PartialEq)]
pub enum DeleteImportedKeyMaterialError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl DeleteImportedKeyMaterialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImportedKeyMaterialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::InvalidArn(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::KMSInternal(
                        err.msg,
                    ))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteImportedKeyMaterialError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DescribeCustomKeyStores
#[derive(Debug, PartialEq)]
pub enum DescribeCustomKeyStoresError {
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DescribeCustomKeyStoresError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCustomKeyStoresError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCustomKeyStoresError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DescribeCustomKeyStoresError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCustomKeyStoresError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCustomKeyStoresError {
    fn description(&self) -> &str {
        match *self {
            DescribeCustomKeyStoresError::CustomKeyStoreNotFound(ref cause) => cause,
            DescribeCustomKeyStoresError::KMSInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeKey
#[derive(Debug, PartialEq)]
pub enum DescribeKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DescribeKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DescribeKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DescribeKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DescribeKeyError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DisableKey
#[derive(Debug, PartialEq)]
pub enum DisableKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DisableKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DisableKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DisableKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisableKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DisableKeyError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisableKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl DisableKeyRotationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableKeyRotationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DisableKeyRotationError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "DisabledException" => {
                    return RusotoError::Service(DisableKeyRotationError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DisableKeyRotationError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisableKeyRotationError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DisableKeyRotationError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisableKeyRotationError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DisableKeyRotationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DisconnectCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum DisconnectCustomKeyStoreError {
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DisconnectCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisconnectCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        DisconnectCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        DisconnectCustomKeyStoreError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisconnectCustomKeyStoreError::KMSInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisconnectCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisconnectCustomKeyStoreError {
    fn description(&self) -> &str {
        match *self {
            DisconnectCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => cause,
            DisconnectCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => cause,
            DisconnectCustomKeyStoreError::KMSInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableKey
#[derive(Debug, PartialEq)]
pub enum EnableKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl EnableKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EnableKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(EnableKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EnableKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EnableKeyError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(EnableKeyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EnableKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl EnableKeyRotationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableKeyRotationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EnableKeyRotationError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(EnableKeyRotationError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(EnableKeyRotationError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EnableKeyRotationError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EnableKeyRotationError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EnableKeyRotationError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(EnableKeyRotationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl EncryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EncryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EncryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(EncryptError::Disabled(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(EncryptError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(EncryptError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EncryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EncryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(EncryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EncryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GenerateDataKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateDataKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GenerateDataKeyError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(GenerateDataKeyError::Disabled(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(GenerateDataKeyError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(GenerateDataKeyError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateDataKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GenerateDataKeyError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(GenerateDataKeyError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateDataKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GenerateDataKeyWithoutPlaintextError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GenerateDataKeyWithoutPlaintextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::DependencyTimeout(err.msg),
                    )
                }
                "DisabledException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::Disabled(
                        err.msg,
                    ))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::InvalidGrantToken(err.msg),
                    )
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::InvalidKeyUsage(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::KMSInternal(
                        err.msg,
                    ))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::KMSInvalidState(err.msg),
                    )
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::KeyUnavailable(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::NotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GenerateRandom
#[derive(Debug, PartialEq)]
pub enum GenerateRandomError {
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl GenerateRandomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateRandomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(GenerateRandomError::CustomKeyStoreInvalidState(
                        err.msg,
                    ))
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(GenerateRandomError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GenerateRandomError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateRandomError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            GenerateRandomError::CustomKeyStoreInvalidState(ref cause) => cause,
            GenerateRandomError::CustomKeyStoreNotFound(ref cause) => cause,
            GenerateRandomError::DependencyTimeout(ref cause) => cause,
            GenerateRandomError::KMSInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by GetKeyPolicy
#[derive(Debug, PartialEq)]
pub enum GetKeyPolicyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GetKeyPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetKeyPolicyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetKeyPolicyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetKeyPolicyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetKeyPolicyError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyPolicyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetKeyRotationStatus
#[derive(Debug, PartialEq)]
pub enum GetKeyRotationStatusError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GetKeyRotationStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyRotationStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetParametersForImport
#[derive(Debug, PartialEq)]
pub enum GetParametersForImportError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GetParametersForImportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetParametersForImportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetParametersForImportError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetParametersForImportError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetParametersForImportError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetParametersForImportError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetParametersForImportError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetParametersForImportError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the provided import token is invalid or is associated with a different customer master key (CMK).</p>
    InvalidImportToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl ImportKeyMaterialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportKeyMaterialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ImportKeyMaterialError::DependencyTimeout(err.msg))
                }
                "ExpiredImportTokenException" => {
                    return RusotoError::Service(ImportKeyMaterialError::ExpiredImportToken(
                        err.msg,
                    ))
                }
                "IncorrectKeyMaterialException" => {
                    return RusotoError::Service(ImportKeyMaterialError::IncorrectKeyMaterial(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidArn(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidCiphertext(err.msg))
                }
                "InvalidImportTokenException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidImportToken(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ImportKeyMaterialError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ImportKeyMaterialError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ImportKeyMaterialError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ImportKeyMaterialError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListAliasesError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListAliasesError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListAliasesError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListAliasesError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListAliasesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            ListAliasesError::InvalidArn(ref cause) => cause,
            ListAliasesError::InvalidMarker(ref cause) => cause,
            ListAliasesError::KMSInternal(ref cause) => cause,
            ListAliasesError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGrants
#[derive(Debug, PartialEq)]
pub enum ListGrantsError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListGrantsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGrantsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListGrantsError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListGrantsError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListGrantsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListGrantsError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ListGrantsError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListGrantsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListKeyPolicies
#[derive(Debug, PartialEq)]
pub enum ListKeyPoliciesError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListKeyPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListKeyPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListKeyPoliciesError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListKeyPoliciesError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListKeyPoliciesError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ListKeyPoliciesError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListKeyPoliciesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
}

impl ListKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListKeysError::DependencyTimeout(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListKeysError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListKeysError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListResourceTags
#[derive(Debug, PartialEq)]
pub enum ListResourceTagsError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListResourceTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ListResourceTagsError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListResourceTagsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListResourceTagsError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListResourceTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListRetirableGrants
#[derive(Debug, PartialEq)]
pub enum ListRetirableGrantsError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListRetirableGrantsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRetirableGrantsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListRetirableGrantsError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListRetirableGrantsError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListRetirableGrantsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListRetirableGrantsError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListRetirableGrantsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by PutKeyPolicy
#[derive(Debug, PartialEq)]
pub enum PutKeyPolicyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl PutKeyPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutKeyPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(PutKeyPolicyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(PutKeyPolicyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(PutKeyPolicyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(PutKeyPolicyError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutKeyPolicyError::LimitExceeded(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(PutKeyPolicyError::MalformedPolicyDocument(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutKeyPolicyError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(PutKeyPolicyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. The request can be retried.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ReEncryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReEncryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ReEncryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(ReEncryptError::Disabled(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(ReEncryptError::InvalidCiphertext(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(ReEncryptError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(ReEncryptError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ReEncryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ReEncryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(ReEncryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ReEncryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by RetireGrant
#[derive(Debug, PartialEq)]
pub enum RetireGrantError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl RetireGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RetireGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(RetireGrantError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(RetireGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantIdException" => {
                    return RusotoError::Service(RetireGrantError::InvalidGrantId(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(RetireGrantError::InvalidGrantToken(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(RetireGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(RetireGrantError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RetireGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by RevokeGrant
#[derive(Debug, PartialEq)]
pub enum RevokeGrantError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl RevokeGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(RevokeGrantError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(RevokeGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantIdException" => {
                    return RusotoError::Service(RevokeGrantError::InvalidGrantId(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(RevokeGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(RevokeGrantError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RevokeGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ScheduleKeyDeletion
#[derive(Debug, PartialEq)]
pub enum ScheduleKeyDeletionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ScheduleKeyDeletionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ScheduleKeyDeletionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(TagResourceError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(TagResourceError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(TagResourceError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TagException" => return RusotoError::Service(TagResourceError::Tag(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UntagResourceError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UntagResourceError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TagException" => return RusotoError::Service(UntagResourceError::Tag(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl UpdateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(UpdateAliasError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UpdateAliasError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum UpdateCustomKeyStoreError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p>The request was rejected because AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID. Retry the request with a different cluster ID.</p>
    CloudHsmClusterNotFound(String),
    /// <p>The request was rejected because the specified AWS CloudHSM cluster has a different cluster certificate than the original cluster. You cannot use the operation to specify an unrelated cluster.</p> <p>Specify a cluster that shares a backup history with the original cluster. This includes clusters that were created from a backup of the current cluster, and clusters that were created from the same backup that produced the current cluster.</p> <p>Clusters that share a backup history have the same cluster certificate. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    CloudHsmClusterNotRelated(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl UpdateCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CloudHsmClusterNotFoundException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotFound(err.msg),
                    )
                }
                "CloudHsmClusterNotRelatedException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotRelated(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(UpdateCustomKeyStoreError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCustomKeyStoreError {
    fn description(&self) -> &str {
        match *self {
            UpdateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => cause,
            UpdateCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => cause,
            UpdateCustomKeyStoreError::CloudHsmClusterNotFound(ref cause) => cause,
            UpdateCustomKeyStoreError::CloudHsmClusterNotRelated(ref cause) => cause,
            UpdateCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => cause,
            UpdateCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => cause,
            UpdateCustomKeyStoreError::KMSInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateKeyDescription
#[derive(Debug, PartialEq)]
pub enum UpdateKeyDescriptionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl UpdateKeyDescriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateKeyDescriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Trait representing the capabilities of the KMS API. KMS clients implement this trait.
pub trait Kms {
    /// <p>Cancels the deletion of a customer master key (CMK). When this operation is successful, the CMK is set to the <code>Disabled</code> state. To enable a CMK, use <a>EnableKey</a>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn cancel_key_deletion(
        &self,
        input: CancelKeyDeletionRequest,
    ) -> RusotoFuture<CancelKeyDeletionResponse, CancelKeyDeletionError>;

    /// <p>Connects or reconnects a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> to its associated AWS CloudHSM cluster.</p> <p>The custom key store must be connected before you can create customer master keys (CMKs) in the key store or use the CMKs it contains. You can disconnect and reconnect a custom key store at any time.</p> <p>To connect a custom key store, its associated AWS CloudHSM cluster must have at least one active HSM. To get the number of active HSMs in a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation. To add HSMs to the cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>The connection process can take an extended amount of time to complete; up to 20 minutes. This operation starts the connection process, but it does not wait for it to complete. When it succeeds, this operation quickly returns an HTTP 200 response and a JSON object with no properties. However, this response does not indicate that the custom key store is connected. To get the connection state of the custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>During the connection process, AWS KMS finds the AWS CloudHSM cluster that is associated with the custom key store, creates the connection infrastructure, connects to the cluster, logs into the AWS CloudHSM client as the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user</a> (CU), and rotates its password.</p> <p>The <code>ConnectCustomKeyStore</code> operation might fail for various reasons. To find the reason, use the <a>DescribeCustomKeyStores</a> operation and see the <code>ConnectionErrorCode</code> in the response. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>To fix the failure, use the <a>DisconnectCustomKeyStore</a> operation to disconnect the custom key store, correct the error, use the <a>UpdateCustomKeyStore</a> operation if necessary, and then use <code>ConnectCustomKeyStore</code> again.</p> <p>If you are having trouble connecting or disconnecting a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn connect_custom_key_store(
        &self,
        input: ConnectCustomKeyStoreRequest,
    ) -> RusotoFuture<ConnectCustomKeyStoreResponse, ConnectCustomKeyStoreError>;

    /// <p>Creates a display name for a customer managed customer master key (CMK). You can use an alias to identify a CMK in selected operations, such as <a>Encrypt</a> and <a>GenerateDataKey</a>. </p> <p>Each CMK can have multiple aliases, but each alias points to only one CMK. The alias name must be unique in the AWS account and region. To simplify code that runs in multiple regions, use the same alias name, but point it to a different CMK in each region. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation.</p> <p>The alias name must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMKs</a>. </p> <p>The alias and the CMK it is mapped to must be in the same AWS account and the same region. You cannot perform this operation on an alias in a different AWS account.</p> <p>To map an existing alias to a different CMK, call <a>UpdateAlias</a>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_alias(&self, input: CreateAliasRequest) -> RusotoFuture<(), CreateAliasError>;

    /// <p>Creates a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that is associated with an <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/clusters.html">AWS CloudHSM cluster</a> that you own and manage.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>Before you create the custom key store, you must assemble the required elements, including an AWS CloudHSM cluster that fulfills the requirements for a custom key store. For details about the required elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the operation completes successfully, it returns the ID of the new custom key store. Before you can use your new custom key store, you need to use the <a>ConnectCustomKeyStore</a> operation to connect the new key store to its AWS CloudHSM cluster. Even if you are not going to use your custom key store immediately, you might want to connect it to verify that all settings are correct and then disconnect it until you are ready to use it.</p> <p>For help with failures, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_custom_key_store(
        &self,
        input: CreateCustomKeyStoreRequest,
    ) -> RusotoFuture<CreateCustomKeyStoreResponse, CreateCustomKeyStoreError>;

    /// <p>Adds a grant to a customer master key (CMK). The grant allows the grantee principal to use the CMK when the conditions specified in the grant are met. When setting permissions, grants are an alternative to key policies. </p> <p>To create a grant that allows a cryptographic operation only when the encryption context in the operation request matches or includes a specified encryption context, use the <code>Constraints</code> parameter. For details, see <a>GrantConstraints</a>.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter. For more information about grants, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> RusotoFuture<CreateGrantResponse, CreateGrantError>;

    /// <p>Creates a customer managed <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">customer master key</a> (CMK) in your AWS account.</p> <p>You can use a CMK to encrypt small amounts of data (up to 4096 bytes) directly. But CMKs are more commonly used to encrypt the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data keys</a> that are used to encrypt data.</p> <p>To create a CMK for imported key material, use the <code>Origin</code> parameter with a value of <code>EXTERNAL</code>.</p> <p>To create a CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, use the <code>CustomKeyStoreId</code> parameter to specify the custom key store. You must also use the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs in different Availability Zones in the AWS Region.</p> <p>You cannot use this operation to create a CMK in a different AWS account.</p>
    fn create_key(
        &self,
        input: CreateKeyRequest,
    ) -> RusotoFuture<CreateKeyResponse, CreateKeyError>;

    /// <p>Decrypts ciphertext. Ciphertext is plaintext that has been previously encrypted by using any of the following operations:</p> <ul> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> </ul> <p>Whenever possible, use key policies to give users permission to call the Decrypt operation on the CMK, instead of IAM policies. Otherwise, you might create an IAM user policy that gives the user Decrypt permission on all CMKs. This user could decrypt ciphertext that was encrypted by CMKs in other accounts if the key policy for the cross-account CMK permits it. If you must use an IAM policy for <code>Decrypt</code> permissions, limit the user to particular CMKs or particular trusted accounts.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn decrypt(&self, input: DecryptRequest) -> RusotoFuture<DecryptResponse, DecryptError>;

    /// <p>Deletes the specified alias. You cannot perform this operation on an alias in a different AWS account. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation. </p> <p>Each CMK can have multiple aliases. To change the alias of a CMK, use <a>DeleteAlias</a> to delete the current alias and <a>CreateAlias</a> to create a new alias. To associate an existing alias with a different customer master key (CMK), call <a>UpdateAlias</a>.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError>;

    /// <p>Deletes a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. This operation does not delete the AWS CloudHSM cluster that is associated with the custom key store, or affect any users or keys in the cluster.</p> <p>The custom key store that you delete cannot contain any AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">customer master keys (CMKs)</a>. Before deleting the key store, verify that you will never need to use any of the CMKs in the key store for any cryptographic operations. Then, use <a>ScheduleKeyDeletion</a> to delete the AWS KMS customer master keys (CMKs) from the key store. When the scheduled waiting period expires, the <code>ScheduleKeyDeletion</code> operation deletes the CMKs. Then it makes a best effort to delete the key material from the associated cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>After all CMKs are deleted from AWS KMS, use <a>DisconnectCustomKeyStore</a> to disconnect the key store from AWS KMS. Then, you can delete the custom key store.</p> <p>Instead of deleting the custom key store, consider using <a>DisconnectCustomKeyStore</a> to disconnect it from AWS KMS. While the key store is disconnected, you cannot create or use the CMKs in the key store. But, you do not need to delete CMKs and you can reconnect a disconnected custom key store at any time.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn delete_custom_key_store(
        &self,
        input: DeleteCustomKeyStoreRequest,
    ) -> RusotoFuture<DeleteCustomKeyStoreResponse, DeleteCustomKeyStoreError>;

    /// <p>Deletes key material that you previously imported. This operation makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK's state. Otherwise, it changes the CMK's state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn delete_imported_key_material(
        &self,
        input: DeleteImportedKeyMaterialRequest,
    ) -> RusotoFuture<(), DeleteImportedKeyMaterialError>;

    /// <p>Gets information about <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key stores</a> in the account and region.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>By default, this operation returns information about all custom key stores in the account and region. To get only information about a particular custom key store, use either the <code>CustomKeyStoreName</code> or <code>CustomKeyStoreId</code> parameter (but not both).</p> <p>To determine whether the custom key store is connected to its AWS CloudHSM cluster, use the <code>ConnectionState</code> element in the response. If an attempt to connect the custom key store failed, the <code>ConnectionState</code> value is <code>FAILED</code> and the <code>ConnectionErrorCode</code> element in the response indicates the cause of the failure. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>Custom key stores have a <code>DISCONNECTED</code> connection state if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If your custom key store state is <code>CONNECTED</code> but you are having trouble using it, make sure that its associated AWS CloudHSM cluster is active and contains the minimum number of HSMs required for the operation, if any.</p> <p> For help repairing your custom key store, see the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting Custom Key Stores</a> topic in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn describe_custom_key_stores(
        &self,
        input: DescribeCustomKeyStoresRequest,
    ) -> RusotoFuture<DescribeCustomKeyStoresResponse, DescribeCustomKeyStoresError>;

    /// <p>Provides detailed information about the specified customer master key (CMK).</p> <p>You can use <code>DescribeKey</code> on a predefined AWS alias, that is, an AWS alias with no key ID. When you do, AWS KMS associates the alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a> and returns its <code>KeyId</code> and <code>Arn</code> in the response.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn describe_key(
        &self,
        input: DescribeKeyRequest,
    ) -> RusotoFuture<DescribeKeyResponse, DescribeKeyError>;

    /// <p>Sets the state of a customer master key (CMK) to disabled, thereby preventing its use for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key(&self, input: DisableKeyRequest) -> RusotoFuture<(), DisableKeyError>;

    /// <p>Disables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key_rotation(
        &self,
        input: DisableKeyRotationRequest,
    ) -> RusotoFuture<(), DisableKeyRotationError>;

    /// <p>Disconnects the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> from its associated AWS CloudHSM cluster. While a custom key store is disconnected, you can manage the custom key store and its customer master keys (CMKs), but you cannot create or use CMKs in the custom key store. You can reconnect the custom key store at any time.</p> <note> <p>While a custom key store is disconnected, all attempts to create customer master keys (CMKs) in the custom key store or to use existing CMKs in cryptographic operations will fail. This action can prevent users from storing and accessing sensitive data.</p> </note> <p/> <p>To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation. To reconnect a custom key store, use the <a>ConnectCustomKeyStore</a> operation.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn disconnect_custom_key_store(
        &self,
        input: DisconnectCustomKeyStoreRequest,
    ) -> RusotoFuture<DisconnectCustomKeyStoreResponse, DisconnectCustomKeyStoreError>;

    /// <p>Sets the key state of a customer master key (CMK) to enabled. This allows you to use the CMK for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn enable_key(&self, input: EnableKeyRequest) -> RusotoFuture<(), EnableKeyError>;

    /// <p>Enables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>You cannot enable automatic rotation of CMKs with imported key material or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn enable_key_rotation(
        &self,
        input: EnableKeyRotationRequest,
    ) -> RusotoFuture<(), EnableKeyRotationError>;

    /// <p>Encrypts plaintext into ciphertext by using a customer master key (CMK). The <code>Encrypt</code> operation has two primary use cases:</p> <ul> <li> <p>You can encrypt up to 4 kilobytes (4096 bytes) of arbitrary data such as an RSA key, a database password, or other sensitive information.</p> </li> <li> <p>You can use the <code>Encrypt</code> operation to move encrypted data from one AWS region to another. In the first region, generate a data key and use the plaintext key to encrypt the data. Then, in the new region, call the <code>Encrypt</code> method on same plaintext data key. Now, you can safely move the encrypted data and encrypted data key to the new region, and decrypt in the new region when necessary.</p> </li> </ul> <p>You don't need use this operation to encrypt a data key within a region. The <a>GenerateDataKey</a> and <a>GenerateDataKeyWithoutPlaintext</a> operations return an encrypted data key.</p> <p>Also, you don't need to use this operation to encrypt data in your application. You can use the plaintext and encrypted data keys that the <code>GenerateDataKey</code> operation returns.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn encrypt(&self, input: EncryptRequest) -> RusotoFuture<EncryptResponse, EncryptError>;

    /// <p>Generates a unique data key. This operation returns a plaintext copy of the data key and a copy that is encrypted under a customer master key (CMK) that you specify. You can use the plaintext key to encrypt your data outside of KMS and store the encrypted data key with the encrypted data.</p> <p> <code>GenerateDataKey</code> returns a unique data key for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the data key.</p> <p>To generate a data key, you need to specify the customer master key (CMK) that will be used to encrypt the data key. You must also specify the length of the data key using either the <code>KeySpec</code> or <code>NumberOfBytes</code> field (but not both). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use <code>KeySpec</code>. To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p> <p>You will find the plaintext copy of the data key in the <code>Plaintext</code> field of the response, and the encrypted copy of the data key in the <code>CiphertextBlob</code> field.</p> <p>We recommend that you use the following pattern to encrypt data locally in your application:</p> <ol> <li> <p>Use the <code>GenerateDataKey</code> operation to get a data encryption key.</p> </li> <li> <p>Use the plaintext data key (returned in the <code>Plaintext</code> field of the response) to encrypt data locally, then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (returned in the <code>CiphertextBlob</code> field of the response) alongside the locally encrypted data.</p> </li> </ol> <p>To decrypt data locally:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key. The operation returns a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data locally, then erase the plaintext data key from memory.</p> </li> </ol> <p>To get only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To get a cryptographically secure random byte string, use <a>GenerateRandom</a>.</p> <p>You can use the optional encryption context to add additional security to your encryption operation. When you specify an <code>EncryptionContext</code> in the <code>GenerateDataKey</code> operation, you must specify the same encryption context (a case-sensitive exact match) in your request to <a>Decrypt</a> the data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key(
        &self,
        input: GenerateDataKeyRequest,
    ) -> RusotoFuture<GenerateDataKeyResponse, GenerateDataKeyError>;

    /// <p>Generates a unique data key. This operation returns a data key that is encrypted under a customer master key (CMK) that you specify. <code>GenerateDataKeyWithoutPlaintext</code> is identical to <a>GenerateDataKey</a> except that returns only the encrypted copy of the data key.</p> <p>Like <code>GenerateDataKey</code>, <code>GenerateDataKeyWithoutPlaintext</code> returns a unique data key for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the data key.</p> <p>This operation is useful for systems that need to encrypt data at some point, but not immediately. When you need to encrypt the data, you call the <a>Decrypt</a> operation on the encrypted copy of the key.</p> <p>It's also useful in distributed systems with different levels of trust. For example, you might store encrypted data in containers. One component of your system creates new containers and stores an encrypted data key with each container. Then, a different component puts the data into the containers. That component first decrypts the data key, uses the plaintext data key to encrypt data, puts the encrypted data into the container, and then destroys the plaintext data key. In this system, the component that creates the containers never sees the plaintext data key.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key_without_plaintext(
        &self,
        input: GenerateDataKeyWithoutPlaintextRequest,
    ) -> RusotoFuture<GenerateDataKeyWithoutPlaintextResponse, GenerateDataKeyWithoutPlaintextError>;

    /// <p>Returns a random byte string that is cryptographically secure.</p> <p>By default, the random byte string is generated in AWS KMS. To generate the byte string in the AWS CloudHSM cluster that is associated with a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, specify the custom key store ID.</p> <p>For more information about entropy and random number generation, see the <a href="https://d0.awsstatic.com/whitepapers/KMS-Cryptographic-Details.pdf">AWS Key Management Service Cryptographic Details</a> whitepaper.</p>
    fn generate_random(
        &self,
        input: GenerateRandomRequest,
    ) -> RusotoFuture<GenerateRandomResponse, GenerateRandomError>;

    /// <p>Gets a key policy attached to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn get_key_policy(
        &self,
        input: GetKeyPolicyRequest,
    ) -> RusotoFuture<GetKeyPolicyResponse, GetKeyPolicyError>;

    /// <p>Gets a Boolean value that indicates whether <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> is enabled for the specified customer master key (CMK).</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <ul> <li> <p>Disabled: The key rotation status does not change when you disable a CMK. However, while the CMK is disabled, AWS KMS does not rotate the backing key.</p> </li> <li> <p>Pending deletion: While a CMK is pending deletion, its key rotation status is <code>false</code> and AWS KMS does not rotate the backing key. If you cancel the deletion, the original key rotation status is restored.</p> </li> </ul> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn get_key_rotation_status(
        &self,
        input: GetKeyRotationStatusRequest,
    ) -> RusotoFuture<GetKeyRotationStatusResponse, GetKeyRotationStatusError>;

    /// <p>Returns the items you need in order to import key material into AWS KMS from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You must specify the key ID of the customer master key (CMK) into which you will import key material. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material. You cannot perform this operation on a CMK in a different AWS account.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request. The public key and import token from the same response must be used together. These items are valid for 24 hours. When they expire, they cannot be used for a subsequent <a>ImportKeyMaterial</a> request. To get new ones, send another <code>GetParametersForImport</code> request.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn get_parameters_for_import(
        &self,
        input: GetParametersForImportRequest,
    ) -> RusotoFuture<GetParametersForImportResponse, GetParametersForImportError>;

    /// <p>Imports key material into an existing AWS KMS customer master key (CMK) that was created without key material. You cannot perform this operation on a CMK in a different AWS account. For more information about creating CMKs with no key material and then importing key material, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Before using this operation, call <a>GetParametersForImport</a>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p> <p>When calling this operation, you must specify the following values:</p> <ul> <li> <p>The key ID or key ARN of a CMK with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a CMK with no key material, call <a>CreateKey</a> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a CMK, call <a>DescribeKey</a>.)</p> </li> <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <a>GetParametersForImport</a>.</p> </li> <li> <p>The import token that <a>GetParametersForImport</a> returned. This token and the public key used to encrypt the key material must have come from the same response.</p> </li> <li> <p>Whether the key material expires and if so, when. If you set an expiration date, you can change it only by reimporting the same key material and specifying a new expiration date. If the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. To use the CMK again, you must reimport the same key material.</p> </li> </ul> <p>When this operation is successful, the key state of the CMK changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the CMK. After you successfully import key material into a CMK, you can reimport the same key material into that CMK, but you cannot import different key material.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn import_key_material(
        &self,
        input: ImportKeyMaterialRequest,
    ) -> RusotoFuture<ImportKeyMaterialResponse, ImportKeyMaterialError>;

    /// <p>Gets a list of aliases in the caller's AWS account and region. You cannot list aliases in other accounts. For more information about aliases, see <a>CreateAlias</a>.</p> <p>By default, the ListAliases command returns all aliases in the account and region. To get only the aliases that point to a particular customer master key (CMK), use the <code>KeyId</code> parameter.</p> <p>The <code>ListAliases</code> response can include aliases that you created and associated with your customer managed CMKs, and aliases that AWS created and associated with AWS managed CMKs in your account. You can recognize AWS aliases because their names have the format <code>aws/&lt;service-name&gt;</code>, such as <code>aws/dynamodb</code>.</p> <p>The response might also include aliases that have no <code>TargetKeyId</code> field. These are predefined aliases that AWS has created but has not yet associated with a CMK. Aliases that AWS creates in your account, including predefined aliases, do not count against your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html#aliases-limit">AWS KMS aliases limit</a>.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError>;

    /// <p>Gets a list of all grants for the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
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

    /// <p>Attaches a key policy to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn put_key_policy(&self, input: PutKeyPolicyRequest) -> RusotoFuture<(), PutKeyPolicyError>;

    /// <p>Encrypts data on the server side with a new customer master key (CMK) without exposing the plaintext of the data on the client side. The data is first decrypted and then reencrypted. You can also use this operation to change the encryption context of a ciphertext. </p> <p>You can reencrypt data using CMKs in different AWS accounts.</p> <p>Unlike other operations, <code>ReEncrypt</code> is authorized twice, once as <code>ReEncryptFrom</code> on the source CMK and once as <code>ReEncryptTo</code> on the destination CMK. We recommend that you include the <code>"kms:ReEncrypt*"</code> permission in your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policies</a> to permit reencryption from or to the CMK. This permission is automatically included in the key policy when you create a CMK through the console. But you must include it manually when you create a CMK programmatically or when you set a key policy with the <a>PutKeyPolicy</a> operation.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn re_encrypt(
        &self,
        input: ReEncryptRequest,
    ) -> RusotoFuture<ReEncryptResponse, ReEncryptError>;

    /// <p>Retires a grant. To clean up, you can retire a grant when you're done using it. You should revoke a grant when you intend to actively deny operations that depend on it. The following are permitted to call this API:</p> <ul> <li> <p>The AWS account (root user) under which the grant was created</p> </li> <li> <p>The <code>RetiringPrincipal</code>, if present in the grant</p> </li> <li> <p>The <code>GranteePrincipal</code>, if <code>RetireGrant</code> is an operation specified in the grant</p> </li> </ul> <p>You must identify the grant to retire by its grant token or by a combination of the grant ID and the Amazon Resource Name (ARN) of the customer master key (CMK). A grant token is a unique variable-length base64-encoded string. A grant ID is a 64 character unique identifier of a grant. The <a>CreateGrant</a> operation returns both.</p>
    fn retire_grant(&self, input: RetireGrantRequest) -> RusotoFuture<(), RetireGrantError>;

    /// <p>Revokes the specified grant for the specified customer master key (CMK). You can revoke a grant to actively deny operations that depend on it.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn revoke_grant(&self, input: RevokeGrantRequest) -> RusotoFuture<(), RevokeGrantError>;

    /// <p>Schedules the deletion of a customer master key (CMK). You may provide a waiting period, specified in days, before deletion occurs. If you do not provide a waiting period, the default period of 30 days is used. When this operation is successful, the key state of the CMK changes to <code>PendingDeletion</code>. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK and all AWS KMS data associated with it, including all aliases that refer to it.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is unrecoverable. To prevent the use of a CMK without deleting it, use <a>DisableKey</a>.</p> </important> <p>If you schedule deletion of a CMK from a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, when the waiting period expires, <code>ScheduleKeyDeletion</code> deletes the CMK from AWS KMS. Then AWS KMS makes a best effort to delete the key material from the associated AWS CloudHSM cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling a CMK for deletion, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn schedule_key_deletion(
        &self,
        input: ScheduleKeyDeletionRequest,
    ) -> RusotoFuture<ScheduleKeyDeletionResponse, ScheduleKeyDeletionError>;

    /// <p>Adds or edits tags for a customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>You can only use a tag key once for each CMK. If you use the tag key again, AWS KMS replaces the current tag value with the specified value.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Removes the specified tags from the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>To remove a tag, specify the tag key. To change the tag value of an existing tag key, use <a>TagResource</a>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Associates an existing alias with a different customer master key (CMK). Each CMK can have multiple aliases, but the aliases must be unique within the account and region. You cannot perform this operation on an alias in a different AWS account.</p> <p>This operation works only on existing aliases. To change the alias of a CMK to a new value, use <a>CreateAlias</a> to create a new alias and <a>DeleteAlias</a> to delete the old alias.</p> <p>Because an alias is not a property of a CMK, you can create, update, and delete the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs in the account, use the <a>ListAliases</a> operation. </p> <p>The alias name must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMKs</a>. </p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn update_alias(&self, input: UpdateAliasRequest) -> RusotoFuture<(), UpdateAliasError>;

    /// <p>Changes the properties of a custom key store. Use the <code>CustomKeyStoreId</code> parameter to identify the custom key store you want to edit. Use the remaining parameters to change the properties of the custom key store.</p> <p>You can only update a custom key store that is disconnected. To disconnect the custom key store, use <a>DisconnectCustomKeyStore</a>. To reconnect the custom key store after the update completes, use <a>ConnectCustomKeyStore</a>. To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>Use the parameters of <code>UpdateCustomKeyStore</code> to edit your keystore settings.</p> <ul> <li> <p>Use the <b>NewCustomKeyStoreName</b> parameter to change the friendly name of the custom key store to the value that you specify.</p> <p> </p> </li> <li> <p>Use the <b>KeyStorePassword</b> parameter tell AWS KMS the current password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU)</a> in the associated AWS CloudHSM cluster. You can use this parameter to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-password">fix connection failures</a> that occur when AWS KMS cannot log into the associated cluster because the <code>kmsuser</code> password has changed. This value does not change the password in the AWS CloudHSM cluster.</p> <p> </p> </li> <li> <p>Use the <b>CloudHsmClusterId</b> parameter to associate the custom key store with a different, but related, AWS CloudHSM cluster. You can use this parameter to repair a custom key store if its AWS CloudHSM cluster becomes corrupted or is deleted, or when you need to create or restore a cluster from a backup. </p> </li> </ul> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn update_custom_key_store(
        &self,
        input: UpdateCustomKeyStoreRequest,
    ) -> RusotoFuture<UpdateCustomKeyStoreResponse, UpdateCustomKeyStoreError>;

    /// <p>Updates the description of a customer master key (CMK). To see the description of a CMK, use <a>DescribeKey</a>. </p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn update_key_description(
        &self,
        input: UpdateKeyDescriptionRequest,
    ) -> RusotoFuture<(), UpdateKeyDescriptionError>;
}
/// A client for the KMS API.
#[derive(Clone)]
pub struct KmsClient {
    client: Client,
    region: region::Region,
}

impl KmsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KmsClient {
        KmsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KmsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KmsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl Kms for KmsClient {
    /// <p>Cancels the deletion of a customer master key (CMK). When this operation is successful, the CMK is set to the <code>Disabled</code> state. To enable a CMK, use <a>EnableKey</a>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn cancel_key_deletion(
        &self,
        input: CancelKeyDeletionRequest,
    ) -> RusotoFuture<CancelKeyDeletionResponse, CancelKeyDeletionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CancelKeyDeletion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| CancelKeyDeletionError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<CancelKeyDeletionError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<CancelKeyDeletionResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<CancelKeyDeletionError>
                            })
                            .and_then(|response| {
                                Err(CancelKeyDeletionError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Connects or reconnects a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> to its associated AWS CloudHSM cluster.</p> <p>The custom key store must be connected before you can create customer master keys (CMKs) in the key store or use the CMKs it contains. You can disconnect and reconnect a custom key store at any time.</p> <p>To connect a custom key store, its associated AWS CloudHSM cluster must have at least one active HSM. To get the number of active HSMs in a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation. To add HSMs to the cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>The connection process can take an extended amount of time to complete; up to 20 minutes. This operation starts the connection process, but it does not wait for it to complete. When it succeeds, this operation quickly returns an HTTP 200 response and a JSON object with no properties. However, this response does not indicate that the custom key store is connected. To get the connection state of the custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>During the connection process, AWS KMS finds the AWS CloudHSM cluster that is associated with the custom key store, creates the connection infrastructure, connects to the cluster, logs into the AWS CloudHSM client as the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user</a> (CU), and rotates its password.</p> <p>The <code>ConnectCustomKeyStore</code> operation might fail for various reasons. To find the reason, use the <a>DescribeCustomKeyStores</a> operation and see the <code>ConnectionErrorCode</code> in the response. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>To fix the failure, use the <a>DisconnectCustomKeyStore</a> operation to disconnect the custom key store, correct the error, use the <a>UpdateCustomKeyStore</a> operation if necessary, and then use <code>ConnectCustomKeyStore</code> again.</p> <p>If you are having trouble connecting or disconnecting a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn connect_custom_key_store(
        &self,
        input: ConnectCustomKeyStoreRequest,
    ) -> RusotoFuture<ConnectCustomKeyStoreResponse, ConnectCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ConnectCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ConnectCustomKeyStoreError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ConnectCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ConnectCustomKeyStoreResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ConnectCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                Err(ConnectCustomKeyStoreError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Creates a display name for a customer managed customer master key (CMK). You can use an alias to identify a CMK in selected operations, such as <a>Encrypt</a> and <a>GenerateDataKey</a>. </p> <p>Each CMK can have multiple aliases, but each alias points to only one CMK. The alias name must be unique in the AWS account and region. To simplify code that runs in multiple regions, use the same alias name, but point it to a different CMK in each region. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation.</p> <p>The alias name must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMKs</a>. </p> <p>The alias and the CMK it is mapped to must be in the same AWS account and the same region. You cannot perform this operation on an alias in a different AWS account.</p> <p>To map an existing alias to a different CMK, call <a>UpdateAlias</a>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_alias(&self, input: CreateAliasRequest) -> RusotoFuture<(), CreateAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<CreateAliasError>
                            })
                            .and_then(|response| Err(CreateAliasError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Creates a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that is associated with an <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/clusters.html">AWS CloudHSM cluster</a> that you own and manage.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>Before you create the custom key store, you must assemble the required elements, including an AWS CloudHSM cluster that fulfills the requirements for a custom key store. For details about the required elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the operation completes successfully, it returns the ID of the new custom key store. Before you can use your new custom key store, you need to use the <a>ConnectCustomKeyStore</a> operation to connect the new key store to its AWS CloudHSM cluster. Even if you are not going to use your custom key store immediately, you might want to connect it to verify that all settings are correct and then disconnect it until you are ready to use it.</p> <p>For help with failures, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_custom_key_store(
        &self,
        input: CreateCustomKeyStoreRequest,
    ) -> RusotoFuture<CreateCustomKeyStoreResponse, CreateCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| CreateCustomKeyStoreError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<CreateCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<CreateCustomKeyStoreResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<CreateCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                Err(CreateCustomKeyStoreError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Adds a grant to a customer master key (CMK). The grant allows the grantee principal to use the CMK when the conditions specified in the grant are met. When setting permissions, grants are an alternative to key policies. </p> <p>To create a grant that allows a cryptographic operation only when the encryption context in the operation request matches or includes a specified encryption context, use the <code>Constraints</code> parameter. For details, see <a>GrantConstraints</a>.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter. For more information about grants, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> RusotoFuture<CreateGrantResponse, CreateGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| CreateGrantError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<CreateGrantError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<CreateGrantResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<CreateGrantError>
                            })
                            .and_then(|response| Err(CreateGrantError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Creates a customer managed <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">customer master key</a> (CMK) in your AWS account.</p> <p>You can use a CMK to encrypt small amounts of data (up to 4096 bytes) directly. But CMKs are more commonly used to encrypt the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data keys</a> that are used to encrypt data.</p> <p>To create a CMK for imported key material, use the <code>Origin</code> parameter with a value of <code>EXTERNAL</code>.</p> <p>To create a CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, use the <code>CustomKeyStoreId</code> parameter to specify the custom key store. You must also use the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs in different Availability Zones in the AWS Region.</p> <p>You cannot use this operation to create a CMK in a different AWS account.</p>
    fn create_key(
        &self,
        input: CreateKeyRequest,
    ) -> RusotoFuture<CreateKeyResponse, CreateKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| CreateKeyError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<CreateKeyError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<CreateKeyResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<CreateKeyError>
                            })
                            .and_then(|response| Err(CreateKeyError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Decrypts ciphertext. Ciphertext is plaintext that has been previously encrypted by using any of the following operations:</p> <ul> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>Encrypt</a> </p> </li> </ul> <p>Whenever possible, use key policies to give users permission to call the Decrypt operation on the CMK, instead of IAM policies. Otherwise, you might create an IAM user policy that gives the user Decrypt permission on all CMKs. This user could decrypt ciphertext that was encrypted by CMKs in other accounts if the key policy for the cross-account CMK permits it. If you must use an IAM policy for <code>Decrypt</code> permissions, limit the user to particular CMKs or particular trusted accounts.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn decrypt(&self, input: DecryptRequest) -> RusotoFuture<DecryptResponse, DecryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Decrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| DecryptError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| RusotoError::HttpDispatch(e) as RusotoError<DecryptError>)
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<DecryptResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| RusotoError::HttpDispatch(e) as RusotoError<DecryptError>)
                            .and_then(|response| Err(DecryptError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Deletes the specified alias. You cannot perform this operation on an alias in a different AWS account. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation. </p> <p>Each CMK can have multiple aliases. To change the alias of a CMK, use <a>DeleteAlias</a> to delete the current alias and <a>CreateAlias</a> to create a new alias. To associate an existing alias with a different customer master key (CMK), call <a>UpdateAlias</a>.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DeleteAliasError>
                            })
                            .and_then(|response| Err(DeleteAliasError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Deletes a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. This operation does not delete the AWS CloudHSM cluster that is associated with the custom key store, or affect any users or keys in the cluster.</p> <p>The custom key store that you delete cannot contain any AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">customer master keys (CMKs)</a>. Before deleting the key store, verify that you will never need to use any of the CMKs in the key store for any cryptographic operations. Then, use <a>ScheduleKeyDeletion</a> to delete the AWS KMS customer master keys (CMKs) from the key store. When the scheduled waiting period expires, the <code>ScheduleKeyDeletion</code> operation deletes the CMKs. Then it makes a best effort to delete the key material from the associated cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>After all CMKs are deleted from AWS KMS, use <a>DisconnectCustomKeyStore</a> to disconnect the key store from AWS KMS. Then, you can delete the custom key store.</p> <p>Instead of deleting the custom key store, consider using <a>DisconnectCustomKeyStore</a> to disconnect it from AWS KMS. While the key store is disconnected, you cannot create or use the CMKs in the key store. But, you do not need to delete CMKs and you can reconnect a disconnected custom key store at any time.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn delete_custom_key_store(
        &self,
        input: DeleteCustomKeyStoreRequest,
    ) -> RusotoFuture<DeleteCustomKeyStoreResponse, DeleteCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| DeleteCustomKeyStoreError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<DeleteCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<DeleteCustomKeyStoreResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<DeleteCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                Err(DeleteCustomKeyStoreError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Deletes key material that you previously imported. This operation makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK's state. Otherwise, it changes the CMK's state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn delete_imported_key_material(
        &self,
        input: DeleteImportedKeyMaterialRequest,
    ) -> RusotoFuture<(), DeleteImportedKeyMaterialError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteImportedKeyMaterial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<DeleteImportedKeyMaterialError>
                            })
                            .and_then(|response| {
                                Err(DeleteImportedKeyMaterialError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Gets information about <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key stores</a> in the account and region.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>By default, this operation returns information about all custom key stores in the account and region. To get only information about a particular custom key store, use either the <code>CustomKeyStoreName</code> or <code>CustomKeyStoreId</code> parameter (but not both).</p> <p>To determine whether the custom key store is connected to its AWS CloudHSM cluster, use the <code>ConnectionState</code> element in the response. If an attempt to connect the custom key store failed, the <code>ConnectionState</code> value is <code>FAILED</code> and the <code>ConnectionErrorCode</code> element in the response indicates the cause of the failure. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>Custom key stores have a <code>DISCONNECTED</code> connection state if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If your custom key store state is <code>CONNECTED</code> but you are having trouble using it, make sure that its associated AWS CloudHSM cluster is active and contains the minimum number of HSMs required for the operation, if any.</p> <p> For help repairing your custom key store, see the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting Custom Key Stores</a> topic in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn describe_custom_key_stores(
        &self,
        input: DescribeCustomKeyStoresRequest,
    ) -> RusotoFuture<DescribeCustomKeyStoresResponse, DescribeCustomKeyStoresError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DescribeCustomKeyStores");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| DescribeCustomKeyStoresError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<DescribeCustomKeyStoresError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<DescribeCustomKeyStoresResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<DescribeCustomKeyStoresError>
                            })
                            .and_then(|response| {
                                Err(DescribeCustomKeyStoresError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Provides detailed information about the specified customer master key (CMK).</p> <p>You can use <code>DescribeKey</code> on a predefined AWS alias, that is, an AWS alias with no key ID. When you do, AWS KMS associates the alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a> and returns its <code>KeyId</code> and <code>Arn</code> in the response.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn describe_key(
        &self,
        input: DescribeKeyRequest,
    ) -> RusotoFuture<DescribeKeyResponse, DescribeKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DescribeKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| DescribeKeyError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DescribeKeyError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<DescribeKeyResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DescribeKeyError>
                            })
                            .and_then(|response| Err(DescribeKeyError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Sets the state of a customer master key (CMK) to disabled, thereby preventing its use for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key(&self, input: DisableKeyRequest) -> RusotoFuture<(), DisableKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisableKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DisableKeyError>
                            })
                            .and_then(|response| Err(DisableKeyError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Disables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key_rotation(
        &self,
        input: DisableKeyRotationRequest,
    ) -> RusotoFuture<(), DisableKeyRotationError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisableKeyRotation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<DisableKeyRotationError>
                            })
                            .and_then(|response| {
                                Err(DisableKeyRotationError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Disconnects the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> from its associated AWS CloudHSM cluster. While a custom key store is disconnected, you can manage the custom key store and its customer master keys (CMKs), but you cannot create or use CMKs in the custom key store. You can reconnect the custom key store at any time.</p> <note> <p>While a custom key store is disconnected, all attempts to create customer master keys (CMKs) in the custom key store or to use existing CMKs in cryptographic operations will fail. This action can prevent users from storing and accessing sensitive data.</p> </note> <p/> <p>To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation. To reconnect a custom key store, use the <a>ConnectCustomKeyStore</a> operation.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn disconnect_custom_key_store(
        &self,
        input: DisconnectCustomKeyStoreRequest,
    ) -> RusotoFuture<DisconnectCustomKeyStoreResponse, DisconnectCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisconnectCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| DisconnectCustomKeyStoreError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<DisconnectCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<DisconnectCustomKeyStoreResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<DisconnectCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                Err(DisconnectCustomKeyStoreError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Sets the key state of a customer master key (CMK) to enabled. This allows you to use the CMK for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn enable_key(&self, input: EnableKeyRequest) -> RusotoFuture<(), EnableKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.EnableKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<EnableKeyError>
                            })
                            .and_then(|response| Err(EnableKeyError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Enables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>You cannot enable automatic rotation of CMKs with imported key material or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn enable_key_rotation(
        &self,
        input: EnableKeyRotationRequest,
    ) -> RusotoFuture<(), EnableKeyRotationError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.EnableKeyRotation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<EnableKeyRotationError>
                            })
                            .and_then(|response| {
                                Err(EnableKeyRotationError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Encrypts plaintext into ciphertext by using a customer master key (CMK). The <code>Encrypt</code> operation has two primary use cases:</p> <ul> <li> <p>You can encrypt up to 4 kilobytes (4096 bytes) of arbitrary data such as an RSA key, a database password, or other sensitive information.</p> </li> <li> <p>You can use the <code>Encrypt</code> operation to move encrypted data from one AWS region to another. In the first region, generate a data key and use the plaintext key to encrypt the data. Then, in the new region, call the <code>Encrypt</code> method on same plaintext data key. Now, you can safely move the encrypted data and encrypted data key to the new region, and decrypt in the new region when necessary.</p> </li> </ul> <p>You don't need use this operation to encrypt a data key within a region. The <a>GenerateDataKey</a> and <a>GenerateDataKeyWithoutPlaintext</a> operations return an encrypted data key.</p> <p>Also, you don't need to use this operation to encrypt data in your application. You can use the plaintext and encrypted data keys that the <code>GenerateDataKey</code> operation returns.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn encrypt(&self, input: EncryptRequest) -> RusotoFuture<EncryptResponse, EncryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Encrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| EncryptError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| RusotoError::HttpDispatch(e) as RusotoError<EncryptError>)
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<EncryptResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| RusotoError::HttpDispatch(e) as RusotoError<EncryptError>)
                            .and_then(|response| Err(EncryptError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Generates a unique data key. This operation returns a plaintext copy of the data key and a copy that is encrypted under a customer master key (CMK) that you specify. You can use the plaintext key to encrypt your data outside of KMS and store the encrypted data key with the encrypted data.</p> <p> <code>GenerateDataKey</code> returns a unique data key for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the data key.</p> <p>To generate a data key, you need to specify the customer master key (CMK) that will be used to encrypt the data key. You must also specify the length of the data key using either the <code>KeySpec</code> or <code>NumberOfBytes</code> field (but not both). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use <code>KeySpec</code>. To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p> <p>You will find the plaintext copy of the data key in the <code>Plaintext</code> field of the response, and the encrypted copy of the data key in the <code>CiphertextBlob</code> field.</p> <p>We recommend that you use the following pattern to encrypt data locally in your application:</p> <ol> <li> <p>Use the <code>GenerateDataKey</code> operation to get a data encryption key.</p> </li> <li> <p>Use the plaintext data key (returned in the <code>Plaintext</code> field of the response) to encrypt data locally, then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (returned in the <code>CiphertextBlob</code> field of the response) alongside the locally encrypted data.</p> </li> </ol> <p>To decrypt data locally:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key. The operation returns a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data locally, then erase the plaintext data key from memory.</p> </li> </ol> <p>To get only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To get a cryptographically secure random byte string, use <a>GenerateRandom</a>.</p> <p>You can use the optional encryption context to add additional security to your encryption operation. When you specify an <code>EncryptionContext</code> in the <code>GenerateDataKey</code> operation, you must specify the same encryption context (a case-sensitive exact match) in your request to <a>Decrypt</a> the data key. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key(
        &self,
        input: GenerateDataKeyRequest,
    ) -> RusotoFuture<GenerateDataKeyResponse, GenerateDataKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateDataKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| GenerateDataKeyError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<GenerateDataKeyError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<GenerateDataKeyResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<GenerateDataKeyError>
                            })
                            .and_then(|response| Err(GenerateDataKeyError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Generates a unique data key. This operation returns a data key that is encrypted under a customer master key (CMK) that you specify. <code>GenerateDataKeyWithoutPlaintext</code> is identical to <a>GenerateDataKey</a> except that returns only the encrypted copy of the data key.</p> <p>Like <code>GenerateDataKey</code>, <code>GenerateDataKeyWithoutPlaintext</code> returns a unique data key for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the data key.</p> <p>This operation is useful for systems that need to encrypt data at some point, but not immediately. When you need to encrypt the data, you call the <a>Decrypt</a> operation on the encrypted copy of the key.</p> <p>It's also useful in distributed systems with different levels of trust. For example, you might store encrypted data in containers. One component of your system creates new containers and stores an encrypted data key with each container. Then, a different component puts the data into the containers. That component first decrypts the data key, uses the plaintext data key to encrypt data, puts the encrypted data into the container, and then destroys the plaintext data key. In this system, the component that creates the containers never sees the plaintext data key.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| GenerateDataKeyWithoutPlaintextError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<GenerateDataKeyWithoutPlaintextError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<GenerateDataKeyWithoutPlaintextResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<GenerateDataKeyWithoutPlaintextError>
                            })
                            .and_then(|response| {
                                Err(GenerateDataKeyWithoutPlaintextError::from_response(
                                    response,
                                ))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Returns a random byte string that is cryptographically secure.</p> <p>By default, the random byte string is generated in AWS KMS. To generate the byte string in the AWS CloudHSM cluster that is associated with a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, specify the custom key store ID.</p> <p>For more information about entropy and random number generation, see the <a href="https://d0.awsstatic.com/whitepapers/KMS-Cryptographic-Details.pdf">AWS Key Management Service Cryptographic Details</a> whitepaper.</p>
    fn generate_random(
        &self,
        input: GenerateRandomRequest,
    ) -> RusotoFuture<GenerateRandomResponse, GenerateRandomError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateRandom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| GenerateRandomError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<GenerateRandomError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<GenerateRandomResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<GenerateRandomError>
                            })
                            .and_then(|response| Err(GenerateRandomError::from_response(response)))
                    })
                    .boxed()
            }
        })
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| GetKeyPolicyError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<GetKeyPolicyError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<GetKeyPolicyResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<GetKeyPolicyError>
                            })
                            .and_then(|response| Err(GetKeyPolicyError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Gets a Boolean value that indicates whether <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> is enabled for the specified customer master key (CMK).</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <ul> <li> <p>Disabled: The key rotation status does not change when you disable a CMK. However, while the CMK is disabled, AWS KMS does not rotate the backing key.</p> </li> <li> <p>Pending deletion: While a CMK is pending deletion, its key rotation status is <code>false</code> and AWS KMS does not rotate the backing key. If you cancel the deletion, the original key rotation status is restored.</p> </li> </ul> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn get_key_rotation_status(
        &self,
        input: GetKeyRotationStatusRequest,
    ) -> RusotoFuture<GetKeyRotationStatusResponse, GetKeyRotationStatusError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetKeyRotationStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| GetKeyRotationStatusError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<GetKeyRotationStatusError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<GetKeyRotationStatusResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<GetKeyRotationStatusError>
                            })
                            .and_then(|response| {
                                Err(GetKeyRotationStatusError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Returns the items you need in order to import key material into AWS KMS from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You must specify the key ID of the customer master key (CMK) into which you will import key material. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material. You cannot perform this operation on a CMK in a different AWS account.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request. The public key and import token from the same response must be used together. These items are valid for 24 hours. When they expire, they cannot be used for a subsequent <a>ImportKeyMaterial</a> request. To get new ones, send another <code>GetParametersForImport</code> request.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn get_parameters_for_import(
        &self,
        input: GetParametersForImportRequest,
    ) -> RusotoFuture<GetParametersForImportResponse, GetParametersForImportError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetParametersForImport");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| GetParametersForImportError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<GetParametersForImportError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<GetParametersForImportResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<GetParametersForImportError>
                            })
                            .and_then(|response| {
                                Err(GetParametersForImportError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Imports key material into an existing AWS KMS customer master key (CMK) that was created without key material. You cannot perform this operation on a CMK in a different AWS account. For more information about creating CMKs with no key material and then importing key material, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Before using this operation, call <a>GetParametersForImport</a>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p> <p>When calling this operation, you must specify the following values:</p> <ul> <li> <p>The key ID or key ARN of a CMK with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a CMK with no key material, call <a>CreateKey</a> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a CMK, call <a>DescribeKey</a>.)</p> </li> <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <a>GetParametersForImport</a>.</p> </li> <li> <p>The import token that <a>GetParametersForImport</a> returned. This token and the public key used to encrypt the key material must have come from the same response.</p> </li> <li> <p>Whether the key material expires and if so, when. If you set an expiration date, you can change it only by reimporting the same key material and specifying a new expiration date. If the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. To use the CMK again, you must reimport the same key material.</p> </li> </ul> <p>When this operation is successful, the key state of the CMK changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the CMK. After you successfully import key material into a CMK, you can reimport the same key material into that CMK, but you cannot import different key material.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn import_key_material(
        &self,
        input: ImportKeyMaterialRequest,
    ) -> RusotoFuture<ImportKeyMaterialResponse, ImportKeyMaterialError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ImportKeyMaterial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ImportKeyMaterialError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ImportKeyMaterialError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ImportKeyMaterialResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ImportKeyMaterialError>
                            })
                            .and_then(|response| {
                                Err(ImportKeyMaterialError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Gets a list of aliases in the caller's AWS account and region. You cannot list aliases in other accounts. For more information about aliases, see <a>CreateAlias</a>.</p> <p>By default, the ListAliases command returns all aliases in the account and region. To get only the aliases that point to a particular customer master key (CMK), use the <code>KeyId</code> parameter.</p> <p>The <code>ListAliases</code> response can include aliases that you created and associated with your customer managed CMKs, and aliases that AWS created and associated with AWS managed CMKs in your account. You can recognize AWS aliases because their names have the format <code>aws/&lt;service-name&gt;</code>, such as <code>aws/dynamodb</code>.</p> <p>The response might also include aliases that have no <code>TargetKeyId</code> field. These are predefined aliases that AWS has created but has not yet associated with a CMK. Aliases that AWS creates in your account, including predefined aliases, do not count against your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html#aliases-limit">AWS KMS aliases limit</a>.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListAliases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListAliasesError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListAliasesError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListAliasesResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListAliasesError>
                            })
                            .and_then(|response| Err(ListAliasesError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Gets a list of all grants for the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn list_grants(
        &self,
        input: ListGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListGrantsError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListGrantsError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListGrantsError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListGrantsResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListGrantsError>
                            })
                            .and_then(|response| Err(ListGrantsError::from_response(response)))
                    })
                    .boxed()
            }
        })
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListKeyPoliciesError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListKeyPoliciesError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListKeyPoliciesResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListKeyPoliciesError>
                            })
                            .and_then(|response| Err(ListKeyPoliciesError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Gets a list of all customer master keys (CMKs) in the caller's AWS account and region.</p>
    fn list_keys(&self, input: ListKeysRequest) -> RusotoFuture<ListKeysResponse, ListKeysError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListKeys");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListKeysError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| RusotoError::HttpDispatch(e) as RusotoError<ListKeysError>)
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListKeysResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| RusotoError::HttpDispatch(e) as RusotoError<ListKeysError>)
                            .and_then(|response| Err(ListKeysError::from_response(response)))
                    })
                    .boxed()
            }
        })
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListResourceTagsError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListResourceTagsError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListResourceTagsResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ListResourceTagsError>
                            })
                            .and_then(|response| {
                                Err(ListResourceTagsError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
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
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ListRetirableGrantsError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ListRetirableGrantsError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ListGrantsResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ListRetirableGrantsError>
                            })
                            .and_then(|response| {
                                Err(ListRetirableGrantsError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Attaches a key policy to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn put_key_policy(&self, input: PutKeyPolicyRequest) -> RusotoFuture<(), PutKeyPolicyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.PutKeyPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<PutKeyPolicyError>
                            })
                            .and_then(|response| Err(PutKeyPolicyError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Encrypts data on the server side with a new customer master key (CMK) without exposing the plaintext of the data on the client side. The data is first decrypted and then reencrypted. You can also use this operation to change the encryption context of a ciphertext. </p> <p>You can reencrypt data using CMKs in different AWS accounts.</p> <p>Unlike other operations, <code>ReEncrypt</code> is authorized twice, once as <code>ReEncryptFrom</code> on the source CMK and once as <code>ReEncryptTo</code> on the destination CMK. We recommend that you include the <code>"kms:ReEncrypt*"</code> permission in your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policies</a> to permit reencryption from or to the CMK. This permission is automatically included in the key policy when you create a CMK through the console. But you must include it manually when you create a CMK programmatically or when you set a key policy with the <a>PutKeyPolicy</a> operation.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn re_encrypt(
        &self,
        input: ReEncryptRequest,
    ) -> RusotoFuture<ReEncryptResponse, ReEncryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ReEncrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ReEncryptError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ReEncryptError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ReEncryptResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<ReEncryptError>
                            })
                            .and_then(|response| Err(ReEncryptError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Retires a grant. To clean up, you can retire a grant when you're done using it. You should revoke a grant when you intend to actively deny operations that depend on it. The following are permitted to call this API:</p> <ul> <li> <p>The AWS account (root user) under which the grant was created</p> </li> <li> <p>The <code>RetiringPrincipal</code>, if present in the grant</p> </li> <li> <p>The <code>GranteePrincipal</code>, if <code>RetireGrant</code> is an operation specified in the grant</p> </li> </ul> <p>You must identify the grant to retire by its grant token or by a combination of the grant ID and the Amazon Resource Name (ARN) of the customer master key (CMK). A grant token is a unique variable-length base64-encoded string. A grant ID is a 64 character unique identifier of a grant. The <a>CreateGrant</a> operation returns both.</p>
    fn retire_grant(&self, input: RetireGrantRequest) -> RusotoFuture<(), RetireGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.RetireGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<RetireGrantError>
                            })
                            .and_then(|response| Err(RetireGrantError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Revokes the specified grant for the specified customer master key (CMK). You can revoke a grant to actively deny operations that depend on it.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn revoke_grant(&self, input: RevokeGrantRequest) -> RusotoFuture<(), RevokeGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.RevokeGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<RevokeGrantError>
                            })
                            .and_then(|response| Err(RevokeGrantError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Schedules the deletion of a customer master key (CMK). You may provide a waiting period, specified in days, before deletion occurs. If you do not provide a waiting period, the default period of 30 days is used. When this operation is successful, the key state of the CMK changes to <code>PendingDeletion</code>. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK and all AWS KMS data associated with it, including all aliases that refer to it.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is unrecoverable. To prevent the use of a CMK without deleting it, use <a>DisableKey</a>.</p> </important> <p>If you schedule deletion of a CMK from a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, when the waiting period expires, <code>ScheduleKeyDeletion</code> deletes the CMK from AWS KMS. Then AWS KMS makes a best effort to delete the key material from the associated AWS CloudHSM cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling a CMK for deletion, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn schedule_key_deletion(
        &self,
        input: ScheduleKeyDeletionRequest,
    ) -> RusotoFuture<ScheduleKeyDeletionResponse, ScheduleKeyDeletionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ScheduleKeyDeletion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| ScheduleKeyDeletionError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ScheduleKeyDeletionError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<ScheduleKeyDeletionResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<ScheduleKeyDeletionError>
                            })
                            .and_then(|response| {
                                Err(ScheduleKeyDeletionError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Adds or edits tags for a customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>You can only use a tag key once for each CMK. If you use the tag key again, AWS KMS replaces the current tag value with the specified value.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<TagResourceError>
                            })
                            .and_then(|response| Err(TagResourceError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Removes the specified tags from the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>To remove a tag, specify the tag key. To change the tag value of an existing tag key, use <a>TagResource</a>.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<UntagResourceError>
                            })
                            .and_then(|response| Err(UntagResourceError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Associates an existing alias with a different customer master key (CMK). Each CMK can have multiple aliases, but the aliases must be unique within the account and region. You cannot perform this operation on an alias in a different AWS account.</p> <p>This operation works only on existing aliases. To change the alias of a CMK to a new value, use <a>CreateAlias</a> to create a new alias and <a>DeleteAlias</a> to delete the old alias.</p> <p>Because an alias is not a property of a CMK, you can create, update, and delete the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs in the account, use the <a>ListAliases</a> operation. </p> <p>The alias name must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMKs</a>. </p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn update_alias(&self, input: UpdateAliasRequest) -> RusotoFuture<(), UpdateAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e) as RusotoError<UpdateAliasError>
                            })
                            .and_then(|response| Err(UpdateAliasError::from_response(response)))
                    })
                    .boxed()
            }
        })
    }

    /// <p>Changes the properties of a custom key store. Use the <code>CustomKeyStoreId</code> parameter to identify the custom key store you want to edit. Use the remaining parameters to change the properties of the custom key store.</p> <p>You can only update a custom key store that is disconnected. To disconnect the custom key store, use <a>DisconnectCustomKeyStore</a>. To reconnect the custom key store after the update completes, use <a>ConnectCustomKeyStore</a>. To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>Use the parameters of <code>UpdateCustomKeyStore</code> to edit your keystore settings.</p> <ul> <li> <p>Use the <b>NewCustomKeyStoreName</b> parameter to change the friendly name of the custom key store to the value that you specify.</p> <p> </p> </li> <li> <p>Use the <b>KeyStorePassword</b> parameter tell AWS KMS the current password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU)</a> in the associated AWS CloudHSM cluster. You can use this parameter to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-password">fix connection failures</a> that occur when AWS KMS cannot log into the associated cluster because the <code>kmsuser</code> password has changed. This value does not change the password in the AWS CloudHSM cluster.</p> <p> </p> </li> <li> <p>Use the <b>CloudHsmClusterId</b> parameter to associate the custom key store with a different, but related, AWS CloudHSM cluster. You can use this parameter to repair a custom key store if its AWS CloudHSM cluster becomes corrupted or is deleted, or when you need to create or restore a cluster from a backup. </p> </li> </ul> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn update_custom_key_store(
        &self,
        input: UpdateCustomKeyStoreRequest,
    ) -> RusotoFuture<UpdateCustomKeyStoreResponse, UpdateCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                response
                    .buffer()
                    .map_err(|e| UpdateCustomKeyStoreError::from(e))
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<UpdateCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                proto::json::ResponsePayload::new(&response)
                                    .deserialize::<UpdateCustomKeyStoreResponse, _>()
                            })
                    })
                    .boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<UpdateCustomKeyStoreError>
                            })
                            .and_then(|response| {
                                Err(UpdateCustomKeyStoreError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }

    /// <p>Updates the description of a customer master key (CMK). To see the description of a CMK, use <a>DescribeKey</a>. </p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <p>The result of this operation varies with the key state of the CMK. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn update_key_description(
        &self,
        input: UpdateKeyDescriptionRequest,
    ) -> RusotoFuture<(), UpdateKeyDescriptionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateKeyDescription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                futures::future::ready(Ok(std::mem::drop(response))).boxed()
            } else {
                response
                    .buffer()
                    .map(|try_response| {
                        try_response
                            .map_err(|e| {
                                RusotoError::HttpDispatch(e)
                                    as RusotoError<UpdateKeyDescriptionError>
                            })
                            .and_then(|response| {
                                Err(UpdateKeyDescriptionError::from_response(response))
                            })
                    })
                    .boxed()
            }
        })
    }
}
