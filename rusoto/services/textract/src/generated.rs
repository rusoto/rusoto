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
pub struct AnalyzeDocumentRequest {
    /// <p>The input document as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Textract operations, you can't pass image bytes. The document must be an image in JPEG or PNG format.</p> <p>If you're using an AWS SDK to call Amazon Textract, you might not need to base64-encode image bytes that are passed using the <code>Bytes</code> field. </p>
    #[serde(rename = "Document")]
    pub document: Document,
    /// <p>A list of the types of analysis to perform. Add TABLES to the list to return information about the tables that are detected in the input document. Add FORMS to return detected form data. To perform both types of analysis, add TABLES and FORMS to <code>FeatureTypes</code>. All lines and words detected in the document are included in the response (including text that isn't related to the value of <code>FeatureTypes</code>). </p>
    #[serde(rename = "FeatureTypes")]
    pub feature_types: Vec<String>,
    /// <p>Sets the configuration for the human in the loop workflow for analyzing documents.</p>
    #[serde(rename = "HumanLoopConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_config: Option<HumanLoopConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AnalyzeDocumentResponse {
    /// <p>The version of the model used to analyze the document.</p>
    #[serde(rename = "AnalyzeDocumentModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_document_model_version: Option<String>,
    /// <p>The items that are detected and analyzed by <code>AnalyzeDocument</code>.</p>
    #[serde(rename = "Blocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    /// <p>Metadata about the analyzed document. An example is the number of pages.</p>
    #[serde(rename = "DocumentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    /// <p>Shows the results of the human in the loop evaluation.</p>
    #[serde(rename = "HumanLoopActivationOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_output: Option<HumanLoopActivationOutput>,
}

/// <p>A <code>Block</code> represents items that are recognized in a document within a group of pixels close to each other. The information returned in a <code>Block</code> object depends on the type of operation. In text detection for documents (for example <a>DetectDocumentText</a>), you get information about the detected words and lines of text. In text analysis (for example <a>AnalyzeDocument</a>), you can also get information about the fields, tables, and selection elements that are detected in the document.</p> <p>An array of <code>Block</code> objects is returned by both synchronous and asynchronous operations. In synchronous operations, such as <a>DetectDocumentText</a>, the array of <code>Block</code> objects is the entire set of results. In asynchronous operations, such as <a>GetDocumentAnalysis</a>, the array is returned over one or more responses.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works.html">How Amazon Textract Works</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Block {
    /// <p><p>The type of text item that&#39;s recognized. In operations for text detection, the following types are returned:</p> <ul> <li> <p> <i>PAGE</i> - Contains a list of the LINE <code>Block</code> objects that are detected on a document page.</p> </li> <li> <p> <i>WORD</i> - A word detected on a document page. A word is one or more ISO basic Latin script characters that aren&#39;t separated by spaces.</p> </li> <li> <p> <i>LINE</i> - A string of tab-delimited, contiguous words that are detected on a document page.</p> </li> </ul> <p>In text analysis operations, the following types are returned:</p> <ul> <li> <p> <i>PAGE</i> - Contains a list of child <code>Block</code> objects that are detected on a document page.</p> </li> <li> <p> <i>KEY<em>VALUE</em>SET</i> - Stores the KEY and VALUE <code>Block</code> objects for linked text that&#39;s detected on a document page. Use the <code>EntityType</code> field to determine if a KEY<em>VALUE</em>SET object is a KEY <code>Block</code> object or a VALUE <code>Block</code> object. </p> </li> <li> <p> <i>WORD</i> - A word that&#39;s detected on a document page. A word is one or more ISO basic Latin script characters that aren&#39;t separated by spaces.</p> </li> <li> <p> <i>LINE</i> - A string of tab-delimited, contiguous words that are detected on a document page.</p> </li> <li> <p> <i>TABLE</i> - A table that&#39;s detected on a document page. A table is grid-based information with two or more rows or columns, with a cell span of one row and one column each. </p> </li> <li> <p> <i>CELL</i> - A cell within a detected table. The cell is the parent of the block that contains the text in the cell.</p> </li> <li> <p> <i>SELECTION_ELEMENT</i> - A selection element such as an option button (radio button) or a check box that&#39;s detected on a document page. Use the value of <code>SelectionStatus</code> to determine the status of the selection element.</p> </li> </ul></p>
    #[serde(rename = "BlockType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    /// <p>The column in which a table cell appears. The first column position is 1. <code>ColumnIndex</code> isn't returned by <code>DetectDocumentText</code> and <code>GetDocumentTextDetection</code>.</p>
    #[serde(rename = "ColumnIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_index: Option<i64>,
    /// <p>The number of columns that a table cell spans. Currently this value is always 1, even if the number of columns spanned is greater than 1. <code>ColumnSpan</code> isn't returned by <code>DetectDocumentText</code> and <code>GetDocumentTextDetection</code>. </p>
    #[serde(rename = "ColumnSpan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<i64>,
    /// <p>The confidence score that Amazon Textract has in the accuracy of the recognized text and the accuracy of the geometry points around the recognized text.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The type of entity. The following can be returned:</p> <ul> <li> <p> <i>KEY</i> - An identifier for a field on the document.</p> </li> <li> <p> <i>VALUE</i> - The field text.</p> </li> </ul> <p> <code>EntityTypes</code> isn't returned by <code>DetectDocumentText</code> and <code>GetDocumentTextDetection</code>.</p>
    #[serde(rename = "EntityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<String>>,
    /// <p>The location of the recognized text on the image. It includes an axis-aligned, coarse bounding box that surrounds the text, and a finer-grain polygon for more accurate spatial information. </p>
    #[serde(rename = "Geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    /// <p>The identifier for the recognized text. The identifier is only unique for a single operation. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The page on which a block was detected. <code>Page</code> is returned by asynchronous operations. Page values greater than 1 are only returned for multipage documents that are in PDF format. A scanned image (JPEG/PNG), even if it contains multiple document pages, is considered to be a single-page document. The value of <code>Page</code> is always 1. Synchronous operations don't return <code>Page</code> because every input document is considered to be a single-page document.</p>
    #[serde(rename = "Page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// <p><p>A list of child blocks of the current block. For example, a LINE object has child blocks for each WORD block that&#39;s part of the line of text. There aren&#39;t Relationship objects in the list for relationships that don&#39;t exist, such as when the current block has no child blocks. The list size can be the following:</p> <ul> <li> <p>0 - The block has no child blocks.</p> </li> <li> <p>1 - The block has child blocks.</p> </li> </ul></p>
    #[serde(rename = "Relationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Relationship>>,
    /// <p>The row in which a table cell is located. The first row position is 1. <code>RowIndex</code> isn't returned by <code>DetectDocumentText</code> and <code>GetDocumentTextDetection</code>.</p>
    #[serde(rename = "RowIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_index: Option<i64>,
    /// <p>The number of rows that a table cell spans. Currently this value is always 1, even if the number of rows spanned is greater than 1. <code>RowSpan</code> isn't returned by <code>DetectDocumentText</code> and <code>GetDocumentTextDetection</code>.</p>
    #[serde(rename = "RowSpan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<i64>,
    /// <p>The selection status of a selection element, such as an option button or check box. </p>
    #[serde(rename = "SelectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_status: Option<String>,
    /// <p>The word or line of text that's recognized by Amazon Textract. </p>
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// <p>The bounding box around the detected page, text, key-value pair, table, table cell, or selection element on a document page. The <code>left</code> (x-coordinate) and <code>top</code> (y-coordinate) are coordinates that represent the top and left sides of the bounding box. Note that the upper-left corner of the image is the origin (0,0). </p> <p>The <code>top</code> and <code>left</code> values returned are ratios of the overall document page size. For example, if the input image is 700 x 200 pixels, and the top-left coordinate of the bounding box is 350 x 50 pixels, the API returns a <code>left</code> value of 0.5 (350/700) and a <code>top</code> value of 0.25 (50/200).</p> <p>The <code>width</code> and <code>height</code> values represent the dimensions of the bounding box as a ratio of the overall document page dimension. For example, if the document page size is 700 x 200 pixels, and the bounding box width is 70 pixels, the width returned is 0.1. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BoundingBox {
    /// <p>The height of the bounding box as a ratio of the overall document page height.</p>
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    /// <p>The left coordinate of the bounding box as a ratio of overall document page width.</p>
    #[serde(rename = "Left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f32>,
    /// <p>The top coordinate of the bounding box as a ratio of overall document page height.</p>
    #[serde(rename = "Top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f32>,
    /// <p>The width of the bounding box as a ratio of the overall document page width.</p>
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectDocumentTextRequest {
    /// <p>The input document as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Textract operations, you can't pass image bytes. The document must be an image in JPEG or PNG format.</p> <p>If you're using an AWS SDK to call Amazon Textract, you might not need to base64-encode image bytes that are passed using the <code>Bytes</code> field. </p>
    #[serde(rename = "Document")]
    pub document: Document,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectDocumentTextResponse {
    /// <p>An array of <code>Block</code> objects that contain the text that's detected in the document.</p>
    #[serde(rename = "Blocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    /// <p><p/></p>
    #[serde(rename = "DetectDocumentTextModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_document_text_model_version: Option<String>,
    /// <p>Metadata about the document. It contains the number of pages that are detected in the document.</p>
    #[serde(rename = "DocumentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
}

/// <p>The input document, either as bytes or as an S3 object.</p> <p>You pass image bytes to an Amazon Textract API operation by using the <code>Bytes</code> property. For example, you would use the <code>Bytes</code> property to pass a document loaded from a local file system. Image bytes passed by using the <code>Bytes</code> property must be base64 encoded. Your code might not need to encode document file bytes if you're using an AWS SDK to call Amazon Textract API operations. </p> <p>You pass images stored in an S3 bucket to an Amazon Textract API operation by using the <code>S3Object</code> property. Documents stored in an S3 bucket don't need to be base64 encoded.</p> <p>The AWS Region for the S3 bucket that contains the S3 object must match the AWS Region that you use for Amazon Textract operations.</p> <p>If you use the AWS CLI to call Amazon Textract operations, passing image bytes using the Bytes property isn't supported. You must first upload the document to an Amazon S3 bucket, and then call the operation using the S3Object property.</p> <p>For Amazon Textract to process an S3 object, the user must have permission to access the S3 object. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Document {
    /// <p>A blob of base64-encoded document bytes. The maximum size of a document that's provided in a blob of bytes is 5 MB. The document bytes must be in PNG or JPEG format.</p> <p>If you're using an AWS SDK to call Amazon Textract, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. </p>
    #[serde(rename = "Bytes")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<bytes::Bytes>,
    /// <p>Identifies an S3 object as the document source. The maximum size of a document that's stored in an S3 bucket is 5 MB.</p>
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>The Amazon S3 bucket that contains the document to be processed. It's used by asynchronous operations such as <a>StartDocumentTextDetection</a>.</p> <p>The input document can be an image file in JPEG or PNG format. It can also be a file in PDF format.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DocumentLocation {
    /// <p>The Amazon S3 bucket that contains the input document.</p>
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>Information about the input document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DocumentMetadata {
    /// <p>The number of pages that are detected in the document.</p>
    #[serde(rename = "Pages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<i64>,
}

/// <p>Information about where the following items are located on a document page: detected page, text, key-value pairs, tables, table cells, and selection elements.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Geometry {
    /// <p>An axis-aligned coarse representation of the location of the recognized item on the document page.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Within the bounding box, a fine-grained polygon around the recognized item.</p>
    #[serde(rename = "Polygon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Vec<Point>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDocumentAnalysisRequest {
    /// <p>A unique identifier for the text-detection job. The <code>JobId</code> is returned from <code>StartDocumentAnalysis</code>. A <code>JobId</code> value is only valid for 7 days.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The maximum number of results to return per paginated call. The largest value that you can specify is 1,000. If you specify a value greater than 1,000, a maximum of 1,000 results is returned. The default value is 1,000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more blocks to retrieve), Amazon Textract returns a pagination token in the response. You can use this pagination token to retrieve the next set of blocks.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDocumentAnalysisResponse {
    /// <p><p/></p>
    #[serde(rename = "AnalyzeDocumentModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_document_model_version: Option<String>,
    /// <p>The results of the text-analysis operation.</p>
    #[serde(rename = "Blocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    /// <p>Information about a document that Amazon Textract processed. <code>DocumentMetadata</code> is returned in every page of paginated responses from an Amazon Textract video operation.</p>
    #[serde(rename = "DocumentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    /// <p>The current status of the text detection job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Textract returns this token. You can use this token in the subsequent request to retrieve the next set of text detection results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The current status of an asynchronous document-analysis operation.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>A list of warnings that occurred during the document-analysis operation.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDocumentTextDetectionRequest {
    /// <p>A unique identifier for the text detection job. The <code>JobId</code> is returned from <code>StartDocumentTextDetection</code>. A <code>JobId</code> value is only valid for 7 days.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 1,000. If you specify a value greater than 1,000, a maximum of 1,000 results is returned. The default value is 1,000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more blocks to retrieve), Amazon Textract returns a pagination token in the response. You can use this pagination token to retrieve the next set of blocks.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDocumentTextDetectionResponse {
    /// <p>The results of the text-detection operation.</p>
    #[serde(rename = "Blocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    /// <p><p/></p>
    #[serde(rename = "DetectDocumentTextModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_document_text_model_version: Option<String>,
    /// <p>Information about a document that Amazon Textract processed. <code>DocumentMetadata</code> is returned in every page of paginated responses from an Amazon Textract video operation.</p>
    #[serde(rename = "DocumentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    /// <p>The current status of the text detection job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Textract returns this token. You can use this token in the subsequent request to retrieve the next set of text-detection results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The current status of an asynchronous text-detection operation for the document. </p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>A list of warnings that occurred during the text-detection operation for the document.</p>
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
}

/// <p>Shows the results of the human in the loop evaluation. If there is no HumanLoopArn, the input did not trigger human review.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HumanLoopActivationOutput {
    /// <p>Shows the result of condition evaluations, including those conditions which activated a human review.</p>
    #[serde(rename = "HumanLoopActivationConditionsEvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_conditions_evaluation_results: Option<String>,
    /// <p>Shows if and why human review was needed.</p>
    #[serde(rename = "HumanLoopActivationReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_reasons: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the HumanLoop created.</p>
    #[serde(rename = "HumanLoopArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_arn: Option<String>,
}

/// <p>Sets up the human review workflow the document will be sent to if one of the conditions is met. You can also set certain attributes of the image before review. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HumanLoopConfig {
    /// <p>Sets attributes of the input data.</p>
    #[serde(rename = "DataAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HumanLoopDataAttributes>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    #[serde(rename = "FlowDefinitionArn")]
    pub flow_definition_arn: String,
    /// <p>The name of the human workflow used for this image. This should be kept unique within a region.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

/// <p>Allows you to set attributes of the image. Currently, you can declare an image as free of personally identifiable information and adult content. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HumanLoopDataAttributes {
    /// <p>Sets whether the input image is free of personally identifiable information or adult content.</p>
    #[serde(rename = "ContentClassifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_classifiers: Option<Vec<String>>,
}

/// <p>The Amazon Simple Notification Service (Amazon SNS) topic to which Amazon Textract publishes the completion status of an asynchronous document operation, such as <a>StartDocumentTextDetection</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NotificationChannel {
    /// <p>The Amazon Resource Name (ARN) of an IAM role that gives Amazon Textract publishing permissions to the Amazon SNS topic. </p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The Amazon SNS topic that Amazon Textract posts the completion status to.</p>
    #[serde(rename = "SNSTopicArn")]
    pub sns_topic_arn: String,
}

/// <p>The X and Y coordinates of a point on a document page. The X and Y values that are returned are ratios of the overall document page size. For example, if the input document is 700 x 200 and the operation returns X=0.5 and Y=0.25, then the point is at the (350,50) pixel coordinate on the document page.</p> <p>An array of <code>Point</code> objects, <code>Polygon</code>, is returned by <a>DetectDocumentText</a>. <code>Polygon</code> represents a fine-grained polygon around detected text. For more information, see Geometry in the Amazon Textract Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Point {
    /// <p>The value of the X coordinate for a point on a <code>Polygon</code>.</p>
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    /// <p>The value of the Y coordinate for a point on a <code>Polygon</code>.</p>
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

/// <p>Information about how blocks are related to each other. A <code>Block</code> object contains 0 or more <code>Relation</code> objects in a list, <code>Relationships</code>. For more information, see <a>Block</a>.</p> <p>The <code>Type</code> element provides the type of the relationship for all blocks in the <code>IDs</code> array. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Relationship {
    /// <p>An array of IDs for related blocks. You can get the type of the relationship from the <code>Type</code> element.</p>
    #[serde(rename = "Ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>The type of relationship that the blocks in the IDs array have with the current block. The relationship can be <code>VALUE</code> or <code>CHILD</code>. A relationship of type VALUE is a list that contains the ID of the VALUE block that's associated with the KEY of a key-value pair. A relationship of type CHILD is a list of IDs that identify WORD blocks.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The S3 bucket name and file name that identifies the document.</p> <p>The AWS Region for the S3 bucket that contains the document must match the Region that you use for Amazon Textract operations.</p> <p>For Amazon Textract to process a file in an S3 bucket, the user must have permission to access the S3 bucket and file. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3Object {
    /// <p>The name of the S3 bucket.</p>
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>The file name of the input document. Synchronous operations can use image files that are in JPEG or PNG format. Asynchronous operations also support PDF format files.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If the bucket has versioning enabled, you can specify the object version. </p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDocumentAnalysisRequest {
    /// <p>The idempotent token that you use to identify the start request. If you use the same token with multiple <code>StartDocumentAnalysis</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidentally started more than once. For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/api-async.html">Calling Amazon Textract Asynchronous Operations</a>.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The location of the document to be processed.</p>
    #[serde(rename = "DocumentLocation")]
    pub document_location: DocumentLocation,
    /// <p>A list of the types of analysis to perform. Add TABLES to the list to return information about the tables that are detected in the input document. Add FORMS to return detected form data. To perform both types of analysis, add TABLES and FORMS to <code>FeatureTypes</code>. All lines and words detected in the document are included in the response (including text that isn't related to the value of <code>FeatureTypes</code>). </p>
    #[serde(rename = "FeatureTypes")]
    pub feature_types: Vec<String>,
    /// <p>An identifier that you specify that's included in the completion notification published to the Amazon SNS topic. For example, you can use <code>JobTag</code> to identify the type of document that the completion notification corresponds to (such as a tax form or a receipt).</p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The Amazon SNS topic ARN that you want Amazon Textract to publish the completion status of the operation to. </p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDocumentAnalysisResponse {
    /// <p>The identifier for the document text detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetDocumentAnalysis</code>. A <code>JobId</code> value is only valid for 7 days.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDocumentTextDetectionRequest {
    /// <p>The idempotent token that's used to identify the start request. If you use the same token with multiple <code>StartDocumentTextDetection</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidentally started more than once. For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/api-async.html">Calling Amazon Textract Asynchronous Operations</a>.</p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The location of the document to be processed.</p>
    #[serde(rename = "DocumentLocation")]
    pub document_location: DocumentLocation,
    /// <p>An identifier that you specify that's included in the completion notification published to the Amazon SNS topic. For example, you can use <code>JobTag</code> to identify the type of document that the completion notification corresponds to (such as a tax form or a receipt).</p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The Amazon SNS topic ARN that you want Amazon Textract to publish the completion status of the operation to. </p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDocumentTextDetectionResponse {
    /// <p>The identifier of the text detection job for the document. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetDocumentTextDetection</code>. A <code>JobId</code> value is only valid for 7 days.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>A warning about an issue that occurred during asynchronous text analysis (<a>StartDocumentAnalysis</a>) or asynchronous document text detection (<a>StartDocumentTextDetection</a>). </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Warning {
    /// <p>The error code for the warning.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>A list of the pages that the warning applies to.</p>
    #[serde(rename = "Pages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<i64>>,
}

/// Errors returned by AnalyzeDocument
#[derive(Debug, PartialEq)]
pub enum AnalyzeDocumentError {
    /// <p>You aren't authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Textract isn't able to read the document.</p>
    BadDocument(String),
    /// <p>The document can't be processed because it's too large. The maximum document size for synchronous operations 5 MB. The maximum document size for asynchronous operations is 500 MB for PDF files.</p>
    DocumentTooLarge(String),
    /// <p>Indicates you have exceeded the maximum number of active human in the loop workflows available</p>
    HumanLoopQuotaExceeded(String),
    /// <p>Amazon Textract experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>An input parameter violated a constraint. For example, in synchronous operations, an <code>InvalidParameterException</code> exception occurs when neither of the <code>S3Object</code> or <code>Bytes</code> values are supplied in the <code>Document</code> request parameter. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Textract is unable to access the S3 object that's specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Textract.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Textract is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The format of the input document isn't supported. Documents for synchronous operations can be in PNG or JPEG format. Documents for asynchronous operations can also be in PDF format.</p>
    UnsupportedDocument(String),
}

impl AnalyzeDocumentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AnalyzeDocumentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AnalyzeDocumentError::AccessDenied(err.msg))
                }
                "BadDocumentException" => {
                    return RusotoError::Service(AnalyzeDocumentError::BadDocument(err.msg))
                }
                "DocumentTooLargeException" => {
                    return RusotoError::Service(AnalyzeDocumentError::DocumentTooLarge(err.msg))
                }
                "HumanLoopQuotaExceededException" => {
                    return RusotoError::Service(AnalyzeDocumentError::HumanLoopQuotaExceeded(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(AnalyzeDocumentError::InternalServerError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(AnalyzeDocumentError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(AnalyzeDocumentError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        AnalyzeDocumentError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(AnalyzeDocumentError::Throttling(err.msg))
                }
                "UnsupportedDocumentException" => {
                    return RusotoError::Service(AnalyzeDocumentError::UnsupportedDocument(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AnalyzeDocumentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnalyzeDocumentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AnalyzeDocumentError::BadDocument(ref cause) => write!(f, "{}", cause),
            AnalyzeDocumentError::DocumentTooLarge(ref cause) => write!(f, "{}", cause),
            AnalyzeDocumentError::HumanLoopQuotaExceeded(ref cause) => write!(f, "{}", cause),
            AnalyzeDocumentError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AnalyzeDocumentError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            AnalyzeDocumentError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            AnalyzeDocumentError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AnalyzeDocumentError::Throttling(ref cause) => write!(f, "{}", cause),
            AnalyzeDocumentError::UnsupportedDocument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AnalyzeDocumentError {}
/// Errors returned by DetectDocumentText
#[derive(Debug, PartialEq)]
pub enum DetectDocumentTextError {
    /// <p>You aren't authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Textract isn't able to read the document.</p>
    BadDocument(String),
    /// <p>The document can't be processed because it's too large. The maximum document size for synchronous operations 5 MB. The maximum document size for asynchronous operations is 500 MB for PDF files.</p>
    DocumentTooLarge(String),
    /// <p>Amazon Textract experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>An input parameter violated a constraint. For example, in synchronous operations, an <code>InvalidParameterException</code> exception occurs when neither of the <code>S3Object</code> or <code>Bytes</code> values are supplied in the <code>Document</code> request parameter. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Textract is unable to access the S3 object that's specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Textract.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Textract is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The format of the input document isn't supported. Documents for synchronous operations can be in PNG or JPEG format. Documents for asynchronous operations can also be in PDF format.</p>
    UnsupportedDocument(String),
}

impl DetectDocumentTextError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectDocumentTextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectDocumentTextError::AccessDenied(err.msg))
                }
                "BadDocumentException" => {
                    return RusotoError::Service(DetectDocumentTextError::BadDocument(err.msg))
                }
                "DocumentTooLargeException" => {
                    return RusotoError::Service(DetectDocumentTextError::DocumentTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectDocumentTextError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectDocumentTextError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectDocumentTextError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DetectDocumentTextError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectDocumentTextError::Throttling(err.msg))
                }
                "UnsupportedDocumentException" => {
                    return RusotoError::Service(DetectDocumentTextError::UnsupportedDocument(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetectDocumentTextError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectDocumentTextError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetectDocumentTextError::BadDocument(ref cause) => write!(f, "{}", cause),
            DetectDocumentTextError::DocumentTooLarge(ref cause) => write!(f, "{}", cause),
            DetectDocumentTextError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DetectDocumentTextError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DetectDocumentTextError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            DetectDocumentTextError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DetectDocumentTextError::Throttling(ref cause) => write!(f, "{}", cause),
            DetectDocumentTextError::UnsupportedDocument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectDocumentTextError {}
/// Errors returned by GetDocumentAnalysis
#[derive(Debug, PartialEq)]
pub enum GetDocumentAnalysisError {
    /// <p>You aren't authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Textract experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>An invalid job identifier was passed to <a>GetDocumentAnalysis</a> or to <a>GetDocumentAnalysis</a>.</p>
    InvalidJobId(String),
    /// <p>An input parameter violated a constraint. For example, in synchronous operations, an <code>InvalidParameterException</code> exception occurs when neither of the <code>S3Object</code> or <code>Bytes</code> values are supplied in the <code>Document</code> request parameter. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Textract.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Textract is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetDocumentAnalysisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentAnalysisError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDocumentAnalysisError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetDocumentAnalysisError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidJobIdException" => {
                    return RusotoError::Service(GetDocumentAnalysisError::InvalidJobId(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDocumentAnalysisError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetDocumentAnalysisError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDocumentAnalysisError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDocumentAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDocumentAnalysisError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDocumentAnalysisError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetDocumentAnalysisError::InvalidJobId(ref cause) => write!(f, "{}", cause),
            GetDocumentAnalysisError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetDocumentAnalysisError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDocumentAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDocumentAnalysisError {}
/// Errors returned by GetDocumentTextDetection
#[derive(Debug, PartialEq)]
pub enum GetDocumentTextDetectionError {
    /// <p>You aren't authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Textract experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>An invalid job identifier was passed to <a>GetDocumentAnalysis</a> or to <a>GetDocumentAnalysis</a>.</p>
    InvalidJobId(String),
    /// <p>An input parameter violated a constraint. For example, in synchronous operations, an <code>InvalidParameterException</code> exception occurs when neither of the <code>S3Object</code> or <code>Bytes</code> values are supplied in the <code>Document</code> request parameter. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Textract.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Textract is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetDocumentTextDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentTextDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDocumentTextDetectionError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        GetDocumentTextDetectionError::InternalServerError(err.msg),
                    )
                }
                "InvalidJobIdException" => {
                    return RusotoError::Service(GetDocumentTextDetectionError::InvalidJobId(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDocumentTextDetectionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetDocumentTextDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDocumentTextDetectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDocumentTextDetectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDocumentTextDetectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetDocumentTextDetectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetDocumentTextDetectionError::InvalidJobId(ref cause) => write!(f, "{}", cause),
            GetDocumentTextDetectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetDocumentTextDetectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDocumentTextDetectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDocumentTextDetectionError {}
/// Errors returned by StartDocumentAnalysis
#[derive(Debug, PartialEq)]
pub enum StartDocumentAnalysisError {
    /// <p>You aren't authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Textract isn't able to read the document.</p>
    BadDocument(String),
    /// <p>The document can't be processed because it's too large. The maximum document size for synchronous operations 5 MB. The maximum document size for asynchronous operations is 500 MB for PDF files.</p>
    DocumentTooLarge(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation. </p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Textract experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>An input parameter violated a constraint. For example, in synchronous operations, an <code>InvalidParameterException</code> exception occurs when neither of the <code>S3Object</code> or <code>Bytes</code> values are supplied in the <code>Document</code> request parameter. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Textract is unable to access the S3 object that's specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Textract service limit was exceeded. For example, if you start too many asynchronous jobs concurrently, calls to start operations (<code>StartDocumentTextDetection</code>, for example) raise a LimitExceededException exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Textract service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Textract.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Textract is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The format of the input document isn't supported. Documents for synchronous operations can be in PNG or JPEG format. Documents for asynchronous operations can also be in PDF format.</p>
    UnsupportedDocument(String),
}

impl StartDocumentAnalysisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartDocumentAnalysisError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartDocumentAnalysisError::AccessDenied(err.msg))
                }
                "BadDocumentException" => {
                    return RusotoError::Service(StartDocumentAnalysisError::BadDocument(err.msg))
                }
                "DocumentTooLargeException" => {
                    return RusotoError::Service(StartDocumentAnalysisError::DocumentTooLarge(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartDocumentAnalysisError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartDocumentAnalysisError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartDocumentAnalysisError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartDocumentAnalysisError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartDocumentAnalysisError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartDocumentAnalysisError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartDocumentAnalysisError::Throttling(err.msg))
                }
                "UnsupportedDocumentException" => {
                    return RusotoError::Service(StartDocumentAnalysisError::UnsupportedDocument(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartDocumentAnalysisError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDocumentAnalysisError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartDocumentAnalysisError::BadDocument(ref cause) => write!(f, "{}", cause),
            StartDocumentAnalysisError::DocumentTooLarge(ref cause) => write!(f, "{}", cause),
            StartDocumentAnalysisError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentAnalysisError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartDocumentAnalysisError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartDocumentAnalysisError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            StartDocumentAnalysisError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartDocumentAnalysisError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentAnalysisError::Throttling(ref cause) => write!(f, "{}", cause),
            StartDocumentAnalysisError::UnsupportedDocument(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartDocumentAnalysisError {}
/// Errors returned by StartDocumentTextDetection
#[derive(Debug, PartialEq)]
pub enum StartDocumentTextDetectionError {
    /// <p>You aren't authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Textract isn't able to read the document.</p>
    BadDocument(String),
    /// <p>The document can't be processed because it's too large. The maximum document size for synchronous operations 5 MB. The maximum document size for asynchronous operations is 500 MB for PDF files.</p>
    DocumentTooLarge(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation. </p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Textract experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>An input parameter violated a constraint. For example, in synchronous operations, an <code>InvalidParameterException</code> exception occurs when neither of the <code>S3Object</code> or <code>Bytes</code> values are supplied in the <code>Document</code> request parameter. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Textract is unable to access the S3 object that's specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Textract service limit was exceeded. For example, if you start too many asynchronous jobs concurrently, calls to start operations (<code>StartDocumentTextDetection</code>, for example) raise a LimitExceededException exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Textract service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Textract.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Textract is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The format of the input document isn't supported. Documents for synchronous operations can be in PNG or JPEG format. Documents for asynchronous operations can also be in PDF format.</p>
    UnsupportedDocument(String),
}

impl StartDocumentTextDetectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartDocumentTextDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartDocumentTextDetectionError::AccessDenied(
                        err.msg,
                    ))
                }
                "BadDocumentException" => {
                    return RusotoError::Service(StartDocumentTextDetectionError::BadDocument(
                        err.msg,
                    ))
                }
                "DocumentTooLargeException" => {
                    return RusotoError::Service(StartDocumentTextDetectionError::DocumentTooLarge(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartDocumentTextDetectionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        StartDocumentTextDetectionError::InternalServerError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartDocumentTextDetectionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartDocumentTextDetectionError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartDocumentTextDetectionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartDocumentTextDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartDocumentTextDetectionError::Throttling(
                        err.msg,
                    ))
                }
                "UnsupportedDocumentException" => {
                    return RusotoError::Service(
                        StartDocumentTextDetectionError::UnsupportedDocument(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartDocumentTextDetectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDocumentTextDetectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartDocumentTextDetectionError::BadDocument(ref cause) => write!(f, "{}", cause),
            StartDocumentTextDetectionError::DocumentTooLarge(ref cause) => write!(f, "{}", cause),
            StartDocumentTextDetectionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentTextDetectionError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentTextDetectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartDocumentTextDetectionError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            StartDocumentTextDetectionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartDocumentTextDetectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDocumentTextDetectionError::Throttling(ref cause) => write!(f, "{}", cause),
            StartDocumentTextDetectionError::UnsupportedDocument(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartDocumentTextDetectionError {}
/// Trait representing the capabilities of the Amazon Textract API. Amazon Textract clients implement this trait.
#[async_trait]
pub trait Textract {
    /// <p>Analyzes an input document for relationships between detected items. </p> <p>The types of information returned are as follows: </p> <ul> <li> <p>Form data (key-value pairs). The related information is returned in two <a>Block</a> objects, each of type <code>KEY_VALUE_SET</code>: a KEY <code>Block</code> object and a VALUE <code>Block</code> object. For example, <i>Name: Ana Silva Carolina</i> contains a key and value. <i>Name:</i> is the key. <i>Ana Silva Carolina</i> is the value.</p> </li> <li> <p>Table and table cell data. A TABLE <code>Block</code> object contains information about a detected table. A CELL <code>Block</code> object is returned for each cell in a table.</p> </li> <li> <p>Lines and words of text. A LINE <code>Block</code> object contains one or more WORD <code>Block</code> objects. All lines and words that are detected in the document are returned (including text that doesn't have a relationship with the value of <code>FeatureTypes</code>). </p> </li> </ul> <p>Selection elements such as check boxes and option buttons (radio buttons) can be detected in form data and in tables. A SELECTION_ELEMENT <code>Block</code> object contains information about a selection element, including the selection status.</p> <p>You can choose which type of analysis to perform by specifying the <code>FeatureTypes</code> list. </p> <p>The output is returned in a list of <code>Block</code> objects.</p> <p> <code>AnalyzeDocument</code> is a synchronous operation. To analyze documents asynchronously, use <a>StartDocumentAnalysis</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-analyzing.html">Document Text Analysis</a>.</p>
    async fn analyze_document(
        &self,
        input: AnalyzeDocumentRequest,
    ) -> Result<AnalyzeDocumentResponse, RusotoError<AnalyzeDocumentError>>;

    /// <p>Detects text in the input document. Amazon Textract can detect lines of text and the words that make up a line of text. The input document must be an image in JPEG or PNG format. <code>DetectDocumentText</code> returns the detected text in an array of <a>Block</a> objects. </p> <p>Each document page has as an associated <code>Block</code> of type PAGE. Each PAGE <code>Block</code> object is the parent of LINE <code>Block</code> objects that represent the lines of detected text on a page. A LINE <code>Block</code> object is a parent for each word that makes up the line. Words are represented by <code>Block</code> objects of type WORD.</p> <p> <code>DetectDocumentText</code> is a synchronous operation. To analyze documents asynchronously, use <a>StartDocumentTextDetection</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-detecting.html">Document Text Detection</a>.</p>
    async fn detect_document_text(
        &self,
        input: DetectDocumentTextRequest,
    ) -> Result<DetectDocumentTextResponse, RusotoError<DetectDocumentTextError>>;

    /// <p>Gets the results for an Amazon Textract asynchronous operation that analyzes text in a document.</p> <p>You start asynchronous text analysis by calling <a>StartDocumentAnalysis</a>, which returns a job identifier (<code>JobId</code>). When the text analysis operation finishes, Amazon Textract publishes a completion status to the Amazon Simple Notification Service (Amazon SNS) topic that's registered in the initial call to <code>StartDocumentAnalysis</code>. To get the results of the text-detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetDocumentAnalysis</code>, and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartDocumentAnalysis</code>.</p> <p> <code>GetDocumentAnalysis</code> returns an array of <a>Block</a> objects. The following types of information are returned: </p> <ul> <li> <p>Form data (key-value pairs). The related information is returned in two <a>Block</a> objects, each of type <code>KEY_VALUE_SET</code>: a KEY <code>Block</code> object and a VALUE <code>Block</code> object. For example, <i>Name: Ana Silva Carolina</i> contains a key and value. <i>Name:</i> is the key. <i>Ana Silva Carolina</i> is the value.</p> </li> <li> <p>Table and table cell data. A TABLE <code>Block</code> object contains information about a detected table. A CELL <code>Block</code> object is returned for each cell in a table.</p> </li> <li> <p>Lines and words of text. A LINE <code>Block</code> object contains one or more WORD <code>Block</code> objects. All lines and words that are detected in the document are returned (including text that doesn't have a relationship with the value of the <code>StartDocumentAnalysis</code> <code>FeatureTypes</code> input parameter). </p> </li> </ul> <p>Selection elements such as check boxes and option buttons (radio buttons) can be detected in form data and in tables. A SELECTION_ELEMENT <code>Block</code> object contains information about a selection element, including the selection status.</p> <p>Use the <code>MaxResults</code> parameter to limit the number of blocks that are returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetDocumentAnalysis</code>, and populate the <code>NextToken</code> request parameter with the token value that's returned from the previous call to <code>GetDocumentAnalysis</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-analyzing.html">Document Text Analysis</a>.</p>
    async fn get_document_analysis(
        &self,
        input: GetDocumentAnalysisRequest,
    ) -> Result<GetDocumentAnalysisResponse, RusotoError<GetDocumentAnalysisError>>;

    /// <p>Gets the results for an Amazon Textract asynchronous operation that detects text in a document. Amazon Textract can detect lines of text and the words that make up a line of text.</p> <p>You start asynchronous text detection by calling <a>StartDocumentTextDetection</a>, which returns a job identifier (<code>JobId</code>). When the text detection operation finishes, Amazon Textract publishes a completion status to the Amazon Simple Notification Service (Amazon SNS) topic that's registered in the initial call to <code>StartDocumentTextDetection</code>. To get the results of the text-detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetDocumentTextDetection</code>, and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartDocumentTextDetection</code>.</p> <p> <code>GetDocumentTextDetection</code> returns an array of <a>Block</a> objects. </p> <p>Each document page has as an associated <code>Block</code> of type PAGE. Each PAGE <code>Block</code> object is the parent of LINE <code>Block</code> objects that represent the lines of detected text on a page. A LINE <code>Block</code> object is a parent for each word that makes up the line. Words are represented by <code>Block</code> objects of type WORD.</p> <p>Use the MaxResults parameter to limit the number of blocks that are returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetDocumentTextDetection</code>, and populate the <code>NextToken</code> request parameter with the token value that's returned from the previous call to <code>GetDocumentTextDetection</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-detecting.html">Document Text Detection</a>.</p>
    async fn get_document_text_detection(
        &self,
        input: GetDocumentTextDetectionRequest,
    ) -> Result<GetDocumentTextDetectionResponse, RusotoError<GetDocumentTextDetectionError>>;

    /// <p>Starts the asynchronous analysis of an input document for relationships between detected items such as key-value pairs, tables, and selection elements.</p> <p> <code>StartDocumentAnalysis</code> can analyze text in documents that are in JPEG, PNG, and PDF format. The documents are stored in an Amazon S3 bucket. Use <a>DocumentLocation</a> to specify the bucket name and file name of the document. </p> <p> <code>StartDocumentAnalysis</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When text analysis is finished, Amazon Textract publishes a completion status to the Amazon Simple Notification Service (Amazon SNS) topic that you specify in <code>NotificationChannel</code>. To get the results of the text analysis operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetDocumentAnalysis</a>, and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartDocumentAnalysis</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-analyzing.html">Document Text Analysis</a>.</p>
    async fn start_document_analysis(
        &self,
        input: StartDocumentAnalysisRequest,
    ) -> Result<StartDocumentAnalysisResponse, RusotoError<StartDocumentAnalysisError>>;

    /// <p>Starts the asynchronous detection of text in a document. Amazon Textract can detect lines of text and the words that make up a line of text.</p> <p> <code>StartDocumentTextDetection</code> can analyze text in documents that are in JPEG, PNG, and PDF format. The documents are stored in an Amazon S3 bucket. Use <a>DocumentLocation</a> to specify the bucket name and file name of the document. </p> <p> <code>StartTextDetection</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When text detection is finished, Amazon Textract publishes a completion status to the Amazon Simple Notification Service (Amazon SNS) topic that you specify in <code>NotificationChannel</code>. To get the results of the text detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetDocumentTextDetection</a>, and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartDocumentTextDetection</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-detecting.html">Document Text Detection</a>.</p>
    async fn start_document_text_detection(
        &self,
        input: StartDocumentTextDetectionRequest,
    ) -> Result<StartDocumentTextDetectionResponse, RusotoError<StartDocumentTextDetectionError>>;
}
/// A client for the Amazon Textract API.
#[derive(Clone)]
pub struct TextractClient {
    client: Client,
    region: region::Region,
}

impl TextractClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> TextractClient {
        TextractClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> TextractClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        TextractClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> TextractClient {
        TextractClient { client, region }
    }
}

#[async_trait]
impl Textract for TextractClient {
    /// <p>Analyzes an input document for relationships between detected items. </p> <p>The types of information returned are as follows: </p> <ul> <li> <p>Form data (key-value pairs). The related information is returned in two <a>Block</a> objects, each of type <code>KEY_VALUE_SET</code>: a KEY <code>Block</code> object and a VALUE <code>Block</code> object. For example, <i>Name: Ana Silva Carolina</i> contains a key and value. <i>Name:</i> is the key. <i>Ana Silva Carolina</i> is the value.</p> </li> <li> <p>Table and table cell data. A TABLE <code>Block</code> object contains information about a detected table. A CELL <code>Block</code> object is returned for each cell in a table.</p> </li> <li> <p>Lines and words of text. A LINE <code>Block</code> object contains one or more WORD <code>Block</code> objects. All lines and words that are detected in the document are returned (including text that doesn't have a relationship with the value of <code>FeatureTypes</code>). </p> </li> </ul> <p>Selection elements such as check boxes and option buttons (radio buttons) can be detected in form data and in tables. A SELECTION_ELEMENT <code>Block</code> object contains information about a selection element, including the selection status.</p> <p>You can choose which type of analysis to perform by specifying the <code>FeatureTypes</code> list. </p> <p>The output is returned in a list of <code>Block</code> objects.</p> <p> <code>AnalyzeDocument</code> is a synchronous operation. To analyze documents asynchronously, use <a>StartDocumentAnalysis</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-analyzing.html">Document Text Analysis</a>.</p>
    async fn analyze_document(
        &self,
        input: AnalyzeDocumentRequest,
    ) -> Result<AnalyzeDocumentResponse, RusotoError<AnalyzeDocumentError>> {
        let mut request = SignedRequest::new("POST", "textract", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Textract.AnalyzeDocument");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<AnalyzeDocumentResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AnalyzeDocumentError::from_response(response))
        }
    }

    /// <p>Detects text in the input document. Amazon Textract can detect lines of text and the words that make up a line of text. The input document must be an image in JPEG or PNG format. <code>DetectDocumentText</code> returns the detected text in an array of <a>Block</a> objects. </p> <p>Each document page has as an associated <code>Block</code> of type PAGE. Each PAGE <code>Block</code> object is the parent of LINE <code>Block</code> objects that represent the lines of detected text on a page. A LINE <code>Block</code> object is a parent for each word that makes up the line. Words are represented by <code>Block</code> objects of type WORD.</p> <p> <code>DetectDocumentText</code> is a synchronous operation. To analyze documents asynchronously, use <a>StartDocumentTextDetection</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-detecting.html">Document Text Detection</a>.</p>
    async fn detect_document_text(
        &self,
        input: DetectDocumentTextRequest,
    ) -> Result<DetectDocumentTextResponse, RusotoError<DetectDocumentTextError>> {
        let mut request = SignedRequest::new("POST", "textract", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Textract.DetectDocumentText");
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
                .deserialize::<DetectDocumentTextResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectDocumentTextError::from_response(response))
        }
    }

    /// <p>Gets the results for an Amazon Textract asynchronous operation that analyzes text in a document.</p> <p>You start asynchronous text analysis by calling <a>StartDocumentAnalysis</a>, which returns a job identifier (<code>JobId</code>). When the text analysis operation finishes, Amazon Textract publishes a completion status to the Amazon Simple Notification Service (Amazon SNS) topic that's registered in the initial call to <code>StartDocumentAnalysis</code>. To get the results of the text-detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetDocumentAnalysis</code>, and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartDocumentAnalysis</code>.</p> <p> <code>GetDocumentAnalysis</code> returns an array of <a>Block</a> objects. The following types of information are returned: </p> <ul> <li> <p>Form data (key-value pairs). The related information is returned in two <a>Block</a> objects, each of type <code>KEY_VALUE_SET</code>: a KEY <code>Block</code> object and a VALUE <code>Block</code> object. For example, <i>Name: Ana Silva Carolina</i> contains a key and value. <i>Name:</i> is the key. <i>Ana Silva Carolina</i> is the value.</p> </li> <li> <p>Table and table cell data. A TABLE <code>Block</code> object contains information about a detected table. A CELL <code>Block</code> object is returned for each cell in a table.</p> </li> <li> <p>Lines and words of text. A LINE <code>Block</code> object contains one or more WORD <code>Block</code> objects. All lines and words that are detected in the document are returned (including text that doesn't have a relationship with the value of the <code>StartDocumentAnalysis</code> <code>FeatureTypes</code> input parameter). </p> </li> </ul> <p>Selection elements such as check boxes and option buttons (radio buttons) can be detected in form data and in tables. A SELECTION_ELEMENT <code>Block</code> object contains information about a selection element, including the selection status.</p> <p>Use the <code>MaxResults</code> parameter to limit the number of blocks that are returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetDocumentAnalysis</code>, and populate the <code>NextToken</code> request parameter with the token value that's returned from the previous call to <code>GetDocumentAnalysis</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-analyzing.html">Document Text Analysis</a>.</p>
    async fn get_document_analysis(
        &self,
        input: GetDocumentAnalysisRequest,
    ) -> Result<GetDocumentAnalysisResponse, RusotoError<GetDocumentAnalysisError>> {
        let mut request = SignedRequest::new("POST", "textract", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Textract.GetDocumentAnalysis");
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
                .deserialize::<GetDocumentAnalysisResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDocumentAnalysisError::from_response(response))
        }
    }

    /// <p>Gets the results for an Amazon Textract asynchronous operation that detects text in a document. Amazon Textract can detect lines of text and the words that make up a line of text.</p> <p>You start asynchronous text detection by calling <a>StartDocumentTextDetection</a>, which returns a job identifier (<code>JobId</code>). When the text detection operation finishes, Amazon Textract publishes a completion status to the Amazon Simple Notification Service (Amazon SNS) topic that's registered in the initial call to <code>StartDocumentTextDetection</code>. To get the results of the text-detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetDocumentTextDetection</code>, and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartDocumentTextDetection</code>.</p> <p> <code>GetDocumentTextDetection</code> returns an array of <a>Block</a> objects. </p> <p>Each document page has as an associated <code>Block</code> of type PAGE. Each PAGE <code>Block</code> object is the parent of LINE <code>Block</code> objects that represent the lines of detected text on a page. A LINE <code>Block</code> object is a parent for each word that makes up the line. Words are represented by <code>Block</code> objects of type WORD.</p> <p>Use the MaxResults parameter to limit the number of blocks that are returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetDocumentTextDetection</code>, and populate the <code>NextToken</code> request parameter with the token value that's returned from the previous call to <code>GetDocumentTextDetection</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-detecting.html">Document Text Detection</a>.</p>
    async fn get_document_text_detection(
        &self,
        input: GetDocumentTextDetectionRequest,
    ) -> Result<GetDocumentTextDetectionResponse, RusotoError<GetDocumentTextDetectionError>> {
        let mut request = SignedRequest::new("POST", "textract", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Textract.GetDocumentTextDetection");
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
                .deserialize::<GetDocumentTextDetectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDocumentTextDetectionError::from_response(response))
        }
    }

    /// <p>Starts the asynchronous analysis of an input document for relationships between detected items such as key-value pairs, tables, and selection elements.</p> <p> <code>StartDocumentAnalysis</code> can analyze text in documents that are in JPEG, PNG, and PDF format. The documents are stored in an Amazon S3 bucket. Use <a>DocumentLocation</a> to specify the bucket name and file name of the document. </p> <p> <code>StartDocumentAnalysis</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When text analysis is finished, Amazon Textract publishes a completion status to the Amazon Simple Notification Service (Amazon SNS) topic that you specify in <code>NotificationChannel</code>. To get the results of the text analysis operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetDocumentAnalysis</a>, and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartDocumentAnalysis</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-analyzing.html">Document Text Analysis</a>.</p>
    async fn start_document_analysis(
        &self,
        input: StartDocumentAnalysisRequest,
    ) -> Result<StartDocumentAnalysisResponse, RusotoError<StartDocumentAnalysisError>> {
        let mut request = SignedRequest::new("POST", "textract", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Textract.StartDocumentAnalysis");
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
                .deserialize::<StartDocumentAnalysisResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartDocumentAnalysisError::from_response(response))
        }
    }

    /// <p>Starts the asynchronous detection of text in a document. Amazon Textract can detect lines of text and the words that make up a line of text.</p> <p> <code>StartDocumentTextDetection</code> can analyze text in documents that are in JPEG, PNG, and PDF format. The documents are stored in an Amazon S3 bucket. Use <a>DocumentLocation</a> to specify the bucket name and file name of the document. </p> <p> <code>StartTextDetection</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When text detection is finished, Amazon Textract publishes a completion status to the Amazon Simple Notification Service (Amazon SNS) topic that you specify in <code>NotificationChannel</code>. To get the results of the text detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetDocumentTextDetection</a>, and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartDocumentTextDetection</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/textract/latest/dg/how-it-works-detecting.html">Document Text Detection</a>.</p>
    async fn start_document_text_detection(
        &self,
        input: StartDocumentTextDetectionRequest,
    ) -> Result<StartDocumentTextDetectionResponse, RusotoError<StartDocumentTextDetectionError>>
    {
        let mut request = SignedRequest::new("POST", "textract", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Textract.StartDocumentTextDetection");
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
                .deserialize::<StartDocumentTextDetectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartDocumentTextDetectionError::from_response(response))
        }
    }
}
