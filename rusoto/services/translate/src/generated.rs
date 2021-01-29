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

impl TranslateClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "translate", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch(
        &self,
        request: SignedRequest,
    ) -> Result<HttpResponse, RusotoError<std::convert::Infallible>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await?;
            return Err(RusotoError::Unknown(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>The custom terminology applied to the input text by Amazon Translate for the translated text response. This is optional in the response and will only be present if you specified terminology input in the request. Currently, only one terminology can be applied per TranslateText request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AppliedTerminology {
    /// <p>The name of the custom terminology applied to the input text by Amazon Translate for the translated text response.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The specific terms of the custom terminology applied to the input text by Amazon Translate for the translated text response. A maximum of 250 terms will be returned, and the specific terms applied will be the first 250 terms in the source text. </p>
    #[serde(rename = "Terms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<Term>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateParallelDataRequest {
    /// <p>A unique identifier for the request. This token is automatically generated when you use Amazon Translate through an AWS SDK.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>A custom description for the parallel data resource in Amazon Translate.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>A custom name for the parallel data resource in Amazon Translate. You must assign a name that is unique in the account and region.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies the format and S3 location of the parallel data input file.</p>
    #[serde(rename = "ParallelDataConfig")]
    pub parallel_data_config: ParallelDataConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateParallelDataResponse {
    /// <p>The custom name that you assigned to the parallel data resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the parallel data resource. When the resource is ready for you to use, the status is <code>ACTIVE</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteParallelDataRequest {
    /// <p>The name of the parallel data resource that is being deleted.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteParallelDataResponse {
    /// <p>The name of the parallel data resource that is being deleted.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the parallel data deletion.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTerminologyRequest {
    /// <p>The name of the custom terminology being deleted. </p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTextTranslationJobRequest {
    /// <p>The identifier that Amazon Translate generated for the job. The <a>StartTextTranslationJob</a> operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTextTranslationJobResponse {
    /// <p>An object that contains the properties associated with an asynchronous batch translation job.</p>
    #[serde(rename = "TextTranslationJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_translation_job_properties: Option<TextTranslationJobProperties>,
}

/// <p>The encryption key used to encrypt this object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncryptionKey {
    /// <p>The Amazon Resource Name (ARN) of the encryption key being used to encrypt the custom terminology.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The type of encryption key used by Amazon Translate to encrypt custom terminologies.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetParallelDataRequest {
    /// <p>The name of the parallel data resource that is being retrieved.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetParallelDataResponse {
    /// <p>The Amazon S3 location of a file that provides any errors or warnings that were produced by your input file. This file was created when Amazon Translate attempted to create a parallel data resource. The location is returned as a presigned URL to that has a 30 minute expiration.</p>
    #[serde(rename = "AuxiliaryDataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_data_location: Option<ParallelDataDataLocation>,
    /// <p>The location of the most recent parallel data input file that was successfully imported into Amazon Translate. The location is returned as a presigned URL that has a 30 minute expiration.</p>
    #[serde(rename = "DataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location: Option<ParallelDataDataLocation>,
    /// <p>The Amazon S3 location of a file that provides any errors or warnings that were produced by your input file. This file was created when Amazon Translate attempted to update a parallel data resource. The location is returned as a presigned URL to that has a 30 minute expiration.</p>
    #[serde(rename = "LatestUpdateAttemptAuxiliaryDataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_update_attempt_auxiliary_data_location: Option<ParallelDataDataLocation>,
    /// <p>The properties of the parallel data resource that is being retrieved.</p>
    #[serde(rename = "ParallelDataProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_data_properties: Option<ParallelDataProperties>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTerminologyRequest {
    /// <p>The name of the custom terminology being retrieved.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The data format of the custom terminology being retrieved, either CSV or TMX.</p>
    #[serde(rename = "TerminologyDataFormat")]
    pub terminology_data_format: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTerminologyResponse {
    /// <p>The data location of the custom terminology being retrieved. The custom terminology file is returned in a presigned url that has a 30 minute expiration.</p>
    #[serde(rename = "TerminologyDataLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_data_location: Option<TerminologyDataLocation>,
    /// <p>The properties of the custom terminology being retrieved.</p>
    #[serde(rename = "TerminologyProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_properties: Option<TerminologyProperties>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportTerminologyRequest {
    /// <p>The description of the custom terminology being imported.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The encryption key for the custom terminology being imported.</p>
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The merge strategy of the custom terminology being imported. Currently, only the OVERWRITE merge strategy is supported. In this case, the imported terminology will overwrite an existing terminology of the same name.</p>
    #[serde(rename = "MergeStrategy")]
    pub merge_strategy: String,
    /// <p>The name of the custom terminology being imported.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The terminology data for the custom terminology being imported.</p>
    #[serde(rename = "TerminologyData")]
    pub terminology_data: TerminologyData,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportTerminologyResponse {
    /// <p>The properties of the custom terminology being imported.</p>
    #[serde(rename = "TerminologyProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_properties: Option<TerminologyProperties>,
}

/// <p>The input configuration properties for requesting a batch translation job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputDataConfig {
    /// <p><p>Describes the format of the data that you submit to Amazon Translate as input. You can specify one of the following multipurpose internet mail extension (MIME) types:</p> <ul> <li> <p> <code>text/html</code>: The input data consists of one or more HTML files. Amazon Translate translates only the text that resides in the <code>html</code> element in each file.</p> </li> <li> <p> <code>text/plain</code>: The input data consists of one or more unformatted text files. Amazon Translate translates every character in this type of input.</p> </li> <li> <p> <code>application/vnd.openxmlformats-officedocument.wordprocessingml.document</code>: The input data consists of one or more Word documents (.docx).</p> </li> <li> <p> <code>application/vnd.openxmlformats-officedocument.presentationml.presentation</code>: The input data consists of one or more PowerPoint Presentation files (.pptx).</p> </li> <li> <p> <code>application/vnd.openxmlformats-officedocument.spreadsheetml.sheet</code>: The input data consists of one or more Excel Workbook files (.xlsx).</p> </li> </ul> <important> <p>If you structure your input data as HTML, ensure that you set this parameter to <code>text/html</code>. By doing so, you cut costs by limiting the translation to the contents of the <code>html</code> element in each file. Otherwise, if you set this parameter to <code>text/plain</code>, your costs will cover the translation of every character.</p> </important></p>
    #[serde(rename = "ContentType")]
    pub content_type: String,
    /// <p>The URI of the AWS S3 folder that contains the input file. The folder must be in the same Region as the API endpoint you are calling.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>The number of documents successfully and unsuccessfully processed during a translation job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobDetails {
    /// <p>The number of documents that could not be processed during a translation job.</p>
    #[serde(rename = "DocumentsWithErrorsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_with_errors_count: Option<i64>,
    /// <p>The number of documents used as input in a translation job.</p>
    #[serde(rename = "InputDocumentsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_documents_count: Option<i64>,
    /// <p>The number of documents successfully processed during a translation job.</p>
    #[serde(rename = "TranslatedDocumentsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_documents_count: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListParallelDataRequest {
    /// <p>The maximum number of parallel data resources returned for each request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A string that specifies the next page of results to return in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListParallelDataResponse {
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The properties of the parallel data resources returned by this request.</p>
    #[serde(rename = "ParallelDataPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_data_properties_list: Option<Vec<ParallelDataProperties>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTerminologiesRequest {
    /// <p>The maximum number of custom terminologies returned per list request.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the result of the request to ListTerminologies was truncated, include the NextToken to fetch the next group of custom terminologies. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTerminologiesResponse {
    /// <p> If the response to the ListTerminologies was truncated, the NextToken fetches the next group of custom terminologies.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The properties list of the custom terminologies returned on the list request.</p>
    #[serde(rename = "TerminologyPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_properties_list: Option<Vec<TerminologyProperties>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTextTranslationJobsRequest {
    /// <p>The parameters that specify which batch translation jobs to retrieve. Filters include job name, job status, and submission time. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TextTranslationJobFilter>,
    /// <p>The maximum number of results to return in each page. The default value is 100.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTextTranslationJobsResponse {
    /// <p>The token to use to retreive the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list containing the properties of each job that is returned.</p>
    #[serde(rename = "TextTranslationJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_translation_job_properties_list: Option<Vec<TextTranslationJobProperties>>,
}

/// <p>The output configuration properties for a batch translation job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputDataConfig {
    /// <p>The URI of the S3 folder that contains a translation job's output file. The folder must be in the same Region as the API endpoint that you are calling.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>Specifies the format and S3 location of the parallel data input file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ParallelDataConfig {
    /// <p>The format of the parallel data input file.</p>
    #[serde(rename = "Format")]
    pub format: String,
    /// <p>The URI of the Amazon S3 folder that contains the parallel data input file. The folder must be in the same Region as the API endpoint you are calling.</p>
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

/// <p>The location of the most recent parallel data input file that was successfully imported into Amazon Translate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParallelDataDataLocation {
    /// <p>The Amazon S3 location of the parallel data input file. The location is returned as a presigned URL to that has a 30 minute expiration.</p>
    #[serde(rename = "Location")]
    pub location: String,
    /// <p>Describes the repository that contains the parallel data input file.</p>
    #[serde(rename = "RepositoryType")]
    pub repository_type: String,
}

/// <p>The properties of a parallel data resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParallelDataProperties {
    /// <p>The Amazon Resource Name (ARN) of the parallel data resource.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time at which the parallel data resource was created.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description assigned to the parallel data resource.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The number of records unsuccessfully imported from the parallel data input file.</p>
    #[serde(rename = "FailedRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_record_count: Option<i64>,
    /// <p>The number of UTF-8 characters that Amazon Translate imported from the parallel data input file. This number includes only the characters in your translation examples. It does not include characters that are used to format your file. For example, if you provided a Translation Memory Exchange (.tmx) file, this number does not include the tags.</p>
    #[serde(rename = "ImportedDataSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_data_size: Option<i64>,
    /// <p>The number of records successfully imported from the parallel data input file.</p>
    #[serde(rename = "ImportedRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_record_count: Option<i64>,
    /// <p>The time at which the parallel data resource was last updated.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The time that the most recent update was attempted.</p>
    #[serde(rename = "LatestUpdateAttemptAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_update_attempt_at: Option<f64>,
    /// <p>The status of the most recent update attempt for the parallel data resource.</p>
    #[serde(rename = "LatestUpdateAttemptStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_update_attempt_status: Option<String>,
    /// <p>Additional information from Amazon Translate about the parallel data resource. </p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The custom name assigned to the parallel data resource.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies the format and S3 location of the parallel data input file.</p>
    #[serde(rename = "ParallelDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_data_config: Option<ParallelDataConfig>,
    /// <p>The number of items in the input file that Amazon Translate skipped when you created or updated the parallel data resource. For example, Amazon Translate skips empty records, empty target texts, and empty lines.</p>
    #[serde(rename = "SkippedRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_record_count: Option<i64>,
    /// <p>The source language of the translations in the parallel data file.</p>
    #[serde(rename = "SourceLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language_code: Option<String>,
    /// <p>The status of the parallel data resource. When the parallel data is ready for you to use, the status is <code>ACTIVE</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The language codes for the target languages available in the parallel data file. All possible target languages are returned as an array.</p>
    #[serde(rename = "TargetLanguageCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language_codes: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartTextTranslationJobRequest {
    /// <p>A unique identifier for the request. This token is auto-generated when using the Amazon Translate SDK.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity Access and Management (IAM) role that grants Amazon Translate read access to your input data. For more nformation, see <a>identity-and-access-management</a>.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and S3 location of the input documents for the translation job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The name of the batch translation job to be performed.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Specifies the S3 folder to which your job output will be saved. </p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
    /// <p>The names of the parallel data resources to use in the batch translation job. For a list of available parallel data resources, use the <a>ListParallelData</a> operation.</p>
    #[serde(rename = "ParallelDataNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_data_names: Option<Vec<String>>,
    /// <p>The language code of the input language. For a list of language codes, see <a>what-is-languages</a>.</p> <p>Amazon Translate does not automatically detect a source language during batch translation jobs.</p>
    #[serde(rename = "SourceLanguageCode")]
    pub source_language_code: String,
    /// <p>The language code of the output language.</p>
    #[serde(rename = "TargetLanguageCodes")]
    pub target_language_codes: Vec<String>,
    /// <p>The name of the terminology to use in the batch translation job. For a list of available terminologies, use the <a>ListTerminologies</a> operation.</p>
    #[serde(rename = "TerminologyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartTextTranslationJobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this ID with the <a>DescribeTextTranslationJob</a> operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p><p>The status of the job. Possible values include:</p> <ul> <li> <p> <code>SUBMITTED</code> - The job has been received and is queued for processing.</p> </li> <li> <p> <code>IN<em>PROGRESS</code> - Amazon Translate is processing the job.</p> </li> <li> <p> <code>COMPLETED</code> - The job was successfully completed and the output is available.</p> </li> <li> <p> <code>COMPLETED</em>WITH<em>ERROR</code> - The job was completed with errors. The errors can be analyzed in the job&#39;s output.</p> </li> <li> <p> <code>FAILED</code> - The job did not complete. To get details, use the <a>DescribeTextTranslationJob</a> operation.</p> </li> <li> <p> <code>STOP</em>REQUESTED</code> - The user who started the job has requested that it be stopped.</p> </li> <li> <p> <code>STOPPED</code> - The job has been stopped.</p> </li> </ul></p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopTextTranslationJobRequest {
    /// <p>The job ID of the job to be stopped.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopTextTranslationJobResponse {
    /// <p>The job ID of the stopped batch translation job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The status of the designated job. Upon successful completion, the job's status will be <code>STOPPED</code>.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

/// <p>The term being translated by the custom terminology.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Term {
    /// <p>The source text of the term being translated by the custom terminology.</p>
    #[serde(rename = "SourceText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_text: Option<String>,
    /// <p>The target text of the term being translated by the custom terminology.</p>
    #[serde(rename = "TargetText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_text: Option<String>,
}

/// <p>The data associated with the custom terminology.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TerminologyData {
    /// <p>The file containing the custom terminology data. Your version of the AWS SDK performs a Base64-encoding on this field before sending a request to the AWS service. Users of the SDK should not perform Base64-encoding themselves.</p>
    #[serde(rename = "File")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub file: bytes::Bytes,
    /// <p>The data format of the custom terminology. Either CSV or TMX.</p>
    #[serde(rename = "Format")]
    pub format: String,
}

/// <p>The location of the custom terminology data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminologyDataLocation {
    /// <p>The location of the custom terminology data.</p>
    #[serde(rename = "Location")]
    pub location: String,
    /// <p>The repository type for the custom terminology data.</p>
    #[serde(rename = "RepositoryType")]
    pub repository_type: String,
}

/// <p>The properties of the custom terminology.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminologyProperties {
    /// <p> The Amazon Resource Name (ARN) of the custom terminology. </p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time at which the custom terminology was created, based on the timestamp.</p>
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the custom terminology properties.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The encryption key for the custom terminology.</p>
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,
    /// <p>The time at which the custom terminology was last update, based on the timestamp.</p>
    #[serde(rename = "LastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The name of the custom terminology.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The size of the file used when importing a custom terminology.</p>
    #[serde(rename = "SizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// <p>The language code for the source text of the translation request for which the custom terminology is being used.</p>
    #[serde(rename = "SourceLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language_code: Option<String>,
    /// <p>The language codes for the target languages available with the custom terminology file. All possible target languages are returned in array.</p>
    #[serde(rename = "TargetLanguageCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language_codes: Option<Vec<String>>,
    /// <p>The number of terms included in the custom terminology.</p>
    #[serde(rename = "TermCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_count: Option<i64>,
}

/// <p>Provides information for filtering a list of translation jobs. For more information, see <a>ListTextTranslationJobs</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TextTranslationJobFilter {
    /// <p>Filters the list of jobs by name.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>Filters the list of jobs based by job status.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing and returns only the jobs submitted after the specified time. Jobs are returned in descending order, newest to oldest.</p>
    #[serde(rename = "SubmittedAfterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_after_time: Option<f64>,
    /// <p>Filters the list of jobs based on the time that the job was submitted for processing and returns only the jobs submitted before the specified time. Jobs are returned in ascending order, oldest to newest.</p>
    #[serde(rename = "SubmittedBeforeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_before_time: Option<f64>,
}

/// <p>Provides information about a translation job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TextTranslationJobProperties {
    /// <p>The Amazon Resource Name (ARN) of an AWS Identity Access and Management (IAM) role that granted Amazon Translate read access to the job's input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time at which the translation job ended.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The input configuration properties that were specified when the job was requested.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The number of documents successfully and unsuccessfully processed during the translation job.</p>
    #[serde(rename = "JobDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobDetails>,
    /// <p>The ID of the translation job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The user-defined name of the translation job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The status of the translation job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>An explanation of any errors that may have occured during the translation job.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The output configuration properties that were specified when the job was requested.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>A list containing the names of the parallel data resources applied to the translation job.</p>
    #[serde(rename = "ParallelDataNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_data_names: Option<Vec<String>>,
    /// <p>The language code of the language of the source text. The language must be a language supported by Amazon Translate.</p>
    #[serde(rename = "SourceLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language_code: Option<String>,
    /// <p>The time at which the translation job was submitted.</p>
    #[serde(rename = "SubmittedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_time: Option<f64>,
    /// <p>The language code of the language of the target text. The language must be a language supported by Amazon Translate.</p>
    #[serde(rename = "TargetLanguageCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language_codes: Option<Vec<String>>,
    /// <p>A list containing the names of the terminologies applied to a translation job. Only one terminology can be applied per <a>StartTextTranslationJob</a> request at this time.</p>
    #[serde(rename = "TerminologyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_names: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TranslateTextRequest {
    /// <p>The language code for the language of the source text. The language must be a language supported by Amazon Translate. For a list of language codes, see <a>what-is-languages</a>.</p> <p>To have Amazon Translate determine the source language of your text, you can specify <code>auto</code> in the <code>SourceLanguageCode</code> field. If you specify <code>auto</code>, Amazon Translate will call <a href="https://docs.aws.amazon.com/comprehend/latest/dg/comprehend-general.html">Amazon Comprehend</a> to determine the source language.</p>
    #[serde(rename = "SourceLanguageCode")]
    pub source_language_code: String,
    /// <p>The language code requested for the language of the target text. The language must be a language supported by Amazon Translate.</p>
    #[serde(rename = "TargetLanguageCode")]
    pub target_language_code: String,
    /// <p>The name of the terminology list file to be used in the TranslateText request. You can use 1 terminology list at most in a <code>TranslateText</code> request. Terminology lists can contain a maximum of 256 terms.</p>
    #[serde(rename = "TerminologyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_names: Option<Vec<String>>,
    /// <p>The text to translate. The text string can be a maximum of 5,000 bytes long. Depending on your character set, this may be fewer than 5,000 characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TranslateTextResponse {
    /// <p>The names of the custom terminologies applied to the input text by Amazon Translate for the translated text response.</p>
    #[serde(rename = "AppliedTerminologies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_terminologies: Option<Vec<AppliedTerminology>>,
    /// <p>The language code for the language of the source text.</p>
    #[serde(rename = "SourceLanguageCode")]
    pub source_language_code: String,
    /// <p>The language code for the language of the target text. </p>
    #[serde(rename = "TargetLanguageCode")]
    pub target_language_code: String,
    /// <p>The translated text.</p>
    #[serde(rename = "TranslatedText")]
    pub translated_text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateParallelDataRequest {
    /// <p>A unique identifier for the request. This token is automatically generated when you use Amazon Translate through an AWS SDK.</p>
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    /// <p>A custom description for the parallel data resource in Amazon Translate.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the parallel data resource being updated.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies the format and S3 location of the parallel data input file.</p>
    #[serde(rename = "ParallelDataConfig")]
    pub parallel_data_config: ParallelDataConfig,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateParallelDataResponse {
    /// <p>The time that the most recent update was attempted.</p>
    #[serde(rename = "LatestUpdateAttemptAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_update_attempt_at: Option<f64>,
    /// <p>The status of the parallel data update attempt. When the updated parallel data resource is ready for you to use, the status is <code>ACTIVE</code>.</p>
    #[serde(rename = "LatestUpdateAttemptStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_update_attempt_status: Option<String>,
    /// <p>The name of the parallel data resource being updated.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the parallel data resource that you are attempting to update. Your update request is accepted only if this status is either <code>ACTIVE</code> or <code>FAILED</code>.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Errors returned by CreateParallelData
#[derive(Debug, PartialEq)]
pub enum CreateParallelDataError {
    /// <p>There was a conflict processing the request. Try your request again.</p>
    Conflict(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request. </p>
    InvalidRequest(String),
    /// <p>The specified limit has been exceeded. Review your request and retry it with a quantity below the stated limit.</p>
    LimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl CreateParallelDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateParallelDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateParallelDataError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateParallelDataError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateParallelDataError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateParallelDataError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateParallelDataError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateParallelDataError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<CreateParallelDataError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for CreateParallelDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateParallelDataError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateParallelDataError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateParallelDataError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateParallelDataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateParallelDataError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateParallelDataError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateParallelDataError {}
/// Errors returned by DeleteParallelData
#[derive(Debug, PartialEq)]
pub enum DeleteParallelDataError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl DeleteParallelDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteParallelDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteParallelDataError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteParallelDataError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteParallelDataError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteParallelDataError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteParallelDataError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteParallelDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteParallelDataError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteParallelDataError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteParallelDataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteParallelDataError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteParallelDataError {}
/// Errors returned by DeleteTerminology
#[derive(Debug, PartialEq)]
pub enum DeleteTerminologyError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl DeleteTerminologyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTerminologyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteTerminologyError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteTerminologyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTerminologyError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteTerminologyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<DeleteTerminologyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DeleteTerminologyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTerminologyError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteTerminologyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteTerminologyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTerminologyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTerminologyError {}
/// Errors returned by DescribeTextTranslationJob
#[derive(Debug, PartialEq)]
pub enum DescribeTextTranslationJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl DescribeTextTranslationJobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTextTranslationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeTextTranslationJobError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTextTranslationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeTextTranslationJobError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<DescribeTextTranslationJobError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for DescribeTextTranslationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTextTranslationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeTextTranslationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTextTranslationJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTextTranslationJobError {}
/// Errors returned by GetParallelData
#[derive(Debug, PartialEq)]
pub enum GetParallelDataError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl GetParallelDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetParallelDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetParallelDataError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetParallelDataError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetParallelDataError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetParallelDataError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<GetParallelDataError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for GetParallelDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetParallelDataError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetParallelDataError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetParallelDataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetParallelDataError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetParallelDataError {}
/// Errors returned by GetTerminology
#[derive(Debug, PartialEq)]
pub enum GetTerminologyError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl GetTerminologyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTerminologyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetTerminologyError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetTerminologyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetTerminologyError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetTerminologyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<GetTerminologyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for GetTerminologyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTerminologyError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetTerminologyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetTerminologyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetTerminologyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTerminologyError {}
/// Errors returned by ImportTerminology
#[derive(Debug, PartialEq)]
pub enum ImportTerminologyError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p>The specified limit has been exceeded. Review your request and retry it with a quantity below the stated limit.</p>
    LimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl ImportTerminologyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportTerminologyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ImportTerminologyError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ImportTerminologyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ImportTerminologyError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ImportTerminologyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ImportTerminologyError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ImportTerminologyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportTerminologyError::InternalServer(ref cause) => write!(f, "{}", cause),
            ImportTerminologyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ImportTerminologyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ImportTerminologyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportTerminologyError {}
/// Errors returned by ListParallelData
#[derive(Debug, PartialEq)]
pub enum ListParallelDataError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl ListParallelDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListParallelDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListParallelDataError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListParallelDataError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListParallelDataError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListParallelDataError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListParallelDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListParallelDataError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListParallelDataError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListParallelDataError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListParallelDataError {}
/// Errors returned by ListTerminologies
#[derive(Debug, PartialEq)]
pub enum ListTerminologiesError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl ListTerminologiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTerminologiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTerminologiesError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTerminologiesError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTerminologiesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<ListTerminologiesError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListTerminologiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTerminologiesError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTerminologiesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListTerminologiesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTerminologiesError {}
/// Errors returned by ListTextTranslationJobs
#[derive(Debug, PartialEq)]
pub enum ListTextTranslationJobsError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilter(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request. </p>
    InvalidRequest(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl ListTextTranslationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTextTranslationJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTextTranslationJobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidFilterException" => {
                    return RusotoError::Service(ListTextTranslationJobsError::InvalidFilter(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTextTranslationJobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTextTranslationJobsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<ListTextTranslationJobsError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for ListTextTranslationJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTextTranslationJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTextTranslationJobsError::InvalidFilter(ref cause) => write!(f, "{}", cause),
            ListTextTranslationJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTextTranslationJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTextTranslationJobsError {}
/// Errors returned by StartTextTranslationJob
#[derive(Debug, PartialEq)]
pub enum StartTextTranslationJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request. </p>
    InvalidRequest(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
    /// <p>Amazon Translate does not support translation from the language of the source text into the requested target language. For more information, see <a>how-to-error-msg</a>. </p>
    UnsupportedLanguagePair(String),
}

impl StartTextTranslationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTextTranslationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartTextTranslationJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartTextTranslationJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartTextTranslationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartTextTranslationJobError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedLanguagePairException" => {
                    return RusotoError::Service(
                        StartTextTranslationJobError::UnsupportedLanguagePair(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<StartTextTranslationJobError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for StartTextTranslationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartTextTranslationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartTextTranslationJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartTextTranslationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartTextTranslationJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            StartTextTranslationJobError::UnsupportedLanguagePair(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartTextTranslationJobError {}
/// Errors returned by StopTextTranslationJob
#[derive(Debug, PartialEq)]
pub enum StopTextTranslationJobError {
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl StopTextTranslationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopTextTranslationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopTextTranslationJobError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopTextTranslationJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StopTextTranslationJobError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(
        err: RusotoError<std::convert::Infallible>,
    ) -> RusotoError<StopTextTranslationJobError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for StopTextTranslationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopTextTranslationJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopTextTranslationJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopTextTranslationJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopTextTranslationJobError {}
/// Errors returned by TranslateText
#[derive(Debug, PartialEq)]
pub enum TranslateTextError {
    /// <p>The confidence that Amazon Comprehend accurately detected the source language is low. If a low confidence level is acceptable for your application, you can use the language in the exception to call Amazon Translate again. For more information, see the <a href="https://docs.aws.amazon.com/comprehend/latest/dg/API_DetectDominantLanguage.html">DetectDominantLanguage</a> operation in the <i>Amazon Comprehend Developer Guide</i>. </p>
    DetectedLanguageLowConfidence(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request. </p>
    InvalidRequest(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p>The Amazon Translate service is temporarily unavailable. Please wait a bit and then retry your request.</p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
    /// <p>Amazon Translate does not support translation from the language of the source text into the requested target language. For more information, see <a>how-to-error-msg</a>. </p>
    UnsupportedLanguagePair(String),
}

impl TranslateTextError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TranslateTextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DetectedLanguageLowConfidenceException" => {
                    return RusotoError::Service(TranslateTextError::DetectedLanguageLowConfidence(
                        err.msg,
                    ))
                }
                "InternalServerException" => {
                    return RusotoError::Service(TranslateTextError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TranslateTextError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TranslateTextError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TranslateTextError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(TranslateTextError::TextSizeLimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TranslateTextError::TooManyRequests(err.msg))
                }
                "UnsupportedLanguagePairException" => {
                    return RusotoError::Service(TranslateTextError::UnsupportedLanguagePair(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<TranslateTextError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for TranslateTextError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TranslateTextError::DetectedLanguageLowConfidence(ref cause) => write!(f, "{}", cause),
            TranslateTextError::InternalServer(ref cause) => write!(f, "{}", cause),
            TranslateTextError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TranslateTextError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TranslateTextError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            TranslateTextError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            TranslateTextError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            TranslateTextError::UnsupportedLanguagePair(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TranslateTextError {}
/// Errors returned by UpdateParallelData
#[derive(Debug, PartialEq)]
pub enum UpdateParallelDataError {
    /// <p>Another modification is being made. That modification must complete before you can make your change.</p>
    ConcurrentModification(String),
    /// <p>There was a conflict processing the request. Try your request again.</p>
    Conflict(String),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServer(String),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to correct it, and then retry your operation.</p>
    InvalidParameterValue(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request. </p>
    InvalidRequest(String),
    /// <p>The specified limit has been exceeded. Review your request and retry it with a quantity below the stated limit.</p>
    LimitExceeded(String),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking for and see if a different resource will accomplish your needs before retrying the revised request.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again.</p>
    TooManyRequests(String),
}

impl UpdateParallelDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateParallelDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateParallelDataError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateParallelDataError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(UpdateParallelDataError::InternalServer(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateParallelDataError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateParallelDataError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateParallelDataError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateParallelDataError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateParallelDataError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }

    fn refine(err: RusotoError<std::convert::Infallible>) -> RusotoError<UpdateParallelDataError> {
        match err {
            RusotoError::Service(err) => match err {},
            RusotoError::HttpDispatch(err) => RusotoError::HttpDispatch(err),
            RusotoError::Credentials(err) => RusotoError::Credentials(err),
            RusotoError::Validation(err) => RusotoError::Validation(err),
            RusotoError::ParseError(err) => RusotoError::ParseError(err),
            RusotoError::Unknown(res) => Self::from_response(res),
            RusotoError::Blocking => RusotoError::Blocking,
        }
    }
}
impl fmt::Display for UpdateParallelDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateParallelDataError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateParallelDataError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateParallelDataError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateParallelDataError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateParallelDataError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateParallelDataError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateParallelDataError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateParallelDataError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateParallelDataError {}
/// Trait representing the capabilities of the Amazon Translate API. Amazon Translate clients implement this trait.
#[async_trait]
pub trait Translate {
    /// <p>Creates a parallel data resource in Amazon Translate by importing an input file from Amazon S3. Parallel data files contain examples of source phrases and their translations from your translation memory. By adding parallel data, you can influence the style, tone, and word choice in your translation output.</p>
    async fn create_parallel_data(
        &self,
        input: CreateParallelDataRequest,
    ) -> Result<CreateParallelDataResponse, RusotoError<CreateParallelDataError>>;

    /// <p>Deletes a parallel data resource in Amazon Translate.</p>
    async fn delete_parallel_data(
        &self,
        input: DeleteParallelDataRequest,
    ) -> Result<DeleteParallelDataResponse, RusotoError<DeleteParallelDataError>>;

    /// <p>A synchronous action that deletes a custom terminology.</p>
    async fn delete_terminology(
        &self,
        input: DeleteTerminologyRequest,
    ) -> Result<(), RusotoError<DeleteTerminologyError>>;

    /// <p>Gets the properties associated with an asycnhronous batch translation job including name, ID, status, source and target languages, input/output S3 buckets, and so on.</p>
    async fn describe_text_translation_job(
        &self,
        input: DescribeTextTranslationJobRequest,
    ) -> Result<DescribeTextTranslationJobResponse, RusotoError<DescribeTextTranslationJobError>>;

    /// <p>Provides information about a parallel data resource.</p>
    async fn get_parallel_data(
        &self,
        input: GetParallelDataRequest,
    ) -> Result<GetParallelDataResponse, RusotoError<GetParallelDataError>>;

    /// <p>Retrieves a custom terminology.</p>
    async fn get_terminology(
        &self,
        input: GetTerminologyRequest,
    ) -> Result<GetTerminologyResponse, RusotoError<GetTerminologyError>>;

    /// <p>Creates or updates a custom terminology, depending on whether or not one already exists for the given terminology name. Importing a terminology with the same name as an existing one will merge the terminologies based on the chosen merge strategy. Currently, the only supported merge strategy is OVERWRITE, and so the imported terminology will overwrite an existing terminology of the same name.</p> <p>If you import a terminology that overwrites an existing one, the new terminology take up to 10 minutes to fully propagate and be available for use in a translation due to cache policies with the DataPlane service that performs the translations.</p>
    async fn import_terminology(
        &self,
        input: ImportTerminologyRequest,
    ) -> Result<ImportTerminologyResponse, RusotoError<ImportTerminologyError>>;

    /// <p>Provides a list of your parallel data resources in Amazon Translate.</p>
    async fn list_parallel_data(
        &self,
        input: ListParallelDataRequest,
    ) -> Result<ListParallelDataResponse, RusotoError<ListParallelDataError>>;

    /// <p>Provides a list of custom terminologies associated with your account.</p>
    async fn list_terminologies(
        &self,
        input: ListTerminologiesRequest,
    ) -> Result<ListTerminologiesResponse, RusotoError<ListTerminologiesError>>;

    /// <p>Gets a list of the batch translation jobs that you have submitted.</p>
    async fn list_text_translation_jobs(
        &self,
        input: ListTextTranslationJobsRequest,
    ) -> Result<ListTextTranslationJobsResponse, RusotoError<ListTextTranslationJobsError>>;

    /// <p><p>Starts an asynchronous batch translation job. Batch translation jobs can be used to translate large volumes of text across multiple documents at once. For more information, see <a>async</a>.</p> <p>Batch translation jobs can be described with the <a>DescribeTextTranslationJob</a> operation, listed with the <a>ListTextTranslationJobs</a> operation, and stopped with the <a>StopTextTranslationJob</a> operation.</p> <note> <p>Amazon Translate does not support batch translation of multiple source languages at once.</p> </note></p>
    async fn start_text_translation_job(
        &self,
        input: StartTextTranslationJobRequest,
    ) -> Result<StartTextTranslationJobResponse, RusotoError<StartTextTranslationJobError>>;

    /// <p>Stops an asynchronous batch translation job that is in progress.</p> <p>If the job's state is <code>IN_PROGRESS</code>, the job will be marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state. Otherwise, the job is put into the <code>STOPPED</code> state.</p> <p>Asynchronous batch translation jobs are started with the <a>StartTextTranslationJob</a> operation. You can use the <a>DescribeTextTranslationJob</a> or <a>ListTextTranslationJobs</a> operations to get a batch translation job's <code>JobId</code>.</p>
    async fn stop_text_translation_job(
        &self,
        input: StopTextTranslationJobRequest,
    ) -> Result<StopTextTranslationJobResponse, RusotoError<StopTextTranslationJobError>>;

    /// <p>Translates input text from the source language to the target language. For a list of available languages and language codes, see <a>what-is-languages</a>.</p>
    async fn translate_text(
        &self,
        input: TranslateTextRequest,
    ) -> Result<TranslateTextResponse, RusotoError<TranslateTextError>>;

    /// <p>Updates a previously created parallel data resource by importing a new input file from Amazon S3.</p>
    async fn update_parallel_data(
        &self,
        input: UpdateParallelDataRequest,
    ) -> Result<UpdateParallelDataResponse, RusotoError<UpdateParallelDataError>>;
}
/// A client for the Amazon Translate API.
#[derive(Clone)]
pub struct TranslateClient {
    client: Client,
    region: region::Region,
}

impl TranslateClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> TranslateClient {
        TranslateClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TranslateClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        TranslateClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> TranslateClient {
        TranslateClient { client, region }
    }
}

#[async_trait]
impl Translate for TranslateClient {
    /// <p>Creates a parallel data resource in Amazon Translate by importing an input file from Amazon S3. Parallel data files contain examples of source phrases and their translations from your translation memory. By adding parallel data, you can influence the style, tone, and word choice in your translation output.</p>
    async fn create_parallel_data(
        &self,
        input: CreateParallelDataRequest,
    ) -> Result<CreateParallelDataResponse, RusotoError<CreateParallelDataError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.CreateParallelData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(CreateParallelDataError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateParallelDataResponse, _>()
    }

    /// <p>Deletes a parallel data resource in Amazon Translate.</p>
    async fn delete_parallel_data(
        &self,
        input: DeleteParallelDataRequest,
    ) -> Result<DeleteParallelDataResponse, RusotoError<DeleteParallelDataError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.DeleteParallelData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteParallelDataError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteParallelDataResponse, _>()
    }

    /// <p>A synchronous action that deletes a custom terminology.</p>
    async fn delete_terminology(
        &self,
        input: DeleteTerminologyRequest,
    ) -> Result<(), RusotoError<DeleteTerminologyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.DeleteTerminology",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DeleteTerminologyError::refine)?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Gets the properties associated with an asycnhronous batch translation job including name, ID, status, source and target languages, input/output S3 buckets, and so on.</p>
    async fn describe_text_translation_job(
        &self,
        input: DescribeTextTranslationJobRequest,
    ) -> Result<DescribeTextTranslationJobResponse, RusotoError<DescribeTextTranslationJobError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.DescribeTextTranslationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(DescribeTextTranslationJobError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeTextTranslationJobResponse, _>()
    }

    /// <p>Provides information about a parallel data resource.</p>
    async fn get_parallel_data(
        &self,
        input: GetParallelDataRequest,
    ) -> Result<GetParallelDataResponse, RusotoError<GetParallelDataError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.GetParallelData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetParallelDataError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetParallelDataResponse, _>()
    }

    /// <p>Retrieves a custom terminology.</p>
    async fn get_terminology(
        &self,
        input: GetTerminologyRequest,
    ) -> Result<GetTerminologyResponse, RusotoError<GetTerminologyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.GetTerminology",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(GetTerminologyError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetTerminologyResponse, _>()
    }

    /// <p>Creates or updates a custom terminology, depending on whether or not one already exists for the given terminology name. Importing a terminology with the same name as an existing one will merge the terminologies based on the chosen merge strategy. Currently, the only supported merge strategy is OVERWRITE, and so the imported terminology will overwrite an existing terminology of the same name.</p> <p>If you import a terminology that overwrites an existing one, the new terminology take up to 10 minutes to fully propagate and be available for use in a translation due to cache policies with the DataPlane service that performs the translations.</p>
    async fn import_terminology(
        &self,
        input: ImportTerminologyRequest,
    ) -> Result<ImportTerminologyResponse, RusotoError<ImportTerminologyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.ImportTerminology",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ImportTerminologyError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ImportTerminologyResponse, _>()
    }

    /// <p>Provides a list of your parallel data resources in Amazon Translate.</p>
    async fn list_parallel_data(
        &self,
        input: ListParallelDataRequest,
    ) -> Result<ListParallelDataResponse, RusotoError<ListParallelDataError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.ListParallelData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListParallelDataError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListParallelDataResponse, _>()
    }

    /// <p>Provides a list of custom terminologies associated with your account.</p>
    async fn list_terminologies(
        &self,
        input: ListTerminologiesRequest,
    ) -> Result<ListTerminologiesResponse, RusotoError<ListTerminologiesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.ListTerminologies",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListTerminologiesError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTerminologiesResponse, _>()
    }

    /// <p>Gets a list of the batch translation jobs that you have submitted.</p>
    async fn list_text_translation_jobs(
        &self,
        input: ListTextTranslationJobsRequest,
    ) -> Result<ListTextTranslationJobsResponse, RusotoError<ListTextTranslationJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.ListTextTranslationJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(ListTextTranslationJobsError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListTextTranslationJobsResponse, _>()
    }

    /// <p><p>Starts an asynchronous batch translation job. Batch translation jobs can be used to translate large volumes of text across multiple documents at once. For more information, see <a>async</a>.</p> <p>Batch translation jobs can be described with the <a>DescribeTextTranslationJob</a> operation, listed with the <a>ListTextTranslationJobs</a> operation, and stopped with the <a>StopTextTranslationJob</a> operation.</p> <note> <p>Amazon Translate does not support batch translation of multiple source languages at once.</p> </note></p>
    async fn start_text_translation_job(
        &self,
        input: StartTextTranslationJobRequest,
    ) -> Result<StartTextTranslationJobResponse, RusotoError<StartTextTranslationJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.StartTextTranslationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(StartTextTranslationJobError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartTextTranslationJobResponse, _>()
    }

    /// <p>Stops an asynchronous batch translation job that is in progress.</p> <p>If the job's state is <code>IN_PROGRESS</code>, the job will be marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state. Otherwise, the job is put into the <code>STOPPED</code> state.</p> <p>Asynchronous batch translation jobs are started with the <a>StartTextTranslationJob</a> operation. You can use the <a>DescribeTextTranslationJob</a> or <a>ListTextTranslationJobs</a> operations to get a batch translation job's <code>JobId</code>.</p>
    async fn stop_text_translation_job(
        &self,
        input: StopTextTranslationJobRequest,
    ) -> Result<StopTextTranslationJobResponse, RusotoError<StopTextTranslationJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.StopTextTranslationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(StopTextTranslationJobError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StopTextTranslationJobResponse, _>()
    }

    /// <p>Translates input text from the source language to the target language. For a list of available languages and language codes, see <a>what-is-languages</a>.</p>
    async fn translate_text(
        &self,
        input: TranslateTextRequest,
    ) -> Result<TranslateTextResponse, RusotoError<TranslateTextError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.TranslateText",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(TranslateTextError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TranslateTextResponse, _>()
    }

    /// <p>Updates a previously created parallel data resource by importing a new input file from Amazon S3.</p>
    async fn update_parallel_data(
        &self,
        input: UpdateParallelDataRequest,
    ) -> Result<UpdateParallelDataResponse, RusotoError<UpdateParallelDataError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSShineFrontendService_20170701.UpdateParallelData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request)
            .await
            .map_err(UpdateParallelDataError::refine)?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateParallelDataResponse, _>()
    }
}
