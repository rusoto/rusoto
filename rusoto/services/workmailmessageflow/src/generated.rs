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
#[allow(unused_imports)]
use rusoto_core::pagination::{all_pages, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// see [WorkmailMessageFlow::get_raw_message_content]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRawMessageContentRequest {
    /// <p>The identifier of the email message to retrieve.</p>
    #[serde(rename = "messageId")]
    pub message_id: String,
}

/// see [WorkmailMessageFlow::get_raw_message_content]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetRawMessageContentResponse {
    /// <p>The raw content of the email message, in MIME format.</p>
    pub message_content: bytes::Bytes,
}

/// Errors returned by GetRawMessageContent
#[derive(Debug, PartialEq)]
pub enum GetRawMessageContentError {
    /// <p>The requested email message is not found.</p>
    ResourceNotFound(String),
}

impl GetRawMessageContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRawMessageContentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRawMessageContentError::ResourceNotFound(
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
impl fmt::Display for GetRawMessageContentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRawMessageContentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRawMessageContentError {}
/// Trait representing the capabilities of the Amazon WorkMail Message Flow API. Amazon WorkMail Message Flow clients implement this trait.
#[async_trait]
pub trait WorkmailMessageFlow: Clone + Sync + Send + 'static {
    /// <p>Retrieves the raw content of an in-transit email message, in MIME format. </p>
    async fn get_raw_message_content(
        &self,
        input: GetRawMessageContentRequest,
    ) -> Result<GetRawMessageContentResponse, RusotoError<GetRawMessageContentError>>;
}
/// A client for the Amazon WorkMail Message Flow API.
#[derive(Clone)]
pub struct WorkmailMessageFlowClient {
    client: Client,
    region: region::Region,
}

impl WorkmailMessageFlowClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WorkmailMessageFlowClient {
        WorkmailMessageFlowClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WorkmailMessageFlowClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        WorkmailMessageFlowClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> WorkmailMessageFlowClient {
        WorkmailMessageFlowClient { client, region }
    }
}

#[async_trait]
impl WorkmailMessageFlow for WorkmailMessageFlowClient {
    /// <p>Retrieves the raw content of an in-transit email message, in MIME format. </p>
    #[allow(unused_mut)]
    async fn get_raw_message_content(
        &self,
        input: GetRawMessageContentRequest,
    ) -> Result<GetRawMessageContentResponse, RusotoError<GetRawMessageContentError>> {
        let request_uri = format!("/messages/{message_id}", message_id = input.message_id);

        let mut request =
            SignedRequest::new("GET", "workmailmessageflow", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetRawMessageContentResponse::default();
            result.message_content = response.body;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRawMessageContentError::from_response(response))
        }
    }
}
