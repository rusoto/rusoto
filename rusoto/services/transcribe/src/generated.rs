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
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateVocabularyRequest {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>An array of strings that contains the vocabulary entries. </p>
    #[serde(rename = "Phrases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
    /// <p>The S3 location of the text file that contains the definition of the custom vocabulary. The URI must be in the same region as the API endpoint that you are calling. The general form is </p> <p> <code> https://s3-&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>For example:</p> <p> <code>https://s3-us-east-1.amazonaws.com/examplebucket/vocab.txt</code> </p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>For more information about custom vocabularies, see <a href="http://docs.aws.amazon.com/transcribe/latest/dg/how-it-works.html#how-vocabulary">Custom Vocabularies</a>.</p>
    #[serde(rename = "VocabularyFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    /// <p>The name of the vocabulary. The name must be unique within an AWS account. The name is case-sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
pub struct DeleteTranscriptionJobRequest {
    /// <p>The name of the transcription job to be deleted.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>When specified, returns only transcription jobs with the specified status. Jobs are ordered by creation date, with the newest jobs returned first. If you don’t specify a status, Amazon Transcribe returns all transcription jobs ordered by creation date. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The S3 location of the input media file. The URI must be in the same region as the API endpoint that you are calling. The general form is:</p> <p> <code> https://s3-&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>For example:</p> <p> <code>https://s3-us-east-1.amazonaws.com/examplebucket/example.mp4</code> </p> <p> <code>https://s3-us-east-1.amazonaws.com/examplebucket/mediadocs/example.mp4</code> </p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p>
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
    /// <p>The location where the transcription is stored.</p> <p>If you set the <code>OutputBucketName</code>, Amazon Transcribe puts the transcription in the specified S3 bucket. When you call the <a>GetTranscriptionJob</a> operation, the operation returns this location in the <code>TranscriptFileUri</code> field. The S3 bucket must have permissions that allow Amazon Transcribe to put files in the bucket. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/access-control-managing-permissions.html#auth-role-iam-user">Permissions Required for IAM User Roles</a>.</p> <p>Amazon Transcribe uses the default Amazon S3 key for server-side encryption of transcripts that are placed in your S3 bucket. You can't specify your own encryption key.</p> <p>If you don't set the <code>OutputBucketName</code>, Amazon Transcribe generates a pre-signed URL, a shareable URL that provides secure access to your transcription, and returns it in the <code>TranscriptFileUri</code> field. Use this URL to download the transcription.</p>
    #[serde(rename = "OutputBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bucket_name: Option<String>,
    /// <p>A <code>Settings</code> object that provides optional settings for a transcription job.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    /// <p>The name of the job. Note that you can't use the strings "." or ".." by themselves as the job name. The name must also be unique within an AWS account.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartTranscriptionJobResponse {
    /// <p>An object containing details of the asynchronous transcription job.</p>
    #[serde(rename = "TranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

/// <p>Identifies the location of a transcription.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Transcript {
    /// <p>The location where the transcription is stored.</p> <p>Use this URI to access the transcription. If you specified an S3 bucket in the <code>OutputBucketName</code> field when you created the job, this is the URI of that bucket. If you chose to store the transcription in Amazon Transcribe, this is a shareable URL that provides secure access to that location.</p>
    #[serde(rename = "TranscriptFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_file_uri: Option<String>,
}

/// <p>Describes an asynchronous transcription job that was created with the <code>StartTranscriptionJob</code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TranscriptionJob {
    /// <p>A timestamp that shows when the job was completed.</p>
    #[serde(rename = "CompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// <p>A timestamp that shows when the job was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p><p>If the <code>TranscriptionJobStatus</code> field is <code>FAILED</code>, this field contains information about why the job failed.</p> <p>The <code>FailureReason</code> field can contain one of the following values:</p> <ul> <li> <p> <code>Unsupported media format</code> - The media format specified in the <code>MediaFormat</code> field of the request isn&#39;t valid. See the description of the <code>MediaFormat</code> field for a list of valid values.</p> </li> <li> <p> <code>The media format provided does not match the detected media format</code> - The media format of the audio file doesn&#39;t match the format specified in the <code>MediaFormat</code> field in the request. Check the media format of your media file and make sure that the two values match.</p> </li> <li> <p> <code>Invalid sample rate for audio file</code> - The sample rate specified in the <code>MediaSampleRateHertz</code> of the request isn&#39;t valid. The sample rate must be between 8000 and 48000 Hertz.</p> </li> <li> <p> <code>The sample rate provided does not match the detected sample rate</code> - The sample rate in the audio file doesn&#39;t match the sample rate specified in the <code>MediaSampleRateHertz</code> field in the request. Check the sample rate of your media file and make sure that the two values match.</p> </li> <li> <p> <code>Invalid file size: file size too large</code> - The size of your audio file is larger than Amazon Transcribe can process. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/limits-guidelines.html#limits">Limits</a> in the <i>Amazon Transcribe Developer Guide</i>.</p> </li> <li> <p> <code>Invalid number of channels: number of channels too large</code> - Your audio contains more channels than Amazon Transcribe is configured to process. To request additional channels, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits-amazon-transcribe">Amazon Transcribe Limits</a> in the <i>Amazon Web Services General Reference</i>.</p> </li> </ul></p>
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

/// <p>Provides a summary of information about a transcription job. .</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
    /// <p>The S3 location of the text file that contains the definition of the custom vocabulary. The URI must be in the same region as the API endpoint that you are calling. The general form is </p> <p> <code> https://s3-&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>For example:</p> <p> <code>https://s3-us-east-1.amazonaws.com/examplebucket/vocab.txt</code> </p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>For more information about custom vocabularies, see <a href="http://docs.aws.amazon.com/transcribe/latest/dg/how-it-works.html#how-vocabulary">Custom Vocabularies</a>.</p>
    #[serde(rename = "VocabularyFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    /// <p>The name of the vocabulary to update. The name is case-sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Provides information about a custom vocabulary. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>StartTranscriptionJob</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl CreateVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVocabularyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateVocabularyError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateVocabularyError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVocabularyError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by DeleteTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum DeleteTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl DeleteTranscriptionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteTranscriptionJobError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteTranscriptionJobError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteTranscriptionJobError::LimitExceeded(
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
impl fmt::Display for DeleteTranscriptionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTranscriptionJobError {
    fn description(&self) -> &str {
        match *self {
            DeleteTranscriptionJobError::BadRequest(ref cause) => cause,
            DeleteTranscriptionJobError::InternalFailure(ref cause) => cause,
            DeleteTranscriptionJobError::LimitExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVocabulary
#[derive(Debug, PartialEq)]
pub enum DeleteVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl DeleteVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVocabularyError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteVocabularyError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteVocabularyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum GetTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl GetTranscriptionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetTranscriptionJobError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetTranscriptionJobError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetTranscriptionJobError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTranscriptionJobError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by GetVocabulary
#[derive(Debug, PartialEq)]
pub enum GetVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl GetVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVocabularyError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetVocabularyError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetVocabularyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListTranscriptionJobs
#[derive(Debug, PartialEq)]
pub enum ListTranscriptionJobsError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListTranscriptionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTranscriptionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTranscriptionJobsError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListTranscriptionJobsError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTranscriptionJobsError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListVocabularies
#[derive(Debug, PartialEq)]
pub enum ListVocabulariesError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListVocabulariesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVocabulariesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVocabulariesError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListVocabulariesError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListVocabulariesError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by StartTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum StartTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>StartTranscriptionJob</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl StartTranscriptionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTranscriptionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(StartTranscriptionJobError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartTranscriptionJobError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(StartTranscriptionJobError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartTranscriptionJobError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by UpdateVocabulary
#[derive(Debug, PartialEq)]
pub enum UpdateVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>StartTranscriptionJob</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl UpdateVocabularyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVocabularyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVocabularyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateVocabularyError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateVocabularyError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateVocabularyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVocabularyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Trait representing the capabilities of the Amazon Transcribe Service API. Amazon Transcribe Service clients implement this trait.
#[async_trait]
pub trait Transcribe {
    /// <p>Creates a new custom vocabulary that you can use to change the way Amazon Transcribe handles transcription of an audio file. </p>
    async fn create_vocabulary(
        &self,
        input: CreateVocabularyRequest,
    ) -> Result<CreateVocabularyResponse, RusotoError<CreateVocabularyError>>;

    /// <p>Deletes a previously submitted transcription job along with any other generated results such as the transcription, models, and so on.</p>
    async fn delete_transcription_job(
        &self,
        input: DeleteTranscriptionJobRequest,
    ) -> Result<(), RusotoError<DeleteTranscriptionJobError>>;

    /// <p>Deletes a vocabulary from Amazon Transcribe. </p>
    async fn delete_vocabulary(
        &self,
        input: DeleteVocabularyRequest,
    ) -> Result<(), RusotoError<DeleteVocabularyError>>;

    /// <p>Returns information about a transcription job. To see the status of the job, check the <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>, the job is finished and you can find the results at the location specified in the <code>TranscriptionFileUri</code> field.</p>
    async fn get_transcription_job(
        &self,
        input: GetTranscriptionJobRequest,
    ) -> Result<GetTranscriptionJobResponse, RusotoError<GetTranscriptionJobError>>;

    /// <p>Gets information about a vocabulary. </p>
    async fn get_vocabulary(
        &self,
        input: GetVocabularyRequest,
    ) -> Result<GetVocabularyResponse, RusotoError<GetVocabularyError>>;

    /// <p>Lists transcription jobs with the specified status.</p>
    async fn list_transcription_jobs(
        &self,
        input: ListTranscriptionJobsRequest,
    ) -> Result<ListTranscriptionJobsResponse, RusotoError<ListTranscriptionJobsError>>;

    /// <p>Returns a list of vocabularies that match the specified criteria. If no criteria are specified, returns the entire list of vocabularies.</p>
    async fn list_vocabularies(
        &self,
        input: ListVocabulariesRequest,
    ) -> Result<ListVocabulariesResponse, RusotoError<ListVocabulariesError>>;

    /// <p>Starts an asynchronous job to transcribe speech to text. </p>
    async fn start_transcription_job(
        &self,
        input: StartTranscriptionJobRequest,
    ) -> Result<StartTranscriptionJobResponse, RusotoError<StartTranscriptionJobError>>;

    /// <p>Updates an existing vocabulary with new values. The <code>UpdateVocabulary</code> operation overwrites all of the existing information with the values that you provide in the request. </p>
    async fn update_vocabulary(
        &self,
        input: UpdateVocabularyRequest,
    ) -> Result<UpdateVocabularyResponse, RusotoError<UpdateVocabularyError>>;
}
/// A client for the Amazon Transcribe Service API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TranscribeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        TranscribeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

#[async_trait]
impl Transcribe for TranscribeClient {
    /// <p>Creates a new custom vocabulary that you can use to change the way Amazon Transcribe handles transcription of an audio file. </p>
    async fn create_vocabulary(
        &self,
        input: CreateVocabularyRequest,
    ) -> Result<CreateVocabularyResponse, RusotoError<CreateVocabularyError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.CreateVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVocabularyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVocabularyError::from_response(response))
        }
    }

    /// <p>Deletes a previously submitted transcription job along with any other generated results such as the transcription, models, and so on.</p>
    async fn delete_transcription_job(
        &self,
        input: DeleteTranscriptionJobRequest,
    ) -> Result<(), RusotoError<DeleteTranscriptionJobError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.DeleteTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTranscriptionJobError::from_response(response))
        }
    }

    /// <p>Deletes a vocabulary from Amazon Transcribe. </p>
    async fn delete_vocabulary(
        &self,
        input: DeleteVocabularyRequest,
    ) -> Result<(), RusotoError<DeleteVocabularyError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.DeleteVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            Ok(std::mem::drop(response))
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVocabularyError::from_response(response))
        }
    }

    /// <p>Returns information about a transcription job. To see the status of the job, check the <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>, the job is finished and you can find the results at the location specified in the <code>TranscriptionFileUri</code> field.</p>
    async fn get_transcription_job(
        &self,
        input: GetTranscriptionJobRequest,
    ) -> Result<GetTranscriptionJobResponse, RusotoError<GetTranscriptionJobError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.GetTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetTranscriptionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetTranscriptionJobError::from_response(response))
        }
    }

    /// <p>Gets information about a vocabulary. </p>
    async fn get_vocabulary(
        &self,
        input: GetVocabularyRequest,
    ) -> Result<GetVocabularyResponse, RusotoError<GetVocabularyError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.GetVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetVocabularyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetVocabularyError::from_response(response))
        }
    }

    /// <p>Lists transcription jobs with the specified status.</p>
    async fn list_transcription_jobs(
        &self,
        input: ListTranscriptionJobsRequest,
    ) -> Result<ListTranscriptionJobsResponse, RusotoError<ListTranscriptionJobsError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.ListTranscriptionJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTranscriptionJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTranscriptionJobsError::from_response(response))
        }
    }

    /// <p>Returns a list of vocabularies that match the specified criteria. If no criteria are specified, returns the entire list of vocabularies.</p>
    async fn list_vocabularies(
        &self,
        input: ListVocabulariesRequest,
    ) -> Result<ListVocabulariesResponse, RusotoError<ListVocabulariesError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.ListVocabularies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVocabulariesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListVocabulariesError::from_response(response))
        }
    }

    /// <p>Starts an asynchronous job to transcribe speech to text. </p>
    async fn start_transcription_job(
        &self,
        input: StartTranscriptionJobRequest,
    ) -> Result<StartTranscriptionJobResponse, RusotoError<StartTranscriptionJobError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.StartTranscriptionJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StartTranscriptionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartTranscriptionJobError::from_response(response))
        }
    }

    /// <p>Updates an existing vocabulary with new values. The <code>UpdateVocabulary</code> operation overwrites all of the existing information with the values that you provide in the request. </p>
    async fn update_vocabulary(
        &self,
        input: UpdateVocabularyRequest,
    ) -> Result<UpdateVocabularyResponse, RusotoError<UpdateVocabularyError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.UpdateVocabulary");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateVocabularyResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVocabularyError::from_response(response))
        }
    }
}
