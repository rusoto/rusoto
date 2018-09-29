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
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateMemberAccountRequest {
    /// <p>The ID of the AWS account that you want to associate with Amazon Macie as a member account.</p>
    #[serde(rename = "memberAccountId")]
    pub member_account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateS3ResourcesRequest {
    /// <p>The ID of the Amazon Macie member account whose resources you want to associate with Macie. </p>
    #[serde(rename = "memberAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_id: Option<String>,
    /// <p>The S3 resources that you want to associate with Amazon Macie for monitoring and data classification. </p>
    #[serde(rename = "s3Resources")]
    pub s_3_resources: Vec<S3ResourceClassification>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateS3ResourcesResult {
    /// <p>S3 resources that couldn't be associated with Amazon Macie. An error code and an error message are provided for each failed item. </p>
    #[serde(rename = "failedS3Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_s3_resources: Option<Vec<FailedS3Resource>>,
}

/// <p>The classification type that Amazon Macie applies to the associated S3 resources. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationType {
    /// <p>A continuous classification of the objects that are added to a specified S3 bucket. Amazon Macie begins performing continuous classification after a bucket is successfully associated with Amazon Macie. </p>
    #[serde(rename = "continuous")]
    pub continuous: String,
    /// <p>A one-time classification of all of the existing objects in a specified S3 bucket. </p>
    #[serde(rename = "oneTime")]
    pub one_time: String,
}

/// <p>The classification type that Amazon Macie applies to the associated S3 resources. At least one of the classification types (oneTime or continuous) must be specified. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ClassificationTypeUpdate {
    /// <p>A continuous classification of the objects that are added to a specified S3 bucket. Amazon Macie begins performing continuous classification after a bucket is successfully associated with Amazon Macie. </p>
    #[serde(rename = "continuous")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous: Option<String>,
    /// <p>A one-time classification of all of the existing objects in a specified S3 bucket. </p>
    #[serde(rename = "oneTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateMemberAccountRequest {
    /// <p>The ID of the member account that you want to remove from Amazon Macie.</p>
    #[serde(rename = "memberAccountId")]
    pub member_account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateS3ResourcesRequest {
    /// <p>The S3 resources (buckets or prefixes) that you want to remove from being monitored and classified by Amazon Macie. </p>
    #[serde(rename = "associatedS3Resources")]
    pub associated_s3_resources: Vec<S3Resource>,
    /// <p>The ID of the Amazon Macie member account whose resources you want to remove from being monitored by Amazon Macie. </p>
    #[serde(rename = "memberAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateS3ResourcesResult {
    /// <p>S3 resources that couldn't be removed from being monitored and classified by Amazon Macie. An error code and an error message are provided for each failed item. </p>
    #[serde(rename = "failedS3Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_s3_resources: Option<Vec<FailedS3Resource>>,
}

/// <p>Includes details about the failed S3 resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FailedS3Resource {
    /// <p>The status code of a failed item.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message of a failed item.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The failed S3 resources.</p>
    #[serde(rename = "failedItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_item: Option<S3Resource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListMemberAccountsRequest {
    /// <p>Use this parameter to indicate the maximum number of items that you want in the response. The default value is 250. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this parameter when paginating results. Set the value of this parameter to null on your first call to the ListMemberAccounts action. Subsequent calls to the action fill nextToken in the request with the value of nextToken from the previous response to continue listing data. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListMemberAccountsResult {
    /// <p>A list of the Amazon Macie member accounts returned by the action. The current master account is also included in this list. </p>
    #[serde(rename = "memberAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_accounts: Option<Vec<MemberAccount>>,
    /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListS3ResourcesRequest {
    /// <p>Use this parameter to indicate the maximum number of items that you want in the response. The default value is 250. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The Amazon Macie member account ID whose associated S3 resources you want to list. </p>
    #[serde(rename = "memberAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_id: Option<String>,
    /// <p>Use this parameter when paginating results. Set its value to null on your first call to the ListS3Resources action. Subsequent calls to the action fill nextToken in the request with the value of nextToken from the previous response to continue listing data. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListS3ResourcesResult {
    /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the associated S3 resources returned by the action.</p>
    #[serde(rename = "s3Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_resources: Option<Vec<S3ResourceClassification>>,
}

/// <p>Contains information about the Amazon Macie member account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MemberAccount {
    /// <p>The AWS account ID of the Amazon Macie member account.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

/// <p>Contains information about the S3 resource. This data type is used as a request parameter in the DisassociateS3Resources action and can be used as a response parameter in the AssociateS3Resources and UpdateS3Resources actions. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Resource {
    /// <p>The name of the S3 bucket.</p>
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// <p>The prefix of the S3 bucket. </p>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <p>The S3 resources that you want to associate with Amazon Macie for monitoring and data classification. This data type is used as a request parameter in the AssociateS3Resources action and a response parameter in the ListS3Resources action. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3ResourceClassification {
    /// <p>The name of the S3 bucket that you want to associate with Amazon Macie.</p>
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// <p>The classification type that you want to specify for the resource associated with Amazon Macie. </p>
    #[serde(rename = "classificationType")]
    pub classification_type: ClassificationType,
    /// <p>The prefix of the S3 bucket that you want to associate with Amazon Macie.</p>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <p>The S3 resources whose classification types you want to update. This data type is used as a request parameter in the UpdateS3Resources action. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct S3ResourceClassificationUpdate {
    /// <p>The name of the S3 bucket whose classification types you want to update.</p>
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// <p>The classification type that you want to update for the resource associated with Amazon Macie. </p>
    #[serde(rename = "classificationTypeUpdate")]
    pub classification_type_update: ClassificationTypeUpdate,
    /// <p>The prefix of the S3 bucket whose classification types you want to update.</p>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateS3ResourcesRequest {
    /// <p>The AWS ID of the Amazon Macie member account whose S3 resources' classification types you want to update. </p>
    #[serde(rename = "memberAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_id: Option<String>,
    /// <p>The S3 resources whose classification types you want to update.</p>
    #[serde(rename = "s3ResourcesUpdate")]
    pub s_3_resources_update: Vec<S3ResourceClassificationUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateS3ResourcesResult {
    /// <p>The S3 resources whose classification types can't be updated. An error code and an error message are provided for each failed item. </p>
    #[serde(rename = "failedS3Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_s3_resources: Option<Vec<FailedS3Resource>>,
}

/// Errors returned by AssociateMemberAccount
#[derive(Debug, PartialEq)]
pub enum AssociateMemberAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded. </p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateMemberAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateMemberAccountError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalException" => {
                    return AssociateMemberAccountError::Internal(String::from(error_message))
                }
                "InvalidInputException" => {
                    return AssociateMemberAccountError::InvalidInput(String::from(error_message))
                }
                "LimitExceededException" => {
                    return AssociateMemberAccountError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return AssociateMemberAccountError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AssociateMemberAccountError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateMemberAccountError {
    fn from(err: serde_json::error::Error) -> AssociateMemberAccountError {
        AssociateMemberAccountError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateMemberAccountError {
    fn from(err: CredentialsError) -> AssociateMemberAccountError {
        AssociateMemberAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateMemberAccountError {
    fn from(err: HttpDispatchError) -> AssociateMemberAccountError {
        AssociateMemberAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateMemberAccountError {
    fn from(err: io::Error) -> AssociateMemberAccountError {
        AssociateMemberAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateMemberAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateMemberAccountError {
    fn description(&self) -> &str {
        match *self {
            AssociateMemberAccountError::Internal(ref cause) => cause,
            AssociateMemberAccountError::InvalidInput(ref cause) => cause,
            AssociateMemberAccountError::LimitExceeded(ref cause) => cause,
            AssociateMemberAccountError::Validation(ref cause) => cause,
            AssociateMemberAccountError::Credentials(ref err) => err.description(),
            AssociateMemberAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateMemberAccountError::ParseError(ref cause) => cause,
            AssociateMemberAccountError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateS3Resources
#[derive(Debug, PartialEq)]
pub enum AssociateS3ResourcesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded. </p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateS3ResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateS3ResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return AssociateS3ResourcesError::AccessDenied(String::from(error_message))
                }
                "InternalException" => {
                    return AssociateS3ResourcesError::Internal(String::from(error_message))
                }
                "InvalidInputException" => {
                    return AssociateS3ResourcesError::InvalidInput(String::from(error_message))
                }
                "LimitExceededException" => {
                    return AssociateS3ResourcesError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return AssociateS3ResourcesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AssociateS3ResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateS3ResourcesError {
    fn from(err: serde_json::error::Error) -> AssociateS3ResourcesError {
        AssociateS3ResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateS3ResourcesError {
    fn from(err: CredentialsError) -> AssociateS3ResourcesError {
        AssociateS3ResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateS3ResourcesError {
    fn from(err: HttpDispatchError) -> AssociateS3ResourcesError {
        AssociateS3ResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateS3ResourcesError {
    fn from(err: io::Error) -> AssociateS3ResourcesError {
        AssociateS3ResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateS3ResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateS3ResourcesError {
    fn description(&self) -> &str {
        match *self {
            AssociateS3ResourcesError::AccessDenied(ref cause) => cause,
            AssociateS3ResourcesError::Internal(ref cause) => cause,
            AssociateS3ResourcesError::InvalidInput(ref cause) => cause,
            AssociateS3ResourcesError::LimitExceeded(ref cause) => cause,
            AssociateS3ResourcesError::Validation(ref cause) => cause,
            AssociateS3ResourcesError::Credentials(ref err) => err.description(),
            AssociateS3ResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateS3ResourcesError::ParseError(ref cause) => cause,
            AssociateS3ResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateMemberAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateMemberAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateMemberAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateMemberAccountError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalException" => {
                    return DisassociateMemberAccountError::Internal(String::from(error_message))
                }
                "InvalidInputException" => {
                    return DisassociateMemberAccountError::InvalidInput(String::from(error_message))
                }
                "ValidationException" => {
                    return DisassociateMemberAccountError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DisassociateMemberAccountError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateMemberAccountError {
    fn from(err: serde_json::error::Error) -> DisassociateMemberAccountError {
        DisassociateMemberAccountError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateMemberAccountError {
    fn from(err: CredentialsError) -> DisassociateMemberAccountError {
        DisassociateMemberAccountError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateMemberAccountError {
    fn from(err: HttpDispatchError) -> DisassociateMemberAccountError {
        DisassociateMemberAccountError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateMemberAccountError {
    fn from(err: io::Error) -> DisassociateMemberAccountError {
        DisassociateMemberAccountError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateMemberAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateMemberAccountError {
    fn description(&self) -> &str {
        match *self {
            DisassociateMemberAccountError::Internal(ref cause) => cause,
            DisassociateMemberAccountError::InvalidInput(ref cause) => cause,
            DisassociateMemberAccountError::Validation(ref cause) => cause,
            DisassociateMemberAccountError::Credentials(ref err) => err.description(),
            DisassociateMemberAccountError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateMemberAccountError::ParseError(ref cause) => cause,
            DisassociateMemberAccountError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateS3Resources
#[derive(Debug, PartialEq)]
pub enum DisassociateS3ResourcesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateS3ResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateS3ResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return DisassociateS3ResourcesError::AccessDenied(String::from(error_message))
                }
                "InternalException" => {
                    return DisassociateS3ResourcesError::Internal(String::from(error_message))
                }
                "InvalidInputException" => {
                    return DisassociateS3ResourcesError::InvalidInput(String::from(error_message))
                }
                "ValidationException" => {
                    return DisassociateS3ResourcesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DisassociateS3ResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateS3ResourcesError {
    fn from(err: serde_json::error::Error) -> DisassociateS3ResourcesError {
        DisassociateS3ResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateS3ResourcesError {
    fn from(err: CredentialsError) -> DisassociateS3ResourcesError {
        DisassociateS3ResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateS3ResourcesError {
    fn from(err: HttpDispatchError) -> DisassociateS3ResourcesError {
        DisassociateS3ResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateS3ResourcesError {
    fn from(err: io::Error) -> DisassociateS3ResourcesError {
        DisassociateS3ResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateS3ResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateS3ResourcesError {
    fn description(&self) -> &str {
        match *self {
            DisassociateS3ResourcesError::AccessDenied(ref cause) => cause,
            DisassociateS3ResourcesError::Internal(ref cause) => cause,
            DisassociateS3ResourcesError::InvalidInput(ref cause) => cause,
            DisassociateS3ResourcesError::Validation(ref cause) => cause,
            DisassociateS3ResourcesError::Credentials(ref err) => err.description(),
            DisassociateS3ResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateS3ResourcesError::ParseError(ref cause) => cause,
            DisassociateS3ResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListMemberAccounts
#[derive(Debug, PartialEq)]
pub enum ListMemberAccountsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListMemberAccountsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListMemberAccountsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalException" => {
                    return ListMemberAccountsError::Internal(String::from(error_message))
                }
                "InvalidInputException" => {
                    return ListMemberAccountsError::InvalidInput(String::from(error_message))
                }
                "ValidationException" => {
                    return ListMemberAccountsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListMemberAccountsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListMemberAccountsError {
    fn from(err: serde_json::error::Error) -> ListMemberAccountsError {
        ListMemberAccountsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListMemberAccountsError {
    fn from(err: CredentialsError) -> ListMemberAccountsError {
        ListMemberAccountsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListMemberAccountsError {
    fn from(err: HttpDispatchError) -> ListMemberAccountsError {
        ListMemberAccountsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListMemberAccountsError {
    fn from(err: io::Error) -> ListMemberAccountsError {
        ListMemberAccountsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListMemberAccountsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMemberAccountsError {
    fn description(&self) -> &str {
        match *self {
            ListMemberAccountsError::Internal(ref cause) => cause,
            ListMemberAccountsError::InvalidInput(ref cause) => cause,
            ListMemberAccountsError::Validation(ref cause) => cause,
            ListMemberAccountsError::Credentials(ref err) => err.description(),
            ListMemberAccountsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListMemberAccountsError::ParseError(ref cause) => cause,
            ListMemberAccountsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListS3Resources
#[derive(Debug, PartialEq)]
pub enum ListS3ResourcesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListS3ResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListS3ResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return ListS3ResourcesError::AccessDenied(String::from(error_message))
                }
                "InternalException" => {
                    return ListS3ResourcesError::Internal(String::from(error_message))
                }
                "InvalidInputException" => {
                    return ListS3ResourcesError::InvalidInput(String::from(error_message))
                }
                "ValidationException" => {
                    return ListS3ResourcesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListS3ResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListS3ResourcesError {
    fn from(err: serde_json::error::Error) -> ListS3ResourcesError {
        ListS3ResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListS3ResourcesError {
    fn from(err: CredentialsError) -> ListS3ResourcesError {
        ListS3ResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListS3ResourcesError {
    fn from(err: HttpDispatchError) -> ListS3ResourcesError {
        ListS3ResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListS3ResourcesError {
    fn from(err: io::Error) -> ListS3ResourcesError {
        ListS3ResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListS3ResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListS3ResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListS3ResourcesError::AccessDenied(ref cause) => cause,
            ListS3ResourcesError::Internal(ref cause) => cause,
            ListS3ResourcesError::InvalidInput(ref cause) => cause,
            ListS3ResourcesError::Validation(ref cause) => cause,
            ListS3ResourcesError::Credentials(ref err) => err.description(),
            ListS3ResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListS3ResourcesError::ParseError(ref cause) => cause,
            ListS3ResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateS3Resources
#[derive(Debug, PartialEq)]
pub enum UpdateS3ResourcesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateS3ResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateS3ResourcesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "AccessDeniedException" => {
                    return UpdateS3ResourcesError::AccessDenied(String::from(error_message))
                }
                "InternalException" => {
                    return UpdateS3ResourcesError::Internal(String::from(error_message))
                }
                "InvalidInputException" => {
                    return UpdateS3ResourcesError::InvalidInput(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateS3ResourcesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateS3ResourcesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateS3ResourcesError {
    fn from(err: serde_json::error::Error) -> UpdateS3ResourcesError {
        UpdateS3ResourcesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateS3ResourcesError {
    fn from(err: CredentialsError) -> UpdateS3ResourcesError {
        UpdateS3ResourcesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateS3ResourcesError {
    fn from(err: HttpDispatchError) -> UpdateS3ResourcesError {
        UpdateS3ResourcesError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateS3ResourcesError {
    fn from(err: io::Error) -> UpdateS3ResourcesError {
        UpdateS3ResourcesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateS3ResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateS3ResourcesError {
    fn description(&self) -> &str {
        match *self {
            UpdateS3ResourcesError::AccessDenied(ref cause) => cause,
            UpdateS3ResourcesError::Internal(ref cause) => cause,
            UpdateS3ResourcesError::InvalidInput(ref cause) => cause,
            UpdateS3ResourcesError::Validation(ref cause) => cause,
            UpdateS3ResourcesError::Credentials(ref err) => err.description(),
            UpdateS3ResourcesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateS3ResourcesError::ParseError(ref cause) => cause,
            UpdateS3ResourcesError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Macie API. Amazon Macie clients implement this trait.
pub trait Macie {
    /// <p>Associates a specified AWS account with Amazon Macie as a member account.</p>
    fn associate_member_account(
        &self,
        input: AssociateMemberAccountRequest,
    ) -> RusotoFuture<(), AssociateMemberAccountError>;

    /// <p>Associates specified S3 resources with Amazon Macie for monitoring and data classification. If memberAccountId isn't specified, the action associates specified S3 resources with Macie for the current master account. If memberAccountId is specified, the action associates specified S3 resources with Macie for the specified member account. </p>
    fn associate_s3_resources(
        &self,
        input: AssociateS3ResourcesRequest,
    ) -> RusotoFuture<AssociateS3ResourcesResult, AssociateS3ResourcesError>;

    /// <p>Removes the specified member account from Amazon Macie.</p>
    fn disassociate_member_account(
        &self,
        input: DisassociateMemberAccountRequest,
    ) -> RusotoFuture<(), DisassociateMemberAccountError>;

    /// <p>Removes specified S3 resources from being monitored by Amazon Macie. If memberAccountId isn't specified, the action removes specified S3 resources from Macie for the current master account. If memberAccountId is specified, the action removes specified S3 resources from Macie for the specified member account.</p>
    fn disassociate_s3_resources(
        &self,
        input: DisassociateS3ResourcesRequest,
    ) -> RusotoFuture<DisassociateS3ResourcesResult, DisassociateS3ResourcesError>;

    /// <p>Lists all Amazon Macie member accounts for the current Amazon Macie master account.</p>
    fn list_member_accounts(
        &self,
        input: ListMemberAccountsRequest,
    ) -> RusotoFuture<ListMemberAccountsResult, ListMemberAccountsError>;

    /// <p>Lists all the S3 resources associated with Amazon Macie. If memberAccountId isn't specified, the action lists the S3 resources associated with Amazon Macie for the current master account. If memberAccountId is specified, the action lists the S3 resources associated with Amazon Macie for the specified member account. </p>
    fn list_s3_resources(
        &self,
        input: ListS3ResourcesRequest,
    ) -> RusotoFuture<ListS3ResourcesResult, ListS3ResourcesError>;

    /// <p>Updates the classification types for the specified S3 resources. If memberAccountId isn't specified, the action updates the classification types of the S3 resources associated with Amazon Macie for the current master account. If memberAccountId is specified, the action updates the classification types of the S3 resources associated with Amazon Macie for the specified member account. </p>
    fn update_s3_resources(
        &self,
        input: UpdateS3ResourcesRequest,
    ) -> RusotoFuture<UpdateS3ResourcesResult, UpdateS3ResourcesError>;
}
/// A client for the Amazon Macie API.
pub struct MacieClient {
    client: Client,
    region: region::Region,
}

impl MacieClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MacieClient {
        MacieClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MacieClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        MacieClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Macie for MacieClient {
    /// <p>Associates a specified AWS account with Amazon Macie as a member account.</p>
    fn associate_member_account(
        &self,
        input: AssociateMemberAccountRequest,
    ) -> RusotoFuture<(), AssociateMemberAccountError> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.AssociateMemberAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AssociateMemberAccountError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Associates specified S3 resources with Amazon Macie for monitoring and data classification. If memberAccountId isn't specified, the action associates specified S3 resources with Macie for the current master account. If memberAccountId is specified, the action associates specified S3 resources with Macie for the specified member account. </p>
    fn associate_s3_resources(
        &self,
        input: AssociateS3ResourcesRequest,
    ) -> RusotoFuture<AssociateS3ResourcesResult, AssociateS3ResourcesError> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.AssociateS3Resources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AssociateS3ResourcesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AssociateS3ResourcesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Removes the specified member account from Amazon Macie.</p>
    fn disassociate_member_account(
        &self,
        input: DisassociateMemberAccountRequest,
    ) -> RusotoFuture<(), DisassociateMemberAccountError> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.DisassociateMemberAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateMemberAccountError::from_response(response))
                }))
            }
        })
    }

    /// <p>Removes specified S3 resources from being monitored by Amazon Macie. If memberAccountId isn't specified, the action removes specified S3 resources from Macie for the current master account. If memberAccountId is specified, the action removes specified S3 resources from Macie for the specified member account.</p>
    fn disassociate_s3_resources(
        &self,
        input: DisassociateS3ResourcesRequest,
    ) -> RusotoFuture<DisassociateS3ResourcesResult, DisassociateS3ResourcesError> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.DisassociateS3Resources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateS3ResourcesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateS3ResourcesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists all Amazon Macie member accounts for the current Amazon Macie master account.</p>
    fn list_member_accounts(
        &self,
        input: ListMemberAccountsRequest,
    ) -> RusotoFuture<ListMemberAccountsResult, ListMemberAccountsError> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.ListMemberAccounts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListMemberAccountsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListMemberAccountsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all the S3 resources associated with Amazon Macie. If memberAccountId isn't specified, the action lists the S3 resources associated with Amazon Macie for the current master account. If memberAccountId is specified, the action lists the S3 resources associated with Amazon Macie for the specified member account. </p>
    fn list_s3_resources(
        &self,
        input: ListS3ResourcesRequest,
    ) -> RusotoFuture<ListS3ResourcesResult, ListS3ResourcesError> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.ListS3Resources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListS3ResourcesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListS3ResourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the classification types for the specified S3 resources. If memberAccountId isn't specified, the action updates the classification types of the S3 resources associated with Amazon Macie for the current master account. If memberAccountId is specified, the action updates the classification types of the S3 resources associated with Amazon Macie for the specified member account. </p>
    fn update_s3_resources(
        &self,
        input: UpdateS3ResourcesRequest,
    ) -> RusotoFuture<UpdateS3ResourcesResult, UpdateS3ResourcesError> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.UpdateS3Resources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateS3ResourcesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateS3ResourcesError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
