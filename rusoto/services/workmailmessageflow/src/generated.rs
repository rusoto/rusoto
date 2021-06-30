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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRawMessageContentRequest {
    /// <p>The identifier of the email message to retrieve.</p>
    #[serde(rename = "messageId")]
    pub message_id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetRawMessageContentResponse {
    /// <p>The raw content of the email message, in MIME format.</p>
    pub message_content: bytes::Bytes,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRawMessageContentRequest {
    /// <p>Describes the raw message content of the updated email message.</p>
    #[serde(rename = "content")]
    pub content: RawMessageContent,
    /// <p>The identifier of the email message being updated.</p>
    #[serde(rename = "messageId")]
    pub message_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRawMessageContentResponse {}

/// <p><p>Provides the MIME content of the updated email message as an S3 object. All MIME content must meet the following criteria:</p> <ul> <li> <p>Each part of a multipart MIME message must be formatted properly.</p> </li> <li> <p>Attachments must be of a content type that Amazon SES supports. For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/mime-types-appendix.html">Unsupported Attachment Types</a>.</p> </li> <li> <p>If any of the MIME parts in a message contain content that is outside of the 7-bit ASCII character range, we recommend encoding that content.</p> </li> <li> <p>Per <a href="https://tools.ietf.org/html/rfc5321#section-4.5.3.1.6">RFC 5321</a>, the maximum length of each line of text, including the &lt;CRLF&gt;, must not exceed 1,000 characters.</p> </li> <li> <p>The message must contain all the required header fields. Check the returned error message for more information.</p> </li> <li> <p>The value of immutable headers must remain unchanged. Check the returned error message for more information.</p> </li> <li> <p>Certain unique headers can only appear once. Check the returned error message for more information.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RawMessageContent {
    /// <p>The S3 reference of an email message.</p>
    #[serde(rename = "s3Reference")]
    pub s_3_reference: S3Reference,
}

/// <p><p>Amazon S3 object representing the updated message content, in MIME format.</p> <note> <p>The region for the S3 bucket containing the S3 object must match the region used for WorkMail operations. Also, for WorkMail to process an S3 object, it must have permission to access that object. For more information, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/update-with-lambda.html"> Updating message content with AWS Lambda</a>.</p> </note></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3Reference {
    /// <p>The S3 bucket name.</p>
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// <p>The S3 key object name.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>If you enable versioning for the bucket, you can specify the object version.</p>
    #[serde(rename = "objectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version: Option<String>,
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
/// Errors returned by PutRawMessageContent
#[derive(Debug, PartialEq)]
pub enum PutRawMessageContentError {
    /// <p><p>WorkMail could not access the updated email content. Possible reasons:</p> <ul> <li> <p>You made the request in a region other than your S3 bucket region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-owner-condition.html">S3 bucket owner</a> is not the same as the calling AWS account.</p> </li> <li> <p>You have an incomplete or missing S3 bucket policy. For more information about policies, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/update-with-lambda.html"> Updating message content with AWS Lambda </a> in the <i>WorkMail Administrator Guide</i>.</p> </li> </ul></p>
    InvalidContentLocation(String),
    /// <p>The requested email is not eligible for update. This is usually the case for a redirected email.</p>
    MessageFrozen(String),
    /// <p>The requested email could not be updated due to an error in the MIME content. Check the error message for more information about what caused the error.</p>
    MessageRejected(String),
    /// <p>The requested email message is not found.</p>
    ResourceNotFound(String),
}

impl PutRawMessageContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRawMessageContentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidContentLocation" => {
                    return RusotoError::Service(PutRawMessageContentError::InvalidContentLocation(
                        err.msg,
                    ))
                }
                "MessageFrozen" => {
                    return RusotoError::Service(PutRawMessageContentError::MessageFrozen(err.msg))
                }
                "MessageRejected" => {
                    return RusotoError::Service(PutRawMessageContentError::MessageRejected(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRawMessageContentError::ResourceNotFound(
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
impl fmt::Display for PutRawMessageContentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRawMessageContentError::InvalidContentLocation(ref cause) => write!(f, "{}", cause),
            PutRawMessageContentError::MessageFrozen(ref cause) => write!(f, "{}", cause),
            PutRawMessageContentError::MessageRejected(ref cause) => write!(f, "{}", cause),
            PutRawMessageContentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRawMessageContentError {}
/// Trait representing the capabilities of the Amazon WorkMail Message Flow API. Amazon WorkMail Message Flow clients implement this trait.
#[async_trait]
pub trait WorkmailMessageFlow {
    /// <p>Retrieves the raw content of an in-transit email message, in MIME format.</p>
    async fn get_raw_message_content(
        &self,
        input: GetRawMessageContentRequest,
    ) -> Result<GetRawMessageContentResponse, RusotoError<GetRawMessageContentError>>;

    /// <p><p>Updates the raw content of an in-transit email message, in MIME format.</p> <p>This example describes how to update in-transit email message. For more information and examples for using this API, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/update-with-lambda.html"> Updating message content with AWS Lambda</a>.</p> <note> <p>Updates to an in-transit message only appear when you call <code>PutRawMessageContent</code> from an AWS Lambda function configured with a synchronous <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/lambda.html#synchronous-rules"> Run Lambda</a> rule. If you call <code>PutRawMessageContent</code> on a delivered or sent message, the message remains unchanged, even though <a href="https://docs.aws.amazon.com/workmail/latest/APIReference/API_messageflow_GetRawMessageContent.html">GetRawMessageContent</a> returns an updated message. </p> </note></p>
    async fn put_raw_message_content(
        &self,
        input: PutRawMessageContentRequest,
    ) -> Result<PutRawMessageContentResponse, RusotoError<PutRawMessageContentError>>;
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
    /// <p>Retrieves the raw content of an in-transit email message, in MIME format.</p>
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

    /// <p><p>Updates the raw content of an in-transit email message, in MIME format.</p> <p>This example describes how to update in-transit email message. For more information and examples for using this API, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/update-with-lambda.html"> Updating message content with AWS Lambda</a>.</p> <note> <p>Updates to an in-transit message only appear when you call <code>PutRawMessageContent</code> from an AWS Lambda function configured with a synchronous <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/lambda.html#synchronous-rules"> Run Lambda</a> rule. If you call <code>PutRawMessageContent</code> on a delivered or sent message, the message remains unchanged, even though <a href="https://docs.aws.amazon.com/workmail/latest/APIReference/API_messageflow_GetRawMessageContent.html">GetRawMessageContent</a> returns an updated message. </p> </note></p>
    #[allow(unused_mut)]
    async fn put_raw_message_content(
        &self,
        input: PutRawMessageContentRequest,
    ) -> Result<PutRawMessageContentResponse, RusotoError<PutRawMessageContentError>> {
        let request_uri = format!("/messages/{message_id}", message_id = input.message_id);

        let mut request =
            SignedRequest::new("POST", "workmailmessageflow", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutRawMessageContentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutRawMessageContentError::from_response(response))
        }
    }
}
