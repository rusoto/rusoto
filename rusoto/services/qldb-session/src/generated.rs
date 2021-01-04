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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl QldbSessionClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "qldb", &self.region, request_uri);
        request.set_endpoint_prefix("session.qldb".to_string());

        request.set_content_type("application/x-amz-json-1.0".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

/// <p>Contains the details of the transaction to abort.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AbortTransactionRequest {}

/// <p>Contains the details of the aborted transaction.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AbortTransactionResult {
    /// <p>Contains server-side performance information for the command.</p>
    #[serde(rename = "TimingInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_information: Option<TimingInformation>,
}

/// <p>Contains the details of the transaction to commit.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CommitTransactionRequest {
    /// <p>Specifies the commit digest for the transaction to commit. For every active transaction, the commit digest must be passed. QLDB validates <code>CommitDigest</code> and rejects the commit with an error if the digest computed on the client does not match the digest computed by QLDB.</p> <p>The purpose of the <code>CommitDigest</code> parameter is to ensure that QLDB commits a transaction if and only if the server has processed the exact set of statements sent by the client, in the same order that client sent them, and with no duplicates.</p>
    #[serde(rename = "CommitDigest")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub commit_digest: bytes::Bytes,
    /// <p>Specifies the transaction ID of the transaction to commit.</p>
    #[serde(rename = "TransactionId")]
    pub transaction_id: String,
}

/// <p>Contains the details of the committed transaction.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>Contains metrics about the number of I/O requests that were consumed.</p>
    #[serde(rename = "ConsumedIOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_i_os: Option<IOUsage>,
    /// <p>Contains server-side performance information for the command.</p>
    #[serde(rename = "TimingInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_information: Option<TimingInformation>,
    /// <p>The transaction ID of the committed transaction.</p>
    #[serde(rename = "TransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>Specifies a request to end the session.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndSessionRequest {}

/// <p>Contains the details of the ended session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndSessionResult {
    /// <p>Contains server-side performance information for the command.</p>
    #[serde(rename = "TimingInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_information: Option<TimingInformation>,
}

/// <p>Specifies a request to execute a statement.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExecuteStatementRequest {
    /// <p>Specifies the parameters for the parameterized statement in the request.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ValueHolder>>,
    /// <p>Specifies the statement of the request.</p>
    #[serde(rename = "Statement")]
    pub statement: String,
    /// <p>Specifies the transaction ID of the request.</p>
    #[serde(rename = "TransactionId")]
    pub transaction_id: String,
}

/// <p>Contains the details of the executed statement.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExecuteStatementResult {
    /// <p>Contains metrics about the number of I/O requests that were consumed.</p>
    #[serde(rename = "ConsumedIOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_i_os: Option<IOUsage>,
    /// <p>Contains the details of the first fetched page.</p>
    #[serde(rename = "FirstPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page: Option<Page>,
    /// <p>Contains server-side performance information for the command.</p>
    #[serde(rename = "TimingInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_information: Option<TimingInformation>,
}

/// <p>Specifies the details of the page to be fetched.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FetchPageRequest {
    /// <p>Specifies the next page token of the page to be fetched.</p>
    #[serde(rename = "NextPageToken")]
    pub next_page_token: String,
    /// <p>Specifies the transaction ID of the page to be fetched.</p>
    #[serde(rename = "TransactionId")]
    pub transaction_id: String,
}

/// <p>Contains the page that was fetched.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FetchPageResult {
    /// <p>Contains metrics about the number of I/O requests that were consumed.</p>
    #[serde(rename = "ConsumedIOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_i_os: Option<IOUsage>,
    /// <p>Contains details of the fetched page.</p>
    #[serde(rename = "Page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Page>,
    /// <p>Contains server-side performance information for the command.</p>
    #[serde(rename = "TimingInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_information: Option<TimingInformation>,
}

/// <p>Contains I/O usage metrics for a command that was invoked.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IOUsage {
    /// <p>The number of read I/O requests that the command performed.</p>
    #[serde(rename = "ReadIOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_i_os: Option<i64>,
    /// <p>The number of write I/O requests that the command performed.</p>
    #[serde(rename = "WriteIOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_i_os: Option<i64>,
}

/// <p>Contains details of the fetched page.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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

/// <p>Specifies a request to start a new session.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSessionRequest {
    /// <p>The name of the ledger to start a new session against.</p>
    #[serde(rename = "LedgerName")]
    pub ledger_name: String,
}

/// <p>Contains the details of the started session.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSessionResult {
    /// <p>Session token of the started session. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    #[serde(rename = "SessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
    /// <p>Contains server-side performance information for the command.</p>
    #[serde(rename = "TimingInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_information: Option<TimingInformation>,
}

/// <p>Specifies a request to start a transaction.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartTransactionRequest {}

/// <p>Contains the details of the started transaction.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTransactionResult {
    /// <p>Contains server-side performance information for the command.</p>
    #[serde(rename = "TimingInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_information: Option<TimingInformation>,
    /// <p>The transaction ID of the started transaction.</p>
    #[serde(rename = "TransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>Contains server-side performance information for a command. Amazon QLDB captures timing information between the times when it receives the request and when it sends the corresponding response.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TimingInformation {
    /// <p>The amount of time that was taken for the command to finish processing, measured in milliseconds.</p>
    #[serde(rename = "ProcessingTimeMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_time_milliseconds: Option<i64>,
}

/// <p>A structure that can contain a value in multiple encoding formats.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ValueHolder {
    /// <p>An Amazon Ion binary value contained in a <code>ValueHolder</code> structure.</p>
    #[serde(rename = "IonBinary")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ion_binary: Option<bytes::Bytes>,
    /// <p>An Amazon Ion plaintext value contained in a <code>ValueHolder</code> structure.</p>
    #[serde(rename = "IonText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ion_text: Option<String>,
}

/// Errors returned by SendCommand
#[derive(Debug, PartialEq)]
pub enum SendCommandError {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequest(String),
    /// <p>Returned if the session doesn't exist anymore because it timed out or expired.</p>
    InvalidSession(String),
    /// <p>Returned if a resource limit such as number of active sessions is exceeded.</p>
    LimitExceeded(String),
    /// <p>Returned when a transaction cannot be written to the journal due to a failure in the verification phase of <i>optimistic concurrency control</i> (OCC).</p>
    OccConflict(String),
    /// <p>Returned when the rate of requests exceeds the allowed throughput.</p>
    RateExceeded(String),
}

impl SendCommandError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendCommandError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            #[allow(clippy::single_match)]
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
    /// <p><p>Sends a command to an Amazon QLDB ledger.</p> <note> <p>Instead of interacting directly with this API, we recommend using the QLDB driver or the QLDB shell to execute data transactions on a ledger.</p> <ul> <li> <p>If you are working with an AWS SDK, use the QLDB driver. The driver provides a high-level abstraction layer above this <i>QLDB Session</i> data plane and manages <code>SendCommand</code> API calls for you. For information and a list of supported programming languages, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/getting-started-driver.html">Getting started with the driver</a> in the <i>Amazon QLDB Developer Guide</i>.</p> </li> <li> <p>If you are working with the AWS Command Line Interface (AWS CLI), use the QLDB shell. The shell is a command line interface that uses the QLDB driver to interact with a ledger. For information, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/data-shell.html">Accessing Amazon QLDB using the QLDB shell</a>.</p> </li> </ul> </note></p>
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
    /// <p><p>Sends a command to an Amazon QLDB ledger.</p> <note> <p>Instead of interacting directly with this API, we recommend using the QLDB driver or the QLDB shell to execute data transactions on a ledger.</p> <ul> <li> <p>If you are working with an AWS SDK, use the QLDB driver. The driver provides a high-level abstraction layer above this <i>QLDB Session</i> data plane and manages <code>SendCommand</code> API calls for you. For information and a list of supported programming languages, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/getting-started-driver.html">Getting started with the driver</a> in the <i>Amazon QLDB Developer Guide</i>.</p> </li> <li> <p>If you are working with the AWS Command Line Interface (AWS CLI), use the QLDB shell. The shell is a command line interface that uses the QLDB driver to interact with a ledger. For information, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/data-shell.html">Accessing Amazon QLDB using the QLDB shell</a>.</p> </li> </ul> </note></p>
    async fn send_command(
        &self,
        input: SendCommandRequest,
    ) -> Result<SendCommandResult, RusotoError<SendCommandError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "QLDBSession.SendCommand");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SendCommandError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SendCommandResult, _>()
    }
}
