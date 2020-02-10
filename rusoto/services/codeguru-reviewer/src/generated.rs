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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use futures::prelude::*;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
use std::pin::Pin;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateRepositoryRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> <p>If you want to add a new repository association, this parameter specifies a unique identifier for the new repository association that helps ensure idempotency.</p> <p>If you use the AWS CLI or one of the AWS SDK to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes that in the request. If you don't use the SDK and instead generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a ClientRequestToken yourself for new versions and include that value in the request.</p> <p>You typically only need to interact with this value if you implement your own retry logic and want to ensure that a given repository association is not created twice. We recommend that you generate a UUID-type value to ensure uniqueness within the specified repository association.</p> <p>Amazon CodeGuru Reviewer uses this value to prevent the accidental creation of duplicate repository associations if there are failures and retries. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The repository to associate.</p>
    #[serde(rename = "Repository")]
    pub repository: Repository,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateRepositoryResponse {
    /// <p>Information about the repository association.</p>
    #[serde(rename = "RepositoryAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
}

/// <p>Information about an AWS CodeCommit repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CodeCommitRepository {
    /// <p>The name of the AWS CodeCommit repository.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRepositoryAssociationRequest {
    /// <p>The Amazon Resource Name (ARN) identifying the association.</p>
    #[serde(rename = "AssociationArn")]
    pub association_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRepositoryAssociationResponse {
    /// <p>Information about the repository association.</p>
    #[serde(rename = "RepositoryAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateRepositoryRequest {
    /// <p>The Amazon Resource Name (ARN) identifying the association.</p>
    #[serde(rename = "AssociationArn")]
    pub association_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateRepositoryResponse {
    /// <p>Information about the disassociated repository.</p>
    #[serde(rename = "RepositoryAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRepositoryAssociationsRequest {
    /// <p>The maximum number of repository association results returned by <code>ListRepositoryAssociations</code> in paginated output. When this parameter is used, <code>ListRepositoryAssociations</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRepositoryAssociations</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListRepositoryAssociations</code> returns up to 100 results and a <code>nextToken</code> value if applicable. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>List of names to use as a filter.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListRepositoryAssociations</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. </p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of owners to use as a filter. For AWS CodeCommit, the owner is the AWS account id. For GitHub, it is the GitHub account name.</p>
    #[serde(rename = "Owners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    /// <p>List of provider types to use as a filter.</p>
    #[serde(rename = "ProviderTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_types: Option<Vec<String>>,
    /// <p>List of states to use as a filter.</p>
    #[serde(rename = "States")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRepositoryAssociationsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListRecommendations</code> request. When the results of a <code>ListRecommendations</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of repository associations that meet the criteria of the request.</p>
    #[serde(rename = "RepositoryAssociationSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association_summaries: Option<Vec<RepositoryAssociationSummary>>,
}

/// <p>Information about a repository.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Repository {
    /// <p>Information about an AWS CodeCommit repository.</p>
    #[serde(rename = "CodeCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_commit: Option<CodeCommitRepository>,
}

/// <p>Information about a repository association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RepositoryAssociation {
    /// <p>The Amazon Resource Name (ARN) identifying the repository association.</p>
    #[serde(rename = "AssociationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    /// <p>The id of the repository association.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the repository association was created.</p>
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the repository association was last updated.</p>
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    /// <p>The name of the repository.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the repository.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The provider type of the repository association.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p>The state of the repository association.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of why the repository association is in the current state.</p>
    #[serde(rename = "StateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// <p>Information about a repository association.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RepositoryAssociationSummary {
    /// <p>The Amazon Resource Name (ARN) identifying the repository association.</p>
    #[serde(rename = "AssociationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    /// <p>The repository association ID.</p>
    #[serde(rename = "AssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, since the repository association was last updated. </p>
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    /// <p>The name of the repository association.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The owner of the repository association.</p>
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The provider type of the repository association.</p>
    #[serde(rename = "ProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    /// <p><p>The state of the repository association.</p> <dl> <dt>Associated</dt> <dd> <p>Amazon CodeGuru Reviewer is associated with the repository. </p> </dd> <dt>Associating</dt> <dd> <p>The association is in progress. </p> </dd> <dt>Failed</dt> <dd> <p>The association failed. For more information about troubleshooting (or why it failed), see [troubleshooting topic]. </p> </dd> <dt>Disassociating</dt> <dd> <p>Amazon CodeGuru Reviewer is in the process of disassociating with the repository. </p> </dd> </dl></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Errors returned by AssociateRepository
#[derive(Debug, PartialEq)]
pub enum AssociateRepositoryError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl AssociateRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateRepositoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AssociateRepositoryError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(AssociateRepositoryError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(AssociateRepositoryError::InternalServer(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AssociateRepositoryError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateRepositoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AssociateRepositoryError::Conflict(ref cause) => write!(f, "{}", cause),
            AssociateRepositoryError::InternalServer(ref cause) => write!(f, "{}", cause),
            AssociateRepositoryError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateRepositoryError {}
/// Errors returned by DescribeRepositoryAssociation
#[derive(Debug, PartialEq)]
pub enum DescribeRepositoryAssociationError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request was not found.</p>
    NotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeRepositoryAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRepositoryAssociationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeRepositoryAssociationError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeRepositoryAssociationError::InternalServer(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeRepositoryAssociationError::NotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeRepositoryAssociationError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRepositoryAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRepositoryAssociationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeRepositoryAssociationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeRepositoryAssociationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeRepositoryAssociationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRepositoryAssociationError {}
/// Errors returned by DisassociateRepository
#[derive(Debug, PartialEq)]
pub enum DisassociateRepositoryError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    Conflict(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource specified in the request was not found.</p>
    NotFound(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DisassociateRepositoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateRepositoryError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DisassociateRepositoryError::AccessDenied(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DisassociateRepositoryError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DisassociateRepositoryError::InternalServer(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisassociateRepositoryError::NotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DisassociateRepositoryError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateRepositoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateRepositoryError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisassociateRepositoryError::Conflict(ref cause) => write!(f, "{}", cause),
            DisassociateRepositoryError::InternalServer(ref cause) => write!(f, "{}", cause),
            DisassociateRepositoryError::NotFound(ref cause) => write!(f, "{}", cause),
            DisassociateRepositoryError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateRepositoryError {}
/// Errors returned by ListRepositoryAssociations
#[derive(Debug, PartialEq)]
pub enum ListRepositoryAssociationsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListRepositoryAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListRepositoryAssociationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListRepositoryAssociationsError::InternalServer(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListRepositoryAssociationsError::Throttling(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRepositoryAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRepositoryAssociationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListRepositoryAssociationsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRepositoryAssociationsError {}
/// Trait representing the capabilities of the CodeGuruReviewer API. CodeGuruReviewer clients implement this trait.
pub trait CodeGuruReviewer {
    /// <p>Associates an AWS CodeCommit repository with Amazon CodeGuru Reviewer. When you associate an AWS CodeCommit repository with Amazon CodeGuru Reviewer, Amazon CodeGuru Reviewer will provide recommendations for each pull request. You can view recommendations in the AWS CodeCommit repository.</p> <p>You can associate a GitHub repository using the Amazon CodeGuru Reviewer console.</p>
    fn associate_repository(
        &self,
        input: AssociateRepositoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        AssociateRepositoryResponse,
                        RusotoError<AssociateRepositoryError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Describes a repository association.</p>
    fn describe_repository_association(
        &self,
        input: DescribeRepositoryAssociationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeRepositoryAssociationResponse,
                        RusotoError<DescribeRepositoryAssociationError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Removes the association between Amazon CodeGuru Reviewer and a repository.</p>
    fn disassociate_repository(
        &self,
        input: DisassociateRepositoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DisassociateRepositoryResponse,
                        RusotoError<DisassociateRepositoryError>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    /// <p>Lists repository associations. You can optionally filter on one or more of the following recommendation properties: provider types, states, names, and owners.</p>
    fn list_repository_associations(
        &self,
        input: ListRepositoryAssociationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListRepositoryAssociationsResponse,
                        RusotoError<ListRepositoryAssociationsError>,
                    >,
                > + Send
                + 'static,
        >,
    >;
}
/// A client for the CodeGuruReviewer API.
#[derive(Clone)]
pub struct CodeGuruReviewerClient {
    client: Client,
    region: region::Region,
}

impl CodeGuruReviewerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CodeGuruReviewerClient {
        CodeGuruReviewerClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CodeGuruReviewerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CodeGuruReviewerClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CodeGuruReviewerClient {
        CodeGuruReviewerClient { client, region }
    }
}

impl CodeGuruReviewer for CodeGuruReviewerClient {
    /// <p>Associates an AWS CodeCommit repository with Amazon CodeGuru Reviewer. When you associate an AWS CodeCommit repository with Amazon CodeGuru Reviewer, Amazon CodeGuru Reviewer will provide recommendations for each pull request. You can view recommendations in the AWS CodeCommit repository.</p> <p>You can associate a GitHub repository using the Amazon CodeGuru Reviewer console.</p>
    fn associate_repository(
        &self,
        input: AssociateRepositoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        AssociateRepositoryResponse,
                        RusotoError<AssociateRepositoryError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/associations";

        let mut request =
            SignedRequest::new("POST", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<AssociateRepositoryResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(AssociateRepositoryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Describes a repository association.</p>
    fn describe_repository_association(
        &self,
        input: DescribeRepositoryAssociationRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DescribeRepositoryAssociationResponse,
                        RusotoError<DescribeRepositoryAssociationError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/associations/{association_arn}",
            association_arn = input.association_arn
        );

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DescribeRepositoryAssociationResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DescribeRepositoryAssociationError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Removes the association between Amazon CodeGuru Reviewer and a repository.</p>
    fn disassociate_repository(
        &self,
        input: DisassociateRepositoryRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        DisassociateRepositoryResponse,
                        RusotoError<DisassociateRepositoryError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = format!(
            "/associations/{association_arn}",
            association_arn = input.association_arn
        );

        let mut request =
            SignedRequest::new("DELETE", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<DisassociateRepositoryResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(DisassociateRepositoryError::from_response(response))
            }
        }
        .boxed()
    }

    /// <p>Lists repository associations. You can optionally filter on one or more of the following recommendation properties: provider types, states, names, and owners.</p>
    fn list_repository_associations(
        &self,
        input: ListRepositoryAssociationsRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        ListRepositoryAssociationsResponse,
                        RusotoError<ListRepositoryAssociationsError>,
                    >,
                > + Send
                + 'static,
        >,
    > {
        let request_uri = "/associations";

        let mut request =
            SignedRequest::new("GET", "codeguru-reviewer", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.names {
            for item in x.iter() {
                params.put("Name", item);
            }
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.owners {
            for item in x.iter() {
                params.put("Owner", item);
            }
        }
        if let Some(ref x) = input.provider_types {
            for item in x.iter() {
                params.put("ProviderType", item);
            }
        }
        if let Some(ref x) = input.states {
            for item in x.iter() {
                params.put("State", item);
            }
        }
        request.set_params(params);

        let fut = self.client.sign_and_dispatch(request);
        async move {
            let mut response = fut.await.map_err(RusotoError::from)?;
            if response.status.is_success() {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                let result = proto::json::ResponsePayload::new(&response)
                    .deserialize::<ListRepositoryAssociationsResponse, _>()?;

                Ok(result)
            } else {
                let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
                Err(ListRepositoryAssociationsError::from_response(response))
            }
        }
        .boxed()
    }
}
