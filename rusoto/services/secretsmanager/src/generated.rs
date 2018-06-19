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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelRotateSecretRequest {
    /// <p>Specifies the secret for which you want to cancel a rotation request. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelRotateSecretResponse {
    /// <p>The ARN of the secret for which rotation was canceled.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the secret for which rotation was canceled.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier of the version of the secret that was created during the rotation. This version might not be complete, and should be evaluated for possible deletion. At the very least, you should remove the <code>VersionStage</code> value <code>AWSPENDING</code> to enable this version to be deleted. Failing to clean up a cancelled rotation can block you from successfully starting future rotations.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSecretRequest {
    /// <p>(Optional) If you include <code>SecretString</code> or <code>SecretBinary</code>, then an initial version is created as part of the secret, and this parameter specifies a unique identifier for the new version. </p> <note> <p>If you use the AWS CLI or one of the AWS SDK to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes as the value for this parameter in the request. If you don't use the SDK and instead generate a raw HTTP request to the AWS Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> yourself for the new version and include that value in the request.</p> </note> <p>This value helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during a rotation. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness of your versions within the specified secret. </p> <ul> <li> <p>If the <code>ClientRequestToken</code> value isn't already associated with a version of the secret then a new version of the secret is created. </p> </li> <li> <p>If a version with this value already exists and that version's <code>SecretString</code> and <code>SecretBinary</code> values are the same as those in the request, then the request is ignored (the operation is idempotent).</p> </li> <li> <p>If a version with this value already exists and that version's <code>SecretString</code> and <code>SecretBinary</code> values are different from those in the request then the request fails because you cannot modify an existing version. Instead, use <a>PutSecretValue</a> to create a new version.</p> </li> </ul> <p>This value becomes the <code>SecretVersionId</code> of the new version.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>(Optional) Specifies a user-provided description of the secret.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>(Optional) Specifies the ARN or alias of the AWS KMS customer master key (CMK) to be used to encrypt the <code>SecretString</code> and <code>SecretBinary</code> values in the versions stored in this secret.</p> <p>If you don&#39;t specify this value, then Secrets Manager defaults to using the AWS account&#39;s default CMK (the one named <code>aws/secretsmanager</code>). If a KMS CMK with that name doesn&#39;t yet exist, then AWS Secrets Manager creates it for you automatically the first time it needs to encrypt a version&#39;s <code>SecretString</code> or <code>SecretBinary</code> fields.</p> <important> <p>You can use the account&#39;s default CMK to encrypt and decrypt only if you call this operation using credentials from the same account that owns the secret. If the secret is in a different account, then you must create a custom CMK and specify the ARN in this field. </p> </important></p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Specifies the friendly name of the new secret. The secret name can consist of uppercase letters, lowercase letters, digits, and any of the following characters: /_+=.@-    Spaces are not permitted.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>(Optional) Specifies binary data that you want to encrypt and store in the new version of the secret. To use this parameter in the command-line tools, we recommend that you store your binary data in a file and then use the appropriate technique for your tool to pass the contents of the file as a parameter.</p> <p>Either <code>SecretString</code>, <code>SecretBinary</code>, or both must have a value. They cannot both be empty.</p> <p>This <code>SecretBinary</code> value is stored separately from the <code>SecretString</code>, but the two parameters jointly share a maximum size limit.</p> <p>This parameter is not available using the Secrets Manager console. It can be accessed only by using the AWS CLI or one of the AWS SDKs.</p>
    #[serde(rename = "SecretBinary")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub secret_binary: Option<Vec<u8>>,
    /// <p>(Optional) Specifies text data that you want to encrypt and store in this new version of the secret.</p> <p>Either <code>SecretString</code>, <code>SecretBinary</code>, or both must have a value. They cannot both be empty.</p> <p>This string value is stored separately from the <code>SecretBinary</code>, but the two parameters jointly share a maximum size limit.</p> <p>If you create a secret by using the Secrets Manager console then Secrets Manager puts the protected secret text in only the <code>SecretString</code> parameter. The Secrets Manager console stores the information as a JSON structure of key/value pairs that the Lambda rotation function knows how to parse.</p> <p>For storing multiple values, we recommend that you use a JSON text string argument and specify key/value pairs. For information on how to format a JSON parameter for the various command line tool environments, see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for Parameters</a> in the <i>AWS CLI User Guide</i>. For example:</p> <p> <code>[{"Key":"username","Value":"bob"},{"Key":"password","Value":"abc123xyz456"}]</code> </p> <p>If your command-line tool or SDK requires quotation marks around the parameter, you should use single quotes to avoid confusion with the double quotes required in the JSON text. </p>
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    /// <p><p>(Optional) Specifies a list of user-defined tags that are attached to the secret. Each tag is a &quot;Key&quot; and &quot;Value&quot; pair of strings. This operation only appends tags to the existing list of tags. To remove tags, you must use <a>UntagResource</a>.</p> <important> <ul> <li> <p>AWS Secrets Manager tag key names are case sensitive. A tag with the key &quot;ABC&quot; is a different tag from one with key &quot;abc&quot;.</p> </li> <li> <p>If you check tags in IAM policy <code>Condition</code> elements as part of your security strategy, then adding or removing a tag can change permissions. If the successful completion of this operation would result in you losing your permissions for this secret, then this operation is blocked and returns an <code>Access Denied</code> error.</p> </li> </ul> </important> <p>This parameter requires a JSON text string argument. For information on how to format a JSON parameter for the various command line tool environments, see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for Parameters</a> in the <i>AWS CLI User Guide</i>. For example:</p> <p> <code>[{&quot;Key&quot;:&quot;CostCenter&quot;,&quot;Value&quot;:&quot;12345&quot;},{&quot;Key&quot;:&quot;environment&quot;,&quot;Value&quot;:&quot;production&quot;}]</code> </p> <p>If your command-line tool or SDK requires quotation marks around the parameter, you should use single quotes to avoid confusion with the double quotes required in the JSON text. </p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per secret—50</p> </li> <li> <p>Maximum key length—127 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length—255 Unicode characters in UTF-8</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use the <code>aws:</code> prefix in your tag names or values because it is reserved for AWS use. You can&#39;t edit or delete tag names or values with this prefix. Tags with this prefix do not count against your tags per secret limit.</p> </li> <li> <p>If your tagging schema will be used across multiple services and resources, remember that other services might have restrictions on allowed characters. Generally allowed characters are: letters, spaces, and numbers representable in UTF-8, plus the following special characters: + - = . _ : / @.</p> </li> </ul></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSecretResponse {
    /// <p><p>The Amazon Resource Name (ARN) of the secret that you just created.</p> <note> <p>AWS Secrets Manager automatically adds several random characters to the name at the end of the ARN when you initially create a secret. This affects only the ARN and not the actual friendly name. This ensures that if you create a new secret with the same name as an old secret that you previously deleted, then users with access to the old secret <i>don&#39;t</i> automatically get access to the new secret because the ARNs are different.</p> </note></p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the secret that you just created.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier that's associated with the version of the secret you just created.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSecretRequest {
    /// <p>(Optional) Specifies the number of days that AWS Secrets Manager waits before it can delete the secret.</p> <p>This value can range from 7 to 30 days. The default value is 30.</p>
    #[serde(rename = "RecoveryWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window_in_days: Option<i64>,
    /// <p>Specifies the secret that you want to delete. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteSecretResponse {
    /// <p>The ARN of the secret that is now scheduled for deletion.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time after which this secret will be deleted by AWS Secrets Manager and is no longer recoverable. This value is the date and time of the delete request plus the number of days specified in <code>RecoveryWindowInDays</code>.</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The friendly name of the secret that is now scheduled for deletion.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSecretRequest {
    /// <p>The identifier of the secret whose details you want to retrieve. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSecretResponse {
    /// <p>The ARN of the secret.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>This value exists if the secret is scheduled for deletion. Some time after the specified date and time, Secrets Manager deletes the secret and all of its versions.</p> <p>If a secret is scheduled for deletion, then its details, including the encrypted secret information, is not accessible. To cancel a scheduled deletion and restore access, use <a>RestoreSecret</a>.</p>
    #[serde(rename = "DeletedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<f64>,
    /// <p>The user-provided description of the secret.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN or alias of the AWS KMS customer master key (CMK) that's used to encrypt the <code>SecretString</code> and <code>SecretBinary</code> fields in each version of the secret. If you don't provide a key, then AWS Secrets Manager defaults to encrypting the secret fields with the default KMS CMK (the one named <code>awssecretsmanager</code>) for this account.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The last date that this secret was accessed. This value is truncated to midnight of the date and therefore shows only the date, not the time.</p>
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<f64>,
    /// <p>The last date and time that this secret was modified in any way.</p>
    #[serde(rename = "LastChangedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_date: Option<f64>,
    /// <p>The last date and time that the Secrets Manager rotation process for this secret was invoked.</p>
    #[serde(rename = "LastRotatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotated_date: Option<f64>,
    /// <p>The user-provided friendly name of the secret.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies whether automatic rotation is enabled for this secret.</p> <p>To enable rotation, use <a>RotateSecret</a> with <code>AutomaticallyRotateAfterDays</code> set to a value greater than 0. To disable rotation, use <a>CancelRotateSecret</a>.</p>
    #[serde(rename = "RotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<bool>,
    /// <p>The ARN of a Lambda function that's invoked by AWS Secrets Manager to rotate the secret either automatically per the schedule or manually by a call to <code>RotateSecret</code>.</p>
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<String>,
    /// <p>A structure that contains the rotation configuration for this secret.</p>
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    /// <p>The list of user-defined tags that are associated with the secret. To add tags to a secret, use <a>TagResource</a>. To remove tags, use <a>UntagResource</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>A list of all of the currently assigned <code>VersionStage</code> staging labels and the <code>SecretVersionId</code> that each is attached to. Staging labels are used to keep track of the different versions during the rotation process.</p> <note> <p>A version that does not have any staging labels attached is considered deprecated and subject to deletion. Such versions are not included in this list.</p> </note></p>
    #[serde(rename = "VersionIdsToStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_ids_to_stages: Option<::std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRandomPasswordRequest {
    /// <p>A string that includes characters that should not be included in the generated password. The default is that all characters from the included sets can be used.</p>
    #[serde(rename = "ExcludeCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_characters: Option<String>,
    /// <p>Specifies that the generated password should not include lowercase letters. The default if you do not include this switch parameter is that lowercase letters can be included.</p>
    #[serde(rename = "ExcludeLowercase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_lowercase: Option<bool>,
    /// <p>Specifies that the generated password should not include digits. The default if you do not include this switch parameter is that digits can be included.</p>
    #[serde(rename = "ExcludeNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_numbers: Option<bool>,
    /// <p>Specifies that the generated password should not include punctuation characters. The default if you do not include this switch parameter is that punctuation characters can be included.</p>
    #[serde(rename = "ExcludePunctuation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_punctuation: Option<bool>,
    /// <p>Specifies that the generated password should not include uppercase letters. The default if you do not include this switch parameter is that uppercase letters can be included.</p>
    #[serde(rename = "ExcludeUppercase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uppercase: Option<bool>,
    /// <p>Specifies that the generated password can include the space character. The default if you do not include this switch parameter is that the space character is not included.</p>
    #[serde(rename = "IncludeSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_space: Option<bool>,
    /// <p>The desired length of the generated password. The default value if you do not include this parameter is 32 characters.</p>
    #[serde(rename = "PasswordLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_length: Option<i64>,
    /// <p>A boolean value that specifies whether the generated password must include at least one of every allowed character type. The default value is <code>True</code> and the operation requires at least one of every character type.</p>
    #[serde(rename = "RequireEachIncludedType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_each_included_type: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetRandomPasswordResponse {
    /// <p>A string with the generated password.</p>
    #[serde(rename = "RandomPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_password: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSecretValueRequest {
    /// <p>Specifies the secret containing the version that you want to retrieve. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    /// <p>Specifies the unique identifier of the version of the secret that you want to retrieve. If you specify this parameter then don't specify <code>VersionStage</code>. If you don't specify either a <code>VersionStage</code> or <code>SecretVersionId</code> then the default is to perform the operation on the version with the <code>VersionStage</code> value of <code>AWSCURRENT</code>.</p> <p>This value is typically a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value with 32 hexadecimal digits.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// <p>Specifies the secret version that you want to retrieve by the staging label attached to the version.</p> <p>Staging labels are used to keep track of different versions during the rotation process. If you use this parameter then don't specify <code>SecretVersionId</code>. If you don't specify either a <code>VersionStage</code> or <code>SecretVersionId</code>, then the default is to perform the operation on the version with the <code>VersionStage</code> value of <code>AWSCURRENT</code>.</p>
    #[serde(rename = "VersionStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stage: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSecretValueResponse {
    /// <p>The ARN of the secret.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that this version of the secret was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The friendly name of the secret.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The decrypted part of the protected secret information that was originally provided as binary data in the form of a byte array. The response parameter represents the binary data as a <a href="https://tools.ietf.org/html/rfc4648#section-4">base64-encoded</a> string.</p> <p>This parameter is not used if the secret is created by the Secrets Manager console.</p> <p>If you store custom information in this field of the secret, then you must code your Lambda rotation function to parse and interpret whatever you store in the <code>SecretString</code> or <code>SecretBinary</code> fields.</p>
    #[serde(rename = "SecretBinary")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub secret_binary: Option<Vec<u8>>,
    /// <p>The decrypted part of the protected secret information that was originally provided as a string.</p> <p>If you create this secret by using the Secrets Manager console then only the <code>SecretString</code> parameter contains data. Secrets Manager stores the information as a JSON structure of key/value pairs that the Lambda rotation function knows how to parse.</p> <p>If you store custom information in the secret by using the <a>CreateSecret</a>, <a>UpdateSecret</a>, or <a>PutSecretValue</a> API operations instead of the AWS Secrets Manager console, or by using the <b>Other secret type</b> in the console, then you must code your Lambda rotation function to parse and interpret those values.</p>
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    /// <p>The unique identifier of this version of the secret.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// <p>A list of all of the staging labels currently attached to this version of the secret.</p>
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSecretVersionIdsRequest {
    /// <p>(Optional) Specifies that you want the results to include versions that do not have any staging labels attached to them. Such versions are considered deprecated and are subject to deletion by Secrets Manager as needed.</p>
    #[serde(rename = "IncludeDeprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deprecated: Option<bool>,
    /// <p>(Optional) Limits the number of results that you want to include in the response. If you don't include this parameter, it defaults to a value that's specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that AWS Secrets Manager might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier for the secret containing the versions you want to list. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListSecretVersionIdsResponse {
    /// <p><p>The Amazon Resource Name (ARN) for the secret.</p> <note> <p>AWS Secrets Manager automatically adds several random characters to the name at the end of the ARN when you initially create a secret. This affects only the ARN and not the actual friendly name. This ensures that if you create a new secret with the same name as an old secret that you previously deleted, then users with access to the old secret <i>don&#39;t</i> automatically get access to the new secret because the ARNs are different.</p> </note></p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the secret.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If present in the response, this value indicates that there's more output available than what's included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a very long list. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to continue processing and get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back empty (as <code>null</code>).</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of the currently available versions of the specified secret.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<SecretVersionsListEntry>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSecretsRequest {
    /// <p>(Optional) Limits the number of results that you want to include in the response. If you don't include this parameter, it defaults to a value that's specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (isn't null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that AWS Secrets Manager might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>(Optional) Use this parameter in a request if you receive a <code>NextToken</code> response in a previous request that indicates that there's more output available. In a subsequent call, set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListSecretsResponse {
    /// <p>If present in the response, this value indicates that there's more output available than what's included in the current response. This can occur even when the response includes no values at all, such as when you ask for a filtered view of a very long list. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to continue processing and get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back empty (as <code>null</code>).</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the secrets in the account.</p>
    #[serde(rename = "SecretList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_list: Option<Vec<SecretListEntry>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutSecretValueRequest {
    /// <p>(Optional) Specifies a unique identifier for the new version of the secret. </p> <note> <p>If you use the AWS CLI or one of the AWS SDK to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes that in the request. If you don't use the SDK and instead generate a raw HTTP request to the AWS Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> yourself for new versions and include that value in the request. </p> </note> <p>This value helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during the Lambda rotation function's processing. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness within the specified secret. </p> <ul> <li> <p>If the <code>ClientRequestToken</code> value isn't already associated with a version of the secret then a new version of the secret is created. </p> </li> <li> <p>If a version with this value already exists and that version's <code>SecretString</code> or <code>SecretBinary</code> values are the same as those in the request then the request is ignored (the operation is idempotent). </p> </li> <li> <p>If a version with this value already exists and that version's <code>SecretString</code> and <code>SecretBinary</code> values are different from those in the request then the request fails because you cannot modify an existing secret version. You can only create new versions to store new secret values.</p> </li> </ul> <p>This value becomes the <code>SecretVersionId</code> of the new version.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p><p>(Optional) Specifies binary data that you want to encrypt and store in the new version of the secret. To use this parameter in the command-line tools, we recommend that you store your binary data in a file and then use the appropriate technique for your tool to pass the contents of the file as a parameter. Either <code>SecretBinary</code> or <code>SecretString</code> must have a value. They cannot both be empty.</p> <p>This parameter is not accessible if the secret using the Secrets Manager console.</p> <p/></p>
    #[serde(rename = "SecretBinary")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub secret_binary: Option<Vec<u8>>,
    /// <p>Specifies the secret to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret. The secret must already exist.</p> <p>The secret name can consist of uppercase letters, lowercase letters, digits, and any of the following characters: /_+=.@-    Spaces are not permitted.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    /// <p>(Optional) Specifies text data that you want to encrypt and store in this new version of the secret. Either <code>SecretString</code> or <code>SecretBinary</code> must have a value. They cannot both be empty.</p> <p>If you create this secret by using the Secrets Manager console then Secrets Manager puts the protected secret text in only the <code>SecretString</code> parameter. The Secrets Manager console stores the information as a JSON structure of key/value pairs that the default Lambda rotation function knows how to parse.</p> <p>For storing multiple values, we recommend that you use a JSON text string argument and specify key/value pairs. For information on how to format a JSON parameter for the various command line tool environments, see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for Parameters</a> in the <i>AWS CLI User Guide</i>.</p>
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    /// <p>(Optional) Specifies a list of staging labels that are attached to this version of the secret. These staging labels are used to track the versions through the rotation process by the Lambda rotation function.</p> <p>A staging label must be unique to a single version of the secret. If you specify a staging label that's already associated with a different version of the same secret then that staging label is automatically removed from the other version and attached to this version.</p> <p>If you do not specify a value for <code>VersionStages</code> then AWS Secrets Manager automatically moves the staging label <code>AWSCURRENT</code> to this new version.</p>
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutSecretValueResponse {
    /// <p>The Amazon Resource Name (ARN) for the secret for which you just created a version.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the secret for which you just created or updated a version.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier of the version of the secret you just created or updated.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// <p>The list of staging labels that are currently attached to this version of the secret. Staging labels are used to track a version as it progresses through the secret rotation process.</p>
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RestoreSecretRequest {
    /// <p>Specifies the secret that you want to restore from a previously scheduled deletion. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RestoreSecretResponse {
    /// <p>The ARN of the secret that was restored.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the secret that was restored.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RotateSecretRequest {
    /// <p>(Optional) Specifies a unique identifier for the new version of the secret that helps ensure idempotency. </p> <p>If you use the AWS CLI or one of the AWS SDK to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes that in the request for this parameter. If you don't use the SDK and instead generate a raw HTTP request to the AWS Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> yourself for new versions and include that value in the request.</p> <p>You only need to specify your own value if you are implementing your own retry logic and want to ensure that a given secret is not created twice. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness within the specified secret. </p> <p>Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during the function's processing.</p> <ul> <li> <p>If the <code>ClientRequestToken</code> value isn't already associated with a version of the secret then a new version of the secret is created. </p> </li> <li> <p>If a version with this value already exists and that version's <code>SecretString</code> and <code>SecretBinary</code> values are the same as the request, then the request is ignored (the operation is idempotent). </p> </li> <li> <p>If a version with this value already exists and that version's <code>SecretString</code> and <code>SecretBinary</code> values are different from the request then an error occurs because you cannot modify an existing secret value.</p> </li> </ul> <p>This value becomes the <code>SecretVersionId</code> of the new version.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>(Optional) Specifies the ARN of the Lambda function that can rotate the secret.</p>
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<String>,
    /// <p>A structure that defines the rotation configuration for this secret.</p>
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    /// <p>Specifies the secret that you want to rotate. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RotateSecretResponse {
    /// <p>The ARN of the secret.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the secret.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the new version of the secret created by the rotation started by this request.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>A structure that defines the rotation configuration for the secret.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RotationRulesType {
    /// <p>Specifies the number of days between automatic scheduled rotations of the secret.</p>
    #[serde(rename = "AutomaticallyAfterDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_after_days: Option<i64>,
}

/// <p>A structure that contains the details about a secret. It does not include the encrypted <code>SecretString</code> and <code>SecretBinary</code> values. To get those values, use the <a>GetSecretValue</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SecretListEntry {
    /// <p>The Amazon Resource Name (ARN) of the secret.</p> <p>For more information about ARNs in AWS Secrets Manager, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#iam-resources">Policy Resources</a> in the <i>AWS Secrets Manager User Guide</i>.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time on which this secret was deleted. Not present on active secrets. The secret can be recovered until the number of days in the recovery window has passed, as specified in the <code>RecoveryWindowInDays</code> parameter of the <a>DeleteSecret</a> operation.</p>
    #[serde(rename = "DeletedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<f64>,
    /// <p>The user-provided description of the secret.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN or alias of the AWS KMS customer master key (CMK) that's used to encrypt the <code>SecretString</code> and <code>SecretBinary</code> fields in each version of the secret. If you don't provide a key, then AWS Secrets Manager defaults to encrypting the secret fields with the default KMS CMK (the one named <code>awssecretsmanager</code>) for this account.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The last date that this secret was accessed. This value is truncated to midnight of the date and therefore shows only the date, not the time.</p>
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<f64>,
    /// <p>The last date and time that this secret was modified in any way.</p>
    #[serde(rename = "LastChangedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_date: Option<f64>,
    /// <p>The last date and time that the rotation process for this secret was invoked.</p>
    #[serde(rename = "LastRotatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotated_date: Option<f64>,
    /// <p>The friendly name of the secret. You can use forward slashes in the name to represent a path hierarchy. For example, <code>/prod/databases/dbserver1</code> could represent the secret for a server named <code>dbserver1</code> in the folder <code>databases</code> in the folder <code>prod</code>. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Indicated whether automatic, scheduled rotation is enabled for this secret.</p>
    #[serde(rename = "RotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<bool>,
    /// <p>The ARN of an AWS Lambda function that's invoked by AWS Secrets Manager to rotate and expire the secret either automatically per the schedule or manually by a call to <a>RotateSecret</a>.</p>
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<String>,
    /// <p>A structure that defines the rotation configuration for the secret.</p>
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    /// <p><p>A list of all of the currently assigned <code>SecretVersionStage</code> staging labels and the <code>SecretVersionId</code> that each is attached to. Staging labels are used to keep track of the different versions during the rotation process.</p> <note> <p>A version that does not have any <code>SecretVersionStage</code> is considered deprecated and subject to deletion. Such versions are not included in this list.</p> </note></p>
    #[serde(rename = "SecretVersionsToStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_versions_to_stages: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The list of user-defined tags that are associated with the secret. To add tags to a secret, use <a>TagResource</a>. To remove tags, use <a>UntagResource</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>A structure that contains information about one version of a secret.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SecretVersionsListEntry {
    /// <p>The date and time this version of the secret was created.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The date that this version of the secret was last accessed. Note that the resolution of this field is at the date level and does not include the time.</p>
    #[serde(rename = "LastAccessedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<f64>,
    /// <p>The unique version identifier of this version of the secret.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// <p>An array of staging labels that are currently associated with this version of the secret.</p>
    #[serde(rename = "VersionStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

/// <p>A structure that contains information about a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key identifier, or name, of the tag.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The string value that's associated with the key of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The identifier for the secret that you want to attach tags to. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    /// <p>The tags to attach to the secret. Each element in the list consists of a <code>Key</code> and a <code>Value</code>.</p> <p>This parameter to the API requires a JSON text string argument. For information on how to format a JSON parameter for the various command line tool environments, see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for Parameters</a> in the <i>AWS CLI User Guide</i>. For the AWS CLI, you can also use the syntax: <code>--Tags Key="Key1",Value="Value1",Key="Key2",Value="Value2"[,…]</code> </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The identifier for the secret that you want to remove tags from. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    /// <p>A list of tag key names to remove from the secret. You don't specify the value. Both the key and its associated value are removed.</p> <p>This parameter to the API requires a JSON text string argument. For information on how to format a JSON parameter for the various command line tool environments, see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for Parameters</a> in the <i>AWS CLI User Guide</i>.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSecretRequest {
    /// <p>(Optional) If you want to add a new version to the secret, this parameter specifies a unique identifier for the new version that helps ensure idempotency. </p> <p>If you use the AWS CLI or one of the AWS SDK to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes that in the request. If you don't use the SDK and instead generate a raw HTTP request to the AWS Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> yourself for new versions and include that value in the request.</p> <p>You typically only need to interact with this value if you implement your own retry logic and want to ensure that a given secret is not created twice. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness within the specified secret. </p> <p>Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during the Lambda rotation function's processing.</p> <ul> <li> <p>If the <code>ClientRequestToken</code> value isn't already associated with a version of the secret then a new version of the secret is created. </p> </li> <li> <p>If a version with this value already exists and that version's <code>SecretString</code> and <code>SecretBinary</code> values are the same as those in the request then the request is ignored (the operation is idempotent). </p> </li> <li> <p>If a version with this value already exists and that version's <code>SecretString</code> and <code>SecretBinary</code> values are different from the request then an error occurs because you cannot modify an existing secret value.</p> </li> </ul> <p>This value becomes the <code>SecretVersionId</code> of the new version.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>(Optional) Specifies a user-provided description of the secret.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>(Optional) Specifies the ARN or alias of the KMS customer master key (CMK) to be used to encrypt the protected text in the versions of this secret.</p> <p>If you don&#39;t specify this value, then Secrets Manager defaults to using the default CMK in the account (the one named <code>aws/secretsmanager</code>). If a KMS CMK with that name doesn&#39;t exist, then AWS Secrets Manager creates it for you automatically the first time it needs to encrypt a version&#39;s <code>Plaintext</code> or <code>PlaintextString</code> fields.</p> <important> <p>You can only use the account&#39;s default CMK to encrypt and decrypt if you call this operation using credentials from the same account that owns the secret. If the secret is in a different account, then you must create a custom CMK and provide the ARN in this field. </p> </important></p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>(Optional) Specifies binary data that you want to encrypt and store in the new version of the secret. To use this parameter in the command-line tools, we recommend that you store your binary data in a file and then use the appropriate technique for your tool to pass the contents of the file as a parameter. Either <code>SecretBinary</code> or <code>SecretString</code> must have a value. They cannot both be empty.</p> <p>This parameter is not accessible using the Secrets Manager console.</p>
    #[serde(rename = "SecretBinary")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub secret_binary: Option<Vec<u8>>,
    /// <p>Specifies the secret that you want to update or to which you want to add a new version. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    /// <p>(Optional) Specifies text data that you want to encrypt and store in this new version of the secret. Either <code>SecretBinary</code> or <code>SecretString</code> must have a value. They cannot both be empty.</p> <p>If you create this secret by using the Secrets Manager console then Secrets Manager puts the protected secret text in only the <code>SecretString</code> parameter. The Secrets Manager console stores the information as a JSON structure of key/value pairs that the default Lambda rotation function knows how to parse.</p> <p>For storing multiple values, we recommend that you use a JSON text string argument and specify key/value pairs. For information on how to format a JSON parameter for the various command line tool environments, see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for Parameters</a> in the <i>AWS CLI User Guide</i>.</p>
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateSecretResponse {
    /// <p><p>The ARN of this secret.</p> <note> <p>AWS Secrets Manager automatically adds several random characters to the name at the end of the ARN when you initially create a secret. This affects only the ARN and not the actual friendly name. This ensures that if you create a new secret with the same name as an old secret that you previously deleted, then users with access to the old secret <i>don&#39;t</i> automatically get access to the new secret because the ARNs are different.</p> </note></p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of this secret.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If a version of the secret was created or updated by this operation, then its unique identifier is returned.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateSecretVersionStageRequest {
    /// <p>(Optional) The secret version ID that you want to add the staging labels to.</p> <p>If any of the staging labels are already attached to a different version of the secret, then they are removed from that version before adding them to this version.</p>
    #[serde(rename = "MoveToVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_version_id: Option<String>,
    /// <p>(Optional) Specifies the secret version ID of the version that the staging labels are to be removed from.</p> <p>If you want to move a label to a new version, you do not have to explicitly remove it with this parameter. Adding a label using the <code>MoveToVersionId</code> parameter automatically removes it from the old version. However, if you do include both the "MoveTo" and "RemoveFrom" parameters, then the move is successful only if the staging labels are actually present on the "RemoveFrom" version. If a staging label was on a different version than "RemoveFrom", then the request fails.</p>
    #[serde(rename = "RemoveFromVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_from_version_id: Option<String>,
    /// <p>Specifies the secret with the version whose list of staging labels you want to modify. You can specify either the Amazon Resource Name (ARN) or the friendly name of the secret.</p>
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    /// <p>The list of staging labels to add to this version.</p>
    #[serde(rename = "VersionStage")]
    pub version_stage: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateSecretVersionStageResponse {
    /// <p>The ARN of the secret with the staging labels that were modified.</p>
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the secret with the staging labels that were modified.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Errors returned by CancelRotateSecret
#[derive(Debug, PartialEq)]
pub enum CancelRotateSecretError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelRotateSecretError {
    pub fn from_body(body: &str) -> CancelRotateSecretError {
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
                    "InternalServiceError" => {
                        CancelRotateSecretError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CancelRotateSecretError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CancelRotateSecretError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CancelRotateSecretError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelRotateSecretError::Validation(error_message.to_string())
                    }
                    _ => CancelRotateSecretError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelRotateSecretError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelRotateSecretError {
    fn from(err: serde_json::error::Error) -> CancelRotateSecretError {
        CancelRotateSecretError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelRotateSecretError {
    fn from(err: CredentialsError) -> CancelRotateSecretError {
        CancelRotateSecretError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelRotateSecretError {
    fn from(err: HttpDispatchError) -> CancelRotateSecretError {
        CancelRotateSecretError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelRotateSecretError {
    fn from(err: io::Error) -> CancelRotateSecretError {
        CancelRotateSecretError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelRotateSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelRotateSecretError {
    fn description(&self) -> &str {
        match *self {
            CancelRotateSecretError::InternalServiceError(ref cause) => cause,
            CancelRotateSecretError::InvalidParameter(ref cause) => cause,
            CancelRotateSecretError::InvalidRequest(ref cause) => cause,
            CancelRotateSecretError::ResourceNotFound(ref cause) => cause,
            CancelRotateSecretError::Validation(ref cause) => cause,
            CancelRotateSecretError::Credentials(ref err) => err.description(),
            CancelRotateSecretError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CancelRotateSecretError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSecret
#[derive(Debug, PartialEq)]
pub enum CreateSecretError {
    /// <p>AWS Secrets Manager can't encrypt the protected secret text using the provided KMS key. Check that the customer master key (CMK) is available, enabled, and not in an invalid state. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a>.</p>
    EncryptionFailure(String),
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>The request failed because it would exceed one of the AWS Secrets Manager internal limits.</p>
    LimitExceeded(String),
    /// <p>The policy document that you provided isn't valid.</p>
    MalformedPolicyDocument(String),
    /// <p>A resource with the ID you requested already exists.</p>
    ResourceExists(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSecretError {
    pub fn from_body(body: &str) -> CreateSecretError {
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
                    "EncryptionFailure" => {
                        CreateSecretError::EncryptionFailure(String::from(error_message))
                    }
                    "InternalServiceError" => {
                        CreateSecretError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateSecretError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateSecretError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateSecretError::LimitExceeded(String::from(error_message))
                    }
                    "MalformedPolicyDocumentException" => {
                        CreateSecretError::MalformedPolicyDocument(String::from(error_message))
                    }
                    "ResourceExistsException" => {
                        CreateSecretError::ResourceExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateSecretError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSecretError::Validation(error_message.to_string())
                    }
                    _ => CreateSecretError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSecretError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSecretError {
    fn from(err: serde_json::error::Error) -> CreateSecretError {
        CreateSecretError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSecretError {
    fn from(err: CredentialsError) -> CreateSecretError {
        CreateSecretError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSecretError {
    fn from(err: HttpDispatchError) -> CreateSecretError {
        CreateSecretError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSecretError {
    fn from(err: io::Error) -> CreateSecretError {
        CreateSecretError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSecretError {
    fn description(&self) -> &str {
        match *self {
            CreateSecretError::EncryptionFailure(ref cause) => cause,
            CreateSecretError::InternalServiceError(ref cause) => cause,
            CreateSecretError::InvalidParameter(ref cause) => cause,
            CreateSecretError::InvalidRequest(ref cause) => cause,
            CreateSecretError::LimitExceeded(ref cause) => cause,
            CreateSecretError::MalformedPolicyDocument(ref cause) => cause,
            CreateSecretError::ResourceExists(ref cause) => cause,
            CreateSecretError::ResourceNotFound(ref cause) => cause,
            CreateSecretError::Validation(ref cause) => cause,
            CreateSecretError::Credentials(ref err) => err.description(),
            CreateSecretError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSecretError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSecret
#[derive(Debug, PartialEq)]
pub enum DeleteSecretError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSecretError {
    pub fn from_body(body: &str) -> DeleteSecretError {
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
                    "InternalServiceError" => {
                        DeleteSecretError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteSecretError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteSecretError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteSecretError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSecretError::Validation(error_message.to_string())
                    }
                    _ => DeleteSecretError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSecretError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSecretError {
    fn from(err: serde_json::error::Error) -> DeleteSecretError {
        DeleteSecretError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSecretError {
    fn from(err: CredentialsError) -> DeleteSecretError {
        DeleteSecretError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSecretError {
    fn from(err: HttpDispatchError) -> DeleteSecretError {
        DeleteSecretError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSecretError {
    fn from(err: io::Error) -> DeleteSecretError {
        DeleteSecretError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSecretError {
    fn description(&self) -> &str {
        match *self {
            DeleteSecretError::InternalServiceError(ref cause) => cause,
            DeleteSecretError::InvalidParameter(ref cause) => cause,
            DeleteSecretError::InvalidRequest(ref cause) => cause,
            DeleteSecretError::ResourceNotFound(ref cause) => cause,
            DeleteSecretError::Validation(ref cause) => cause,
            DeleteSecretError::Credentials(ref err) => err.description(),
            DeleteSecretError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSecretError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSecret
#[derive(Debug, PartialEq)]
pub enum DescribeSecretError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSecretError {
    pub fn from_body(body: &str) -> DescribeSecretError {
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
                    "InternalServiceError" => {
                        DescribeSecretError::InternalServiceError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeSecretError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeSecretError::Validation(error_message.to_string())
                    }
                    _ => DescribeSecretError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSecretError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSecretError {
    fn from(err: serde_json::error::Error) -> DescribeSecretError {
        DescribeSecretError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSecretError {
    fn from(err: CredentialsError) -> DescribeSecretError {
        DescribeSecretError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSecretError {
    fn from(err: HttpDispatchError) -> DescribeSecretError {
        DescribeSecretError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSecretError {
    fn from(err: io::Error) -> DescribeSecretError {
        DescribeSecretError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSecretError {
    fn description(&self) -> &str {
        match *self {
            DescribeSecretError::InternalServiceError(ref cause) => cause,
            DescribeSecretError::ResourceNotFound(ref cause) => cause,
            DescribeSecretError::Validation(ref cause) => cause,
            DescribeSecretError::Credentials(ref err) => err.description(),
            DescribeSecretError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeSecretError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRandomPassword
#[derive(Debug, PartialEq)]
pub enum GetRandomPasswordError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRandomPasswordError {
    pub fn from_body(body: &str) -> GetRandomPasswordError {
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
                    "InternalServiceError" => {
                        GetRandomPasswordError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetRandomPasswordError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetRandomPasswordError::InvalidRequest(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRandomPasswordError::Validation(error_message.to_string())
                    }
                    _ => GetRandomPasswordError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRandomPasswordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRandomPasswordError {
    fn from(err: serde_json::error::Error) -> GetRandomPasswordError {
        GetRandomPasswordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRandomPasswordError {
    fn from(err: CredentialsError) -> GetRandomPasswordError {
        GetRandomPasswordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRandomPasswordError {
    fn from(err: HttpDispatchError) -> GetRandomPasswordError {
        GetRandomPasswordError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRandomPasswordError {
    fn from(err: io::Error) -> GetRandomPasswordError {
        GetRandomPasswordError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRandomPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRandomPasswordError {
    fn description(&self) -> &str {
        match *self {
            GetRandomPasswordError::InternalServiceError(ref cause) => cause,
            GetRandomPasswordError::InvalidParameter(ref cause) => cause,
            GetRandomPasswordError::InvalidRequest(ref cause) => cause,
            GetRandomPasswordError::Validation(ref cause) => cause,
            GetRandomPasswordError::Credentials(ref err) => err.description(),
            GetRandomPasswordError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRandomPasswordError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSecretValue
#[derive(Debug, PartialEq)]
pub enum GetSecretValueError {
    /// <p>AWS Secrets Manager can't decrypt the protected secret text using the provided KMS key. </p>
    DecryptionFailure(String),
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSecretValueError {
    pub fn from_body(body: &str) -> GetSecretValueError {
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
                    "DecryptionFailure" => {
                        GetSecretValueError::DecryptionFailure(String::from(error_message))
                    }
                    "InternalServiceError" => {
                        GetSecretValueError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetSecretValueError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetSecretValueError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetSecretValueError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSecretValueError::Validation(error_message.to_string())
                    }
                    _ => GetSecretValueError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSecretValueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSecretValueError {
    fn from(err: serde_json::error::Error) -> GetSecretValueError {
        GetSecretValueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSecretValueError {
    fn from(err: CredentialsError) -> GetSecretValueError {
        GetSecretValueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSecretValueError {
    fn from(err: HttpDispatchError) -> GetSecretValueError {
        GetSecretValueError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSecretValueError {
    fn from(err: io::Error) -> GetSecretValueError {
        GetSecretValueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSecretValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSecretValueError {
    fn description(&self) -> &str {
        match *self {
            GetSecretValueError::DecryptionFailure(ref cause) => cause,
            GetSecretValueError::InternalServiceError(ref cause) => cause,
            GetSecretValueError::InvalidParameter(ref cause) => cause,
            GetSecretValueError::InvalidRequest(ref cause) => cause,
            GetSecretValueError::ResourceNotFound(ref cause) => cause,
            GetSecretValueError::Validation(ref cause) => cause,
            GetSecretValueError::Credentials(ref err) => err.description(),
            GetSecretValueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSecretValueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSecretVersionIds
#[derive(Debug, PartialEq)]
pub enum ListSecretVersionIdsError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid <code>NextToken</code> value.</p>
    InvalidNextToken(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListSecretVersionIdsError {
    pub fn from_body(body: &str) -> ListSecretVersionIdsError {
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
                    "InternalServiceError" => {
                        ListSecretVersionIdsError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListSecretVersionIdsError::InvalidNextToken(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListSecretVersionIdsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListSecretVersionIdsError::Validation(error_message.to_string())
                    }
                    _ => ListSecretVersionIdsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSecretVersionIdsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSecretVersionIdsError {
    fn from(err: serde_json::error::Error) -> ListSecretVersionIdsError {
        ListSecretVersionIdsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSecretVersionIdsError {
    fn from(err: CredentialsError) -> ListSecretVersionIdsError {
        ListSecretVersionIdsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSecretVersionIdsError {
    fn from(err: HttpDispatchError) -> ListSecretVersionIdsError {
        ListSecretVersionIdsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSecretVersionIdsError {
    fn from(err: io::Error) -> ListSecretVersionIdsError {
        ListSecretVersionIdsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSecretVersionIdsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSecretVersionIdsError {
    fn description(&self) -> &str {
        match *self {
            ListSecretVersionIdsError::InternalServiceError(ref cause) => cause,
            ListSecretVersionIdsError::InvalidNextToken(ref cause) => cause,
            ListSecretVersionIdsError::ResourceNotFound(ref cause) => cause,
            ListSecretVersionIdsError::Validation(ref cause) => cause,
            ListSecretVersionIdsError::Credentials(ref err) => err.description(),
            ListSecretVersionIdsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSecretVersionIdsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSecrets
#[derive(Debug, PartialEq)]
pub enum ListSecretsError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid <code>NextToken</code> value.</p>
    InvalidNextToken(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListSecretsError {
    pub fn from_body(body: &str) -> ListSecretsError {
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
                    "InternalServiceError" => {
                        ListSecretsError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidNextTokenException" => {
                        ListSecretsError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListSecretsError::InvalidParameter(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListSecretsError::Validation(error_message.to_string())
                    }
                    _ => ListSecretsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSecretsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSecretsError {
    fn from(err: serde_json::error::Error) -> ListSecretsError {
        ListSecretsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSecretsError {
    fn from(err: CredentialsError) -> ListSecretsError {
        ListSecretsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSecretsError {
    fn from(err: HttpDispatchError) -> ListSecretsError {
        ListSecretsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSecretsError {
    fn from(err: io::Error) -> ListSecretsError {
        ListSecretsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSecretsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSecretsError {
    fn description(&self) -> &str {
        match *self {
            ListSecretsError::InternalServiceError(ref cause) => cause,
            ListSecretsError::InvalidNextToken(ref cause) => cause,
            ListSecretsError::InvalidParameter(ref cause) => cause,
            ListSecretsError::Validation(ref cause) => cause,
            ListSecretsError::Credentials(ref err) => err.description(),
            ListSecretsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListSecretsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutSecretValue
#[derive(Debug, PartialEq)]
pub enum PutSecretValueError {
    /// <p>AWS Secrets Manager can't encrypt the protected secret text using the provided KMS key. Check that the customer master key (CMK) is available, enabled, and not in an invalid state. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a>.</p>
    EncryptionFailure(String),
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>The request failed because it would exceed one of the AWS Secrets Manager internal limits.</p>
    LimitExceeded(String),
    /// <p>A resource with the ID you requested already exists.</p>
    ResourceExists(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutSecretValueError {
    pub fn from_body(body: &str) -> PutSecretValueError {
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
                    "EncryptionFailure" => {
                        PutSecretValueError::EncryptionFailure(String::from(error_message))
                    }
                    "InternalServiceError" => {
                        PutSecretValueError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        PutSecretValueError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        PutSecretValueError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutSecretValueError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceExistsException" => {
                        PutSecretValueError::ResourceExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutSecretValueError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutSecretValueError::Validation(error_message.to_string())
                    }
                    _ => PutSecretValueError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutSecretValueError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutSecretValueError {
    fn from(err: serde_json::error::Error) -> PutSecretValueError {
        PutSecretValueError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutSecretValueError {
    fn from(err: CredentialsError) -> PutSecretValueError {
        PutSecretValueError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutSecretValueError {
    fn from(err: HttpDispatchError) -> PutSecretValueError {
        PutSecretValueError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutSecretValueError {
    fn from(err: io::Error) -> PutSecretValueError {
        PutSecretValueError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutSecretValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutSecretValueError {
    fn description(&self) -> &str {
        match *self {
            PutSecretValueError::EncryptionFailure(ref cause) => cause,
            PutSecretValueError::InternalServiceError(ref cause) => cause,
            PutSecretValueError::InvalidParameter(ref cause) => cause,
            PutSecretValueError::InvalidRequest(ref cause) => cause,
            PutSecretValueError::LimitExceeded(ref cause) => cause,
            PutSecretValueError::ResourceExists(ref cause) => cause,
            PutSecretValueError::ResourceNotFound(ref cause) => cause,
            PutSecretValueError::Validation(ref cause) => cause,
            PutSecretValueError::Credentials(ref err) => err.description(),
            PutSecretValueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutSecretValueError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreSecret
#[derive(Debug, PartialEq)]
pub enum RestoreSecretError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RestoreSecretError {
    pub fn from_body(body: &str) -> RestoreSecretError {
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
                    "InternalServiceError" => {
                        RestoreSecretError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RestoreSecretError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RestoreSecretError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RestoreSecretError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RestoreSecretError::Validation(error_message.to_string())
                    }
                    _ => RestoreSecretError::Unknown(String::from(body)),
                }
            }
            Err(_) => RestoreSecretError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RestoreSecretError {
    fn from(err: serde_json::error::Error) -> RestoreSecretError {
        RestoreSecretError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RestoreSecretError {
    fn from(err: CredentialsError) -> RestoreSecretError {
        RestoreSecretError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RestoreSecretError {
    fn from(err: HttpDispatchError) -> RestoreSecretError {
        RestoreSecretError::HttpDispatch(err)
    }
}
impl From<io::Error> for RestoreSecretError {
    fn from(err: io::Error) -> RestoreSecretError {
        RestoreSecretError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RestoreSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreSecretError {
    fn description(&self) -> &str {
        match *self {
            RestoreSecretError::InternalServiceError(ref cause) => cause,
            RestoreSecretError::InvalidParameter(ref cause) => cause,
            RestoreSecretError::InvalidRequest(ref cause) => cause,
            RestoreSecretError::ResourceNotFound(ref cause) => cause,
            RestoreSecretError::Validation(ref cause) => cause,
            RestoreSecretError::Credentials(ref err) => err.description(),
            RestoreSecretError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RestoreSecretError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RotateSecret
#[derive(Debug, PartialEq)]
pub enum RotateSecretError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RotateSecretError {
    pub fn from_body(body: &str) -> RotateSecretError {
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
                    "InternalServiceError" => {
                        RotateSecretError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RotateSecretError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RotateSecretError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RotateSecretError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RotateSecretError::Validation(error_message.to_string())
                    }
                    _ => RotateSecretError::Unknown(String::from(body)),
                }
            }
            Err(_) => RotateSecretError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RotateSecretError {
    fn from(err: serde_json::error::Error) -> RotateSecretError {
        RotateSecretError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RotateSecretError {
    fn from(err: CredentialsError) -> RotateSecretError {
        RotateSecretError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RotateSecretError {
    fn from(err: HttpDispatchError) -> RotateSecretError {
        RotateSecretError::HttpDispatch(err)
    }
}
impl From<io::Error> for RotateSecretError {
    fn from(err: io::Error) -> RotateSecretError {
        RotateSecretError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RotateSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RotateSecretError {
    fn description(&self) -> &str {
        match *self {
            RotateSecretError::InternalServiceError(ref cause) => cause,
            RotateSecretError::InvalidParameter(ref cause) => cause,
            RotateSecretError::InvalidRequest(ref cause) => cause,
            RotateSecretError::ResourceNotFound(ref cause) => cause,
            RotateSecretError::Validation(ref cause) => cause,
            RotateSecretError::Credentials(ref err) => err.description(),
            RotateSecretError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RotateSecretError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
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
                    "InternalServiceError" => {
                        TagResourceError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        TagResourceError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TagResourceError::ResourceNotFound(String::from(error_message))
                    }
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
            TagResourceError::InternalServiceError(ref cause) => cause,
            TagResourceError::InvalidParameter(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
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
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
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
                    "InternalServiceError" => {
                        UntagResourceError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UntagResourceError::InvalidParameter(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UntagResourceError::ResourceNotFound(String::from(error_message))
                    }
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
            UntagResourceError::InternalServiceError(ref cause) => cause,
            UntagResourceError::InvalidParameter(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSecret
#[derive(Debug, PartialEq)]
pub enum UpdateSecretError {
    /// <p>AWS Secrets Manager can't encrypt the protected secret text using the provided KMS key. Check that the customer master key (CMK) is available, enabled, and not in an invalid state. For more information, see <a href="http://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a>.</p>
    EncryptionFailure(String),
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>The request failed because it would exceed one of the AWS Secrets Manager internal limits.</p>
    LimitExceeded(String),
    /// <p>The policy document that you provided isn't valid.</p>
    MalformedPolicyDocument(String),
    /// <p>A resource with the ID you requested already exists.</p>
    ResourceExists(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateSecretError {
    pub fn from_body(body: &str) -> UpdateSecretError {
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
                    "EncryptionFailure" => {
                        UpdateSecretError::EncryptionFailure(String::from(error_message))
                    }
                    "InternalServiceError" => {
                        UpdateSecretError::InternalServiceError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateSecretError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateSecretError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateSecretError::LimitExceeded(String::from(error_message))
                    }
                    "MalformedPolicyDocumentException" => {
                        UpdateSecretError::MalformedPolicyDocument(String::from(error_message))
                    }
                    "ResourceExistsException" => {
                        UpdateSecretError::ResourceExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateSecretError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSecretError::Validation(error_message.to_string())
                    }
                    _ => UpdateSecretError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSecretError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSecretError {
    fn from(err: serde_json::error::Error) -> UpdateSecretError {
        UpdateSecretError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSecretError {
    fn from(err: CredentialsError) -> UpdateSecretError {
        UpdateSecretError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSecretError {
    fn from(err: HttpDispatchError) -> UpdateSecretError {
        UpdateSecretError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSecretError {
    fn from(err: io::Error) -> UpdateSecretError {
        UpdateSecretError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSecretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSecretError {
    fn description(&self) -> &str {
        match *self {
            UpdateSecretError::EncryptionFailure(ref cause) => cause,
            UpdateSecretError::InternalServiceError(ref cause) => cause,
            UpdateSecretError::InvalidParameter(ref cause) => cause,
            UpdateSecretError::InvalidRequest(ref cause) => cause,
            UpdateSecretError::LimitExceeded(ref cause) => cause,
            UpdateSecretError::MalformedPolicyDocument(ref cause) => cause,
            UpdateSecretError::ResourceExists(ref cause) => cause,
            UpdateSecretError::ResourceNotFound(ref cause) => cause,
            UpdateSecretError::Validation(ref cause) => cause,
            UpdateSecretError::Credentials(ref err) => err.description(),
            UpdateSecretError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateSecretError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSecretVersionStage
#[derive(Debug, PartialEq)]
pub enum UpdateSecretVersionStageError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(String),
    /// <p>You provided an invalid value for a parameter.</p>
    InvalidParameter(String),
    /// <p>You provided a parameter value that is not valid for the current state of the resource. For example, if you try to enable rotation on a secret, you must already have a Lambda function ARN configured or included as a parameter in this call.</p>
    InvalidRequest(String),
    /// <p>The request failed because it would exceed one of the AWS Secrets Manager internal limits.</p>
    LimitExceeded(String),
    /// <p>We can't find the resource that you asked for.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateSecretVersionStageError {
    pub fn from_body(body: &str) -> UpdateSecretVersionStageError {
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
                    "InternalServiceError" => UpdateSecretVersionStageError::InternalServiceError(
                        String::from(error_message),
                    ),
                    "InvalidParameterException" => {
                        UpdateSecretVersionStageError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateSecretVersionStageError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateSecretVersionStageError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateSecretVersionStageError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSecretVersionStageError::Validation(error_message.to_string())
                    }
                    _ => UpdateSecretVersionStageError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSecretVersionStageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSecretVersionStageError {
    fn from(err: serde_json::error::Error) -> UpdateSecretVersionStageError {
        UpdateSecretVersionStageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSecretVersionStageError {
    fn from(err: CredentialsError) -> UpdateSecretVersionStageError {
        UpdateSecretVersionStageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSecretVersionStageError {
    fn from(err: HttpDispatchError) -> UpdateSecretVersionStageError {
        UpdateSecretVersionStageError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateSecretVersionStageError {
    fn from(err: io::Error) -> UpdateSecretVersionStageError {
        UpdateSecretVersionStageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateSecretVersionStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSecretVersionStageError {
    fn description(&self) -> &str {
        match *self {
            UpdateSecretVersionStageError::InternalServiceError(ref cause) => cause,
            UpdateSecretVersionStageError::InvalidParameter(ref cause) => cause,
            UpdateSecretVersionStageError::InvalidRequest(ref cause) => cause,
            UpdateSecretVersionStageError::LimitExceeded(ref cause) => cause,
            UpdateSecretVersionStageError::ResourceNotFound(ref cause) => cause,
            UpdateSecretVersionStageError::Validation(ref cause) => cause,
            UpdateSecretVersionStageError::Credentials(ref err) => err.description(),
            UpdateSecretVersionStageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateSecretVersionStageError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Secrets Manager API. AWS Secrets Manager clients implement this trait.
pub trait SecretsManager {
    /// <p><p>Disables automatic scheduled rotation and cancels the rotation of a secret if one is currently in progress.</p> <p>To re-enable scheduled rotation, call <a>RotateSecret</a> with <code>AutomaticallyRotateAfterDays</code> set to a value greater than 0. This will immediately rotate your secret and then enable the automatic schedule.</p> <note> <p>If you cancel a rotation that is in progress, it can leave the <code>VersionStage</code> labels in an unexpected state. Depending on what step of the rotation was in progress, you might need to remove the staging label <code>AWSPENDING</code> from the partially created version, specified by the <code>SecretVersionId</code> response value. You should also evaluate the partially rotated new version to see if it should be deleted, which you can do by removing all staging labels from the new version&#39;s <code>VersionStage</code> field.</p> </note> <p>To successfully start a rotation, the staging label <code>AWSPENDING</code> must be in one of the following states:</p> <ul> <li> <p>Not be attached to any version at all</p> </li> <li> <p>Attached to the same version as the staging label <code>AWSCURRENT</code> </p> </li> </ul> <p>If the staging label <code>AWSPENDING</code> is attached to a different version than the version with <code>AWSCURRENT</code> then the attempt to rotate fails.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:CancelRotateSecret</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To configure rotation for a secret or to manually trigger a rotation, use <a>RotateSecret</a>.</p> </li> <li> <p>To get the rotation configuration details for a secret, use <a>DescribeSecret</a>.</p> </li> <li> <p>To list all of the currently available secrets, use <a>ListSecrets</a>.</p> </li> <li> <p>To list all of the versions currently associated with a secret, use <a>ListSecretVersionIds</a>.</p> </li> </ul></p>
    fn cancel_rotate_secret(
        &self,
        input: CancelRotateSecretRequest,
    ) -> RusotoFuture<CancelRotateSecretResponse, CancelRotateSecretError>;

    /// <p><p>Creates a new secret. A secret in AWS Secrets Manager consists of both the protected secret data and the important information needed to manage the secret.</p> <p>Secrets Manager stores the encrypted secret data in one of a collection of &quot;versions&quot; associated with the secret. Each version contains a copy of the encrypted secret data. Each version is associated with one or more &quot;staging labels&quot; that identify where the version is in the rotation cycle. The <code>SecretVersionsToStages</code> field of the secret contains the mapping of staging labels to the active versions of the secret. Versions without a staging label are considered deprecated and are not included in the list.</p> <p>You provide the secret data to be encrypted by putting text in the <code>SecretString</code> parameter or binary data in the <code>SecretBinary</code> parameter. If you include <code>SecretString</code> or <code>SecretBinary</code> then Secrets Manager also creates an initial secret version and, if you don&#39;t supply a staging label, automatically maps the new version&#39;s ID to the staging label <code>AWSCURRENT</code>.</p> <important> <ul> <li> <p>If you call an operation that needs to encrypt or decrypt the <code>SecretString</code> and <code>SecretBinary</code> for a secret in the same account as the calling user and that secret doesn&#39;t specify a KMS encryption key, AWS Secrets Manager uses the account&#39;s default AWS managed customer master key (CMK) with the alias <code>aws/secretsmanager</code>. If this key doesn&#39;t already exist in your account then AWS Secrets Manager creates it for you automatically. All users in the same AWS account automatically have access to use the default CMK. Note that if an AWS Secrets Manager API call results in AWS having to create the account&#39;s AWS-managed CMK, it can result in a one-time significant delay in returning the result.</p> </li> <li> <p>If the secret is in a different AWS account from the credentials calling an API that requires encryption or decryption of the secret value then you must create and use a custom KMS CMK because you can&#39;t access the default CMK for the account using credentials from a different AWS account. Store the ARN of the CMK in the secret when you create the secret or when you update it by including it in the <code>KMSKeyId</code>. If you call an API that must encrypt or decrypt <code>SecretString</code> or <code>SecretBinary</code> using credentials from a different account then the KMS key policy must grant cross-account access to that other account&#39;s user or role.</p> </li> </ul> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:CreateSecret</p> </li> <li> <p>kms:GenerateDataKey - needed only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s default AWS managed CMK for Secrets Manager.</p> </li> <li> <p>kms:Encrypt - needed only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s default AWS managed CMK for Secrets Manager.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To delete a secret, use <a>DeleteSecret</a>.</p> </li> <li> <p>To modify an existing secret, use <a>UpdateSecret</a>.</p> </li> <li> <p>To create a new version of a secret, use <a>PutSecretValue</a>.</p> </li> <li> <p>To retrieve the encrypted secure string and secure binary values, use <a>GetSecretValue</a>.</p> </li> <li> <p>To retrieve all other details for a secret, use <a>DescribeSecret</a>. This does not include the encrypted secure string and secure binary values.</p> </li> <li> <p>To retrieve the list of secret versions associated with the current secret, use <a>DescribeSecret</a> and examine the <code>SecretVersionsToStages</code> response value.</p> </li> </ul></p>
    fn create_secret(
        &self,
        input: CreateSecretRequest,
    ) -> RusotoFuture<CreateSecretResponse, CreateSecretError>;

    /// <p><p>Deletes an entire secret and all of its versions. You can optionally include a recovery window during which you can restore the secret. If you don&#39;t provide a recovery window value, the operation defaults to 30 days. Secrets Manager attaches a <code>DeletionDate</code> stamp to the secret that specifies the end of the recovery window. At the end of the recovery window, Secrets Manager deletes the secret permanently.</p> <p>At any time before recovery period ends, you can use <a>RestoreSecret</a> to remove the <code>DeletionDate</code> and cancel the deletion of the secret.</p> <p>You cannot access the encrypted secret information in any secret that is scheduled for deletion. If you need to access that information, you can cancel the deletion with <a>RestoreSecret</a> and then retrieve the information.</p> <note> <ul> <li> <p>There is no explicit operation to delete a version of a secret. Instead, remove all staging labels from the <code>VersionStage</code> field of a version. That marks the version as deprecated and allows AWS Secrets Manager to delete it as needed. Versions that do not have any staging labels do not show up in <a>ListSecretVersionIds</a> unless you specify <code>IncludeDeprecated</code>.</p> </li> <li> <p>The permanent secret deletion at the end of the waiting period is performed as a background task with low priority. There is no guarantee of a specific time after the recovery window for the actual delete operation to occur.</p> </li> </ul> </note> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:DeleteSecret</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To create a secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To cancel deletion of a version of a secret before the recovery period has expired, use <a>RestoreSecret</a>.</p> </li> </ul></p>
    fn delete_secret(
        &self,
        input: DeleteSecretRequest,
    ) -> RusotoFuture<DeleteSecretResponse, DeleteSecretError>;

    /// <p><p>Retrieves the details of a secret. It does not include the encrypted fields. Only those fields that are populated with a value are returned in the response. </p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:DescribeSecret</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To create a secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To modify a secret, use <a>UpdateSecret</a>.</p> </li> <li> <p>To retrieve the encrypted secret information in a version of the secret, use <a>GetSecretValue</a>.</p> </li> <li> <p>To list all of the secrets in the AWS account, use <a>ListSecrets</a>.</p> </li> </ul></p>
    fn describe_secret(
        &self,
        input: DescribeSecretRequest,
    ) -> RusotoFuture<DescribeSecretResponse, DescribeSecretError>;

    /// <p><p>Generates a random password of the specified complexity. This operation is intended for use in the Lambda rotation function. Per best practice, we recommend that you specify the maximum length and include every character type that the system you are generating a password for can support.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:GetRandomPassword</p> </li> </ul></p>
    fn get_random_password(
        &self,
        input: GetRandomPasswordRequest,
    ) -> RusotoFuture<GetRandomPasswordResponse, GetRandomPasswordError>;

    /// <p><p>Retrieves the contents of the encrypted fields <code>SecretString</code> and <code>SecretBinary</code> from the specified version of a secret.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:GetSecretValue</p> </li> <li> <p>kms:Decrypt - required only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s default AWS managed CMK for Secrets Manager.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To create a new version of the secret with different encrypted information, use <a>PutSecretValue</a>.</p> </li> <li> <p>To retrieve the non-encrypted details for the secret, use <a>DescribeSecret</a>.</p> </li> </ul></p>
    fn get_secret_value(
        &self,
        input: GetSecretValueRequest,
    ) -> RusotoFuture<GetSecretValueResponse, GetSecretValueError>;

    /// <p><p>Lists all of the versions attached to the specified secret. The output does not include the <code>SecretString</code> or <code>SecretBinary</code> fields. By default, the list includes only versions that have at least one staging label in <code>VersionStage</code> attached.</p> <note> <p>Always check the <code>NextToken</code> response parameter when calling any of the <code>List*</code> operations. These operations can occasionally return an empty or shorter than expected list of results even when there are more results available. When this happens, the <code>NextToken</code> response parameter contains a value to pass to the next call to the same API to request the next part of the list.</p> </note> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:ListSecretVersionIds</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To list the secrets in an account, use <a>ListSecrets</a>.</p> </li> </ul></p>
    fn list_secret_version_ids(
        &self,
        input: ListSecretVersionIdsRequest,
    ) -> RusotoFuture<ListSecretVersionIdsResponse, ListSecretVersionIdsError>;

    /// <p><p>Lists all of the secrets that are stored by AWS Secrets Manager in the AWS account. To list the versions currently stored for a specific secret, use <a>ListSecretVersionIds</a>. The encrypted fields <code>SecretString</code> and <code>SecretBinary</code> are not included in the output. To get that information, call the <a>GetSecretValue</a> operation.</p> <note> <p>Always check the <code>NextToken</code> response parameter when calling any of the <code>List*</code> operations. These operations can occasionally return an empty or shorter than expected list of results even when there are more results available. When this happens, the <code>NextToken</code> response parameter contains a value to pass to the next call to the same API to request the next part of the list.</p> </note> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:ListSecrets</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To list the versions attached to a secret, use <a>ListSecretVersionIds</a>.</p> </li> </ul></p>
    fn list_secrets(
        &self,
        input: ListSecretsRequest,
    ) -> RusotoFuture<ListSecretsResponse, ListSecretsError>;

    /// <p><p>Stores a new encrypted secret value in the specified secret. To do this, the operation creates a new version and attaches it to the secret. The version can contain a new <code>SecretString</code> value or a new <code>SecretBinary</code> value.</p> <note> <p>The AWS Secrets Manager console uses only the <code>SecretString</code> field. To add binary data to a secret with the <code>SecretBinary</code> field you must use the AWS CLI or one of the AWS SDKs.</p> </note> <ul> <li> <p>If this operation creates the first version for the secret then Secrets Manager automatically attaches the staging label <code>AWSCURRENT</code> to the new version.</p> </li> <li> <p>If another version of this secret already exists, then this operation does not automatically move any staging labels other than those that you specify in the <code>VersionStages</code> parameter.</p> </li> <li> <p>This operation is idempotent. If a version with a <code>SecretVersionId</code> with the same value as the <code>ClientRequestToken</code> parameter already exists and you specify the same secret data, the operation succeeds but does nothing. However, if the secret data is different, then the operation fails because you cannot modify an existing version; you can only create new ones.</p> </li> <li> <p>If this operation moves the staging label <code>AWSCURRENT</code> to this version (because you included it in the <code>StagingLabels</code> parameter) then Secrets Manager also automatically moves the staging label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p> </li> </ul> <important> <ul> <li> <p>If you call an operation that needs to encrypt or decrypt the <code>SecretString</code> and <code>SecretBinary</code> for a secret in the same account as the calling user and that secret doesn&#39;t specify a KMS encryption key, AWS Secrets Manager uses the account&#39;s default AWS managed customer master key (CMK) with the alias <code>aws/secretsmanager</code>. If this key doesn&#39;t already exist in your account then AWS Secrets Manager creates it for you automatically. All users in the same AWS account automatically have access to use the default CMK. Note that if an AWS Secrets Manager API call results in AWS having to create the account&#39;s AWS-managed CMK, it can result in a one-time significant delay in returning the result.</p> </li> <li> <p>If the secret is in a different AWS account from the credentials calling an API that requires encryption or decryption of the secret value then you must create and use a custom KMS CMK because you can&#39;t access the default CMK for the account using credentials from a different AWS account. Store the ARN of the CMK in the secret when you create the secret or when you update it by including it in the <code>KMSKeyId</code>. If you call an API that must encrypt or decrypt <code>SecretString</code> or <code>SecretBinary</code> using credentials from a different account then the KMS key policy must grant cross-account access to that other account&#39;s user or role.</p> </li> </ul> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:PutSecretValue</p> </li> <li> <p>kms:GenerateDataKey - needed only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s AWS managed CMK for Secrets Manager.</p> </li> <li> <p>kms:Encrypt - needed only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s AWS managed CMK for Secrets Manager.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To retrieve the encrypted value you store in the version of a secret, use <a>GetSecretValue</a>.</p> </li> <li> <p>To create a secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To get the details for a secret, use <a>DescribeSecret</a>.</p> </li> <li> <p>To list the versions attached to a secret, use <a>ListSecretVersionIds</a>.</p> </li> </ul></p>
    fn put_secret_value(
        &self,
        input: PutSecretValueRequest,
    ) -> RusotoFuture<PutSecretValueResponse, PutSecretValueError>;

    /// <p><p>Cancels the scheduled deletion of a secret by removing the <code>DeletedDate</code> time stamp. This makes the secret accessible to query once again.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:RestoreSecret</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To delete a secret, use <a>DeleteSecret</a>.</p> </li> </ul></p>
    fn restore_secret(
        &self,
        input: RestoreSecretRequest,
    ) -> RusotoFuture<RestoreSecretResponse, RestoreSecretError>;

    /// <p><p>Configures and starts the asynchronous process of rotating this secret. If you include the configuration parameters, the operation sets those values for the secret and then immediately starts a rotation. If you do not include the configuration parameters, the operation starts a rotation with the values already stored in the secret. After the rotation completes, the protected service and its clients all use the new version of the secret. </p> <p>This required configuration information includes the ARN of an AWS Lambda function and the time between scheduled rotations. The Lambda rotation function creates a new version of the secret and creates or updates the credentials on the protected service to match. After testing the new credentials, the function marks the new secret with the staging label <code>AWSCURRENT</code> so that your clients all immediately begin to use the new version. For more information about rotating secrets and how to configure a Lambda function to rotate the secrets for your protected service, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.com/;asm-service-name;/latest/userguide/rotating-secrets.html">Rotating Secrets in AWS Secrets Manager</a> in the <i>AWS Secrets Manager User Guide</i>.</p> <p>The rotation function must end with the versions of the secret in one of two states:</p> <ul> <li> <p>The <code>AWSPENDING</code> and <code>AWSCURRENT</code> staging labels are attached to the same version of the secret, or</p> </li> <li> <p>The <code>AWSPENDING</code> staging label is not attached to any version of the secret.</p> </li> </ul> <p>If instead the <code>AWSPENDING</code> staging label is present but is not attached to the same version as <code>AWSCURRENT</code> then any later invocation of <code>RotateSecret</code> assumes that a previous rotation request is still in progress and returns an error.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:RotateSecret</p> </li> <li> <p>lambda:InvokeFunction (on the function specified in the secret&#39;s metadata)</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To list the secrets in your account, use <a>ListSecrets</a>.</p> </li> <li> <p>To get the details for a version of a secret, use <a>DescribeSecret</a>.</p> </li> <li> <p>To create a new version of a secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To attach staging labels to or remove staging labels from a version of a secret, use <a>UpdateSecretVersionStage</a>.</p> </li> </ul></p>
    fn rotate_secret(
        &self,
        input: RotateSecretRequest,
    ) -> RusotoFuture<RotateSecretResponse, RotateSecretError>;

    /// <p><p>Attaches one or more tags, each consisting of a key name and a value, to the specified secret. Tags are part of the secret&#39;s overall metadata, and are not associated with any specific version of the secret. This operation only appends tags to the existing list of tags. To remove tags, you must use <a>UntagResource</a>.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per secret—50</p> </li> <li> <p>Maximum key length—127 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length—255 Unicode characters in UTF-8</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use the <code>aws:</code> prefix in your tag names or values because it is reserved for AWS use. You can&#39;t edit or delete tag names or values with this prefix. Tags with this prefix do not count against your tags per secret limit.</p> </li> <li> <p>If your tagging schema will be used across multiple services and resources, remember that other services might have restrictions on allowed characters. Generally allowed characters are: letters, spaces, and numbers representable in UTF-8, plus the following special characters: + - = . _ : / @.</p> </li> </ul> <important> <p>If you use tags as part of your security strategy, then adding or removing a tag can change permissions. If successfully completing this operation would result in you losing your permissions for this secret, then the operation is blocked and returns an Access Denied error.</p> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:TagResource</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To remove one or more tags from the collection attached to a secret, use <a>UntagResource</a>.</p> </li> <li> <p>To view the list of tags attached to a secret, use <a>DescribeSecret</a>.</p> </li> </ul></p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p><p>Removes one or more tags from the specified secret.</p> <p>This operation is idempotent. If a requested tag is not attached to the secret, no error is returned and the secret metadata is unchanged.</p> <important> <p>If you use tags as part of your security strategy, then removing a tag can change permissions. If successfully completing this operation would result in you losing your permissions for this secret, then the operation is blocked and returns an Access Denied error.</p> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:UntagResource</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To add one or more tags to the collection attached to a secret, use <a>TagResource</a>.</p> </li> <li> <p>To view the list of tags attached to a secret, use <a>DescribeSecret</a>.</p> </li> </ul></p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p><p>Modifies many of the details of a secret. If you include a <code>ClientRequestToken</code> and either <code>SecretString</code> or <code>SecretBinary</code> then it also creates a new version attached to the secret.</p> <p>To modify the rotation configuration of a secret, use <a>RotateSecret</a> instead.</p> <note> <p>The AWS Secrets Manager console uses only the <code>SecretString</code> parameter and therefore limits you to encrypting and storing only a text string. To encrypt and store binary data as part of the version of a secret, you must use either the AWS CLI or one of the AWS SDKs.</p> </note> <ul> <li> <p>If this update creates the first version of the secret or if you did not include the <code>VersionStages</code> parameter then Secrets Manager automatically attaches the staging label <code>AWSCURRENT</code> to the new version and removes it from any version that had it previously. The previous version (if any) is then given the staging label <code>AWSPREVIOUS</code>.</p> </li> <li> <p>If a version with a <code>SecretVersionId</code> with the same value as the <code>ClientRequestToken</code> parameter already exists, the operation generates an error. You cannot modify an existing version, you can only create new ones.</p> </li> </ul> <important> <ul> <li> <p>If you call an operation that needs to encrypt or decrypt the <code>SecretString</code> and <code>SecretBinary</code> for a secret in the same account as the calling user and that secret doesn&#39;t specify a KMS encryption key, AWS Secrets Manager uses the account&#39;s default AWS managed customer master key (CMK) with the alias <code>aws/secretsmanager</code>. If this key doesn&#39;t already exist in your account then AWS Secrets Manager creates it for you automatically. All users in the same AWS account automatically have access to use the default CMK. Note that if an AWS Secrets Manager API call results in AWS having to create the account&#39;s AWS-managed CMK, it can result in a one-time significant delay in returning the result.</p> </li> <li> <p>If the secret is in a different AWS account from the credentials calling an API that requires encryption or decryption of the secret value then you must create and use a custom KMS CMK because you can&#39;t access the default CMK for the account using credentials from a different AWS account. Store the ARN of the CMK in the secret when you create the secret or when you update it by including it in the <code>KMSKeyId</code>. If you call an API that must encrypt or decrypt <code>SecretString</code> or <code>SecretBinary</code> using credentials from a different account then the KMS key policy must grant cross-account access to that other account&#39;s user or role.</p> </li> </ul> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:UpdateSecret</p> </li> <li> <p>kms:GenerateDataKey - needed only if you use a custom KMS key to encrypt the secret. You do not need this permission to use the account&#39;s AWS managed CMK for Secrets Manager.</p> </li> <li> <p>kms:Decrypt - needed only if you use a custom KMS key to encrypt the secret. You do not need this permission to use the account&#39;s AWS managed CMK for Secrets Manager.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To create a new secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To add only a new version to an existing secret, use <a>PutSecretValue</a>.</p> </li> <li> <p>To get the details for a secret, use <a>DescribeSecret</a>.</p> </li> <li> <p>To list the versions contained in a secret, use <a>ListSecretVersionIds</a>.</p> </li> </ul></p>
    fn update_secret(
        &self,
        input: UpdateSecretRequest,
    ) -> RusotoFuture<UpdateSecretResponse, UpdateSecretError>;

    /// <p><p>Modifies the staging labels attached to a version of a secret. Staging labels are used to track a version as it progresses through the secret rotation process. You can attach a staging label to only one version of a secret at a time. If a staging label to be added is already attached to another version, then it is moved--removed from the other version first and then attached to this one. For more information about staging labels, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.com/;asm-service-name;/latest/userguide/terms-concepts.html#term_label">Staging Labels</a> in the <i>AWS Secrets Manager User Guide</i>. </p> <p>The staging labels that you specify in the <code>VersionStage</code> parameter are added to the existing list of staging labels--they don&#39;t replace it.</p> <p>You can move the <code>AWSCURRENT</code> staging label to this version by including it in this call.</p> <note> <p>Whenever you move <code>AWSCURRENT</code>, Secrets Manager automatically moves the label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p> </note> <p>If this action results in the last label being removed from a version, then the version is considered to be &#39;deprecated&#39; and can be deleted by Secrets Manager.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:UpdateSecretVersionStage</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To get the list of staging labels that are currently associated with a version of a secret, use <code> <a>DescribeSecret</a> </code> and examine the <code>SecretVersionsToStages</code> response value.</p> </li> </ul></p>
    fn update_secret_version_stage(
        &self,
        input: UpdateSecretVersionStageRequest,
    ) -> RusotoFuture<UpdateSecretVersionStageResponse, UpdateSecretVersionStageError>;
}
/// A client for the AWS Secrets Manager API.
pub struct SecretsManagerClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl SecretsManagerClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> SecretsManagerClient {
        SecretsManagerClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> SecretsManagerClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        SecretsManagerClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> SecretsManager for SecretsManagerClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p><p>Disables automatic scheduled rotation and cancels the rotation of a secret if one is currently in progress.</p> <p>To re-enable scheduled rotation, call <a>RotateSecret</a> with <code>AutomaticallyRotateAfterDays</code> set to a value greater than 0. This will immediately rotate your secret and then enable the automatic schedule.</p> <note> <p>If you cancel a rotation that is in progress, it can leave the <code>VersionStage</code> labels in an unexpected state. Depending on what step of the rotation was in progress, you might need to remove the staging label <code>AWSPENDING</code> from the partially created version, specified by the <code>SecretVersionId</code> response value. You should also evaluate the partially rotated new version to see if it should be deleted, which you can do by removing all staging labels from the new version&#39;s <code>VersionStage</code> field.</p> </note> <p>To successfully start a rotation, the staging label <code>AWSPENDING</code> must be in one of the following states:</p> <ul> <li> <p>Not be attached to any version at all</p> </li> <li> <p>Attached to the same version as the staging label <code>AWSCURRENT</code> </p> </li> </ul> <p>If the staging label <code>AWSPENDING</code> is attached to a different version than the version with <code>AWSCURRENT</code> then the attempt to rotate fails.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:CancelRotateSecret</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To configure rotation for a secret or to manually trigger a rotation, use <a>RotateSecret</a>.</p> </li> <li> <p>To get the rotation configuration details for a secret, use <a>DescribeSecret</a>.</p> </li> <li> <p>To list all of the currently available secrets, use <a>ListSecrets</a>.</p> </li> <li> <p>To list all of the versions currently associated with a secret, use <a>ListSecretVersionIds</a>.</p> </li> </ul></p>
    fn cancel_rotate_secret(
        &self,
        input: CancelRotateSecretRequest,
    ) -> RusotoFuture<CancelRotateSecretResponse, CancelRotateSecretError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.CancelRotateSecret");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CancelRotateSecretResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelRotateSecretError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Creates a new secret. A secret in AWS Secrets Manager consists of both the protected secret data and the important information needed to manage the secret.</p> <p>Secrets Manager stores the encrypted secret data in one of a collection of &quot;versions&quot; associated with the secret. Each version contains a copy of the encrypted secret data. Each version is associated with one or more &quot;staging labels&quot; that identify where the version is in the rotation cycle. The <code>SecretVersionsToStages</code> field of the secret contains the mapping of staging labels to the active versions of the secret. Versions without a staging label are considered deprecated and are not included in the list.</p> <p>You provide the secret data to be encrypted by putting text in the <code>SecretString</code> parameter or binary data in the <code>SecretBinary</code> parameter. If you include <code>SecretString</code> or <code>SecretBinary</code> then Secrets Manager also creates an initial secret version and, if you don&#39;t supply a staging label, automatically maps the new version&#39;s ID to the staging label <code>AWSCURRENT</code>.</p> <important> <ul> <li> <p>If you call an operation that needs to encrypt or decrypt the <code>SecretString</code> and <code>SecretBinary</code> for a secret in the same account as the calling user and that secret doesn&#39;t specify a KMS encryption key, AWS Secrets Manager uses the account&#39;s default AWS managed customer master key (CMK) with the alias <code>aws/secretsmanager</code>. If this key doesn&#39;t already exist in your account then AWS Secrets Manager creates it for you automatically. All users in the same AWS account automatically have access to use the default CMK. Note that if an AWS Secrets Manager API call results in AWS having to create the account&#39;s AWS-managed CMK, it can result in a one-time significant delay in returning the result.</p> </li> <li> <p>If the secret is in a different AWS account from the credentials calling an API that requires encryption or decryption of the secret value then you must create and use a custom KMS CMK because you can&#39;t access the default CMK for the account using credentials from a different AWS account. Store the ARN of the CMK in the secret when you create the secret or when you update it by including it in the <code>KMSKeyId</code>. If you call an API that must encrypt or decrypt <code>SecretString</code> or <code>SecretBinary</code> using credentials from a different account then the KMS key policy must grant cross-account access to that other account&#39;s user or role.</p> </li> </ul> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:CreateSecret</p> </li> <li> <p>kms:GenerateDataKey - needed only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s default AWS managed CMK for Secrets Manager.</p> </li> <li> <p>kms:Encrypt - needed only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s default AWS managed CMK for Secrets Manager.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To delete a secret, use <a>DeleteSecret</a>.</p> </li> <li> <p>To modify an existing secret, use <a>UpdateSecret</a>.</p> </li> <li> <p>To create a new version of a secret, use <a>PutSecretValue</a>.</p> </li> <li> <p>To retrieve the encrypted secure string and secure binary values, use <a>GetSecretValue</a>.</p> </li> <li> <p>To retrieve all other details for a secret, use <a>DescribeSecret</a>. This does not include the encrypted secure string and secure binary values.</p> </li> <li> <p>To retrieve the list of secret versions associated with the current secret, use <a>DescribeSecret</a> and examine the <code>SecretVersionsToStages</code> response value.</p> </li> </ul></p>
    fn create_secret(
        &self,
        input: CreateSecretRequest,
    ) -> RusotoFuture<CreateSecretResponse, CreateSecretError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.CreateSecret");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSecretResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateSecretError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Deletes an entire secret and all of its versions. You can optionally include a recovery window during which you can restore the secret. If you don&#39;t provide a recovery window value, the operation defaults to 30 days. Secrets Manager attaches a <code>DeletionDate</code> stamp to the secret that specifies the end of the recovery window. At the end of the recovery window, Secrets Manager deletes the secret permanently.</p> <p>At any time before recovery period ends, you can use <a>RestoreSecret</a> to remove the <code>DeletionDate</code> and cancel the deletion of the secret.</p> <p>You cannot access the encrypted secret information in any secret that is scheduled for deletion. If you need to access that information, you can cancel the deletion with <a>RestoreSecret</a> and then retrieve the information.</p> <note> <ul> <li> <p>There is no explicit operation to delete a version of a secret. Instead, remove all staging labels from the <code>VersionStage</code> field of a version. That marks the version as deprecated and allows AWS Secrets Manager to delete it as needed. Versions that do not have any staging labels do not show up in <a>ListSecretVersionIds</a> unless you specify <code>IncludeDeprecated</code>.</p> </li> <li> <p>The permanent secret deletion at the end of the waiting period is performed as a background task with low priority. There is no guarantee of a specific time after the recovery window for the actual delete operation to occur.</p> </li> </ul> </note> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:DeleteSecret</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To create a secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To cancel deletion of a version of a secret before the recovery period has expired, use <a>RestoreSecret</a>.</p> </li> </ul></p>
    fn delete_secret(
        &self,
        input: DeleteSecretRequest,
    ) -> RusotoFuture<DeleteSecretResponse, DeleteSecretError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.DeleteSecret");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSecretResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSecretError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieves the details of a secret. It does not include the encrypted fields. Only those fields that are populated with a value are returned in the response. </p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:DescribeSecret</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To create a secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To modify a secret, use <a>UpdateSecret</a>.</p> </li> <li> <p>To retrieve the encrypted secret information in a version of the secret, use <a>GetSecretValue</a>.</p> </li> <li> <p>To list all of the secrets in the AWS account, use <a>ListSecrets</a>.</p> </li> </ul></p>
    fn describe_secret(
        &self,
        input: DescribeSecretRequest,
    ) -> RusotoFuture<DescribeSecretResponse, DescribeSecretError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.DescribeSecret");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeSecretResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSecretError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Generates a random password of the specified complexity. This operation is intended for use in the Lambda rotation function. Per best practice, we recommend that you specify the maximum length and include every character type that the system you are generating a password for can support.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:GetRandomPassword</p> </li> </ul></p>
    fn get_random_password(
        &self,
        input: GetRandomPasswordRequest,
    ) -> RusotoFuture<GetRandomPasswordResponse, GetRandomPasswordError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.GetRandomPassword");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRandomPasswordResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetRandomPasswordError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Retrieves the contents of the encrypted fields <code>SecretString</code> and <code>SecretBinary</code> from the specified version of a secret.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:GetSecretValue</p> </li> <li> <p>kms:Decrypt - required only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s default AWS managed CMK for Secrets Manager.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To create a new version of the secret with different encrypted information, use <a>PutSecretValue</a>.</p> </li> <li> <p>To retrieve the non-encrypted details for the secret, use <a>DescribeSecret</a>.</p> </li> </ul></p>
    fn get_secret_value(
        &self,
        input: GetSecretValueRequest,
    ) -> RusotoFuture<GetSecretValueResponse, GetSecretValueError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.GetSecretValue");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSecretValueResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetSecretValueError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Lists all of the versions attached to the specified secret. The output does not include the <code>SecretString</code> or <code>SecretBinary</code> fields. By default, the list includes only versions that have at least one staging label in <code>VersionStage</code> attached.</p> <note> <p>Always check the <code>NextToken</code> response parameter when calling any of the <code>List*</code> operations. These operations can occasionally return an empty or shorter than expected list of results even when there are more results available. When this happens, the <code>NextToken</code> response parameter contains a value to pass to the next call to the same API to request the next part of the list.</p> </note> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:ListSecretVersionIds</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To list the secrets in an account, use <a>ListSecrets</a>.</p> </li> </ul></p>
    fn list_secret_version_ids(
        &self,
        input: ListSecretVersionIdsRequest,
    ) -> RusotoFuture<ListSecretVersionIdsResponse, ListSecretVersionIdsError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.ListSecretVersionIds");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSecretVersionIdsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListSecretVersionIdsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Lists all of the secrets that are stored by AWS Secrets Manager in the AWS account. To list the versions currently stored for a specific secret, use <a>ListSecretVersionIds</a>. The encrypted fields <code>SecretString</code> and <code>SecretBinary</code> are not included in the output. To get that information, call the <a>GetSecretValue</a> operation.</p> <note> <p>Always check the <code>NextToken</code> response parameter when calling any of the <code>List*</code> operations. These operations can occasionally return an empty or shorter than expected list of results even when there are more results available. When this happens, the <code>NextToken</code> response parameter contains a value to pass to the next call to the same API to request the next part of the list.</p> </note> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:ListSecrets</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To list the versions attached to a secret, use <a>ListSecretVersionIds</a>.</p> </li> </ul></p>
    fn list_secrets(
        &self,
        input: ListSecretsRequest,
    ) -> RusotoFuture<ListSecretsResponse, ListSecretsError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.ListSecrets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSecretsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListSecretsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Stores a new encrypted secret value in the specified secret. To do this, the operation creates a new version and attaches it to the secret. The version can contain a new <code>SecretString</code> value or a new <code>SecretBinary</code> value.</p> <note> <p>The AWS Secrets Manager console uses only the <code>SecretString</code> field. To add binary data to a secret with the <code>SecretBinary</code> field you must use the AWS CLI or one of the AWS SDKs.</p> </note> <ul> <li> <p>If this operation creates the first version for the secret then Secrets Manager automatically attaches the staging label <code>AWSCURRENT</code> to the new version.</p> </li> <li> <p>If another version of this secret already exists, then this operation does not automatically move any staging labels other than those that you specify in the <code>VersionStages</code> parameter.</p> </li> <li> <p>This operation is idempotent. If a version with a <code>SecretVersionId</code> with the same value as the <code>ClientRequestToken</code> parameter already exists and you specify the same secret data, the operation succeeds but does nothing. However, if the secret data is different, then the operation fails because you cannot modify an existing version; you can only create new ones.</p> </li> <li> <p>If this operation moves the staging label <code>AWSCURRENT</code> to this version (because you included it in the <code>StagingLabels</code> parameter) then Secrets Manager also automatically moves the staging label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p> </li> </ul> <important> <ul> <li> <p>If you call an operation that needs to encrypt or decrypt the <code>SecretString</code> and <code>SecretBinary</code> for a secret in the same account as the calling user and that secret doesn&#39;t specify a KMS encryption key, AWS Secrets Manager uses the account&#39;s default AWS managed customer master key (CMK) with the alias <code>aws/secretsmanager</code>. If this key doesn&#39;t already exist in your account then AWS Secrets Manager creates it for you automatically. All users in the same AWS account automatically have access to use the default CMK. Note that if an AWS Secrets Manager API call results in AWS having to create the account&#39;s AWS-managed CMK, it can result in a one-time significant delay in returning the result.</p> </li> <li> <p>If the secret is in a different AWS account from the credentials calling an API that requires encryption or decryption of the secret value then you must create and use a custom KMS CMK because you can&#39;t access the default CMK for the account using credentials from a different AWS account. Store the ARN of the CMK in the secret when you create the secret or when you update it by including it in the <code>KMSKeyId</code>. If you call an API that must encrypt or decrypt <code>SecretString</code> or <code>SecretBinary</code> using credentials from a different account then the KMS key policy must grant cross-account access to that other account&#39;s user or role.</p> </li> </ul> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:PutSecretValue</p> </li> <li> <p>kms:GenerateDataKey - needed only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s AWS managed CMK for Secrets Manager.</p> </li> <li> <p>kms:Encrypt - needed only if you use a customer-created KMS key to encrypt the secret. You do not need this permission to use the account&#39;s AWS managed CMK for Secrets Manager.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To retrieve the encrypted value you store in the version of a secret, use <a>GetSecretValue</a>.</p> </li> <li> <p>To create a secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To get the details for a secret, use <a>DescribeSecret</a>.</p> </li> <li> <p>To list the versions attached to a secret, use <a>ListSecretVersionIds</a>.</p> </li> </ul></p>
    fn put_secret_value(
        &self,
        input: PutSecretValueRequest,
    ) -> RusotoFuture<PutSecretValueResponse, PutSecretValueError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.PutSecretValue");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutSecretValueResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutSecretValueError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Cancels the scheduled deletion of a secret by removing the <code>DeletedDate</code> time stamp. This makes the secret accessible to query once again.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:RestoreSecret</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To delete a secret, use <a>DeleteSecret</a>.</p> </li> </ul></p>
    fn restore_secret(
        &self,
        input: RestoreSecretRequest,
    ) -> RusotoFuture<RestoreSecretResponse, RestoreSecretError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.RestoreSecret");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RestoreSecretResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RestoreSecretError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Configures and starts the asynchronous process of rotating this secret. If you include the configuration parameters, the operation sets those values for the secret and then immediately starts a rotation. If you do not include the configuration parameters, the operation starts a rotation with the values already stored in the secret. After the rotation completes, the protected service and its clients all use the new version of the secret. </p> <p>This required configuration information includes the ARN of an AWS Lambda function and the time between scheduled rotations. The Lambda rotation function creates a new version of the secret and creates or updates the credentials on the protected service to match. After testing the new credentials, the function marks the new secret with the staging label <code>AWSCURRENT</code> so that your clients all immediately begin to use the new version. For more information about rotating secrets and how to configure a Lambda function to rotate the secrets for your protected service, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.com/;asm-service-name;/latest/userguide/rotating-secrets.html">Rotating Secrets in AWS Secrets Manager</a> in the <i>AWS Secrets Manager User Guide</i>.</p> <p>The rotation function must end with the versions of the secret in one of two states:</p> <ul> <li> <p>The <code>AWSPENDING</code> and <code>AWSCURRENT</code> staging labels are attached to the same version of the secret, or</p> </li> <li> <p>The <code>AWSPENDING</code> staging label is not attached to any version of the secret.</p> </li> </ul> <p>If instead the <code>AWSPENDING</code> staging label is present but is not attached to the same version as <code>AWSCURRENT</code> then any later invocation of <code>RotateSecret</code> assumes that a previous rotation request is still in progress and returns an error.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:RotateSecret</p> </li> <li> <p>lambda:InvokeFunction (on the function specified in the secret&#39;s metadata)</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To list the secrets in your account, use <a>ListSecrets</a>.</p> </li> <li> <p>To get the details for a version of a secret, use <a>DescribeSecret</a>.</p> </li> <li> <p>To create a new version of a secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To attach staging labels to or remove staging labels from a version of a secret, use <a>UpdateSecretVersionStage</a>.</p> </li> </ul></p>
    fn rotate_secret(
        &self,
        input: RotateSecretRequest,
    ) -> RusotoFuture<RotateSecretResponse, RotateSecretError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.RotateSecret");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RotateSecretResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(RotateSecretError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Attaches one or more tags, each consisting of a key name and a value, to the specified secret. Tags are part of the secret&#39;s overall metadata, and are not associated with any specific version of the secret. This operation only appends tags to the existing list of tags. To remove tags, you must use <a>UntagResource</a>.</p> <p>The following basic restrictions apply to tags:</p> <ul> <li> <p>Maximum number of tags per secret—50</p> </li> <li> <p>Maximum key length—127 Unicode characters in UTF-8</p> </li> <li> <p>Maximum value length—255 Unicode characters in UTF-8</p> </li> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>Do not use the <code>aws:</code> prefix in your tag names or values because it is reserved for AWS use. You can&#39;t edit or delete tag names or values with this prefix. Tags with this prefix do not count against your tags per secret limit.</p> </li> <li> <p>If your tagging schema will be used across multiple services and resources, remember that other services might have restrictions on allowed characters. Generally allowed characters are: letters, spaces, and numbers representable in UTF-8, plus the following special characters: + - = . _ : / @.</p> </li> </ul> <important> <p>If you use tags as part of your security strategy, then adding or removing a tag can change permissions. If successfully completing this operation would result in you losing your permissions for this secret, then the operation is blocked and returns an Access Denied error.</p> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:TagResource</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To remove one or more tags from the collection attached to a secret, use <a>UntagResource</a>.</p> </li> <li> <p>To view the list of tags attached to a secret, use <a>DescribeSecret</a>.</p> </li> </ul></p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.TagResource");
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

    /// <p><p>Removes one or more tags from the specified secret.</p> <p>This operation is idempotent. If a requested tag is not attached to the secret, no error is returned and the secret metadata is unchanged.</p> <important> <p>If you use tags as part of your security strategy, then removing a tag can change permissions. If successfully completing this operation would result in you losing your permissions for this secret, then the operation is blocked and returns an Access Denied error.</p> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:UntagResource</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To add one or more tags to the collection attached to a secret, use <a>TagResource</a>.</p> </li> <li> <p>To view the list of tags attached to a secret, use <a>DescribeSecret</a>.</p> </li> </ul></p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.UntagResource");
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

    /// <p><p>Modifies many of the details of a secret. If you include a <code>ClientRequestToken</code> and either <code>SecretString</code> or <code>SecretBinary</code> then it also creates a new version attached to the secret.</p> <p>To modify the rotation configuration of a secret, use <a>RotateSecret</a> instead.</p> <note> <p>The AWS Secrets Manager console uses only the <code>SecretString</code> parameter and therefore limits you to encrypting and storing only a text string. To encrypt and store binary data as part of the version of a secret, you must use either the AWS CLI or one of the AWS SDKs.</p> </note> <ul> <li> <p>If this update creates the first version of the secret or if you did not include the <code>VersionStages</code> parameter then Secrets Manager automatically attaches the staging label <code>AWSCURRENT</code> to the new version and removes it from any version that had it previously. The previous version (if any) is then given the staging label <code>AWSPREVIOUS</code>.</p> </li> <li> <p>If a version with a <code>SecretVersionId</code> with the same value as the <code>ClientRequestToken</code> parameter already exists, the operation generates an error. You cannot modify an existing version, you can only create new ones.</p> </li> </ul> <important> <ul> <li> <p>If you call an operation that needs to encrypt or decrypt the <code>SecretString</code> and <code>SecretBinary</code> for a secret in the same account as the calling user and that secret doesn&#39;t specify a KMS encryption key, AWS Secrets Manager uses the account&#39;s default AWS managed customer master key (CMK) with the alias <code>aws/secretsmanager</code>. If this key doesn&#39;t already exist in your account then AWS Secrets Manager creates it for you automatically. All users in the same AWS account automatically have access to use the default CMK. Note that if an AWS Secrets Manager API call results in AWS having to create the account&#39;s AWS-managed CMK, it can result in a one-time significant delay in returning the result.</p> </li> <li> <p>If the secret is in a different AWS account from the credentials calling an API that requires encryption or decryption of the secret value then you must create and use a custom KMS CMK because you can&#39;t access the default CMK for the account using credentials from a different AWS account. Store the ARN of the CMK in the secret when you create the secret or when you update it by including it in the <code>KMSKeyId</code>. If you call an API that must encrypt or decrypt <code>SecretString</code> or <code>SecretBinary</code> using credentials from a different account then the KMS key policy must grant cross-account access to that other account&#39;s user or role.</p> </li> </ul> </important> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:UpdateSecret</p> </li> <li> <p>kms:GenerateDataKey - needed only if you use a custom KMS key to encrypt the secret. You do not need this permission to use the account&#39;s AWS managed CMK for Secrets Manager.</p> </li> <li> <p>kms:Decrypt - needed only if you use a custom KMS key to encrypt the secret. You do not need this permission to use the account&#39;s AWS managed CMK for Secrets Manager.</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To create a new secret, use <a>CreateSecret</a>.</p> </li> <li> <p>To add only a new version to an existing secret, use <a>PutSecretValue</a>.</p> </li> <li> <p>To get the details for a secret, use <a>DescribeSecret</a>.</p> </li> <li> <p>To list the versions contained in a secret, use <a>ListSecretVersionIds</a>.</p> </li> </ul></p>
    fn update_secret(
        &self,
        input: UpdateSecretRequest,
    ) -> RusotoFuture<UpdateSecretResponse, UpdateSecretError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.UpdateSecret");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateSecretResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSecretError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p><p>Modifies the staging labels attached to a version of a secret. Staging labels are used to track a version as it progresses through the secret rotation process. You can attach a staging label to only one version of a secret at a time. If a staging label to be added is already attached to another version, then it is moved--removed from the other version first and then attached to this one. For more information about staging labels, see <a href="http://docs.aws.amazon.com/http:/docs.aws.amazon.com/;asm-service-name;/latest/userguide/terms-concepts.html#term_label">Staging Labels</a> in the <i>AWS Secrets Manager User Guide</i>. </p> <p>The staging labels that you specify in the <code>VersionStage</code> parameter are added to the existing list of staging labels--they don&#39;t replace it.</p> <p>You can move the <code>AWSCURRENT</code> staging label to this version by including it in this call.</p> <note> <p>Whenever you move <code>AWSCURRENT</code>, Secrets Manager automatically moves the label <code>AWSPREVIOUS</code> to the version that <code>AWSCURRENT</code> was removed from.</p> </note> <p>If this action results in the last label being removed from a version, then the version is considered to be &#39;deprecated&#39; and can be deleted by Secrets Manager.</p> <p> <b>Minimum permissions</b> </p> <p>To run this command, you must have the following permissions:</p> <ul> <li> <p>secretsmanager:UpdateSecretVersionStage</p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p>To get the list of staging labels that are currently associated with a version of a secret, use <code> <a>DescribeSecret</a> </code> and examine the <code>SecretVersionsToStages</code> response value.</p> </li> </ul></p>
    fn update_secret_version_stage(
        &self,
        input: UpdateSecretVersionStageRequest,
    ) -> RusotoFuture<UpdateSecretVersionStageResponse, UpdateSecretVersionStageError> {
        let mut request = SignedRequest::new("POST", "secretsmanager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "secretsmanager.UpdateSecretVersionStage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateSecretVersionStageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateSecretVersionStageError::from_body(
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
