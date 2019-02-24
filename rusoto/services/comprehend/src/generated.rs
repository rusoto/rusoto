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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Describes the result metrics for the test data associated with an documentation classifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ClassifierEvaluationMetrics {
    /// <p>The fraction of the labels that were correct recognized. It is computed by dividing the number of labels in the test documents that were correctly recognized by the total number of labels in the test documents.</p>
    #[serde(rename = "Accuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    /// <p>A measure of how accurate the classifier results are for the test data. It is derived from the <code>Precision</code> and <code>Recall</code> values. The <code>F1Score</code> is the harmonic average of the two scores. The highest score is 1, and the worst score is 0. </p>
    #[serde(rename = "F1Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    /// <p>A measure of the usefulness of the classifier results in the test data. High precision means that the classifier returned substantially more relevant results than irrelevant ones.</p>
    #[serde(rename = "Precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    /// <p>A measure of how complete the classifier results are for the test data. High recall means that the classifier returned most of the relevant results. </p>
    #[serde(rename = "Recall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
}

/// <p>Provides information about a document classifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ClassifierMetadata {
    /// <p> Describes the result metrics for the test data associated with an documentation classifier.</p>
    #[serde(rename = "EvaluationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<ClassifierEvaluationMetrics>,
    /// <p>The number of labels in the input data. </p>
    #[serde(rename = "NumberOfLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_labels: Option<i64>,
    /// <p>The number of documents in the input data that were used to test the classifier. Typically this is 10 to 20 percent of the input documents.</p>
    #[serde(rename = "NumberOfTestDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_test_documents: Option<i64>,
    /// <p>The number of documents in the input data that were used to train the classifier. Typically this is 80 to 90 percent of the input documents.</p>
    #[serde(rename = "NumberOfTrainedDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_trained_documents: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDocumentClassifierRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Management (IAM) role that grants Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>The name of the document classifier.</p>
    #[serde(rename = "DocumentClassifierName")]
    pub document_classifier_name: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: DocumentClassifierInputDataConfig,
    /// <p>The language of the input documents. You can specify English ("en") or Spanish ("es"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDocumentClassifierResponse {
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier.</p>
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEntityRecognizerRequest {
    /// <p> A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Management (IAM) role that grants Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data. The S3 bucket containing the input data must be located in the same region as the entity recognizer being created. </p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: EntityRecognizerInputDataConfig,
    /// <p> The language of the input documents. All documents must be in the same language. Only English ("en") is currently supported. </p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>The name given to the newly created recognizer. Recognizer names can be a maximum of 256 characters. Alphanumeric characters, hyphens (-) and underscores (_) are allowed. The name must be unique in the account/region.</p>
    #[serde(rename = "RecognizerName")]
    pub recognizer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateEntityRecognizerResponse {
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDocumentClassifierRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier. </p>
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDocumentClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEntityRecognizerRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
    #[serde(rename = "EntityRecognizerArn")]
    pub entity_recognizer_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEntityRecognizerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDocumentClassificationJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDocumentClassificationJobResponse {
    /// <p>An object that describes the properties associated with the document classification job.</p>
    #[serde(rename = "DocumentClassificationJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classification_job_properties: Option<DocumentClassificationJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDocumentClassifierRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier. The operation returns this identifier in its response.</p>
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDocumentClassifierResponse {
    /// <p>An object that contains the properties associated with a document classifier.</p>
    #[serde(rename = "DocumentClassifierProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_properties: Option<DocumentClassifierProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDominantLanguageDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEntitiesDetectionJobResponse {
    /// <p>An object that contains the properties associated with an entities detection job.</p>
    #[serde(rename = "EntitiesDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_detection_job_properties: Option<EntitiesDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEntityRecognizerRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
    #[serde(rename = "EntityRecognizerArn")]
    pub entity_recognizer_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEntityRecognizerResponse {
    /// <p>Describes information associated with an entity recognizer.</p>
    #[serde(rename = "EntityRecognizerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_properties: Option<EntityRecognizerProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeKeyPhrasesDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DetectSyntaxResponse {
    /// <p>A collection of syntax tokens describing the text. For each token, the response provides the text, the token type, where the text begins and ends, and the level of confidence that Amazon Comprehend has that the token is correct. For a list of token types, see <a>how-syntax</a>.</p>
    #[serde(rename = "SyntaxTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_tokens: Option<Vec<SyntaxToken>>,
}

/// <p>Provides information for filtering a list of document classification jobs. For more information, see the operation. You can provide only one filter parameter in each request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DocumentClassificationJobFilter {
    /// <p>Filters on the name of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Filters the list based on job status. Returns only jobs with the specified status.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted before the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing. Returns only jobs submitted after the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>Provides information about a document classification job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DocumentClassificationJobProperties {
    /// <p>The Amazon Resource Name (ARN) of the AWS identity and Access Management (IAM) role that grants Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier. </p>
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
    /// <p>The time that the document classification job completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input data configuration that you supplied when you created the document classification job.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The identifier assigned to the document classification job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name that you assigned to the document classification job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current status of the document classification job. If the status is <code>FAILED</code>, the <code>Message</code> field shows the reason for the failure.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>A description of the status of the job.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The output data configuration that you supplied when you created the document classification job.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>The time that the document classification job was submitted for processing.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

/// <p>Provides information for filtering a list of document classifiers. You can only specify one filtering parameter in a request. For more information, see the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DocumentClassifierFilter {
    /// <p>Filters the list of classifiers based on status. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Filters the list of classifiers based on the time that the classifier was submitted for processing. Returns only classifiers submitted after the specified time. Classifiers are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of classifiers based on the time that the classifier was submitted for processing. Returns only classifiers submitted before the specified time. Classifiers are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>The input properties for training a document classifier. </p> <p>For more information on how the input file is formatted, see <a>how-document-classification-training-data</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentClassifierInputDataConfig {
    /// <p>The Amazon S3 URI for the input data. The S3 bucket must be in the same region as the API endpoint that you are calling. The URI can point to a single input file or it can provide the prefix for a collection of input files.</p> <p>For example, if you use the URI <code>S3://bucketName/prefix</code>, if the prefix is a single file, Amazon Comprehend uses that file as input. If more than one file begins with the prefix, Amazon Comprehend uses all of them as input.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Provides information about a document classifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DocumentClassifierProperties {
    /// <p>Information about the document classifier, including the number of documents used for training the classifier, the number of documents used for test the classifier, and an accuracy rating.</p>
    #[serde(rename = "ClassifierMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifier_metadata: Option<ClassifierMetadata>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Management (IAM) role that grants Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier.</p>
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
    /// <p>The time that training the document classifier completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input data configuration that you supplied when you created the document classifier for training.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<DocumentClassifierInputDataConfig>,
    /// <p>The language code for the language of the documents that the classifier was trained on.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Additional information about the status of the classifier.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status of the document classifier. If the status is <code>TRAINED</code> the classifier is ready to use. If the status is <code>FAILED</code> you can see additional information about why the classifier wasn't trained in the <code>Message</code> field.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time that the document classifier was submitted for training.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    /// <p>The time that training of the document classifier was completed. Indicates the time when the training completes on documentation classifiers. You are billed for the time interval between this time and the value of TrainingStartTime.</p>
    #[serde(rename = "TrainingEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_time: Option<f64>,
    /// <p>Indicates the time when the training starts on documentation classifiers. You are billed for the time interval between this time and the value of TrainingEndTime. </p>
    #[serde(rename = "TrainingStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_start_time: Option<f64>,
}

/// <p>Returns the code for the dominant language in the input text and the level of confidence that Amazon Comprehend has in the accuracy of the detection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct EntitiesDetectionJobProperties {
    /// <p>The Amazon Resource Name (ARN) that gives Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time that the entities detection job completed</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Describes the annotations associated with a entity recognizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityRecognizerAnnotations {
    /// <p> Specifies the Amazon S3 location where the annotations for an entity recognizer are located. The URI must be in the same region as the API endpoint that you are calling.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Describes the training documents submitted with an entity recognizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityRecognizerDocuments {
    /// <p> Specifies the Amazon S3 location where the training documents for an entity recognizer are located. The URI must be in the same region as the API endpoint that you are calling.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Describes the entity recognizer submitted with an entity recognizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityRecognizerEntityList {
    /// <p>Specifies the Amazon S3 location where the entity list is located. The URI must be in the same region as the API endpoint that you are calling.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p> Detailed information about the accuracy of an entity recognizer. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EntityRecognizerEvaluationMetrics {
    /// <p>A measure of how accurate the recognizer results are for the test data. It is derived from the <code>Precision</code> and <code>Recall</code> values. The <code>F1Score</code> is the harmonic average of the two scores. The highest score is 1, and the worst score is 0. </p>
    #[serde(rename = "F1Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    /// <p>A measure of the usefulness of the recognizer results in the test data. High precision means that the recognizer returned substantially more relevant results than irrelevant ones. </p>
    #[serde(rename = "Precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    /// <p>A measure of how complete the recognizer results are for the test data. High recall means that the recognizer returned most of the relevant results.</p>
    #[serde(rename = "Recall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
}

/// <p>Provides information for filtering a list of entity recognizers. You can only specify one filtering parameter in a request. For more information, see the operation./&gt;</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EntityRecognizerFilter {
    /// <p>The status of an entity recognizer.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Filters the list of entities based on the time that the list was submitted for processing. Returns only jobs submitted after the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    /// <p>Filters the list of entities based on the time that the list was submitted for processing. Returns only jobs submitted before the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

/// <p>Specifies the format and location of the input data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityRecognizerInputDataConfig {
    /// <p>S3 location of the annotations file for an entity recognizer.</p>
    #[serde(rename = "Annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<EntityRecognizerAnnotations>,
    /// <p>S3 location of the documents folder for an entity recognizer</p>
    #[serde(rename = "Documents")]
    pub documents: EntityRecognizerDocuments,
    /// <p>S3 location of the entity list for an entity recognizer.</p>
    #[serde(rename = "EntityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_list: Option<EntityRecognizerEntityList>,
    /// <p>The entity types in the input data for an entity recognizer.</p>
    #[serde(rename = "EntityTypes")]
    pub entity_types: Vec<EntityTypesListItem>,
}

/// <p>Detailed information about an entity recognizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EntityRecognizerMetadata {
    /// <p>Entity types from the metadata of an entity recognizer.</p>
    #[serde(rename = "EntityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<EntityRecognizerMetadataEntityTypesListItem>>,
    /// <p> Detailed information about the accuracy of an entity recognizer.</p>
    #[serde(rename = "EvaluationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<EntityRecognizerEvaluationMetrics>,
    /// <p> The number of documents in the input data that were used to test the entity recognizer. Typically this is 10 to 20 percent of the input documents.</p>
    #[serde(rename = "NumberOfTestDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_test_documents: Option<i64>,
    /// <p> The number of documents in the input data that were used to train the entity recognizer. Typically this is 80 to 90 percent of the input documents.</p>
    #[serde(rename = "NumberOfTrainedDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_trained_documents: Option<i64>,
}

/// <p>Individual item from the list of entity types in the metadata of an entity recognizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EntityRecognizerMetadataEntityTypesListItem {
    /// <p>Type of entity from the list of entity types in the metadata of an entity recognizer. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes information about an entity recognizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EntityRecognizerProperties {
    /// <p> The Amazon Resource Name (ARN) of the AWS Identity and Management (IAM) role that grants Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time that the recognizer creation completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
    /// <p>The input data properties of an entity recognizer.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<EntityRecognizerInputDataConfig>,
    /// <p> The language of the input documents. All documents must be in the same language. Only English ("en") is currently supported.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p> A description of the status of the recognizer.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p> Provides information about an entity recognizer.</p>
    #[serde(rename = "RecognizerMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer_metadata: Option<EntityRecognizerMetadata>,
    /// <p>Provides the status of the entity recognizer.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time that the recognizer was submitted for processing.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    /// <p>The time that training of the entity recognizer was completed.</p>
    #[serde(rename = "TrainingEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_time: Option<f64>,
    /// <p>The time that training of the entity recognizer started.</p>
    #[serde(rename = "TrainingStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_start_time: Option<f64>,
}

/// <p>Information about an individual item on a list of entity types.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityTypesListItem {
    /// <p>Entity type of an item on an entity type list.</p>
    #[serde(rename = "Type")]
    pub type_: String,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListDocumentClassificationJobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DocumentClassificationJobFilter>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListDocumentClassificationJobsResponse {
    /// <p>A list containing the properties of each job returned.</p>
    #[serde(rename = "DocumentClassificationJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classification_job_properties_list:
        Option<Vec<DocumentClassificationJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDocumentClassifiersRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs on their name, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DocumentClassifierFilter>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListDocumentClassifiersResponse {
    /// <p>A list containing the properties of each job returned.</p>
    #[serde(rename = "DocumentClassifierPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_properties_list: Option<Vec<DocumentClassifierProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct ListEntityRecognizersRequest {
    /// <p>Filters the list of entities returned. You can filter on <code>Status</code>, <code>SubmitTimeBefore</code>, or <code>SubmitTimeAfter</code>. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EntityRecognizerFilter>,
    /// <p> The maximum number of results to return on each page. The default is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListEntityRecognizersResponse {
    /// <p>The list of properties of an entity recognizer.</p>
    #[serde(rename = "EntityRecognizerPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_properties_list: Option<Vec<EntityRecognizerProperties>>,
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
pub struct StartDocumentClassificationJobRequest {
    /// <p>A unique identifier for the request. If you do not set the client request token, Amazon Comprehend generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the document classifier to use to process the job.</p>
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartDocumentClassificationJobResponse {
    /// <p>The identifier generated for the job. To get the status of the job, use this identifier with the operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p><p>The status of the job:</p> <ul> <li> <p>SUBMITTED - The job has been received and queued for processing.</p> </li> <li> <p>IN<em>PROGRESS - Amazon Comprehend is processing the job.</p> </li> <li> <p>COMPLETED - The job was successfully completed and the output is available.</p> </li> <li> <p>FAILED - The job did not complete. For details, use the operation.</p> </li> <li> <p>STOP</em>REQUESTED - Amazon Comprehend has received a stop request for the job and is processing the request.</p> </li> <li> <p>STOPPED - The job was successfully stopped without completing.</p> </li> </ul></p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The Amazon Resource Name (ARN) that identifies the specific entity recognizer to be used by the <code>StartEntitiesDetectionJob</code>. This ARN is optional and is only used for a custom entity recognition job.</p>
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The language of the input documents. All documents must be in the same language. You can specify any of the languages supported by Amazon Comprehend: English ("en"), Spanish ("es"), French ("fr"), German ("de"), Italian ("it"), or Portuguese ("pt"). If custom entities recognition is used, this parameter is ignored and the language used for training the model is used instead.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartEntitiesDetectionJobResponse {
    /// <p>The identifier generated for the job. To get the status of job, use this identifier with the operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p><p>The status of the job. </p> <ul> <li> <p>SUBMITTED - The job has been received and is queued for processing.</p> </li> <li> <p>IN<em>PROGRESS - Amazon Comprehend is processing the job.</p> </li> <li> <p>COMPLETED - The job was successfully completed and the output is available.</p> </li> <li> <p>FAILED - The job did not complete. To get details, use the operation.</p> </li> <li> <p>STOP</em>REQUESTED - Amazon Comprehend has received a stop request for the job and is processing the request.</p> </li> <li> <p>STOPPED - The job was successfully stopped without completing.</p> </li> </ul></p>
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopTrainingDocumentClassifierRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier currently being trained.</p>
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopTrainingDocumentClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopTrainingEntityRecognizerRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer currently being trained.</p>
    #[serde(rename = "EntityRecognizerArn")]
    pub entity_recognizer_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopTrainingEntityRecognizerResponse {}

/// <p>Represents a work in the input text that was recognized and assigned a part of speech. There is one syntax token record for each word in the source text.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
                    ));
                }
                "InternalServerException" => {
                    return BatchDetectDominantLanguageError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return BatchDetectDominantLanguageError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectDominantLanguageError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return BatchDetectDominantLanguageError::Validation(error_message.to_string());
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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
                    ));
                }
                "InternalServerException" => {
                    return BatchDetectEntitiesError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return BatchDetectEntitiesError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectEntitiesError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "UnsupportedLanguageException" => {
                    return BatchDetectEntitiesError::UnsupportedLanguage(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return BatchDetectEntitiesError::Validation(error_message.to_string());
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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
                    ));
                }
                "InternalServerException" => {
                    return BatchDetectKeyPhrasesError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return BatchDetectKeyPhrasesError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectKeyPhrasesError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "UnsupportedLanguageException" => {
                    return BatchDetectKeyPhrasesError::UnsupportedLanguage(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return BatchDetectKeyPhrasesError::Validation(error_message.to_string());
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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
                    ));
                }
                "InternalServerException" => {
                    return BatchDetectSentimentError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return BatchDetectSentimentError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectSentimentError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "UnsupportedLanguageException" => {
                    return BatchDetectSentimentError::UnsupportedLanguage(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return BatchDetectSentimentError::Validation(error_message.to_string());
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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
                    ));
                }
                "InternalServerException" => {
                    return BatchDetectSyntaxError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return BatchDetectSyntaxError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return BatchDetectSyntaxError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "UnsupportedLanguageException" => {
                    return BatchDetectSyntaxError::UnsupportedLanguage(String::from(error_message));
                }
                "ValidationException" => {
                    return BatchDetectSyntaxError::Validation(error_message.to_string());
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
/// Errors returned by CreateDocumentClassifier
#[derive(Debug, PartialEq)]
pub enum CreateDocumentClassifierError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The maximum number of recognizers per account has been exceeded. Review the recognizers, perform cleanup, and then try your request again.</p>
    ResourceLimitExceeded(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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

impl CreateDocumentClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDocumentClassifierError {
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
                    return CreateDocumentClassifierError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return CreateDocumentClassifierError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceInUseException" => {
                    return CreateDocumentClassifierError::ResourceInUse(String::from(error_message));
                }
                "ResourceLimitExceededException" => {
                    return CreateDocumentClassifierError::ResourceLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return CreateDocumentClassifierError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "UnsupportedLanguageException" => {
                    return CreateDocumentClassifierError::UnsupportedLanguage(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreateDocumentClassifierError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateDocumentClassifierError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDocumentClassifierError {
    fn from(err: serde_json::error::Error) -> CreateDocumentClassifierError {
        CreateDocumentClassifierError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDocumentClassifierError {
    fn from(err: CredentialsError) -> CreateDocumentClassifierError {
        CreateDocumentClassifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDocumentClassifierError {
    fn from(err: HttpDispatchError) -> CreateDocumentClassifierError {
        CreateDocumentClassifierError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDocumentClassifierError {
    fn from(err: io::Error) -> CreateDocumentClassifierError {
        CreateDocumentClassifierError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDocumentClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDocumentClassifierError {
    fn description(&self) -> &str {
        match *self {
            CreateDocumentClassifierError::InternalServer(ref cause) => cause,
            CreateDocumentClassifierError::InvalidRequest(ref cause) => cause,
            CreateDocumentClassifierError::ResourceInUse(ref cause) => cause,
            CreateDocumentClassifierError::ResourceLimitExceeded(ref cause) => cause,
            CreateDocumentClassifierError::TooManyRequests(ref cause) => cause,
            CreateDocumentClassifierError::UnsupportedLanguage(ref cause) => cause,
            CreateDocumentClassifierError::Validation(ref cause) => cause,
            CreateDocumentClassifierError::Credentials(ref err) => err.description(),
            CreateDocumentClassifierError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDocumentClassifierError::ParseError(ref cause) => cause,
            CreateDocumentClassifierError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateEntityRecognizer
#[derive(Debug, PartialEq)]
pub enum CreateEntityRecognizerError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The maximum number of recognizers per account has been exceeded. Review the recognizers, perform cleanup, and then try your request again.</p>
    ResourceLimitExceeded(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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

impl CreateEntityRecognizerError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateEntityRecognizerError {
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
                    return CreateEntityRecognizerError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return CreateEntityRecognizerError::InvalidRequest(String::from(error_message));
                }
                "ResourceInUseException" => {
                    return CreateEntityRecognizerError::ResourceInUse(String::from(error_message));
                }
                "ResourceLimitExceededException" => {
                    return CreateEntityRecognizerError::ResourceLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return CreateEntityRecognizerError::TooManyRequests(String::from(error_message));
                }
                "UnsupportedLanguageException" => {
                    return CreateEntityRecognizerError::UnsupportedLanguage(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreateEntityRecognizerError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateEntityRecognizerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateEntityRecognizerError {
    fn from(err: serde_json::error::Error) -> CreateEntityRecognizerError {
        CreateEntityRecognizerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateEntityRecognizerError {
    fn from(err: CredentialsError) -> CreateEntityRecognizerError {
        CreateEntityRecognizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEntityRecognizerError {
    fn from(err: HttpDispatchError) -> CreateEntityRecognizerError {
        CreateEntityRecognizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEntityRecognizerError {
    fn from(err: io::Error) -> CreateEntityRecognizerError {
        CreateEntityRecognizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEntityRecognizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEntityRecognizerError {
    fn description(&self) -> &str {
        match *self {
            CreateEntityRecognizerError::InternalServer(ref cause) => cause,
            CreateEntityRecognizerError::InvalidRequest(ref cause) => cause,
            CreateEntityRecognizerError::ResourceInUse(ref cause) => cause,
            CreateEntityRecognizerError::ResourceLimitExceeded(ref cause) => cause,
            CreateEntityRecognizerError::TooManyRequests(ref cause) => cause,
            CreateEntityRecognizerError::UnsupportedLanguage(ref cause) => cause,
            CreateEntityRecognizerError::Validation(ref cause) => cause,
            CreateEntityRecognizerError::Credentials(ref err) => err.description(),
            CreateEntityRecognizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEntityRecognizerError::ParseError(ref cause) => cause,
            CreateEntityRecognizerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDocumentClassifier
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentClassifierError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
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

impl DeleteDocumentClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDocumentClassifierError {
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
                    return DeleteDocumentClassifierError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return DeleteDocumentClassifierError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceInUseException" => {
                    return DeleteDocumentClassifierError::ResourceInUse(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteDocumentClassifierError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ResourceUnavailableException" => {
                    return DeleteDocumentClassifierError::ResourceUnavailable(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DeleteDocumentClassifierError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteDocumentClassifierError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteDocumentClassifierError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDocumentClassifierError {
    fn from(err: serde_json::error::Error) -> DeleteDocumentClassifierError {
        DeleteDocumentClassifierError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDocumentClassifierError {
    fn from(err: CredentialsError) -> DeleteDocumentClassifierError {
        DeleteDocumentClassifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDocumentClassifierError {
    fn from(err: HttpDispatchError) -> DeleteDocumentClassifierError {
        DeleteDocumentClassifierError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDocumentClassifierError {
    fn from(err: io::Error) -> DeleteDocumentClassifierError {
        DeleteDocumentClassifierError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDocumentClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDocumentClassifierError {
    fn description(&self) -> &str {
        match *self {
            DeleteDocumentClassifierError::InternalServer(ref cause) => cause,
            DeleteDocumentClassifierError::InvalidRequest(ref cause) => cause,
            DeleteDocumentClassifierError::ResourceInUse(ref cause) => cause,
            DeleteDocumentClassifierError::ResourceNotFound(ref cause) => cause,
            DeleteDocumentClassifierError::ResourceUnavailable(ref cause) => cause,
            DeleteDocumentClassifierError::TooManyRequests(ref cause) => cause,
            DeleteDocumentClassifierError::Validation(ref cause) => cause,
            DeleteDocumentClassifierError::Credentials(ref err) => err.description(),
            DeleteDocumentClassifierError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDocumentClassifierError::ParseError(ref cause) => cause,
            DeleteDocumentClassifierError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEntityRecognizer
#[derive(Debug, PartialEq)]
pub enum DeleteEntityRecognizerError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
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

impl DeleteEntityRecognizerError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEntityRecognizerError {
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
                    return DeleteEntityRecognizerError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DeleteEntityRecognizerError::InvalidRequest(String::from(error_message));
                }
                "ResourceInUseException" => {
                    return DeleteEntityRecognizerError::ResourceInUse(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteEntityRecognizerError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ResourceUnavailableException" => {
                    return DeleteEntityRecognizerError::ResourceUnavailable(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DeleteEntityRecognizerError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteEntityRecognizerError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteEntityRecognizerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEntityRecognizerError {
    fn from(err: serde_json::error::Error) -> DeleteEntityRecognizerError {
        DeleteEntityRecognizerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEntityRecognizerError {
    fn from(err: CredentialsError) -> DeleteEntityRecognizerError {
        DeleteEntityRecognizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEntityRecognizerError {
    fn from(err: HttpDispatchError) -> DeleteEntityRecognizerError {
        DeleteEntityRecognizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEntityRecognizerError {
    fn from(err: io::Error) -> DeleteEntityRecognizerError {
        DeleteEntityRecognizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEntityRecognizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEntityRecognizerError {
    fn description(&self) -> &str {
        match *self {
            DeleteEntityRecognizerError::InternalServer(ref cause) => cause,
            DeleteEntityRecognizerError::InvalidRequest(ref cause) => cause,
            DeleteEntityRecognizerError::ResourceInUse(ref cause) => cause,
            DeleteEntityRecognizerError::ResourceNotFound(ref cause) => cause,
            DeleteEntityRecognizerError::ResourceUnavailable(ref cause) => cause,
            DeleteEntityRecognizerError::TooManyRequests(ref cause) => cause,
            DeleteEntityRecognizerError::Validation(ref cause) => cause,
            DeleteEntityRecognizerError::Credentials(ref err) => err.description(),
            DeleteEntityRecognizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEntityRecognizerError::ParseError(ref cause) => cause,
            DeleteEntityRecognizerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDocumentClassificationJob
#[derive(Debug, PartialEq)]
pub enum DescribeDocumentClassificationJobError {
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

impl DescribeDocumentClassificationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDocumentClassificationJobError {
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
                    return DescribeDocumentClassificationJobError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return DescribeDocumentClassificationJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return DescribeDocumentClassificationJobError::JobNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DescribeDocumentClassificationJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeDocumentClassificationJobError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DescribeDocumentClassificationJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDocumentClassificationJobError {
    fn from(err: serde_json::error::Error) -> DescribeDocumentClassificationJobError {
        DescribeDocumentClassificationJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDocumentClassificationJobError {
    fn from(err: CredentialsError) -> DescribeDocumentClassificationJobError {
        DescribeDocumentClassificationJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDocumentClassificationJobError {
    fn from(err: HttpDispatchError) -> DescribeDocumentClassificationJobError {
        DescribeDocumentClassificationJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDocumentClassificationJobError {
    fn from(err: io::Error) -> DescribeDocumentClassificationJobError {
        DescribeDocumentClassificationJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDocumentClassificationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDocumentClassificationJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeDocumentClassificationJobError::InternalServer(ref cause) => cause,
            DescribeDocumentClassificationJobError::InvalidRequest(ref cause) => cause,
            DescribeDocumentClassificationJobError::JobNotFound(ref cause) => cause,
            DescribeDocumentClassificationJobError::TooManyRequests(ref cause) => cause,
            DescribeDocumentClassificationJobError::Validation(ref cause) => cause,
            DescribeDocumentClassificationJobError::Credentials(ref err) => err.description(),
            DescribeDocumentClassificationJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDocumentClassificationJobError::ParseError(ref cause) => cause,
            DescribeDocumentClassificationJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDocumentClassifier
#[derive(Debug, PartialEq)]
pub enum DescribeDocumentClassifierError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
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

impl DescribeDocumentClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDocumentClassifierError {
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
                    return DescribeDocumentClassifierError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return DescribeDocumentClassifierError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return DescribeDocumentClassifierError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DescribeDocumentClassifierError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeDocumentClassifierError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeDocumentClassifierError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDocumentClassifierError {
    fn from(err: serde_json::error::Error) -> DescribeDocumentClassifierError {
        DescribeDocumentClassifierError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDocumentClassifierError {
    fn from(err: CredentialsError) -> DescribeDocumentClassifierError {
        DescribeDocumentClassifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDocumentClassifierError {
    fn from(err: HttpDispatchError) -> DescribeDocumentClassifierError {
        DescribeDocumentClassifierError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDocumentClassifierError {
    fn from(err: io::Error) -> DescribeDocumentClassifierError {
        DescribeDocumentClassifierError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDocumentClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDocumentClassifierError {
    fn description(&self) -> &str {
        match *self {
            DescribeDocumentClassifierError::InternalServer(ref cause) => cause,
            DescribeDocumentClassifierError::InvalidRequest(ref cause) => cause,
            DescribeDocumentClassifierError::ResourceNotFound(ref cause) => cause,
            DescribeDocumentClassifierError::TooManyRequests(ref cause) => cause,
            DescribeDocumentClassifierError::Validation(ref cause) => cause,
            DescribeDocumentClassifierError::Credentials(ref err) => err.description(),
            DescribeDocumentClassifierError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDocumentClassifierError::ParseError(ref cause) => cause,
            DescribeDocumentClassifierError::Unknown(_) => "unknown error",
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
                    ));
                }
                "InvalidRequestException" => {
                    return DescribeDominantLanguageDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return DescribeDominantLanguageDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DescribeDominantLanguageDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeDominantLanguageDetectionJobError::Validation(
                        error_message.to_string(),
                    );
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
                    ));
                }
                "InvalidRequestException" => {
                    return DescribeEntitiesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return DescribeEntitiesDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DescribeEntitiesDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeEntitiesDetectionJobError::Validation(error_message.to_string());
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
/// Errors returned by DescribeEntityRecognizer
#[derive(Debug, PartialEq)]
pub enum DescribeEntityRecognizerError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
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

impl DescribeEntityRecognizerError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeEntityRecognizerError {
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
                    return DescribeEntityRecognizerError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return DescribeEntityRecognizerError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return DescribeEntityRecognizerError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DescribeEntityRecognizerError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeEntityRecognizerError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeEntityRecognizerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeEntityRecognizerError {
    fn from(err: serde_json::error::Error) -> DescribeEntityRecognizerError {
        DescribeEntityRecognizerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEntityRecognizerError {
    fn from(err: CredentialsError) -> DescribeEntityRecognizerError {
        DescribeEntityRecognizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEntityRecognizerError {
    fn from(err: HttpDispatchError) -> DescribeEntityRecognizerError {
        DescribeEntityRecognizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEntityRecognizerError {
    fn from(err: io::Error) -> DescribeEntityRecognizerError {
        DescribeEntityRecognizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEntityRecognizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEntityRecognizerError {
    fn description(&self) -> &str {
        match *self {
            DescribeEntityRecognizerError::InternalServer(ref cause) => cause,
            DescribeEntityRecognizerError::InvalidRequest(ref cause) => cause,
            DescribeEntityRecognizerError::ResourceNotFound(ref cause) => cause,
            DescribeEntityRecognizerError::TooManyRequests(ref cause) => cause,
            DescribeEntityRecognizerError::Validation(ref cause) => cause,
            DescribeEntityRecognizerError::Credentials(ref err) => err.description(),
            DescribeEntityRecognizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEntityRecognizerError::ParseError(ref cause) => cause,
            DescribeEntityRecognizerError::Unknown(_) => "unknown error",
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
                    ));
                }
                "InvalidRequestException" => {
                    return DescribeKeyPhrasesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return DescribeKeyPhrasesDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DescribeKeyPhrasesDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeKeyPhrasesDetectionJobError::Validation(
                        error_message.to_string(),
                    );
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
                    ));
                }
                "InvalidRequestException" => {
                    return DescribeSentimentDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return DescribeSentimentDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return DescribeSentimentDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeSentimentDetectionJobError::Validation(error_message.to_string());
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
                    ));
                }
                "InvalidRequestException" => {
                    return DescribeTopicsDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return DescribeTopicsDetectionJobError::JobNotFound(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DescribeTopicsDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DescribeTopicsDetectionJobError::Validation(error_message.to_string());
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
                    return DetectDominantLanguageError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DetectDominantLanguageError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return DetectDominantLanguageError::TextSizeLimitExceeded(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DetectDominantLanguageError::Validation(error_message.to_string());
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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
                    return DetectEntitiesError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DetectEntitiesError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return DetectEntitiesError::TextSizeLimitExceeded(String::from(error_message));
                }
                "UnsupportedLanguageException" => {
                    return DetectEntitiesError::UnsupportedLanguage(String::from(error_message));
                }
                "ValidationException" => {
                    return DetectEntitiesError::Validation(error_message.to_string());
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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
                    return DetectKeyPhrasesError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DetectKeyPhrasesError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return DetectKeyPhrasesError::TextSizeLimitExceeded(String::from(error_message));
                }
                "UnsupportedLanguageException" => {
                    return DetectKeyPhrasesError::UnsupportedLanguage(String::from(error_message));
                }
                "ValidationException" => {
                    return DetectKeyPhrasesError::Validation(error_message.to_string());
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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
                    return DetectSentimentError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DetectSentimentError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return DetectSentimentError::TextSizeLimitExceeded(String::from(error_message));
                }
                "UnsupportedLanguageException" => {
                    return DetectSentimentError::UnsupportedLanguage(String::from(error_message));
                }
                "ValidationException" => {
                    return DetectSentimentError::Validation(error_message.to_string());
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, Amazon Comprehend accepts only English or Spanish text. </p>
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
                    return DetectSyntaxError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DetectSyntaxError::InvalidRequest(String::from(error_message));
                }
                "TextSizeLimitExceededException" => {
                    return DetectSyntaxError::TextSizeLimitExceeded(String::from(error_message));
                }
                "UnsupportedLanguageException" => {
                    return DetectSyntaxError::UnsupportedLanguage(String::from(error_message));
                }
                "ValidationException" => {
                    return DetectSyntaxError::Validation(error_message.to_string());
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
/// Errors returned by ListDocumentClassificationJobs
#[derive(Debug, PartialEq)]
pub enum ListDocumentClassificationJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListDocumentClassificationJobs</code> operation is invalid. Specify a different filter.</p>
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

impl ListDocumentClassificationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDocumentClassificationJobsError {
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
                    return ListDocumentClassificationJobsError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidFilterException" => {
                    return ListDocumentClassificationJobsError::InvalidFilter(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return ListDocumentClassificationJobsError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return ListDocumentClassificationJobsError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListDocumentClassificationJobsError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return ListDocumentClassificationJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDocumentClassificationJobsError {
    fn from(err: serde_json::error::Error) -> ListDocumentClassificationJobsError {
        ListDocumentClassificationJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDocumentClassificationJobsError {
    fn from(err: CredentialsError) -> ListDocumentClassificationJobsError {
        ListDocumentClassificationJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDocumentClassificationJobsError {
    fn from(err: HttpDispatchError) -> ListDocumentClassificationJobsError {
        ListDocumentClassificationJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDocumentClassificationJobsError {
    fn from(err: io::Error) -> ListDocumentClassificationJobsError {
        ListDocumentClassificationJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDocumentClassificationJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDocumentClassificationJobsError {
    fn description(&self) -> &str {
        match *self {
            ListDocumentClassificationJobsError::InternalServer(ref cause) => cause,
            ListDocumentClassificationJobsError::InvalidFilter(ref cause) => cause,
            ListDocumentClassificationJobsError::InvalidRequest(ref cause) => cause,
            ListDocumentClassificationJobsError::TooManyRequests(ref cause) => cause,
            ListDocumentClassificationJobsError::Validation(ref cause) => cause,
            ListDocumentClassificationJobsError::Credentials(ref err) => err.description(),
            ListDocumentClassificationJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDocumentClassificationJobsError::ParseError(ref cause) => cause,
            ListDocumentClassificationJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDocumentClassifiers
#[derive(Debug, PartialEq)]
pub enum ListDocumentClassifiersError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListDocumentClassificationJobs</code> operation is invalid. Specify a different filter.</p>
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

impl ListDocumentClassifiersError {
    pub fn from_response(res: BufferedHttpResponse) -> ListDocumentClassifiersError {
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
                    return ListDocumentClassifiersError::InternalServer(String::from(error_message));
                }
                "InvalidFilterException" => {
                    return ListDocumentClassifiersError::InvalidFilter(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListDocumentClassifiersError::InvalidRequest(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListDocumentClassifiersError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListDocumentClassifiersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListDocumentClassifiersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDocumentClassifiersError {
    fn from(err: serde_json::error::Error) -> ListDocumentClassifiersError {
        ListDocumentClassifiersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDocumentClassifiersError {
    fn from(err: CredentialsError) -> ListDocumentClassifiersError {
        ListDocumentClassifiersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDocumentClassifiersError {
    fn from(err: HttpDispatchError) -> ListDocumentClassifiersError {
        ListDocumentClassifiersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDocumentClassifiersError {
    fn from(err: io::Error) -> ListDocumentClassifiersError {
        ListDocumentClassifiersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDocumentClassifiersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDocumentClassifiersError {
    fn description(&self) -> &str {
        match *self {
            ListDocumentClassifiersError::InternalServer(ref cause) => cause,
            ListDocumentClassifiersError::InvalidFilter(ref cause) => cause,
            ListDocumentClassifiersError::InvalidRequest(ref cause) => cause,
            ListDocumentClassifiersError::TooManyRequests(ref cause) => cause,
            ListDocumentClassifiersError::Validation(ref cause) => cause,
            ListDocumentClassifiersError::Credentials(ref err) => err.description(),
            ListDocumentClassifiersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDocumentClassifiersError::ParseError(ref cause) => cause,
            ListDocumentClassifiersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDominantLanguageDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListDominantLanguageDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListDocumentClassificationJobs</code> operation is invalid. Specify a different filter.</p>
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
                    ));
                }
                "InvalidFilterException" => {
                    return ListDominantLanguageDetectionJobsError::InvalidFilter(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return ListDominantLanguageDetectionJobsError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return ListDominantLanguageDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListDominantLanguageDetectionJobsError::Validation(
                        error_message.to_string(),
                    );
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
    /// <p>The filter specified for the <code>ListDocumentClassificationJobs</code> operation is invalid. Specify a different filter.</p>
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
                    ));
                }
                "InvalidFilterException" => {
                    return ListEntitiesDetectionJobsError::InvalidFilter(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return ListEntitiesDetectionJobsError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return ListEntitiesDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListEntitiesDetectionJobsError::Validation(error_message.to_string());
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
/// Errors returned by ListEntityRecognizers
#[derive(Debug, PartialEq)]
pub enum ListEntityRecognizersError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListDocumentClassificationJobs</code> operation is invalid. Specify a different filter.</p>
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

impl ListEntityRecognizersError {
    pub fn from_response(res: BufferedHttpResponse) -> ListEntityRecognizersError {
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
                    return ListEntityRecognizersError::InternalServer(String::from(error_message));
                }
                "InvalidFilterException" => {
                    return ListEntityRecognizersError::InvalidFilter(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListEntityRecognizersError::InvalidRequest(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListEntityRecognizersError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListEntityRecognizersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListEntityRecognizersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListEntityRecognizersError {
    fn from(err: serde_json::error::Error) -> ListEntityRecognizersError {
        ListEntityRecognizersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListEntityRecognizersError {
    fn from(err: CredentialsError) -> ListEntityRecognizersError {
        ListEntityRecognizersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListEntityRecognizersError {
    fn from(err: HttpDispatchError) -> ListEntityRecognizersError {
        ListEntityRecognizersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListEntityRecognizersError {
    fn from(err: io::Error) -> ListEntityRecognizersError {
        ListEntityRecognizersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListEntityRecognizersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEntityRecognizersError {
    fn description(&self) -> &str {
        match *self {
            ListEntityRecognizersError::InternalServer(ref cause) => cause,
            ListEntityRecognizersError::InvalidFilter(ref cause) => cause,
            ListEntityRecognizersError::InvalidRequest(ref cause) => cause,
            ListEntityRecognizersError::TooManyRequests(ref cause) => cause,
            ListEntityRecognizersError::Validation(ref cause) => cause,
            ListEntityRecognizersError::Credentials(ref err) => err.description(),
            ListEntityRecognizersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListEntityRecognizersError::ParseError(ref cause) => cause,
            ListEntityRecognizersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListKeyPhrasesDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListKeyPhrasesDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the <code>ListDocumentClassificationJobs</code> operation is invalid. Specify a different filter.</p>
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
                    ));
                }
                "InvalidFilterException" => {
                    return ListKeyPhrasesDetectionJobsError::InvalidFilter(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return ListKeyPhrasesDetectionJobsError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return ListKeyPhrasesDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListKeyPhrasesDetectionJobsError::Validation(error_message.to_string());
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
    /// <p>The filter specified for the <code>ListDocumentClassificationJobs</code> operation is invalid. Specify a different filter.</p>
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
                    ));
                }
                "InvalidFilterException" => {
                    return ListSentimentDetectionJobsError::InvalidFilter(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return ListSentimentDetectionJobsError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return ListSentimentDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListSentimentDetectionJobsError::Validation(error_message.to_string());
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
    /// <p>The filter specified for the <code>ListDocumentClassificationJobs</code> operation is invalid. Specify a different filter.</p>
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
                    return ListTopicsDetectionJobsError::InternalServer(String::from(error_message));
                }
                "InvalidFilterException" => {
                    return ListTopicsDetectionJobsError::InvalidFilter(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListTopicsDetectionJobsError::InvalidRequest(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListTopicsDetectionJobsError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListTopicsDetectionJobsError::Validation(error_message.to_string());
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
/// Errors returned by StartDocumentClassificationJob
#[derive(Debug, PartialEq)]
pub enum StartDocumentClassificationJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
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

impl StartDocumentClassificationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> StartDocumentClassificationJobError {
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
                    return StartDocumentClassificationJobError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return StartDocumentClassificationJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return StartDocumentClassificationJobError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ResourceUnavailableException" => {
                    return StartDocumentClassificationJobError::ResourceUnavailable(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return StartDocumentClassificationJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartDocumentClassificationJobError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return StartDocumentClassificationJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartDocumentClassificationJobError {
    fn from(err: serde_json::error::Error) -> StartDocumentClassificationJobError {
        StartDocumentClassificationJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartDocumentClassificationJobError {
    fn from(err: CredentialsError) -> StartDocumentClassificationJobError {
        StartDocumentClassificationJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartDocumentClassificationJobError {
    fn from(err: HttpDispatchError) -> StartDocumentClassificationJobError {
        StartDocumentClassificationJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartDocumentClassificationJobError {
    fn from(err: io::Error) -> StartDocumentClassificationJobError {
        StartDocumentClassificationJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartDocumentClassificationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartDocumentClassificationJobError {
    fn description(&self) -> &str {
        match *self {
            StartDocumentClassificationJobError::InternalServer(ref cause) => cause,
            StartDocumentClassificationJobError::InvalidRequest(ref cause) => cause,
            StartDocumentClassificationJobError::ResourceNotFound(ref cause) => cause,
            StartDocumentClassificationJobError::ResourceUnavailable(ref cause) => cause,
            StartDocumentClassificationJobError::TooManyRequests(ref cause) => cause,
            StartDocumentClassificationJobError::Validation(ref cause) => cause,
            StartDocumentClassificationJobError::Credentials(ref err) => err.description(),
            StartDocumentClassificationJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartDocumentClassificationJobError::ParseError(ref cause) => cause,
            StartDocumentClassificationJobError::Unknown(_) => "unknown error",
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
                    ));
                }
                "InvalidRequestException" => {
                    return StartDominantLanguageDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return StartDominantLanguageDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartDominantLanguageDetectionJobError::Validation(
                        error_message.to_string(),
                    );
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
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
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
                    ));
                }
                "InvalidRequestException" => {
                    return StartEntitiesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return StartEntitiesDetectionJobError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ResourceUnavailableException" => {
                    return StartEntitiesDetectionJobError::ResourceUnavailable(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return StartEntitiesDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartEntitiesDetectionJobError::Validation(error_message.to_string());
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
            StartEntitiesDetectionJobError::ResourceNotFound(ref cause) => cause,
            StartEntitiesDetectionJobError::ResourceUnavailable(ref cause) => cause,
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
                    ));
                }
                "InvalidRequestException" => {
                    return StartKeyPhrasesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return StartKeyPhrasesDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartKeyPhrasesDetectionJobError::Validation(error_message.to_string());
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
                    ));
                }
                "InvalidRequestException" => {
                    return StartSentimentDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return StartSentimentDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartSentimentDetectionJobError::Validation(error_message.to_string());
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
                    return StartTopicsDetectionJobError::InternalServer(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return StartTopicsDetectionJobError::InvalidRequest(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return StartTopicsDetectionJobError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartTopicsDetectionJobError::Validation(error_message.to_string());
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
                    ));
                }
                "InvalidRequestException" => {
                    return StopDominantLanguageDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return StopDominantLanguageDetectionJobError::JobNotFound(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StopDominantLanguageDetectionJobError::Validation(
                        error_message.to_string(),
                    );
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
                    ));
                }
                "InvalidRequestException" => {
                    return StopEntitiesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return StopEntitiesDetectionJobError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return StopEntitiesDetectionJobError::Validation(error_message.to_string());
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
                    ));
                }
                "InvalidRequestException" => {
                    return StopKeyPhrasesDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return StopKeyPhrasesDetectionJobError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return StopKeyPhrasesDetectionJobError::Validation(error_message.to_string());
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
                    ));
                }
                "InvalidRequestException" => {
                    return StopSentimentDetectionJobError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "JobNotFoundException" => {
                    return StopSentimentDetectionJobError::JobNotFound(String::from(error_message));
                }
                "ValidationException" => {
                    return StopSentimentDetectionJobError::Validation(error_message.to_string());
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
/// Errors returned by StopTrainingDocumentClassifier
#[derive(Debug, PartialEq)]
pub enum StopTrainingDocumentClassifierError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
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

impl StopTrainingDocumentClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> StopTrainingDocumentClassifierError {
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
                    return StopTrainingDocumentClassifierError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return StopTrainingDocumentClassifierError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return StopTrainingDocumentClassifierError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return StopTrainingDocumentClassifierError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StopTrainingDocumentClassifierError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return StopTrainingDocumentClassifierError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopTrainingDocumentClassifierError {
    fn from(err: serde_json::error::Error) -> StopTrainingDocumentClassifierError {
        StopTrainingDocumentClassifierError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopTrainingDocumentClassifierError {
    fn from(err: CredentialsError) -> StopTrainingDocumentClassifierError {
        StopTrainingDocumentClassifierError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopTrainingDocumentClassifierError {
    fn from(err: HttpDispatchError) -> StopTrainingDocumentClassifierError {
        StopTrainingDocumentClassifierError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopTrainingDocumentClassifierError {
    fn from(err: io::Error) -> StopTrainingDocumentClassifierError {
        StopTrainingDocumentClassifierError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopTrainingDocumentClassifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTrainingDocumentClassifierError {
    fn description(&self) -> &str {
        match *self {
            StopTrainingDocumentClassifierError::InternalServer(ref cause) => cause,
            StopTrainingDocumentClassifierError::InvalidRequest(ref cause) => cause,
            StopTrainingDocumentClassifierError::ResourceNotFound(ref cause) => cause,
            StopTrainingDocumentClassifierError::TooManyRequests(ref cause) => cause,
            StopTrainingDocumentClassifierError::Validation(ref cause) => cause,
            StopTrainingDocumentClassifierError::Credentials(ref err) => err.description(),
            StopTrainingDocumentClassifierError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopTrainingDocumentClassifierError::ParseError(ref cause) => cause,
            StopTrainingDocumentClassifierError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopTrainingEntityRecognizer
#[derive(Debug, PartialEq)]
pub enum StopTrainingEntityRecognizerError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
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

impl StopTrainingEntityRecognizerError {
    pub fn from_response(res: BufferedHttpResponse) -> StopTrainingEntityRecognizerError {
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
                    return StopTrainingEntityRecognizerError::InternalServer(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return StopTrainingEntityRecognizerError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return StopTrainingEntityRecognizerError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "TooManyRequestsException" => {
                    return StopTrainingEntityRecognizerError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StopTrainingEntityRecognizerError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StopTrainingEntityRecognizerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopTrainingEntityRecognizerError {
    fn from(err: serde_json::error::Error) -> StopTrainingEntityRecognizerError {
        StopTrainingEntityRecognizerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopTrainingEntityRecognizerError {
    fn from(err: CredentialsError) -> StopTrainingEntityRecognizerError {
        StopTrainingEntityRecognizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopTrainingEntityRecognizerError {
    fn from(err: HttpDispatchError) -> StopTrainingEntityRecognizerError {
        StopTrainingEntityRecognizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopTrainingEntityRecognizerError {
    fn from(err: io::Error) -> StopTrainingEntityRecognizerError {
        StopTrainingEntityRecognizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopTrainingEntityRecognizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTrainingEntityRecognizerError {
    fn description(&self) -> &str {
        match *self {
            StopTrainingEntityRecognizerError::InternalServer(ref cause) => cause,
            StopTrainingEntityRecognizerError::InvalidRequest(ref cause) => cause,
            StopTrainingEntityRecognizerError::ResourceNotFound(ref cause) => cause,
            StopTrainingEntityRecognizerError::TooManyRequests(ref cause) => cause,
            StopTrainingEntityRecognizerError::Validation(ref cause) => cause,
            StopTrainingEntityRecognizerError::Credentials(ref err) => err.description(),
            StopTrainingEntityRecognizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopTrainingEntityRecognizerError::ParseError(ref cause) => cause,
            StopTrainingEntityRecognizerError::Unknown(_) => "unknown error",
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

    /// <p>Creates a new document classifier that you can use to categorize documents. To create a classifier you provide a set of training documents that labeled with the categories that you want to use. After the classifier is trained you can use it to categorize a set of labeled documents into the categories. For more information, see <a>how-document-classification</a>.</p>
    fn create_document_classifier(
        &self,
        input: CreateDocumentClassifierRequest,
    ) -> RusotoFuture<CreateDocumentClassifierResponse, CreateDocumentClassifierError>;

    /// <p>Creates an entity recognizer using submitted files. After your <code>CreateEntityRecognizer</code> request is submitted, you can check job status using the API. </p>
    fn create_entity_recognizer(
        &self,
        input: CreateEntityRecognizerRequest,
    ) -> RusotoFuture<CreateEntityRecognizerResponse, CreateEntityRecognizerError>;

    /// <p>Deletes a previously created document classifier</p> <p>Only those classifiers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p> <p>This is an asynchronous action that puts the classifier into a DELETING state, and it is then removed by a background job. Once removed, the classifier disappears from your account and is no longer available for use. </p>
    fn delete_document_classifier(
        &self,
        input: DeleteDocumentClassifierRequest,
    ) -> RusotoFuture<DeleteDocumentClassifierResponse, DeleteDocumentClassifierError>;

    /// <p>Deletes an entity recognizer.</p> <p>Only those recognizers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p> <p>This is an asynchronous action that puts the recognizer into a DELETING state, and it is then removed by a background job. Once removed, the recognizer disappears from your account and is no longer available for use. </p>
    fn delete_entity_recognizer(
        &self,
        input: DeleteEntityRecognizerRequest,
    ) -> RusotoFuture<DeleteEntityRecognizerResponse, DeleteEntityRecognizerError>;

    /// <p>Gets the properties associated with a document classification job. Use this operation to get the status of a classification job.</p>
    fn describe_document_classification_job(
        &self,
        input: DescribeDocumentClassificationJobRequest,
    ) -> RusotoFuture<
        DescribeDocumentClassificationJobResponse,
        DescribeDocumentClassificationJobError,
    >;

    /// <p>Gets the properties associated with a document classifier.</p>
    fn describe_document_classifier(
        &self,
        input: DescribeDocumentClassifierRequest,
    ) -> RusotoFuture<DescribeDocumentClassifierResponse, DescribeDocumentClassifierError>;

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

    /// <p>Provides details about an entity recognizer including status, S3 buckets containing training data, recognizer metadata, metrics, and so on.</p>
    fn describe_entity_recognizer(
        &self,
        input: DescribeEntityRecognizerRequest,
    ) -> RusotoFuture<DescribeEntityRecognizerResponse, DescribeEntityRecognizerError>;

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

    /// <p>Gets a list of the documentation classification jobs that you have submitted.</p>
    fn list_document_classification_jobs(
        &self,
        input: ListDocumentClassificationJobsRequest,
    ) -> RusotoFuture<ListDocumentClassificationJobsResponse, ListDocumentClassificationJobsError>;

    /// <p>Gets a list of the document classifiers that you have created.</p>
    fn list_document_classifiers(
        &self,
        input: ListDocumentClassifiersRequest,
    ) -> RusotoFuture<ListDocumentClassifiersResponse, ListDocumentClassifiersError>;

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

    /// <p>Gets a list of the properties of all entity recognizers that you created, including recognizers currently in training. Allows you to filter the list of recognizers based on criteria such as status and submission time. This call returns up to 500 entity recognizers in the list, with a default number of 100 recognizers in the list.</p> <p>The results of this list are not in any particular order. Please get the list and sort locally if needed.</p>
    fn list_entity_recognizers(
        &self,
        input: ListEntityRecognizersRequest,
    ) -> RusotoFuture<ListEntityRecognizersResponse, ListEntityRecognizersError>;

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

    /// <p>Starts an asynchronous document classification job. Use the operation to track the progress of the job.</p>
    fn start_document_classification_job(
        &self,
        input: StartDocumentClassificationJobRequest,
    ) -> RusotoFuture<StartDocumentClassificationJobResponse, StartDocumentClassificationJobError>;

    /// <p>Starts an asynchronous dominant language detection job for a collection of documents. Use the operation to track the status of a job.</p>
    fn start_dominant_language_detection_job(
        &self,
        input: StartDominantLanguageDetectionJobRequest,
    ) -> RusotoFuture<
        StartDominantLanguageDetectionJobResponse,
        StartDominantLanguageDetectionJobError,
    >;

    /// <p>Starts an asynchronous entity detection job for a collection of documents. Use the operation to track the status of a job.</p> <p>This API can be used for either standard entity detection or custom entity recognition. In order to be used for custom entity recognition, the optional <code>EntityRecognizerArn</code> must be used in order to provide access to the recognizer being used to detect the custom entity.</p>
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

    /// <p>Stops a document classifier training job while in progress.</p> <p>If the training job state is <code>TRAINING</code>, the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the training job completes before it can be stopped, it is put into the <code>TRAINED</code>; otherwise the training job is stopped and put into the <code>STOPPED</code> state and the service sends back an HTTP 200 response with an empty HTTP body. </p>
    fn stop_training_document_classifier(
        &self,
        input: StopTrainingDocumentClassifierRequest,
    ) -> RusotoFuture<StopTrainingDocumentClassifierResponse, StopTrainingDocumentClassifierError>;

    /// <p>Stops an entity recognizer training job while in progress.</p> <p>If the training job state is <code>TRAINING</code>, the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the training job completes before it can be stopped, it is put into the <code>TRAINED</code>; otherwise the training job is stopped and putted into the <code>STOPPED</code> state and the service sends back an HTTP 200 response with an empty HTTP body.</p>
    fn stop_training_entity_recognizer(
        &self,
        input: StopTrainingEntityRecognizerRequest,
    ) -> RusotoFuture<StopTrainingEntityRecognizerResponse, StopTrainingEntityRecognizerError>;
}
/// A client for the Amazon Comprehend API.
#[derive(Clone)]
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

    /// <p>Creates a new document classifier that you can use to categorize documents. To create a classifier you provide a set of training documents that labeled with the categories that you want to use. After the classifier is trained you can use it to categorize a set of labeled documents into the categories. For more information, see <a>how-document-classification</a>.</p>
    fn create_document_classifier(
        &self,
        input: CreateDocumentClassifierRequest,
    ) -> RusotoFuture<CreateDocumentClassifierResponse, CreateDocumentClassifierError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.CreateDocumentClassifier",
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

                    serde_json::from_str::<CreateDocumentClassifierResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDocumentClassifierError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an entity recognizer using submitted files. After your <code>CreateEntityRecognizer</code> request is submitted, you can check job status using the API. </p>
    fn create_entity_recognizer(
        &self,
        input: CreateEntityRecognizerRequest,
    ) -> RusotoFuture<CreateEntityRecognizerResponse, CreateEntityRecognizerError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.CreateEntityRecognizer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateEntityRecognizerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateEntityRecognizerError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a previously created document classifier</p> <p>Only those classifiers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p> <p>This is an asynchronous action that puts the classifier into a DELETING state, and it is then removed by a background job. Once removed, the classifier disappears from your account and is no longer available for use. </p>
    fn delete_document_classifier(
        &self,
        input: DeleteDocumentClassifierRequest,
    ) -> RusotoFuture<DeleteDocumentClassifierResponse, DeleteDocumentClassifierError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DeleteDocumentClassifier",
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

                    serde_json::from_str::<DeleteDocumentClassifierResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDocumentClassifierError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes an entity recognizer.</p> <p>Only those recognizers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p> <p>This is an asynchronous action that puts the recognizer into a DELETING state, and it is then removed by a background job. Once removed, the recognizer disappears from your account and is no longer available for use. </p>
    fn delete_entity_recognizer(
        &self,
        input: DeleteEntityRecognizerRequest,
    ) -> RusotoFuture<DeleteEntityRecognizerResponse, DeleteEntityRecognizerError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DeleteEntityRecognizer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteEntityRecognizerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteEntityRecognizerError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets the properties associated with a document classification job. Use this operation to get the status of a classification job.</p>
    fn describe_document_classification_job(
        &self,
        input: DescribeDocumentClassificationJobRequest,
    ) -> RusotoFuture<
        DescribeDocumentClassificationJobResponse,
        DescribeDocumentClassificationJobError,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeDocumentClassificationJob",
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

                    serde_json::from_str::<DescribeDocumentClassificationJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDocumentClassificationJobError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Gets the properties associated with a document classifier.</p>
    fn describe_document_classifier(
        &self,
        input: DescribeDocumentClassifierRequest,
    ) -> RusotoFuture<DescribeDocumentClassifierResponse, DescribeDocumentClassifierError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeDocumentClassifier",
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

                    serde_json::from_str::<DescribeDocumentClassifierResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDocumentClassifierError::from_response(response))
                }))
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

    /// <p>Provides details about an entity recognizer including status, S3 buckets containing training data, recognizer metadata, metrics, and so on.</p>
    fn describe_entity_recognizer(
        &self,
        input: DescribeEntityRecognizerRequest,
    ) -> RusotoFuture<DescribeEntityRecognizerResponse, DescribeEntityRecognizerError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeEntityRecognizer",
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

                    serde_json::from_str::<DescribeEntityRecognizerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEntityRecognizerError::from_response(response))
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

    /// <p>Gets a list of the documentation classification jobs that you have submitted.</p>
    fn list_document_classification_jobs(
        &self,
        input: ListDocumentClassificationJobsRequest,
    ) -> RusotoFuture<ListDocumentClassificationJobsResponse, ListDocumentClassificationJobsError>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListDocumentClassificationJobs",
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

                    serde_json::from_str::<ListDocumentClassificationJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDocumentClassificationJobsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets a list of the document classifiers that you have created.</p>
    fn list_document_classifiers(
        &self,
        input: ListDocumentClassifiersRequest,
    ) -> RusotoFuture<ListDocumentClassifiersResponse, ListDocumentClassifiersError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListDocumentClassifiers",
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

                    serde_json::from_str::<ListDocumentClassifiersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDocumentClassifiersError::from_response(response))
                }))
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

    /// <p>Gets a list of the properties of all entity recognizers that you created, including recognizers currently in training. Allows you to filter the list of recognizers based on criteria such as status and submission time. This call returns up to 500 entity recognizers in the list, with a default number of 100 recognizers in the list.</p> <p>The results of this list are not in any particular order. Please get the list and sort locally if needed.</p>
    fn list_entity_recognizers(
        &self,
        input: ListEntityRecognizersRequest,
    ) -> RusotoFuture<ListEntityRecognizersResponse, ListEntityRecognizersError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.ListEntityRecognizers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListEntityRecognizersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListEntityRecognizersError::from_response(response))
                    }),
                )
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

    /// <p>Starts an asynchronous document classification job. Use the operation to track the progress of the job.</p>
    fn start_document_classification_job(
        &self,
        input: StartDocumentClassificationJobRequest,
    ) -> RusotoFuture<StartDocumentClassificationJobResponse, StartDocumentClassificationJobError>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartDocumentClassificationJob",
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

                    serde_json::from_str::<StartDocumentClassificationJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartDocumentClassificationJobError::from_response(response))
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

    /// <p>Starts an asynchronous entity detection job for a collection of documents. Use the operation to track the status of a job.</p> <p>This API can be used for either standard entity detection or custom entity recognition. In order to be used for custom entity recognition, the optional <code>EntityRecognizerArn</code> must be used in order to provide access to the recognizer being used to detect the custom entity.</p>
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

    /// <p>Stops a document classifier training job while in progress.</p> <p>If the training job state is <code>TRAINING</code>, the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the training job completes before it can be stopped, it is put into the <code>TRAINED</code>; otherwise the training job is stopped and put into the <code>STOPPED</code> state and the service sends back an HTTP 200 response with an empty HTTP body. </p>
    fn stop_training_document_classifier(
        &self,
        input: StopTrainingDocumentClassifierRequest,
    ) -> RusotoFuture<StopTrainingDocumentClassifierResponse, StopTrainingDocumentClassifierError>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopTrainingDocumentClassifier",
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

                    serde_json::from_str::<StopTrainingDocumentClassifierResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopTrainingDocumentClassifierError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops an entity recognizer training job while in progress.</p> <p>If the training job state is <code>TRAINING</code>, the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the training job completes before it can be stopped, it is put into the <code>TRAINED</code>; otherwise the training job is stopped and putted into the <code>STOPPED</code> state and the service sends back an HTTP 200 response with an empty HTTP body.</p>
    fn stop_training_entity_recognizer(
        &self,
        input: StopTrainingEntityRecognizerRequest,
    ) -> RusotoFuture<StopTrainingEntityRecognizerResponse, StopTrainingEntityRecognizerError> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopTrainingEntityRecognizer",
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

                    serde_json::from_str::<StopTrainingEntityRecognizerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopTrainingEntityRecognizerError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
