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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeJobExecutionRequest {
    /// <p>Optional. A number that identifies a particular job execution on a particular device. If not specified, the latest job execution is returned.</p>
    #[serde(rename = "executionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    /// <p>Optional. When set to true, the response contains the job document. The default is false.</p>
    #[serde(rename = "includeJobDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_job_document: Option<bool>,
    /// <p>The unique identifier assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The thing name associated with the device the job execution is running on.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeJobExecutionResponse {
    /// <p>Contains data about a job execution.</p>
    #[serde(rename = "execution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<JobExecution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPendingJobExecutionsRequest {
    /// <p>The name of the thing that is executing the job.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPendingJobExecutionsResponse {
    /// <p>A list of JobExecutionSummary objects with status IN_PROGRESS.</p>
    #[serde(rename = "inProgressJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_jobs: Option<Vec<JobExecutionSummary>>,
    /// <p>A list of JobExecutionSummary objects with status QUEUED.</p>
    #[serde(rename = "queuedJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_jobs: Option<Vec<JobExecutionSummary>>,
}

/// <p>Contains data about a job execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobExecution {
    /// <p>The estimated number of seconds that remain before the job execution status will be changed to <code>TIMED_OUT</code>.</p>
    #[serde(rename = "approximateSecondsBeforeTimedOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_seconds_before_timed_out: Option<i64>,
    /// <p>A number that identifies a particular job execution on a particular device. It can be used later in commands that return or update job execution information.</p>
    #[serde(rename = "executionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    /// <p>The content of the job document.</p>
    #[serde(rename = "jobDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_document: Option<String>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was last updated. </p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<i64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
    #[serde(rename = "queuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<i64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was started.</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
    #[serde(rename = "statusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the thing that is executing the job.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
    #[serde(rename = "versionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>Contains data about the state of a job execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobExecutionState {
    /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
    #[serde(rename = "statusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
    #[serde(rename = "versionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

/// <p>Contains a subset of information about a job execution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct JobExecutionSummary {
    /// <p>A number that identifies a particular job execution on a particular device.</p>
    #[serde(rename = "executionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<i64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
    #[serde(rename = "queuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<i64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution started.</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// <p>The version of the job execution. Job execution versions are incremented each time AWS IoT Jobs receives an update from a device.</p>
    #[serde(rename = "versionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartNextPendingJobExecutionRequest {
    /// <p>A collection of name/value pairs that describe the status of the job execution. If not specified, the statusDetails are unchanged.</p>
    #[serde(rename = "statusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the amount of time this device has to finish execution of this job. If the job execution status is not set to a terminal state before this timer expires, or before the timer is reset (by calling <code>UpdateJobExecution</code>, setting the status to <code>IN_PROGRESS</code> and specifying a new timeout value in field <code>stepTimeoutInMinutes</code>) the job execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting this timeout has no effect on that job execution timeout which may have been specified when the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
    #[serde(rename = "stepTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_timeout_in_minutes: Option<i64>,
    /// <p>The name of the thing associated with the device.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartNextPendingJobExecutionResponse {
    /// <p>A JobExecution object.</p>
    #[serde(rename = "execution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<JobExecution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJobExecutionRequest {
    /// <p>Optional. A number that identifies a particular job execution on a particular device.</p>
    #[serde(rename = "executionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    /// <p>Optional. The expected current version of the job execution. Each time you update the job execution, its version is incremented. If the version of the job execution stored in Jobs does not match, the update is rejected with a VersionMismatch error, and an ErrorResponse that contains the current job execution status data is returned. (This makes it unnecessary to perform a separate DescribeJobExecution request in order to obtain the job execution status data.)</p>
    #[serde(rename = "expectedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    /// <p>Optional. When set to true, the response contains the job document. The default is false.</p>
    #[serde(rename = "includeJobDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_job_document: Option<bool>,
    /// <p>Optional. When included and set to true, the response contains the JobExecutionState data. The default is false.</p>
    #[serde(rename = "includeJobExecutionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_job_execution_state: Option<bool>,
    /// <p>The unique identifier assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The new status for the job execution (IN_PROGRESS, FAILED, SUCCESS, or REJECTED). This must be specified on every update.</p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p> Optional. A collection of name/value pairs that describe the status of the job execution. If not specified, the statusDetails are unchanged.</p>
    #[serde(rename = "statusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the amount of time this device has to finish execution of this job. If the job execution status is not set to a terminal state before this timer expires, or before the timer is reset (by again calling <code>UpdateJobExecution</code>, setting the status to <code>IN_PROGRESS</code> and specifying a new timeout value in this field) the job execution status will be automatically set to <code>TIMED_OUT</code>. Note that setting or resetting this timeout has no effect on that job execution timeout which may have been specified when the job was created (<code>CreateJob</code> using field <code>timeoutConfig</code>).</p>
    #[serde(rename = "stepTimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_timeout_in_minutes: Option<i64>,
    /// <p>The name of the thing associated with the device.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateJobExecutionResponse {
    /// <p>A JobExecutionState object.</p>
    #[serde(rename = "executionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_state: Option<JobExecutionState>,
    /// <p>The contents of the Job Documents.</p>
    #[serde(rename = "jobDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_document: Option<String>,
}

/// Errors returned by DescribeJobExecution
#[derive(Debug, PartialEq)]
pub enum DescribeJobExecutionError {
    /// <p>The certificate is invalid.</p>
    CertificateValidation(String),
    /// <p>The contents of the request were invalid. For example, this code is returned when an UpdateJobExecution request contains invalid status details. The message contains details about the error.</p>
    InvalidRequest(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The job is in a terminal state.</p>
    TerminalState(String),
    /// <p>The rate exceeds the limit.</p>
    Throttling(String),
}

impl DescribeJobExecutionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobExecutionError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "CertificateValidationException" => {
                    return RusotoError::Service(DescribeJobExecutionError::CertificateValidation(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeJobExecutionError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeJobExecutionError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeJobExecutionError::ServiceUnavailable(
                        String::from(error_message),
                    ))
                }
                "TerminalStateException" => {
                    return RusotoError::Service(DescribeJobExecutionError::TerminalState(
                        String::from(error_message),
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeJobExecutionError::Throttling(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeJobExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobExecutionError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobExecutionError::CertificateValidation(ref cause) => cause,
            DescribeJobExecutionError::InvalidRequest(ref cause) => cause,
            DescribeJobExecutionError::ResourceNotFound(ref cause) => cause,
            DescribeJobExecutionError::ServiceUnavailable(ref cause) => cause,
            DescribeJobExecutionError::TerminalState(ref cause) => cause,
            DescribeJobExecutionError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPendingJobExecutions
#[derive(Debug, PartialEq)]
pub enum GetPendingJobExecutionsError {
    /// <p>The certificate is invalid.</p>
    CertificateValidation(String),
    /// <p>The contents of the request were invalid. For example, this code is returned when an UpdateJobExecution request contains invalid status details. The message contains details about the error.</p>
    InvalidRequest(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The rate exceeds the limit.</p>
    Throttling(String),
}

impl GetPendingJobExecutionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPendingJobExecutionsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "CertificateValidationException" => {
                    return RusotoError::Service(
                        GetPendingJobExecutionsError::CertificateValidation(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetPendingJobExecutionsError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPendingJobExecutionsError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetPendingJobExecutionsError::ServiceUnavailable(
                        String::from(error_message),
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetPendingJobExecutionsError::Throttling(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPendingJobExecutionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPendingJobExecutionsError {
    fn description(&self) -> &str {
        match *self {
            GetPendingJobExecutionsError::CertificateValidation(ref cause) => cause,
            GetPendingJobExecutionsError::InvalidRequest(ref cause) => cause,
            GetPendingJobExecutionsError::ResourceNotFound(ref cause) => cause,
            GetPendingJobExecutionsError::ServiceUnavailable(ref cause) => cause,
            GetPendingJobExecutionsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by StartNextPendingJobExecution
#[derive(Debug, PartialEq)]
pub enum StartNextPendingJobExecutionError {
    /// <p>The certificate is invalid.</p>
    CertificateValidation(String),
    /// <p>The contents of the request were invalid. For example, this code is returned when an UpdateJobExecution request contains invalid status details. The message contains details about the error.</p>
    InvalidRequest(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The rate exceeds the limit.</p>
    Throttling(String),
}

impl StartNextPendingJobExecutionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartNextPendingJobExecutionError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "CertificateValidationException" => {
                    return RusotoError::Service(
                        StartNextPendingJobExecutionError::CertificateValidation(String::from(
                            error_message,
                        )),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartNextPendingJobExecutionError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StartNextPendingJobExecutionError::ResourceNotFound(String::from(
                            error_message,
                        )),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        StartNextPendingJobExecutionError::ServiceUnavailable(String::from(
                            error_message,
                        )),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartNextPendingJobExecutionError::Throttling(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartNextPendingJobExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartNextPendingJobExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartNextPendingJobExecutionError::CertificateValidation(ref cause) => cause,
            StartNextPendingJobExecutionError::InvalidRequest(ref cause) => cause,
            StartNextPendingJobExecutionError::ResourceNotFound(ref cause) => cause,
            StartNextPendingJobExecutionError::ServiceUnavailable(ref cause) => cause,
            StartNextPendingJobExecutionError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateJobExecution
#[derive(Debug, PartialEq)]
pub enum UpdateJobExecutionError {
    /// <p>The certificate is invalid.</p>
    CertificateValidation(String),
    /// <p>The contents of the request were invalid. For example, this code is returned when an UpdateJobExecution request contains invalid status details. The message contains details about the error.</p>
    InvalidRequest(String),
    /// <p>An update attempted to change the job execution to a state that is invalid because of the job execution's current state (for example, an attempt to change a request in state SUCCESS to state IN_PROGRESS). In this case, the body of the error message also contains the executionState field.</p>
    InvalidStateTransition(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The rate exceeds the limit.</p>
    Throttling(String),
}

impl UpdateJobExecutionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobExecutionError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "CertificateValidationException" => {
                    return RusotoError::Service(UpdateJobExecutionError::CertificateValidation(
                        String::from(error_message),
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateJobExecutionError::InvalidRequest(
                        String::from(error_message),
                    ))
                }
                "InvalidStateTransitionException" => {
                    return RusotoError::Service(UpdateJobExecutionError::InvalidStateTransition(
                        String::from(error_message),
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateJobExecutionError::ResourceNotFound(
                        String::from(error_message),
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateJobExecutionError::ServiceUnavailable(
                        String::from(error_message),
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateJobExecutionError::Throttling(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateJobExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateJobExecutionError {
    fn description(&self) -> &str {
        match *self {
            UpdateJobExecutionError::CertificateValidation(ref cause) => cause,
            UpdateJobExecutionError::InvalidRequest(ref cause) => cause,
            UpdateJobExecutionError::InvalidStateTransition(ref cause) => cause,
            UpdateJobExecutionError::ResourceNotFound(ref cause) => cause,
            UpdateJobExecutionError::ServiceUnavailable(ref cause) => cause,
            UpdateJobExecutionError::Throttling(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS IoT Jobs Data Plane API. AWS IoT Jobs Data Plane clients implement this trait.
pub trait IotJobsData {
    /// <p>Gets details of a job execution.</p>
    fn describe_job_execution(
        &self,
        input: DescribeJobExecutionRequest,
    ) -> RusotoFuture<DescribeJobExecutionResponse, DescribeJobExecutionError>;

    /// <p>Gets the list of all jobs for a thing that are not in a terminal status.</p>
    fn get_pending_job_executions(
        &self,
        input: GetPendingJobExecutionsRequest,
    ) -> RusotoFuture<GetPendingJobExecutionsResponse, GetPendingJobExecutionsError>;

    /// <p>Gets and starts the next pending (status IN_PROGRESS or QUEUED) job execution for a thing.</p>
    fn start_next_pending_job_execution(
        &self,
        input: StartNextPendingJobExecutionRequest,
    ) -> RusotoFuture<StartNextPendingJobExecutionResponse, StartNextPendingJobExecutionError>;

    /// <p>Updates the status of a job execution.</p>
    fn update_job_execution(
        &self,
        input: UpdateJobExecutionRequest,
    ) -> RusotoFuture<UpdateJobExecutionResponse, UpdateJobExecutionError>;
}
/// A client for the AWS IoT Jobs Data Plane API.
#[derive(Clone)]
pub struct IotJobsDataClient {
    client: Client,
    region: region::Region,
}

impl IotJobsDataClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IotJobsDataClient {
        IotJobsDataClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IotJobsDataClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        IotJobsDataClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl IotJobsData for IotJobsDataClient {
    /// <p>Gets details of a job execution.</p>
    fn describe_job_execution(
        &self,
        input: DescribeJobExecutionRequest,
    ) -> RusotoFuture<DescribeJobExecutionResponse, DescribeJobExecutionError> {
        let request_uri = format!(
            "/things/{thing_name}/jobs/{job_id}",
            job_id = input.job_id,
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("GET", "iot-jobs-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.jobs.iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.execution_number {
            params.put("executionNumber", x);
        }
        if let Some(ref x) = input.include_job_document {
            params.put("includeJobDocument", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeJobExecutionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeJobExecutionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets the list of all jobs for a thing that are not in a terminal status.</p>
    fn get_pending_job_executions(
        &self,
        input: GetPendingJobExecutionsRequest,
    ) -> RusotoFuture<GetPendingJobExecutionsResponse, GetPendingJobExecutionsError> {
        let request_uri = format!("/things/{thing_name}/jobs", thing_name = input.thing_name);

        let mut request = SignedRequest::new("GET", "iot-jobs-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.jobs.iot".to_string());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetPendingJobExecutionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetPendingJobExecutionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets and starts the next pending (status IN_PROGRESS or QUEUED) job execution for a thing.</p>
    fn start_next_pending_job_execution(
        &self,
        input: StartNextPendingJobExecutionRequest,
    ) -> RusotoFuture<StartNextPendingJobExecutionResponse, StartNextPendingJobExecutionError> {
        let request_uri = format!(
            "/things/{thing_name}/jobs/$next",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("PUT", "iot-jobs-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.jobs.iot".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<StartNextPendingJobExecutionResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartNextPendingJobExecutionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates the status of a job execution.</p>
    fn update_job_execution(
        &self,
        input: UpdateJobExecutionRequest,
    ) -> RusotoFuture<UpdateJobExecutionResponse, UpdateJobExecutionError> {
        let request_uri = format!(
            "/things/{thing_name}/jobs/{job_id}",
            job_id = input.job_id,
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("POST", "iot-jobs-data", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("data.jobs.iot".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateJobExecutionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateJobExecutionError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
