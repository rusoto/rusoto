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
/// <p>The result of calling the operation. The operation returns one object for each document that is successfully processed by the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectDominantLanguageItemResult {
    /// <p>The zero-based index of the document in the input list.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>One or more <a>DominantLanguage</a> objects describing the dominant languages in the document.</p>
    #[serde(rename = "Languages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<DominantLanguage>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetectDominantLanguageRequest {
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document should contain at least 20 characters and must contain fewer than 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectDominantLanguageResponse {
    /// <p>A list containing one object for each document that contained an error. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If there are no errors in the batch, the <code>ErrorList</code> is empty.</p>
    #[serde(rename = "ErrorList")]
    pub error_list: Vec<BatchItemError>,
    /// <p>A list of objects containing the results of the operation. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If all of the documents contain an error, the <code>ResultList</code> is empty.</p>
    #[serde(rename = "ResultList")]
    pub result_list: Vec<BatchDetectDominantLanguageItemResult>,
}

/// <p>The result of calling the operation. The operation returns one object for each document that is successfully processed by the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectEntitiesItemResult {
    /// <p>One or more <a>Entity</a> objects, one for each entity detected in the document.</p>
    #[serde(rename = "Entities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    /// <p>The zero-based index of the document in the input list.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetectEntitiesRequest {
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document must contain fewer than 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectEntitiesResponse {
    /// <p>A list containing one object for each document that contained an error. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If there are no errors in the batch, the <code>ErrorList</code> is empty.</p>
    #[serde(rename = "ErrorList")]
    pub error_list: Vec<BatchItemError>,
    /// <p>A list of objects containing the results of the operation. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If all of the documents contain an error, the <code>ResultList</code> is empty.</p>
    #[serde(rename = "ResultList")]
    pub result_list: Vec<BatchDetectEntitiesItemResult>,
}

/// <p>The result of calling the operation. The operation returns one object for each document that is successfully processed by the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectKeyPhrasesItemResult {
    /// <p>The zero-based index of the document in the input list.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>One or more <a>KeyPhrase</a> objects, one for each key phrase detected in the document.</p>
    #[serde(rename = "KeyPhrases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases: Option<Vec<KeyPhrase>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetectKeyPhrasesRequest {
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectKeyPhrasesResponse {
    /// <p>A list containing one object for each document that contained an error. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If there are no errors in the batch, the <code>ErrorList</code> is empty.</p>
    #[serde(rename = "ErrorList")]
    pub error_list: Vec<BatchItemError>,
    /// <p>A list of objects containing the results of the operation. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If all of the documents contain an error, the <code>ResultList</code> is empty.</p>
    #[serde(rename = "ResultList")]
    pub result_list: Vec<BatchDetectKeyPhrasesItemResult>,
}

/// <p>The result of calling the operation. The operation returns one object for each document that is successfully processed by the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectSentimentItemResult {
    /// <p>The zero-based index of the document in the input list.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The sentiment detected in the document.</p>
    #[serde(rename = "Sentiment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
    /// <p>The level of confidence that Amazon Comprehend has in the accuracy of its sentiment detection.</p>
    #[serde(rename = "SentimentScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_score: Option<SentimentScore>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetectSentimentRequest {
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectSentimentResponse {
    /// <p>A list containing one object for each document that contained an error. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If there are no errors in the batch, the <code>ErrorList</code> is empty.</p>
    #[serde(rename = "ErrorList")]
    pub error_list: Vec<BatchItemError>,
    /// <p>A list of objects containing the results of the operation. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If all of the documents contain an error, the <code>ResultList</code> is empty.</p>
    #[serde(rename = "ResultList")]
    pub result_list: Vec<BatchDetectSentimentItemResult>,
}

/// <p>The result of calling the operation. The operation returns one object that is successfully processed by the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectSyntaxItemResult {
    /// <p>The zero-based index of the document in the input list.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The syntax tokens for the words in the document, one token for each word.</p>
    #[serde(rename = "SyntaxTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_tokens: Option<Vec<SyntaxToken>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchDetectSyntaxRequest {
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchDetectSyntaxResponse {
    /// <p>A list containing one object for each document that contained an error. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If there are no errors in the batch, the <code>ErrorList</code> is empty.</p>
    #[serde(rename = "ErrorList")]
    pub error_list: Vec<BatchItemError>,
    /// <p>A list of objects containing the results of the operation. The results are sorted in ascending order by the <code>Index</code> field and match the order of the documents in the input list. If all of the documents contain an error, the <code>ResultList</code> is empty.</p>
    #[serde(rename = "ResultList")]
    pub result_list: Vec<BatchDetectSyntaxItemResult>,
}

/// <p>Describes an error that occurred while processing a document in a batch. The operation returns on <code>BatchItemError</code> object for each document that contained an error.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BatchItemError {
    /// <p>The numeric error code of the error.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A text description of the error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The zero-based index of the document in the input list.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDominantLanguageDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDominantLanguageDetectionJobResponse {
    /// <p>An object that contains the properties associated with a dominant language detection job.</p>
    #[serde(rename = "DominantLanguageDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_language_detection_job_properties: Option<DominantLanguageDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEntitiesDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeEntitiesDetectionJobResponse {
    /// <p>An object that contains the properties associated with an entities detection job.</p>
    #[serde(rename = "EntitiesDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_detection_job_properties: Option<EntitiesDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeKeyPhrasesDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeKeyPhrasesDetectionJobResponse {
    /// <p>An object that contains the properties associated with a key phrases detection job. </p>
    #[serde(rename = "KeyPhrasesDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases_detection_job_properties: Option<KeyPhrasesDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSentimentDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSentimentDetectionJobResponse {
    /// <p>An object that contains the properties associated with a sentiment detection job.</p>
    #[serde(rename = "SentimentDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_detection_job_properties: Option<SentimentDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTopicsDetectionJobRequest {
    /// <p>The identifier assigned by the user to the detection job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeTopicsDetectionJobResponse {
    /// <p>The list of properties for the requested job.</p>
    #[serde(rename = "TopicsDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics_detection_job_properties: Option<TopicsDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectDominantLanguageRequest {
    /// <p>A UTF-8 text string. Each string should contain at least 20 characters and must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetectDominantLanguageResponse {
    /// <p>The languages that Amazon Comprehend detected in the input text. For each language, the response returns the RFC 5646 language code and the level of confidence that Amazon Comprehend has in the accuracy of its inference. For more information about RFC 5646, see <a href="https://tools.ietf.org/html/rfc5646">Tags for Identifying Languages</a> on the <i>IETF Tools</i> web site.</p>
    #[serde(rename = "Languages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<DominantLanguage>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectEntitiesRequest {
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A UTF-8 text string. Each string must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetectEntitiesResponse {
    /// <p>A collection of entities identified in the input text. For each entity, the response provides the entity text, entity type, where the entity text begins and ends, and the level of confidence that Amazon Comprehend has in the detection. For a list of entity types, see <a>how-entities</a>. </p>
    #[serde(rename = "Entities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectKeyPhrasesRequest {
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A UTF-8 text string. Each string must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetectKeyPhrasesResponse {
    /// <p>A collection of key phrases that Amazon Comprehend identified in the input text. For each key phrase, the response provides the text of the key phrase, where the key phrase begins and ends, and the level of confidence that Amazon Comprehend has in the accuracy of the detection. </p>
    #[serde(rename = "KeyPhrases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases: Option<Vec<KeyPhrase>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectSentimentRequest {
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A UTF-8 text string. Each string must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetectSentimentResponse {
    /// <p>The inferred sentiment that Amazon Comprehend has the highest level of confidence in.</p>
    #[serde(rename = "Sentiment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
    /// <p>An object that lists the sentiments, and their corresponding confidence levels.</p>
    #[serde(rename = "SentimentScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_score: Option<SentimentScore>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectSyntaxRequest {
    /// <p>The language code of the input documents. You can specify English ("en") or Spanish ("es").</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A UTF-8 string. Each string must contain fewer that 5,000 bytes of UTF encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DetectSyntaxResponse {
    /// <p>A collection of syntax tokens describing the text. For each token, the response provides the text, the token type, where the text begins and ends, and the level of confidence that Amazon Comprehend has that the token is correct. For a list of token types, see <a>how-syntax</a>.</p>
    #[serde(rename = "SyntaxTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_tokens: Option<Vec<SyntaxToken>>,
}

/// <p>Returns the code for the dominant language in the input text and the level of confidence that Amazon Comprehend has in the accuracy of the detection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DominantLanguage {
    /// <p>The RFC 5646 language code for the dominant language. For more information about RFC 5646, see <a href="https://tools.ietf.org/html/rfc5646">Tags for Identifying Languages</a> on the <i>IETF Tools</i> web site.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The level of confidence that Amazon Comprehend has in the accuracy of the detection.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

/// <p>Provides information for filtering a list of dominant language detection jobs. For more information, see the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DominantLanguageDetectionJobFilter {
    /// <p>Filters on the name of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Filters the list of jobs based on job status. Returns only jobs with the specified status.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted after the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted before the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>Provides information about a dominant language detection job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DominantLanguageDetectionJobProperties {
    /// <p>The Amazon Resource Name (ARN) that gives Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time that the dominant language detection job completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input data configuration that you supplied when you created the dominant language detection job.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The identifier assigned to the dominant language detection job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name that you assigned to the dominant language detection job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current status of the dominant language detection job. If the status is <code>FAILED</code>, the <code>Message</code> field shows the reason for the failure.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>A description for the status of a job.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The output data configuration that you supplied when you created the dominant language detection job.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>The time that the dominant language detection job was submitted for processing.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

/// <p>Provides information for filtering a list of dominant language detection jobs. For more information, see the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EntitiesDetectionJobFilter {
    /// <p>Filters on the name of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Filters the list of jobs based on job status. Returns only jobs with the specified status.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted after the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted before the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>Provides information about an entities detection job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EntitiesDetectionJobProperties {
    /// <p>The Amazon Resource Name (ARN) that gives Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time that the entities detection job completed</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input data configuration that you supplied when you created the entities detection job.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The identifier assigned to the entities detection job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name that you assigned the entities detection job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current status of the entities detection job. If the status is <code>FAILED</code>, the <code>Message</code> field shows the reason for the failure.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The language code of the input documents.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>A description of the status of a job.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The output data configuration that you supplied when you created the entities detection job. </p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>The time that the entities detection job was submitted for processing.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

/// <p>Provides information about an entity. </p> <p> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Entity {
    /// <p>A character offset in the input text that shows where the entity begins (the first character is at position 0). The offset returns the position of each UTF-8 code point in the string. A <i>code point</i> is the abstract character from a particular graphical representation. For example, a multi-byte UTF-8 character maps to a single code point.</p>
    #[serde(rename = "BeginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p>A character offset in the input text that shows where the entity ends. The offset returns the position of each UTF-8 code point in the string. A <i>code point</i> is the abstract character from a particular graphical representation. For example, a multi-byte UTF-8 character maps to a single code point. </p>
    #[serde(rename = "EndOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p>The level of confidence that Amazon Comprehend has in the accuracy of the detection.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p>The text of the entity.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>The entity's type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The input properties for a topic detection job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputDataConfig {
    /// <p><p>Specifies how the text in an input file should be processed:</p> <ul> <li> <p> <code>ONE<em>DOC</em>PER<em>FILE</code> - Each file is considered a separate document. Use this option when you are processing large documents, such as newspaper articles or scientific papers.</p> </li> <li> <p> <code>ONE</em>DOC<em>PER</em>LINE</code> - Each line in a file is considered a separate document. Use this option when you are processing many short documents, such as text messages.</p> </li> </ul></p>
    #[serde(rename = "InputFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    /// <p>The Amazon S3 URI for the input data. The URI must be in same region as the API endpoint that you are calling. The URI can point to a single input file or it can provide the prefix for a collection of data files. </p> <p>For example, if you use the URI <code>S3://bucketName/prefix</code>, if the prefix is a single file, Amazon Comprehend uses that file as input. If more than one file begins with the prefix, Amazon Comprehend uses all of them as input.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Describes a key noun phrase.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KeyPhrase {
    /// <p>A character offset in the input text that shows where the key phrase begins (the first character is at position 0). The offset returns the position of each UTF-8 code point in the string. A <i>code point</i> is the abstract character from a particular graphical representation. For example, a multi-byte UTF-8 character maps to a single code point.</p>
    #[serde(rename = "BeginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p>A character offset in the input text where the key phrase ends. The offset returns the position of each UTF-8 code point in the string. A <code>code point</code> is the abstract character from a particular graphical representation. For example, a multi-byte UTF-8 character maps to a single code point.</p>
    #[serde(rename = "EndOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p>The level of confidence that Amazon Comprehend has in the accuracy of the detection.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p>The text of a key noun phrase.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// <p>Provides information for filtering a list of dominant language detection jobs. For more information, see the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KeyPhrasesDetectionJobFilter {
    /// <p>Filters on the name of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Filters the list of jobs based on job status. Returns only jobs with the specified status.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted after the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted before the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>Provides information about a key phrases detection job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KeyPhrasesDetectionJobProperties {
    /// <p>The Amazon Resource Name (ARN) that gives Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time that the key phrases detection job completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input data configuration that you supplied when you created the key phrases detection job.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The identifier assigned to the key phrases detection job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name that you assigned the key phrases detection job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current status of the key phrases detection job. If the status is <code>FAILED</code>, the <code>Message</code> field shows the reason for the failure.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The language code of the input documents.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>A description of the status of a job.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The output data configuration that you supplied when you created the key phrases detection job.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>The time that the key phrases detection job was submitted for processing.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDominantLanguageDetectionJobsRequest {
    /// <p>Filters that jobs that are returned. You can filter jobs on their name, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DominantLanguageDetectionJobFilter>,
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDominantLanguageDetectionJobsResponse {
    /// <p>A list containing the properties of each job that is returned.</p>
    #[serde(rename = "DominantLanguageDetectionJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_language_detection_job_properties_list:
        Option<Vec<DominantLanguageDetectionJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEntitiesDetectionJobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs on their name, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EntitiesDetectionJobFilter>,
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListEntitiesDetectionJobsResponse {
    /// <p>A list containing the properties of each job that is returned.</p>
    #[serde(rename = "EntitiesDetectionJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_detection_job_properties_list: Option<Vec<EntitiesDetectionJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListKeyPhrasesDetectionJobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs on their name, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<KeyPhrasesDetectionJobFilter>,
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListKeyPhrasesDetectionJobsResponse {
    /// <p>A list containing the properties of each job that is returned.</p>
    #[serde(rename = "KeyPhrasesDetectionJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases_detection_job_properties_list: Option<Vec<KeyPhrasesDetectionJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSentimentDetectionJobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs on their name, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SentimentDetectionJobFilter>,
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListSentimentDetectionJobsResponse {
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list containing the properties of each job that is returned.</p>
    #[serde(rename = "SentimentDetectionJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_detection_job_properties_list: Option<Vec<SentimentDetectionJobProperties>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTopicsDetectionJobsRequest {
    /// <p>Filters the jobs that are returned. Jobs can be filtered on their name, status, or the date and time that they were submitted. You can set only one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TopicsDetectionJobFilter>,
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTopicsDetectionJobsResponse {
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list containing the properties of each job that is returned.</p>
    #[serde(rename = "TopicsDetectionJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics_detection_job_properties_list: Option<Vec<TopicsDetectionJobProperties>>,
}

/// <p><p>Provides configuration parameters for the output of topic detection jobs.</p> <p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputDataConfig {
    /// <p>When you use the <code>OutputDataConfig</code> object with asynchronous operations, you specify the Amazon S3 location where you want to write the output data. The URI must be in the same region as the API endpoint that you are calling. The location is used as the prefix for the actual location of the output file.</p> <p>When the topic detection job is finished, the service creates an output file in a directory specific to the job. The <code>S3Uri</code> field contains the location of the output file, called <code>output.tar.gz</code>. It is a compressed archive that contains the ouput of the operation.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Identifies the part of speech represented by the token and gives the confidence that Amazon Comprehend has that the part of speech was correctly identified. For more information about the parts of speech that Amazon Comprehend can identify, see <a>how-syntax</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PartOfSpeechTag {
    /// <p>The confidence that Amazon Comprehend has that the part of speech was correctly identified.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p>Identifies the part of speech that the token represents.</p>
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// <p>Provides information for filtering a list of dominant language detection jobs. For more information, see the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SentimentDetectionJobFilter {
    /// <p>Filters on the name of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Filters the list of jobs based on job status. Returns only jobs with the specified status.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted after the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted before the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>Provides information about a sentiment detection job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SentimentDetectionJobProperties {
    /// <p>The Amazon Resource Name (ARN) that gives Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time that the sentiment detection job ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input data configuration that you supplied when you created the sentiment detection job.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The identifier assigned to the sentiment detection job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name that you assigned to the sentiment detection job</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current status of the sentiment detection job. If the status is <code>FAILED</code>, the <code>Messages</code> field shows the reason for the failure.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The language code of the input documents.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>A description of the status of a job.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The output data configuration that you supplied when you created the sentiment detection job.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>The time that the sentiment detection job was submitted for processing.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

/// <p>Describes the level of confidence that Amazon Comprehend has in the accuracy of its detection of sentiments.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SentimentScore {
    /// <p>The level of confidence that Amazon Comprehend has in the accuracy of its detection of the <code>MIXED</code> sentiment.</p>
    #[serde(rename = "Mixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed: Option<f32>,
    /// <p>The level of confidence that Amazon Comprehend has in the accuracy of its detection of the <code>NEGATIVE</code> sentiment.</p>
    #[serde(rename = "Negative")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative: Option<f32>,
    /// <p>The level of confidence that Amazon Comprehend has in the accuracy of its detection of the <code>NEUTRAL</code> sentiment.</p>
    #[serde(rename = "Neutral")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral: Option<f32>,
    /// <p>The level of confidence that Amazon Comprehend has in the accuracy of its detection of the <code>POSITIVE</code> sentiment.</p>
    #[serde(rename = "Positive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartDominantLanguageDetectionJobRequest {
    /// <p>A unique identifier for the request. If you do not set the client request token, Amazon Comprehend generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions">https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions</a>.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>An identifier for the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartDominantLanguageDetectionJobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this identifier with the operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p><p>The status of the job. </p> <ul> <li> <p>SUBMITTED - The job has been received and is queued for processing.</p> </li> <li> <p>IN_PROGRESS - Amazon Comprehend is processing the job.</p> </li> <li> <p>COMPLETED - The job was successfully completed and the output is available.</p> </li> <li> <p>FAILED - The job did not complete. To get details, use the operation.</p> </li> </ul></p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartEntitiesDetectionJobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions">https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions</a>.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartEntitiesDetectionJobResponse {
    /// <p>The identifier generated for the job. To get the status of job, use this identifier with the operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p><p>The status of the job. </p> <ul> <li> <p>SUBMITTED - The job has been received and is queued for processing.</p> </li> <li> <p>IN_PROGRESS - Amazon Comprehend is processing the job.</p> </li> <li> <p>COMPLETED - The job was successfully completed and the output is available.</p> </li> <li> <p>FAILED - The job did not complete. To get details, use the operation.</p> </li> </ul></p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartKeyPhrasesDetectionJobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions">https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions</a>.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartKeyPhrasesDetectionJobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this identifier with the operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p><p>The status of the job. </p> <ul> <li> <p>SUBMITTED - The job has been received and is queued for processing.</p> </li> <li> <p>IN_PROGRESS - Amazon Comprehend is processing the job.</p> </li> <li> <p>COMPLETED - The job was successfully completed and the output is available.</p> </li> <li> <p>FAILED - The job did not complete. To get details, use the operation.</p> </li> </ul></p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartSentimentDetectionJobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions">https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions</a>.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files. </p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartSentimentDetectionJobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this identifier with the operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p><p>The status of the job. </p> <ul> <li> <p>SUBMITTED - The job has been received and is queued for processing.</p> </li> <li> <p>IN_PROGRESS - Amazon Comprehend is processing the job.</p> </li> <li> <p>COMPLETED - The job was successfully completed and the output is available.</p> </li> <li> <p>FAILED - The job did not complete. To get details, use the operation.</p> </li> </ul></p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartTopicsDetectionJobRequest {
    /// <p>A unique identifier for the request. If you do not set the client request token, Amazon Comprehend generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions">https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions.html#auth-role-permissions</a>.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The number of topics to detect.</p>
    #[serde(rename = "NumberOfTopics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_topics: Option<i64>,
    /// <p>Specifies where to send the output files. The output is a compressed archive with two files, <code>topic-terms.csv</code> that lists the terms associated with each topic, and <code>doc-topics.csv</code> that lists the documents associated with each topic</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartTopicsDetectionJobResponse {
    /// <p>The identifier generated for the job. To get the status of the job, use this identifier with the <code>DescribeTopicDetectionJob</code> operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p><p>The status of the job: </p> <ul> <li> <p>SUBMITTED - The job has been received and is queued for processing.</p> </li> <li> <p>IN_PROGRESS - Amazon Comprehend is processing the job.</p> </li> <li> <p>COMPLETED - The job was successfully completed and the output is available.</p> </li> <li> <p>FAILED - The job did not complete. To get details, use the <code>DescribeTopicDetectionJob</code> operation.</p> </li> </ul></p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopDominantLanguageDetectionJobRequest {
    /// <p>The identifier of the dominant language detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopDominantLanguageDetectionJobResponse {
    /// <p>The identifier of the dominant language detection job to stop.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>Either <code>STOP_REQUESTED</code> if the job is currently running, or <code>STOPPED</code> if the job was previously stopped with the <code>StopDominantLanguageDetectionJob</code> operation.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopEntitiesDetectionJobRequest {
    /// <p>The identifier of the entities detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopEntitiesDetectionJobResponse {
    /// <p>The identifier of the entities detection job to stop.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>Either <code>STOP_REQUESTED</code> if the job is currently running, or <code>STOPPED</code> if the job was previously stopped with the <code>StopEntitiesDetectionJob</code> operation.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopKeyPhrasesDetectionJobRequest {
    /// <p>The identifier of the key phrases detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopKeyPhrasesDetectionJobResponse {
    /// <p>The identifier of the key phrases detection job to stop.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>Either <code>STOP_REQUESTED</code> if the job is currently running, or <code>STOPPED</code> if the job was previously stopped with the <code>StopKeyPhrasesDetectionJob</code> operation.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopSentimentDetectionJobRequest {
    /// <p>The identifier of the sentiment detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopSentimentDetectionJobResponse {
    /// <p>The identifier of the sentiment detection job to stop.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>Either <code>STOP_REQUESTED</code> if the job is currently running, or <code>STOPPED</code> if the job was previously stopped with the <code>StopSentimentDetectionJob</code> operation.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

/// <p>Represents a work in the input text that was recognized and assigned a part of speech. There is one syntax token record for each word in the source text.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SyntaxToken {
    /// <p>The zero-based offset from the beginning of the source text to the first character in the word.</p>
    #[serde(rename = "BeginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p>The zero-based offset from the beginning of the source text to the last character in the word.</p>
    #[serde(rename = "EndOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p>Provides the part of speech label and the confidence level that Amazon Comprehend has that the part of speech was correctly identified. For more information, see <a>how-syntax</a>.</p>
    #[serde(rename = "PartOfSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_speech: Option<PartOfSpeechTag>,
    /// <p>The word that was recognized in the source text.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>A unique identifier for a token.</p>
    #[serde(rename = "TokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<i64>,
}

/// <p>Provides information for filtering topic detection jobs. For more information, see .</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TopicsDetectionJobFilter {
    /// <p><p/></p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Filters the list of topic detection jobs based on job status. Returns only jobs with the specified status.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Only returns jobs submitted after the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Only returns jobs submitted before the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>Provides information about a topic detection job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TopicsDetectionJobProperties {
    /// <p>The time that the topic detection job was completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input data configuration supplied when you created the topic detection job.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The identifier assigned to the topic detection job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name of the topic detection job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current status of the topic detection job. If the status is <code>Failed</code>, the reason for the failure is shown in the <code>Message</code> field.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>A description for the status of a job.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The number of topics to detect supplied when you created the topic detection job. The default is 10. </p>
    #[serde(rename = "NumberOfTopics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_topics: Option<i64>,
    /// <p>The output data configuration supplied when you created the topic detection job.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>The time that the topic detection job was submitted for processing.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

/// Errors returned by BatchDetectDominantLanguage
#[derive(Debug, PartialEq)]
pub enum BatchDetectDominantLanguageError {
    /// <p>The number of documents in the request exceeds the limit of 25. Try your request again with fewer documents.</p>
    BatchSizeLimitExceeded(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
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

impl BatchDetectDominantLanguageError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchDetectDominantLanguageError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BatchSizeLimitExceededException" => {
                    return BatchDetectDominantLanguageError::BatchSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InternalServerException" => {
                    return BatchDetectDominantLanguageError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return BatchDetectDominantLanguageError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectDominantLanguageError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchDetectDominantLanguageError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchDetectDominantLanguageError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchDetectDominantLanguageError {
    fn from(err: serde_json::error::Error) -> BatchDetectDominantLanguageError {
        BatchDetectDominantLanguageError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDetectDominantLanguageError {
    fn from(err: CredentialsError) -> BatchDetectDominantLanguageError {
        BatchDetectDominantLanguageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDetectDominantLanguageError {
    fn from(err: HttpDispatchError) -> BatchDetectDominantLanguageError {
        BatchDetectDominantLanguageError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDetectDominantLanguageError {
    fn from(err: io::Error) -> BatchDetectDominantLanguageError {
        BatchDetectDominantLanguageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDetectDominantLanguageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDetectDominantLanguageError {
    fn description(&self) -> &str {
        match *self {
            BatchDetectDominantLanguageError::BatchSizeLimitExceeded(ref cause) => cause,
            BatchDetectDominantLanguageError::InternalServer(ref cause) => cause,
            BatchDetectDominantLanguageError::InvalidRequest(ref cause) => cause,
            BatchDetectDominantLanguageError::TextSizeLimitExceeded(ref cause) => cause,
            BatchDetectDominantLanguageError::Validation(ref cause) => cause,
            BatchDetectDominantLanguageError::Credentials(ref err) => err.description(),
            BatchDetectDominantLanguageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDetectDominantLanguageError::ParseError(ref cause) => cause,
            BatchDetectDominantLanguageError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchDetectEntities
#[derive(Debug, PartialEq)]
pub enum BatchDetectEntitiesError {
    /// <p>The number of documents in the request exceeds the limit of 25. Try your request again with fewer documents.</p>
    BatchSizeLimitExceeded(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all APIs except <code>DetectDominantLanguage</code>, Amazon Comprehend accepts only English or Spanish text. For the <code>DetectDominantLanguage</code> API, Amazon Comprehend detects 100 languages. For a list of languages, see <a>how-languages</a> </p>
    UnsupportedLanguage(String),
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

impl BatchDetectEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchDetectEntitiesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BatchSizeLimitExceededException" => {
                    return BatchDetectEntitiesError::BatchSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InternalServerException" => {
                    return BatchDetectEntitiesError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return BatchDetectEntitiesError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectEntitiesError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return BatchDetectEntitiesError::UnsupportedLanguage(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchDetectEntitiesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchDetectEntitiesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchDetectEntitiesError {
    fn from(err: serde_json::error::Error) -> BatchDetectEntitiesError {
        BatchDetectEntitiesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDetectEntitiesError {
    fn from(err: CredentialsError) -> BatchDetectEntitiesError {
        BatchDetectEntitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDetectEntitiesError {
    fn from(err: HttpDispatchError) -> BatchDetectEntitiesError {
        BatchDetectEntitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDetectEntitiesError {
    fn from(err: io::Error) -> BatchDetectEntitiesError {
        BatchDetectEntitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDetectEntitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDetectEntitiesError {
    fn description(&self) -> &str {
        match *self {
            BatchDetectEntitiesError::BatchSizeLimitExceeded(ref cause) => cause,
            BatchDetectEntitiesError::InternalServer(ref cause) => cause,
            BatchDetectEntitiesError::InvalidRequest(ref cause) => cause,
            BatchDetectEntitiesError::TextSizeLimitExceeded(ref cause) => cause,
            BatchDetectEntitiesError::UnsupportedLanguage(ref cause) => cause,
            BatchDetectEntitiesError::Validation(ref cause) => cause,
            BatchDetectEntitiesError::Credentials(ref err) => err.description(),
            BatchDetectEntitiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDetectEntitiesError::ParseError(ref cause) => cause,
            BatchDetectEntitiesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchDetectKeyPhrases
#[derive(Debug, PartialEq)]
pub enum BatchDetectKeyPhrasesError {
    /// <p>The number of documents in the request exceeds the limit of 25. Try your request again with fewer documents.</p>
    BatchSizeLimitExceeded(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all APIs except <code>DetectDominantLanguage</code>, Amazon Comprehend accepts only English or Spanish text. For the <code>DetectDominantLanguage</code> API, Amazon Comprehend detects 100 languages. For a list of languages, see <a>how-languages</a> </p>
    UnsupportedLanguage(String),
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

impl BatchDetectKeyPhrasesError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchDetectKeyPhrasesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BatchSizeLimitExceededException" => {
                    return BatchDetectKeyPhrasesError::BatchSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InternalServerException" => {
                    return BatchDetectKeyPhrasesError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return BatchDetectKeyPhrasesError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectKeyPhrasesError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return BatchDetectKeyPhrasesError::UnsupportedLanguage(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchDetectKeyPhrasesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchDetectKeyPhrasesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchDetectKeyPhrasesError {
    fn from(err: serde_json::error::Error) -> BatchDetectKeyPhrasesError {
        BatchDetectKeyPhrasesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDetectKeyPhrasesError {
    fn from(err: CredentialsError) -> BatchDetectKeyPhrasesError {
        BatchDetectKeyPhrasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDetectKeyPhrasesError {
    fn from(err: HttpDispatchError) -> BatchDetectKeyPhrasesError {
        BatchDetectKeyPhrasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDetectKeyPhrasesError {
    fn from(err: io::Error) -> BatchDetectKeyPhrasesError {
        BatchDetectKeyPhrasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDetectKeyPhrasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDetectKeyPhrasesError {
    fn description(&self) -> &str {
        match *self {
            BatchDetectKeyPhrasesError::BatchSizeLimitExceeded(ref cause) => cause,
            BatchDetectKeyPhrasesError::InternalServer(ref cause) => cause,
            BatchDetectKeyPhrasesError::InvalidRequest(ref cause) => cause,
            BatchDetectKeyPhrasesError::TextSizeLimitExceeded(ref cause) => cause,
            BatchDetectKeyPhrasesError::UnsupportedLanguage(ref cause) => cause,
            BatchDetectKeyPhrasesError::Validation(ref cause) => cause,
            BatchDetectKeyPhrasesError::Credentials(ref err) => err.description(),
            BatchDetectKeyPhrasesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDetectKeyPhrasesError::ParseError(ref cause) => cause,
            BatchDetectKeyPhrasesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchDetectSentiment
#[derive(Debug, PartialEq)]
pub enum BatchDetectSentimentError {
    /// <p>The number of documents in the request exceeds the limit of 25. Try your request again with fewer documents.</p>
    BatchSizeLimitExceeded(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all APIs except <code>DetectDominantLanguage</code>, Amazon Comprehend accepts only English or Spanish text. For the <code>DetectDominantLanguage</code> API, Amazon Comprehend detects 100 languages. For a list of languages, see <a>how-languages</a> </p>
    UnsupportedLanguage(String),
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

impl BatchDetectSentimentError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchDetectSentimentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BatchSizeLimitExceededException" => {
                    return BatchDetectSentimentError::BatchSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InternalServerException" => {
                    return BatchDetectSentimentError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return BatchDetectSentimentError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectSentimentError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return BatchDetectSentimentError::UnsupportedLanguage(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return BatchDetectSentimentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchDetectSentimentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchDetectSentimentError {
    fn from(err: serde_json::error::Error) -> BatchDetectSentimentError {
        BatchDetectSentimentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDetectSentimentError {
    fn from(err: CredentialsError) -> BatchDetectSentimentError {
        BatchDetectSentimentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDetectSentimentError {
    fn from(err: HttpDispatchError) -> BatchDetectSentimentError {
        BatchDetectSentimentError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDetectSentimentError {
    fn from(err: io::Error) -> BatchDetectSentimentError {
        BatchDetectSentimentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDetectSentimentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDetectSentimentError {
    fn description(&self) -> &str {
        match *self {
            BatchDetectSentimentError::BatchSizeLimitExceeded(ref cause) => cause,
            BatchDetectSentimentError::InternalServer(ref cause) => cause,
            BatchDetectSentimentError::InvalidRequest(ref cause) => cause,
            BatchDetectSentimentError::TextSizeLimitExceeded(ref cause) => cause,
            BatchDetectSentimentError::UnsupportedLanguage(ref cause) => cause,
            BatchDetectSentimentError::Validation(ref cause) => cause,
            BatchDetectSentimentError::Credentials(ref err) => err.description(),
            BatchDetectSentimentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDetectSentimentError::ParseError(ref cause) => cause,
            BatchDetectSentimentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by BatchDetectSyntax
#[derive(Debug, PartialEq)]
pub enum BatchDetectSyntaxError {
    /// <p>The number of documents in the request exceeds the limit of 25. Try your request again with fewer documents.</p>
    BatchSizeLimitExceeded(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all APIs except <code>DetectDominantLanguage</code>, Amazon Comprehend accepts only English or Spanish text. For the <code>DetectDominantLanguage</code> API, Amazon Comprehend detects 100 languages. For a list of languages, see <a>how-languages</a> </p>
    UnsupportedLanguage(String),
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

impl BatchDetectSyntaxError {
    pub fn from_response(res: BufferedHttpResponse) -> BatchDetectSyntaxError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "BatchSizeLimitExceededException" => {
                    return BatchDetectSyntaxError::BatchSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "InternalServerException" => {
                    return BatchDetectSyntaxError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return BatchDetectSyntaxError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectSyntaxError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return BatchDetectSyntaxError::UnsupportedLanguage(String::from(error_message))
                }
                "ValidationException" => {
                    return BatchDetectSyntaxError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BatchDetectSyntaxError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchDetectSyntaxError {
    fn from(err: serde_json::error::Error) -> BatchDetectSyntaxError {
        BatchDetectSyntaxError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchDetectSyntaxError {
    fn from(err: CredentialsError) -> BatchDetectSyntaxError {
        BatchDetectSyntaxError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchDetectSyntaxError {
    fn from(err: HttpDispatchError) -> BatchDetectSyntaxError {
        BatchDetectSyntaxError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchDetectSyntaxError {
    fn from(err: io::Error) -> BatchDetectSyntaxError {
        BatchDetectSyntaxError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BatchDetectSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchDetectSyntaxError {
    fn description(&self) -> &str {
        match *self {
            BatchDetectSyntaxError::BatchSizeLimitExceeded(ref cause) => cause,
            BatchDetectSyntaxError::InternalServer(ref cause) => cause,
            BatchDetectSyntaxError::InvalidRequest(ref cause) => cause,
            BatchDetectSyntaxError::TextSizeLimitExceeded(ref cause) => cause,
            BatchDetectSyntaxError::UnsupportedLanguage(ref cause) => cause,
            BatchDetectSyntaxError::Validation(ref cause) => cause,
            BatchDetectSyntaxError::Credentials(ref err) => err.description(),
            BatchDetectSyntaxError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            BatchDetectSyntaxError::ParseError(ref cause) => cause,
            BatchDetectSyntaxError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDominantLanguageDetectionJob
#[derive(Debug, PartialEq)]
pub enum DescribeDominantLanguageDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl DescribeDominantLanguageDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDominantLanguageDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DescribeDominantLanguageDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return DescribeDominantLanguageDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return DescribeDominantLanguageDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return DescribeDominantLanguageDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeDominantLanguageDetectionJobError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeDominantLanguageDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDominantLanguageDetectionJobError {
    fn from(err: serde_json::error::Error) -> DescribeDominantLanguageDetectionJobError {
        DescribeDominantLanguageDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDominantLanguageDetectionJobError {
    fn from(err: CredentialsError) -> DescribeDominantLanguageDetectionJobError {
        DescribeDominantLanguageDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDominantLanguageDetectionJobError {
    fn from(err: HttpDispatchError) -> DescribeDominantLanguageDetectionJobError {
        DescribeDominantLanguageDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDominantLanguageDetectionJobError {
    fn from(err: io::Error) -> DescribeDominantLanguageDetectionJobError {
        DescribeDominantLanguageDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDominantLanguageDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDominantLanguageDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeDominantLanguageDetectionJobError::InternalServer(ref cause) => cause,
            DescribeDominantLanguageDetectionJobError::InvalidRequest(ref cause) => cause,
            DescribeDominantLanguageDetectionJobError::JobNotFound(ref cause) => cause,
            DescribeDominantLanguageDetectionJobError::TooManyRequests(ref cause) => cause,
            DescribeDominantLanguageDetectionJobError::Validation(ref cause) => cause,
            DescribeDominantLanguageDetectionJobError::Credentials(ref err) => err.description(),
            DescribeDominantLanguageDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDominantLanguageDetectionJobError::ParseError(ref cause) => cause,
            DescribeDominantLanguageDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeEntitiesDetectionJob
#[derive(Debug, PartialEq)]
pub enum DescribeEntitiesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl DescribeEntitiesDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEntitiesDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DescribeEntitiesDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return DescribeEntitiesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return DescribeEntitiesDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return DescribeEntitiesDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeEntitiesDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeEntitiesDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeEntitiesDetectionJobError {
    fn from(err: serde_json::error::Error) -> DescribeEntitiesDetectionJobError {
        DescribeEntitiesDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEntitiesDetectionJobError {
    fn from(err: CredentialsError) -> DescribeEntitiesDetectionJobError {
        DescribeEntitiesDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEntitiesDetectionJobError {
    fn from(err: HttpDispatchError) -> DescribeEntitiesDetectionJobError {
        DescribeEntitiesDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEntitiesDetectionJobError {
    fn from(err: io::Error) -> DescribeEntitiesDetectionJobError {
        DescribeEntitiesDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEntitiesDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEntitiesDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeEntitiesDetectionJobError::InternalServer(ref cause) => cause,
            DescribeEntitiesDetectionJobError::InvalidRequest(ref cause) => cause,
            DescribeEntitiesDetectionJobError::JobNotFound(ref cause) => cause,
            DescribeEntitiesDetectionJobError::TooManyRequests(ref cause) => cause,
            DescribeEntitiesDetectionJobError::Validation(ref cause) => cause,
            DescribeEntitiesDetectionJobError::Credentials(ref err) => err.description(),
            DescribeEntitiesDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEntitiesDetectionJobError::ParseError(ref cause) => cause,
            DescribeEntitiesDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeKeyPhrasesDetectionJob
#[derive(Debug, PartialEq)]
pub enum DescribeKeyPhrasesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl DescribeKeyPhrasesDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeKeyPhrasesDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DescribeKeyPhrasesDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return DescribeKeyPhrasesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return DescribeKeyPhrasesDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return DescribeKeyPhrasesDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeKeyPhrasesDetectionJobError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeKeyPhrasesDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeKeyPhrasesDetectionJobError {
    fn from(err: serde_json::error::Error) -> DescribeKeyPhrasesDetectionJobError {
        DescribeKeyPhrasesDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeKeyPhrasesDetectionJobError {
    fn from(err: CredentialsError) -> DescribeKeyPhrasesDetectionJobError {
        DescribeKeyPhrasesDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeKeyPhrasesDetectionJobError {
    fn from(err: HttpDispatchError) -> DescribeKeyPhrasesDetectionJobError {
        DescribeKeyPhrasesDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeKeyPhrasesDetectionJobError {
    fn from(err: io::Error) -> DescribeKeyPhrasesDetectionJobError {
        DescribeKeyPhrasesDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeKeyPhrasesDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeKeyPhrasesDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeKeyPhrasesDetectionJobError::InternalServer(ref cause) => cause,
            DescribeKeyPhrasesDetectionJobError::InvalidRequest(ref cause) => cause,
            DescribeKeyPhrasesDetectionJobError::JobNotFound(ref cause) => cause,
            DescribeKeyPhrasesDetectionJobError::TooManyRequests(ref cause) => cause,
            DescribeKeyPhrasesDetectionJobError::Validation(ref cause) => cause,
            DescribeKeyPhrasesDetectionJobError::Credentials(ref err) => err.description(),
            DescribeKeyPhrasesDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeKeyPhrasesDetectionJobError::ParseError(ref cause) => cause,
            DescribeKeyPhrasesDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeSentimentDetectionJob
#[derive(Debug, PartialEq)]
pub enum DescribeSentimentDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl DescribeSentimentDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeSentimentDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DescribeSentimentDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return DescribeSentimentDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return DescribeSentimentDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return DescribeSentimentDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeSentimentDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeSentimentDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeSentimentDetectionJobError {
    fn from(err: serde_json::error::Error) -> DescribeSentimentDetectionJobError {
        DescribeSentimentDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSentimentDetectionJobError {
    fn from(err: CredentialsError) -> DescribeSentimentDetectionJobError {
        DescribeSentimentDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSentimentDetectionJobError {
    fn from(err: HttpDispatchError) -> DescribeSentimentDetectionJobError {
        DescribeSentimentDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSentimentDetectionJobError {
    fn from(err: io::Error) -> DescribeSentimentDetectionJobError {
        DescribeSentimentDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSentimentDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSentimentDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeSentimentDetectionJobError::InternalServer(ref cause) => cause,
            DescribeSentimentDetectionJobError::InvalidRequest(ref cause) => cause,
            DescribeSentimentDetectionJobError::JobNotFound(ref cause) => cause,
            DescribeSentimentDetectionJobError::TooManyRequests(ref cause) => cause,
            DescribeSentimentDetectionJobError::Validation(ref cause) => cause,
            DescribeSentimentDetectionJobError::Credentials(ref err) => err.description(),
            DescribeSentimentDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSentimentDetectionJobError::ParseError(ref cause) => cause,
            DescribeSentimentDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeTopicsDetectionJob
#[derive(Debug, PartialEq)]
pub enum DescribeTopicsDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl DescribeTopicsDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeTopicsDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DescribeTopicsDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return DescribeTopicsDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return DescribeTopicsDetectionJobError::JobNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DescribeTopicsDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeTopicsDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeTopicsDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeTopicsDetectionJobError {
    fn from(err: serde_json::error::Error) -> DescribeTopicsDetectionJobError {
        DescribeTopicsDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTopicsDetectionJobError {
    fn from(err: CredentialsError) -> DescribeTopicsDetectionJobError {
        DescribeTopicsDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTopicsDetectionJobError {
    fn from(err: HttpDispatchError) -> DescribeTopicsDetectionJobError {
        DescribeTopicsDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTopicsDetectionJobError {
    fn from(err: io::Error) -> DescribeTopicsDetectionJobError {
        DescribeTopicsDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTopicsDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTopicsDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeTopicsDetectionJobError::InternalServer(ref cause) => cause,
            DescribeTopicsDetectionJobError::InvalidRequest(ref cause) => cause,
            DescribeTopicsDetectionJobError::JobNotFound(ref cause) => cause,
            DescribeTopicsDetectionJobError::TooManyRequests(ref cause) => cause,
            DescribeTopicsDetectionJobError::Validation(ref cause) => cause,
            DescribeTopicsDetectionJobError::Credentials(ref err) => err.description(),
            DescribeTopicsDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTopicsDetectionJobError::ParseError(ref cause) => cause,
            DescribeTopicsDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectDominantLanguage
#[derive(Debug, PartialEq)]
pub enum DetectDominantLanguageError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
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

impl DetectDominantLanguageError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectDominantLanguageError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DetectDominantLanguageError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return DetectDominantLanguageError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return DetectDominantLanguageError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DetectDominantLanguageError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectDominantLanguageError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectDominantLanguageError {
    fn from(err: serde_json::error::Error) -> DetectDominantLanguageError {
        DetectDominantLanguageError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectDominantLanguageError {
    fn from(err: CredentialsError) -> DetectDominantLanguageError {
        DetectDominantLanguageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectDominantLanguageError {
    fn from(err: HttpDispatchError) -> DetectDominantLanguageError {
        DetectDominantLanguageError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectDominantLanguageError {
    fn from(err: io::Error) -> DetectDominantLanguageError {
        DetectDominantLanguageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectDominantLanguageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectDominantLanguageError {
    fn description(&self) -> &str {
        match *self {
            DetectDominantLanguageError::InternalServer(ref cause) => cause,
            DetectDominantLanguageError::InvalidRequest(ref cause) => cause,
            DetectDominantLanguageError::TextSizeLimitExceeded(ref cause) => cause,
            DetectDominantLanguageError::Validation(ref cause) => cause,
            DetectDominantLanguageError::Credentials(ref err) => err.description(),
            DetectDominantLanguageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetectDominantLanguageError::ParseError(ref cause) => cause,
            DetectDominantLanguageError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectEntities
#[derive(Debug, PartialEq)]
pub enum DetectEntitiesError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all APIs except <code>DetectDominantLanguage</code>, Amazon Comprehend accepts only English or Spanish text. For the <code>DetectDominantLanguage</code> API, Amazon Comprehend detects 100 languages. For a list of languages, see <a>how-languages</a> </p>
    UnsupportedLanguage(String),
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

impl DetectEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectEntitiesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DetectEntitiesError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return DetectEntitiesError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return DetectEntitiesError::TextSizeLimitExceeded(String::from(error_message))
                }
                "UnsupportedLanguageException" => {
                    return DetectEntitiesError::UnsupportedLanguage(String::from(error_message))
                }
                "ValidationException" => {
                    return DetectEntitiesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectEntitiesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectEntitiesError {
    fn from(err: serde_json::error::Error) -> DetectEntitiesError {
        DetectEntitiesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectEntitiesError {
    fn from(err: CredentialsError) -> DetectEntitiesError {
        DetectEntitiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectEntitiesError {
    fn from(err: HttpDispatchError) -> DetectEntitiesError {
        DetectEntitiesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectEntitiesError {
    fn from(err: io::Error) -> DetectEntitiesError {
        DetectEntitiesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectEntitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectEntitiesError {
    fn description(&self) -> &str {
        match *self {
            DetectEntitiesError::InternalServer(ref cause) => cause,
            DetectEntitiesError::InvalidRequest(ref cause) => cause,
            DetectEntitiesError::TextSizeLimitExceeded(ref cause) => cause,
            DetectEntitiesError::UnsupportedLanguage(ref cause) => cause,
            DetectEntitiesError::Validation(ref cause) => cause,
            DetectEntitiesError::Credentials(ref err) => err.description(),
            DetectEntitiesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectEntitiesError::ParseError(ref cause) => cause,
            DetectEntitiesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectKeyPhrases
#[derive(Debug, PartialEq)]
pub enum DetectKeyPhrasesError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all APIs except <code>DetectDominantLanguage</code>, Amazon Comprehend accepts only English or Spanish text. For the <code>DetectDominantLanguage</code> API, Amazon Comprehend detects 100 languages. For a list of languages, see <a>how-languages</a> </p>
    UnsupportedLanguage(String),
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

impl DetectKeyPhrasesError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectKeyPhrasesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DetectKeyPhrasesError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return DetectKeyPhrasesError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return DetectKeyPhrasesError::TextSizeLimitExceeded(String::from(error_message))
                }
                "UnsupportedLanguageException" => {
                    return DetectKeyPhrasesError::UnsupportedLanguage(String::from(error_message))
                }
                "ValidationException" => {
                    return DetectKeyPhrasesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectKeyPhrasesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectKeyPhrasesError {
    fn from(err: serde_json::error::Error) -> DetectKeyPhrasesError {
        DetectKeyPhrasesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectKeyPhrasesError {
    fn from(err: CredentialsError) -> DetectKeyPhrasesError {
        DetectKeyPhrasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectKeyPhrasesError {
    fn from(err: HttpDispatchError) -> DetectKeyPhrasesError {
        DetectKeyPhrasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectKeyPhrasesError {
    fn from(err: io::Error) -> DetectKeyPhrasesError {
        DetectKeyPhrasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectKeyPhrasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectKeyPhrasesError {
    fn description(&self) -> &str {
        match *self {
            DetectKeyPhrasesError::InternalServer(ref cause) => cause,
            DetectKeyPhrasesError::InvalidRequest(ref cause) => cause,
            DetectKeyPhrasesError::TextSizeLimitExceeded(ref cause) => cause,
            DetectKeyPhrasesError::UnsupportedLanguage(ref cause) => cause,
            DetectKeyPhrasesError::Validation(ref cause) => cause,
            DetectKeyPhrasesError::Credentials(ref err) => err.description(),
            DetectKeyPhrasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectKeyPhrasesError::ParseError(ref cause) => cause,
            DetectKeyPhrasesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectSentiment
#[derive(Debug, PartialEq)]
pub enum DetectSentimentError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all APIs except <code>DetectDominantLanguage</code>, Amazon Comprehend accepts only English or Spanish text. For the <code>DetectDominantLanguage</code> API, Amazon Comprehend detects 100 languages. For a list of languages, see <a>how-languages</a> </p>
    UnsupportedLanguage(String),
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

impl DetectSentimentError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectSentimentError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DetectSentimentError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return DetectSentimentError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return DetectSentimentError::TextSizeLimitExceeded(String::from(error_message))
                }
                "UnsupportedLanguageException" => {
                    return DetectSentimentError::UnsupportedLanguage(String::from(error_message))
                }
                "ValidationException" => {
                    return DetectSentimentError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectSentimentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectSentimentError {
    fn from(err: serde_json::error::Error) -> DetectSentimentError {
        DetectSentimentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectSentimentError {
    fn from(err: CredentialsError) -> DetectSentimentError {
        DetectSentimentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectSentimentError {
    fn from(err: HttpDispatchError) -> DetectSentimentError {
        DetectSentimentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectSentimentError {
    fn from(err: io::Error) -> DetectSentimentError {
        DetectSentimentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectSentimentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectSentimentError {
    fn description(&self) -> &str {
        match *self {
            DetectSentimentError::InternalServer(ref cause) => cause,
            DetectSentimentError::InvalidRequest(ref cause) => cause,
            DetectSentimentError::TextSizeLimitExceeded(ref cause) => cause,
            DetectSentimentError::UnsupportedLanguage(ref cause) => cause,
            DetectSentimentError::Validation(ref cause) => cause,
            DetectSentimentError::Credentials(ref err) => err.description(),
            DetectSentimentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectSentimentError::ParseError(ref cause) => cause,
            DetectSentimentError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DetectSyntax
#[derive(Debug, PartialEq)]
pub enum DetectSyntaxError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all APIs except <code>DetectDominantLanguage</code>, Amazon Comprehend accepts only English or Spanish text. For the <code>DetectDominantLanguage</code> API, Amazon Comprehend detects 100 languages. For a list of languages, see <a>how-languages</a> </p>
    UnsupportedLanguage(String),
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

impl DetectSyntaxError {
    pub fn from_response(res: BufferedHttpResponse) -> DetectSyntaxError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return DetectSyntaxError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return DetectSyntaxError::InvalidRequest(String::from(error_message))
                }
                "TextSizeLimitExceededException" => {
                    return DetectSyntaxError::TextSizeLimitExceeded(String::from(error_message))
                }
                "UnsupportedLanguageException" => {
                    return DetectSyntaxError::UnsupportedLanguage(String::from(error_message))
                }
                "ValidationException" => {
                    return DetectSyntaxError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DetectSyntaxError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DetectSyntaxError {
    fn from(err: serde_json::error::Error) -> DetectSyntaxError {
        DetectSyntaxError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectSyntaxError {
    fn from(err: CredentialsError) -> DetectSyntaxError {
        DetectSyntaxError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectSyntaxError {
    fn from(err: HttpDispatchError) -> DetectSyntaxError {
        DetectSyntaxError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetectSyntaxError {
    fn from(err: io::Error) -> DetectSyntaxError {
        DetectSyntaxError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetectSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectSyntaxError {
    fn description(&self) -> &str {
        match *self {
            DetectSyntaxError::InternalServer(ref cause) => cause,
            DetectSyntaxError::InvalidRequest(ref cause) => cause,
            DetectSyntaxError::TextSizeLimitExceeded(ref cause) => cause,
            DetectSyntaxError::UnsupportedLanguage(ref cause) => cause,
            DetectSyntaxError::Validation(ref cause) => cause,
            DetectSyntaxError::Credentials(ref err) => err.description(),
            DetectSyntaxError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectSyntaxError::ParseError(ref cause) => cause,
            DetectSyntaxError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDominantLanguageDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListDominantLanguageDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListTopicDetectionJobs</code> operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl ListDominantLanguageDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDominantLanguageDetectionJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return ListDominantLanguageDetectionJobsError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidFilterException" => {
                    return ListDominantLanguageDetectionJobsError::InvalidFilter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return ListDominantLanguageDetectionJobsError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return ListDominantLanguageDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListDominantLanguageDetectionJobsError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return ListDominantLanguageDetectionJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDominantLanguageDetectionJobsError {
    fn from(err: serde_json::error::Error) -> ListDominantLanguageDetectionJobsError {
        ListDominantLanguageDetectionJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDominantLanguageDetectionJobsError {
    fn from(err: CredentialsError) -> ListDominantLanguageDetectionJobsError {
        ListDominantLanguageDetectionJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDominantLanguageDetectionJobsError {
    fn from(err: HttpDispatchError) -> ListDominantLanguageDetectionJobsError {
        ListDominantLanguageDetectionJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDominantLanguageDetectionJobsError {
    fn from(err: io::Error) -> ListDominantLanguageDetectionJobsError {
        ListDominantLanguageDetectionJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDominantLanguageDetectionJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDominantLanguageDetectionJobsError {
    fn description(&self) -> &str {
        match *self {
            ListDominantLanguageDetectionJobsError::InternalServer(ref cause) => cause,
            ListDominantLanguageDetectionJobsError::InvalidFilter(ref cause) => cause,
            ListDominantLanguageDetectionJobsError::InvalidRequest(ref cause) => cause,
            ListDominantLanguageDetectionJobsError::TooManyRequests(ref cause) => cause,
            ListDominantLanguageDetectionJobsError::Validation(ref cause) => cause,
            ListDominantLanguageDetectionJobsError::Credentials(ref err) => err.description(),
            ListDominantLanguageDetectionJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDominantLanguageDetectionJobsError::ParseError(ref cause) => cause,
            ListDominantLanguageDetectionJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListEntitiesDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListEntitiesDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListTopicDetectionJobs</code> operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl ListEntitiesDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListEntitiesDetectionJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return ListEntitiesDetectionJobsError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidFilterException" => {
                    return ListEntitiesDetectionJobsError::InvalidFilter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return ListEntitiesDetectionJobsError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return ListEntitiesDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListEntitiesDetectionJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListEntitiesDetectionJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListEntitiesDetectionJobsError {
    fn from(err: serde_json::error::Error) -> ListEntitiesDetectionJobsError {
        ListEntitiesDetectionJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListEntitiesDetectionJobsError {
    fn from(err: CredentialsError) -> ListEntitiesDetectionJobsError {
        ListEntitiesDetectionJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListEntitiesDetectionJobsError {
    fn from(err: HttpDispatchError) -> ListEntitiesDetectionJobsError {
        ListEntitiesDetectionJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListEntitiesDetectionJobsError {
    fn from(err: io::Error) -> ListEntitiesDetectionJobsError {
        ListEntitiesDetectionJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListEntitiesDetectionJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEntitiesDetectionJobsError {
    fn description(&self) -> &str {
        match *self {
            ListEntitiesDetectionJobsError::InternalServer(ref cause) => cause,
            ListEntitiesDetectionJobsError::InvalidFilter(ref cause) => cause,
            ListEntitiesDetectionJobsError::InvalidRequest(ref cause) => cause,
            ListEntitiesDetectionJobsError::TooManyRequests(ref cause) => cause,
            ListEntitiesDetectionJobsError::Validation(ref cause) => cause,
            ListEntitiesDetectionJobsError::Credentials(ref err) => err.description(),
            ListEntitiesDetectionJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListEntitiesDetectionJobsError::ParseError(ref cause) => cause,
            ListEntitiesDetectionJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListKeyPhrasesDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListKeyPhrasesDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListTopicDetectionJobs</code> operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl ListKeyPhrasesDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListKeyPhrasesDetectionJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return ListKeyPhrasesDetectionJobsError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidFilterException" => {
                    return ListKeyPhrasesDetectionJobsError::InvalidFilter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return ListKeyPhrasesDetectionJobsError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return ListKeyPhrasesDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListKeyPhrasesDetectionJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListKeyPhrasesDetectionJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListKeyPhrasesDetectionJobsError {
    fn from(err: serde_json::error::Error) -> ListKeyPhrasesDetectionJobsError {
        ListKeyPhrasesDetectionJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListKeyPhrasesDetectionJobsError {
    fn from(err: CredentialsError) -> ListKeyPhrasesDetectionJobsError {
        ListKeyPhrasesDetectionJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListKeyPhrasesDetectionJobsError {
    fn from(err: HttpDispatchError) -> ListKeyPhrasesDetectionJobsError {
        ListKeyPhrasesDetectionJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListKeyPhrasesDetectionJobsError {
    fn from(err: io::Error) -> ListKeyPhrasesDetectionJobsError {
        ListKeyPhrasesDetectionJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListKeyPhrasesDetectionJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListKeyPhrasesDetectionJobsError {
    fn description(&self) -> &str {
        match *self {
            ListKeyPhrasesDetectionJobsError::InternalServer(ref cause) => cause,
            ListKeyPhrasesDetectionJobsError::InvalidFilter(ref cause) => cause,
            ListKeyPhrasesDetectionJobsError::InvalidRequest(ref cause) => cause,
            ListKeyPhrasesDetectionJobsError::TooManyRequests(ref cause) => cause,
            ListKeyPhrasesDetectionJobsError::Validation(ref cause) => cause,
            ListKeyPhrasesDetectionJobsError::Credentials(ref err) => err.description(),
            ListKeyPhrasesDetectionJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListKeyPhrasesDetectionJobsError::ParseError(ref cause) => cause,
            ListKeyPhrasesDetectionJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListSentimentDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListSentimentDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListTopicDetectionJobs</code> operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl ListSentimentDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListSentimentDetectionJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return ListSentimentDetectionJobsError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidFilterException" => {
                    return ListSentimentDetectionJobsError::InvalidFilter(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return ListSentimentDetectionJobsError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return ListSentimentDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListSentimentDetectionJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListSentimentDetectionJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListSentimentDetectionJobsError {
    fn from(err: serde_json::error::Error) -> ListSentimentDetectionJobsError {
        ListSentimentDetectionJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSentimentDetectionJobsError {
    fn from(err: CredentialsError) -> ListSentimentDetectionJobsError {
        ListSentimentDetectionJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSentimentDetectionJobsError {
    fn from(err: HttpDispatchError) -> ListSentimentDetectionJobsError {
        ListSentimentDetectionJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSentimentDetectionJobsError {
    fn from(err: io::Error) -> ListSentimentDetectionJobsError {
        ListSentimentDetectionJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSentimentDetectionJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSentimentDetectionJobsError {
    fn description(&self) -> &str {
        match *self {
            ListSentimentDetectionJobsError::InternalServer(ref cause) => cause,
            ListSentimentDetectionJobsError::InvalidFilter(ref cause) => cause,
            ListSentimentDetectionJobsError::InvalidRequest(ref cause) => cause,
            ListSentimentDetectionJobsError::TooManyRequests(ref cause) => cause,
            ListSentimentDetectionJobsError::Validation(ref cause) => cause,
            ListSentimentDetectionJobsError::Credentials(ref err) => err.description(),
            ListSentimentDetectionJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSentimentDetectionJobsError::ParseError(ref cause) => cause,
            ListSentimentDetectionJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListTopicsDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListTopicsDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListTopicDetectionJobs</code> operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl ListTopicsDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListTopicsDetectionJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return ListTopicsDetectionJobsError::InternalServer(String::from(error_message))
                }
                "InvalidFilterException" => {
                    return ListTopicsDetectionJobsError::InvalidFilter(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return ListTopicsDetectionJobsError::InvalidRequest(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return ListTopicsDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ListTopicsDetectionJobsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListTopicsDetectionJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTopicsDetectionJobsError {
    fn from(err: serde_json::error::Error) -> ListTopicsDetectionJobsError {
        ListTopicsDetectionJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTopicsDetectionJobsError {
    fn from(err: CredentialsError) -> ListTopicsDetectionJobsError {
        ListTopicsDetectionJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTopicsDetectionJobsError {
    fn from(err: HttpDispatchError) -> ListTopicsDetectionJobsError {
        ListTopicsDetectionJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTopicsDetectionJobsError {
    fn from(err: io::Error) -> ListTopicsDetectionJobsError {
        ListTopicsDetectionJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTopicsDetectionJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTopicsDetectionJobsError {
    fn description(&self) -> &str {
        match *self {
            ListTopicsDetectionJobsError::InternalServer(ref cause) => cause,
            ListTopicsDetectionJobsError::InvalidFilter(ref cause) => cause,
            ListTopicsDetectionJobsError::InvalidRequest(ref cause) => cause,
            ListTopicsDetectionJobsError::TooManyRequests(ref cause) => cause,
            ListTopicsDetectionJobsError::Validation(ref cause) => cause,
            ListTopicsDetectionJobsError::Credentials(ref err) => err.description(),
            ListTopicsDetectionJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTopicsDetectionJobsError::ParseError(ref cause) => cause,
            ListTopicsDetectionJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartDominantLanguageDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartDominantLanguageDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl StartDominantLanguageDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StartDominantLanguageDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StartDominantLanguageDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StartDominantLanguageDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return StartDominantLanguageDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartDominantLanguageDetectionJobError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return StartDominantLanguageDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartDominantLanguageDetectionJobError {
    fn from(err: serde_json::error::Error) -> StartDominantLanguageDetectionJobError {
        StartDominantLanguageDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartDominantLanguageDetectionJobError {
    fn from(err: CredentialsError) -> StartDominantLanguageDetectionJobError {
        StartDominantLanguageDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartDominantLanguageDetectionJobError {
    fn from(err: HttpDispatchError) -> StartDominantLanguageDetectionJobError {
        StartDominantLanguageDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartDominantLanguageDetectionJobError {
    fn from(err: io::Error) -> StartDominantLanguageDetectionJobError {
        StartDominantLanguageDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartDominantLanguageDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartDominantLanguageDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StartDominantLanguageDetectionJobError::InternalServer(ref cause) => cause,
            StartDominantLanguageDetectionJobError::InvalidRequest(ref cause) => cause,
            StartDominantLanguageDetectionJobError::TooManyRequests(ref cause) => cause,
            StartDominantLanguageDetectionJobError::Validation(ref cause) => cause,
            StartDominantLanguageDetectionJobError::Credentials(ref err) => err.description(),
            StartDominantLanguageDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartDominantLanguageDetectionJobError::ParseError(ref cause) => cause,
            StartDominantLanguageDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartEntitiesDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartEntitiesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl StartEntitiesDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StartEntitiesDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StartEntitiesDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StartEntitiesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return StartEntitiesDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartEntitiesDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartEntitiesDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartEntitiesDetectionJobError {
    fn from(err: serde_json::error::Error) -> StartEntitiesDetectionJobError {
        StartEntitiesDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartEntitiesDetectionJobError {
    fn from(err: CredentialsError) -> StartEntitiesDetectionJobError {
        StartEntitiesDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartEntitiesDetectionJobError {
    fn from(err: HttpDispatchError) -> StartEntitiesDetectionJobError {
        StartEntitiesDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartEntitiesDetectionJobError {
    fn from(err: io::Error) -> StartEntitiesDetectionJobError {
        StartEntitiesDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartEntitiesDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartEntitiesDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StartEntitiesDetectionJobError::InternalServer(ref cause) => cause,
            StartEntitiesDetectionJobError::InvalidRequest(ref cause) => cause,
            StartEntitiesDetectionJobError::TooManyRequests(ref cause) => cause,
            StartEntitiesDetectionJobError::Validation(ref cause) => cause,
            StartEntitiesDetectionJobError::Credentials(ref err) => err.description(),
            StartEntitiesDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartEntitiesDetectionJobError::ParseError(ref cause) => cause,
            StartEntitiesDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartKeyPhrasesDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartKeyPhrasesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl StartKeyPhrasesDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StartKeyPhrasesDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StartKeyPhrasesDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StartKeyPhrasesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return StartKeyPhrasesDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartKeyPhrasesDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartKeyPhrasesDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartKeyPhrasesDetectionJobError {
    fn from(err: serde_json::error::Error) -> StartKeyPhrasesDetectionJobError {
        StartKeyPhrasesDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartKeyPhrasesDetectionJobError {
    fn from(err: CredentialsError) -> StartKeyPhrasesDetectionJobError {
        StartKeyPhrasesDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartKeyPhrasesDetectionJobError {
    fn from(err: HttpDispatchError) -> StartKeyPhrasesDetectionJobError {
        StartKeyPhrasesDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartKeyPhrasesDetectionJobError {
    fn from(err: io::Error) -> StartKeyPhrasesDetectionJobError {
        StartKeyPhrasesDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartKeyPhrasesDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartKeyPhrasesDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StartKeyPhrasesDetectionJobError::InternalServer(ref cause) => cause,
            StartKeyPhrasesDetectionJobError::InvalidRequest(ref cause) => cause,
            StartKeyPhrasesDetectionJobError::TooManyRequests(ref cause) => cause,
            StartKeyPhrasesDetectionJobError::Validation(ref cause) => cause,
            StartKeyPhrasesDetectionJobError::Credentials(ref err) => err.description(),
            StartKeyPhrasesDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartKeyPhrasesDetectionJobError::ParseError(ref cause) => cause,
            StartKeyPhrasesDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartSentimentDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartSentimentDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl StartSentimentDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StartSentimentDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StartSentimentDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StartSentimentDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return StartSentimentDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartSentimentDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartSentimentDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartSentimentDetectionJobError {
    fn from(err: serde_json::error::Error) -> StartSentimentDetectionJobError {
        StartSentimentDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartSentimentDetectionJobError {
    fn from(err: CredentialsError) -> StartSentimentDetectionJobError {
        StartSentimentDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartSentimentDetectionJobError {
    fn from(err: HttpDispatchError) -> StartSentimentDetectionJobError {
        StartSentimentDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartSentimentDetectionJobError {
    fn from(err: io::Error) -> StartSentimentDetectionJobError {
        StartSentimentDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartSentimentDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartSentimentDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StartSentimentDetectionJobError::InternalServer(ref cause) => cause,
            StartSentimentDetectionJobError::InvalidRequest(ref cause) => cause,
            StartSentimentDetectionJobError::TooManyRequests(ref cause) => cause,
            StartSentimentDetectionJobError::Validation(ref cause) => cause,
            StartSentimentDetectionJobError::Credentials(ref err) => err.description(),
            StartSentimentDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartSentimentDetectionJobError::ParseError(ref cause) => cause,
            StartSentimentDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartTopicsDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartTopicsDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
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

impl StartTopicsDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StartTopicsDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StartTopicsDetectionJobError::InternalServer(String::from(error_message))
                }
                "InvalidRequestException" => {
                    return StartTopicsDetectionJobError::InvalidRequest(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return StartTopicsDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StartTopicsDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StartTopicsDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartTopicsDetectionJobError {
    fn from(err: serde_json::error::Error) -> StartTopicsDetectionJobError {
        StartTopicsDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartTopicsDetectionJobError {
    fn from(err: CredentialsError) -> StartTopicsDetectionJobError {
        StartTopicsDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartTopicsDetectionJobError {
    fn from(err: HttpDispatchError) -> StartTopicsDetectionJobError {
        StartTopicsDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartTopicsDetectionJobError {
    fn from(err: io::Error) -> StartTopicsDetectionJobError {
        StartTopicsDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartTopicsDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartTopicsDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StartTopicsDetectionJobError::InternalServer(ref cause) => cause,
            StartTopicsDetectionJobError::InvalidRequest(ref cause) => cause,
            StartTopicsDetectionJobError::TooManyRequests(ref cause) => cause,
            StartTopicsDetectionJobError::Validation(ref cause) => cause,
            StartTopicsDetectionJobError::Credentials(ref err) => err.description(),
            StartTopicsDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartTopicsDetectionJobError::ParseError(ref cause) => cause,
            StartTopicsDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopDominantLanguageDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopDominantLanguageDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
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

impl StopDominantLanguageDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StopDominantLanguageDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StopDominantLanguageDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StopDominantLanguageDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return StopDominantLanguageDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return StopDominantLanguageDetectionJobError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return StopDominantLanguageDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopDominantLanguageDetectionJobError {
    fn from(err: serde_json::error::Error) -> StopDominantLanguageDetectionJobError {
        StopDominantLanguageDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopDominantLanguageDetectionJobError {
    fn from(err: CredentialsError) -> StopDominantLanguageDetectionJobError {
        StopDominantLanguageDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopDominantLanguageDetectionJobError {
    fn from(err: HttpDispatchError) -> StopDominantLanguageDetectionJobError {
        StopDominantLanguageDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopDominantLanguageDetectionJobError {
    fn from(err: io::Error) -> StopDominantLanguageDetectionJobError {
        StopDominantLanguageDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopDominantLanguageDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopDominantLanguageDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StopDominantLanguageDetectionJobError::InternalServer(ref cause) => cause,
            StopDominantLanguageDetectionJobError::InvalidRequest(ref cause) => cause,
            StopDominantLanguageDetectionJobError::JobNotFound(ref cause) => cause,
            StopDominantLanguageDetectionJobError::Validation(ref cause) => cause,
            StopDominantLanguageDetectionJobError::Credentials(ref err) => err.description(),
            StopDominantLanguageDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopDominantLanguageDetectionJobError::ParseError(ref cause) => cause,
            StopDominantLanguageDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopEntitiesDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopEntitiesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
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

impl StopEntitiesDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StopEntitiesDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StopEntitiesDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StopEntitiesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return StopEntitiesDetectionJobError::JobNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return StopEntitiesDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopEntitiesDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopEntitiesDetectionJobError {
    fn from(err: serde_json::error::Error) -> StopEntitiesDetectionJobError {
        StopEntitiesDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopEntitiesDetectionJobError {
    fn from(err: CredentialsError) -> StopEntitiesDetectionJobError {
        StopEntitiesDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopEntitiesDetectionJobError {
    fn from(err: HttpDispatchError) -> StopEntitiesDetectionJobError {
        StopEntitiesDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopEntitiesDetectionJobError {
    fn from(err: io::Error) -> StopEntitiesDetectionJobError {
        StopEntitiesDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopEntitiesDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopEntitiesDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StopEntitiesDetectionJobError::InternalServer(ref cause) => cause,
            StopEntitiesDetectionJobError::InvalidRequest(ref cause) => cause,
            StopEntitiesDetectionJobError::JobNotFound(ref cause) => cause,
            StopEntitiesDetectionJobError::Validation(ref cause) => cause,
            StopEntitiesDetectionJobError::Credentials(ref err) => err.description(),
            StopEntitiesDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopEntitiesDetectionJobError::ParseError(ref cause) => cause,
            StopEntitiesDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopKeyPhrasesDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopKeyPhrasesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
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

impl StopKeyPhrasesDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StopKeyPhrasesDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StopKeyPhrasesDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StopKeyPhrasesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return StopKeyPhrasesDetectionJobError::JobNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return StopKeyPhrasesDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopKeyPhrasesDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopKeyPhrasesDetectionJobError {
    fn from(err: serde_json::error::Error) -> StopKeyPhrasesDetectionJobError {
        StopKeyPhrasesDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopKeyPhrasesDetectionJobError {
    fn from(err: CredentialsError) -> StopKeyPhrasesDetectionJobError {
        StopKeyPhrasesDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopKeyPhrasesDetectionJobError {
    fn from(err: HttpDispatchError) -> StopKeyPhrasesDetectionJobError {
        StopKeyPhrasesDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopKeyPhrasesDetectionJobError {
    fn from(err: io::Error) -> StopKeyPhrasesDetectionJobError {
        StopKeyPhrasesDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopKeyPhrasesDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopKeyPhrasesDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StopKeyPhrasesDetectionJobError::InternalServer(ref cause) => cause,
            StopKeyPhrasesDetectionJobError::InvalidRequest(ref cause) => cause,
            StopKeyPhrasesDetectionJobError::JobNotFound(ref cause) => cause,
            StopKeyPhrasesDetectionJobError::Validation(ref cause) => cause,
            StopKeyPhrasesDetectionJobError::Credentials(ref err) => err.description(),
            StopKeyPhrasesDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopKeyPhrasesDetectionJobError::ParseError(ref cause) => cause,
            StopKeyPhrasesDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopSentimentDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopSentimentDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
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

impl StopSentimentDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StopSentimentDetectionJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalServerException" => {
                    return StopSentimentDetectionJobError::InternalServer(String::from(
                        error_message,
                    ))
                }
                "InvalidRequestException" => {
                    return StopSentimentDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ))
                }
                "JobNotFoundException" => {
                    return StopSentimentDetectionJobError::JobNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return StopSentimentDetectionJobError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return StopSentimentDetectionJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopSentimentDetectionJobError {
    fn from(err: serde_json::error::Error) -> StopSentimentDetectionJobError {
        StopSentimentDetectionJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopSentimentDetectionJobError {
    fn from(err: CredentialsError) -> StopSentimentDetectionJobError {
        StopSentimentDetectionJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopSentimentDetectionJobError {
    fn from(err: HttpDispatchError) -> StopSentimentDetectionJobError {
        StopSentimentDetectionJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopSentimentDetectionJobError {
    fn from(err: io::Error) -> StopSentimentDetectionJobError {
        StopSentimentDetectionJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopSentimentDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopSentimentDetectionJobError {
    fn description(&self) -> &str {
        match *self {
            StopSentimentDetectionJobError::InternalServer(ref cause) => cause,
            StopSentimentDetectionJobError::InvalidRequest(ref cause) => cause,
            StopSentimentDetectionJobError::JobNotFound(ref cause) => cause,
            StopSentimentDetectionJobError::Validation(ref cause) => cause,
            StopSentimentDetectionJobError::Credentials(ref err) => err.description(),
            StopSentimentDetectionJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopSentimentDetectionJobError::ParseError(ref cause) => cause,
            StopSentimentDetectionJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Comprehend API. Amazon Comprehend clients implement this trait.
pub trait Comprehend {
    /// <p>Determines the dominant language of the input text for a batch of documents. For a list of languages that Amazon Comprehend can detect, see <a href="http://docs.aws.amazon.com/comprehend/latest/dg/how-languages.html">Amazon Comprehend Supported Languages</a>. </p>
    fn batch_detect_dominant_language(
        &self,
        input: BatchDetectDominantLanguageRequest,
    ) -> RusotoFuture<BatchDetectDominantLanguageResponse, BatchDetectDominantLanguageError>;

    /// <p>Inspects the text of a batch of documents for named entities and returns information about them. For more information about named entities, see <a>how-entities</a> </p>
    fn batch_detect_entities(
        &self,
        input: BatchDetectEntitiesRequest,
    ) -> RusotoFuture<BatchDetectEntitiesResponse, BatchDetectEntitiesError>;

    /// <p>Detects the key noun phrases found in a batch of documents.</p>
    fn batch_detect_key_phrases(
        &self,
        input: BatchDetectKeyPhrasesRequest,
    ) -> RusotoFuture<BatchDetectKeyPhrasesResponse, BatchDetectKeyPhrasesError>;

    /// <p>Inspects a batch of documents and returns an inference of the prevailing sentiment, <code>POSITIVE</code>, <code>NEUTRAL</code>, <code>MIXED</code>, or <code>NEGATIVE</code>, in each one.</p>
    fn batch_detect_sentiment(
        &self,
        input: BatchDetectSentimentRequest,
    ) -> RusotoFuture<BatchDetectSentimentResponse, BatchDetectSentimentError>;

    /// <p>Inspects the text of a batch of documents for the syntax and part of speech of the words in the document and returns information about them. For more information, see <a>how-syntax</a>.</p>
    fn batch_detect_syntax(
        &self,
        input: BatchDetectSyntaxRequest,
    ) -> RusotoFuture<BatchDetectSyntaxResponse, BatchDetectSyntaxError>;

    /// <p>Gets the properties associated with a dominant language detection job. Use this operation to get the status of a detection job.</p>
    fn describe_dominant_language_detection_job(
        &self,
        input: DescribeDominantLanguageDetectionJobRequest,
    ) -> RusotoFuture<
        DescribeDominantLanguageDetectionJobResponse,
        DescribeDominantLanguageDetectionJobError,
    >;

    /// <p>Gets the properties associated with an entities detection job. Use this operation to get the status of a detection job.</p>
    fn describe_entities_detection_job(
        &self,
        input: DescribeEntitiesDetectionJobRequest,
    ) -> RusotoFuture<DescribeEntitiesDetectionJobResponse, DescribeEntitiesDetectionJobError>;

    /// <p>Gets the properties associated with a key phrases detection job. Use this operation to get the status of a detection job.</p>
    fn describe_key_phrases_detection_job(
        &self,
        input: DescribeKeyPhrasesDetectionJobRequest,
    ) -> RusotoFuture<DescribeKeyPhrasesDetectionJobResponse, DescribeKeyPhrasesDetectionJobError>;

    /// <p>Gets the properties associated with a sentiment detection job. Use this operation to get the status of a detection job.</p>
    fn describe_sentiment_detection_job(
        &self,
        input: DescribeSentimentDetectionJobRequest,
    ) -> RusotoFuture<DescribeSentimentDetectionJobResponse, DescribeSentimentDetectionJobError>;

    /// <p>Gets the properties associated with a topic detection job. Use this operation to get the status of a detection job.</p>
    fn describe_topics_detection_job(
        &self,
        input: DescribeTopicsDetectionJobRequest,
    ) -> RusotoFuture<DescribeTopicsDetectionJobResponse, DescribeTopicsDetectionJobError>;

    /// <p>Determines the dominant language of the input text. For a list of languages that Amazon Comprehend can detect, see <a href="http://docs.aws.amazon.com/comprehend/latest/dg/how-languages.html">Amazon Comprehend Supported Languages</a>. </p>
    fn detect_dominant_language(
        &self,
        input: DetectDominantLanguageRequest,
    ) -> RusotoFuture<DetectDominantLanguageResponse, DetectDominantLanguageError>;

    /// <p>Inspects text for named entities, and returns information about them. For more information, about named entities, see <a>how-entities</a>. </p>
    fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> RusotoFuture<DetectEntitiesResponse, DetectEntitiesError>;

    /// <p>Detects the key noun phrases found in the text. </p>
    fn detect_key_phrases(
        &self,
        input: DetectKeyPhrasesRequest,
    ) -> RusotoFuture<DetectKeyPhrasesResponse, DetectKeyPhrasesError>;

    /// <p>Inspects text and returns an inference of the prevailing sentiment (<code>POSITIVE</code>, <code>NEUTRAL</code>, <code>MIXED</code>, or <code>NEGATIVE</code>). </p>
    fn detect_sentiment(
        &self,
        input: DetectSentimentRequest,
    ) -> RusotoFuture<DetectSentimentResponse, DetectSentimentError>;

    /// <p>Inspects text for syntax and the part of speech of words in the document. For more information, <a>how-syntax</a>.</p>
    fn detect_syntax(
        &self,
        input: DetectSyntaxRequest,
    ) -> RusotoFuture<DetectSyntaxResponse, DetectSyntaxError>;

    /// <p>Gets a list of the dominant language detection jobs that you have submitted.</p>
    fn list_dominant_language_detection_jobs(
        &self,
        input: ListDominantLanguageDetectionJobsRequest,
    ) -> RusotoFuture<
        ListDominantLanguageDetectionJobsResponse,
        ListDominantLanguageDetectionJobsError,
    >;

    /// <p>Gets a list of the entity detection jobs that you have submitted.</p>
    fn list_entities_detection_jobs(
        &self,
        input: ListEntitiesDetectionJobsRequest,
    ) -> RusotoFuture<ListEntitiesDetectionJobsResponse, ListEntitiesDetectionJobsError>;

    /// <p>Get a list of key phrase detection jobs that you have submitted.</p>
    fn list_key_phrases_detection_jobs(
        &self,
        input: ListKeyPhrasesDetectionJobsRequest,
    ) -> RusotoFuture<ListKeyPhrasesDetectionJobsResponse, ListKeyPhrasesDetectionJobsError>;

    /// <p>Gets a list of sentiment detection jobs that you have submitted.</p>
    fn list_sentiment_detection_jobs(
        &self,
        input: ListSentimentDetectionJobsRequest,
    ) -> RusotoFuture<ListSentimentDetectionJobsResponse, ListSentimentDetectionJobsError>;

    /// <p>Gets a list of the topic detection jobs that you have submitted.</p>
    fn list_topics_detection_jobs(
        &self,
        input: ListTopicsDetectionJobsRequest,
    ) -> RusotoFuture<ListTopicsDetectionJobsResponse, ListTopicsDetectionJobsError>;

    /// <p>Starts an asynchronous dominant language detection job for a collection of documents. Use the operation to track the status of a job.</p>
    fn start_dominant_language_detection_job(
        &self,
        input: StartDominantLanguageDetectionJobRequest,
    ) -> RusotoFuture<
        StartDominantLanguageDetectionJobResponse,
        StartDominantLanguageDetectionJobError,
    >;

    /// <p>Starts an asynchronous entity detection job for a collection of documents. Use the operation to track the status of a job.</p>
    fn start_entities_detection_job(
        &self,
        input: StartEntitiesDetectionJobRequest,
    ) -> RusotoFuture<StartEntitiesDetectionJobResponse, StartEntitiesDetectionJobError>;

    /// <p>Starts an asynchronous key phrase detection job for a collection of documents. Use the operation to track the status of a job.</p>
    fn start_key_phrases_detection_job(
        &self,
        input: StartKeyPhrasesDetectionJobRequest,
    ) -> RusotoFuture<StartKeyPhrasesDetectionJobResponse, StartKeyPhrasesDetectionJobError>;

    /// <p>Starts an asynchronous sentiment detection job for a collection of documents. use the operation to track the status of a job.</p>
    fn start_sentiment_detection_job(
        &self,
        input: StartSentimentDetectionJobRequest,
    ) -> RusotoFuture<StartSentimentDetectionJobResponse, StartSentimentDetectionJobError>;

    /// <p>Starts an asynchronous topic detection job. Use the <code>DescribeTopicDetectionJob</code> operation to track the status of a job.</p>
    fn start_topics_detection_job(
        &self,
        input: StartTopicsDetectionJobRequest,
    ) -> RusotoFuture<StartTopicsDetectionJobResponse, StartTopicsDetectionJobError>;

    /// <p>Stops a dominant language detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    fn stop_dominant_language_detection_job(
        &self,
        input: StopDominantLanguageDetectionJobRequest,
    ) -> RusotoFuture<StopDominantLanguageDetectionJobResponse, StopDominantLanguageDetectionJobError>;

    /// <p>Stops an entities detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    fn stop_entities_detection_job(
        &self,
        input: StopEntitiesDetectionJobRequest,
    ) -> RusotoFuture<StopEntitiesDetectionJobResponse, StopEntitiesDetectionJobError>;

    /// <p>Stops a key phrases detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    fn stop_key_phrases_detection_job(
        &self,
        input: StopKeyPhrasesDetectionJobRequest,
    ) -> RusotoFuture<StopKeyPhrasesDetectionJobResponse, StopKeyPhrasesDetectionJobError>;

    /// <p>Stops a sentiment detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is be stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    fn stop_sentiment_detection_job(
        &self,
        input: StopSentimentDetectionJobRequest,
    ) -> RusotoFuture<StopSentimentDetectionJobResponse, StopSentimentDetectionJobError>;
}
/// A client for the Amazon Comprehend API.
pub struct ComprehendClient {
    client: Client,
    region: region::Region,
}

impl ComprehendClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ComprehendClient {
        ComprehendClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ComprehendClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ComprehendClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Comprehend for ComprehendClient {
    /// <p>Determines the dominant language of the input text for a batch of documents. For a list of languages that Amazon Comprehend can detect, see <a href="http://docs.aws.amazon.com/comprehend/latest/dg/how-languages.html">Amazon Comprehend Supported Languages</a>. </p>
    fn batch_detect_dominant_language(
        &self,
        input: BatchDetectDominantLanguageRequest,
    ) -> RusotoFuture<BatchDetectDominantLanguageResponse, BatchDetectDominantLanguageError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.BatchDetectDominantLanguage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchDetectDominantLanguageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchDetectDominantLanguageError::from_response(response))
                }))
            }
        })
    }

    /// <p>Inspects the text of a batch of documents for named entities and returns information about them. For more information about named entities, see <a>how-entities</a> </p>
    fn batch_detect_entities(
        &self,
        input: BatchDetectEntitiesRequest,
    ) -> RusotoFuture<BatchDetectEntitiesResponse, BatchDetectEntitiesError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.BatchDetectEntities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchDetectEntitiesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDetectEntitiesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Detects the key noun phrases found in a batch of documents.</p>
    fn batch_detect_key_phrases(
        &self,
        input: BatchDetectKeyPhrasesRequest,
    ) -> RusotoFuture<BatchDetectKeyPhrasesResponse, BatchDetectKeyPhrasesError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.BatchDetectKeyPhrases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchDetectKeyPhrasesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDetectKeyPhrasesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Inspects a batch of documents and returns an inference of the prevailing sentiment, <code>POSITIVE</code>, <code>NEUTRAL</code>, <code>MIXED</code>, or <code>NEGATIVE</code>, in each one.</p>
    fn batch_detect_sentiment(
        &self,
        input: BatchDetectSentimentRequest,
    ) -> RusotoFuture<BatchDetectSentimentResponse, BatchDetectSentimentError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.BatchDetectSentiment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchDetectSentimentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchDetectSentimentError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Inspects the text of a batch of documents for the syntax and part of speech of the words in the document and returns information about them. For more information, see <a>how-syntax</a>.</p>
    fn batch_detect_syntax(
        &self,
        input: BatchDetectSyntaxRequest,
    ) -> RusotoFuture<BatchDetectSyntaxResponse, BatchDetectSyntaxError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.BatchDetectSyntax");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<BatchDetectSyntaxResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchDetectSyntaxError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the properties associated with a dominant language detection job. Use this operation to get the status of a detection job.</p>
    fn describe_dominant_language_detection_job(
        &self,
        input: DescribeDominantLanguageDetectionJobRequest,
    ) -> RusotoFuture<
        DescribeDominantLanguageDetectionJobResponse,
        DescribeDominantLanguageDetectionJobError,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeDominantLanguageDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDominantLanguageDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDominantLanguageDetectionJobError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Gets the properties associated with an entities detection job. Use this operation to get the status of a detection job.</p>
    fn describe_entities_detection_job(
        &self,
        input: DescribeEntitiesDetectionJobRequest,
    ) -> RusotoFuture<DescribeEntitiesDetectionJobResponse, DescribeEntitiesDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeEntitiesDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeEntitiesDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEntitiesDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the properties associated with a key phrases detection job. Use this operation to get the status of a detection job.</p>
    fn describe_key_phrases_detection_job(
        &self,
        input: DescribeKeyPhrasesDetectionJobRequest,
    ) -> RusotoFuture<DescribeKeyPhrasesDetectionJobResponse, DescribeKeyPhrasesDetectionJobError>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeKeyPhrasesDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeKeyPhrasesDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeKeyPhrasesDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the properties associated with a sentiment detection job. Use this operation to get the status of a detection job.</p>
    fn describe_sentiment_detection_job(
        &self,
        input: DescribeSentimentDetectionJobRequest,
    ) -> RusotoFuture<DescribeSentimentDetectionJobResponse, DescribeSentimentDetectionJobError>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeSentimentDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeSentimentDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSentimentDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the properties associated with a topic detection job. Use this operation to get the status of a detection job.</p>
    fn describe_topics_detection_job(
        &self,
        input: DescribeTopicsDetectionJobRequest,
    ) -> RusotoFuture<DescribeTopicsDetectionJobResponse, DescribeTopicsDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeTopicsDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTopicsDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTopicsDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Determines the dominant language of the input text. For a list of languages that Amazon Comprehend can detect, see <a href="http://docs.aws.amazon.com/comprehend/latest/dg/how-languages.html">Amazon Comprehend Supported Languages</a>. </p>
    fn detect_dominant_language(
        &self,
        input: DetectDominantLanguageRequest,
    ) -> RusotoFuture<DetectDominantLanguageResponse, DetectDominantLanguageError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectDominantLanguage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectDominantLanguageResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DetectDominantLanguageError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Inspects text for named entities, and returns information about them. For more information, about named entities, see <a>how-entities</a>. </p>
    fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> RusotoFuture<DetectEntitiesResponse, DetectEntitiesError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectEntities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectEntitiesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectEntitiesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detects the key noun phrases found in the text. </p>
    fn detect_key_phrases(
        &self,
        input: DetectKeyPhrasesRequest,
    ) -> RusotoFuture<DetectKeyPhrasesResponse, DetectKeyPhrasesError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectKeyPhrases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectKeyPhrasesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectKeyPhrasesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Inspects text and returns an inference of the prevailing sentiment (<code>POSITIVE</code>, <code>NEUTRAL</code>, <code>MIXED</code>, or <code>NEGATIVE</code>). </p>
    fn detect_sentiment(
        &self,
        input: DetectSentimentRequest,
    ) -> RusotoFuture<DetectSentimentResponse, DetectSentimentError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectSentiment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectSentimentResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectSentimentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Inspects text for syntax and the part of speech of words in the document. For more information, <a>how-syntax</a>.</p>
    fn detect_syntax(
        &self,
        input: DetectSyntaxRequest,
    ) -> RusotoFuture<DetectSyntaxResponse, DetectSyntaxError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectSyntax");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DetectSyntaxResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectSyntaxError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of the dominant language detection jobs that you have submitted.</p>
    fn list_dominant_language_detection_jobs(
        &self,
        input: ListDominantLanguageDetectionJobsRequest,
    ) -> RusotoFuture<
        ListDominantLanguageDetectionJobsResponse,
        ListDominantLanguageDetectionJobsError,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListDominantLanguageDetectionJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDominantLanguageDetectionJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDominantLanguageDetectionJobsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Gets a list of the entity detection jobs that you have submitted.</p>
    fn list_entities_detection_jobs(
        &self,
        input: ListEntitiesDetectionJobsRequest,
    ) -> RusotoFuture<ListEntitiesDetectionJobsResponse, ListEntitiesDetectionJobsError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListEntitiesDetectionJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListEntitiesDetectionJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListEntitiesDetectionJobsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Get a list of key phrase detection jobs that you have submitted.</p>
    fn list_key_phrases_detection_jobs(
        &self,
        input: ListKeyPhrasesDetectionJobsRequest,
    ) -> RusotoFuture<ListKeyPhrasesDetectionJobsResponse, ListKeyPhrasesDetectionJobsError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListKeyPhrasesDetectionJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListKeyPhrasesDetectionJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListKeyPhrasesDetectionJobsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets a list of sentiment detection jobs that you have submitted.</p>
    fn list_sentiment_detection_jobs(
        &self,
        input: ListSentimentDetectionJobsRequest,
    ) -> RusotoFuture<ListSentimentDetectionJobsResponse, ListSentimentDetectionJobsError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListSentimentDetectionJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSentimentDetectionJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSentimentDetectionJobsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets a list of the topic detection jobs that you have submitted.</p>
    fn list_topics_detection_jobs(
        &self,
        input: ListTopicsDetectionJobsRequest,
    ) -> RusotoFuture<ListTopicsDetectionJobsResponse, ListTopicsDetectionJobsError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListTopicsDetectionJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTopicsDetectionJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTopicsDetectionJobsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts an asynchronous dominant language detection job for a collection of documents. Use the operation to track the status of a job.</p>
    fn start_dominant_language_detection_job(
        &self,
        input: StartDominantLanguageDetectionJobRequest,
    ) -> RusotoFuture<
        StartDominantLanguageDetectionJobResponse,
        StartDominantLanguageDetectionJobError,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartDominantLanguageDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartDominantLanguageDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartDominantLanguageDetectionJobError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Starts an asynchronous entity detection job for a collection of documents. Use the operation to track the status of a job.</p>
    fn start_entities_detection_job(
        &self,
        input: StartEntitiesDetectionJobRequest,
    ) -> RusotoFuture<StartEntitiesDetectionJobResponse, StartEntitiesDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartEntitiesDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartEntitiesDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartEntitiesDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts an asynchronous key phrase detection job for a collection of documents. Use the operation to track the status of a job.</p>
    fn start_key_phrases_detection_job(
        &self,
        input: StartKeyPhrasesDetectionJobRequest,
    ) -> RusotoFuture<StartKeyPhrasesDetectionJobResponse, StartKeyPhrasesDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartKeyPhrasesDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartKeyPhrasesDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartKeyPhrasesDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts an asynchronous sentiment detection job for a collection of documents. use the operation to track the status of a job.</p>
    fn start_sentiment_detection_job(
        &self,
        input: StartSentimentDetectionJobRequest,
    ) -> RusotoFuture<StartSentimentDetectionJobResponse, StartSentimentDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartSentimentDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartSentimentDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartSentimentDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts an asynchronous topic detection job. Use the <code>DescribeTopicDetectionJob</code> operation to track the status of a job.</p>
    fn start_topics_detection_job(
        &self,
        input: StartTopicsDetectionJobRequest,
    ) -> RusotoFuture<StartTopicsDetectionJobResponse, StartTopicsDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartTopicsDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartTopicsDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartTopicsDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops a dominant language detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    fn stop_dominant_language_detection_job(
        &self,
        input: StopDominantLanguageDetectionJobRequest,
    ) -> RusotoFuture<StopDominantLanguageDetectionJobResponse, StopDominantLanguageDetectionJobError>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopDominantLanguageDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopDominantLanguageDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopDominantLanguageDetectionJobError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Stops an entities detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    fn stop_entities_detection_job(
        &self,
        input: StopEntitiesDetectionJobRequest,
    ) -> RusotoFuture<StopEntitiesDetectionJobResponse, StopEntitiesDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopEntitiesDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopEntitiesDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopEntitiesDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops a key phrases detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    fn stop_key_phrases_detection_job(
        &self,
        input: StopKeyPhrasesDetectionJobRequest,
    ) -> RusotoFuture<StopKeyPhrasesDetectionJobResponse, StopKeyPhrasesDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopKeyPhrasesDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopKeyPhrasesDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopKeyPhrasesDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops a sentiment detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is be stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    fn stop_sentiment_detection_job(
        &self,
        input: StopSentimentDetectionJobRequest,
    ) -> RusotoFuture<StopSentimentDetectionJobResponse, StopSentimentDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopSentimentDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopSentimentDetectionJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopSentimentDetectionJobError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
