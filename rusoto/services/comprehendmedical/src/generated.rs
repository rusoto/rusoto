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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p> An extracted segment of the text that is an attribute of an entity, or otherwise related to an entity, such as the dosage of a medication taken. It contains information about the attribute such as id, begin and end offset within the input text, and the segment of the input text. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Attribute {
    /// <p> The 0-based character offset in the input text that shows where the attribute begins. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "BeginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p> The 0-based character offset in the input text that shows where the attribute ends. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "EndOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p> The numeric identifier for this attribute. This is a monotonically increasing id unique within this response rather than a global unique identifier. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p> The level of confidence that Amazon Comprehend Medical has that this attribute is correctly related to this entity. </p>
    #[serde(rename = "RelationshipScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_score: Option<f32>,
    /// <p> The level of confidence that Amazon Comprehend Medical has that the segment of text is correctly recognized as an attribute. </p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p> The segment of input text extracted as this attribute.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p> Contextual information for this attribute. </p>
    #[serde(rename = "Traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<Trait>>,
    /// <p> The type of attribute. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Provides information for filtering a list of detection jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ComprehendMedicalAsyncJobFilter {
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

/// <p>Provides information about a detection job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComprehendMedicalAsyncJobProperties {
    /// <p>The Amazon Resource Name (ARN) that gives Amazon Comprehend Medical read access to your input data.</p>
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    /// <p>The time that the detection job completed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The date and time that job metadata is deleted from the server. Output files in your S3 bucket will not be deleted. After the metadata is deleted, the job will no longer appear in the results of the <code>ListEntitiesDetectionV2Job</code> or the <code>ListPHIDetectionJobs</code> operation.</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>The input data configuration that you supplied when you created the detection job.</p>
    #[serde(rename = "InputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    /// <p>The identifier assigned to the detection job.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The name that you assigned to the detection job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>The current status of the detection job. If the status is <code>FAILED</code>, the <code>Message</code> field shows the reason for the failure.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The AWS Key Management Service key, if any, used to encrypt the output files. </p>
    #[serde(rename = "KMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The language code of the input documents.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>The path to the file that describes the results of a batch job.</p>
    #[serde(rename = "ManifestFilePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_file_path: Option<String>,
    /// <p>A description of the status of a job.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The version of the model used to analyze the documents. The version number looks like X.X.X. You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "ModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    /// <p>The output data configuration that you supplied when you created the detection job.</p>
    #[serde(rename = "OutputDataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    /// <p>The time that the detection job was submitted for processing.</p>
    #[serde(rename = "SubmitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEntitiesDetectionV2JobRequest {
    /// <p>The identifier that Amazon Comprehend Medical generated for the job. The <code>StartEntitiesDetectionV2Job</code> operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEntitiesDetectionV2JobResponse {
    /// <p>An object that contains the properties associated with a detection job.</p>
    #[serde(rename = "ComprehendMedicalAsyncJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties: Option<ComprehendMedicalAsyncJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePHIDetectionJobRequest {
    /// <p>The identifier that Amazon Comprehend Medical generated for the job. The <code>StartPHIDetectionJob</code> operation returns this identifier in its response.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePHIDetectionJobResponse {
    /// <p>An object that contains the properties associated with a detection job.</p>
    #[serde(rename = "ComprehendMedicalAsyncJobProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties: Option<ComprehendMedicalAsyncJobProperties>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectEntitiesRequest {
    /// <p> A UTF-8 text string containing the clinical content being examined for entities. Each string must contain fewer than 20,000 bytes of characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectEntitiesResponse {
    /// <p> The collection of medical entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence that Amazon Comprehend Medical has in the detection and analysis. Attributes and traits of the entity are also returned.</p>
    #[serde(rename = "Entities")]
    pub entities: Vec<Entity>,
    /// <p>The version of the model used to analyze the documents. The version number looks like X.X.X. You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "ModelVersion")]
    pub model_version: String,
    /// <p> If the result of the previous request to <code>DetectEntities</code> was truncated, include the <code>PaginationToken</code> to fetch the next page of entities.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p> Attributes extracted from the input text that we were unable to relate to an entity.</p>
    #[serde(rename = "UnmappedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmapped_attributes: Option<Vec<UnmappedAttribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectEntitiesV2Request {
    /// <p>A UTF-8 string containing the clinical content being examined for entities. Each string must contain fewer than 20,000 bytes of characters.</p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectEntitiesV2Response {
    /// <p>The collection of medical entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence in the detection and analysis. Attributes and traits of the entity are also returned.</p>
    #[serde(rename = "Entities")]
    pub entities: Vec<Entity>,
    /// <p>The version of the model used to analyze the documents. The version number looks like X.X.X. You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "ModelVersion")]
    pub model_version: String,
    /// <p>If the result to the <code>DetectEntitiesV2</code> operation was truncated, include the <code>PaginationToken</code> to fetch the next page of entities.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>Attributes extracted from the input text that couldn't be related to an entity.</p>
    #[serde(rename = "UnmappedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmapped_attributes: Option<Vec<UnmappedAttribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectPHIRequest {
    /// <p> A UTF-8 text string containing the clinical content being examined for PHI entities. Each string must contain fewer than 20,000 bytes of characters. </p>
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectPHIResponse {
    /// <p> The collection of PHI entities extracted from the input text and their associated information. For each entity, the response provides the entity text, the entity category, where the entity text begins and ends, and the level of confidence that Amazon Comprehend Medical has in its detection. </p>
    #[serde(rename = "Entities")]
    pub entities: Vec<Entity>,
    /// <p>The version of the model used to analyze the documents. The version number looks like X.X.X. You can use this information to track the model used for a particular batch of documents.</p>
    #[serde(rename = "ModelVersion")]
    pub model_version: String,
    /// <p> If the result of the previous request to <code>DetectPHI</code> was truncated, include the <code>PaginationToken</code> to fetch the next page of PHI entities. </p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

/// <p> Provides information about an extracted medical entity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Entity {
    /// <p> The extracted attributes that relate to this entity.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p> The 0-based character offset in the input text that shows where the entity begins. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "BeginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// <p> The category of the entity.</p>
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p> The 0-based character offset in the input text that shows where the entity ends. The offset returns the UTF-8 code point in the string. </p>
    #[serde(rename = "EndOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// <p> The numeric identifier for the entity. This is a monotonically increasing id unique within this response rather than a global unique identifier. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The level of confidence that Amazon Comprehend Medical has in the accuracy of the detection.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// <p> The segment of input text extracted as this entity.</p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// <p>Contextual information for the entity</p>
    #[serde(rename = "Traits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<Trait>>,
    /// <p> Describes the specific type of entity with category of entities. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The input properties for an entities detection job</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputDataConfig {
    /// <p>The URI of the S3 bucket that contains the input data. The bucket must be in the same region as the API endpoint that you are calling.</p> <p>Each file in the document collection must be less than 40 KB. You can store a maximum of 30 GB in the bucket.</p>
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    /// <p>The path to the input data files in the S3 bucket.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEntitiesDetectionV2JobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs based on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ComprehendMedicalAsyncJobFilter>,
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
pub struct ListEntitiesDetectionV2JobsResponse {
    /// <p>A list containing the properties of each job returned.</p>
    #[serde(rename = "ComprehendMedicalAsyncJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties_list:
        Option<Vec<ComprehendMedicalAsyncJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPHIDetectionJobsRequest {
    /// <p>Filters the jobs that are returned. You can filter jobs based on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ComprehendMedicalAsyncJobFilter>,
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
pub struct ListPHIDetectionJobsResponse {
    /// <p>A list containing the properties of each job returned.</p>
    #[serde(rename = "ComprehendMedicalAsyncJobPropertiesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_async_job_properties_list:
        Option<Vec<ComprehendMedicalAsyncJobProperties>>,
    /// <p>Identifies the next page of results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The output properties for a detection job.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputDataConfig {
    /// <p>When you use the <code>OutputDataConfig</code> object with asynchronous operations, you specify the Amazon S3 location where you want to write the output data. The URI must be in the same region as the API endpoint that you are calling. The location is used as the prefix for the actual location of the output.</p>
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    /// <p>The path to the output data files in the S3 bucket. Amazon Comprehend Medical creates an output directory using the job ID so that the output from one job does not overwrite the output of another.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartEntitiesDetectionV2JobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend Medical generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend Medical read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions-med.html#auth-role-permissions-med"> Role-Based Permissions Required for Asynchronous Operations</a>.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>An AWS Key Management Service key to encrypt your output files. If you do not specify a key, the files are written in plain text.</p>
    #[serde(rename = "KMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The language of the input documents. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartEntitiesDetectionV2JobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this identifier with the <code>DescribeEntitiesDetectionV2Job</code> operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartPHIDetectionJobRequest {
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend Medical generates one.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that grants Amazon Comprehend Medical read access to your input data. For more information, see <a href="https://docs.aws.amazon.com/comprehend/latest/dg/access-control-managing-permissions-med.html#auth-role-permissions-med"> Role-Based Permissions Required for Asynchronous Operations</a>.</p>
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// <p>Specifies the format and location of the input data for the job.</p>
    #[serde(rename = "InputDataConfig")]
    pub input_data_config: InputDataConfig,
    /// <p>The identifier of the job.</p>
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// <p>An AWS Key Management Service key to encrypt your output files. If you do not specify a key, the files are written in plain text.</p>
    #[serde(rename = "KMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The language of the input documents. All documents must be in the same language.</p>
    #[serde(rename = "LanguageCode")]
    pub language_code: String,
    /// <p>Specifies where to send the output files.</p>
    #[serde(rename = "OutputDataConfig")]
    pub output_data_config: OutputDataConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartPHIDetectionJobResponse {
    /// <p>The identifier generated for the job. To get the status of a job, use this identifier with the <code>DescribePHIDetectionJob</code> operation.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopEntitiesDetectionV2JobRequest {
    /// <p>The identifier of the medical entities job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopEntitiesDetectionV2JobResponse {
    /// <p>The identifier of the medical entities detection job that was stopped.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopPHIDetectionJobRequest {
    /// <p>The identifier of the PHI detection job to stop.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopPHIDetectionJobResponse {
    /// <p>The identifier of the PHI detection job that was stopped.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p> Provides contextual information about the extracted entity. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Trait {
    /// <p> Provides a name or contextual description about the trait. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p> The level of confidence that Amazon Comprehend Medical has in the accuracy of this trait.</p>
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

/// <p> An attribute that we extracted, but were unable to relate to an entity. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnmappedAttribute {
    /// <p> The specific attribute that has been extracted but not mapped to an entity. </p>
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<Attribute>,
    /// <p> The type of the attribute, could be one of the following values: "MEDICATION", "MEDICAL_CONDITION", "ANATOMY", "TEST_AND_TREATMENT_PROCEDURE" or "PROTECTED_HEALTH_INFORMATION". </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Errors returned by DescribeEntitiesDetectionV2Job
#[derive(Debug, PartialEq)]
pub enum DescribeEntitiesDetectionV2JobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DescribeEntitiesDetectionV2JobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEntitiesDetectionV2JobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionV2JobError::InternalServer(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionV2JobError::InvalidRequest(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionV2JobError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeEntitiesDetectionV2JobError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEntitiesDetectionV2JobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEntitiesDetectionV2JobError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEntitiesDetectionV2JobError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEntitiesDetectionV2JobError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEntitiesDetectionV2JobError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEntitiesDetectionV2JobError {}
/// Errors returned by DescribePHIDetectionJob
#[derive(Debug, PartialEq)]
pub enum DescribePHIDetectionJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DescribePHIDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePHIDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribePHIDetectionJobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribePHIDetectionJobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePHIDetectionJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribePHIDetectionJobError::TooManyRequests(
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
impl fmt::Display for DescribePHIDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePHIDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribePHIDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribePHIDetectionJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribePHIDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePHIDetectionJobError {}
/// Errors returned by DetectEntities
#[derive(Debug, PartialEq)]
pub enum DetectEntitiesError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Amazon Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DetectEntitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectEntitiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectEntitiesError::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(DetectEntitiesError::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectEntitiesError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DetectEntitiesError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectEntitiesError::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetectEntitiesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectEntitiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectEntitiesError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::InvalidEncoding(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectEntitiesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectEntitiesError {}
/// Errors returned by DetectEntitiesV2
#[derive(Debug, PartialEq)]
pub enum DetectEntitiesV2Error {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Amazon Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DetectEntitiesV2Error {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectEntitiesV2Error> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::TextSizeLimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetectEntitiesV2Error::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectEntitiesV2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectEntitiesV2Error::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::InvalidEncoding(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectEntitiesV2Error::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectEntitiesV2Error {}
/// Errors returned by DetectPHI
#[derive(Debug, PartialEq)]
pub enum DetectPHIError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The input text was not in valid UTF-8 character encoding. Check your text then retry your request.</p>
    InvalidEncoding(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> The Amazon Comprehend Medical service is temporarily unavailable. Please wait and then retry your request. </p>
    ServiceUnavailable(String),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or use a smaller document and then retry your request. </p>
    TextSizeLimitExceeded(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl DetectPHIError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectPHIError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DetectPHIError::InternalServer(err.msg))
                }
                "InvalidEncodingException" => {
                    return RusotoError::Service(DetectPHIError::InvalidEncoding(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DetectPHIError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DetectPHIError::ServiceUnavailable(err.msg))
                }
                "TextSizeLimitExceededException" => {
                    return RusotoError::Service(DetectPHIError::TextSizeLimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetectPHIError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectPHIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectPHIError::InternalServer(ref cause) => write!(f, "{}", cause),
            DetectPHIError::InvalidEncoding(ref cause) => write!(f, "{}", cause),
            DetectPHIError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DetectPHIError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DetectPHIError::TextSizeLimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectPHIError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectPHIError {}
/// Errors returned by ListEntitiesDetectionV2Jobs
#[derive(Debug, PartialEq)]
pub enum ListEntitiesDetectionV2JobsError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl ListEntitiesDetectionV2JobsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListEntitiesDetectionV2JobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListEntitiesDetectionV2JobsError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListEntitiesDetectionV2JobsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEntitiesDetectionV2JobsError::TooManyRequests(
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
impl fmt::Display for ListEntitiesDetectionV2JobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEntitiesDetectionV2JobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListEntitiesDetectionV2JobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListEntitiesDetectionV2JobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEntitiesDetectionV2JobsError {}
/// Errors returned by ListPHIDetectionJobs
#[derive(Debug, PartialEq)]
pub enum ListPHIDetectionJobsError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl ListPHIDetectionJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPHIDetectionJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListPHIDetectionJobsError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPHIDetectionJobsError::InvalidRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPHIDetectionJobsError::TooManyRequests(
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
impl fmt::Display for ListPHIDetectionJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPHIDetectionJobsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListPHIDetectionJobsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListPHIDetectionJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPHIDetectionJobsError {}
/// Errors returned by StartEntitiesDetectionV2Job
#[derive(Debug, PartialEq)]
pub enum StartEntitiesDetectionV2JobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl StartEntitiesDetectionV2JobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartEntitiesDetectionV2JobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartEntitiesDetectionV2JobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartEntitiesDetectionV2JobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StartEntitiesDetectionV2JobError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartEntitiesDetectionV2JobError::TooManyRequests(
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
impl fmt::Display for StartEntitiesDetectionV2JobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartEntitiesDetectionV2JobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionV2JobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionV2JobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartEntitiesDetectionV2JobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartEntitiesDetectionV2JobError {}
/// Errors returned by StartPHIDetectionJob
#[derive(Debug, PartialEq)]
pub enum StartPHIDetectionJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and then try your request again. Contact customer support for more information about a service limit increase. </p>
    TooManyRequests(String),
}

impl StartPHIDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartPHIDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartPHIDetectionJobError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartPHIDetectionJobError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartPHIDetectionJobError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartPHIDetectionJobError::TooManyRequests(
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
impl fmt::Display for StartPHIDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartPHIDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartPHIDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartPHIDetectionJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartPHIDetectionJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartPHIDetectionJobError {}
/// Errors returned by StopEntitiesDetectionV2Job
#[derive(Debug, PartialEq)]
pub enum StopEntitiesDetectionV2JobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
}

impl StopEntitiesDetectionV2JobError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopEntitiesDetectionV2JobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopEntitiesDetectionV2JobError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopEntitiesDetectionV2JobError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopEntitiesDetectionV2JobError::ResourceNotFound(
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
impl fmt::Display for StopEntitiesDetectionV2JobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopEntitiesDetectionV2JobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopEntitiesDetectionV2JobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopEntitiesDetectionV2JobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopEntitiesDetectionV2JobError {}
/// Errors returned by StopPHIDetectionJob
#[derive(Debug, PartialEq)]
pub enum StopPHIDetectionJobError {
    /// <p> An internal server error occurred. Retry your request. </p>
    InternalServer(String),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid and then retry the request.</p>
    InvalidRequest(String),
    /// <p>The resource identified by the specified Amazon Resource Name (ARN) was not found. Check the ARN and try your request again.</p>
    ResourceNotFound(String),
}

impl StopPHIDetectionJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopPHIDetectionJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StopPHIDetectionJobError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopPHIDetectionJobError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopPHIDetectionJobError::ResourceNotFound(
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
impl fmt::Display for StopPHIDetectionJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopPHIDetectionJobError::InternalServer(ref cause) => write!(f, "{}", cause),
            StopPHIDetectionJobError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopPHIDetectionJobError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopPHIDetectionJobError {}
/// Trait representing the capabilities of the ComprehendMedical API. ComprehendMedical clients implement this trait.
pub trait ComprehendMedical: region::GetRegion {
    /// <p>Gets the properties associated with a medical entities detection job. Use this operation to get the status of a detection job.</p>
    fn describe_entities_detection_v2_job(
        &self,
        input: DescribeEntitiesDetectionV2JobRequest,
    ) -> RusotoFuture<DescribeEntitiesDetectionV2JobResponse, DescribeEntitiesDetectionV2JobError>;

    /// <p>Gets the properties associated with a protected health information (PHI) detection job. Use this operation to get the status of a detection job.</p>
    fn describe_phi_detection_job(
        &self,
        input: DescribePHIDetectionJobRequest,
    ) -> RusotoFuture<DescribePHIDetectionJobResponse, DescribePHIDetectionJobError>;

    /// <p>The <code>DetectEntities</code> operation is deprecated. You should use the <a>DetectEntitiesV2</a> operation instead.</p> <p> Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information .</p>
    fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> RusotoFuture<DetectEntitiesResponse, DetectEntitiesError>;

    /// <p>Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information.</p> <p>The <code>DetectEntitiesV2</code> operation replaces the <a>DetectEntities</a> operation. This new action uses a different model for determining the entities in your medical text and changes the way that some entities are returned in the output. You should use the <code>DetectEntitiesV2</code> operation in all new applications.</p> <p>The <code>DetectEntitiesV2</code> operation returns the <code>Acuity</code> and <code>Direction</code> entities as attributes instead of types. It does not return the <code>Quality</code> or <code>Quantity</code> entities.</p>
    fn detect_entities_v2(
        &self,
        input: DetectEntitiesV2Request,
    ) -> RusotoFuture<DetectEntitiesV2Response, DetectEntitiesV2Error>;

    /// <p> Inspects the clinical text for protected health information (PHI) entities and entity category, location, and confidence score on that information.</p>
    fn detect_phi(
        &self,
        input: DetectPHIRequest,
    ) -> RusotoFuture<DetectPHIResponse, DetectPHIError>;

    /// <p>Gets a list of medical entity detection jobs that you have submitted.</p>
    fn list_entities_detection_v2_jobs(
        &self,
        input: ListEntitiesDetectionV2JobsRequest,
    ) -> RusotoFuture<ListEntitiesDetectionV2JobsResponse, ListEntitiesDetectionV2JobsError>;

    /// <p>Gets a list of protected health information (PHI) detection jobs that you have submitted.</p>
    fn list_phi_detection_jobs(
        &self,
        input: ListPHIDetectionJobsRequest,
    ) -> RusotoFuture<ListPHIDetectionJobsResponse, ListPHIDetectionJobsError>;

    /// <p>Starts an asynchronous medical entity detection job for a collection of documents. Use the <code>DescribeEntitiesDetectionV2Job</code> operation to track the status of a job.</p>
    fn start_entities_detection_v2_job(
        &self,
        input: StartEntitiesDetectionV2JobRequest,
    ) -> RusotoFuture<StartEntitiesDetectionV2JobResponse, StartEntitiesDetectionV2JobError>;

    /// <p>Starts an asynchronous job to detect protected health information (PHI). Use the <code>DescribePHIDetectionJob</code> operation to track the status of a job.</p>
    fn start_phi_detection_job(
        &self,
        input: StartPHIDetectionJobRequest,
    ) -> RusotoFuture<StartPHIDetectionJobResponse, StartPHIDetectionJobError>;

    /// <p>Stops a medical entities detection job in progress.</p>
    fn stop_entities_detection_v2_job(
        &self,
        input: StopEntitiesDetectionV2JobRequest,
    ) -> RusotoFuture<StopEntitiesDetectionV2JobResponse, StopEntitiesDetectionV2JobError>;

    /// <p>Stops a protected health information (PHI) detection job in progress.</p>
    fn stop_phi_detection_job(
        &self,
        input: StopPHIDetectionJobRequest,
    ) -> RusotoFuture<StopPHIDetectionJobResponse, StopPHIDetectionJobError>;
}
/// A client for the ComprehendMedical API.
#[derive(Clone)]
pub struct ComprehendMedicalClient {
    client: Client,
    region: region::Region,
}

impl ComprehendMedicalClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ComprehendMedicalClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ComprehendMedicalClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ComprehendMedicalClient {
        ComprehendMedicalClient { client, region }
    }
}

impl fmt::Debug for ComprehendMedicalClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComprehendMedicalClient")
            .field("region", &self.region)
            .finish()
    }
}

impl region::GetRegion for ComprehendMedicalClient {
    fn region(&self) -> &region::Region {
        &self.region
    }
}

impl ComprehendMedical for ComprehendMedicalClient {
    /// <p>Gets the properties associated with a medical entities detection job. Use this operation to get the status of a detection job.</p>
    fn describe_entities_detection_v2_job(
        &self,
        input: DescribeEntitiesDetectionV2JobRequest,
    ) -> RusotoFuture<DescribeEntitiesDetectionV2JobResponse, DescribeEntitiesDetectionV2JobError>
    {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.DescribeEntitiesDetectionV2Job",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEntitiesDetectionV2JobResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEntitiesDetectionV2JobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the properties associated with a protected health information (PHI) detection job. Use this operation to get the status of a detection job.</p>
    fn describe_phi_detection_job(
        &self,
        input: DescribePHIDetectionJobRequest,
    ) -> RusotoFuture<DescribePHIDetectionJobResponse, DescribePHIDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.DescribePHIDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePHIDetectionJobResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribePHIDetectionJobError::from_response(response))
                }))
            }
        })
    }

    /// <p>The <code>DetectEntities</code> operation is deprecated. You should use the <a>DetectEntitiesV2</a> operation instead.</p> <p> Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information .</p>
    fn detect_entities(
        &self,
        input: DetectEntitiesRequest,
    ) -> RusotoFuture<DetectEntitiesResponse, DetectEntitiesError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ComprehendMedical_20181030.DetectEntities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectEntitiesResponse, _>()
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

    /// <p>Inspects the clinical text for a variety of medical entities and returns specific information about them such as entity category, location, and confidence score on that information.</p> <p>The <code>DetectEntitiesV2</code> operation replaces the <a>DetectEntities</a> operation. This new action uses a different model for determining the entities in your medical text and changes the way that some entities are returned in the output. You should use the <code>DetectEntitiesV2</code> operation in all new applications.</p> <p>The <code>DetectEntitiesV2</code> operation returns the <code>Acuity</code> and <code>Direction</code> entities as attributes instead of types. It does not return the <code>Quality</code> or <code>Quantity</code> entities.</p>
    fn detect_entities_v2(
        &self,
        input: DetectEntitiesV2Request,
    ) -> RusotoFuture<DetectEntitiesV2Response, DetectEntitiesV2Error> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.DetectEntitiesV2",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectEntitiesV2Response, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectEntitiesV2Error::from_response(response))),
                )
            }
        })
    }

    /// <p> Inspects the clinical text for protected health information (PHI) entities and entity category, location, and confidence score on that information.</p>
    fn detect_phi(
        &self,
        input: DetectPHIRequest,
    ) -> RusotoFuture<DetectPHIResponse, DetectPHIError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "ComprehendMedical_20181030.DetectPHI");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectPHIResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectPHIError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of medical entity detection jobs that you have submitted.</p>
    fn list_entities_detection_v2_jobs(
        &self,
        input: ListEntitiesDetectionV2JobsRequest,
    ) -> RusotoFuture<ListEntitiesDetectionV2JobsResponse, ListEntitiesDetectionV2JobsError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.ListEntitiesDetectionV2Jobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListEntitiesDetectionV2JobsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListEntitiesDetectionV2JobsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets a list of protected health information (PHI) detection jobs that you have submitted.</p>
    fn list_phi_detection_jobs(
        &self,
        input: ListPHIDetectionJobsRequest,
    ) -> RusotoFuture<ListPHIDetectionJobsResponse, ListPHIDetectionJobsError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.ListPHIDetectionJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPHIDetectionJobsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListPHIDetectionJobsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts an asynchronous medical entity detection job for a collection of documents. Use the <code>DescribeEntitiesDetectionV2Job</code> operation to track the status of a job.</p>
    fn start_entities_detection_v2_job(
        &self,
        input: StartEntitiesDetectionV2JobRequest,
    ) -> RusotoFuture<StartEntitiesDetectionV2JobResponse, StartEntitiesDetectionV2JobError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StartEntitiesDetectionV2Job",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartEntitiesDetectionV2JobResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartEntitiesDetectionV2JobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts an asynchronous job to detect protected health information (PHI). Use the <code>DescribePHIDetectionJob</code> operation to track the status of a job.</p>
    fn start_phi_detection_job(
        &self,
        input: StartPHIDetectionJobRequest,
    ) -> RusotoFuture<StartPHIDetectionJobResponse, StartPHIDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StartPHIDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartPHIDetectionJobResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartPHIDetectionJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops a medical entities detection job in progress.</p>
    fn stop_entities_detection_v2_job(
        &self,
        input: StopEntitiesDetectionV2JobRequest,
    ) -> RusotoFuture<StopEntitiesDetectionV2JobResponse, StopEntitiesDetectionV2JobError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StopEntitiesDetectionV2Job",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopEntitiesDetectionV2JobResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopEntitiesDetectionV2JobError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops a protected health information (PHI) detection job in progress.</p>
    fn stop_phi_detection_job(
        &self,
        input: StopPHIDetectionJobRequest,
    ) -> RusotoFuture<StopPHIDetectionJobResponse, StopPHIDetectionJobError> {
        let mut request = SignedRequest::new("POST", "comprehendmedical", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ComprehendMedical_20181030.StopPHIDetectionJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopPHIDetectionJobResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopPHIDetectionJobError::from_response(response))
                    }),
                )
            }
        })
    }
}
