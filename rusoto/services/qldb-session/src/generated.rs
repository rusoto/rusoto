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
/// <p>Contains the details of the transaction to abort.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AbortTransactionRequest {}

/// <p>Contains the details of the aborted transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AbortTransactionResult {}

/// <p>Contains the details of the transaction to commit.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CommitTransactionRequest {
    /// <p>Specifies the commit digest for the transaction to commit. For every active transaction, the commit digest must be passed. QLDB validates <code>CommitDigest</code> and rejects the commit with an error if the digest computed on the client does not match the digest computed by QLDB.</p>
    #[serde(rename = "CommitDigest")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub commit_digest: bytes::Bytes,
    /// <p>Specifies the transaction id of the transaction to commit.</p>
    #[serde(rename = "TransactionId")]
    pub transaction_id: String,
}

/// <p>Contains the details of the committed transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CommitTransactionResult {
    /// <p>The commit digest of the committed transaction.</p>
    #[serde(rename = "CommitDigest")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_digest: Option<bytes::Bytes>,
    /// <p>The transaction id of the committed transaction.</p>
    #[serde(rename = "TransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>Specifies a request to end the session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndSessionRequest {}

/// <p>Contains the details of the ended session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndSessionResult {}

/// <p>Specifies a request to execute a statement.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecuteStatementRequest {
    /// <p>Specifies the parameters for the parameterized statement in the request.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ValueHolder>>,
    /// <p>Specifies the statement of the request.</p>
    #[serde(rename = "Statement")]
    pub statement: String,
    /// <p>Specifies the transaction id of the request.</p>
    #[serde(rename = "TransactionId")]
    pub transaction_id: String,
}

/// <p>Contains the details of the executed statement.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecuteStatementResult {
    /// <p>Contains the details of the first fetched page.</p>
    #[serde(rename = "FirstPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page: Option<Page>,
}

/// <p>Specifies the details of the page to be fetched.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FetchPageRequest {
    /// <p>Specifies the next page token of the page to be fetched.</p>
    #[serde(rename = "NextPageToken")]
    pub next_page_token: String,
    /// <p>Specifies the transaction id of the page to be fetched.</p>
    #[serde(rename = "TransactionId")]
    pub transaction_id: String,
}

/// <p>Contains the page that was fetched.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FetchPageResult {
    /// <p>Contains details of the fetched page.</p>
    #[serde(rename = "Page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Page>,
}

/// <p>Contains details of the fetched page.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Page {
    /// <p>The token of the next page.</p>
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>A structure that contains values in multiple encoding formats.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ValueHolder>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendCommandRequest {
    /// <p>Command to abort the current transaction.</p>
    #[serde(rename = "AbortTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_transaction: Option<AbortTransactionRequest>,
    /// <p>Command to commit the specified transaction.</p>
    #[serde(rename = "CommitTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_transaction: Option<CommitTransactionRequest>,
    /// <p>Command to end the current session.</p>
    #[serde(rename = "EndSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_session: Option<EndSessionRequest>,
    /// <p>Command to execute a statement in the specified transaction.</p>
    #[serde(rename = "ExecuteStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_statement: Option<ExecuteStatementRequest>,
    /// <p>Command to fetch a page.</p>
    #[serde(rename = "FetchPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_page: Option<FetchPageRequest>,
    /// <p>Specifies the session token for the current command. A session token is constant throughout the life of the session.</p> <p>To obtain a session token, run the <code>StartSession</code> command. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    #[serde(rename = "SessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
    /// <p>Command to start a new session. A session token is obtained as part of the response.</p>
    #[serde(rename = "StartSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_session: Option<StartSessionRequest>,
    /// <p>Command to start a new transaction.</p>
    #[serde(rename = "StartTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_transaction: Option<StartTransactionRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendCommandResult {
    /// <p>Contains the details of the aborted transaction.</p>
    #[serde(rename = "AbortTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_transaction: Option<AbortTransactionResult>,
    /// <p>Contains the details of the committed transaction.</p>
    #[serde(rename = "CommitTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_transaction: Option<CommitTransactionResult>,
    /// <p>Contains the details of the ended session.</p>
    #[serde(rename = "EndSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_session: Option<EndSessionResult>,
    /// <p>Contains the details of the executed statement.</p>
    #[serde(rename = "ExecuteStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_statement: Option<ExecuteStatementResult>,
    /// <p>Contains the details of the fetched page.</p>
    #[serde(rename = "FetchPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_page: Option<FetchPageResult>,
    /// <p>Contains the details of the started session that includes a session token. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    #[serde(rename = "StartSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_session: Option<StartSessionResult>,
    /// <p>Contains the details of the started transaction.</p>
    #[serde(rename = "StartTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_transaction: Option<StartTransactionResult>,
}

/// <p>Specifies a request to start a a new session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSessionRequest {
    /// <p>The name of the ledger to start a new session against.</p>
    #[serde(rename = "LedgerName")]
    pub ledger_name: String,
}

/// <p>Contains the details of the started session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSessionResult {
    /// <p>Session token of the started session. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    #[serde(rename = "SessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

/// <p>Specifies a request to start a transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartTransactionRequest {}

/// <p>Contains the details of the started transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTransactionResult {
    /// <p>The transaction id of the started transaction.</p>
    #[serde(rename = "TransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>A structure that can contains values in multiple encoding formats.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueHolder {
    /// <p>An Amazon Ion binary value contained in a <code>ValueHolder</code> structure. </p>
    #[serde(rename = "IonBinary")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ion_binary: Option<bytes::Bytes>,
    /// <p>An Amazon Ion plaintext value contained in a <code>ValueHolder</code> structure. </p>
    #[serde(rename = "IonText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ion_text: Option<String>,
}

/// Errors returned by SendCommand
#[derive(Debug, PartialEq)]
pub enum SendCommandError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the session doesn't exist anymore because it timed-out or expired.</p>
    InvalidSession(String),
    /// <p>Returned if a resource limit such as number of active sessions is exceeded.</p>
    LimitExceeded(String),
    /// <p>Returned when a transaction cannot be written to the journal due to a failure in the verification phase of Optimistic Concurrency Control.</p>
    OccConflict(String),
    /// <p>Returned when the rate of requests exceeds the allowed throughput.</p>
    RateExceeded(String),
}

impl SendCommandError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendCommandError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(SendCommandError::BadRequest(err.msg))
                }
                "InvalidSessionException" => {
                    return RusotoError::Service(SendCommandError::InvalidSession(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SendCommandError::LimitExceeded(err.msg))
                }
                "OccConflictException" => {
                    return RusotoError::Service(SendCommandError::OccConflict(err.msg))
                }
                "RateExceededException" => {
                    return RusotoError::Service(SendCommandError::RateExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendCommandError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendCommandError::BadRequest(ref cause) => write!(f, "{}", cause),
            SendCommandError::InvalidSession(ref cause) => write!(f, "{}", cause),
            SendCommandError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            SendCommandError::OccConflict(ref cause) => write!(f, "{}", cause),
            SendCommandError::RateExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendCommandError {}
/// Trait representing the capabilities of the QLDB Session API. QLDB Session clients implement this trait.
#[async_trait]
pub trait QldbSession {
    /// <p>Sends a command to an Amazon QLDB ledger.</p>
    async fn send_command(
        &self,
        input: SendCommandRequest,
    ) -> Result<SendCommandResult, RusotoError<SendCommandError>>;
}
/// A client for the QLDB Session API.
#[derive(Clone)]
pub struct QldbSessionClient {
    client: Client,
    region: region::Region,
}

impl QldbSessionClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> QldbSessionClient {
        QldbSessionClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> QldbSessionClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        QldbSessionClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> QldbSessionClient {
        QldbSessionClient { client, region }
    }
}

#[async_trait]
impl QldbSession for QldbSessionClient {
    /// <p>Sends a command to an Amazon QLDB ledger.</p>
    async fn send_command(
        &self,
        input: SendCommandRequest,
    ) -> Result<SendCommandResult, RusotoError<SendCommandError>> {
        let mut request = SignedRequest::new("POST", "qldb", &self.region, "/");
        request.set_endpoint_prefix("session.qldb".to_string());
        request.set_content_type("application/x-amz-json-1.0".to_owned());
        request.add_header("x-amz-target", "QLDBSession.SendCommand");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SendCommandResult, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SendCommandError::from_response(response))
        }
    }
}
