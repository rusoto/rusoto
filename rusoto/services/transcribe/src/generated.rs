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
pub struct CreateVocabularyFilterRequest {
    /// <p>The language code of the words in the vocabulary filter. All words in the filter must be in the same language. The vocabulary filter can only be used with transcription jobs in the specified language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>The Amazon S3 location of a text file used as input to create the vocabulary filter. Only use characters from the character set defined for custom vocabularies. For a list of character sets, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-vocabulary.html#charsets">Character Sets for Custom Vocabularies</a>.</p> <p>The specified file must be less than 50 KB of UTF-8 characters.</p> <p>If you provide the location of a list of words in the <code>VocabularyFilterFileUri</code> parameter, you can't use the <code>Words</code> parameter.</p>
    #[serde(rename = "VocabularyFilterFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_file_uri: Option<String>,
    /// <p>The vocabulary filter name. The name must be unique within the account that contains it.</p>
    #[serde(rename = "VocabularyFilterName")]
    pub vocabulary_filter_name: String,
    /// <p>The words to use in the vocabulary filter. Only use characters from the character set defined for custom vocabularies. For a list of character sets, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-vocabulary.html#charsets">Character Sets for Custom Vocabularies</a>.</p> <p>If you provide a list of words in the <code>Words</code> parameter, you can't use the <code>VocabularyFilterFileUri</code> parameter.</p>
    #[serde(rename = "Words")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVocabularyFilterResponse {
    /// <p>The language code of the words in the collection.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date and time that the vocabulary filter was modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary filter.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVocabularyRequest {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>An array of strings that contains the vocabulary entries. </p>
    #[serde(rename = "Phrases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
    /// <p>The S3 location of the text file that contains the definition of the custom vocabulary. The URI must be in the same region as the API endpoint that you are calling. The general form is </p> <p> <code> https://s3.&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>For example:</p> <p> <code>https://s3.us-east-1.amazonaws.com/examplebucket/vocab.txt</code> </p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>For more information about custom vocabularies, see <a href="http://docs.aws.amazon.com/transcribe/latest/dg/how-it-works.html#how-vocabulary">Custom Vocabularies</a>.</p>
    #[serde(rename = "VocabularyFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    /// <p>The name of the vocabulary. The name must be unique within an AWS account. The name is case-sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTranscriptionJobRequest {
    /// <p>The name of the transcription job to be deleted.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVocabularyFilterRequest {
    /// <p>The name of the vocabulary filter to remove.</p>
    #[serde(rename = "VocabularyFilterName")]
    pub vocabulary_filter_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVocabularyRequest {
    /// <p>The name of the vocabulary to delete. </p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTranscriptionJobRequest {
    /// <p>The name of the job.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTranscriptionJobResponse {
    /// <p>An object that contains the results of the transcription job.</p>
    #[serde(rename = "TranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVocabularyFilterRequest {
    /// <p>The name of the vocabulary filter for which to return information.</p>
    #[serde(rename = "VocabularyFilterName")]
    pub vocabulary_filter_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetVocabularyFilterResponse {
    /// <p>The URI of the list of words in the vocabulary filter. You can use this URI to get the list of words.</p>
    #[serde(rename = "DownloadUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    /// <p>The language code of the words in the vocabulary filter.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date and time that the contents of the vocabulary filter were updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary filter.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetVocabularyRequest {
    /// <p>The name of the vocabulary to return information about. The name is case-sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Provides information about when a transcription job should be executed.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobExecutionSettings {
    /// <p>Indicates whether a job should be queued by Amazon Transcribe when the concurrent execution limit is exceeded. When the <code>AllowDeferredExecution</code> field is true, jobs are queued and will be executed when the number of executing jobs falls below the concurrent execution limit. If the field is false, Amazon Transcribe returns a <code>LimitExceededException</code> exception.</p> <p>If you specify the <code>AllowDeferredExecution</code> field, you must specify the <code>DataAccessRoleArn</code> field.</p>
    #[serde(rename = "AllowDeferredExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deferred_execution: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a role that has access to the S3 bucket that contains the input files. Amazon Transcribe will assume this role to read queued media files. If you have specified an output S3 bucket for the transcription results, this role should have access to the output bucket as well.</p> <p>If you specify the <code>AllowDeferredExecution</code> field, you must specify the <code>DataAccessRoleArn</code> field.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVocabularyFiltersRequest {
    /// <p>The maximum number of filters to return in the response. If there are fewer results in the list, this response contains only the actual results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Filters the response so that it only contains vocabulary filters whose name contains the specified string.</p>
    #[serde(rename = "NameContains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// <p>If the result of the previous request to <code>ListVocabularyFilters</code> was truncated, include the <code>NextToken</code> to fetch the next set of collections.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVocabularyFiltersResponse {
    /// <p>The <code>ListVocabularyFilters</code> operation returns a page of collections at a time. The maximum size of the page is set by the <code>MaxResults</code> parameter. If there are more jobs in the list than the page size, Amazon Transcribe returns the <code>NextPage</code> token. Include the token in the next request to the <code>ListVocabularyFilters</code> operation to return in the next page of jobs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of vocabulary filters. It will contain at most <code>MaxResults</code> number of filters. If there are more filters, call the <code>ListVocabularyFilters</code> operation again with the <code>NextToken</code> parameter in the request set to the value of the <code>NextToken</code> field in the response.</p>
    #[serde(rename = "VocabularyFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filters: Option<Vec<VocabularyFilterInfo>>,
}

/// <p>Describes the input media file in a transcription request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Media {
    /// <p>The S3 location of the input media file. The URI must be in the same region as the API endpoint that you are calling. The general form is:</p> <p> <code> https://s3.&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>For example:</p> <p> <code>https://s3.us-east-1.amazonaws.com/examplebucket/example.mp4</code> </p> <p> <code>https://s3.us-east-1.amazonaws.com/examplebucket/mediadocs/example.mp4</code> </p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p>
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
    /// <p>The number of alternative transcriptions that the service should return. If you specify the <code>MaxAlternatives</code> field, you must set the <code>ShowAlternatives</code> field to true.</p>
    #[serde(rename = "MaxAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_alternatives: Option<i64>,
    /// <p>The maximum number of speakers to identify in the input audio. If there are more speakers in the audio than this number, multiple speakers will be identified as a single speaker. If you specify the <code>MaxSpeakerLabels</code> field, you must set the <code>ShowSpeakerLabels</code> field to true.</p>
    #[serde(rename = "MaxSpeakerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speaker_labels: Option<i64>,
    /// <p>Determines whether the transcription contains alternative transcriptions. If you set the <code>ShowAlternatives</code> field to true, you must also set the maximum number of alternatives to return in the <code>MaxAlternatives</code> field.</p>
    #[serde(rename = "ShowAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alternatives: Option<bool>,
    /// <p>Determines whether the transcription job uses speaker recognition to identify different speakers in the input audio. Speaker recognition labels individual speakers in the audio file. If you set the <code>ShowSpeakerLabels</code> field to true, you must also set the maximum number of speaker labels <code>MaxSpeakerLabels</code> field.</p> <p>You can't set both <code>ShowSpeakerLabels</code> and <code>ChannelIdentification</code> in the same request. If you set both, your request returns a <code>BadRequestException</code>.</p>
    #[serde(rename = "ShowSpeakerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_speaker_labels: Option<bool>,
    /// <p>Set to <code>mask</code> to remove filtered text from the transcript and replace it with three asterisks ("***") as placeholder text. Set to <code>remove</code> to remove filtered text from the transcript without using placeholder text.</p>
    #[serde(rename = "VocabularyFilterMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_method: Option<String>,
    /// <p>The name of the vocabulary filter to use when transcribing the audio. The filter that you specify must have the same language code as the transcription job.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
    /// <p>The name of a vocabulary to use when processing the transcription job.</p>
    #[serde(rename = "VocabularyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartTranscriptionJobRequest {
    /// <p>Provides information about how a transcription job is executed. Use this field to indicate that the job can be queued for deferred execution if the concurrency limit is reached and there are no slots available to immediately run the job.</p>
    #[serde(rename = "JobExecutionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_settings: Option<JobExecutionSettings>,
    /// <p>The language code for the language used in the input media file.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>An object that describes the input media for a transcription job.</p>
    #[serde(rename = "Media")]
    pub media: Media,
    /// <p>The format of the input media file.</p>
    #[serde(rename = "MediaFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<String>,
    /// <p>The sample rate, in Hertz, of the audio track in the input media file. </p> <p>If you do not specify the media sample rate, Amazon Transcribe determines the sample rate. If you specify the sample rate, it must match the sample rate detected by Amazon Transcribe. In most cases, you should leave the <code>MediaSampleRateHertz</code> field blank and let Amazon Transcribe determine the sample rate.</p>
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i64>,
    /// <p>The location where the transcription is stored.</p> <p>If you set the <code>OutputBucketName</code>, Amazon Transcribe puts the transcription in the specified S3 bucket. When you call the <a>GetTranscriptionJob</a> operation, the operation returns this location in the <code>TranscriptFileUri</code> field. The S3 bucket must have permissions that allow Amazon Transcribe to put files in the bucket. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/security_iam_id-based-policy-examples.html#auth-role-iam-user">Permissions Required for IAM User Roles</a>.</p> <p>You can specify an AWS Key Management Service (KMS) key to encrypt the output of your transcription using the <code>OutputEncryptionKMSKeyId</code> parameter. If you don't specify a KMS key, Amazon Transcribe uses the default Amazon S3 key for server-side encryption of transcripts that are placed in your S3 bucket.</p> <p>If you don't set the <code>OutputBucketName</code>, Amazon Transcribe generates a pre-signed URL, a shareable URL that provides secure access to your transcription, and returns it in the <code>TranscriptFileUri</code> field. Use this URL to download the transcription.</p>
    #[serde(rename = "OutputBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bucket_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Key Management Service (KMS) key used to encrypt the output of the transcription job. The user calling the <code>StartTranscriptionJob</code> operation must have permission to use the specified KMS key.</p> <p>You can use either of the following to identify a KMS key in the current account:</p> <ul> <li> <p>KMS Key ID: "1234abcd-12ab-34cd-56ef-1234567890ab"</p> </li> <li> <p>KMS Key Alias: "alias/ExampleAlias"</p> </li> </ul> <p>You can use either of the following to identify a KMS key in the current account or another account:</p> <ul> <li> <p>Amazon Resource Name (ARN) of a KMS Key: "arn:aws:kms:region:account ID:key/1234abcd-12ab-34cd-56ef-1234567890ab"</p> </li> <li> <p>ARN of a KMS Key Alias: "arn:aws:kms:region:account ID:alias/ExampleAlias"</p> </li> </ul> <p>If you don't specify an encryption key, the output of the transcription job is encrypted with the default Amazon S3 key (SSE-S3). </p> <p>If you specify a KMS key to encrypt your output, you must also specify an output location in the <code>OutputBucketName</code> parameter.</p>
    #[serde(rename = "OutputEncryptionKMSKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encryption_kms_key_id: Option<String>,
    /// <p>A <code>Settings</code> object that provides optional settings for a transcription job.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    /// <p>The name of the job. Note that you can't use the strings "." or ".." by themselves as the job name. The name must also be unique within an AWS account.</p>
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTranscriptionJobResponse {
    /// <p>An object containing details of the asynchronous transcription job.</p>
    #[serde(rename = "TranscriptionJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

/// <p>Identifies the location of a transcription.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Transcript {
    /// <p>The location where the transcription is stored.</p> <p>Use this URI to access the transcription. If you specified an S3 bucket in the <code>OutputBucketName</code> field when you created the job, this is the URI of that bucket. If you chose to store the transcription in Amazon Transcribe, this is a shareable URL that provides secure access to that location.</p>
    #[serde(rename = "TranscriptFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_file_uri: Option<String>,
}

/// <p>Describes an asynchronous transcription job that was created with the <code>StartTranscriptionJob</code> operation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Provides information about how a transcription job is executed.</p>
    #[serde(rename = "JobExecutionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_settings: Option<JobExecutionSettings>,
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
    /// <p>A timestamp that shows with the job was started processing.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>A timestamp that shows when the job started processing.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVocabularyFilterRequest {
    /// <p>The Amazon S3 location of a text file used as input to create the vocabulary filter. Only use characters from the character set defined for custom vocabularies. For a list of character sets, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-vocabulary.html#charsets">Character Sets for Custom Vocabularies</a>.</p> <p>The specified file must be less than 50 KB of UTF-8 characters.</p> <p>If you provide the location of a list of words in the <code>VocabularyFilterFileUri</code> parameter, you can't use the <code>Words</code> parameter.</p>
    #[serde(rename = "VocabularyFilterFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_file_uri: Option<String>,
    /// <p>The name of the vocabulary filter to update.</p>
    #[serde(rename = "VocabularyFilterName")]
    pub vocabulary_filter_name: String,
    /// <p>The words to use in the vocabulary filter. Only use characters from the character set defined for custom vocabularies. For a list of character sets, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-vocabulary.html#charsets">Character Sets for Custom Vocabularies</a>.</p> <p>If you provide a list of words in the <code>Words</code> parameter, you can't use the <code>VocabularyFilterFileUri</code> parameter.</p>
    #[serde(rename = "Words")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVocabularyFilterResponse {
    /// <p>The language code of the words in the vocabulary filter.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date and time that the vocabulary filter was updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the updated vocabulary filter.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVocabularyRequest {
    /// <p>The language code of the vocabulary entries.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>An array of strings containing the vocabulary entries.</p>
    #[serde(rename = "Phrases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
    /// <p>The S3 location of the text file that contains the definition of the custom vocabulary. The URI must be in the same region as the API endpoint that you are calling. The general form is </p> <p> <code> https://s3.&lt;aws-region&gt;.amazonaws.com/&lt;bucket-name&gt;/&lt;keyprefix&gt;/&lt;objectkey&gt; </code> </p> <p>For example:</p> <p> <code>https://s3.us-east-1.amazonaws.com/examplebucket/vocab.txt</code> </p> <p>For more information about S3 object names, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys">Object Keys</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>For more information about custom vocabularies, see <a href="http://docs.aws.amazon.com/transcribe/latest/dg/how-it-works.html#how-vocabulary">Custom Vocabularies</a>.</p>
    #[serde(rename = "VocabularyFileUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    /// <p>The name of the vocabulary to update. The name is case-sensitive.</p>
    #[serde(rename = "VocabularyName")]
    pub vocabulary_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Provides information about a vocabulary filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VocabularyFilterInfo {
    /// <p>The language code of the words in the vocabulary filter.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The date and time that the vocabulary was last updated.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>The name of the vocabulary filter. The name must be unique in the account that holds the filter.</p>
    #[serde(rename = "VocabularyFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

/// <p>Provides information about a custom vocabulary. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>When you are using the <code>CreateVocabulary</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVocabularyError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVocabularyError {}
/// Errors returned by CreateVocabularyFilter
#[derive(Debug, PartialEq)]
pub enum CreateVocabularyFilterError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>CreateVocabulary</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
    Conflict(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl CreateVocabularyFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVocabularyFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVocabularyFilterError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateVocabularyFilterError::Conflict(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateVocabularyFilterError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateVocabularyFilterError::LimitExceeded(
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
impl fmt::Display for CreateVocabularyFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVocabularyFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateVocabularyFilterError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateVocabularyFilterError::InternalFailure(ref cause) => write!(f, "{}", cause),
            CreateVocabularyFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVocabularyFilterError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteTranscriptionJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTranscriptionJobError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVocabularyError {}
/// Errors returned by DeleteVocabularyFilter
#[derive(Debug, PartialEq)]
pub enum DeleteVocabularyFilterError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl DeleteVocabularyFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVocabularyFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVocabularyFilterError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteVocabularyFilterError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteVocabularyFilterError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVocabularyFilterError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteVocabularyFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVocabularyFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyFilterError::InternalFailure(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteVocabularyFilterError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVocabularyFilterError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetTranscriptionJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetTranscriptionJobError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTranscriptionJobError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVocabularyError {}
/// Errors returned by GetVocabularyFilter
#[derive(Debug, PartialEq)]
pub enum GetVocabularyFilterError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl GetVocabularyFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVocabularyFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVocabularyFilterError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetVocabularyFilterError::InternalFailure(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetVocabularyFilterError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetVocabularyFilterError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVocabularyFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetVocabularyFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetVocabularyFilterError::InternalFailure(ref cause) => write!(f, "{}", cause),
            GetVocabularyFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetVocabularyFilterError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetVocabularyFilterError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTranscriptionJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTranscriptionJobsError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListTranscriptionJobsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTranscriptionJobsError {}
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVocabulariesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVocabulariesError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListVocabulariesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVocabulariesError {}
/// Errors returned by ListVocabularyFilters
#[derive(Debug, PartialEq)]
pub enum ListVocabularyFiltersError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
}

impl ListVocabularyFiltersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVocabularyFiltersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListVocabularyFiltersError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListVocabularyFiltersError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListVocabularyFiltersError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListVocabularyFiltersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVocabularyFiltersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListVocabularyFiltersError::InternalFailure(ref cause) => write!(f, "{}", cause),
            ListVocabularyFiltersError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVocabularyFiltersError {}
/// Errors returned by StartTranscriptionJob
#[derive(Debug, PartialEq)]
pub enum StartTranscriptionJobError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>CreateVocabulary</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartTranscriptionJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartTranscriptionJobError::Conflict(ref cause) => write!(f, "{}", cause),
            StartTranscriptionJobError::InternalFailure(ref cause) => write!(f, "{}", cause),
            StartTranscriptionJobError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartTranscriptionJobError {}
/// Errors returned by UpdateVocabulary
#[derive(Debug, PartialEq)]
pub enum UpdateVocabularyError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>When you are using the <code>CreateVocabulary</code> operation, the <code>JobName</code> field is a duplicate of a previously entered job name. Resend your request with a different name.</p> <p>When you are using the <code>UpdateVocabulary</code> operation, there are two jobs running at the same time. Resend the second request later.</p>
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVocabularyError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVocabularyError {}
/// Errors returned by UpdateVocabularyFilter
#[derive(Debug, PartialEq)]
pub enum UpdateVocabularyFilterError {
    /// <p>Your request didn't pass one or more validation tests. For example, if the transcription you're trying to delete doesn't exist or if it is in a non-terminal state (for example, it's "in progress"). See the exception <code>Message</code> field for more information.</p>
    BadRequest(String),
    /// <p>There was an internal error. Check the error message and try your request again.</p>
    InternalFailure(String),
    /// <p>Either you have sent too many requests or your input file is too long. Wait before you resend your request, or use a smaller file and resend the request.</p>
    LimitExceeded(String),
    /// <p>We can't find the requested resource. Check the name and try your request again.</p>
    NotFound(String),
}

impl UpdateVocabularyFilterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVocabularyFilterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVocabularyFilterError::BadRequest(err.msg))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateVocabularyFilterError::InternalFailure(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateVocabularyFilterError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVocabularyFilterError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateVocabularyFilterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVocabularyFilterError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyFilterError::InternalFailure(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyFilterError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateVocabularyFilterError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVocabularyFilterError {}
/// Trait representing the capabilities of the Amazon Transcribe Service API. Amazon Transcribe Service clients implement this trait.
#[async_trait]
pub trait Transcribe {
    /// <p>Creates a new custom vocabulary that you can use to change the way Amazon Transcribe handles transcription of an audio file. </p>
    async fn create_vocabulary(
        &self,
        input: CreateVocabularyRequest,
    ) -> Result<CreateVocabularyResponse, RusotoError<CreateVocabularyError>>;

    /// <p>Creates a new vocabulary filter that you can use to filter words, such as profane words, from the output of a transcription job.</p>
    async fn create_vocabulary_filter(
        &self,
        input: CreateVocabularyFilterRequest,
    ) -> Result<CreateVocabularyFilterResponse, RusotoError<CreateVocabularyFilterError>>;

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

    /// <p>Removes a vocabulary filter.</p>
    async fn delete_vocabulary_filter(
        &self,
        input: DeleteVocabularyFilterRequest,
    ) -> Result<(), RusotoError<DeleteVocabularyFilterError>>;

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

    /// <p>Returns information about a vocabulary filter.</p>
    async fn get_vocabulary_filter(
        &self,
        input: GetVocabularyFilterRequest,
    ) -> Result<GetVocabularyFilterResponse, RusotoError<GetVocabularyFilterError>>;

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

    /// <p>Gets information about vocabulary filters.</p>
    async fn list_vocabulary_filters(
        &self,
        input: ListVocabularyFiltersRequest,
    ) -> Result<ListVocabularyFiltersResponse, RusotoError<ListVocabularyFiltersError>>;

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

    /// <p>Updates a vocabulary filter with a new list of filtered words.</p>
    async fn update_vocabulary_filter(
        &self,
        input: UpdateVocabularyFilterRequest,
    ) -> Result<UpdateVocabularyFilterResponse, RusotoError<UpdateVocabularyFilterError>>;
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

    pub fn new_with_client(client: Client, region: region::Region) -> TranscribeClient {
        TranscribeClient { client, region }
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

        let mut response = self
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

    /// <p>Creates a new vocabulary filter that you can use to filter words, such as profane words, from the output of a transcription job.</p>
    async fn create_vocabulary_filter(
        &self,
        input: CreateVocabularyFilterRequest,
    ) -> Result<CreateVocabularyFilterResponse, RusotoError<CreateVocabularyFilterError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.CreateVocabularyFilter");
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
                .deserialize::<CreateVocabularyFilterResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVocabularyFilterError::from_response(response))
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
            Err(DeleteVocabularyError::from_response(response))
        }
    }

    /// <p>Removes a vocabulary filter.</p>
    async fn delete_vocabulary_filter(
        &self,
        input: DeleteVocabularyFilterRequest,
    ) -> Result<(), RusotoError<DeleteVocabularyFilterError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.DeleteVocabularyFilter");
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
            Err(DeleteVocabularyFilterError::from_response(response))
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

        let mut response = self
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

        let mut response = self
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

    /// <p>Returns information about a vocabulary filter.</p>
    async fn get_vocabulary_filter(
        &self,
        input: GetVocabularyFilterRequest,
    ) -> Result<GetVocabularyFilterResponse, RusotoError<GetVocabularyFilterError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.GetVocabularyFilter");
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
                .deserialize::<GetVocabularyFilterResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetVocabularyFilterError::from_response(response))
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

        let mut response = self
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

        let mut response = self
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

    /// <p>Gets information about vocabulary filters.</p>
    async fn list_vocabulary_filters(
        &self,
        input: ListVocabularyFiltersRequest,
    ) -> Result<ListVocabularyFiltersResponse, RusotoError<ListVocabularyFiltersError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.ListVocabularyFilters");
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
                .deserialize::<ListVocabularyFiltersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListVocabularyFiltersError::from_response(response))
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

        let mut response = self
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

        let mut response = self
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

    /// <p>Updates a vocabulary filter with a new list of filtered words.</p>
    async fn update_vocabulary_filter(
        &self,
        input: UpdateVocabularyFilterRequest,
    ) -> Result<UpdateVocabularyFilterResponse, RusotoError<UpdateVocabularyFilterError>> {
        let mut request = SignedRequest::new("POST", "transcribe", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Transcribe.UpdateVocabularyFilter");
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
                .deserialize::<UpdateVocabularyFilterResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateVocabularyFilterError::from_response(response))
        }
    }
}
