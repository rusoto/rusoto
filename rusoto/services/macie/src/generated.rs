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

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateMemberAccountRequest {
    /// <p>The ID of the AWS account that you want to associate with Amazon Macie as a member account.</p>
    #[serde(rename = "memberAccountId")]
    pub member_account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateMemberAccountRequest {
    /// <p>The ID of the member account that you want to remove from Amazon Macie.</p>
    #[serde(rename = "memberAccountId")]
    pub member_account_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateS3ResourcesResult {
    /// <p>S3 resources that couldn't be removed from being monitored and classified by Amazon Macie. An error code and an error message are provided for each failed item. </p>
    #[serde(rename = "failedS3Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_s3_resources: Option<Vec<FailedS3Resource>>,
}

/// <p>Includes details about the failed S3 resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl AssociateMemberAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateMemberAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(AssociateMemberAccountError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AssociateMemberAccountError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateMemberAccountError::LimitExceeded(
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
impl fmt::Display for AssociateMemberAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateMemberAccountError::Internal(ref cause) => write!(f, "{}", cause),
            AssociateMemberAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AssociateMemberAccountError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateMemberAccountError {}
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
}

impl AssociateS3ResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateS3ResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateS3ResourcesError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(AssociateS3ResourcesError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AssociateS3ResourcesError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(AssociateS3ResourcesError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AssociateS3ResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateS3ResourcesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociateS3ResourcesError::Internal(ref cause) => write!(f, "{}", cause),
            AssociateS3ResourcesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AssociateS3ResourcesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateS3ResourcesError {}
/// Errors returned by DisassociateMemberAccount
#[derive(Debug, PartialEq)]
pub enum DisassociateMemberAccountError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
}

impl DisassociateMemberAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateMemberAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DisassociateMemberAccountError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisassociateMemberAccountError::InvalidInput(
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
impl fmt::Display for DisassociateMemberAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateMemberAccountError::Internal(ref cause) => write!(f, "{}", cause),
            DisassociateMemberAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateMemberAccountError {}
/// Errors returned by DisassociateS3Resources
#[derive(Debug, PartialEq)]
pub enum DisassociateS3ResourcesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
}

impl DisassociateS3ResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateS3ResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateS3ResourcesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(DisassociateS3ResourcesError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisassociateS3ResourcesError::InvalidInput(
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
impl fmt::Display for DisassociateS3ResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateS3ResourcesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisassociateS3ResourcesError::Internal(ref cause) => write!(f, "{}", cause),
            DisassociateS3ResourcesError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateS3ResourcesError {}
/// Errors returned by ListMemberAccounts
#[derive(Debug, PartialEq)]
pub enum ListMemberAccountsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
}

impl ListMemberAccountsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMemberAccountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(ListMemberAccountsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListMemberAccountsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListMemberAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMemberAccountsError::Internal(ref cause) => write!(f, "{}", cause),
            ListMemberAccountsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMemberAccountsError {}
/// Errors returned by ListS3Resources
#[derive(Debug, PartialEq)]
pub enum ListS3ResourcesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
}

impl ListS3ResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListS3ResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListS3ResourcesError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListS3ResourcesError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListS3ResourcesError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListS3ResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListS3ResourcesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListS3ResourcesError::Internal(ref cause) => write!(f, "{}", cause),
            ListS3ResourcesError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListS3ResourcesError {}
/// Errors returned by UpdateS3Resources
#[derive(Debug, PartialEq)]
pub enum UpdateS3ResourcesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter. </p>
    InvalidInput(String),
}

impl UpdateS3ResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateS3ResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateS3ResourcesError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(UpdateS3ResourcesError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateS3ResourcesError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateS3ResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateS3ResourcesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateS3ResourcesError::Internal(ref cause) => write!(f, "{}", cause),
            UpdateS3ResourcesError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateS3ResourcesError {}
/// Trait representing the capabilities of the Amazon Macie API. Amazon Macie clients implement this trait.
#[async_trait]
pub trait Macie {
    /// <p>Associates a specified AWS account with Amazon Macie as a member account.</p>
    async fn associate_member_account(
        &self,
        input: AssociateMemberAccountRequest,
    ) -> Result<(), RusotoError<AssociateMemberAccountError>>;

    /// <p>Associates specified S3 resources with Amazon Macie for monitoring and data classification. If memberAccountId isn't specified, the action associates specified S3 resources with Macie for the current master account. If memberAccountId is specified, the action associates specified S3 resources with Macie for the specified member account. </p>
    async fn associate_s3_resources(
        &self,
        input: AssociateS3ResourcesRequest,
    ) -> Result<AssociateS3ResourcesResult, RusotoError<AssociateS3ResourcesError>>;

    /// <p>Removes the specified member account from Amazon Macie.</p>
    async fn disassociate_member_account(
        &self,
        input: DisassociateMemberAccountRequest,
    ) -> Result<(), RusotoError<DisassociateMemberAccountError>>;

    /// <p>Removes specified S3 resources from being monitored by Amazon Macie. If memberAccountId isn't specified, the action removes specified S3 resources from Macie for the current master account. If memberAccountId is specified, the action removes specified S3 resources from Macie for the specified member account.</p>
    async fn disassociate_s3_resources(
        &self,
        input: DisassociateS3ResourcesRequest,
    ) -> Result<DisassociateS3ResourcesResult, RusotoError<DisassociateS3ResourcesError>>;

    /// <p>Lists all Amazon Macie member accounts for the current Amazon Macie master account.</p>
    async fn list_member_accounts(
        &self,
        input: ListMemberAccountsRequest,
    ) -> Result<ListMemberAccountsResult, RusotoError<ListMemberAccountsError>>;

    /// <p>Lists all the S3 resources associated with Amazon Macie. If memberAccountId isn't specified, the action lists the S3 resources associated with Amazon Macie for the current master account. If memberAccountId is specified, the action lists the S3 resources associated with Amazon Macie for the specified member account. </p>
    async fn list_s3_resources(
        &self,
        input: ListS3ResourcesRequest,
    ) -> Result<ListS3ResourcesResult, RusotoError<ListS3ResourcesError>>;

    /// <p>Updates the classification types for the specified S3 resources. If memberAccountId isn't specified, the action updates the classification types of the S3 resources associated with Amazon Macie for the current master account. If memberAccountId is specified, the action updates the classification types of the S3 resources associated with Amazon Macie for the specified member account. </p>
    async fn update_s3_resources(
        &self,
        input: UpdateS3ResourcesRequest,
    ) -> Result<UpdateS3ResourcesResult, RusotoError<UpdateS3ResourcesError>>;
}
/// A client for the Amazon Macie API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MacieClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MacieClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MacieClient {
        MacieClient { client, region }
    }
}

#[async_trait]
impl Macie for MacieClient {
    /// <p>Associates a specified AWS account with Amazon Macie as a member account.</p>
    async fn associate_member_account(
        &self,
        input: AssociateMemberAccountRequest,
    ) -> Result<(), RusotoError<AssociateMemberAccountError>> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.AssociateMemberAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateMemberAccountError::from_response(response))
        }
    }

    /// <p>Associates specified S3 resources with Amazon Macie for monitoring and data classification. If memberAccountId isn't specified, the action associates specified S3 resources with Macie for the current master account. If memberAccountId is specified, the action associates specified S3 resources with Macie for the specified member account. </p>
    async fn associate_s3_resources(
        &self,
        input: AssociateS3ResourcesRequest,
    ) -> Result<AssociateS3ResourcesResult, RusotoError<AssociateS3ResourcesError>> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.AssociateS3Resources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateS3ResourcesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateS3ResourcesError::from_response(response))
        }
    }

    /// <p>Removes the specified member account from Amazon Macie.</p>
    async fn disassociate_member_account(
        &self,
        input: DisassociateMemberAccountRequest,
    ) -> Result<(), RusotoError<DisassociateMemberAccountError>> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.DisassociateMemberAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateMemberAccountError::from_response(response))
        }
    }

    /// <p>Removes specified S3 resources from being monitored by Amazon Macie. If memberAccountId isn't specified, the action removes specified S3 resources from Macie for the current master account. If memberAccountId is specified, the action removes specified S3 resources from Macie for the specified member account.</p>
    async fn disassociate_s3_resources(
        &self,
        input: DisassociateS3ResourcesRequest,
    ) -> Result<DisassociateS3ResourcesResult, RusotoError<DisassociateS3ResourcesError>> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.DisassociateS3Resources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateS3ResourcesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateS3ResourcesError::from_response(response))
        }
    }

    /// <p>Lists all Amazon Macie member accounts for the current Amazon Macie master account.</p>
    async fn list_member_accounts(
        &self,
        input: ListMemberAccountsRequest,
    ) -> Result<ListMemberAccountsResult, RusotoError<ListMemberAccountsError>> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.ListMemberAccounts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMemberAccountsResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListMemberAccountsError::from_response(response))
        }
    }

    /// <p>Lists all the S3 resources associated with Amazon Macie. If memberAccountId isn't specified, the action lists the S3 resources associated with Amazon Macie for the current master account. If memberAccountId is specified, the action lists the S3 resources associated with Amazon Macie for the specified member account. </p>
    async fn list_s3_resources(
        &self,
        input: ListS3ResourcesRequest,
    ) -> Result<ListS3ResourcesResult, RusotoError<ListS3ResourcesError>> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.ListS3Resources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListS3ResourcesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListS3ResourcesError::from_response(response))
        }
    }

    /// <p>Updates the classification types for the specified S3 resources. If memberAccountId isn't specified, the action updates the classification types of the S3 resources associated with Amazon Macie for the current master account. If memberAccountId is specified, the action updates the classification types of the S3 resources associated with Amazon Macie for the specified member account. </p>
    async fn update_s3_resources(
        &self,
        input: UpdateS3ResourcesRequest,
    ) -> Result<UpdateS3ResourcesResult, RusotoError<UpdateS3ResourcesError>> {
        let mut request = SignedRequest::new("POST", "macie", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "MacieService.UpdateS3Resources");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateS3ResourcesResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateS3ResourcesError::from_response(response))
        }
    }
}
