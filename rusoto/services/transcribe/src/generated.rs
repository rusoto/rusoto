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
pub struct CreateVocabularyRequest {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>An array of strings that contains the vocabulary entries. </p>
    #[serde(rename = "Phrases")]
    pub phrases: Vec<String>,
    /// <p>The name of the vocabulary. The name must be unique within an AWS account. The name is case-sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateVocabularyResponse {
    /// <p>If the <code>VocabularyState</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date and time that the vocabulary was created.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary. When the <code>VocabularyState</code> field contains <code>READY</code> the vocabulary is ready to be used in a <code>StartTranscriptionJob</code> request.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVocabularyRequest {
    /// <p>The name of the vocabulary to delete. </p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTranscriptionJobRequest {
    /// <p>The name of the job.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetTranscriptionJobResponse {
    /// <p>An object that contains the results of the transcription job.</p>
    #[serde(rename = "TranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVocabularyRequest {
    /// <p>The name of the vocabulary to return information about. The name is case-sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetVocabularyResponse {
    /// <p>The S3 location where the vocabulary is stored. Use this URI to get the contents of the vocabulary. The URI is available for a limited time.</p>
    #[serde(rename = "DownloadUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    /// <p>If the <code>VocabularyState</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date and time that the vocabulary was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary to return.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTranscriptionJobsRequest {
    /// <p>When specified, the jobs returned in the list are limited to jobs whose name contains the specified string.</p>
    #[serde(rename = "JobNameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name_contains: Option<String>,
    /// <p>The maximum number of jobs to return in the response. If there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the previous request to <code>ListTranscriptionJobs</code> was truncated, include the <code>NextToken</code> to fetch the next set of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When specified, returns only transcription jobs with the specified status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTranscriptionJobsResponse {
    /// <p>The <code>ListTranscriptionJobs</code> operation returns a page of jobs at a time. The maximum size of the page is set by the <code>MaxResults</code> parameter. If there are more jobs in the list than the page size, Amazon Transcribe returns the <code>NextPage</code> token. Include the token in the next request to the <code>ListTranscriptionJobs</code> operation to return in the next page of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The requested status of the jobs returned.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A list of objects containing summary information for a transcription job.</p>
    #[serde(rename = "TranscriptionJobSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_summaries: Option<Vec<TranscriptionJobSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVocabulariesRequest {
    /// <p>The maximum number of vocabularies to return in the response. If there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>When specified, the vocabularies returned in the list are limited to vocabularies whose name contains the specified string. The search is case-insensitive, <code>ListVocabularies</code> will return both "vocabularyname" and "VocabularyName" in the response list.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous request to <code>ListVocabularies</code> was truncated, include the <code>NextToken</code> to fetch the next set of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When specified, only returns vocabularies with the <code>VocabularyState</code> field equal to the specified state.</p>
    #[serde(rename = "StateEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_equals: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListVocabulariesResponse {
    /// <p>The <code>ListVocabularies</code> operation returns a page of vocabularies at a time. The maximum size of the page is set by the <code>MaxResults</code> parameter. If there are more jobs in the list than the page size, Amazon Transcribe returns the <code>NextPage</code> token. Include the token in the next request to the <code>ListVocabularies</code> operation to return in the next page of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The requested vocabulary state.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A list of objects that describe the vocabularies that match the search criteria in the request.</p>
    #[serde(rename = "Vocabularies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabularies: Option<Vec<VocabularyInfo>>,
}

/// <p>Describes the input media file in a transcription request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Media {
    /// <p>The S3 location of the input media file. The URI must be in the same region as the API endpoint that you are calling. The general form is:</p> <p> <code> https://&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>For example:</p> <p> <code>https://s3-us-east-1.amazonaws.com/examplebucket/example.mp4</code> </p> <p> <code>https://s3-us-east-1.amazonaws.com/examplebucket/mediadocs/example.mp4</code> </p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    #[serde(rename = "MediaFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_file_uri: Option<String>,
}

/// <p>Provides optional settings for the <code>StartTranscriptionJob</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    /// <p>Instructs Amazon Transcribe to process each audio channel separately and then merge the transcription output of each channel into a single transcription. </p> <p>Amazon Transcribe also produces a transcription of each item detected on an audio channel, including the start time and end time of the item and alternative transcriptions of the item including the confidence that Amazon Transcribe has in the transcription.</p> <p>You can't set both <code>ShowSpeakerLabels</code> and <code>ChannelIdentification</code> in the same request. If you set both, your request returns a <code>BadRequestException</code>.</p>
    #[serde(rename = "ChannelIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_identification: Option<bool>,
    /// <p>The maximum number of speakers to identify in the input audio. If there are more speakers in the audio than this number, multiple speakers will be identified as a single speaker. If you specify the <code>MaxSpeakerLabels</code> field, you must set the <code>ShowSpeakerLabels</code> field to true.</p>
    #[serde(rename = "MaxSpeakerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speaker_labels: Option<i64>,
    /// <p>Determines whether the transcription job uses speaker recognition to identify different speakers in the input audio. Speaker recognition labels individual speakers in the audio file. If you set the <code>ShowSpeakerLabels</code> field to true, you must also set the maximum number of speaker labels <code>MaxSpeakerLabels</code> field.</p> <p>You can't set both <code>ShowSpeakerLabels</code> and <code>ChannelIdentification</code> in the same request. If you set both, your request returns a <code>BadRequestException</code>.</p>
    #[serde(rename = "ShowSpeakerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_speaker_labels: Option<bool>,
    /// <p>The name of a vocabulary to use when processing the transcription job.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartTranscriptionJobRequest {
    /// <p>The language code for the language used in the input media file.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>An object that describes the input media for a transcription job.</p>
    #[serde(rename = "Media")]
    pub media: Media,
    /// <p>The format of the input media file.</p>
    #[serde(rename = "MediaFormat")]
    pub media_format: String,
    /// <p>The sample rate, in Hertz, of the audio track in the input media file. </p>
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i64>,
    /// <p>The location where the transcription is stored.</p> <p>If you set the <code>OutputBucketName</code>, Amazon Transcribe puts the transcription in the specified S3 bucket. When you call the <a>GetTranscriptionJob</a> operation, the operation returns this location in the <code>TranscriptFileUri</code> field. The S3 bucket must have permissions that allow Amazon Transcribe to put files in the bucket. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/access-control-managing-permissions.html#auth-role-iam-user">Permissions Required for IAM User Roles</a>.</p> <p>If you don't set the <code>OutputBucketName</code>, Amazon Transcribe generates a pre-signed URL, a shareable URL that provides secure access to your transcription, and returns it in the <code>TranscriptFileUri</code> field. Use this URL to download the transcription.</p>
    #[serde(rename = "OutputBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bucket_name: Option<String>,
    /// <p>A <code>Settings</code> object that provides optional settings for a transcription job.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    /// <p>The name of the job. You can't use the strings "." or ".." in the job name. The name must be unique within an AWS account.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartTranscriptionJobResponse {
    /// <p>An object containing details of the asynchronous transcription job.</p>
    #[serde(rename = "TranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

/// <p>Identifies the location of a transcription.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Transcript {
    /// <p>The location where the transcription is stored.</p> <p>Use this URI to access the transcription. If you specified an S3 bucket in the <code>OutputBucketName</code> field when you created the job, this is the URI of that bucket. If you chose to store the transcription in Amazon Transcribe, this is a shareable URL that provides secure access to that location.</p>
    #[serde(rename = "TranscriptFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_file_uri: Option<String>,
}

/// <p>Describes an asynchronous transcription job that was created with the <code>StartTranscriptionJob</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TranscriptionJob {
    /// <p>A timestamp that shows when the job was completed.</p>
    #[serde(rename = "CompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// <p>A timestamp that shows when the job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>If the <code>TranscriptionJobStatus</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language code for the input speech.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>An object that describes the input media for the transcription job.</p>
    #[serde(rename = "Media")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    /// <p>The format of the input media file.</p>
    #[serde(rename = "MediaFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<String>,
    /// <p>The sample rate, in Hertz, of the audio track in the input media file. </p>
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i64>,
    /// <p>Optional settings for the transcription job. Use these settings to turn on speaker recognition, to set the maximum number of speakers that should be identified and to specify a custom vocabulary to use when processing the transcription job.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    /// <p>An object that describes the output of the transcription job.</p>
    #[serde(rename = "Transcript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Transcript>,
    /// <p>The name of the transcription job.</p>
    #[serde(rename = "TranscriptionJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_name: Option<String>,
    /// <p>The status of the transcription job.</p>
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<String>,
}

/// <p>Provides a summary of information about a transcription job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TranscriptionJobSummary {
    /// <p>A timestamp that shows when the job was completed.</p>
    #[serde(rename = "CompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// <p>A timestamp that shows when the job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>If the <code>TranscriptionJobStatus</code> field is <code>FAILED</code>, a description of the error.</p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The language code for the input speech.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Indicates the location of the output of the transcription job.</p> <p>If the value is <code>CUSTOMER_BUCKET</code> then the location is the S3 bucket specified in the <code>outputBucketName</code> field when the transcription job was started with the <code>StartTranscriptionJob</code> operation.</p> <p>If the value is <code>SERVICE_BUCKET</code> then the output is stored by Amazon Transcribe and can be retrieved using the URI in the <code>GetTranscriptionJob</code> response's <code>TranscriptFileUri</code> field.</p>
    #[serde(rename = "OutputLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location_type: Option<String>,
    /// <p>The name of the transcription job.</p>
    #[serde(rename = "TranscriptionJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_name: Option<String>,
    /// <p>The status of the transcription job. When the status is <code>COMPLETED</code>, use the <code>GetTranscriptionJob</code> operation to get the results of the transcription.</p>
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateVocabularyRequest {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>An array of strings containing the vocabulary entries.</p>
    #[serde(rename = "Phrases")]
    pub phrases: Vec<String>,
    /// <p>The name of the vocabulary to update. The name is case-sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateVocabularyResponse {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date and time that the vocabulary was updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary that was updated.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary. When the <code>VocabularyState</code> field contains <code>READY</code> the vocabulary is ready to be used in a <code>StartTranscriptionJob</code> request.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

/// <p>Provides information about a custom vocabulary.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VocabularyInfo {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date and time that the vocabulary was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    /// <p>The processing state of the vocabulary. If the state is <code>READY</code> you can use the vocabulary in a <code>StartTranscriptionJob</code> request.</p>
    #[serde(rename = "VocabularyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

/// Errors returned by CreateVocabulary
#[derive(Debug, PartialEq)]
pub enum CreateVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, a name already exists when creating a resource or a name may not exist when getting a transcription job or custom vocabulary. See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>StartTranscriptionJob</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
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

impl CreateVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateVocabularyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return CreateVocabularyError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return CreateVocabularyError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return CreateVocabularyError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return CreateVocabularyError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateVocabularyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateVocabularyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateVocabularyError {
    fn from(err: serde_json::error::Error) -> CreateVocabularyError {
        CreateVocabularyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateVocabularyError {
    fn from(err: CredentialsError) -> CreateVocabularyError {
        CreateVocabularyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateVocabularyError {
    fn from(err: HttpDispatchError) -> CreateVocabularyError {
        CreateVocabularyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateVocabularyError {
    fn from(err: io::Error) -> CreateVocabularyError {
        CreateVocabularyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateVocabularyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateVocabularyError {
    fn description(&self) -> &str {
        match *self {
            CreateVocabularyError::BadRequest(ref cause) => cause,
            CreateVocabularyError::Conflict(ref cause) => cause,
            CreateVocabularyError::InternalFailure(ref cause) => cause,
            CreateVocabularyError::LimitExceeded(ref cause) => cause,
            CreateVocabularyError::Validation(ref cause) => cause,
            CreateVocabularyError::Credentials(ref err) => err.description(),
            CreateVocabularyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateVocabularyError::ParseError(ref cause) => cause,
            CreateVocabularyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteVocabulary
#[derive(Debug, PartialEq)]
pub enum DeleteVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, a name already exists when creating a resource or a name may not exist when getting a transcription job or custom vocabulary. See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
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

impl DeleteVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteVocabularyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return DeleteVocabularyError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return DeleteVocabularyError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return DeleteVocabularyError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return DeleteVocabularyError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteVocabularyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteVocabularyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteVocabularyError {
    fn from(err: serde_json::error::Error) -> DeleteVocabularyError {
        DeleteVocabularyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVocabularyError {
    fn from(err: CredentialsError) -> DeleteVocabularyError {
        DeleteVocabularyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVocabularyError {
    fn from(err: HttpDispatchError) -> DeleteVocabularyError {
        DeleteVocabularyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVocabularyError {
    fn from(err: io::Error) -> DeleteVocabularyError {
        DeleteVocabularyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVocabularyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVocabularyError {
    fn description(&self) -> &str {
        match *self {
            DeleteVocabularyError::BadRequest(ref cause) => cause,
            DeleteVocabularyError::InternalFailure(ref cause) => cause,
            DeleteVocabularyError::LimitExceeded(ref cause) => cause,
            DeleteVocabularyError::NotFound(ref cause) => cause,
            DeleteVocabularyError::Validation(ref cause) => cause,
            DeleteVocabularyError::Credentials(ref err) => err.description(),
            DeleteVocabularyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteVocabularyError::ParseError(ref cause) => cause,
            DeleteVocabularyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum GetTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, a name already exists when creating a resource or a name may not exist when getting a transcription job or custom vocabulary. See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
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

impl GetTranscriptionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> GetTranscriptionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetTranscriptionJobError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetTranscriptionJobError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetTranscriptionJobError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetTranscriptionJobError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetTranscriptionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetTranscriptionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetTranscriptionJobError {
    fn from(err: serde_json::error::Error) -> GetTranscriptionJobError {
        GetTranscriptionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTranscriptionJobError {
    fn from(err: CredentialsError) -> GetTranscriptionJobError {
        GetTranscriptionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTranscriptionJobError {
    fn from(err: HttpDispatchError) -> GetTranscriptionJobError {
        GetTranscriptionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTranscriptionJobError {
    fn from(err: io::Error) -> GetTranscriptionJobError {
        GetTranscriptionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTranscriptionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTranscriptionJobError {
    fn description(&self) -> &str {
        match *self {
            GetTranscriptionJobError::BadRequest(ref cause) => cause,
            GetTranscriptionJobError::InternalFailure(ref cause) => cause,
            GetTranscriptionJobError::LimitExceeded(ref cause) => cause,
            GetTranscriptionJobError::NotFound(ref cause) => cause,
            GetTranscriptionJobError::Validation(ref cause) => cause,
            GetTranscriptionJobError::Credentials(ref err) => err.description(),
            GetTranscriptionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetTranscriptionJobError::ParseError(ref cause) => cause,
            GetTranscriptionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetVocabulary
#[derive(Debug, PartialEq)]
pub enum GetVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, a name already exists when creating a resource or a name may not exist when getting a transcription job or custom vocabulary. See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
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

impl GetVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> GetVocabularyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return GetVocabularyError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return GetVocabularyError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return GetVocabularyError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return GetVocabularyError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetVocabularyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetVocabularyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetVocabularyError {
    fn from(err: serde_json::error::Error) -> GetVocabularyError {
        GetVocabularyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetVocabularyError {
    fn from(err: CredentialsError) -> GetVocabularyError {
        GetVocabularyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetVocabularyError {
    fn from(err: HttpDispatchError) -> GetVocabularyError {
        GetVocabularyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetVocabularyError {
    fn from(err: io::Error) -> GetVocabularyError {
        GetVocabularyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetVocabularyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVocabularyError {
    fn description(&self) -> &str {
        match *self {
            GetVocabularyError::BadRequest(ref cause) => cause,
            GetVocabularyError::InternalFailure(ref cause) => cause,
            GetVocabularyError::LimitExceeded(ref cause) => cause,
            GetVocabularyError::NotFound(ref cause) => cause,
            GetVocabularyError::Validation(ref cause) => cause,
            GetVocabularyError::Credentials(ref err) => err.description(),
            GetVocabularyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetVocabularyError::ParseError(ref cause) => cause,
            GetVocabularyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTranscriptionJobs
#[derive(Debug, PartialEq)]
pub enum ListTranscriptionJobsError {
    /// <p>Your request didn't pass one or more validation tests. For example, a name already exists when creating a resource or a name may not exist when getting a transcription job or custom vocabulary. See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
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

impl ListTranscriptionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTranscriptionJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return ListTranscriptionJobsError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return ListTranscriptionJobsError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListTranscriptionJobsError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return ListTranscriptionJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTranscriptionJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTranscriptionJobsError {
    fn from(err: serde_json::error::Error) -> ListTranscriptionJobsError {
        ListTranscriptionJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTranscriptionJobsError {
    fn from(err: CredentialsError) -> ListTranscriptionJobsError {
        ListTranscriptionJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTranscriptionJobsError {
    fn from(err: HttpDispatchError) -> ListTranscriptionJobsError {
        ListTranscriptionJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTranscriptionJobsError {
    fn from(err: io::Error) -> ListTranscriptionJobsError {
        ListTranscriptionJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTranscriptionJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTranscriptionJobsError {
    fn description(&self) -> &str {
        match *self {
            ListTranscriptionJobsError::BadRequest(ref cause) => cause,
            ListTranscriptionJobsError::InternalFailure(ref cause) => cause,
            ListTranscriptionJobsError::LimitExceeded(ref cause) => cause,
            ListTranscriptionJobsError::Validation(ref cause) => cause,
            ListTranscriptionJobsError::Credentials(ref err) => err.description(),
            ListTranscriptionJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTranscriptionJobsError::ParseError(ref cause) => cause,
            ListTranscriptionJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListVocabularies
#[derive(Debug, PartialEq)]
pub enum ListVocabulariesError {
    /// <p>Your request didn't pass one or more validation tests. For example, a name already exists when creating a resource or a name may not exist when getting a transcription job or custom vocabulary. See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
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

impl ListVocabulariesError {
    pub fn from_response(res: BufferedHttpResponse) -> ListVocabulariesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return ListVocabulariesError::BadRequest(String::from(error_message))
                }
                "InternalFailureException" => {
                    return ListVocabulariesError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return ListVocabulariesError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return ListVocabulariesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListVocabulariesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListVocabulariesError {
    fn from(err: serde_json::error::Error) -> ListVocabulariesError {
        ListVocabulariesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListVocabulariesError {
    fn from(err: CredentialsError) -> ListVocabulariesError {
        ListVocabulariesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVocabulariesError {
    fn from(err: HttpDispatchError) -> ListVocabulariesError {
        ListVocabulariesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVocabulariesError {
    fn from(err: io::Error) -> ListVocabulariesError {
        ListVocabulariesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVocabulariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVocabulariesError {
    fn description(&self) -> &str {
        match *self {
            ListVocabulariesError::BadRequest(ref cause) => cause,
            ListVocabulariesError::InternalFailure(ref cause) => cause,
            ListVocabulariesError::LimitExceeded(ref cause) => cause,
            ListVocabulariesError::Validation(ref cause) => cause,
            ListVocabulariesError::Credentials(ref err) => err.description(),
            ListVocabulariesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListVocabulariesError::ParseError(ref cause) => cause,
            ListVocabulariesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum StartTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, a name already exists when creating a resource or a name may not exist when getting a transcription job or custom vocabulary. See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>StartTranscriptionJob</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
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

impl StartTranscriptionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StartTranscriptionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return StartTranscriptionJobError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return StartTranscriptionJobError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return StartTranscriptionJobError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return StartTranscriptionJobError::LimitExceeded(String::from(error_message))
                }
                "ValidationException" => {
                    return StartTranscriptionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartTranscriptionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartTranscriptionJobError {
    fn from(err: serde_json::error::Error) -> StartTranscriptionJobError {
        StartTranscriptionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartTranscriptionJobError {
    fn from(err: CredentialsError) -> StartTranscriptionJobError {
        StartTranscriptionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartTranscriptionJobError {
    fn from(err: HttpDispatchError) -> StartTranscriptionJobError {
        StartTranscriptionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartTranscriptionJobError {
    fn from(err: io::Error) -> StartTranscriptionJobError {
        StartTranscriptionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartTranscriptionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartTranscriptionJobError {
    fn description(&self) -> &str {
        match *self {
            StartTranscriptionJobError::BadRequest(ref cause) => cause,
            StartTranscriptionJobError::Conflict(ref cause) => cause,
            StartTranscriptionJobError::InternalFailure(ref cause) => cause,
            StartTranscriptionJobError::LimitExceeded(ref cause) => cause,
            StartTranscriptionJobError::Validation(ref cause) => cause,
            StartTranscriptionJobError::Credentials(ref err) => err.description(),
            StartTranscriptionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartTranscriptionJobError::ParseError(ref cause) => cause,
            StartTranscriptionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateVocabulary
#[derive(Debug, PartialEq)]
pub enum UpdateVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, a name already exists when creating a resource or a name may not exist when getting a transcription job or custom vocabulary. See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>StartTranscriptionJob</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
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

impl UpdateVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateVocabularyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BadRequestException" => {
                    return UpdateVocabularyError::BadRequest(String::from(error_message))
                }
                "ConflictException" => {
                    return UpdateVocabularyError::Conflict(String::from(error_message))
                }
                "InternalFailureException" => {
                    return UpdateVocabularyError::InternalFailure(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateVocabularyError::LimitExceeded(String::from(error_message))
                }
                "NotFoundException" => {
                    return UpdateVocabularyError::NotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateVocabularyError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateVocabularyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateVocabularyError {
    fn from(err: serde_json::error::Error) -> UpdateVocabularyError {
        UpdateVocabularyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateVocabularyError {
    fn from(err: CredentialsError) -> UpdateVocabularyError {
        UpdateVocabularyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateVocabularyError {
    fn from(err: HttpDispatchError) -> UpdateVocabularyError {
        UpdateVocabularyError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateVocabularyError {
    fn from(err: io::Error) -> UpdateVocabularyError {
        UpdateVocabularyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateVocabularyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVocabularyError {
    fn description(&self) -> &str {
        match *self {
            UpdateVocabularyError::BadRequest(ref cause) => cause,
            UpdateVocabularyError::Conflict(ref cause) => cause,
            UpdateVocabularyError::InternalFailure(ref cause) => cause,
            UpdateVocabularyError::LimitExceeded(ref cause) => cause,
            UpdateVocabularyError::NotFound(ref cause) => cause,
            UpdateVocabularyError::Validation(ref cause) => cause,
            UpdateVocabularyError::Credentials(ref err) => err.description(),
            UpdateVocabularyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateVocabularyError::ParseError(ref cause) => cause,
            UpdateVocabularyError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Transcribe Service API. Amazon Transcribe Service clients implement this trait.
pub trait Transcribe {
    /// <p>Creates a new custom vocabulary that you can use to change the way Amazon Transcribe handles transcription of an audio file.</p>
    fn create_vocabulary(
        &self,
        input: CreateVocabularyRequest,
    ) -> RusotoFuture<CreateVocabularyResponse, CreateVocabularyError>;

    /// <p>Deletes a vocabulary from Amazon Transcribe. </p>
    fn delete_vocabulary(
        &self,
        input: DeleteVocabularyRequest,
    ) -> RusotoFuture<(), DeleteVocabularyError>;

    /// <p>Returns information about a transcription job. To see the status of the job, check the <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>, the job is finished and you can find the results at the location specified in the <code>TranscriptionFileUri</code> field.</p>
    fn get_transcription_job(
        &self,
        input: GetTranscriptionJobRequest,
    ) -> RusotoFuture<GetTranscriptionJobResponse, GetTranscriptionJobError>;

    /// <p>Gets information about a vocabulary.</p>
    fn get_vocabulary(
        &self,
        input: GetVocabularyRequest,
    ) -> RusotoFuture<GetVocabularyResponse, GetVocabularyError>;

    /// <p>Lists transcription jobs with the specified status.</p>
    fn list_transcription_jobs(
        &self,
        input: ListTranscriptionJobsRequest,
    ) -> RusotoFuture<ListTranscriptionJobsResponse, ListTranscriptionJobsError>;

    /// <p>Returns a list of vocabularies that match the specified criteria. If no criteria are specified, returns the entire list of vocabularies.</p>
    fn list_vocabularies(
        &self,
        input: ListVocabulariesRequest,
    ) -> RusotoFuture<ListVocabulariesResponse, ListVocabulariesError>;

    /// <p>Starts an asynchronous job to transcribe speech to text.</p>
    fn start_transcription_job(
        &self,
        input: StartTranscriptionJobRequest,
    ) -> RusotoFuture<StartTranscriptionJobResponse, StartTranscriptionJobError>;

    /// <p>Updates an existing vocabulary with new values. The <code>UpdateVocabulary</code> operation overwrites all of the existing information with the values that you provide in the request.</p>
    fn update_vocabulary(
        &self,
        input: UpdateVocabularyRequest,
    ) -> RusotoFuture<UpdateVocabularyResponse, UpdateVocabularyError>;
}
/// A client for the Amazon Transcribe Service API.
pub struct TranscribeClient {
    client: Client,
    region: region::Region,
}

impl TranscribeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> TranscribeClient {
        TranscribeClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TranscribeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        TranscribeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Transcribe for TranscribeClient {
    /// <p>Creates a new custom vocabulary that you can use to change the way Amazon Transcribe handles transcription of an audio file.</p>
    fn create_vocabulary(
        &self,
        input: CreateVocabularyRequest,
    ) -> RusotoFuture<CreateVocabularyResponse, CreateVocabularyError> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.CreateVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateVocabularyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateVocabularyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a vocabulary from Amazon Transcribe. </p>
    fn delete_vocabulary(
        &self,
        input: DeleteVocabularyRequest,
    ) -> RusotoFuture<(), DeleteVocabularyError> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.DeleteVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteVocabularyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a transcription job. To see the status of the job, check the <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>, the job is finished and you can find the results at the location specified in the <code>TranscriptionFileUri</code> field.</p>
    fn get_transcription_job(
        &self,
        input: GetTranscriptionJobRequest,
    ) -> RusotoFuture<GetTranscriptionJobResponse, GetTranscriptionJobError> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.GetTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetTranscriptionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetTranscriptionJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about a vocabulary.</p>
    fn get_vocabulary(
        &self,
        input: GetVocabularyRequest,
    ) -> RusotoFuture<GetVocabularyResponse, GetVocabularyError> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.GetVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetVocabularyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetVocabularyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists transcription jobs with the specified status.</p>
    fn list_transcription_jobs(
        &self,
        input: ListTranscriptionJobsRequest,
    ) -> RusotoFuture<ListTranscriptionJobsResponse, ListTranscriptionJobsError> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.ListTranscriptionJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTranscriptionJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTranscriptionJobsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of vocabularies that match the specified criteria. If no criteria are specified, returns the entire list of vocabularies.</p>
    fn list_vocabularies(
        &self,
        input: ListVocabulariesRequest,
    ) -> RusotoFuture<ListVocabulariesResponse, ListVocabulariesError> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.ListVocabularies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListVocabulariesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListVocabulariesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts an asynchronous job to transcribe speech to text.</p>
    fn start_transcription_job(
        &self,
        input: StartTranscriptionJobRequest,
    ) -> RusotoFuture<StartTranscriptionJobResponse, StartTranscriptionJobError> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.StartTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartTranscriptionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartTranscriptionJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates an existing vocabulary with new values. The <code>UpdateVocabulary</code> operation overwrites all of the existing information with the values that you provide in the request.</p>
    fn update_vocabulary(
        &self,
        input: UpdateVocabularyRequest,
    ) -> RusotoFuture<UpdateVocabularyResponse, UpdateVocabularyError> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.UpdateVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateVocabularyResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateVocabularyError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
