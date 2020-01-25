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
/// <p>The result of calling the operation. The operation returns one object for each document that is successfully processed by the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDetectDominantLanguageRequest {
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document should contain at least 20 characters and must contain fewer than 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDetectEntitiesRequest {
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document must contain fewer than 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDetectKeyPhrasesRequest {
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDetectSentimentRequest {
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDetectSyntaxRequest {
    /// <p>The language of the input documents. You can specify any of the following languages supported by Amazon Comprehend: German ("de"), English ("en"), Spanish ("es"), French ("fr"), Italian ("it"), or Portuguese ("pt"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A list containing the text of the input documents. The list can contain a maximum of 25 documents. Each document must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "TextList")]
    pub text_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClassifierEvaluationMetrics {
    /// <p>The fraction of the labels that were correct recognized. It is computed by dividing the number of labels in the test documents that were correctly recognized by the total number of labels in the test documents.</p>
    #[serde(rename = "Accuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    /// <p>A measure of how accurate the classifier results are for the test data. It is derived from the <code>Precision</code> and <code>Recall</code> values. The <code>F1Score</code> is the harmonic average of the two scores. The highest score is 1, and the worst score is 0. </p>
    #[serde(rename = "F1Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    /// <p>Indicates the fraction of labels that are incorrectly predicted. Also seen as the fraction of wrong labels compared to the total number of labels. Scores closer to zero are better.</p>
    #[serde(rename = "HammingLoss")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hamming_loss: Option<f64>,
    /// <p>A measure of how accurate the classifier results are for the test data. It is a combination of the <code>Micro Precision</code> and <code>Micro Recall</code> values. The <code>Micro F1Score</code> is the harmonic mean of the two scores. The highest score is 1, and the worst score is 0.</p>
    #[serde(rename = "MicroF1Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub micro_f1_score: Option<f64>,
    /// <p>A measure of the usefulness of the recognizer results in the test data. High precision means that the recognizer returned substantially more relevant results than irrelevant ones. Unlike the Precision metric which comes from averaging the precision of all available labels, this is based on the overall score of all precision scores added together.</p>
    #[serde(rename = "MicroPrecision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub micro_precision: Option<f64>,
    /// <p>A measure of how complete the classifier results are for the test data. High recall means that the classifier returned most of the relevant results. Specifically, this indicates how many of the correct categories in the text that the model can predict. It is a percentage of correct categories in the text that can found. Instead of averaging the recall scores of all labels (as with Recall), micro Recall is based on the overall score of all recall scores added together.</p>
    #[serde(rename = "MicroRecall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub micro_recall: Option<f64>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ClassifyDocumentRequest {
    /// <p>The Amazon Resource Number (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The document text to be analyzed.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClassifyDocumentResponse {
    /// <p>The classes used by the document being analyzed. These are used for multi-class trained models. Individual classes are mutually exclusive and each document is expected to have only a single class assigned to it. For example, an animal can be a dog or a cat, but not both at the same time. </p>
    #[serde(rename = "Classes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<DocumentClass>>,
    /// <p>The labels used the document being analyzed. These are used for multi-label trained models. Individual labels represent different categories that are related in some manner and are not multually exclusive. For example, a movie can be just an action movie, or it can be an action movie, a science fiction movie, and a comedy, all at the same time. </p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<DocumentLabel>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The language of the input documents. You can specify any of the following languages supported by Amazon Comprehend: German ("de"), English ("en"), Spanish ("es"), French ("fr"), Italian ("it"), or Portuguese ("pt"). All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Indicates the mode in which the classifier will be trained. The classifier can be trained in multi-class mode, which identifies one and only one class for each document, or multi-label mode, which identifies one or more labels for each document. In multi-label mode, multiple labels for an individual document are separated by a delimiter. The default delimiter between labels is a pipe (|).</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>Enables the addition of output results configuration parameters for custom classifier jobs.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<DocumentClassifierOutputDataConfig>,
    /// <p>Tags to be associated with the document classifier being created. A tag is a key-value pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with "Sales" as the key might be added to a resource to indicate its use by the sales department. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your custom classifier. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDocumentClassifierResponse {
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier.</p>
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEndpointRequest {
    /// <p>An idempotency token provided by the customer. If this token matches a previous endpoint creation request, Amazon Comprehend will not return a <code>ResourceInUseException</code>. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p> The desired number of inference units to be used by the model using this endpoint. Each inference unit represents of a throughput of 100 characters per second.</p>
    #[serde(rename = "DesiredInferenceUnits")]
    pub desired_inference_units: i64,
    /// <p>This is the descriptive suffix that becomes part of the <code>EndpointArn</code> used for all subsequent requests to this resource. </p>
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,
    /// <p>The Amazon Resource Number (ARN) of the model to which the endpoint will be attached.</p>
    #[serde(rename = "ModelArn")]
    pub model_arn: String,
    /// <p>Tags associated with the endpoint being created. A tag is a key-value pair that adds metadata to the endpoint. For example, a tag with "Sales" as the key might be added to an endpoint to indicate its use by the sales department. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEndpointResponse {
    /// <p>The Amazon Resource Number (ARN) of the endpoint being created.</p>
    #[serde(rename = "EndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Tags to be associated with the entity recognizer being created. A tag is a key-value pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with "Sales" as the key might be added to a resource to indicate its use by the sales department. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your custom entity recognizer. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEntityRecognizerResponse {
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDocumentClassifierRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier. </p>
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDocumentClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEndpointRequest {
    /// <p>The Amazon Resource Number (ARN) of the endpoint being deleted.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEndpointResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEntityRecognizerRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
    #[serde(rename = "EntityRecognizerArn")]
    pub entity_recognizer_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEntityRecognizerResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDocumentClassificationJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDocumentClassificationJobResponse {
    /// <p>An object that describes the properties associated with the document classification job.</p>
    #[serde(rename = "DocumentClassificationJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classification_job_properties: Option<DocumentClassificationJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDocumentClassifierRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier. The operation returns this identifier in its response.</p>
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDocumentClassifierResponse {
    /// <p>An object that contains the properties associated with a document classifier.</p>
    #[serde(rename = "DocumentClassifierProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_properties: Option<DocumentClassifierProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeDominantLanguageDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDominantLanguageDetectionJobResponse {
    /// <p>An object that contains the properties associated with a dominant language detection job.</p>
    #[serde(rename = "DominantLanguageDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_language_detection_job_properties: Option<DominantLanguageDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointRequest {
    /// <p>The Amazon Resource Number (ARN) of the endpoint being described.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointResponse {
    /// <p>Describes information associated with the specific endpoint.</p>
    #[serde(rename = "EndpointProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_properties: Option<EndpointProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEntitiesDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEntitiesDetectionJobResponse {
    /// <p>An object that contains the properties associated with an entities detection job.</p>
    #[serde(rename = "EntitiesDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_detection_job_properties: Option<EntitiesDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEntityRecognizerRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
    #[serde(rename = "EntityRecognizerArn")]
    pub entity_recognizer_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEntityRecognizerResponse {
    /// <p>Describes information associated with an entity recognizer.</p>
    #[serde(rename = "EntityRecognizerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_properties: Option<EntityRecognizerProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeKeyPhrasesDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeKeyPhrasesDetectionJobResponse {
    /// <p>An object that contains the properties associated with a key phrases detection job. </p>
    #[serde(rename = "KeyPhrasesDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases_detection_job_properties: Option<KeyPhrasesDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSentimentDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSentimentDetectionJobResponse {
    /// <p>An object that contains the properties associated with a sentiment detection job.</p>
    #[serde(rename = "SentimentDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_detection_job_properties: Option<SentimentDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTopicsDetectionJobRequest {
    /// <p>The identifier assigned by the user to the detection job.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTopicsDetectionJobResponse {
    /// <p>The list of properties for the requested job.</p>
    #[serde(rename = "TopicsDetectionJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics_detection_job_properties: Option<TopicsDetectionJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectDominantLanguageRequest {
    /// <p>A UTF-8 text string. Each string should contain at least 20 characters and must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectDominantLanguageResponse {
    /// <p>The languages that Amazon Comprehend detected in the input text. For each language, the response returns the RFC 5646 language code and the level of confidence that Amazon Comprehend has in the accuracy of its inference. For more information about RFC 5646, see <a href="https://tools.ietf.org/html/rfc5646">Tags for Identifying Languages</a> on the <i>IETF Tools</i> web site.</p>
    #[serde(rename = "Languages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<DominantLanguage>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectEntitiesRequest {
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A UTF-8 text string. Each string must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectEntitiesResponse {
    /// <p>A collection of entities identified in the input text. For each entity, the response provides the entity text, entity type, where the entity text begins and ends, and the level of confidence that Amazon Comprehend has in the detection. For a list of entity types, see <a>how-entities</a>. </p>
    #[serde(rename = "Entities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectKeyPhrasesRequest {
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A UTF-8 text string. Each string must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectKeyPhrasesResponse {
    /// <p>A collection of key phrases that Amazon Comprehend identified in the input text. For each key phrase, the response provides the text of the key phrase, where the key phrase begins and ends, and the level of confidence that Amazon Comprehend has in the accuracy of the detection. </p>
    #[serde(rename = "KeyPhrases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases: Option<Vec<KeyPhrase>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectSentimentRequest {
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A UTF-8 text string. Each string must contain fewer that 5,000 bytes of UTF-8 encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectSyntaxRequest {
    /// <p>The language code of the input documents. You can specify any of the following languages supported by Amazon Comprehend: German ("de"), English ("en"), Spanish ("es"), French ("fr"), Italian ("it"), or Portuguese ("pt").</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>A UTF-8 string. Each string must contain fewer that 5,000 bytes of UTF encoded characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectSyntaxResponse {
    /// <p>A collection of syntax tokens describing the text. For each token, the response provides the text, the token type, where the text begins and ends, and the level of confidence that Amazon Comprehend has that the token is correct. For a list of token types, see <a>how-syntax</a>.</p>
    #[serde(rename = "SyntaxTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_tokens: Option<Vec<SyntaxToken>>,
}

/// <p>Specifies the class that categorizes the document being analyzed</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentClass {
    /// <p>The name of the class.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The confidence score that Amazon Comprehend has this class correctly attributed.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

/// <p>Provides information for filtering a list of document classification jobs. For more information, see the operation. You can provide only one filter parameter in each request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p> Configuration parameters for a private Virtual Private Cloud (VPC) containing the resources you are using for your document classification job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Provides information for filtering a list of document classifiers. You can only specify one filtering parameter in a request. For more information, see the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>Indicates the delimiter used to separate each label for training a multi-label classifier. The default delimiter between labels is a pipe (|). You can use a different character as a delimiter (if it's an allowed character) by specifying it under Delimiter for labels. If the training documents use a delimiter other than the default or the delimiter you specify, the labels on that line will be combined to make a single unique label, such as LABELLABELLABEL.</p>
    #[serde(rename = "LabelDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_delimiter: Option<String>,
    /// <p>The Amazon S3 URI for the input data. The S3 bucket must be in the same region as the API endpoint that you are calling. The URI can point to a single input file or it can provide the prefix for a collection of input files.</p> <p>For example, if you use the URI <code>S3://bucketName/prefix</code>, if the prefix is a single file, Amazon Comprehend uses that file as input. If more than one file begins with the prefix, Amazon Comprehend uses all of them as input.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Provides output results configuration parameters for custom classifier jobs. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentClassifierOutputDataConfig {
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt the output results from an analysis job. The KmsKeyId can be one of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>KMS Key Alias: <code>&quot;alias/ExampleAlias&quot;</code> </p> </li> <li> <p>ARN of a KMS Key Alias: <code>&quot;arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>When you use the <code>OutputDataConfig</code> object while creating a custom classifier, you specify the Amazon S3 location where you want to write the confusion matrix. The URI must be in the same region as the API endpoint that you are calling. The location is used as the prefix for the actual location of this output file.</p> <p>When the custom classifier job is finished, the service creates the output file in a directory specific to the job. The <code>S3Uri</code> field contains the location of the output file, called <code>output.tar.gz</code>. It is a compressed archive that contains the confusion matrix.</p>
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

/// <p>Provides information about a document classifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Indicates the mode in which the specific classifier was trained. This also indicates the format of input documents and the format of the confusion matrix. Each classifier can only be trained in one mode and this cannot be changed once the classifier is trained.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p> Provides output results configuration parameters for custom classifier jobs.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<DocumentClassifierOutputDataConfig>,
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p> Configuration parameters for a private Virtual Private Cloud (VPC) containing the resources you are using for your custom classifier. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Specifies one of the label or labels that categorize the document being analyzed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentLabel {
    /// <p>The name of the label.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The confidence score that Amazon Comprehend has this label correctly attributed.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

/// <p>Returns the code for the dominant language in the input text and the level of confidence that Amazon Comprehend has in the accuracy of the detection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p> Configuration parameters for a private Virtual Private Cloud (VPC) containing the resources you are using for your dominant language detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>The filter used to determine which endpoints are are returned. You can filter jobs on their name, model, status, or the date and time that they were created. You can only set one filter at a time. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EndpointFilter {
    /// <p>Specifies a date after which the returned endpoint or endpoints were created.</p>
    #[serde(rename = "CreationTimeAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    /// <p>Specifies a date before which the returned endpoint or endpoints were created.</p>
    #[serde(rename = "CreationTimeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    /// <p>The Amazon Resource Number (ARN) of the model to which the endpoint is attached.</p>
    #[serde(rename = "ModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    /// <p>Specifies the status of the endpoint being returned. Possible values are: Creating, Ready, Updating, Deleting, Failed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Specifies information about the specified endpoint.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EndpointProperties {
    /// <p>The creation date and time of the endpoint.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The number of inference units currently used by the model using this endpoint.</p>
    #[serde(rename = "CurrentInferenceUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_inference_units: Option<i64>,
    /// <p>The desired number of inference units to be used by the model using this endpoint. Each inference unit represents of a throughput of 100 characters per second.</p>
    #[serde(rename = "DesiredInferenceUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_inference_units: Option<i64>,
    /// <p>The Amazon Resource Number (ARN) of the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    /// <p>The date and time that the endpoint was last modified.</p>
    #[serde(rename = "LastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    /// <p>Specifies a reason for failure in cases of <code>Failed</code> status.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The Amazon Resource Number (ARN) of the model to which the endpoint is attached.</p>
    #[serde(rename = "ModelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    /// <p>Specifies the status of the endpoint. Because the endpoint updates and creation are asynchronous, so customers will need to wait for the endpoint to be <code>Ready</code> status before making inference requests.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Provides information for filtering a list of dominant language detection jobs. For more information, see the operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p> Configuration parameters for a private Virtual Private Cloud (VPC) containing the resources you are using for your entity detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Provides information about an entity. </p> <p> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>Detailed information about the accuracy of an entity recognizer. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The entity types in the input data for an entity recognizer. A maximum of 12 entity types can be used at one time to train an entity recognizer.</p>
    #[serde(rename = "EntityTypes")]
    pub entity_types: Vec<EntityTypesListItem>,
}

/// <p>Detailed information about an entity recognizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntityRecognizerMetadata {
    /// <p>Entity types from the metadata of an entity recognizer.</p>
    #[serde(rename = "EntityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<EntityRecognizerMetadataEntityTypesListItem>>,
    /// <p>Detailed information about the accuracy of an entity recognizer.</p>
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntityRecognizerMetadataEntityTypesListItem {
    /// <p>Detailed information about the accuracy of the entity recognizer for a specific item on the list of entity types. </p>
    #[serde(rename = "EvaluationMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<EntityTypesEvaluationMetrics>,
    /// <p>Indicates the number of times the given entity type was seen in the training data. </p>
    #[serde(rename = "NumberOfTrainMentions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_train_mentions: Option<i64>,
    /// <p>Type of entity from the list of entity types in the metadata of an entity recognizer. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Describes information about an entity recognizer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p> Configuration parameters for a private Virtual Private Cloud (VPC) containing the resources you are using for your custom entity recognizer. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Detailed information about the accuracy of an entity recognizer for a specific entity type. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntityTypesEvaluationMetrics {
    /// <p>A measure of how accurate the recognizer results are for for a specific entity type in the test data. It is derived from the <code>Precision</code> and <code>Recall</code> values. The <code>F1Score</code> is the harmonic average of the two scores. The highest score is 1, and the worst score is 0. </p>
    #[serde(rename = "F1Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    /// <p>A measure of the usefulness of the recognizer results for a specific entity type in the test data. High precision means that the recognizer returned substantially more relevant results than irrelevant ones. </p>
    #[serde(rename = "Precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    /// <p>A measure of how complete the recognizer results are for a specific entity type in the test data. High recall means that the recognizer returned most of the relevant results.</p>
    #[serde(rename = "Recall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p> Configuration parameters for a private Virtual Private Cloud (VPC) containing the resources you are using for your key phrases detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEndpointsRequest {
    /// <p>Filters the endpoints that are returned. You can filter endpoints on their name, model, status, or the date and time that they were created. You can only set one filter at a time. </p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EndpointFilter>,
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEndpointsResponse {
    /// <p>Displays a list of endpoint properties being retrieved by the service in response to the request.</p>
    #[serde(rename = "EndpointPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_properties_list: Option<Vec<EndpointProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the given Amazon Comprehend resource you are querying. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The Amazon Resource Name (ARN) of the given Amazon Comprehend resource you are querying.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>Tags associated with the Amazon Comprehend resource being queried. A tag is a key-value pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with "Sales" as the key might be added to a resource to indicate its use by the sales department. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt the output results from an analysis job. The KmsKeyId can be one of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>KMS Key Alias: <code>&quot;alias/ExampleAlias&quot;</code> </p> </li> <li> <p>ARN of a KMS Key Alias: <code>&quot;arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>When you use the <code>OutputDataConfig</code> object with asynchronous operations, you specify the Amazon S3 location where you want to write the output data. The URI must be in the same region as the API endpoint that you are calling. The location is used as the prefix for the actual location of the output file.</p> <p>When the topic detection job is finished, the service creates an output file in a directory specific to the job. The <code>S3Uri</code> field contains the location of the output file, called <code>output.tar.gz</code>. It is a compressed archive that contains the ouput of the operation.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Identifies the part of speech represented by the token and gives the confidence that Amazon Comprehend has that the part of speech was correctly identified. For more information about the parts of speech that Amazon Comprehend can identify, see <a>how-syntax</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p> Configuration parameters for a private Virtual Private Cloud (VPC) containing the resources you are using for your sentiment detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>Describes the level of confidence that Amazon Comprehend has in the accuracy of its detection of sentiments.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your document classification job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your dominant language detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The language of the input documents. All documents must be in the same language. You can specify any of the languages supported by Amazon Comprehend. If custom entities recognition is used, this parameter is ignored and the language used for training the model is used instead.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your entity detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p> Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your key phrases detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files. </p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your sentiment detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for your topic detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopDominantLanguageDetectionJobRequest {
    /// <p>The identifier of the dominant language detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopEntitiesDetectionJobRequest {
    /// <p>The identifier of the entities detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopKeyPhrasesDetectionJobRequest {
    /// <p>The identifier of the key phrases detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopSentimentDetectionJobRequest {
    /// <p>The identifier of the sentiment detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopTrainingDocumentClassifierRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier currently being trained.</p>
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopTrainingDocumentClassifierResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopTrainingEntityRecognizerRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the entity recognizer currently being trained.</p>
    #[serde(rename = "EntityRecognizerArn")]
    pub entity_recognizer_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopTrainingEntityRecognizerResponse {}

/// <p>Represents a work in the input text that was recognized and assigned a part of speech. There is one syntax token record for each word in the source text.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>A key-value pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with the key-value pair ‘Department’:’Sales’ might be added to a resource to indicate its use by a particular department. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The initial part of a key-value pair that forms a tag associated with a given resource. For instance, if you want to show which resources are used by which departments, you might use “Department” as the key portion of the pair, with multiple possible values such as “sales,” “legal,” and “administration.” </p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p> The second part of a key-value pair that forms a tag associated with a given resource. For instance, if you want to show which resources are used by which departments, you might use “Department” as the initial (key) portion of the pair, with a value of “sales” to indicate the sales department. </p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the given Amazon Comprehend resource to which you want to associate the tags. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Tags being associated with a specific Amazon Comprehend resource. There can be a maximum of 50 tags (both existing and pending) associated with a specific resource. </p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Provides information for filtering topic detection jobs. For more information, see .</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TopicsDetectionJobProperties {
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Management (IAM) role that grants Amazon Comprehend read access to your job data. </p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
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
    /// <p><p>ID for the AWS Key Management Service (KMS) key that Amazon Comprehend uses to encrypt data on the storage volume attached to the ML compute instance(s) that process the analysis job. The VolumeKmsKeyId can be either of the following formats:</p> <ul> <li> <p>KMS Key ID: <code>&quot;1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> <li> <p>Amazon Resource Name (ARN) of a KMS Key: <code>&quot;arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    /// <p>Configuration parameters for a private Virtual Private Cloud (VPC) containing the resources you are using for your topic detection job. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p> The Amazon Resource Name (ARN) of the given Amazon Comprehend resource from which you want to remove the tags. </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The initial part of a key-value pair that forms a tag being removed from a given resource. For example, a tag with "Sales" as the key might be added to a resource to indicate its use by the sales department. Keys must be unique and cannot be duplicated for a particular resource. </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEndpointRequest {
    /// <p> The desired number of inference units to be used by the model using this endpoint. Each inference unit represents of a throughput of 100 characters per second.</p>
    #[serde(rename = "DesiredInferenceUnits")]
    pub desired_inference_units: i64,
    /// <p>The Amazon Resource Number (ARN) of the endpoint being updated.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateEndpointResponse {}

/// <p> Configuration parameters for an optional private Virtual Private Cloud (VPC) containing the resources you are using for the job. For For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/what-is-amazon-vpc.html">Amazon VPC</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VpcConfig {
    /// <p>The ID number for a security group on an instance of your private VPC. Security groups on your VPC function serve as a virtual firewall to control inbound and outbound traffic and provides security for the resources that you’ll be accessing on the VPC. This ID number is preceded by "sg-", for instance: "sg-03b388029b0a285ea". For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_SecurityGroups.html">Security Groups for your VPC</a>. </p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The ID for each subnet being used in your private VPC. This subnet is a subset of the a range of IPv4 addresses used by the VPC and is specific to a given availability zone in the VPC’s region. This ID number is preceded by "subnet-", for instance: "subnet-04ccf456919e69055". For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">VPCs and Subnets</a>. </p>
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
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
}

impl BatchDetectDominantLanguageError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchDetectDominantLanguageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BatchSizeLimitExceededException" => {
                    return RusotoError::Service(
                        BatchDetectDominantLanguageError::BatchSizeLimitExceeded(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(BatchDetectDominantLanguageError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchDetectDominantLanguageError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(
                        BatchDetectDominantLanguageError::TextSizeLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchDetectDominantLanguageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDetectDominantLanguageError::BatchSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            BatchDetectDominantLanguageError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchDetectDominantLanguageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchDetectDominantLanguageError::TextSizeLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchDetectDominantLanguageError {}
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl BatchDetectEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDetectEntitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BatchSizeLimitExceededException" => {
                    return RusotoError::Service(BatchDetectEntitiesError::BatchSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(BatchDetectEntitiesError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchDetectEntitiesError::InvalidRequest(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(BatchDetectEntitiesError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(BatchDetectEntitiesError::UnsupportedLanguage(
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
impl fmt::Display for BatchDetectEntitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDetectEntitiesError::BatchSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchDetectEntitiesError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchDetectEntitiesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchDetectEntitiesError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchDetectEntitiesError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDetectEntitiesError {}
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl BatchDetectKeyPhrasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDetectKeyPhrasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BatchSizeLimitExceededException" => {
                    return RusotoError::Service(
                        BatchDetectKeyPhrasesError::BatchSizeLimitExceeded(err.msg),
                    )
                }
                "InternalServerException" => {
                    return RusotoError::Service(BatchDetectKeyPhrasesError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchDetectKeyPhrasesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(BatchDetectKeyPhrasesError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(BatchDetectKeyPhrasesError::UnsupportedLanguage(
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
impl fmt::Display for BatchDetectKeyPhrasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDetectKeyPhrasesError::BatchSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchDetectKeyPhrasesError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchDetectKeyPhrasesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchDetectKeyPhrasesError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchDetectKeyPhrasesError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDetectKeyPhrasesError {}
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl BatchDetectSentimentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDetectSentimentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BatchSizeLimitExceededException" => {
                    return RusotoError::Service(BatchDetectSentimentError::BatchSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(BatchDetectSentimentError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchDetectSentimentError::InvalidRequest(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(BatchDetectSentimentError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(BatchDetectSentimentError::UnsupportedLanguage(
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
impl fmt::Display for BatchDetectSentimentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDetectSentimentError::BatchSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchDetectSentimentError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchDetectSentimentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchDetectSentimentError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchDetectSentimentError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDetectSentimentError {}
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
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl BatchDetectSyntaxError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDetectSyntaxError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BatchSizeLimitExceededException" => {
                    return RusotoError::Service(BatchDetectSyntaxError::BatchSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(BatchDetectSyntaxError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchDetectSyntaxError::InvalidRequest(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(BatchDetectSyntaxError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(BatchDetectSyntaxError::UnsupportedLanguage(
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
impl fmt::Display for BatchDetectSyntaxError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDetectSyntaxError::BatchSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchDetectSyntaxError::InternalServer(ref cause) => write!(f, "{}", cause),
            BatchDetectSyntaxError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            BatchDetectSyntaxError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            BatchDetectSyntaxError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDetectSyntaxError {}
/// Errors returned by ClassifyDocument
#[derive(Debug, PartialEq)]
pub enum ClassifyDocumentError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
}

impl ClassifyDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ClassifyDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ClassifyDocumentError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ClassifyDocumentError::InvalidRequest(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(ClassifyDocumentError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(ClassifyDocumentError::TextSizeLimitExceeded(
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
impl fmt::Display for ClassifyDocumentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClassifyDocumentError::InternalServer(ref cause) => write!(f, "{}", cause),
            ClassifyDocumentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ClassifyDocumentError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            ClassifyDocumentError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ClassifyDocumentError {}
/// Errors returned by CreateDocumentClassifier
#[derive(Debug, PartialEq)]
pub enum CreateDocumentClassifierError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it.</p>
    KmsKeyValidation(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The maximum number of recognizers per account has been exceeded. Review the recognizers, perform cleanup, and then try your request again.</p>
    ResourceLimitExceeded(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
    /// <p>The request contains more tags than can be associated with a resource (50 tags per resource). The maximum number of tags includes both existing tags and those included in your current request. </p>
    TooManyTags(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl CreateDocumentClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDocumentClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateDocumentClassifierError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateDocumentClassifierError::InvalidRequest(
                        err.msg,
                    ))
                }
                "KmsKeyValidationException" => {
                    return RusotoError::Service(CreateDocumentClassifierError::KmsKeyValidation(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateDocumentClassifierError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        CreateDocumentClassifierError::ResourceLimitExceeded(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDocumentClassifierError::TooManyRequests(
                        err.msg,
                    ))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateDocumentClassifierError::TooManyTags(
                        err.msg,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(
                        CreateDocumentClassifierError::UnsupportedLanguage(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDocumentClassifierError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDocumentClassifierError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateDocumentClassifierError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateDocumentClassifierError::KmsKeyValidation(ref cause) => write!(f, "{}", cause),
            CreateDocumentClassifierError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateDocumentClassifierError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDocumentClassifierError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateDocumentClassifierError::TooManyTags(ref cause) => write!(f, "{}", cause),
            CreateDocumentClassifierError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDocumentClassifierError {}
/// Errors returned by CreateEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateEndpointError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The maximum number of recognizers per account has been exceeded. Review the recognizers, perform cleanup, and then try your request again.</p>
    ResourceLimitExceeded(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
    /// <p>The request contains more tags than can be associated with a resource (50 tags per resource). The maximum number of tags includes both existing tags and those included in your current request. </p>
    TooManyTags(String),
}

impl CreateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateEndpointError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateEndpointError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateEndpointError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(CreateEndpointError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateEndpointError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(CreateEndpointError::ResourceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEndpointError::TooManyRequests(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateEndpointError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEndpointError {}
/// Errors returned by CreateEntityRecognizer
#[derive(Debug, PartialEq)]
pub enum CreateEntityRecognizerError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it.</p>
    KmsKeyValidation(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The maximum number of recognizers per account has been exceeded. Review the recognizers, perform cleanup, and then try your request again.</p>
    ResourceLimitExceeded(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
    /// <p>The request contains more tags than can be associated with a resource (50 tags per resource). The maximum number of tags includes both existing tags and those included in your current request. </p>
    TooManyTags(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl CreateEntityRecognizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEntityRecognizerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateEntityRecognizerError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateEntityRecognizerError::InvalidRequest(
                        err.msg,
                    ))
                }
                "KmsKeyValidationException" => {
                    return RusotoError::Service(CreateEntityRecognizerError::KmsKeyValidation(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateEntityRecognizerError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        CreateEntityRecognizerError::ResourceLimitExceeded(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEntityRecognizerError::TooManyRequests(
                        err.msg,
                    ))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateEntityRecognizerError::TooManyTags(err.msg))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(CreateEntityRecognizerError::UnsupportedLanguage(
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
impl fmt::Display for CreateEntityRecognizerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEntityRecognizerError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateEntityRecognizerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateEntityRecognizerError::KmsKeyValidation(ref cause) => write!(f, "{}", cause),
            CreateEntityRecognizerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateEntityRecognizerError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEntityRecognizerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateEntityRecognizerError::TooManyTags(ref cause) => write!(f, "{}", cause),
            CreateEntityRecognizerError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEntityRecognizerError {}
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
}

impl DeleteDocumentClassifierError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDocumentClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteDocumentClassifierError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteDocumentClassifierError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteDocumentClassifierError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDocumentClassifierError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        DeleteDocumentClassifierError::ResourceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDocumentClassifierError::TooManyRequests(
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
impl fmt::Display for DeleteDocumentClassifierError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDocumentClassifierError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteDocumentClassifierError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteDocumentClassifierError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteDocumentClassifierError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDocumentClassifierError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteDocumentClassifierError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDocumentClassifierError {}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl DeleteEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteEndpointError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteEndpointError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteEndpointError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteEndpointError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEndpointError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEndpointError {}
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
}

impl DeleteEntityRecognizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEntityRecognizerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteEntityRecognizerError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteEntityRecognizerError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteEntityRecognizerError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteEntityRecognizerError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(DeleteEntityRecognizerError::ResourceUnavailable(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEntityRecognizerError::TooManyRequests(
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
impl fmt::Display for DeleteEntityRecognizerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEntityRecognizerError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteEntityRecognizerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteEntityRecognizerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteEntityRecognizerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteEntityRecognizerError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            DeleteEntityRecognizerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEntityRecognizerError {}
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
}

impl DescribeDocumentClassificationJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDocumentClassificationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeDocumentClassificationJobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeDocumentClassificationJobError::InvalidRequest(err.msg),
                    )
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(
                        DescribeDocumentClassificationJobError::JobNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeDocumentClassificationJobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDocumentClassificationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDocumentClassificationJobError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDocumentClassificationJobError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDocumentClassificationJobError::JobNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDocumentClassificationJobError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDocumentClassificationJobError {}
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
}

impl DescribeDocumentClassifierError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDocumentClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeDocumentClassifierError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDocumentClassifierError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDocumentClassifierError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeDocumentClassifierError::TooManyRequests(
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
impl fmt::Display for DescribeDocumentClassifierError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDocumentClassifierError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeDocumentClassifierError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeDocumentClassifierError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDocumentClassifierError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDocumentClassifierError {}
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
}

impl DescribeDominantLanguageDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDominantLanguageDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeDominantLanguageDetectionJobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeDominantLanguageDetectionJobError::InvalidRequest(err.msg),
                    )
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(
                        DescribeDominantLanguageDetectionJobError::JobNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeDominantLanguageDetectionJobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDominantLanguageDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDominantLanguageDetectionJobError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDominantLanguageDetectionJobError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDominantLanguageDetectionJobError::JobNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeDominantLanguageDetectionJobError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeDominantLanguageDetectionJobError {}
/// Errors returned by DescribeEndpoint
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl DescribeEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeEndpointError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeEndpointError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEndpointError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeEndpointError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeEndpointError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEndpointError {}
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
}

impl DescribeEntitiesDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEntitiesDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeEntitiesDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeEntitiesDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(DescribeEntitiesDetectionJobError::JobNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionJobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEntitiesDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEntitiesDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeEntitiesDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeEntitiesDetectionJobError::JobNotFound(ref cause) => write!(f, "{}", cause),
            DescribeEntitiesDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEntitiesDetectionJobError {}
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
}

impl DescribeEntityRecognizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEntityRecognizerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeEntityRecognizerError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeEntityRecognizerError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeEntityRecognizerError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeEntityRecognizerError::TooManyRequests(
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
impl fmt::Display for DescribeEntityRecognizerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEntityRecognizerError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeEntityRecognizerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeEntityRecognizerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeEntityRecognizerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEntityRecognizerError {}
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
}

impl DescribeKeyPhrasesDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeKeyPhrasesDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeKeyPhrasesDetectionJobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeKeyPhrasesDetectionJobError::InvalidRequest(err.msg),
                    )
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(DescribeKeyPhrasesDetectionJobError::JobNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeKeyPhrasesDetectionJobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeKeyPhrasesDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeKeyPhrasesDetectionJobError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeKeyPhrasesDetectionJobError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeKeyPhrasesDetectionJobError::JobNotFound(ref cause) => write!(f, "{}", cause),
            DescribeKeyPhrasesDetectionJobError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeKeyPhrasesDetectionJobError {}
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
}

impl DescribeSentimentDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeSentimentDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeSentimentDetectionJobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeSentimentDetectionJobError::InvalidRequest(err.msg),
                    )
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(DescribeSentimentDetectionJobError::JobNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeSentimentDetectionJobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeSentimentDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSentimentDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeSentimentDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeSentimentDetectionJobError::JobNotFound(ref cause) => write!(f, "{}", cause),
            DescribeSentimentDetectionJobError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeSentimentDetectionJobError {}
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
}

impl DescribeTopicsDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTopicsDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeTopicsDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeTopicsDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(DescribeTopicsDetectionJobError::JobNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeTopicsDetectionJobError::TooManyRequests(
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
impl fmt::Display for DescribeTopicsDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTopicsDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeTopicsDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeTopicsDetectionJobError::JobNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTopicsDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTopicsDetectionJobError {}
/// Errors returned by DetectDominantLanguage
#[derive(Debug, PartialEq)]
pub enum DetectDominantLanguageError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
}

impl DetectDominantLanguageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectDominantLanguageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectDominantLanguageError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectDominantLanguageError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(
                        DetectDominantLanguageError::TextSizeLimitExceeded(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectDominantLanguageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectDominantLanguageError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectDominantLanguageError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectDominantLanguageError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectDominantLanguageError {}
/// Errors returned by DetectEntities
#[derive(Debug, PartialEq)]
pub enum DetectEntitiesError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl DetectEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectEntitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectEntitiesError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectEntitiesError::InvalidRequest(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectEntitiesError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(DetectEntitiesError::UnsupportedLanguage(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectEntitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectEntitiesError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectEntitiesError {}
/// Errors returned by DetectKeyPhrases
#[derive(Debug, PartialEq)]
pub enum DetectKeyPhrasesError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl DetectKeyPhrasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectKeyPhrasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectKeyPhrasesError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectKeyPhrasesError::InvalidRequest(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectKeyPhrasesError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(DetectKeyPhrasesError::UnsupportedLanguage(
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
impl fmt::Display for DetectKeyPhrasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectKeyPhrasesError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectKeyPhrasesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectKeyPhrasesError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectKeyPhrasesError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectKeyPhrasesError {}
/// Errors returned by DetectSentiment
#[derive(Debug, PartialEq)]
pub enum DetectSentimentError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl DetectSentimentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectSentimentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectSentimentError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectSentimentError::InvalidRequest(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectSentimentError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(DetectSentimentError::UnsupportedLanguage(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectSentimentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectSentimentError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectSentimentError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectSentimentError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectSentimentError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectSentimentError {}
/// Errors returned by DetectSyntax
#[derive(Debug, PartialEq)]
pub enum DetectSyntaxError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The size of the input text exceeds the limit. Use a smaller document.</p>
    TextSizeLimitExceeded(String),
    /// <p>Amazon Comprehend can't process the language of the input text. For all custom entity recognition APIs (such as <code>CreateEntityRecognizer</code>), only English is accepted. For most other APIs, such as those for Custom Classification, Amazon Comprehend accepts text in all supported languages. For a list of supported languages, see <a>supported-languages</a>. </p>
    UnsupportedLanguage(String),
}

impl DetectSyntaxError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectSyntaxError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectSyntaxError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectSyntaxError::InvalidRequest(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectSyntaxError::TextSizeLimitExceeded(err.msg))
                }
                "UnsupportedLanguageException" => {
                    return RusotoError::Service(DetectSyntaxError::UnsupportedLanguage(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectSyntaxError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectSyntaxError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectSyntaxError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectSyntaxError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectSyntaxError::UnsupportedLanguage(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectSyntaxError {}
/// Errors returned by ListDocumentClassificationJobs
#[derive(Debug, PartialEq)]
pub enum ListDocumentClassificationJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListDocumentClassificationJobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDocumentClassificationJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        ListDocumentClassificationJobsError::InternalServer(err.msg),
                    )
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(
                        ListDocumentClassificationJobsError::InvalidFilter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        ListDocumentClassificationJobsError::InvalidRequest(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListDocumentClassificationJobsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDocumentClassificationJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDocumentClassificationJobsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDocumentClassificationJobsError::InvalidFilter(ref cause) => write!(f, "{}", cause),
            ListDocumentClassificationJobsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDocumentClassificationJobsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDocumentClassificationJobsError {}
/// Errors returned by ListDocumentClassifiers
#[derive(Debug, PartialEq)]
pub enum ListDocumentClassifiersError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListDocumentClassifiersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDocumentClassifiersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListDocumentClassifiersError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(ListDocumentClassifiersError::InvalidFilter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDocumentClassifiersError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDocumentClassifiersError::TooManyRequests(
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
impl fmt::Display for ListDocumentClassifiersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDocumentClassifiersError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListDocumentClassifiersError::InvalidFilter(ref cause) => write!(f, "{}", cause),
            ListDocumentClassifiersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListDocumentClassifiersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDocumentClassifiersError {}
/// Errors returned by ListDominantLanguageDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListDominantLanguageDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListDominantLanguageDetectionJobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDominantLanguageDetectionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        ListDominantLanguageDetectionJobsError::InternalServer(err.msg),
                    )
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(
                        ListDominantLanguageDetectionJobsError::InvalidFilter(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        ListDominantLanguageDetectionJobsError::InvalidRequest(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListDominantLanguageDetectionJobsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDominantLanguageDetectionJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDominantLanguageDetectionJobsError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDominantLanguageDetectionJobsError::InvalidFilter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDominantLanguageDetectionJobsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDominantLanguageDetectionJobsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDominantLanguageDetectionJobsError {}
/// Errors returned by ListEndpoints
#[derive(Debug, PartialEq)]
pub enum ListEndpointsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListEndpointsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListEndpointsError::InvalidRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEndpointsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEndpointsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListEndpointsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListEndpointsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEndpointsError {}
/// Errors returned by ListEntitiesDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListEntitiesDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListEntitiesDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEntitiesDetectionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListEntitiesDetectionJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(ListEntitiesDetectionJobsError::InvalidFilter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListEntitiesDetectionJobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEntitiesDetectionJobsError::TooManyRequests(
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
impl fmt::Display for ListEntitiesDetectionJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEntitiesDetectionJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListEntitiesDetectionJobsError::InvalidFilter(ref cause) => write!(f, "{}", cause),
            ListEntitiesDetectionJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListEntitiesDetectionJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEntitiesDetectionJobsError {}
/// Errors returned by ListEntityRecognizers
#[derive(Debug, PartialEq)]
pub enum ListEntityRecognizersError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListEntityRecognizersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEntityRecognizersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListEntityRecognizersError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(ListEntityRecognizersError::InvalidFilter(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListEntityRecognizersError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEntityRecognizersError::TooManyRequests(
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
impl fmt::Display for ListEntityRecognizersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEntityRecognizersError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListEntityRecognizersError::InvalidFilter(ref cause) => write!(f, "{}", cause),
            ListEntityRecognizersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListEntityRecognizersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEntityRecognizersError {}
/// Errors returned by ListKeyPhrasesDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListKeyPhrasesDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListKeyPhrasesDetectionJobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListKeyPhrasesDetectionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListKeyPhrasesDetectionJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(ListKeyPhrasesDetectionJobsError::InvalidFilter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListKeyPhrasesDetectionJobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListKeyPhrasesDetectionJobsError::TooManyRequests(
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
impl fmt::Display for ListKeyPhrasesDetectionJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListKeyPhrasesDetectionJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListKeyPhrasesDetectionJobsError::InvalidFilter(ref cause) => write!(f, "{}", cause),
            ListKeyPhrasesDetectionJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListKeyPhrasesDetectionJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListKeyPhrasesDetectionJobsError {}
/// Errors returned by ListSentimentDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListSentimentDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListSentimentDetectionJobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListSentimentDetectionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListSentimentDetectionJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(ListSentimentDetectionJobsError::InvalidFilter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListSentimentDetectionJobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListSentimentDetectionJobsError::TooManyRequests(
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
impl fmt::Display for ListSentimentDetectionJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSentimentDetectionJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListSentimentDetectionJobsError::InvalidFilter(ref cause) => write!(f, "{}", cause),
            ListSentimentDetectionJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListSentimentDetectionJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSentimentDetectionJobsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
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
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTopicsDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListTopicsDetectionJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl ListTopicsDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTopicsDetectionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTopicsDetectionJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(ListTopicsDetectionJobsError::InvalidFilter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTopicsDetectionJobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTopicsDetectionJobsError::TooManyRequests(
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
impl fmt::Display for ListTopicsDetectionJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTopicsDetectionJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTopicsDetectionJobsError::InvalidFilter(ref cause) => write!(f, "{}", cause),
            ListTopicsDetectionJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTopicsDetectionJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTopicsDetectionJobsError {}
/// Errors returned by StartDocumentClassificationJob
#[derive(Debug, PartialEq)]
pub enum StartDocumentClassificationJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it.</p>
    KmsKeyValidation(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl StartDocumentClassificationJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartDocumentClassificationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        StartDocumentClassificationJobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        StartDocumentClassificationJobError::InvalidRequest(err.msg),
                    )
                }
                "KmsKeyValidationException" => {
                    return RusotoError::Service(
                        StartDocumentClassificationJobError::KmsKeyValidation(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StartDocumentClassificationJobError::ResourceNotFound(err.msg),
                    )
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        StartDocumentClassificationJobError::ResourceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        StartDocumentClassificationJobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartDocumentClassificationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDocumentClassificationJobError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentClassificationJobError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentClassificationJobError::KmsKeyValidation(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentClassificationJobError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentClassificationJobError::ResourceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentClassificationJobError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartDocumentClassificationJobError {}
/// Errors returned by StartDominantLanguageDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartDominantLanguageDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it.</p>
    KmsKeyValidation(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl StartDominantLanguageDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartDominantLanguageDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        StartDominantLanguageDetectionJobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        StartDominantLanguageDetectionJobError::InvalidRequest(err.msg),
                    )
                }
                "KmsKeyValidationException" => {
                    return RusotoError::Service(
                        StartDominantLanguageDetectionJobError::KmsKeyValidation(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        StartDominantLanguageDetectionJobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartDominantLanguageDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDominantLanguageDetectionJobError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDominantLanguageDetectionJobError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDominantLanguageDetectionJobError::KmsKeyValidation(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDominantLanguageDetectionJobError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartDominantLanguageDetectionJobError {}
/// Errors returned by StartEntitiesDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartEntitiesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it.</p>
    KmsKeyValidation(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl StartEntitiesDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartEntitiesDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartEntitiesDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartEntitiesDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "KmsKeyValidationException" => {
                    return RusotoError::Service(StartEntitiesDetectionJobError::KmsKeyValidation(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartEntitiesDetectionJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(
                        StartEntitiesDetectionJobError::ResourceUnavailable(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartEntitiesDetectionJobError::TooManyRequests(
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
impl fmt::Display for StartEntitiesDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartEntitiesDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionJobError::KmsKeyValidation(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionJobError::ResourceUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            StartEntitiesDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartEntitiesDetectionJobError {}
/// Errors returned by StartKeyPhrasesDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartKeyPhrasesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it.</p>
    KmsKeyValidation(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl StartKeyPhrasesDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartKeyPhrasesDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartKeyPhrasesDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartKeyPhrasesDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "KmsKeyValidationException" => {
                    return RusotoError::Service(
                        StartKeyPhrasesDetectionJobError::KmsKeyValidation(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartKeyPhrasesDetectionJobError::TooManyRequests(
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
impl fmt::Display for StartKeyPhrasesDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartKeyPhrasesDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartKeyPhrasesDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartKeyPhrasesDetectionJobError::KmsKeyValidation(ref cause) => write!(f, "{}", cause),
            StartKeyPhrasesDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartKeyPhrasesDetectionJobError {}
/// Errors returned by StartSentimentDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartSentimentDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it.</p>
    KmsKeyValidation(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl StartSentimentDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartSentimentDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartSentimentDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartSentimentDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "KmsKeyValidationException" => {
                    return RusotoError::Service(StartSentimentDetectionJobError::KmsKeyValidation(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartSentimentDetectionJobError::TooManyRequests(
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
impl fmt::Display for StartSentimentDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartSentimentDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartSentimentDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartSentimentDetectionJobError::KmsKeyValidation(ref cause) => write!(f, "{}", cause),
            StartSentimentDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartSentimentDetectionJobError {}
/// Errors returned by StartTopicsDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartTopicsDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The KMS customer managed key (CMK) entered cannot be validated. Verify the key and re-enter it.</p>
    KmsKeyValidation(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl StartTopicsDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTopicsDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartTopicsDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartTopicsDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "KmsKeyValidationException" => {
                    return RusotoError::Service(StartTopicsDetectionJobError::KmsKeyValidation(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartTopicsDetectionJobError::TooManyRequests(
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
impl fmt::Display for StartTopicsDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartTopicsDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartTopicsDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartTopicsDetectionJobError::KmsKeyValidation(ref cause) => write!(f, "{}", cause),
            StartTopicsDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartTopicsDetectionJobError {}
/// Errors returned by StopDominantLanguageDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopDominantLanguageDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
}

impl StopDominantLanguageDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopDominantLanguageDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        StopDominantLanguageDetectionJobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        StopDominantLanguageDetectionJobError::InvalidRequest(err.msg),
                    )
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(
                        StopDominantLanguageDetectionJobError::JobNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopDominantLanguageDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopDominantLanguageDetectionJobError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            StopDominantLanguageDetectionJobError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            StopDominantLanguageDetectionJobError::JobNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopDominantLanguageDetectionJobError {}
/// Errors returned by StopEntitiesDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopEntitiesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
}

impl StopEntitiesDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopEntitiesDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopEntitiesDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopEntitiesDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(StopEntitiesDetectionJobError::JobNotFound(
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
impl fmt::Display for StopEntitiesDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopEntitiesDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopEntitiesDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopEntitiesDetectionJobError::JobNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopEntitiesDetectionJobError {}
/// Errors returned by StopKeyPhrasesDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopKeyPhrasesDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
}

impl StopKeyPhrasesDetectionJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopKeyPhrasesDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopKeyPhrasesDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopKeyPhrasesDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(StopKeyPhrasesDetectionJobError::JobNotFound(
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
impl fmt::Display for StopKeyPhrasesDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopKeyPhrasesDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopKeyPhrasesDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopKeyPhrasesDetectionJobError::JobNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopKeyPhrasesDetectionJobError {}
/// Errors returned by StopSentimentDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopSentimentDetectionJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified job was not found. Check the job ID and try again.</p>
    JobNotFound(String),
}

impl StopSentimentDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopSentimentDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopSentimentDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopSentimentDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "JobNotFoundException" => {
                    return RusotoError::Service(StopSentimentDetectionJobError::JobNotFound(
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
impl fmt::Display for StopSentimentDetectionJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopSentimentDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopSentimentDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopSentimentDetectionJobError::JobNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopSentimentDetectionJobError {}
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
}

impl StopTrainingDocumentClassifierError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopTrainingDocumentClassifierError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        StopTrainingDocumentClassifierError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        StopTrainingDocumentClassifierError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StopTrainingDocumentClassifierError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        StopTrainingDocumentClassifierError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopTrainingDocumentClassifierError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopTrainingDocumentClassifierError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            StopTrainingDocumentClassifierError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            StopTrainingDocumentClassifierError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            StopTrainingDocumentClassifierError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StopTrainingDocumentClassifierError {}
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
}

impl StopTrainingEntityRecognizerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopTrainingEntityRecognizerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopTrainingEntityRecognizerError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopTrainingEntityRecognizerError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StopTrainingEntityRecognizerError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        StopTrainingEntityRecognizerError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopTrainingEntityRecognizerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopTrainingEntityRecognizerError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopTrainingEntityRecognizerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopTrainingEntityRecognizerError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            StopTrainingEntityRecognizerError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopTrainingEntityRecognizerError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Concurrent modification of the tags associated with an Amazon Comprehend resource is not supported. </p>
    ConcurrentModification(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The request contains more tags than can be associated with a resource (50 tags per resource). The maximum number of tags includes both existing tags and those included in your current request. </p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Concurrent modification of the tags associated with an Amazon Comprehend resource is not supported. </p>
    ConcurrentModification(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The request contains more tag keys than can be associated with a resource (50 tag keys per resource).</p>
    TooManyTagKeys(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagKeysException" => {
                    return RusotoError::Service(UntagResourceError::TooManyTagKeys(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyTagKeys(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateEndpoint
#[derive(Debug, PartialEq)]
pub enum UpdateEndpointError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The request is invalid.</p>
    InvalidRequest(String),
    /// <p>The specified name is already in use. Use a different name and try your request again.</p>
    ResourceInUse(String),
    /// <p>The maximum number of recognizers per account has been exceeded. Review the recognizers, perform cleanup, and then try your request again.</p>
    ResourceLimitExceeded(String),
    /// <p>The specified resource ARN was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p>The specified resource is not available. Check to see if the resource is in the <code>TRAINED</code> state and try your request again.</p>
    ResourceUnavailable(String),
    /// <p>The number of requests exceeds the limit. Resubmit your request later.</p>
    TooManyRequests(String),
}

impl UpdateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateEndpointError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateEndpointError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateEndpointError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(UpdateEndpointError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateEndpointError::ResourceNotFound(err.msg))
                }
                "ResourceUnavailableException" => {
                    return RusotoError::Service(UpdateEndpointError::ResourceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEndpointError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::ResourceUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateEndpointError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEndpointError {}
/// Trait representing the capabilities of the Amazon Comprehend API. Amazon Comprehend clients implement this trait.
#[async_trait]
pub trait Comprehend {
    /// <p>Determines the dominant language of the input text for a batch of documents. For a list of languages that Amazon Comprehend can detect, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/how-languages.html">Amazon Comprehend Supported Languages</a>. </p>
    async fn batch_detect_dominant_language(
        &self,
        input: BatchDetectDominantLanguageRequest,
    ) -> Result<BatchDetectDominantLanguageResponse, RusotoError<BatchDetectDominantLanguageError>>;

    /// <p>Inspects the text of a batch of documents for named entities and returns information about them. For more information about named entities, see <a>how-entities</a> </p>
    async fn batch_detect_entities(
        &self,
        input: BatchDetectEntitiesRequest,
    ) -> Result<BatchDetectEntitiesResponse, RusotoError<BatchDetectEntitiesError>>;

    /// <p>Detects the key noun phrases found in a batch of documents.</p>
    async fn batch_detect_key_phrases(
        &self,
        input: BatchDetectKeyPhrasesRequest,
    ) -> Result<BatchDetectKeyPhrasesResponse, RusotoError<BatchDetectKeyPhrasesError>>;

    /// <p>Inspects a batch of documents and returns an inference of the prevailing sentiment, <code>POSITIVE</code>, <code>NEUTRAL</code>, <code>MIXED</code>, or <code>NEGATIVE</code>, in each one.</p>
    async fn batch_detect_sentiment(
        &self,
        input: BatchDetectSentimentRequest,
    ) -> Result<BatchDetectSentimentResponse, RusotoError<BatchDetectSentimentError>>;

    /// <p>Inspects the text of a batch of documents for the syntax and part of speech of the words in the document and returns information about them. For more information, see <a>how-syntax</a>.</p>
    async fn batch_detect_syntax(
        &self,
        input: BatchDetectSyntaxRequest,
    ) -> Result<BatchDetectSyntaxResponse, RusotoError<BatchDetectSyntaxError>>;

    /// <p>Creates a new document classification request to analyze a single document in real-time, using a previously created and trained custom model and an endpoint.</p>
    async fn classify_document(
        &self,
        input: ClassifyDocumentRequest,
    ) -> Result<ClassifyDocumentResponse, RusotoError<ClassifyDocumentError>>;

    /// <p>Creates a new document classifier that you can use to categorize documents. To create a classifier you provide a set of training documents that labeled with the categories that you want to use. After the classifier is trained you can use it to categorize a set of labeled documents into the categories. For more information, see <a>how-document-classification</a>.</p>
    async fn create_document_classifier(
        &self,
        input: CreateDocumentClassifierRequest,
    ) -> Result<CreateDocumentClassifierResponse, RusotoError<CreateDocumentClassifierError>>;

    /// <p>Creates a model-specific endpoint for synchronous inference for a previously trained custom model </p>
    async fn create_endpoint(
        &self,
        input: CreateEndpointRequest,
    ) -> Result<CreateEndpointResponse, RusotoError<CreateEndpointError>>;

    /// <p>Creates an entity recognizer using submitted files. After your <code>CreateEntityRecognizer</code> request is submitted, you can check job status using the API. </p>
    async fn create_entity_recognizer(
        &self,
        input: CreateEntityRecognizerRequest,
    ) -> Result<CreateEntityRecognizerResponse, RusotoError<CreateEntityRecognizerError>>;

    /// <p>Deletes a previously created document classifier</p> <p>Only those classifiers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p> <p>This is an asynchronous action that puts the classifier into a DELETING state, and it is then removed by a background job. Once removed, the classifier disappears from your account and is no longer available for use. </p>
    async fn delete_document_classifier(
        &self,
        input: DeleteDocumentClassifierRequest,
    ) -> Result<DeleteDocumentClassifierResponse, RusotoError<DeleteDocumentClassifierError>>;

    /// <p>Deletes a model-specific endpoint for a previously-trained custom model. All endpoints must be deleted in order for the model to be deleted.</p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointRequest,
    ) -> Result<DeleteEndpointResponse, RusotoError<DeleteEndpointError>>;

    /// <p>Deletes an entity recognizer.</p> <p>Only those recognizers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p> <p>This is an asynchronous action that puts the recognizer into a DELETING state, and it is then removed by a background job. Once removed, the recognizer disappears from your account and is no longer available for use. </p>
    async fn delete_entity_recognizer(
        &self,
        input: DeleteEntityRecognizerRequest,
    ) -> Result<DeleteEntityRecognizerResponse, RusotoError<DeleteEntityRecognizerError>>;

    /// <p>Gets the properties associated with a document classification job. Use this operation to get the status of a classification job.</p>
    async fn describe_document_classification_job(
        &self,
        input: DescribeDocumentClassificationJobRequest,
    ) -> Result<
        DescribeDocumentClassificationJobResponse,
        RusotoError<DescribeDocumentClassificationJobError>,
    >;

    /// <p>Gets the properties associated with a document classifier.</p>
    async fn describe_document_classifier(
        &self,
        input: DescribeDocumentClassifierRequest,
    ) -> Result<DescribeDocumentClassifierResponse, RusotoError<DescribeDocumentClassifierError>>;

    /// <p>Gets the properties associated with a dominant language detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_dominant_language_detection_job(
        &self,
        input: DescribeDominantLanguageDetectionJobRequest,
    ) -> Result<
        DescribeDominantLanguageDetectionJobResponse,
        RusotoError<DescribeDominantLanguageDetectionJobError>,
    >;

    /// <p>Gets the properties associated with a specific endpoint. Use this operation to get the status of an endpoint.</p>
    async fn describe_endpoint(
        &self,
        input: DescribeEndpointRequest,
    ) -> Result<DescribeEndpointResponse, RusotoError<DescribeEndpointError>>;

    /// <p>Gets the properties associated with an entities detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_entities_detection_job(
        &self,
        input: DescribeEntitiesDetectionJobRequest,
    ) -> Result<DescribeEntitiesDetectionJobResponse, RusotoError<DescribeEntitiesDetectionJobError>>;

    /// <p>Provides details about an entity recognizer including status, S3 buckets containing training data, recognizer metadata, metrics, and so on.</p>
    async fn describe_entity_recognizer(
        &self,
        input: DescribeEntityRecognizerRequest,
    ) -> Result<DescribeEntityRecognizerResponse, RusotoError<DescribeEntityRecognizerError>>;

    /// <p>Gets the properties associated with a key phrases detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_key_phrases_detection_job(
        &self,
        input: DescribeKeyPhrasesDetectionJobRequest,
    ) -> Result<
        DescribeKeyPhrasesDetectionJobResponse,
        RusotoError<DescribeKeyPhrasesDetectionJobError>,
    >;

    /// <p>Gets the properties associated with a sentiment detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_sentiment_detection_job(
        &self,
        input: DescribeSentimentDetectionJobRequest,
    ) -> Result<
        DescribeSentimentDetectionJobResponse,
        RusotoError<DescribeSentimentDetectionJobError>,
    >;

    /// <p>Gets the properties associated with a topic detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_topics_detection_job(
        &self,
        input: DescribeTopicsDetectionJobRequest,
    ) -> Result<DescribeTopicsDetectionJobResponse, RusotoError<DescribeTopicsDetectionJobError>>;

    /// <p>Determines the dominant language of the input text. For a list of languages that Amazon Comprehend can detect, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/how-languages.html">Amazon Comprehend Supported Languages</a>. </p>
    async fn detect_dominant_language(
        &self,
        input: DetectDominantLanguageRequest,
    ) -> Result<DetectDominantLanguageResponse, RusotoError<DetectDominantLanguageError>>;

    /// <p>Inspects text for named entities, and returns information about them. For more information, about named entities, see <a>how-entities</a>. </p>
    async fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> Result<DetectEntitiesResponse, RusotoError<DetectEntitiesError>>;

    /// <p>Detects the key noun phrases found in the text. </p>
    async fn detect_key_phrases(
        &self,
        input: DetectKeyPhrasesRequest,
    ) -> Result<DetectKeyPhrasesResponse, RusotoError<DetectKeyPhrasesError>>;

    /// <p>Inspects text and returns an inference of the prevailing sentiment (<code>POSITIVE</code>, <code>NEUTRAL</code>, <code>MIXED</code>, or <code>NEGATIVE</code>). </p>
    async fn detect_sentiment(
        &self,
        input: DetectSentimentRequest,
    ) -> Result<DetectSentimentResponse, RusotoError<DetectSentimentError>>;

    /// <p>Inspects text for syntax and the part of speech of words in the document. For more information, <a>how-syntax</a>.</p>
    async fn detect_syntax(
        &self,
        input: DetectSyntaxRequest,
    ) -> Result<DetectSyntaxResponse, RusotoError<DetectSyntaxError>>;

    /// <p>Gets a list of the documentation classification jobs that you have submitted.</p>
    async fn list_document_classification_jobs(
        &self,
        input: ListDocumentClassificationJobsRequest,
    ) -> Result<
        ListDocumentClassificationJobsResponse,
        RusotoError<ListDocumentClassificationJobsError>,
    >;

    /// <p>Gets a list of the document classifiers that you have created.</p>
    async fn list_document_classifiers(
        &self,
        input: ListDocumentClassifiersRequest,
    ) -> Result<ListDocumentClassifiersResponse, RusotoError<ListDocumentClassifiersError>>;

    /// <p>Gets a list of the dominant language detection jobs that you have submitted.</p>
    async fn list_dominant_language_detection_jobs(
        &self,
        input: ListDominantLanguageDetectionJobsRequest,
    ) -> Result<
        ListDominantLanguageDetectionJobsResponse,
        RusotoError<ListDominantLanguageDetectionJobsError>,
    >;

    /// <p>Gets a list of all existing endpoints that you've created.</p>
    async fn list_endpoints(
        &self,
        input: ListEndpointsRequest,
    ) -> Result<ListEndpointsResponse, RusotoError<ListEndpointsError>>;

    /// <p>Gets a list of the entity detection jobs that you have submitted.</p>
    async fn list_entities_detection_jobs(
        &self,
        input: ListEntitiesDetectionJobsRequest,
    ) -> Result<ListEntitiesDetectionJobsResponse, RusotoError<ListEntitiesDetectionJobsError>>;

    /// <p>Gets a list of the properties of all entity recognizers that you created, including recognizers currently in training. Allows you to filter the list of recognizers based on criteria such as status and submission time. This call returns up to 500 entity recognizers in the list, with a default number of 100 recognizers in the list.</p> <p>The results of this list are not in any particular order. Please get the list and sort locally if needed.</p>
    async fn list_entity_recognizers(
        &self,
        input: ListEntityRecognizersRequest,
    ) -> Result<ListEntityRecognizersResponse, RusotoError<ListEntityRecognizersError>>;

    /// <p>Get a list of key phrase detection jobs that you have submitted.</p>
    async fn list_key_phrases_detection_jobs(
        &self,
        input: ListKeyPhrasesDetectionJobsRequest,
    ) -> Result<ListKeyPhrasesDetectionJobsResponse, RusotoError<ListKeyPhrasesDetectionJobsError>>;

    /// <p>Gets a list of sentiment detection jobs that you have submitted.</p>
    async fn list_sentiment_detection_jobs(
        &self,
        input: ListSentimentDetectionJobsRequest,
    ) -> Result<ListSentimentDetectionJobsResponse, RusotoError<ListSentimentDetectionJobsError>>;

    /// <p>Lists all tags associated with a given Amazon Comprehend resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Gets a list of the topic detection jobs that you have submitted.</p>
    async fn list_topics_detection_jobs(
        &self,
        input: ListTopicsDetectionJobsRequest,
    ) -> Result<ListTopicsDetectionJobsResponse, RusotoError<ListTopicsDetectionJobsError>>;

    /// <p>Starts an asynchronous document classification job. Use the operation to track the progress of the job.</p>
    async fn start_document_classification_job(
        &self,
        input: StartDocumentClassificationJobRequest,
    ) -> Result<
        StartDocumentClassificationJobResponse,
        RusotoError<StartDocumentClassificationJobError>,
    >;

    /// <p>Starts an asynchronous dominant language detection job for a collection of documents. Use the operation to track the status of a job.</p>
    async fn start_dominant_language_detection_job(
        &self,
        input: StartDominantLanguageDetectionJobRequest,
    ) -> Result<
        StartDominantLanguageDetectionJobResponse,
        RusotoError<StartDominantLanguageDetectionJobError>,
    >;

    /// <p>Starts an asynchronous entity detection job for a collection of documents. Use the operation to track the status of a job.</p> <p>This API can be used for either standard entity detection or custom entity recognition. In order to be used for custom entity recognition, the optional <code>EntityRecognizerArn</code> must be used in order to provide access to the recognizer being used to detect the custom entity.</p>
    async fn start_entities_detection_job(
        &self,
        input: StartEntitiesDetectionJobRequest,
    ) -> Result<StartEntitiesDetectionJobResponse, RusotoError<StartEntitiesDetectionJobError>>;

    /// <p>Starts an asynchronous key phrase detection job for a collection of documents. Use the operation to track the status of a job.</p>
    async fn start_key_phrases_detection_job(
        &self,
        input: StartKeyPhrasesDetectionJobRequest,
    ) -> Result<StartKeyPhrasesDetectionJobResponse, RusotoError<StartKeyPhrasesDetectionJobError>>;

    /// <p>Starts an asynchronous sentiment detection job for a collection of documents. use the operation to track the status of a job.</p>
    async fn start_sentiment_detection_job(
        &self,
        input: StartSentimentDetectionJobRequest,
    ) -> Result<StartSentimentDetectionJobResponse, RusotoError<StartSentimentDetectionJobError>>;

    /// <p>Starts an asynchronous topic detection job. Use the <code>DescribeTopicDetectionJob</code> operation to track the status of a job.</p>
    async fn start_topics_detection_job(
        &self,
        input: StartTopicsDetectionJobRequest,
    ) -> Result<StartTopicsDetectionJobResponse, RusotoError<StartTopicsDetectionJobError>>;

    /// <p>Stops a dominant language detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    async fn stop_dominant_language_detection_job(
        &self,
        input: StopDominantLanguageDetectionJobRequest,
    ) -> Result<
        StopDominantLanguageDetectionJobResponse,
        RusotoError<StopDominantLanguageDetectionJobError>,
    >;

    /// <p>Stops an entities detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    async fn stop_entities_detection_job(
        &self,
        input: StopEntitiesDetectionJobRequest,
    ) -> Result<StopEntitiesDetectionJobResponse, RusotoError<StopEntitiesDetectionJobError>>;

    /// <p>Stops a key phrases detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    async fn stop_key_phrases_detection_job(
        &self,
        input: StopKeyPhrasesDetectionJobRequest,
    ) -> Result<StopKeyPhrasesDetectionJobResponse, RusotoError<StopKeyPhrasesDetectionJobError>>;

    /// <p>Stops a sentiment detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is be stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    async fn stop_sentiment_detection_job(
        &self,
        input: StopSentimentDetectionJobRequest,
    ) -> Result<StopSentimentDetectionJobResponse, RusotoError<StopSentimentDetectionJobError>>;

    /// <p>Stops a document classifier training job while in progress.</p> <p>If the training job state is <code>TRAINING</code>, the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the training job completes before it can be stopped, it is put into the <code>TRAINED</code>; otherwise the training job is stopped and put into the <code>STOPPED</code> state and the service sends back an HTTP 200 response with an empty HTTP body. </p>
    async fn stop_training_document_classifier(
        &self,
        input: StopTrainingDocumentClassifierRequest,
    ) -> Result<
        StopTrainingDocumentClassifierResponse,
        RusotoError<StopTrainingDocumentClassifierError>,
    >;

    /// <p>Stops an entity recognizer training job while in progress.</p> <p>If the training job state is <code>TRAINING</code>, the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the training job completes before it can be stopped, it is put into the <code>TRAINED</code>; otherwise the training job is stopped and putted into the <code>STOPPED</code> state and the service sends back an HTTP 200 response with an empty HTTP body.</p>
    async fn stop_training_entity_recognizer(
        &self,
        input: StopTrainingEntityRecognizerRequest,
    ) -> Result<StopTrainingEntityRecognizerResponse, RusotoError<StopTrainingEntityRecognizerError>>;

    /// <p>Associates a specific tag with an Amazon Comprehend resource. A tag is a key-value pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with "Sales" as the key might be added to a resource to indicate its use by the sales department. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes a specific tag associated with an Amazon Comprehend resource. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates information about the specified endpoint.</p>
    async fn update_endpoint(
        &self,
        input: UpdateEndpointRequest,
    ) -> Result<UpdateEndpointResponse, RusotoError<UpdateEndpointError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ComprehendClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ComprehendClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ComprehendClient {
        ComprehendClient { client, region }
    }
}

#[async_trait]
impl Comprehend for ComprehendClient {
    /// <p>Determines the dominant language of the input text for a batch of documents. For a list of languages that Amazon Comprehend can detect, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/how-languages.html">Amazon Comprehend Supported Languages</a>. </p>
    async fn batch_detect_dominant_language(
        &self,
        input: BatchDetectDominantLanguageRequest,
    ) -> Result<BatchDetectDominantLanguageResponse, RusotoError<BatchDetectDominantLanguageError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.BatchDetectDominantLanguage",
        );
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
                .deserialize::<BatchDetectDominantLanguageResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDetectDominantLanguageError::from_response(response))
        }
    }

    /// <p>Inspects the text of a batch of documents for named entities and returns information about them. For more information about named entities, see <a>how-entities</a> </p>
    async fn batch_detect_entities(
        &self,
        input: BatchDetectEntitiesRequest,
    ) -> Result<BatchDetectEntitiesResponse, RusotoError<BatchDetectEntitiesError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.BatchDetectEntities");
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
                .deserialize::<BatchDetectEntitiesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDetectEntitiesError::from_response(response))
        }
    }

    /// <p>Detects the key noun phrases found in a batch of documents.</p>
    async fn batch_detect_key_phrases(
        &self,
        input: BatchDetectKeyPhrasesRequest,
    ) -> Result<BatchDetectKeyPhrasesResponse, RusotoError<BatchDetectKeyPhrasesError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.BatchDetectKeyPhrases");
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
                .deserialize::<BatchDetectKeyPhrasesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDetectKeyPhrasesError::from_response(response))
        }
    }

    /// <p>Inspects a batch of documents and returns an inference of the prevailing sentiment, <code>POSITIVE</code>, <code>NEUTRAL</code>, <code>MIXED</code>, or <code>NEGATIVE</code>, in each one.</p>
    async fn batch_detect_sentiment(
        &self,
        input: BatchDetectSentimentRequest,
    ) -> Result<BatchDetectSentimentResponse, RusotoError<BatchDetectSentimentError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.BatchDetectSentiment");
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
                .deserialize::<BatchDetectSentimentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDetectSentimentError::from_response(response))
        }
    }

    /// <p>Inspects the text of a batch of documents for the syntax and part of speech of the words in the document and returns information about them. For more information, see <a>how-syntax</a>.</p>
    async fn batch_detect_syntax(
        &self,
        input: BatchDetectSyntaxRequest,
    ) -> Result<BatchDetectSyntaxResponse, RusotoError<BatchDetectSyntaxError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.BatchDetectSyntax");
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
                .deserialize::<BatchDetectSyntaxResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDetectSyntaxError::from_response(response))
        }
    }

    /// <p>Creates a new document classification request to analyze a single document in real-time, using a previously created and trained custom model and an endpoint.</p>
    async fn classify_document(
        &self,
        input: ClassifyDocumentRequest,
    ) -> Result<ClassifyDocumentResponse, RusotoError<ClassifyDocumentError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.ClassifyDocument");
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
                .deserialize::<ClassifyDocumentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ClassifyDocumentError::from_response(response))
        }
    }

    /// <p>Creates a new document classifier that you can use to categorize documents. To create a classifier you provide a set of training documents that labeled with the categories that you want to use. After the classifier is trained you can use it to categorize a set of labeled documents into the categories. For more information, see <a>how-document-classification</a>.</p>
    async fn create_document_classifier(
        &self,
        input: CreateDocumentClassifierRequest,
    ) -> Result<CreateDocumentClassifierResponse, RusotoError<CreateDocumentClassifierError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.CreateDocumentClassifier",
        );
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
                .deserialize::<CreateDocumentClassifierResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDocumentClassifierError::from_response(response))
        }
    }

    /// <p>Creates a model-specific endpoint for synchronous inference for a previously trained custom model </p>
    async fn create_endpoint(
        &self,
        input: CreateEndpointRequest,
    ) -> Result<CreateEndpointResponse, RusotoError<CreateEndpointError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.CreateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateEndpointResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEndpointError::from_response(response))
        }
    }

    /// <p>Creates an entity recognizer using submitted files. After your <code>CreateEntityRecognizer</code> request is submitted, you can check job status using the API. </p>
    async fn create_entity_recognizer(
        &self,
        input: CreateEntityRecognizerRequest,
    ) -> Result<CreateEntityRecognizerResponse, RusotoError<CreateEntityRecognizerError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.CreateEntityRecognizer");
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
                .deserialize::<CreateEntityRecognizerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEntityRecognizerError::from_response(response))
        }
    }

    /// <p>Deletes a previously created document classifier</p> <p>Only those classifiers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p> <p>This is an asynchronous action that puts the classifier into a DELETING state, and it is then removed by a background job. Once removed, the classifier disappears from your account and is no longer available for use. </p>
    async fn delete_document_classifier(
        &self,
        input: DeleteDocumentClassifierRequest,
    ) -> Result<DeleteDocumentClassifierResponse, RusotoError<DeleteDocumentClassifierError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DeleteDocumentClassifier",
        );
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
                .deserialize::<DeleteDocumentClassifierResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDocumentClassifierError::from_response(response))
        }
    }

    /// <p>Deletes a model-specific endpoint for a previously-trained custom model. All endpoints must be deleted in order for the model to be deleted.</p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointRequest,
    ) -> Result<DeleteEndpointResponse, RusotoError<DeleteEndpointError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DeleteEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteEndpointResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEndpointError::from_response(response))
        }
    }

    /// <p>Deletes an entity recognizer.</p> <p>Only those recognizers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p> <p>This is an asynchronous action that puts the recognizer into a DELETING state, and it is then removed by a background job. Once removed, the recognizer disappears from your account and is no longer available for use. </p>
    async fn delete_entity_recognizer(
        &self,
        input: DeleteEntityRecognizerRequest,
    ) -> Result<DeleteEntityRecognizerResponse, RusotoError<DeleteEntityRecognizerError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DeleteEntityRecognizer");
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
                .deserialize::<DeleteEntityRecognizerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEntityRecognizerError::from_response(response))
        }
    }

    /// <p>Gets the properties associated with a document classification job. Use this operation to get the status of a classification job.</p>
    async fn describe_document_classification_job(
        &self,
        input: DescribeDocumentClassificationJobRequest,
    ) -> Result<
        DescribeDocumentClassificationJobResponse,
        RusotoError<DescribeDocumentClassificationJobError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeDocumentClassificationJob",
        );
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
                .deserialize::<DescribeDocumentClassificationJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDocumentClassificationJobError::from_response(
                response,
            ))
        }
    }

    /// <p>Gets the properties associated with a document classifier.</p>
    async fn describe_document_classifier(
        &self,
        input: DescribeDocumentClassifierRequest,
    ) -> Result<DescribeDocumentClassifierResponse, RusotoError<DescribeDocumentClassifierError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeDocumentClassifier",
        );
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
                .deserialize::<DescribeDocumentClassifierResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDocumentClassifierError::from_response(response))
        }
    }

    /// <p>Gets the properties associated with a dominant language detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_dominant_language_detection_job(
        &self,
        input: DescribeDominantLanguageDetectionJobRequest,
    ) -> Result<
        DescribeDominantLanguageDetectionJobResponse,
        RusotoError<DescribeDominantLanguageDetectionJobError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeDominantLanguageDetectionJob",
        );
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
                .deserialize::<DescribeDominantLanguageDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDominantLanguageDetectionJobError::from_response(
                response,
            ))
        }
    }

    /// <p>Gets the properties associated with a specific endpoint. Use this operation to get the status of an endpoint.</p>
    async fn describe_endpoint(
        &self,
        input: DescribeEndpointRequest,
    ) -> Result<DescribeEndpointResponse, RusotoError<DescribeEndpointError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DescribeEndpoint");
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
                .deserialize::<DescribeEndpointResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEndpointError::from_response(response))
        }
    }

    /// <p>Gets the properties associated with an entities detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_entities_detection_job(
        &self,
        input: DescribeEntitiesDetectionJobRequest,
    ) -> Result<DescribeEntitiesDetectionJobResponse, RusotoError<DescribeEntitiesDetectionJobError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeEntitiesDetectionJob",
        );
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
                .deserialize::<DescribeEntitiesDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEntitiesDetectionJobError::from_response(response))
        }
    }

    /// <p>Provides details about an entity recognizer including status, S3 buckets containing training data, recognizer metadata, metrics, and so on.</p>
    async fn describe_entity_recognizer(
        &self,
        input: DescribeEntityRecognizerRequest,
    ) -> Result<DescribeEntityRecognizerResponse, RusotoError<DescribeEntityRecognizerError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeEntityRecognizer",
        );
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
                .deserialize::<DescribeEntityRecognizerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEntityRecognizerError::from_response(response))
        }
    }

    /// <p>Gets the properties associated with a key phrases detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_key_phrases_detection_job(
        &self,
        input: DescribeKeyPhrasesDetectionJobRequest,
    ) -> Result<
        DescribeKeyPhrasesDetectionJobResponse,
        RusotoError<DescribeKeyPhrasesDetectionJobError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeKeyPhrasesDetectionJob",
        );
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
                .deserialize::<DescribeKeyPhrasesDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeKeyPhrasesDetectionJobError::from_response(response))
        }
    }

    /// <p>Gets the properties associated with a sentiment detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_sentiment_detection_job(
        &self,
        input: DescribeSentimentDetectionJobRequest,
    ) -> Result<
        DescribeSentimentDetectionJobResponse,
        RusotoError<DescribeSentimentDetectionJobError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeSentimentDetectionJob",
        );
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
                .deserialize::<DescribeSentimentDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeSentimentDetectionJobError::from_response(response))
        }
    }

    /// <p>Gets the properties associated with a topic detection job. Use this operation to get the status of a detection job.</p>
    async fn describe_topics_detection_job(
        &self,
        input: DescribeTopicsDetectionJobRequest,
    ) -> Result<DescribeTopicsDetectionJobResponse, RusotoError<DescribeTopicsDetectionJobError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.DescribeTopicsDetectionJob",
        );
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
                .deserialize::<DescribeTopicsDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTopicsDetectionJobError::from_response(response))
        }
    }

    /// <p>Determines the dominant language of the input text. For a list of languages that Amazon Comprehend can detect, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/how-languages.html">Amazon Comprehend Supported Languages</a>. </p>
    async fn detect_dominant_language(
        &self,
        input: DetectDominantLanguageRequest,
    ) -> Result<DetectDominantLanguageResponse, RusotoError<DetectDominantLanguageError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectDominantLanguage");
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
                .deserialize::<DetectDominantLanguageResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectDominantLanguageError::from_response(response))
        }
    }

    /// <p>Inspects text for named entities, and returns information about them. For more information, about named entities, see <a>how-entities</a>. </p>
    async fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> Result<DetectEntitiesResponse, RusotoError<DetectEntitiesError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectEntities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DetectEntitiesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectEntitiesError::from_response(response))
        }
    }

    /// <p>Detects the key noun phrases found in the text. </p>
    async fn detect_key_phrases(
        &self,
        input: DetectKeyPhrasesRequest,
    ) -> Result<DetectKeyPhrasesResponse, RusotoError<DetectKeyPhrasesError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectKeyPhrases");
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
                .deserialize::<DetectKeyPhrasesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectKeyPhrasesError::from_response(response))
        }
    }

    /// <p>Inspects text and returns an inference of the prevailing sentiment (<code>POSITIVE</code>, <code>NEUTRAL</code>, <code>MIXED</code>, or <code>NEGATIVE</code>). </p>
    async fn detect_sentiment(
        &self,
        input: DetectSentimentRequest,
    ) -> Result<DetectSentimentResponse, RusotoError<DetectSentimentError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectSentiment");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DetectSentimentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectSentimentError::from_response(response))
        }
    }

    /// <p>Inspects text for syntax and the part of speech of words in the document. For more information, <a>how-syntax</a>.</p>
    async fn detect_syntax(
        &self,
        input: DetectSyntaxRequest,
    ) -> Result<DetectSyntaxResponse, RusotoError<DetectSyntaxError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.DetectSyntax");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DetectSyntaxResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectSyntaxError::from_response(response))
        }
    }

    /// <p>Gets a list of the documentation classification jobs that you have submitted.</p>
    async fn list_document_classification_jobs(
        &self,
        input: ListDocumentClassificationJobsRequest,
    ) -> Result<
        ListDocumentClassificationJobsResponse,
        RusotoError<ListDocumentClassificationJobsError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListDocumentClassificationJobs",
        );
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
                .deserialize::<ListDocumentClassificationJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDocumentClassificationJobsError::from_response(response))
        }
    }

    /// <p>Gets a list of the document classifiers that you have created.</p>
    async fn list_document_classifiers(
        &self,
        input: ListDocumentClassifiersRequest,
    ) -> Result<ListDocumentClassifiersResponse, RusotoError<ListDocumentClassifiersError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListDocumentClassifiers",
        );
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
                .deserialize::<ListDocumentClassifiersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDocumentClassifiersError::from_response(response))
        }
    }

    /// <p>Gets a list of the dominant language detection jobs that you have submitted.</p>
    async fn list_dominant_language_detection_jobs(
        &self,
        input: ListDominantLanguageDetectionJobsRequest,
    ) -> Result<
        ListDominantLanguageDetectionJobsResponse,
        RusotoError<ListDominantLanguageDetectionJobsError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListDominantLanguageDetectionJobs",
        );
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
                .deserialize::<ListDominantLanguageDetectionJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListDominantLanguageDetectionJobsError::from_response(
                response,
            ))
        }
    }

    /// <p>Gets a list of all existing endpoints that you've created.</p>
    async fn list_endpoints(
        &self,
        input: ListEndpointsRequest,
    ) -> Result<ListEndpointsResponse, RusotoError<ListEndpointsError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.ListEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListEndpointsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListEndpointsError::from_response(response))
        }
    }

    /// <p>Gets a list of the entity detection jobs that you have submitted.</p>
    async fn list_entities_detection_jobs(
        &self,
        input: ListEntitiesDetectionJobsRequest,
    ) -> Result<ListEntitiesDetectionJobsResponse, RusotoError<ListEntitiesDetectionJobsError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListEntitiesDetectionJobs",
        );
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
                .deserialize::<ListEntitiesDetectionJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListEntitiesDetectionJobsError::from_response(response))
        }
    }

    /// <p>Gets a list of the properties of all entity recognizers that you created, including recognizers currently in training. Allows you to filter the list of recognizers based on criteria such as status and submission time. This call returns up to 500 entity recognizers in the list, with a default number of 100 recognizers in the list.</p> <p>The results of this list are not in any particular order. Please get the list and sort locally if needed.</p>
    async fn list_entity_recognizers(
        &self,
        input: ListEntityRecognizersRequest,
    ) -> Result<ListEntityRecognizersResponse, RusotoError<ListEntityRecognizersError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.ListEntityRecognizers");
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
                .deserialize::<ListEntityRecognizersResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListEntityRecognizersError::from_response(response))
        }
    }

    /// <p>Get a list of key phrase detection jobs that you have submitted.</p>
    async fn list_key_phrases_detection_jobs(
        &self,
        input: ListKeyPhrasesDetectionJobsRequest,
    ) -> Result<ListKeyPhrasesDetectionJobsResponse, RusotoError<ListKeyPhrasesDetectionJobsError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListKeyPhrasesDetectionJobs",
        );
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
                .deserialize::<ListKeyPhrasesDetectionJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListKeyPhrasesDetectionJobsError::from_response(response))
        }
    }

    /// <p>Gets a list of sentiment detection jobs that you have submitted.</p>
    async fn list_sentiment_detection_jobs(
        &self,
        input: ListSentimentDetectionJobsRequest,
    ) -> Result<ListSentimentDetectionJobsResponse, RusotoError<ListSentimentDetectionJobsError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListSentimentDetectionJobs",
        );
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
                .deserialize::<ListSentimentDetectionJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListSentimentDetectionJobsError::from_response(response))
        }
    }

    /// <p>Lists all tags associated with a given Amazon Comprehend resource. </p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.ListTagsForResource");
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
                .deserialize::<ListTagsForResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Gets a list of the topic detection jobs that you have submitted.</p>
    async fn list_topics_detection_jobs(
        &self,
        input: ListTopicsDetectionJobsRequest,
    ) -> Result<ListTopicsDetectionJobsResponse, RusotoError<ListTopicsDetectionJobsError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.ListTopicsDetectionJobs",
        );
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
                .deserialize::<ListTopicsDetectionJobsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListTopicsDetectionJobsError::from_response(response))
        }
    }

    /// <p>Starts an asynchronous document classification job. Use the operation to track the progress of the job.</p>
    async fn start_document_classification_job(
        &self,
        input: StartDocumentClassificationJobRequest,
    ) -> Result<
        StartDocumentClassificationJobResponse,
        RusotoError<StartDocumentClassificationJobError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartDocumentClassificationJob",
        );
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
                .deserialize::<StartDocumentClassificationJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartDocumentClassificationJobError::from_response(response))
        }
    }

    /// <p>Starts an asynchronous dominant language detection job for a collection of documents. Use the operation to track the status of a job.</p>
    async fn start_dominant_language_detection_job(
        &self,
        input: StartDominantLanguageDetectionJobRequest,
    ) -> Result<
        StartDominantLanguageDetectionJobResponse,
        RusotoError<StartDominantLanguageDetectionJobError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartDominantLanguageDetectionJob",
        );
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
                .deserialize::<StartDominantLanguageDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartDominantLanguageDetectionJobError::from_response(
                response,
            ))
        }
    }

    /// <p>Starts an asynchronous entity detection job for a collection of documents. Use the operation to track the status of a job.</p> <p>This API can be used for either standard entity detection or custom entity recognition. In order to be used for custom entity recognition, the optional <code>EntityRecognizerArn</code> must be used in order to provide access to the recognizer being used to detect the custom entity.</p>
    async fn start_entities_detection_job(
        &self,
        input: StartEntitiesDetectionJobRequest,
    ) -> Result<StartEntitiesDetectionJobResponse, RusotoError<StartEntitiesDetectionJobError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartEntitiesDetectionJob",
        );
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
                .deserialize::<StartEntitiesDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartEntitiesDetectionJobError::from_response(response))
        }
    }

    /// <p>Starts an asynchronous key phrase detection job for a collection of documents. Use the operation to track the status of a job.</p>
    async fn start_key_phrases_detection_job(
        &self,
        input: StartKeyPhrasesDetectionJobRequest,
    ) -> Result<StartKeyPhrasesDetectionJobResponse, RusotoError<StartKeyPhrasesDetectionJobError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartKeyPhrasesDetectionJob",
        );
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
                .deserialize::<StartKeyPhrasesDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartKeyPhrasesDetectionJobError::from_response(response))
        }
    }

    /// <p>Starts an asynchronous sentiment detection job for a collection of documents. use the operation to track the status of a job.</p>
    async fn start_sentiment_detection_job(
        &self,
        input: StartSentimentDetectionJobRequest,
    ) -> Result<StartSentimentDetectionJobResponse, RusotoError<StartSentimentDetectionJobError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartSentimentDetectionJob",
        );
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
                .deserialize::<StartSentimentDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartSentimentDetectionJobError::from_response(response))
        }
    }

    /// <p>Starts an asynchronous topic detection job. Use the <code>DescribeTopicDetectionJob</code> operation to track the status of a job.</p>
    async fn start_topics_detection_job(
        &self,
        input: StartTopicsDetectionJobRequest,
    ) -> Result<StartTopicsDetectionJobResponse, RusotoError<StartTopicsDetectionJobError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StartTopicsDetectionJob",
        );
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
                .deserialize::<StartTopicsDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartTopicsDetectionJobError::from_response(response))
        }
    }

    /// <p>Stops a dominant language detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    async fn stop_dominant_language_detection_job(
        &self,
        input: StopDominantLanguageDetectionJobRequest,
    ) -> Result<
        StopDominantLanguageDetectionJobResponse,
        RusotoError<StopDominantLanguageDetectionJobError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopDominantLanguageDetectionJob",
        );
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
                .deserialize::<StopDominantLanguageDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopDominantLanguageDetectionJobError::from_response(
                response,
            ))
        }
    }

    /// <p>Stops an entities detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    async fn stop_entities_detection_job(
        &self,
        input: StopEntitiesDetectionJobRequest,
    ) -> Result<StopEntitiesDetectionJobResponse, RusotoError<StopEntitiesDetectionJobError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopEntitiesDetectionJob",
        );
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
                .deserialize::<StopEntitiesDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopEntitiesDetectionJobError::from_response(response))
        }
    }

    /// <p>Stops a key phrases detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    async fn stop_key_phrases_detection_job(
        &self,
        input: StopKeyPhrasesDetectionJobRequest,
    ) -> Result<StopKeyPhrasesDetectionJobResponse, RusotoError<StopKeyPhrasesDetectionJobError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopKeyPhrasesDetectionJob",
        );
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
                .deserialize::<StopKeyPhrasesDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopKeyPhrasesDetectionJobError::from_response(response))
        }
    }

    /// <p>Stops a sentiment detection job in progress.</p> <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is be stopped and put into the <code>STOPPED</code> state.</p> <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p> <p>When a job is stopped, any documents already processed are written to the output location.</p>
    async fn stop_sentiment_detection_job(
        &self,
        input: StopSentimentDetectionJobRequest,
    ) -> Result<StopSentimentDetectionJobResponse, RusotoError<StopSentimentDetectionJobError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopSentimentDetectionJob",
        );
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
                .deserialize::<StopSentimentDetectionJobResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopSentimentDetectionJobError::from_response(response))
        }
    }

    /// <p>Stops a document classifier training job while in progress.</p> <p>If the training job state is <code>TRAINING</code>, the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the training job completes before it can be stopped, it is put into the <code>TRAINED</code>; otherwise the training job is stopped and put into the <code>STOPPED</code> state and the service sends back an HTTP 200 response with an empty HTTP body. </p>
    async fn stop_training_document_classifier(
        &self,
        input: StopTrainingDocumentClassifierRequest,
    ) -> Result<
        StopTrainingDocumentClassifierResponse,
        RusotoError<StopTrainingDocumentClassifierError>,
    > {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopTrainingDocumentClassifier",
        );
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
                .deserialize::<StopTrainingDocumentClassifierResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopTrainingDocumentClassifierError::from_response(response))
        }
    }

    /// <p>Stops an entity recognizer training job while in progress.</p> <p>If the training job state is <code>TRAINING</code>, the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the training job completes before it can be stopped, it is put into the <code>TRAINED</code>; otherwise the training job is stopped and putted into the <code>STOPPED</code> state and the service sends back an HTTP 200 response with an empty HTTP body.</p>
    async fn stop_training_entity_recognizer(
        &self,
        input: StopTrainingEntityRecognizerRequest,
    ) -> Result<StopTrainingEntityRecognizerResponse, RusotoError<StopTrainingEntityRecognizerError>>
    {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Comprehend_20171127.StopTrainingEntityRecognizer",
        );
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
                .deserialize::<StopTrainingEntityRecognizerResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopTrainingEntityRecognizerError::from_response(response))
        }
    }

    /// <p>Associates a specific tag with an Amazon Comprehend resource. A tag is a key-value pair that adds as a metadata to a resource used by Amazon Comprehend. For example, a tag with "Sales" as the key might be added to a resource to indicate its use by the sales department. </p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes a specific tag associated with an Amazon Comprehend resource. </p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates information about the specified endpoint.</p>
    async fn update_endpoint(
        &self,
        input: UpdateEndpointRequest,
    ) -> Result<UpdateEndpointResponse, RusotoError<UpdateEndpointError>> {
        let mut request = SignedRequest::new("POST", "comprehend", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Comprehend_20171127.UpdateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<UpdateEndpointResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEndpointError::from_response(response))
        }
    }
}
