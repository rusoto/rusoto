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
/// <p>Structure containing the estimated age range, in years, for a face.</p> <p>Amazon Rekognition estimates an age range for faces detected in the input image. Estimated age ranges can overlap. A face of a 5-year-old might have an estimated range of 4-6, while the face of a 6-year-old might have an estimated range of 4-8.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgeRange {
    /// <p>The highest estimated age.</p>
    #[serde(rename = "High")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    /// <p>The lowest estimated age.</p>
    #[serde(rename = "Low")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<i64>,
}

/// <p>Assets are the images that you use to train and evaluate a model version. Assets are referenced by Sagemaker GroundTruth manifest files. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "GroundTruthManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_truth_manifest: Option<GroundTruthManifest>,
}

/// <p>Indicates whether or not the face has a beard, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Beard {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face has beard or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p><p>Identifies the bounding box around the label, face, or text. The <code>left</code> (x-coordinate) and <code>top</code> (y-coordinate) are coordinates representing the top and left sides of the bounding box. Note that the upper-left corner of the image is the origin (0,0). </p> <p>The <code>top</code> and <code>left</code> values returned are ratios of the overall image size. For example, if the input image is 700x200 pixels, and the top-left coordinate of the bounding box is 350x50 pixels, the API returns a <code>left</code> value of 0.5 (350/700) and a <code>top</code> value of 0.25 (50/200).</p> <p>The <code>width</code> and <code>height</code> values represent the dimensions of the bounding box as a ratio of the overall image dimension. For example, if the input image is 700x200 pixels, and the bounding box width is 70 pixels, the width returned is 0.1. </p> <note> <p> The bounding box coordinates can have negative values. For example, if Amazon Rekognition is able to detect a face that is at the image edge and is only partially visible, the service can return coordinates that are outside the image bounds and, depending on the image edge, you might get negative values or values greater than 1 for the <code>left</code> or <code>top</code> values. </p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BoundingBox {
    /// <p>Height of the bounding box as a ratio of the overall image height.</p>
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    /// <p>Left coordinate of the bounding box as a ratio of overall image width.</p>
    #[serde(rename = "Left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f32>,
    /// <p>Top coordinate of the bounding box as a ratio of overall image height.</p>
    #[serde(rename = "Top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f32>,
    /// <p>Width of the bounding box as a ratio of the overall image width.</p>
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
}

/// <p>Provides information about a celebrity recognized by the <a>RecognizeCelebrities</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Celebrity {
    /// <p>Provides information about the celebrity's face, such as its location on the image.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<ComparedFace>,
    /// <p>A unique identifier for the celebrity. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The confidence, in percentage, that Amazon Rekognition has that the recognized face is the celebrity.</p>
    #[serde(rename = "MatchConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_confidence: Option<f32>,
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional information about the celebrity. If there is no additional information about the celebrity, this list is empty.</p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// <p>Information about a recognized celebrity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CelebrityDetail {
    /// <p>Bounding box around the body of a celebrity.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>The confidence, in percentage, that Amazon Rekognition has that the recognized face is the celebrity. </p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Face details for the recognized celebrity.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>The unique identifier for the celebrity. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional celebrity information. </p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// <p>Information about a detected celebrity and the time the celebrity was detected in a stored video. For more information, see GetCelebrityRecognition in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CelebrityRecognition {
    /// <p>Information about a recognized celebrity.</p>
    #[serde(rename = "Celebrity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrity: Option<CelebrityDetail>,
    /// <p>The time, in milliseconds from the start of the video, that the celebrity was recognized.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Provides information about a face in a target image that matches the source image face analyzed by <code>CompareFaces</code>. The <code>Face</code> property contains the bounding box of the face in the target image. The <code>Similarity</code> property is the confidence that the source image face matches the face in the bounding box.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompareFacesMatch {
    /// <p>Provides face metadata (bounding box and confidence that the bounding box actually contains a face).</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<ComparedFace>,
    /// <p>Level of confidence that the faces match.</p>
    #[serde(rename = "Similarity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CompareFacesRequest {
    /// <p>A filter that specifies a quality bar for how much filtering is done to identify faces. Filtered faces aren't compared. If you specify <code>AUTO</code>, Amazon Rekognition chooses the quality bar. If you specify <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>, filtering removes all faces that don’t meet the chosen quality bar. The quality bar is based on a variety of common use cases. Low-quality detections can occur for a number of reasons. Some examples are an object that's misidentified as a face, a face that's too blurry, or a face with a pose that's too extreme to use. If you specify <code>NONE</code>, no filtering is performed. The default value is <code>NONE</code>. </p> <p>To use quality filtering, the collection you are using must be associated with version 3 of the face model or higher.</p>
    #[serde(rename = "QualityFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_filter: Option<String>,
    /// <p>The minimum level of confidence in the face matches that a match must meet to be included in the <code>FaceMatches</code> array.</p>
    #[serde(rename = "SimilarityThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_threshold: Option<f32>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "SourceImage")]
    pub source_image: Image,
    /// <p>The target image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "TargetImage")]
    pub target_image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompareFacesResponse {
    /// <p>An array of faces in the target image that match the source image face. Each <code>CompareFacesMatch</code> object provides the bounding box, the confidence level that the bounding box contains a face, and the similarity score for the face in the bounding box and the face in the source image.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<CompareFacesMatch>>,
    /// <p>The face in the source image that was used for comparison.</p>
    #[serde(rename = "SourceImageFace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image_face: Option<ComparedSourceImageFace>,
    /// <p>The value of <code>SourceImageOrientationCorrection</code> is always null.</p> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
    #[serde(rename = "SourceImageOrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image_orientation_correction: Option<String>,
    /// <p>The value of <code>TargetImageOrientationCorrection</code> is always null.</p> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
    #[serde(rename = "TargetImageOrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_image_orientation_correction: Option<String>,
    /// <p>An array of faces in the target image that did not match the source image face.</p>
    #[serde(rename = "UnmatchedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmatched_faces: Option<Vec<ComparedFace>>,
}

/// <p>Provides face metadata for target image faces that are analyzed by <code>CompareFaces</code> and <code>RecognizeCelebrities</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComparedFace {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Level of confidence that what the bounding box contains is a face.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>An array of facial landmarks.</p>
    #[serde(rename = "Landmarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<Vec<Landmark>>,
    /// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>
    #[serde(rename = "Pose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    /// <p>Identifies face image brightness and sharpness. </p>
    #[serde(rename = "Quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
}

/// <p>Type that describes the face Amazon Rekognition chose to compare with the faces in the target. This contains a bounding box for the selected face and confidence level that the bounding box contains a face. Note that Amazon Rekognition selects the largest face in the source image for this comparison. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComparedSourceImageFace {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the selected bounding box contains a face.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// <p>Information about an unsafe content label detection in a stored video.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContentModerationDetection {
    /// <p>The unsafe content label detected by in the stored video.</p>
    #[serde(rename = "ModerationLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_label: Option<ModerationLabel>,
    /// <p>Time, in milliseconds from the beginning of the video, that the unsafe content label was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCollectionRequest {
    /// <p>ID for the collection that you are creating.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCollectionResponse {
    /// <p>Amazon Resource Name (ARN) of the collection. You can use this to manage permissions on your resources. </p>
    #[serde(rename = "CollectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_arn: Option<String>,
    /// <p>Version number of the face detection model associated with the collection you are creating.</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>HTTP status code indicating the result of the operation.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProjectRequest {
    /// <p>The name of the project to create.</p>
    #[serde(rename = "ProjectName")]
    pub project_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProjectResponse {
    /// <p>The Amazon Resource Name (ARN) of the new project. You can use the ARN to configure IAM access to the project. </p>
    #[serde(rename = "ProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateProjectVersionRequest {
    /// <p>The Amazon S3 location to store the results of training.</p>
    #[serde(rename = "OutputConfig")]
    pub output_config: OutputConfig,
    /// <p>The ARN of the Amazon Rekognition Custom Labels project that manages the model that you want to train.</p>
    #[serde(rename = "ProjectArn")]
    pub project_arn: String,
    /// <p>The dataset to use for testing.</p>
    #[serde(rename = "TestingData")]
    pub testing_data: TestingData,
    /// <p>The dataset to use for training. </p>
    #[serde(rename = "TrainingData")]
    pub training_data: TrainingData,
    /// <p>A name for the version of the model. This value must be unique.</p>
    #[serde(rename = "VersionName")]
    pub version_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateProjectVersionResponse {
    /// <p>The ARN of the model version that was created. Use <code>DescribeProjectVersion</code> to get the current status of the training operation.</p>
    #[serde(rename = "ProjectVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStreamProcessorRequest {
    /// <p>Kinesis video stream stream that provides the source streaming video. If you are using the AWS CLI, the parameter name is <code>StreamProcessorInput</code>.</p>
    #[serde(rename = "Input")]
    pub input: StreamProcessorInput,
    /// <p>An identifier you assign to the stream processor. You can use <code>Name</code> to manage the stream processor. For example, you can get the current status of the stream processor by calling <a>DescribeStreamProcessor</a>. <code>Name</code> is idempotent. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Kinesis data stream stream to which Amazon Rekognition Video puts the analysis results. If you are using the AWS CLI, the parameter name is <code>StreamProcessorOutput</code>.</p>
    #[serde(rename = "Output")]
    pub output: StreamProcessorOutput,
    /// <p>ARN of the IAM role that allows access to the stream processor.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Face recognition input parameters to be used by the stream processor. Includes the collection to use for face recognition and the face attributes to detect.</p>
    #[serde(rename = "Settings")]
    pub settings: StreamProcessorSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateStreamProcessorResponse {
    /// <p>ARN for the newly create stream processor.</p>
    #[serde(rename = "StreamProcessorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processor_arn: Option<String>,
}

/// <p>A custom label detected in an image by a call to <a>DetectCustomLabels</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomLabel {
    /// <p>The confidence that the model has in the detection of the custom label. The range is 0-100. A higher value indicates a higher confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The location of the detected object on the image that corresponds to the custom label. Includes an axis aligned coarse bounding box surrounding the object and a finer grain polygon for more accurate spatial information.</p>
    #[serde(rename = "Geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    /// <p>The name of the custom label.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCollectionRequest {
    /// <p>ID of the collection to delete.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCollectionResponse {
    /// <p>HTTP status code that indicates the result of the operation.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFacesRequest {
    /// <p>Collection from which to remove the specific faces.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>An array of face IDs to delete.</p>
    #[serde(rename = "FaceIds")]
    pub face_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFacesResponse {
    /// <p>An array of strings (face IDs) of the faces that were deleted.</p>
    #[serde(rename = "DeletedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_faces: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteStreamProcessorRequest {
    /// <p>The name of the stream processor you want to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteStreamProcessorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCollectionRequest {
    /// <p>The ID of the collection to describe.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCollectionResponse {
    /// <p>The Amazon Resource Name (ARN) of the collection.</p>
    #[serde(rename = "CollectionARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_arn: Option<String>,
    /// <p>The number of milliseconds since the Unix epoch time until the creation of the collection. The Unix epoch time is 00:00:00 Coordinated Universal Time (UTC), Thursday, 1 January 1970.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    /// <p>The number of faces that are indexed into the collection. To index faces into a collection, use <a>IndexFaces</a>.</p>
    #[serde(rename = "FaceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_count: Option<i64>,
    /// <p>The version of the face model that's used by the collection for face detection.</p> <p>For more information, see Model Versioning in the Amazon Rekognition Developer Guide.</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProjectVersionsRequest {
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the project that contains the models you want to describe.</p>
    #[serde(rename = "ProjectArn")]
    pub project_arn: String,
    /// <p>A list of model version names that you want to describe. You can add up to 10 model version names to the list. If you don't specify a value, all model descriptions are returned.</p>
    #[serde(rename = "VersionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProjectVersionsResponse {
    /// <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of model descriptions. The list is sorted by the creation date and time of the model versions, latest to earliest.</p>
    #[serde(rename = "ProjectVersionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version_descriptions: Option<Vec<ProjectVersionDescription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProjectsRequest {
    /// <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProjectsResponse {
    /// <p>If the previous response was incomplete (because there is more results to retrieve), Amazon Rekognition Custom Labels returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of project descriptions. The list is sorted by the date and time the projects are created.</p>
    #[serde(rename = "ProjectDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_descriptions: Option<Vec<ProjectDescription>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStreamProcessorRequest {
    /// <p>Name of the stream processor for which you want information.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStreamProcessorResponse {
    /// <p>Date and time the stream processor was created</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    /// <p>Kinesis video stream that provides the source streaming video.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<StreamProcessorInput>,
    /// <p>The time, in Unix format, the stream processor was last updated. For example, when the stream processor moves from a running state to a failed state, or when the user starts or stops the stream processor.</p>
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    /// <p>Name of the stream processor. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Kinesis data stream to which Amazon Rekognition Video puts the analysis results.</p>
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<StreamProcessorOutput>,
    /// <p>ARN of the IAM role that allows access to the stream processor.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Face recognition input parameters that are being used by the stream processor. Includes the collection to use for face recognition and the face attributes to detect.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<StreamProcessorSettings>,
    /// <p>Current status of the stream processor.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Detailed status message about the stream processor.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>ARN of the stream processor.</p>
    #[serde(rename = "StreamProcessorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processor_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectCustomLabelsRequest {
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Maximum number of results you want the service to return in the response. The service returns the specified number of highest confidence labels ranked from highest confidence to lowest.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence lower than this specified value. If you specify a value of 0, all labels are return, regardless of the default thresholds that the model version applies.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    /// <p>The ARN of the model version that you want to use.</p>
    #[serde(rename = "ProjectVersionArn")]
    pub project_version_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectCustomLabelsResponse {
    /// <p>An array of custom labels detected in the input image.</p>
    #[serde(rename = "CustomLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<Vec<CustomLabel>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectFacesRequest {
    /// <p>An array of facial attributes you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>["DEFAULT"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code>, and <code>Landmarks</code>. If you provide <code>["ALL"]</code>, all facial attributes are returned, but the operation takes longer to complete.</p> <p>If you provide both, <code>["ALL", "DEFAULT"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectFacesResponse {
    /// <p>Details of each face found in the image. </p>
    #[serde(rename = "FaceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_details: Option<Vec<FaceDetail>>,
    /// <p>The value of <code>OrientationCorrection</code> is always null.</p> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectLabelsRequest {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. Images stored in an S3 Bucket do not need to be base64-encoded.</p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Maximum number of labels you want the service to return in the response. The service returns the specified number of highest confidence labels. </p>
    #[serde(rename = "MaxLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_labels: Option<i64>,
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with confidence lower than this specified value.</p> <p>If <code>MinConfidence</code> is not specified, the operation returns labels with a confidence values greater than or equal to 55 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectLabelsResponse {
    /// <p>Version number of the label detection model that was used to detect labels.</p>
    #[serde(rename = "LabelModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_model_version: Option<String>,
    /// <p>An array of labels for the real-world objects detected. </p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    /// <p>The value of <code>OrientationCorrection</code> is always null.</p> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectModerationLabelsRequest {
    /// <p>Sets up the configuration for human evaluation, including the FlowDefinition the image will be sent to.</p>
    #[serde(rename = "HumanLoopConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_config: Option<HumanLoopConfig>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence level lower than this specified value.</p> <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectModerationLabelsResponse {
    /// <p>Shows the results of the human in the loop evaluation.</p>
    #[serde(rename = "HumanLoopActivationOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_output: Option<HumanLoopActivationOutput>,
    /// <p>Array of detected Moderation labels and the time, in milliseconds from the start of the video, they were detected.</p>
    #[serde(rename = "ModerationLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_labels: Option<Vec<ModerationLabel>>,
    /// <p>Version number of the moderation detection model that was used to detect unsafe content.</p>
    #[serde(rename = "ModerationModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_model_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetectTextRequest {
    /// <p>The input image as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Rekognition operations, you can't pass image bytes. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetectTextResponse {
    /// <p>An array of text that was detected in the input image.</p>
    #[serde(rename = "TextDetections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_detections: Option<Vec<TextDetection>>,
}

/// <p>The emotions that appear to be expressed on the face, and the confidence level in the determination. The API is only making a determination of the physical appearance of a person's face. It is not a determination of the person’s internal emotional state and should not be used in such a way. For example, a person pretending to have a sad face might not be sad emotionally.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Emotion {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Type of emotion detected.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The evaluation results for the training of a model.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EvaluationResult {
    /// <p>The F1 score for the evaluation of all labels. The F1 score metric evaluates the overall precision and recall performance of the model as a single value. A higher value indicates better precision and recall performance. A lower score indicates that precision, recall, or both are performing poorly. </p>
    #[serde(rename = "F1Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f32>,
    /// <p>The S3 bucket that contains the training summary.</p>
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
}

/// <p>Indicates whether or not the eyes on the face are open, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EyeOpen {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the eyes on the face are open.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Indicates whether or not the face is wearing eye glasses, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Eyeglasses {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is wearing eye glasses or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Describes the face properties such as the bounding box, face ID, image ID of the input image, and external image ID that you assigned. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Face {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the bounding box contains a face (and not a different object such as a tree).</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Identifier that you assign to all the faces in the input image.</p>
    #[serde(rename = "ExternalImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_image_id: Option<String>,
    /// <p>Unique identifier that Amazon Rekognition assigns to the face.</p>
    #[serde(rename = "FaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
    /// <p>Unique identifier that Amazon Rekognition assigns to the input image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

/// <p>Structure containing attributes of the face that the algorithm detected.</p> <p>A <code>FaceDetail</code> object contains either the default facial attributes or all facial attributes. The default attributes are <code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>.</p> <p> <a>GetFaceDetection</a> is the only Amazon Rekognition Video stored video operation that can return a <code>FaceDetail</code> object with all attributes. To specify which attributes to return, use the <code>FaceAttributes</code> input parameter for <a>StartFaceDetection</a>. The following Amazon Rekognition Video operations return only the default attributes. The corresponding Start operations don't have a <code>FaceAttributes</code> input parameter.</p> <ul> <li> <p>GetCelebrityRecognition</p> </li> <li> <p>GetPersonTracking</p> </li> <li> <p>GetFaceSearch</p> </li> </ul> <p>The Amazon Rekognition Image <a>DetectFaces</a> and <a>IndexFaces</a> operations can return all facial attributes. To specify which attributes to return, use the <code>Attributes</code> input parameter for <code>DetectFaces</code>. For <code>IndexFaces</code>, use the <code>DetectAttributes</code> input parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaceDetail {
    /// <p>The estimated age range, in years, for the face. Low represents the lowest estimated age and High represents the highest estimated age.</p>
    #[serde(rename = "AgeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<AgeRange>,
    /// <p>Indicates whether or not the face has a beard, and the confidence level in the determination.</p>
    #[serde(rename = "Beard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beard: Option<Beard>,
    /// <p>Bounding box of the face. Default attribute.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the bounding box contains a face (and not a different object such as a tree). Default attribute.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The emotions that appear to be expressed on the face, and the confidence level in the determination. The API is only making a determination of the physical appearance of a person's face. It is not a determination of the person’s internal emotional state and should not be used in such a way. For example, a person pretending to have a sad face might not be sad emotionally.</p>
    #[serde(rename = "Emotions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotions: Option<Vec<Emotion>>,
    /// <p>Indicates whether or not the face is wearing eye glasses, and the confidence level in the determination.</p>
    #[serde(rename = "Eyeglasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyeglasses: Option<Eyeglasses>,
    /// <p>Indicates whether or not the eyes on the face are open, and the confidence level in the determination.</p>
    #[serde(rename = "EyesOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyes_open: Option<EyeOpen>,
    /// <p>The predicted gender of a detected face. </p>
    #[serde(rename = "Gender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// <p>Indicates the location of landmarks on the face. Default attribute.</p>
    #[serde(rename = "Landmarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<Vec<Landmark>>,
    /// <p>Indicates whether or not the mouth on the face is open, and the confidence level in the determination.</p>
    #[serde(rename = "MouthOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_open: Option<MouthOpen>,
    /// <p>Indicates whether or not the face has a mustache, and the confidence level in the determination.</p>
    #[serde(rename = "Mustache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mustache: Option<Mustache>,
    /// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw. Default attribute.</p>
    #[serde(rename = "Pose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    /// <p>Identifies image brightness and sharpness. Default attribute.</p>
    #[serde(rename = "Quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    /// <p>Indicates whether or not the face is smiling, and the confidence level in the determination.</p>
    #[serde(rename = "Smile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smile: Option<Smile>,
    /// <p>Indicates whether or not the face is wearing sunglasses, and the confidence level in the determination.</p>
    #[serde(rename = "Sunglasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunglasses: Option<Sunglasses>,
}

/// <p>Information about a face detected in a video analysis request and the time the face was detected in the video. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaceDetection {
    /// <p>The face properties for the detected face.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>Time, in milliseconds from the start of the video, that the face was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Provides face metadata. In addition, it also provides the confidence in the match of this face with the input face.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaceMatch {
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the source image, and external image ID that you assigned.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    /// <p>Confidence in the match of this face with the input face.</p>
    #[serde(rename = "Similarity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

/// <p>Object containing both the face metadata (stored in the backend database), and facial attributes that are detected but aren't stored in the database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FaceRecord {
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the input image, and external image ID that you assigned. </p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    /// <p>Structure containing attributes of the face that the algorithm detected.</p>
    #[serde(rename = "FaceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detail: Option<FaceDetail>,
}

/// <p>Input face recognition parameters for an Amazon Rekognition stream processor. <code>FaceRecognitionSettings</code> is a request parameter for <a>CreateStreamProcessor</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaceSearchSettings {
    /// <p>The ID of a collection that contains faces that you want to search for.</p>
    #[serde(rename = "CollectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// <p>Minimum face match confidence score that must be met to return a result for a recognized face. Default is 70. 0 is the lowest confidence. 100 is the highest confidence.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
}

/// <p>The predicted gender of a detected face. </p> <p>Amazon Rekognition makes gender binary (male/female) predictions based on the physical appearance of a face in a particular image. This kind of prediction is not designed to categorize a person’s gender identity, and you shouldn't use Amazon Rekognition to make such a determination. For example, a male actor wearing a long-haired wig and earrings for a role might be predicted as female.</p> <p>Using Amazon Rekognition to make gender binary predictions is best suited for use cases where aggregate gender distribution statistics need to be analyzed without identifying specific users. For example, the percentage of female users compared to male users on a social media platform. </p> <p>We don't recommend using gender binary predictions to make decisions that impact&#x2028; an individual's rights, privacy, or access to services.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Gender {
    /// <p>Level of confidence in the prediction.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The predicted gender of the face.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about where an object (<a>DetectCustomLabels</a>) or text (<a>DetectText</a>) is located on an image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Geometry {
    /// <p>An axis-aligned coarse representation of the detected item's location on the image.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Within the bounding box, a fine-grained polygon around the detected item.</p>
    #[serde(rename = "Polygon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Vec<Point>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCelebrityInfoRequest {
    /// <p>The ID for the celebrity. You get the celebrity ID from a call to the <a>RecognizeCelebrities</a> operation, which recognizes celebrities in an image. </p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCelebrityInfoResponse {
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional celebrity information. </p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCelebrityRecognitionRequest {
    /// <p>Job identifier for the required celebrity recognition analysis. You can get the job identifer from a call to <code>StartCelebrityRecognition</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more recognized celebrities to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of celebrities. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for celebrities returned in <code>Celebrities</code> field. Specify <code>ID</code> to sort by the celebrity identifier, specify <code>TIMESTAMP</code> to sort by the time the celebrity was recognized.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCelebrityRecognitionResponse {
    /// <p>Array of celebrities recognized in the video.</p>
    #[serde(rename = "Celebrities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrities: Option<Vec<CelebrityRecognition>>,
    /// <p>The current status of the celebrity recognition job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of celebrities.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetContentModerationRequest {
    /// <p>The identifier for the unsafe content job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetContentModeration</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Rekognition returns a pagination token in the response. You can use this pagination token to retrieve the next set of unsafe content labels.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>ModerationLabelDetections</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time labels are detected. Use <code>NAME</code> to alphabetically group elements for a label together. Within each label group, the array element are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetContentModerationResponse {
    /// <p>The current status of the unsafe content analysis job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The detected unsafe content labels and the time(s) they were detected.</p>
    #[serde(rename = "ModerationLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_labels: Option<Vec<ContentModerationDetection>>,
    /// <p>Version number of the moderation detection model that was used to detect unsafe content.</p>
    #[serde(rename = "ModerationModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_model_version: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of unsafe content labels. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from <code>GetContentModeration</code>. </p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFaceDetectionRequest {
    /// <p>Unique identifier for the face detection job. The <code>JobId</code> is returned from <code>StartFaceDetection</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more faces to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFaceDetectionResponse {
    /// <p>An array of faces detected in the video. Each element contains a detected face's details and the time, in milliseconds from the start of the video, the face was detected. </p>
    #[serde(rename = "Faces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<FaceDetection>>,
    /// <p>The current status of the face detection job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition returns this token that you can use in the subsequent request to retrieve the next set of faces. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFaceSearchRequest {
    /// <p>The job identifer for the search request. You get the job identifier from an initial call to <code>StartFaceSearch</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more search results to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of search results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for grouping faces in the response. Use <code>TIMESTAMP</code> to group faces by the time that they are recognized. Use <code>INDEX</code> to sort by recognized faces. </p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFaceSearchResponse {
    /// <p>The current status of the face search job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of search results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of persons, <a>PersonMatch</a>, in the video whose face(s) match the face(s) in an Amazon Rekognition collection. It also includes time information for when persons are matched in the video. You specify the input collection in an initial call to <code>StartFaceSearch</code>. Each <code>Persons</code> element includes a time the person was matched, face match details (<code>FaceMatches</code>) for matching faces in the collection, and person information (<code>Person</code>) for the matched person. </p>
    #[serde(rename = "Persons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<PersonMatch>>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation. </p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLabelDetectionRequest {
    /// <p>Job identifier for the label detection operation for which you want results returned. You get the job identifer from an initial call to <code>StartlabelDetection</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more labels to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of labels. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>Labels</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time labels are detected. Use <code>NAME</code> to alphabetically group elements for a label together. Within each label group, the array element are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLabelDetectionResponse {
    /// <p>The current status of the label detection job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Version number of the label detection model that was used to detect labels.</p>
    #[serde(rename = "LabelModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_model_version: Option<String>,
    /// <p>An array of labels detected in the video. Each element contains the detected label and the time, in milliseconds from the start of the video, that the label was detected. </p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelDetection>>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of labels.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPersonTrackingRequest {
    /// <p>The identifier for a job that tracks persons in a video. You get the <code>JobId</code> from a call to <code>StartPersonTracking</code>. </p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more persons to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of persons. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>Persons</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time persons are detected. Use <code>INDEX</code> to sort by the tracked persons. If you sort by <code>INDEX</code>, the array elements for each person are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPersonTrackingResponse {
    /// <p>The current status of the person tracking job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of persons. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of the persons detected in the video and the time(s) their path was tracked throughout the video. An array element will exist for each time a person's path is tracked. </p>
    #[serde(rename = "Persons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<PersonDetection>>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

/// <p>The S3 bucket that contains the Ground Truth manifest file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroundTruthManifest {
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
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

/// <p>Sets up the flow definition the image will be sent to if one of the conditions is met. You can also set certain attributes of the image before review.</p>
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
    /// <p>The name of the human review used for this image. This should be kept unique within a region.</p>
    #[serde(rename = "HumanLoopName")]
    pub human_loop_name: String,
}

/// <p>Allows you to set attributes of the image. Currently, you can declare an image as free of personally identifiable information.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HumanLoopDataAttributes {
    /// <p>Sets whether the input image is free of personally identifiable information.</p>
    #[serde(rename = "ContentClassifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_classifiers: Option<Vec<String>>,
}

/// <p>Provides the input image either as bytes or an S3 object.</p> <p>You pass image bytes to an Amazon Rekognition API operation by using the <code>Bytes</code> property. For example, you would use the <code>Bytes</code> property to pass an image loaded from a local file system. Image bytes passed by using the <code>Bytes</code> property must be base64-encoded. Your code may not need to encode image bytes if you are using an AWS SDK to call Amazon Rekognition API operations. </p> <p>For more information, see Analyzing an Image Loaded from a Local File System in the Amazon Rekognition Developer Guide.</p> <p> You pass images stored in an S3 bucket to an Amazon Rekognition API operation by using the <code>S3Object</code> property. Images stored in an S3 bucket do not need to be base64-encoded.</p> <p>The region for the S3 bucket containing the S3 object must match the region you use for Amazon Rekognition operations.</p> <p>If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes using the Bytes property is not supported. You must first upload the image to an Amazon S3 bucket and then call the operation using the S3Object property.</p> <p>For Amazon Rekognition to process an S3 object, the user must have permission to access the S3 object. For more information, see Resource Based Policies in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Image {
    /// <p>Blob of image bytes up to 5 MBs.</p>
    #[serde(rename = "Bytes")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<bytes::Bytes>,
    /// <p>Identifies an S3 object as the image source.</p>
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>Identifies face image brightness and sharpness. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageQuality {
    /// <p>Value representing brightness of the face. The service returns a value between 0 and 100 (inclusive). A higher value indicates a brighter face image.</p>
    #[serde(rename = "Brightness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,
    /// <p>Value representing sharpness of the face. The service returns a value between 0 and 100 (inclusive). A higher value indicates a sharper face image.</p>
    #[serde(rename = "Sharpness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IndexFacesRequest {
    /// <p>The ID of an existing collection to which you want to add the faces that are detected in the input images.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>An array of facial attributes that you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>["DEFAULT"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code>, and <code>Landmarks</code>. If you provide <code>["ALL"]</code>, all facial attributes are returned, but the operation takes longer to complete.</p> <p>If you provide both, <code>["ALL", "DEFAULT"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>
    #[serde(rename = "DetectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_attributes: Option<Vec<String>>,
    /// <p>The ID you want to assign to all the faces detected in the image.</p>
    #[serde(rename = "ExternalImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_image_id: Option<String>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes isn't supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>The maximum number of faces to index. The value of <code>MaxFaces</code> must be greater than or equal to 1. <code>IndexFaces</code> returns no more than 100 detected faces in an image, even if you specify a larger value for <code>MaxFaces</code>.</p> <p>If <code>IndexFaces</code> detects more faces than the value of <code>MaxFaces</code>, the faces with the lowest quality are filtered out first. If there are still more faces than the value of <code>MaxFaces</code>, the faces with the smallest bounding boxes are filtered out (up to the number that's needed to satisfy the value of <code>MaxFaces</code>). Information about the unindexed faces is available in the <code>UnindexedFaces</code> array. </p> <p>The faces that are returned by <code>IndexFaces</code> are sorted by the largest face bounding box size to the smallest size, in descending order.</p> <p> <code>MaxFaces</code> can be used with a collection associated with any version of the face model.</p>
    #[serde(rename = "MaxFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i64>,
    /// <p>A filter that specifies a quality bar for how much filtering is done to identify faces. Filtered faces aren't indexed. If you specify <code>AUTO</code>, Amazon Rekognition chooses the quality bar. If you specify <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>, filtering removes all faces that don’t meet the chosen quality bar. The default value is <code>AUTO</code>. The quality bar is based on a variety of common use cases. Low-quality detections can occur for a number of reasons. Some examples are an object that's misidentified as a face, a face that's too blurry, or a face with a pose that's too extreme to use. If you specify <code>NONE</code>, no filtering is performed. </p> <p>To use quality filtering, the collection you are using must be associated with version 3 of the face model or higher.</p>
    #[serde(rename = "QualityFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_filter: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IndexFacesResponse {
    /// <p>The version number of the face detection model that's associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>An array of faces detected and added to the collection. For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide. </p>
    #[serde(rename = "FaceRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_records: Option<Vec<FaceRecord>>,
    /// <p>If your collection is associated with a face detection model that's later than version 3.0, the value of <code>OrientationCorrection</code> is always null and no orientation information is returned.</p> <p>If your collection is associated with a face detection model that's version 3.0 or earlier, the following applies:</p> <ul> <li> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction - the bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata. The value of <code>OrientationCorrection</code> is null.</p> </li> <li> <p>If the image doesn't contain orientation information in its Exif metadata, Amazon Rekognition returns an estimated orientation (ROTATE_0, ROTATE_90, ROTATE_180, ROTATE_270). Amazon Rekognition doesn’t perform image correction for images. The bounding box coordinates aren't translated and represent the object locations before the image is rotated.</p> </li> </ul> <p>Bounding box information is returned in the <code>FaceRecords</code> array. You can get the version of the face detection model by calling <a>DescribeCollection</a>. </p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
    /// <p>An array of faces that were detected in the image but weren't indexed. They weren't indexed because the quality filter identified them as low quality, or the <code>MaxFaces</code> request parameter filtered them out. To use the quality filter, you specify the <code>QualityFilter</code> request parameter.</p>
    #[serde(rename = "UnindexedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unindexed_faces: Option<Vec<UnindexedFace>>,
}

/// <p>An instance of a label returned by Amazon Rekognition Image (<a>DetectLabels</a>) or by Amazon Rekognition Video (<a>GetLabelDetection</a>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Instance {
    /// <p>The position of the label instance on the image.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>The confidence that Amazon Rekognition has in the accuracy of the bounding box.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// <p>The Kinesis data stream Amazon Rekognition to which the analysis results of a Amazon Rekognition stream processor are streamed. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisDataStream {
    /// <p>ARN of the output Amazon Kinesis Data Streams stream.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>Kinesis video stream stream that provides the source streaming video for a Amazon Rekognition Video stream processor. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisVideoStream {
    /// <p>ARN of the Kinesis video stream stream that streams the source video.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>Structure containing details about the detected label, including the name, detected instances, parent labels, and level of confidence.</p> <p> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Label {
    /// <p>Level of confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>If <code>Label</code> represents an object, <code>Instances</code> contains the bounding boxes for each instance of the detected object. Bounding boxes are returned for common object labels such as people, cars, furniture, apparel or pets.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    /// <p>The name (label) of the object or scene.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The parent labels for a label. The response includes all ancestor labels.</p>
    #[serde(rename = "Parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Parent>>,
}

/// <p>Information about a label detected in a video analysis request and the time the label was detected in the video. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LabelDetection {
    /// <p>Details about the detected label.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    /// <p>Time, in milliseconds from the start of the video, that the label was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Indicates the location of the landmark on the face.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Landmark {
    /// <p>Type of landmark.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The x-coordinate from the top left of the landmark expressed as the ratio of the width of the image. For example, if the image is 700 x 200 and the x-coordinate of the landmark is at 350 pixels, this value is 0.5. </p>
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    /// <p>The y-coordinate from the top left of the landmark expressed as the ratio of the height of the image. For example, if the image is 700 x 200 and the y-coordinate of the landmark is at 100 pixels, this value is 0.5.</p>
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCollectionsRequest {
    /// <p>Maximum number of collection IDs to return. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCollectionsResponse {
    /// <p>An array of collection IDs.</p>
    #[serde(rename = "CollectionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<String>>,
    /// <p>Version numbers of the face detection models associated with the collections in the array <code>CollectionIds</code>. For example, the value of <code>FaceModelVersions[2]</code> is the version number for the face detection model used by the collection in <code>CollectionId[2]</code>.</p>
    #[serde(rename = "FaceModelVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_versions: Option<Vec<String>>,
    /// <p>If the result is truncated, the response provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFacesRequest {
    /// <p>ID of the collection from which to list the faces.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>Maximum number of faces to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Rekognition returns a pagination token in the response. You can use this pagination token to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFacesResponse {
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>An array of <code>Face</code> objects. </p>
    #[serde(rename = "Faces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<Face>>,
    /// <p>If the response is truncated, Amazon Rekognition returns this token that you can use in the subsequent request to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListStreamProcessorsRequest {
    /// <p>Maximum number of stream processors you want Amazon Rekognition Video to return in the response. The default is 1000. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more stream processors to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of stream processors. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListStreamProcessorsResponse {
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of stream processors. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of stream processors that you have created.</p>
    #[serde(rename = "StreamProcessors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processors: Option<Vec<StreamProcessor>>,
}

/// <p>Provides information about a single type of unsafe content found in an image or video. Each type of moderated content has a label within a hierarchical taxonomy. For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModerationLabel {
    /// <p>Specifies the confidence that Amazon Rekognition has that the label has been correctly identified.</p> <p>If you don't specify the <code>MinConfidence</code> parameter in the call to <code>DetectModerationLabels</code>, the operation returns labels with a confidence value greater than or equal to 50 percent.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The label name for the type of unsafe content detected in the image.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name for the parent label. Labels at the top level of the hierarchy have the parent label <code>""</code>.</p>
    #[serde(rename = "ParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
}

/// <p>Indicates whether or not the mouth on the face is open, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MouthOpen {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the mouth on the face is open or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Indicates whether or not the face has a mustache, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Mustache {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face has mustache or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the completion status of a video analysis operation. For more information, see <a>api-video</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NotificationChannel {
    /// <p>The ARN of an IAM role that gives Amazon Rekognition publishing permissions to the Amazon SNS topic. </p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The Amazon SNS topic to which Amazon Rekognition to posts the completion status.</p>
    #[serde(rename = "SNSTopicArn")]
    pub sns_topic_arn: String,
}

/// <p>The S3 bucket and folder location where training output is placed.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputConfig {
    /// <p>The S3 bucket where training output is placed.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The prefix applied to the training output files. </p>
    #[serde(rename = "S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

/// <p>A parent label for a label. A label can have 0, 1, or more parents. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Parent {
    /// <p>The name of the parent label.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Details about a person detected in a video analysis request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PersonDetail {
    /// <p>Bounding box around the detected person.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Face details for the detected person.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>Identifier for the person detected person within a video. Use to keep track of the person throughout the video. The identifier is not stored by Amazon Rekognition.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

/// <p>Details and path tracking information for a single time a person's path is tracked in a video. Amazon Rekognition operations that track people's paths return an array of <code>PersonDetection</code> objects with elements for each time a person's path is tracked in a video. </p> <p>For more information, see GetPersonTracking in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PersonDetection {
    /// <p>Details about a person whose path was tracked in a video.</p>
    #[serde(rename = "Person")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,
    /// <p>The time, in milliseconds from the start of the video, that the person's path was tracked.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Information about a person whose face matches a face(s) in an Amazon Rekognition collection. Includes information about the faces in the Amazon Rekognition collection (<a>FaceMatch</a>), information about the person (<a>PersonDetail</a>), and the time stamp for when the person was detected in a video. An array of <code>PersonMatch</code> objects is returned by <a>GetFaceSearch</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PersonMatch {
    /// <p>Information about the faces in the input collection that match the face of a person in the video.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Information about the matched person.</p>
    #[serde(rename = "Person")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,
    /// <p>The time, in milliseconds from the beginning of the video, that the person was matched in the video.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>The X and Y coordinates of a point on an image. The X and Y values returned are ratios of the overall image size. For example, if the input image is 700x200 and the operation returns X=0.5 and Y=0.25, then the point is at the (350,50) pixel coordinate on the image.</p> <p>An array of <code>Point</code> objects, <code>Polygon</code>, is returned by <a>DetectText</a> and by <a>DetectCustomLabels</a>. <code>Polygon</code> represents a fine-grained polygon around a detected item. For more information, see Geometry in the Amazon Rekognition Developer Guide. </p>
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

/// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Pose {
    /// <p>Value representing the face rotation on the pitch axis.</p>
    #[serde(rename = "Pitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,
    /// <p>Value representing the face rotation on the roll axis.</p>
    #[serde(rename = "Roll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f32>,
    /// <p>Value representing the face rotation on the yaw axis.</p>
    #[serde(rename = "Yaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaw: Option<f32>,
}

/// <p>A description of a Amazon Rekognition Custom Labels project.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectDescription {
    /// <p>The Unix timestamp for the date and time that the project was created.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    #[serde(rename = "ProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
    /// <p>The current status of the project.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The description of a version of a model.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProjectVersionDescription {
    /// <p>The duration, in seconds, that the model version has been billed for training. This value is only returned if the model version has been successfully trained.</p>
    #[serde(rename = "BillableTrainingTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_training_time_in_seconds: Option<i64>,
    /// <p>The Unix datetime for the date and time that training started.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    /// <p>The training results. <code>EvaluationResult</code> is only returned if training is successful.</p>
    #[serde(rename = "EvaluationResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<EvaluationResult>,
    /// <p>The minimum number of inference units used by the model. For more information, see <a>StartProjectVersion</a>.</p>
    #[serde(rename = "MinInferenceUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_inference_units: Option<i64>,
    /// <p>The location where training results are saved.</p>
    #[serde(rename = "OutputConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<OutputConfig>,
    /// <p>The Amazon Resource Name (ARN) of the model version. </p>
    #[serde(rename = "ProjectVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version_arn: Option<String>,
    /// <p>The current status of the model version.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A descriptive message for an error or warning that occurred.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The manifest file that represents the testing results.</p>
    #[serde(rename = "TestingDataResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing_data_result: Option<TestingDataResult>,
    /// <p>The manifest file that represents the training results.</p>
    #[serde(rename = "TrainingDataResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_result: Option<TrainingDataResult>,
    /// <p>The Unix date and time that training of the model ended.</p>
    #[serde(rename = "TrainingEndTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_timestamp: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RecognizeCelebritiesRequest {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RecognizeCelebritiesResponse {
    /// <p>Details about each celebrity found in the image. Amazon Rekognition can detect a maximum of 15 celebrities in an image.</p>
    #[serde(rename = "CelebrityFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrity_faces: Option<Vec<Celebrity>>,
    /// <p><p>The orientation of the input image (counterclockwise direction). If your application displays the image, you can use this value to correct the orientation. The bounding box coordinates returned in <code>CelebrityFaces</code> and <code>UnrecognizedFaces</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the input image is in .jpeg format, it might contain exchangeable image (Exif) metadata that includes the image&#39;s orientation. If so, and the Exif metadata for the input image populates the orientation field, the value of <code>OrientationCorrection</code> is null. The <code>CelebrityFaces</code> and <code>UnrecognizedFaces</code> bounding box coordinates represent face locations after Exif metadata is used to correct the image orientation. Images in .png format don&#39;t contain Exif metadata. </p> </note></p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
    /// <p>Details about each unrecognized face in the image.</p>
    #[serde(rename = "UnrecognizedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrecognized_faces: Option<Vec<ComparedFace>>,
}

/// <p>Provides the S3 bucket name and object name.</p> <p>The region for the S3 bucket containing the S3 object must match the region you use for Amazon Rekognition operations.</p> <p>For Amazon Rekognition to process an S3 object, the user must have permission to access the S3 object. For more information, see Resource-Based Policies in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Object {
    /// <p>Name of the S3 bucket.</p>
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>S3 object key name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If the bucket is versioning enabled, you can specify the object version. </p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchFacesByImageRequest {
    /// <p>ID of the collection to search.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>(Optional) Specifies the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%. The default value is 80%.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>
    #[serde(rename = "MaxFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i64>,
    /// <p>A filter that specifies a quality bar for how much filtering is done to identify faces. Filtered faces aren't searched for in the collection. If you specify <code>AUTO</code>, Amazon Rekognition chooses the quality bar. If you specify <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>, filtering removes all faces that don’t meet the chosen quality bar. The quality bar is based on a variety of common use cases. Low-quality detections can occur for a number of reasons. Some examples are an object that's misidentified as a face, a face that's too blurry, or a face with a pose that's too extreme to use. If you specify <code>NONE</code>, no filtering is performed. The default value is <code>NONE</code>. </p> <p>To use quality filtering, the collection you are using must be associated with version 3 of the face model or higher.</p>
    #[serde(rename = "QualityFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_filter: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchFacesByImageResponse {
    /// <p>An array of faces that match the input face, along with the confidence in the match.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>The bounding box around the face in the input image that Amazon Rekognition used for the search.</p>
    #[serde(rename = "SearchedFaceBoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_bounding_box: Option<BoundingBox>,
    /// <p>The level of confidence that the <code>searchedFaceBoundingBox</code>, contains a face.</p>
    #[serde(rename = "SearchedFaceConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchFacesRequest {
    /// <p>ID of the collection the face belongs to.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>ID of a face to find matches for in the collection.</p>
    #[serde(rename = "FaceId")]
    pub face_id: String,
    /// <p>Optional value specifying the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%. The default value is 80%. </p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>
    #[serde(rename = "MaxFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchFacesResponse {
    /// <p>An array of faces that matched the input face, along with the confidence in the match.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>ID of the face that was searched for matches in a collection.</p>
    #[serde(rename = "SearchedFaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_id: Option<String>,
}

/// <p>Indicates whether or not the face is smiling, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Smile {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is smiling or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartCelebrityRecognitionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartCelebrityRecognition</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The Amazon SNS topic ARN that you want Amazon Rekognition Video to publish the completion status of the celebrity recognition analysis to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to recognize celebrities. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartCelebrityRecognitionResponse {
    /// <p>The identifier for the celebrity recognition analysis job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetCelebrityRecognition</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartContentModerationRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartContentModeration</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>Specifies the minimum confidence that Amazon Rekognition must have in order to return a moderated content label. Confidence represents how certain Amazon Rekognition is that the moderated content is correctly identified. 0 is the lowest confidence. 100 is the highest confidence. Amazon Rekognition doesn't return any moderated content labels with a confidence level lower than this specified value. If you don't specify <code>MinConfidence</code>, <code>GetContentModeration</code> returns labels with confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    /// <p>The Amazon SNS topic ARN that you want Amazon Rekognition Video to publish the completion status of the unsafe content analysis to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect unsafe content. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartContentModerationResponse {
    /// <p>The identifier for the unsafe content analysis job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetContentModeration</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartFaceDetectionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartFaceDetection</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The face attributes you want returned.</p> <p> <code>DEFAULT</code> - The following subset of facial attributes are returned: BoundingBox, Confidence, Pose, Quality and Landmarks. </p> <p> <code>ALL</code> - All facial attributes are returned.</p>
    #[serde(rename = "FaceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_attributes: Option<String>,
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The ARN of the Amazon SNS topic to which you want Amazon Rekognition Video to publish the completion status of the face detection operation.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect faces. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartFaceDetectionResponse {
    /// <p>The identifier for the face detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetFaceDetection</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartFaceSearchRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartFaceSearch</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>ID of the collection that contains the faces you want to search for.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>The minimum confidence in the person match to return. For example, don't return any matches where confidence in matches is less than 70%. The default value is 80%.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The ARN of the Amazon SNS topic to which you want Amazon Rekognition Video to publish the completion status of the search. </p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video you want to search. The video must be stored in an Amazon S3 bucket. </p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartFaceSearchResponse {
    /// <p>The identifier for the search job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetFaceSearch</code>. </p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartLabelDetectionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartLabelDetection</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>Specifies the minimum confidence that Amazon Rekognition Video must have in order to return a detected label. Confidence represents how certain Amazon Rekognition is that a label is correctly identified.0 is the lowest confidence. 100 is the highest confidence. Amazon Rekognition Video doesn't return any labels with a confidence level lower than this specified value.</p> <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the label detection operation to. </p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect labels. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartLabelDetectionResponse {
    /// <p>The identifier for the label detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetLabelDetection</code>. </p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartPersonTrackingRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartPersonTracking</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the people detection operation to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect people. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartPersonTrackingResponse {
    /// <p>The identifier for the person detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetPersonTracking</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartProjectVersionRequest {
    /// <p>The minimum number of inference units to use. A single inference unit represents 1 hour of processing and can support up to 5 Transaction Pers Second (TPS). Use a higher number to increase the TPS throughput of your model. You are charged for the number of inference units that you use. </p>
    #[serde(rename = "MinInferenceUnits")]
    pub min_inference_units: i64,
    /// <p>The Amazon Resource Name(ARN) of the model version that you want to start.</p>
    #[serde(rename = "ProjectVersionArn")]
    pub project_version_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartProjectVersionResponse {
    /// <p>The current running status of the model. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartStreamProcessorRequest {
    /// <p>The name of the stream processor to start processing.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartStreamProcessorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopProjectVersionRequest {
    /// <p>The Amazon Resource Name (ARN) of the model version that you want to delete.</p> <p>This operation requires permissions to perform the <code>rekognition:StopProjectVersion</code> action.</p>
    #[serde(rename = "ProjectVersionArn")]
    pub project_version_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopProjectVersionResponse {
    /// <p>The current status of the stop operation. </p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopStreamProcessorRequest {
    /// <p>The name of a stream processor created by <a>CreateStreamProcessor</a>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopStreamProcessorResponse {}

/// <p>An object that recognizes faces in a streaming video. An Amazon Rekognition stream processor is created by a call to <a>CreateStreamProcessor</a>. The request parameters for <code>CreateStreamProcessor</code> describe the Kinesis video stream source for the streaming video, face recognition parameters, and where to stream the analysis resullts. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StreamProcessor {
    /// <p>Name of the Amazon Rekognition stream processor. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Current status of the Amazon Rekognition stream processor.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about the source streaming video. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorInput {
    /// <p>The Kinesis video stream input stream for the source streaming video.</p>
    #[serde(rename = "KinesisVideoStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_video_stream: Option<KinesisVideoStream>,
}

/// <p>Information about the Amazon Kinesis Data Streams stream to which a Amazon Rekognition Video stream processor streams the results of a video analysis. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorOutput {
    /// <p>The Amazon Kinesis Data Streams stream to which the Amazon Rekognition stream processor streams the analysis results.</p>
    #[serde(rename = "KinesisDataStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_data_stream: Option<KinesisDataStream>,
}

/// <p>Input parameters used to recognize faces in a streaming video analyzed by a Amazon Rekognition stream processor.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorSettings {
    /// <p>Face search settings to use on a streaming video. </p>
    #[serde(rename = "FaceSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_search: Option<FaceSearchSettings>,
}

/// <p>The S3 bucket that contains the training summary. The training summary includes aggregated evaluation metrics for the entire testing dataset and metrics for each individual label. </p> <p>You get the training summary S3 bucket location by calling <a>DescribeProjectVersions</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Summary {
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>Indicates whether or not the face is wearing sunglasses, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Sunglasses {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is wearing sunglasses or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>The dataset used for testing. Optionally, if <code>AutoCreate</code> is set, Amazon Rekognition Custom Labels creates a testing dataset using an 80/20 split of the training dataset.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestingData {
    /// <p>The assets used for testing.</p>
    #[serde(rename = "Assets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<Asset>>,
    /// <p>If specified, Amazon Rekognition Custom Labels creates a testing dataset with an 80/20 split of the training dataset.</p>
    #[serde(rename = "AutoCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create: Option<bool>,
}

/// <p>A Sagemaker Groundtruth format manifest file representing the dataset used for testing.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestingDataResult {
    /// <p>The testing dataset that was supplied for training.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<TestingData>,
    /// <p>The subset of the dataset that was actually tested. Some images (assets) might not be tested due to file formatting and other issues. </p>
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<TestingData>,
}

/// <p>Information about a word or line of text detected by <a>DetectText</a>.</p> <p>The <code>DetectedText</code> field contains the text that Amazon Rekognition detected in the image. </p> <p>Every word and line has an identifier (<code>Id</code>). Each word belongs to a line and has a parent identifier (<code>ParentId</code>) that identifies the line of text in which the word appears. The word <code>Id</code> is also an index for the word within a line of words. </p> <p>For more information, see Detecting Text in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TextDetection {
    /// <p>The confidence that Amazon Rekognition has in the accuracy of the detected text and the accuracy of the geometry points around the detected text.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The word or line of text recognized by Amazon Rekognition. </p>
    #[serde(rename = "DetectedText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_text: Option<String>,
    /// <p>The location of the detected text on the image. Includes an axis aligned coarse bounding box surrounding the text and a finer grain polygon for more accurate spatial information.</p>
    #[serde(rename = "Geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    /// <p>The identifier for the detected text. The identifier is only unique for a single call to <code>DetectText</code>. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The Parent identifier for the detected text identified by the value of <code>ID</code>. If the type of detected text is <code>LINE</code>, the value of <code>ParentId</code> is <code>Null</code>. </p>
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    /// <p>The type of text that was detected.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The dataset used for training.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingData {
    /// <p>A Sagemaker GroundTruth manifest file that contains the training images (assets).</p>
    #[serde(rename = "Assets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<Asset>>,
}

/// <p>A Sagemaker Groundtruth format manifest file that represents the dataset used for training.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TrainingDataResult {
    /// <p>The training assets that you supplied for training.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<TrainingData>,
    /// <p>The images (assets) that were actually trained by Amazon Rekognition Custom Labels. </p>
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<TrainingData>,
}

/// <p>A face that <a>IndexFaces</a> detected, but didn't index. Use the <code>Reasons</code> response attribute to determine why a face wasn't indexed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnindexedFace {
    /// <p>The structure that contains attributes of a face that <code>IndexFaces</code>detected, but didn't index. </p>
    #[serde(rename = "FaceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detail: Option<FaceDetail>,
    /// <p><p>An array of reasons that specify why a face wasn&#39;t indexed. </p> <ul> <li> <p>EXTREME<em>POSE - The face is at a pose that can&#39;t be detected. For example, the head is turned too far away from the camera.</p> </li> <li> <p>EXCEEDS</em>MAX<em>FACES - The number of faces detected is already higher than that specified by the <code>MaxFaces</code> input parameter for <code>IndexFaces</code>.</p> </li> <li> <p>LOW</em>BRIGHTNESS - The image is too dark.</p> </li> <li> <p>LOW<em>SHARPNESS - The image is too blurry.</p> </li> <li> <p>LOW</em>CONFIDENCE - The face was detected with a low confidence.</p> </li> <li> <p>SMALL<em>BOUNDING</em>BOX - The bounding box around the face is too small.</p> </li> </ul></p>
    #[serde(rename = "Reasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
}

/// <p>Video file stored in an Amazon S3 bucket. Amazon Rekognition video start operations such as <a>StartLabelDetection</a> use <code>Video</code> to specify a video for analysis. The supported file formats are .mp4, .mov and .avi.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Video {
    /// <p>The Amazon S3 bucket name and file name for the video.</p>
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VideoMetadata {
    /// <p>Type of compression used in the analyzed video. </p>
    #[serde(rename = "Codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Length of the video in milliseconds.</p>
    #[serde(rename = "DurationMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    /// <p>Format of the analyzed video. Possible values are MP4, MOV and AVI. </p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Vertical pixel dimension of the video.</p>
    #[serde(rename = "FrameHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_height: Option<i64>,
    /// <p>Number of frames per second in the video.</p>
    #[serde(rename = "FrameRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<f32>,
    /// <p>Horizontal pixel dimension of the video.</p>
    #[serde(rename = "FrameWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<i64>,
}

/// Errors returned by CompareFaces
#[derive(Debug, PartialEq)]
pub enum CompareFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CompareFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompareFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CompareFacesError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(CompareFacesError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CompareFacesError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(CompareFacesError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CompareFacesError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(CompareFacesError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(CompareFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CompareFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CompareFacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompareFacesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CompareFacesError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            CompareFacesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CompareFacesError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            CompareFacesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CompareFacesError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            CompareFacesError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            CompareFacesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CompareFacesError {}
/// Errors returned by CreateCollection
#[derive(Debug, PartialEq)]
pub enum CreateCollectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>A collection with the specified ID already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCollectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateCollectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateCollectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateCollectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        CreateCollectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateCollectionError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateCollectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCollectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCollectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateCollectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateCollectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateCollectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCollectionError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateCollectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCollectionError {}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateProjectError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProjectError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateProjectError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateProjectError::InternalServerError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateProjectError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProjectError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(CreateProjectError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateProjectError::ResourceInUse(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateProjectError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateProjectError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProjectError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateProjectError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateProjectError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateProjectError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProjectError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            CreateProjectError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateProjectError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProjectError {}
/// Errors returned by CreateProjectVersion
#[derive(Debug, PartialEq)]
pub enum CreateProjectVersionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateProjectVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateProjectVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateProjectVersionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateProjectVersionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateProjectVersionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateProjectVersionError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        CreateProjectVersionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateProjectVersionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateProjectVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateProjectVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateProjectVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateProjectVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateProjectVersionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateProjectVersionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateProjectVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateProjectVersionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateProjectVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateProjectVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateProjectVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateProjectVersionError {}
/// Errors returned by CreateStreamProcessor
#[derive(Debug, PartialEq)]
pub enum CreateStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateStreamProcessorError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateStreamProcessorError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        CreateStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateStreamProcessorError::ResourceInUse(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateStreamProcessorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStreamProcessorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateStreamProcessorError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateStreamProcessorError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateStreamProcessorError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateStreamProcessorError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateStreamProcessorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateStreamProcessorError {}
/// Errors returned by DeleteCollection
#[derive(Debug, PartialEq)]
pub enum DeleteCollectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCollectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteCollectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteCollectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteCollectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DeleteCollectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteCollectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteCollectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCollectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCollectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteCollectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteCollectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteCollectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCollectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteCollectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCollectionError {}
/// Errors returned by DeleteFaces
#[derive(Debug, PartialEq)]
pub enum DeleteFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteFacesError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteFacesError::InternalServerError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteFacesError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(DeleteFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFacesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteFacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFacesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteFacesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteFacesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteFacesError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            DeleteFacesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFacesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFacesError {}
/// Errors returned by DeleteStreamProcessor
#[derive(Debug, PartialEq)]
pub enum DeleteStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DeleteStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteStreamProcessorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteStreamProcessorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteStreamProcessorError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteStreamProcessorError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteStreamProcessorError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteStreamProcessorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteStreamProcessorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteStreamProcessorError {}
/// Errors returned by DescribeCollection
#[derive(Debug, PartialEq)]
pub enum DescribeCollectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCollectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeCollectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeCollectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeCollectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DescribeCollectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCollectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeCollectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCollectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCollectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeCollectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeCollectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeCollectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCollectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeCollectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCollectionError {}
/// Errors returned by DescribeProjectVersions
#[derive(Debug, PartialEq)]
pub enum DescribeProjectVersionsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeProjectVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProjectVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeProjectVersionsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeProjectVersionsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        DescribeProjectVersionsError::InvalidPaginationToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeProjectVersionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DescribeProjectVersionsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProjectVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeProjectVersionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeProjectVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProjectVersionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeProjectVersionsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeProjectVersionsError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeProjectVersionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeProjectVersionsError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeProjectVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeProjectVersionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProjectVersionsError {}
/// Errors returned by DescribeProjects
#[derive(Debug, PartialEq)]
pub enum DescribeProjectsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeProjectsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProjectsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeProjectsError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeProjectsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(DescribeProjectsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeProjectsError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DescribeProjectsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeProjectsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeProjectsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProjectsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeProjectsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeProjectsError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            DescribeProjectsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeProjectsError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeProjectsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProjectsError {}
/// Errors returned by DescribeStreamProcessor
#[derive(Debug, PartialEq)]
pub enum DescribeStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeStreamProcessorError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DescribeStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStreamProcessorError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeStreamProcessorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStreamProcessorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeStreamProcessorError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeStreamProcessorError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeStreamProcessorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeStreamProcessorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeStreamProcessorError {}
/// Errors returned by DetectCustomLabels
#[derive(Debug, PartialEq)]
pub enum DetectCustomLabelsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>The requested resource isn't ready. For example, this exception occurs when you call <code>DetectCustomLabels</code> with a model version that isn't deployed. </p>
    ResourceNotReady(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectCustomLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectCustomLabelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectCustomLabelsError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectCustomLabelsError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectCustomLabelsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectCustomLabelsError::InvalidImageFormat(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectCustomLabelsError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectCustomLabelsError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DetectCustomLabelsError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DetectCustomLabelsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DetectCustomLabelsError::ResourceNotFound(err.msg))
                }
                "ResourceNotReadyException" => {
                    return RusotoError::Service(DetectCustomLabelsError::ResourceNotReady(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectCustomLabelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectCustomLabelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectCustomLabelsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DetectCustomLabelsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::ResourceNotReady(ref cause) => write!(f, "{}", cause),
            DetectCustomLabelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectCustomLabelsError {}
/// Errors returned by DetectFaces
#[derive(Debug, PartialEq)]
pub enum DetectFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectFacesError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectFacesError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectFacesError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectFacesError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectFacesError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectFacesError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(DetectFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectFacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectFacesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetectFacesError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            DetectFacesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DetectFacesError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            DetectFacesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DetectFacesError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            DetectFacesError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            DetectFacesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectFacesError {}
/// Errors returned by DetectLabels
#[derive(Debug, PartialEq)]
pub enum DetectLabelsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectLabelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectLabelsError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectLabelsError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectLabelsError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectLabelsError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectLabelsError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectLabelsError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(DetectLabelsError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectLabelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectLabelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectLabelsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetectLabelsError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            DetectLabelsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DetectLabelsError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            DetectLabelsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DetectLabelsError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            DetectLabelsError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            DetectLabelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectLabelsError {}
/// Errors returned by DetectModerationLabels
#[derive(Debug, PartialEq)]
pub enum DetectModerationLabelsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The number of in-progress human reviews you have has exceeded the number allowed.</p>
    HumanLoopQuotaExceeded(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectModerationLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectModerationLabelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectModerationLabelsError::AccessDenied(err.msg))
                }
                "HumanLoopQuotaExceededException" => {
                    return RusotoError::Service(
                        DetectModerationLabelsError::HumanLoopQuotaExceeded(err.msg),
                    )
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectModerationLabelsError::ImageTooLarge(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectModerationLabelsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectModerationLabelsError::InvalidImageFormat(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectModerationLabelsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectModerationLabelsError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DetectModerationLabelsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectModerationLabelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectModerationLabelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectModerationLabelsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetectModerationLabelsError::HumanLoopQuotaExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DetectModerationLabelsError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            DetectModerationLabelsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DetectModerationLabelsError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            DetectModerationLabelsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DetectModerationLabelsError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            DetectModerationLabelsError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DetectModerationLabelsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectModerationLabelsError {}
/// Errors returned by DetectText
#[derive(Debug, PartialEq)]
pub enum DetectTextError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectTextError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectTextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectTextError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectTextError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectTextError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectTextError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectTextError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectTextError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(DetectTextError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectTextError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectTextError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectTextError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetectTextError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            DetectTextError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DetectTextError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            DetectTextError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DetectTextError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            DetectTextError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            DetectTextError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetectTextError {}
/// Errors returned by GetCelebrityInfo
#[derive(Debug, PartialEq)]
pub enum GetCelebrityInfoError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetCelebrityInfoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCelebrityInfoError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetCelebrityInfoError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetCelebrityInfoError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCelebrityInfoError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetCelebrityInfoError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCelebrityInfoError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetCelebrityInfoError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCelebrityInfoError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCelebrityInfoError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetCelebrityInfoError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCelebrityInfoError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetCelebrityInfoError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCelebrityInfoError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetCelebrityInfoError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCelebrityInfoError {}
/// Errors returned by GetCelebrityRecognition
#[derive(Debug, PartialEq)]
pub enum GetCelebrityRecognitionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetCelebrityRecognitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCelebrityRecognitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        GetCelebrityRecognitionError::InvalidPaginationToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetCelebrityRecognitionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCelebrityRecognitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCelebrityRecognitionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetCelebrityRecognitionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetCelebrityRecognitionError::InvalidPaginationToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCelebrityRecognitionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetCelebrityRecognitionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetCelebrityRecognitionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetCelebrityRecognitionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCelebrityRecognitionError {}
/// Errors returned by GetContentModeration
#[derive(Debug, PartialEq)]
pub enum GetContentModerationError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetContentModerationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetContentModerationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetContentModerationError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetContentModerationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetContentModerationError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetContentModerationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetContentModerationError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetContentModerationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetContentModerationError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetContentModerationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetContentModerationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetContentModerationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetContentModerationError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            GetContentModerationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetContentModerationError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetContentModerationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetContentModerationError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetContentModerationError {}
/// Errors returned by GetFaceDetection
#[derive(Debug, PartialEq)]
pub enum GetFaceDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetFaceDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFaceDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetFaceDetectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetFaceDetectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetFaceDetectionError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetFaceDetectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetFaceDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFaceDetectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetFaceDetectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFaceDetectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFaceDetectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetFaceDetectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetFaceDetectionError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            GetFaceDetectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetFaceDetectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetFaceDetectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFaceDetectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFaceDetectionError {}
/// Errors returned by GetFaceSearch
#[derive(Debug, PartialEq)]
pub enum GetFaceSearchError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetFaceSearchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFaceSearchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetFaceSearchError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetFaceSearchError::InternalServerError(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetFaceSearchError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetFaceSearchError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(GetFaceSearchError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFaceSearchError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetFaceSearchError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFaceSearchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFaceSearchError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetFaceSearchError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetFaceSearchError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            GetFaceSearchError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetFaceSearchError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            GetFaceSearchError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFaceSearchError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFaceSearchError {}
/// Errors returned by GetLabelDetection
#[derive(Debug, PartialEq)]
pub enum GetLabelDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetLabelDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLabelDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLabelDetectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetLabelDetectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetLabelDetectionError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetLabelDetectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetLabelDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLabelDetectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetLabelDetectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLabelDetectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLabelDetectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetLabelDetectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetLabelDetectionError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            GetLabelDetectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetLabelDetectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetLabelDetectionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLabelDetectionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLabelDetectionError {}
/// Errors returned by GetPersonTracking
#[derive(Debug, PartialEq)]
pub enum GetPersonTrackingError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetPersonTrackingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPersonTrackingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetPersonTrackingError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetPersonTrackingError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetPersonTrackingError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetPersonTrackingError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetPersonTrackingError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPersonTrackingError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetPersonTrackingError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPersonTrackingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPersonTrackingError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetPersonTrackingError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetPersonTrackingError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            GetPersonTrackingError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetPersonTrackingError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetPersonTrackingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetPersonTrackingError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPersonTrackingError {}
/// Errors returned by IndexFaces
#[derive(Debug, PartialEq)]
pub enum IndexFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl IndexFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<IndexFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(IndexFacesError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(IndexFacesError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(IndexFacesError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(IndexFacesError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(IndexFacesError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(IndexFacesError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(IndexFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(IndexFacesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(IndexFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for IndexFacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IndexFacesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            IndexFacesError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            IndexFacesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            IndexFacesError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            IndexFacesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            IndexFacesError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            IndexFacesError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            IndexFacesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            IndexFacesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for IndexFacesError {}
/// Errors returned by ListCollections
#[derive(Debug, PartialEq)]
pub enum ListCollectionsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListCollectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCollectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListCollectionsError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListCollectionsError::InternalServerError(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListCollectionsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListCollectionsError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        ListCollectionsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListCollectionsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListCollectionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCollectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCollectionsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListCollectionsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListCollectionsError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListCollectionsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListCollectionsError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListCollectionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListCollectionsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCollectionsError {}
/// Errors returned by ListFaces
#[derive(Debug, PartialEq)]
pub enum ListFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListFacesError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListFacesError::InternalServerError(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListFacesError::InvalidPaginationToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListFacesError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(ListFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFacesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFacesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListFacesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListFacesError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListFacesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListFacesError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            ListFacesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListFacesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFacesError {}
/// Errors returned by ListStreamProcessors
#[derive(Debug, PartialEq)]
pub enum ListStreamProcessorsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListStreamProcessorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStreamProcessorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListStreamProcessorsError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListStreamProcessorsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListStreamProcessorsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListStreamProcessorsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        ListStreamProcessorsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListStreamProcessorsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListStreamProcessorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStreamProcessorsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListStreamProcessorsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListStreamProcessorsError::InvalidPaginationToken(ref cause) => write!(f, "{}", cause),
            ListStreamProcessorsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListStreamProcessorsError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            ListStreamProcessorsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListStreamProcessorsError {}
/// Errors returned by RecognizeCelebrities
#[derive(Debug, PartialEq)]
pub enum RecognizeCelebritiesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl RecognizeCelebritiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RecognizeCelebritiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(RecognizeCelebritiesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::InvalidImageFormat(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        RecognizeCelebritiesError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RecognizeCelebritiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RecognizeCelebritiesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RecognizeCelebritiesError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            RecognizeCelebritiesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RecognizeCelebritiesError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            RecognizeCelebritiesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RecognizeCelebritiesError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            RecognizeCelebritiesError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            RecognizeCelebritiesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RecognizeCelebritiesError {}
/// Errors returned by SearchFaces
#[derive(Debug, PartialEq)]
pub enum SearchFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl SearchFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SearchFacesError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(SearchFacesError::InternalServerError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SearchFacesError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(SearchFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchFacesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchFacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchFacesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SearchFacesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SearchFacesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SearchFacesError::ProvisionedThroughputExceeded(ref cause) => write!(f, "{}", cause),
            SearchFacesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SearchFacesError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchFacesError {}
/// Errors returned by SearchFacesByImage
#[derive(Debug, PartialEq)]
pub enum SearchFacesByImageError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl SearchFacesByImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchFacesByImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SearchFacesByImageError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(SearchFacesByImageError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(SearchFacesByImageError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(SearchFacesByImageError::InvalidImageFormat(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SearchFacesByImageError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(SearchFacesByImageError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        SearchFacesByImageError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchFacesByImageError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchFacesByImageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchFacesByImageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchFacesByImageError::AccessDenied(ref cause) => write!(f, "{}", cause),
            SearchFacesByImageError::ImageTooLarge(ref cause) => write!(f, "{}", cause),
            SearchFacesByImageError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SearchFacesByImageError::InvalidImageFormat(ref cause) => write!(f, "{}", cause),
            SearchFacesByImageError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SearchFacesByImageError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            SearchFacesByImageError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            SearchFacesByImageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SearchFacesByImageError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchFacesByImageError {}
/// Errors returned by StartCelebrityRecognition
#[derive(Debug, PartialEq)]
pub enum StartCelebrityRecognitionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartCelebrityRecognitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartCelebrityRecognitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::AccessDenied(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartCelebrityRecognitionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        StartCelebrityRecognitionError::InternalServerError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartCelebrityRecognitionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::Throttling(
                        err.msg,
                    ))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::VideoTooLarge(
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
impl fmt::Display for StartCelebrityRecognitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartCelebrityRecognitionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartCelebrityRecognitionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartCelebrityRecognitionError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            StartCelebrityRecognitionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartCelebrityRecognitionError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            StartCelebrityRecognitionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartCelebrityRecognitionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartCelebrityRecognitionError::Throttling(ref cause) => write!(f, "{}", cause),
            StartCelebrityRecognitionError::VideoTooLarge(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartCelebrityRecognitionError {}
/// Errors returned by StartContentModeration
#[derive(Debug, PartialEq)]
pub enum StartContentModerationError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartContentModerationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartContentModerationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartContentModerationError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartContentModerationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartContentModerationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartContentModerationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartContentModerationError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartContentModerationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartContentModerationError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartContentModerationError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartContentModerationError::VideoTooLarge(
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
impl fmt::Display for StartContentModerationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartContentModerationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartContentModerationError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartContentModerationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartContentModerationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartContentModerationError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            StartContentModerationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartContentModerationError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartContentModerationError::Throttling(ref cause) => write!(f, "{}", cause),
            StartContentModerationError::VideoTooLarge(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartContentModerationError {}
/// Errors returned by StartFaceDetection
#[derive(Debug, PartialEq)]
pub enum StartFaceDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartFaceDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartFaceDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartFaceDetectionError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartFaceDetectionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartFaceDetectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartFaceDetectionError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartFaceDetectionError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartFaceDetectionError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartFaceDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartFaceDetectionError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartFaceDetectionError::VideoTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartFaceDetectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartFaceDetectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartFaceDetectionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartFaceDetectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartFaceDetectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartFaceDetectionError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            StartFaceDetectionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartFaceDetectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartFaceDetectionError::Throttling(ref cause) => write!(f, "{}", cause),
            StartFaceDetectionError::VideoTooLarge(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartFaceDetectionError {}
/// Errors returned by StartFaceSearch
#[derive(Debug, PartialEq)]
pub enum StartFaceSearchError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartFaceSearchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartFaceSearchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartFaceSearchError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(StartFaceSearchError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartFaceSearchError::InternalServerError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartFaceSearchError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartFaceSearchError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartFaceSearchError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartFaceSearchError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartFaceSearchError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartFaceSearchError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartFaceSearchError::VideoTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartFaceSearchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartFaceSearchError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartFaceSearchError::IdempotentParameterMismatch(ref cause) => write!(f, "{}", cause),
            StartFaceSearchError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartFaceSearchError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartFaceSearchError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            StartFaceSearchError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartFaceSearchError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartFaceSearchError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartFaceSearchError::Throttling(ref cause) => write!(f, "{}", cause),
            StartFaceSearchError::VideoTooLarge(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartFaceSearchError {}
/// Errors returned by StartLabelDetection
#[derive(Debug, PartialEq)]
pub enum StartLabelDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartLabelDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartLabelDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartLabelDetectionError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartLabelDetectionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartLabelDetectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartLabelDetectionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartLabelDetectionError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartLabelDetectionError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartLabelDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartLabelDetectionError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartLabelDetectionError::VideoTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartLabelDetectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartLabelDetectionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartLabelDetectionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartLabelDetectionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartLabelDetectionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartLabelDetectionError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            StartLabelDetectionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartLabelDetectionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartLabelDetectionError::Throttling(ref cause) => write!(f, "{}", cause),
            StartLabelDetectionError::VideoTooLarge(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartLabelDetectionError {}
/// Errors returned by StartPersonTracking
#[derive(Debug, PartialEq)]
pub enum StartPersonTrackingError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartPersonTrackingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartPersonTrackingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartPersonTrackingError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartPersonTrackingError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartPersonTrackingError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartPersonTrackingError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartPersonTrackingError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartPersonTrackingError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartPersonTrackingError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartPersonTrackingError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartPersonTrackingError::VideoTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartPersonTrackingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartPersonTrackingError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartPersonTrackingError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            StartPersonTrackingError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartPersonTrackingError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartPersonTrackingError::InvalidS3Object(ref cause) => write!(f, "{}", cause),
            StartPersonTrackingError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartPersonTrackingError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartPersonTrackingError::Throttling(ref cause) => write!(f, "{}", cause),
            StartPersonTrackingError::VideoTooLarge(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartPersonTrackingError {}
/// Errors returned by StartProjectVersion
#[derive(Debug, PartialEq)]
pub enum StartProjectVersionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl StartProjectVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartProjectVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartProjectVersionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartProjectVersionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartProjectVersionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartProjectVersionError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartProjectVersionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartProjectVersionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartProjectVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartProjectVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartProjectVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartProjectVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartProjectVersionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartProjectVersionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartProjectVersionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartProjectVersionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartProjectVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartProjectVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartProjectVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartProjectVersionError {}
/// Errors returned by StartStreamProcessor
#[derive(Debug, PartialEq)]
pub enum StartStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl StartStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartStreamProcessorError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartStreamProcessorError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartStreamProcessorError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartStreamProcessorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartStreamProcessorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartStreamProcessorError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartStreamProcessorError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartStreamProcessorError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartStreamProcessorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartStreamProcessorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartStreamProcessorError {}
/// Errors returned by StopProjectVersion
#[derive(Debug, PartialEq)]
pub enum StopProjectVersionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl StopProjectVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopProjectVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopProjectVersionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StopProjectVersionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopProjectVersionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StopProjectVersionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StopProjectVersionError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopProjectVersionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StopProjectVersionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopProjectVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopProjectVersionError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StopProjectVersionError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StopProjectVersionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopProjectVersionError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StopProjectVersionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StopProjectVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopProjectVersionError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopProjectVersionError {}
/// Errors returned by StopStreamProcessor
#[derive(Debug, PartialEq)]
pub enum StopStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl StopStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopStreamProcessorError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StopStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StopStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StopStreamProcessorError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopStreamProcessorError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StopStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopStreamProcessorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopStreamProcessorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StopStreamProcessorError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StopStreamProcessorError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StopStreamProcessorError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StopStreamProcessorError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopStreamProcessorError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopStreamProcessorError {}
/// Trait representing the capabilities of the Amazon Rekognition API. Amazon Rekognition clients implement this trait.
#[async_trait]
pub trait Rekognition {
    /// <p>Compares a face in the <i>source</i> input image with each of the 100 largest faces detected in the <i>target</i> input image. </p> <note> <p> If the source image contains multiple faces, the service detects the largest face and compares it with each face detected in the target image. </p> </note> <p>You pass the input and target images either as base64-encoded image bytes or as references to images in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes isn't supported. The image must be formatted as a PNG or JPEG file. </p> <p>In response, the operation returns an array of face matches ordered by similarity score in descending order. For each face match, the response provides a bounding box of the face, facial landmarks, pose details (pitch, role, and yaw), quality (brightness and sharpness), and confidence value (indicating the level of confidence that the bounding box contains a face). The response also provides a similarity score, which indicates how closely the faces match. </p> <note> <p>By default, only faces with a similarity score of greater than or equal to 80% are returned in the response. You can change this value by specifying the <code>SimilarityThreshold</code> parameter.</p> </note> <p> <code>CompareFaces</code> also returns an array of faces that don't match the source image. For each face, it returns a bounding box, confidence value, landmarks, pose details, and quality. The response also returns information about the face in the source image, including the bounding box of the face and confidence value.</p> <p>The <code>QualityFilter</code> input parameter allows you to filter out detected faces that don’t meet a required quality bar. The quality bar is based on a variety of common use cases. Use <code>QualityFilter</code> to set the quality bar by specifying <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>. If you do not want to filter detected faces, specify <code>NONE</code>. The default value is <code>NONE</code>. </p> <note> <p>To use quality filtering, you need a collection associated with version 3 of the face model or higher. To get the version of the face model associated with a collection, call <a>DescribeCollection</a>. </p> </note> <p>If the image doesn't contain Exif metadata, <code>CompareFaces</code> returns orientation information for the source and target images. Use these values to display the images with the correct image orientation.</p> <p>If no faces are detected in the source or target images, <code>CompareFaces</code> returns an <code>InvalidParameterException</code> error. </p> <note> <p> This is a stateless API operation. That is, data returned by this operation doesn't persist.</p> </note> <p>For an example, see Comparing Faces in Images in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:CompareFaces</code> action.</p>
    async fn compare_faces(
        &self,
        input: CompareFacesRequest,
    ) -> Result<CompareFacesResponse, RusotoError<CompareFacesError>>;

    /// <p>Creates a collection in an AWS Region. You can add faces to the collection using the <a>IndexFaces</a> operation. </p> <p>For example, you might create collections, one for each of your application users. A user can then index faces using the <code>IndexFaces</code> operation and persist results in a specific collection. Then, a user can search the collection for faces in the user-specific container. </p> <p>When you create a collection, it is associated with the latest version of the face model version.</p> <note> <p>Collection names are case-sensitive.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:CreateCollection</code> action.</p>
    async fn create_collection(
        &self,
        input: CreateCollectionRequest,
    ) -> Result<CreateCollectionResponse, RusotoError<CreateCollectionError>>;

    /// <p>Creates a new Amazon Rekognition Custom Labels project. A project is a logical grouping of resources (images, Labels, models) and operations (training, evaluation and detection). </p> <p>This operation requires permissions to perform the <code>rekognition:CreateProject</code> action.</p>
    async fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> Result<CreateProjectResponse, RusotoError<CreateProjectError>>;

    /// <p>Creates a new version of a model and begins training. Models are managed as part of an Amazon Rekognition Custom Labels project. You can specify one training dataset and one testing dataset. The response from <code>CreateProjectVersion</code> is an Amazon Resource Name (ARN) for the version of the model. </p> <p>Training takes a while to complete. You can get the current status by calling <a>DescribeProjectVersions</a>.</p> <p>Once training has successfully completed, call <a>DescribeProjectVersions</a> to get the training results and evaluate the model. </p> <p>After evaluating the model, you start the model by calling <a>StartProjectVersion</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:CreateProjectVersion</code> action.</p>
    async fn create_project_version(
        &self,
        input: CreateProjectVersionRequest,
    ) -> Result<CreateProjectVersionResponse, RusotoError<CreateProjectVersionError>>;

    /// <p>Creates an Amazon Rekognition stream processor that you can use to detect and recognize faces in a streaming video.</p> <p>Amazon Rekognition Video is a consumer of live video from Amazon Kinesis Video Streams. Amazon Rekognition Video sends analysis results to Amazon Kinesis Data Streams.</p> <p>You provide as input a Kinesis video stream (<code>Input</code>) and a Kinesis data stream (<code>Output</code>) stream. You also specify the face recognition criteria in <code>Settings</code>. For example, the collection containing faces that you want to recognize. Use <code>Name</code> to assign an identifier for the stream processor. You use <code>Name</code> to manage the stream processor. For example, you can start processing the source video by calling <a>StartStreamProcessor</a> with the <code>Name</code> field. </p> <p>After you have finished analyzing a streaming video, use <a>StopStreamProcessor</a> to stop processing. You can delete the stream processor by calling <a>DeleteStreamProcessor</a>.</p>
    async fn create_stream_processor(
        &self,
        input: CreateStreamProcessorRequest,
    ) -> Result<CreateStreamProcessorResponse, RusotoError<CreateStreamProcessorError>>;

    /// <p>Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see <a>delete-collection-procedure</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteCollection</code> action.</p>
    async fn delete_collection(
        &self,
        input: DeleteCollectionRequest,
    ) -> Result<DeleteCollectionResponse, RusotoError<DeleteCollectionError>>;

    /// <p>Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteFaces</code> action.</p>
    async fn delete_faces(
        &self,
        input: DeleteFacesRequest,
    ) -> Result<DeleteFacesResponse, RusotoError<DeleteFacesError>>;

    /// <p>Deletes the stream processor identified by <code>Name</code>. You assign the value for <code>Name</code> when you create the stream processor with <a>CreateStreamProcessor</a>. You might not be able to use the same name for a stream processor for a few seconds after calling <code>DeleteStreamProcessor</code>.</p>
    async fn delete_stream_processor(
        &self,
        input: DeleteStreamProcessorRequest,
    ) -> Result<DeleteStreamProcessorResponse, RusotoError<DeleteStreamProcessorError>>;

    /// <p>Describes the specified collection. You can use <code>DescribeCollection</code> to get information, such as the number of faces indexed into a collection and the version of the model used by the collection for face detection.</p> <p>For more information, see Describing a Collection in the Amazon Rekognition Developer Guide.</p>
    async fn describe_collection(
        &self,
        input: DescribeCollectionRequest,
    ) -> Result<DescribeCollectionResponse, RusotoError<DescribeCollectionError>>;

    /// <p>Lists and describes the models in an Amazon Rekognition Custom Labels project. You can specify up to 10 model versions in <code>ProjectVersionArns</code>. If you don't specify a value, descriptions for all models are returned.</p> <p>This operation requires permissions to perform the <code>rekognition:DescribeProjectVersions</code> action.</p>
    async fn describe_project_versions(
        &self,
        input: DescribeProjectVersionsRequest,
    ) -> Result<DescribeProjectVersionsResponse, RusotoError<DescribeProjectVersionsError>>;

    /// <p>Lists and gets information about your Amazon Rekognition Custom Labels projects.</p> <p>This operation requires permissions to perform the <code>rekognition:DescribeProjects</code> action.</p>
    async fn describe_projects(
        &self,
        input: DescribeProjectsRequest,
    ) -> Result<DescribeProjectsResponse, RusotoError<DescribeProjectsError>>;

    /// <p>Provides information about a stream processor created by <a>CreateStreamProcessor</a>. You can get information about the input and output streams, the input parameters for the face recognition being performed, and the current status of the stream processor.</p>
    async fn describe_stream_processor(
        &self,
        input: DescribeStreamProcessorRequest,
    ) -> Result<DescribeStreamProcessorResponse, RusotoError<DescribeStreamProcessorError>>;

    /// <p>Detects custom labels in a supplied image by using an Amazon Rekognition Custom Labels model. </p> <p>You specify which version of a model version to use by using the <code>ProjectVersionArn</code> input parameter. </p> <p>You pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> For each object that the model version detects on an image, the API returns a (<code>CustomLabel</code>) object in an array (<code>CustomLabels</code>). Each <code>CustomLabel</code> object provides the label name (<code>Name</code>), the level of confidence that the image contains the object (<code>Confidence</code>), and object location information, if it exists, for the label on the image (<code>Geometry</code>). </p> <p>During training model calculates a threshold value that determines if a prediction for a label is true. By default, <code>DetectCustomLabels</code> doesn't return labels whose confidence value is below the model's calculated threshold value. To filter labels that are returned, specify a value for <code>MinConfidence</code> that is higher than the model's calculated threshold. You can get the model's calculated threshold from the model's training results shown in the Amazon Rekognition Custom Labels console. To get all labels, regardless of confidence, specify a <code>MinConfidence</code> value of 0. </p> <p>You can also add the <code>MaxResults</code> parameter to limit the number of labels returned. </p> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectCustomLabels</code> action. </p>
    async fn detect_custom_labels(
        &self,
        input: DetectCustomLabelsRequest,
    ) -> Result<DetectCustomLabelsResponse, RusotoError<DetectCustomLabelsError>>;

    /// <p>Detects faces within an image that is provided as input.</p> <p> <code>DetectFaces</code> detects the 100 largest faces in the image. For each face detected, the operation returns face details. These details include a bounding box of the face, a confidence value (that the bounding box contains a face), and a fixed set of attributes such as facial landmarks (for example, coordinates of eye and mouth), presence of beard, sunglasses, and so on. </p> <p>The face-detection algorithm is most effective on frontal faces. For non-frontal or obscured faces, the algorithm might not detect the faces or might detect faces with lower confidence. </p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:DetectFaces</code> action. </p>
    async fn detect_faces(
        &self,
        input: DetectFacesRequest,
    ) -> Result<DetectFacesResponse, RusotoError<DetectFacesError>>;

    /// <p>Detects instances of real-world entities within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like landscape, evening, and nature. </p> <p>For an example, see Analyzing Images Stored in an Amazon S3 Bucket in the Amazon Rekognition Developer Guide.</p> <note> <p> <code>DetectLabels</code> does not support the detection of activities. However, activity detection is supported for label detection in videos. For more information, see StartLabelDetection in the Amazon Rekognition Developer Guide.</p> </note> <p>You pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> For each object, scene, and concept the API returns one or more labels. Each label provides the object name, and the level of confidence that the image contains the object. For example, suppose the input image has a lighthouse, the sea, and a rock. The response includes all three labels, one for each object. </p> <p> <code>{Name: lighthouse, Confidence: 98.4629}</code> </p> <p> <code>{Name: rock,Confidence: 79.2097}</code> </p> <p> <code> {Name: sea,Confidence: 75.061}</code> </p> <p>In the preceding example, the operation returns one label for each of the three objects. The operation can also return multiple labels for the same object in the image. For example, if the input image shows a flower (for example, a tulip), the operation might return the following three labels. </p> <p> <code>{Name: flower,Confidence: 99.0562}</code> </p> <p> <code>{Name: plant,Confidence: 99.0562}</code> </p> <p> <code>{Name: tulip,Confidence: 99.0562}</code> </p> <p>In this example, the detection algorithm more precisely identifies the flower as a tulip.</p> <p>In response, the API returns an array of labels. In addition, the response also includes the orientation correction. Optionally, you can specify <code>MinConfidence</code> to control the confidence threshold for the labels returned. The default is 55%. You can also add the <code>MaxLabels</code> parameter to limit the number of labels returned. </p> <note> <p>If the object detected is a person, the operation doesn't provide the same facial details that the <a>DetectFaces</a> operation provides.</p> </note> <p> <code>DetectLabels</code> returns bounding boxes for instances of common object labels in an array of <a>Instance</a> objects. An <code>Instance</code> object contains a <a>BoundingBox</a> object, for the location of the label on the image. It also includes the confidence by which the bounding box was detected.</p> <p> <code>DetectLabels</code> also returns a hierarchical taxonomy of detected labels. For example, a detected car might be assigned the label <i>car</i>. The label <i>car</i> has two parent labels: <i>Vehicle</i> (its parent) and <i>Transportation</i> (its grandparent). The response returns the entire list of ancestors for a label. Each ancestor is a unique label in the response. In the previous example, <i>Car</i>, <i>Vehicle</i>, and <i>Transportation</i> are returned as unique labels in the response. </p> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectLabels</code> action. </p>
    async fn detect_labels(
        &self,
        input: DetectLabelsRequest,
    ) -> Result<DetectLabelsResponse, RusotoError<DetectLabelsError>>;

    /// <p>Detects unsafe content in a specified JPEG or PNG format image. Use <code>DetectModerationLabels</code> to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not images containing suggestive content.</p> <p>To filter images, use the labels returned by <code>DetectModerationLabels</code> to determine which types of content are appropriate.</p> <p>For information about moderation labels, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p>
    async fn detect_moderation_labels(
        &self,
        input: DetectModerationLabelsRequest,
    ) -> Result<DetectModerationLabelsResponse, RusotoError<DetectModerationLabelsError>>;

    /// <p>Detects text in the input image and converts it into machine-readable text.</p> <p>Pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, you must pass it as a reference to an image in an Amazon S3 bucket. For the AWS CLI, passing image bytes is not supported. The image must be either a .png or .jpeg formatted file. </p> <p>The <code>DetectText</code> operation returns text in an array of <a>TextDetection</a> elements, <code>TextDetections</code>. Each <code>TextDetection</code> element provides information about a single word or line of text that was detected in the image. </p> <p>A word is one or more ISO basic latin script characters that are not separated by spaces. <code>DetectText</code> can detect up to 50 words in an image.</p> <p>A line is a string of equally spaced words. A line isn't necessarily a complete sentence. For example, a driver's license number is detected as a line. A line ends when there is no aligned text after it. Also, a line ends when there is a large gap between words, relative to the length of the words. This means, depending on the gap between words, Amazon Rekognition may detect multiple lines in text aligned in the same direction. Periods don't represent the end of a line. If a sentence spans multiple lines, the <code>DetectText</code> operation returns multiple lines.</p> <p>To determine whether a <code>TextDetection</code> element is a line of text or a word, use the <code>TextDetection</code> object <code>Type</code> field. </p> <p>To be detected, text must be within +/- 90 degrees orientation of the horizontal axis.</p> <p>For more information, see DetectText in the Amazon Rekognition Developer Guide.</p>
    async fn detect_text(
        &self,
        input: DetectTextRequest,
    ) -> Result<DetectTextResponse, RusotoError<DetectTextError>>;

    /// <p>Gets the name and additional information about a celebrity based on his or her Amazon Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, this list is empty.</p> <p>For more information, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:GetCelebrityInfo</code> action. </p>
    async fn get_celebrity_info(
        &self,
        input: GetCelebrityInfoRequest,
    ) -> Result<GetCelebrityInfoResponse, RusotoError<GetCelebrityInfoError>>;

    /// <p>Gets the celebrity recognition results for a Amazon Rekognition Video analysis started by <a>StartCelebrityRecognition</a>.</p> <p>Celebrity recognition in a video is an asynchronous operation. Analysis is started by a call to <a>StartCelebrityRecognition</a> which returns a job identifier (<code>JobId</code>). When the celebrity recognition operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartCelebrityRecognition</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetCelebrityDetection</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityDetection</code>. </p> <p>For more information, see Working With Stored Videos in the Amazon Rekognition Developer Guide.</p> <p> <code>GetCelebrityRecognition</code> returns detected celebrities and the time(s) they are detected in an array (<code>Celebrities</code>) of <a>CelebrityRecognition</a> objects. Each <code>CelebrityRecognition</code> contains information about the celebrity in a <a>CelebrityDetail</a> object and the time, <code>Timestamp</code>, the celebrity was detected. </p> <note> <p> <code>GetCelebrityRecognition</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Celebrities</code> array is sorted by time (milliseconds from the start of the video). You can also sort the array by celebrity by specifying the value <code>ID</code> in the <code>SortBy</code> input parameter.</p> <p>The <code>CelebrityDetail</code> object includes the celebrity identifer and additional information urls. If you don't store the additional information urls, you can get them later by calling <a>GetCelebrityInfo</a> with the celebrity identifer.</p> <p>No information is returned for faces not recognized as celebrities.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetCelebrityDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetCelebrityRecognition</code>.</p>
    async fn get_celebrity_recognition(
        &self,
        input: GetCelebrityRecognitionRequest,
    ) -> Result<GetCelebrityRecognitionResponse, RusotoError<GetCelebrityRecognitionError>>;

    /// <p>Gets the unsafe content analysis results for a Amazon Rekognition Video analysis started by <a>StartContentModeration</a>.</p> <p>Unsafe content analysis of a video is an asynchronous operation. You start analysis by calling <a>StartContentModeration</a> which returns a job identifier (<code>JobId</code>). When analysis finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartContentModeration</code>. To get the results of the unsafe content analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetContentModeration</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Working with Stored Videos in the Amazon Rekognition Devlopers Guide.</p> <p> <code>GetContentModeration</code> returns detected unsafe content labels, and the time they are detected, in an array, <code>ModerationLabels</code>, of <a>ContentModerationDetection</a> objects. </p> <p>By default, the moderated labels are returned sorted by time, in milliseconds from the start of the video. You can also sort them by moderated label by specifying <code>NAME</code> for the <code>SortBy</code> input parameter. </p> <p>Since video analysis can return a large number of results, use the <code>MaxResults</code> parameter to limit the number of labels returned in a single call to <code>GetContentModeration</code>. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetContentModeration</code> and populate the <code>NextToken</code> request parameter with the value of <code>NextToken</code> returned from the previous call to <code>GetContentModeration</code>.</p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    async fn get_content_moderation(
        &self,
        input: GetContentModerationRequest,
    ) -> Result<GetContentModerationResponse, RusotoError<GetContentModerationError>>;

    /// <p>Gets face detection results for a Amazon Rekognition Video analysis started by <a>StartFaceDetection</a>.</p> <p>Face detection with Amazon Rekognition Video is an asynchronous operation. You start face detection by calling <a>StartFaceDetection</a> which returns a job identifier (<code>JobId</code>). When the face detection operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceDetection</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p> <code>GetFaceDetection</code> returns an array of detected faces (<code>Faces</code>) sorted by the time the faces were detected. </p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetFaceDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetFaceDetection</code>.</p>
    async fn get_face_detection(
        &self,
        input: GetFaceDetectionRequest,
    ) -> Result<GetFaceDetectionResponse, RusotoError<GetFaceDetectionError>>;

    /// <p>Gets the face search results for Amazon Rekognition Video face search started by <a>StartFaceSearch</a>. The search returns faces in a collection that match the faces of persons detected in a video. It also includes the time(s) that faces are matched in the video.</p> <p>Face search in a video is an asynchronous operation. You start face search by calling to <a>StartFaceSearch</a> which returns a job identifier (<code>JobId</code>). When the search operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceSearch</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetFaceSearch</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>.</p> <p>For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>The search results are retured in an array, <code>Persons</code>, of <a>PersonMatch</a> objects. Each<code>PersonMatch</code> element contains details about the matching faces in the input collection, person information (facial attributes, bounding boxes, and person identifer) for the matched person, and the time the person was matched in the video.</p> <note> <p> <code>GetFaceSearch</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Persons</code> array is sorted by the time, in milliseconds from the start of the video, persons are matched. You can also sort by persons by specifying <code>INDEX</code> for the <code>SORTBY</code> input parameter.</p>
    async fn get_face_search(
        &self,
        input: GetFaceSearchRequest,
    ) -> Result<GetFaceSearchResponse, RusotoError<GetFaceSearchError>>;

    /// <p>Gets the label detection results of a Amazon Rekognition Video analysis started by <a>StartLabelDetection</a>. </p> <p>The label detection operation is started by a call to <a>StartLabelDetection</a> which returns a job identifier (<code>JobId</code>). When the label detection operation finishes, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartlabelDetection</code>. To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetLabelDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p> <code>GetLabelDetection</code> returns an array of detected labels (<code>Labels</code>) sorted by the time the labels were detected. You can also sort by the label name by specifying <code>NAME</code> for the <code>SortBy</code> input parameter.</p> <p>The labels returned include the label name, the percentage confidence in the accuracy of the detected label, and the time the label was detected in the video.</p> <p>The returned labels also include bounding box information for common objects, a hierarchical taxonomy of detected labels, and the version of the label model used for detection.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetlabelDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetLabelDetection</code>.</p>
    async fn get_label_detection(
        &self,
        input: GetLabelDetectionRequest,
    ) -> Result<GetLabelDetectionResponse, RusotoError<GetLabelDetectionError>>;

    /// <p>Gets the path tracking results of a Amazon Rekognition Video analysis started by <a>StartPersonTracking</a>.</p> <p>The person path tracking operation is started by a call to <code>StartPersonTracking</code> which returns a job identifier (<code>JobId</code>). When the operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartPersonTracking</code>.</p> <p>To get the results of the person path tracking operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetPersonTracking</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p> <p> <code>GetPersonTracking</code> returns an array, <code>Persons</code>, of tracked persons and the time(s) their paths were tracked in the video. </p> <note> <p> <code>GetPersonTracking</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. </p> <p>For more information, see FaceDetail in the Amazon Rekognition Developer Guide.</p> </note> <p>By default, the array is sorted by the time(s) a person's path is tracked in the video. You can sort by tracked persons by specifying <code>INDEX</code> for the <code>SortBy</code> input parameter.</p> <p>Use the <code>MaxResults</code> parameter to limit the number of items returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetPersonTracking</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetPersonTracking</code>.</p>
    async fn get_person_tracking(
        &self,
        input: GetPersonTrackingRequest,
    ) -> Result<GetPersonTrackingResponse, RusotoError<GetPersonTrackingError>>;

    /// <p>Detects faces in the input image and adds them to the specified collection. </p> <p>Amazon Rekognition doesn't save the actual faces that are detected. Instead, the underlying detection algorithm first detects the faces in the input image. For each face, the algorithm extracts facial features into a feature vector, and stores it in the backend database. Amazon Rekognition uses feature vectors when it performs face match and search operations using the <a>SearchFaces</a> and <a>SearchFacesByImage</a> operations.</p> <p>For more information, see Adding Faces to a Collection in the Amazon Rekognition Developer Guide.</p> <p>To get the number of faces in a collection, call <a>DescribeCollection</a>. </p> <p>If you're using version 1.0 of the face detection model, <code>IndexFaces</code> indexes the 15 largest faces in the input image. Later versions of the face detection model index the 100 largest faces in the input image. </p> <p>If you're using version 4 or later of the face model, image orientation information is not returned in the <code>OrientationCorrection</code> field. </p> <p>To determine which version of the model you're using, call <a>DescribeCollection</a> and supply the collection ID. You can also get the model version from the value of <code>FaceModelVersion</code> in the response from <code>IndexFaces</code> </p> <p>For more information, see Model Versioning in the Amazon Rekognition Developer Guide.</p> <p>If you provide the optional <code>ExternalImageID</code> for the input image you provided, Amazon Rekognition associates this ID with all faces that it detects. When you call the <a>ListFaces</a> operation, the response returns the external ID. You can use this external image ID to create a client-side index to associate the faces with each image. You can then use the index to find all faces in an image.</p> <p>You can specify the maximum number of faces to index with the <code>MaxFaces</code> input parameter. This is useful when you want to index the largest faces in an image and don't want to index smaller faces, such as those belonging to people standing in the background.</p> <p>The <code>QualityFilter</code> input parameter allows you to filter out detected faces that don’t meet a required quality bar. The quality bar is based on a variety of common use cases. By default, <code>IndexFaces</code> chooses the quality bar that's used to filter faces. You can also explicitly choose the quality bar. Use <code>QualityFilter</code>, to set the quality bar by specifying <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>. If you do not want to filter detected faces, specify <code>NONE</code>. </p> <note> <p>To use quality filtering, you need a collection associated with version 3 of the face model or higher. To get the version of the face model associated with a collection, call <a>DescribeCollection</a>. </p> </note> <p>Information about faces detected in an image, but not indexed, is returned in an array of <a>UnindexedFace</a> objects, <code>UnindexedFaces</code>. Faces aren't indexed for reasons such as:</p> <ul> <li> <p>The number of faces detected exceeds the value of the <code>MaxFaces</code> request parameter.</p> </li> <li> <p>The face is too small compared to the image dimensions.</p> </li> <li> <p>The face is too blurry.</p> </li> <li> <p>The image is too dark.</p> </li> <li> <p>The face has an extreme pose.</p> </li> <li> <p>The face doesn’t have enough detail to be suitable for face search.</p> </li> </ul> <p>In response, the <code>IndexFaces</code> operation returns an array of metadata for all detected faces, <code>FaceRecords</code>. This includes: </p> <ul> <li> <p>The bounding box, <code>BoundingBox</code>, of the detected face. </p> </li> <li> <p>A confidence value, <code>Confidence</code>, which indicates the confidence that the bounding box contains a face.</p> </li> <li> <p>A face ID, <code>FaceId</code>, assigned by the service for each face that's detected and stored.</p> </li> <li> <p>An image ID, <code>ImageId</code>, assigned by the service for the input image.</p> </li> </ul> <p>If you request all facial attributes (by using the <code>detectionAttributes</code> parameter), Amazon Rekognition returns detailed facial attributes, such as facial landmarks (for example, location of eye and mouth) and other facial attributes. If you provide the same image, specify the same collection, and use the same external ID in the <code>IndexFaces</code> operation, Amazon Rekognition doesn't save duplicate face metadata.</p> <p/> <p>The input image is passed either as base64-encoded image bytes, or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes isn't supported. The image must be formatted as a PNG or JPEG file. </p> <p>This operation requires permissions to perform the <code>rekognition:IndexFaces</code> action.</p>
    async fn index_faces(
        &self,
        input: IndexFacesRequest,
    ) -> Result<IndexFacesResponse, RusotoError<IndexFacesError>>;

    /// <p>Returns list of collection IDs in your account. If the result is truncated, the response also provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p> <p>For an example, see Listing Collections in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListCollections</code> action.</p>
    async fn list_collections(
        &self,
        input: ListCollectionsRequest,
    ) -> Result<ListCollectionsResponse, RusotoError<ListCollectionsError>>;

    /// <p>Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, see Listing Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListFaces</code> action.</p>
    async fn list_faces(
        &self,
        input: ListFacesRequest,
    ) -> Result<ListFacesResponse, RusotoError<ListFacesError>>;

    /// <p>Gets a list of stream processors that you have created with <a>CreateStreamProcessor</a>. </p>
    async fn list_stream_processors(
        &self,
        input: ListStreamProcessorsRequest,
    ) -> Result<ListStreamProcessorsResponse, RusotoError<ListStreamProcessorsError>>;

    /// <p>Returns an array of celebrities recognized in the input image. For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide. </p> <p> <code>RecognizeCelebrities</code> returns the 100 largest faces in the image. It lists recognized celebrities in the <code>CelebrityFaces</code> array and unrecognized faces in the <code>UnrecognizedFaces</code> array. <code>RecognizeCelebrities</code> doesn't return celebrities whose faces aren't among the largest 100 faces in the image.</p> <p>For each celebrity recognized, <code>RecognizeCelebrities</code> returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p> <p>Amazon Rekognition doesn't retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the <a>GetCelebrityInfo</a> operation.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>For an example, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>
    async fn recognize_celebrities(
        &self,
        input: RecognizeCelebritiesRequest,
    ) -> Result<RecognizeCelebritiesResponse, RusotoError<RecognizeCelebritiesError>>;

    /// <p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <a>IndexFaces</a> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p> </note> <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p> <p>For an example, see Searching for a Face Using Its Face ID in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>
    async fn search_faces(
        &self,
        input: SearchFacesRequest,
    ) -> Result<SearchFacesResponse, RusotoError<SearchFacesError>>;

    /// <p>For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>To search for all faces in an input image, you might first call the <a>IndexFaces</a> operation, and then use the face IDs returned in subsequent calls to the <a>SearchFaces</a> operation. </p> <p> You can also call the <code>DetectFaces</code> operation and use the bounding boxes in the response to make face crops, which then you can pass in to the <code>SearchFacesByImage</code> operation. </p> </note> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> The response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match found. Along with the metadata, the response also includes a <code>similarity</code> indicating how similar the face is to the input face. In the response, the operation also returns the bounding box (and a confidence level that the bounding box contains a face) of the face that Amazon Rekognition used for the input image. </p> <p>For an example, Searching for a Face Using an Image in the Amazon Rekognition Developer Guide.</p> <p>The <code>QualityFilter</code> input parameter allows you to filter out detected faces that don’t meet a required quality bar. The quality bar is based on a variety of common use cases. Use <code>QualityFilter</code> to set the quality bar for filtering by specifying <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>. If you do not want to filter detected faces, specify <code>NONE</code>. The default value is <code>NONE</code>.</p> <note> <p>To use quality filtering, you need a collection associated with version 3 of the face model or higher. To get the version of the face model associated with a collection, call <a>DescribeCollection</a>. </p> </note> <p>This operation requires permissions to perform the <code>rekognition:SearchFacesByImage</code> action.</p>
    async fn search_faces_by_image(
        &self,
        input: SearchFacesByImageRequest,
    ) -> Result<SearchFacesByImageResponse, RusotoError<SearchFacesByImageError>>;

    /// <p>Starts asynchronous recognition of celebrities in a stored video.</p> <p>Amazon Rekognition Video can detect celebrities in a video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartCelebrityRecognition</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When celebrity recognition analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetCelebrityRecognition</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityRecognition</code>. </p> <p>For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide.</p>
    async fn start_celebrity_recognition(
        &self,
        input: StartCelebrityRecognitionRequest,
    ) -> Result<StartCelebrityRecognitionResponse, RusotoError<StartCelebrityRecognitionError>>;

    /// <p> Starts asynchronous detection of unsafe content in a stored video.</p> <p>Amazon Rekognition Video can moderate content in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartContentModeration</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When unsafe content analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the unsafe content analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetContentModeration</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    async fn start_content_moderation(
        &self,
        input: StartContentModerationRequest,
    ) -> Result<StartContentModerationResponse, RusotoError<StartContentModerationError>>;

    /// <p>Starts asynchronous detection of faces in a stored video.</p> <p>Amazon Rekognition Video can detect faces in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceDetection</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When face detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p>For more information, see Detecting Faces in a Stored Video in the Amazon Rekognition Developer Guide.</p>
    async fn start_face_detection(
        &self,
        input: StartFaceDetectionRequest,
    ) -> Result<StartFaceDetectionResponse, RusotoError<StartFaceDetectionError>>;

    /// <p>Starts the asynchronous search for faces in a collection that match the faces of persons detected in a stored video.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceSearch</code> returns a job identifier (<code>JobId</code>) which you use to get the search results once the search has completed. When searching is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceSearch</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>. For more information, see <a>procedure-person-search-videos</a>.</p>
    async fn start_face_search(
        &self,
        input: StartFaceSearchRequest,
    ) -> Result<StartFaceSearchResponse, RusotoError<StartFaceSearchError>>;

    /// <p><p>Starts asynchronous detection of labels in a stored video.</p> <p>Amazon Rekognition Video can detect labels in a video. Labels are instances of real-world entities. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; concepts like landscape, evening, and nature; and activities like a person getting out of a car or a person skiing.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartLabelDetection</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetLabelDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p/></p>
    async fn start_label_detection(
        &self,
        input: StartLabelDetectionRequest,
    ) -> Result<StartLabelDetectionResponse, RusotoError<StartLabelDetectionError>>;

    /// <p>Starts the asynchronous tracking of a person's path in a stored video.</p> <p>Amazon Rekognition Video can track the path of people in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartPersonTracking</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. </p> <p>To get the results of the person detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetPersonTracking</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p>
    async fn start_person_tracking(
        &self,
        input: StartPersonTrackingRequest,
    ) -> Result<StartPersonTrackingResponse, RusotoError<StartPersonTrackingError>>;

    /// <p>Starts the running of the version of a model. Starting a model takes a while to complete. To check the current state of the model, use <a>DescribeProjectVersions</a>.</p> <p>Once the model is running, you can detect custom labels in new images by calling <a>DetectCustomLabels</a>.</p> <note> <p>You are charged for the amount of time that the model is running. To stop a running model, call <a>StopProjectVersion</a>.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:StartProjectVersion</code> action.</p>
    async fn start_project_version(
        &self,
        input: StartProjectVersionRequest,
    ) -> Result<StartProjectVersionResponse, RusotoError<StartProjectVersionError>>;

    /// <p>Starts processing a stream processor. You create a stream processor by calling <a>CreateStreamProcessor</a>. To tell <code>StartStreamProcessor</code> which stream processor to start, use the value of the <code>Name</code> field specified in the call to <code>CreateStreamProcessor</code>.</p>
    async fn start_stream_processor(
        &self,
        input: StartStreamProcessorRequest,
    ) -> Result<StartStreamProcessorResponse, RusotoError<StartStreamProcessorError>>;

    /// <p>Stops a running model. The operation might take a while to complete. To check the current status, call <a>DescribeProjectVersions</a>. </p>
    async fn stop_project_version(
        &self,
        input: StopProjectVersionRequest,
    ) -> Result<StopProjectVersionResponse, RusotoError<StopProjectVersionError>>;

    /// <p>Stops a running stream processor that was created by <a>CreateStreamProcessor</a>.</p>
    async fn stop_stream_processor(
        &self,
        input: StopStreamProcessorRequest,
    ) -> Result<StopStreamProcessorResponse, RusotoError<StopStreamProcessorError>>;
}
/// A client for the Amazon Rekognition API.
#[derive(Clone)]
pub struct RekognitionClient {
    client: Client,
    region: region::Region,
}

impl RekognitionClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> RekognitionClient {
        RekognitionClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> RekognitionClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        RekognitionClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> RekognitionClient {
        RekognitionClient { client, region }
    }
}

#[async_trait]
impl Rekognition for RekognitionClient {
    /// <p>Compares a face in the <i>source</i> input image with each of the 100 largest faces detected in the <i>target</i> input image. </p> <note> <p> If the source image contains multiple faces, the service detects the largest face and compares it with each face detected in the target image. </p> </note> <p>You pass the input and target images either as base64-encoded image bytes or as references to images in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes isn't supported. The image must be formatted as a PNG or JPEG file. </p> <p>In response, the operation returns an array of face matches ordered by similarity score in descending order. For each face match, the response provides a bounding box of the face, facial landmarks, pose details (pitch, role, and yaw), quality (brightness and sharpness), and confidence value (indicating the level of confidence that the bounding box contains a face). The response also provides a similarity score, which indicates how closely the faces match. </p> <note> <p>By default, only faces with a similarity score of greater than or equal to 80% are returned in the response. You can change this value by specifying the <code>SimilarityThreshold</code> parameter.</p> </note> <p> <code>CompareFaces</code> also returns an array of faces that don't match the source image. For each face, it returns a bounding box, confidence value, landmarks, pose details, and quality. The response also returns information about the face in the source image, including the bounding box of the face and confidence value.</p> <p>The <code>QualityFilter</code> input parameter allows you to filter out detected faces that don’t meet a required quality bar. The quality bar is based on a variety of common use cases. Use <code>QualityFilter</code> to set the quality bar by specifying <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>. If you do not want to filter detected faces, specify <code>NONE</code>. The default value is <code>NONE</code>. </p> <note> <p>To use quality filtering, you need a collection associated with version 3 of the face model or higher. To get the version of the face model associated with a collection, call <a>DescribeCollection</a>. </p> </note> <p>If the image doesn't contain Exif metadata, <code>CompareFaces</code> returns orientation information for the source and target images. Use these values to display the images with the correct image orientation.</p> <p>If no faces are detected in the source or target images, <code>CompareFaces</code> returns an <code>InvalidParameterException</code> error. </p> <note> <p> This is a stateless API operation. That is, data returned by this operation doesn't persist.</p> </note> <p>For an example, see Comparing Faces in Images in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:CompareFaces</code> action.</p>
    async fn compare_faces(
        &self,
        input: CompareFacesRequest,
    ) -> Result<CompareFacesResponse, RusotoError<CompareFacesError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CompareFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CompareFacesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CompareFacesError::from_response(response))
        }
    }

    /// <p>Creates a collection in an AWS Region. You can add faces to the collection using the <a>IndexFaces</a> operation. </p> <p>For example, you might create collections, one for each of your application users. A user can then index faces using the <code>IndexFaces</code> operation and persist results in a specific collection. Then, a user can search the collection for faces in the user-specific container. </p> <p>When you create a collection, it is associated with the latest version of the face model version.</p> <note> <p>Collection names are case-sensitive.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:CreateCollection</code> action.</p>
    async fn create_collection(
        &self,
        input: CreateCollectionRequest,
    ) -> Result<CreateCollectionResponse, RusotoError<CreateCollectionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateCollection");
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
                .deserialize::<CreateCollectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCollectionError::from_response(response))
        }
    }

    /// <p>Creates a new Amazon Rekognition Custom Labels project. A project is a logical grouping of resources (images, Labels, models) and operations (training, evaluation and detection). </p> <p>This operation requires permissions to perform the <code>rekognition:CreateProject</code> action.</p>
    async fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> Result<CreateProjectResponse, RusotoError<CreateProjectError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateProjectResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProjectError::from_response(response))
        }
    }

    /// <p>Creates a new version of a model and begins training. Models are managed as part of an Amazon Rekognition Custom Labels project. You can specify one training dataset and one testing dataset. The response from <code>CreateProjectVersion</code> is an Amazon Resource Name (ARN) for the version of the model. </p> <p>Training takes a while to complete. You can get the current status by calling <a>DescribeProjectVersions</a>.</p> <p>Once training has successfully completed, call <a>DescribeProjectVersions</a> to get the training results and evaluate the model. </p> <p>After evaluating the model, you start the model by calling <a>StartProjectVersion</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:CreateProjectVersion</code> action.</p>
    async fn create_project_version(
        &self,
        input: CreateProjectVersionRequest,
    ) -> Result<CreateProjectVersionResponse, RusotoError<CreateProjectVersionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateProjectVersion");
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
                .deserialize::<CreateProjectVersionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateProjectVersionError::from_response(response))
        }
    }

    /// <p>Creates an Amazon Rekognition stream processor that you can use to detect and recognize faces in a streaming video.</p> <p>Amazon Rekognition Video is a consumer of live video from Amazon Kinesis Video Streams. Amazon Rekognition Video sends analysis results to Amazon Kinesis Data Streams.</p> <p>You provide as input a Kinesis video stream (<code>Input</code>) and a Kinesis data stream (<code>Output</code>) stream. You also specify the face recognition criteria in <code>Settings</code>. For example, the collection containing faces that you want to recognize. Use <code>Name</code> to assign an identifier for the stream processor. You use <code>Name</code> to manage the stream processor. For example, you can start processing the source video by calling <a>StartStreamProcessor</a> with the <code>Name</code> field. </p> <p>After you have finished analyzing a streaming video, use <a>StopStreamProcessor</a> to stop processing. You can delete the stream processor by calling <a>DeleteStreamProcessor</a>.</p>
    async fn create_stream_processor(
        &self,
        input: CreateStreamProcessorRequest,
    ) -> Result<CreateStreamProcessorResponse, RusotoError<CreateStreamProcessorError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateStreamProcessor");
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
                .deserialize::<CreateStreamProcessorResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateStreamProcessorError::from_response(response))
        }
    }

    /// <p>Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see <a>delete-collection-procedure</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteCollection</code> action.</p>
    async fn delete_collection(
        &self,
        input: DeleteCollectionRequest,
    ) -> Result<DeleteCollectionResponse, RusotoError<DeleteCollectionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteCollection");
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
                .deserialize::<DeleteCollectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCollectionError::from_response(response))
        }
    }

    /// <p>Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteFaces</code> action.</p>
    async fn delete_faces(
        &self,
        input: DeleteFacesRequest,
    ) -> Result<DeleteFacesResponse, RusotoError<DeleteFacesError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteFacesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFacesError::from_response(response))
        }
    }

    /// <p>Deletes the stream processor identified by <code>Name</code>. You assign the value for <code>Name</code> when you create the stream processor with <a>CreateStreamProcessor</a>. You might not be able to use the same name for a stream processor for a few seconds after calling <code>DeleteStreamProcessor</code>.</p>
    async fn delete_stream_processor(
        &self,
        input: DeleteStreamProcessorRequest,
    ) -> Result<DeleteStreamProcessorResponse, RusotoError<DeleteStreamProcessorError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteStreamProcessor");
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
                .deserialize::<DeleteStreamProcessorResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteStreamProcessorError::from_response(response))
        }
    }

    /// <p>Describes the specified collection. You can use <code>DescribeCollection</code> to get information, such as the number of faces indexed into a collection and the version of the model used by the collection for face detection.</p> <p>For more information, see Describing a Collection in the Amazon Rekognition Developer Guide.</p>
    async fn describe_collection(
        &self,
        input: DescribeCollectionRequest,
    ) -> Result<DescribeCollectionResponse, RusotoError<DescribeCollectionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DescribeCollection");
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
                .deserialize::<DescribeCollectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeCollectionError::from_response(response))
        }
    }

    /// <p>Lists and describes the models in an Amazon Rekognition Custom Labels project. You can specify up to 10 model versions in <code>ProjectVersionArns</code>. If you don't specify a value, descriptions for all models are returned.</p> <p>This operation requires permissions to perform the <code>rekognition:DescribeProjectVersions</code> action.</p>
    async fn describe_project_versions(
        &self,
        input: DescribeProjectVersionsRequest,
    ) -> Result<DescribeProjectVersionsResponse, RusotoError<DescribeProjectVersionsError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DescribeProjectVersions");
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
                .deserialize::<DescribeProjectVersionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProjectVersionsError::from_response(response))
        }
    }

    /// <p>Lists and gets information about your Amazon Rekognition Custom Labels projects.</p> <p>This operation requires permissions to perform the <code>rekognition:DescribeProjects</code> action.</p>
    async fn describe_projects(
        &self,
        input: DescribeProjectsRequest,
    ) -> Result<DescribeProjectsResponse, RusotoError<DescribeProjectsError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DescribeProjects");
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
                .deserialize::<DescribeProjectsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeProjectsError::from_response(response))
        }
    }

    /// <p>Provides information about a stream processor created by <a>CreateStreamProcessor</a>. You can get information about the input and output streams, the input parameters for the face recognition being performed, and the current status of the stream processor.</p>
    async fn describe_stream_processor(
        &self,
        input: DescribeStreamProcessorRequest,
    ) -> Result<DescribeStreamProcessorResponse, RusotoError<DescribeStreamProcessorError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DescribeStreamProcessor");
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
                .deserialize::<DescribeStreamProcessorResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeStreamProcessorError::from_response(response))
        }
    }

    /// <p>Detects custom labels in a supplied image by using an Amazon Rekognition Custom Labels model. </p> <p>You specify which version of a model version to use by using the <code>ProjectVersionArn</code> input parameter. </p> <p>You pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> For each object that the model version detects on an image, the API returns a (<code>CustomLabel</code>) object in an array (<code>CustomLabels</code>). Each <code>CustomLabel</code> object provides the label name (<code>Name</code>), the level of confidence that the image contains the object (<code>Confidence</code>), and object location information, if it exists, for the label on the image (<code>Geometry</code>). </p> <p>During training model calculates a threshold value that determines if a prediction for a label is true. By default, <code>DetectCustomLabels</code> doesn't return labels whose confidence value is below the model's calculated threshold value. To filter labels that are returned, specify a value for <code>MinConfidence</code> that is higher than the model's calculated threshold. You can get the model's calculated threshold from the model's training results shown in the Amazon Rekognition Custom Labels console. To get all labels, regardless of confidence, specify a <code>MinConfidence</code> value of 0. </p> <p>You can also add the <code>MaxResults</code> parameter to limit the number of labels returned. </p> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectCustomLabels</code> action. </p>
    async fn detect_custom_labels(
        &self,
        input: DetectCustomLabelsRequest,
    ) -> Result<DetectCustomLabelsResponse, RusotoError<DetectCustomLabelsError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectCustomLabels");
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
                .deserialize::<DetectCustomLabelsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectCustomLabelsError::from_response(response))
        }
    }

    /// <p>Detects faces within an image that is provided as input.</p> <p> <code>DetectFaces</code> detects the 100 largest faces in the image. For each face detected, the operation returns face details. These details include a bounding box of the face, a confidence value (that the bounding box contains a face), and a fixed set of attributes such as facial landmarks (for example, coordinates of eye and mouth), presence of beard, sunglasses, and so on. </p> <p>The face-detection algorithm is most effective on frontal faces. For non-frontal or obscured faces, the algorithm might not detect the faces or might detect faces with lower confidence. </p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:DetectFaces</code> action. </p>
    async fn detect_faces(
        &self,
        input: DetectFacesRequest,
    ) -> Result<DetectFacesResponse, RusotoError<DetectFacesError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DetectFacesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectFacesError::from_response(response))
        }
    }

    /// <p>Detects instances of real-world entities within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like landscape, evening, and nature. </p> <p>For an example, see Analyzing Images Stored in an Amazon S3 Bucket in the Amazon Rekognition Developer Guide.</p> <note> <p> <code>DetectLabels</code> does not support the detection of activities. However, activity detection is supported for label detection in videos. For more information, see StartLabelDetection in the Amazon Rekognition Developer Guide.</p> </note> <p>You pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> For each object, scene, and concept the API returns one or more labels. Each label provides the object name, and the level of confidence that the image contains the object. For example, suppose the input image has a lighthouse, the sea, and a rock. The response includes all three labels, one for each object. </p> <p> <code>{Name: lighthouse, Confidence: 98.4629}</code> </p> <p> <code>{Name: rock,Confidence: 79.2097}</code> </p> <p> <code> {Name: sea,Confidence: 75.061}</code> </p> <p>In the preceding example, the operation returns one label for each of the three objects. The operation can also return multiple labels for the same object in the image. For example, if the input image shows a flower (for example, a tulip), the operation might return the following three labels. </p> <p> <code>{Name: flower,Confidence: 99.0562}</code> </p> <p> <code>{Name: plant,Confidence: 99.0562}</code> </p> <p> <code>{Name: tulip,Confidence: 99.0562}</code> </p> <p>In this example, the detection algorithm more precisely identifies the flower as a tulip.</p> <p>In response, the API returns an array of labels. In addition, the response also includes the orientation correction. Optionally, you can specify <code>MinConfidence</code> to control the confidence threshold for the labels returned. The default is 55%. You can also add the <code>MaxLabels</code> parameter to limit the number of labels returned. </p> <note> <p>If the object detected is a person, the operation doesn't provide the same facial details that the <a>DetectFaces</a> operation provides.</p> </note> <p> <code>DetectLabels</code> returns bounding boxes for instances of common object labels in an array of <a>Instance</a> objects. An <code>Instance</code> object contains a <a>BoundingBox</a> object, for the location of the label on the image. It also includes the confidence by which the bounding box was detected.</p> <p> <code>DetectLabels</code> also returns a hierarchical taxonomy of detected labels. For example, a detected car might be assigned the label <i>car</i>. The label <i>car</i> has two parent labels: <i>Vehicle</i> (its parent) and <i>Transportation</i> (its grandparent). The response returns the entire list of ancestors for a label. Each ancestor is a unique label in the response. In the previous example, <i>Car</i>, <i>Vehicle</i>, and <i>Transportation</i> are returned as unique labels in the response. </p> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectLabels</code> action. </p>
    async fn detect_labels(
        &self,
        input: DetectLabelsRequest,
    ) -> Result<DetectLabelsResponse, RusotoError<DetectLabelsError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectLabels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DetectLabelsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectLabelsError::from_response(response))
        }
    }

    /// <p>Detects unsafe content in a specified JPEG or PNG format image. Use <code>DetectModerationLabels</code> to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not images containing suggestive content.</p> <p>To filter images, use the labels returned by <code>DetectModerationLabels</code> to determine which types of content are appropriate.</p> <p>For information about moderation labels, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p>
    async fn detect_moderation_labels(
        &self,
        input: DetectModerationLabelsRequest,
    ) -> Result<DetectModerationLabelsResponse, RusotoError<DetectModerationLabelsError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectModerationLabels");
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
                .deserialize::<DetectModerationLabelsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectModerationLabelsError::from_response(response))
        }
    }

    /// <p>Detects text in the input image and converts it into machine-readable text.</p> <p>Pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, you must pass it as a reference to an image in an Amazon S3 bucket. For the AWS CLI, passing image bytes is not supported. The image must be either a .png or .jpeg formatted file. </p> <p>The <code>DetectText</code> operation returns text in an array of <a>TextDetection</a> elements, <code>TextDetections</code>. Each <code>TextDetection</code> element provides information about a single word or line of text that was detected in the image. </p> <p>A word is one or more ISO basic latin script characters that are not separated by spaces. <code>DetectText</code> can detect up to 50 words in an image.</p> <p>A line is a string of equally spaced words. A line isn't necessarily a complete sentence. For example, a driver's license number is detected as a line. A line ends when there is no aligned text after it. Also, a line ends when there is a large gap between words, relative to the length of the words. This means, depending on the gap between words, Amazon Rekognition may detect multiple lines in text aligned in the same direction. Periods don't represent the end of a line. If a sentence spans multiple lines, the <code>DetectText</code> operation returns multiple lines.</p> <p>To determine whether a <code>TextDetection</code> element is a line of text or a word, use the <code>TextDetection</code> object <code>Type</code> field. </p> <p>To be detected, text must be within +/- 90 degrees orientation of the horizontal axis.</p> <p>For more information, see DetectText in the Amazon Rekognition Developer Guide.</p>
    async fn detect_text(
        &self,
        input: DetectTextRequest,
    ) -> Result<DetectTextResponse, RusotoError<DetectTextError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectText");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DetectTextResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DetectTextError::from_response(response))
        }
    }

    /// <p>Gets the name and additional information about a celebrity based on his or her Amazon Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, this list is empty.</p> <p>For more information, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:GetCelebrityInfo</code> action. </p>
    async fn get_celebrity_info(
        &self,
        input: GetCelebrityInfoRequest,
    ) -> Result<GetCelebrityInfoResponse, RusotoError<GetCelebrityInfoError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetCelebrityInfo");
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
                .deserialize::<GetCelebrityInfoResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCelebrityInfoError::from_response(response))
        }
    }

    /// <p>Gets the celebrity recognition results for a Amazon Rekognition Video analysis started by <a>StartCelebrityRecognition</a>.</p> <p>Celebrity recognition in a video is an asynchronous operation. Analysis is started by a call to <a>StartCelebrityRecognition</a> which returns a job identifier (<code>JobId</code>). When the celebrity recognition operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartCelebrityRecognition</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetCelebrityDetection</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityDetection</code>. </p> <p>For more information, see Working With Stored Videos in the Amazon Rekognition Developer Guide.</p> <p> <code>GetCelebrityRecognition</code> returns detected celebrities and the time(s) they are detected in an array (<code>Celebrities</code>) of <a>CelebrityRecognition</a> objects. Each <code>CelebrityRecognition</code> contains information about the celebrity in a <a>CelebrityDetail</a> object and the time, <code>Timestamp</code>, the celebrity was detected. </p> <note> <p> <code>GetCelebrityRecognition</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Celebrities</code> array is sorted by time (milliseconds from the start of the video). You can also sort the array by celebrity by specifying the value <code>ID</code> in the <code>SortBy</code> input parameter.</p> <p>The <code>CelebrityDetail</code> object includes the celebrity identifer and additional information urls. If you don't store the additional information urls, you can get them later by calling <a>GetCelebrityInfo</a> with the celebrity identifer.</p> <p>No information is returned for faces not recognized as celebrities.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetCelebrityDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetCelebrityRecognition</code>.</p>
    async fn get_celebrity_recognition(
        &self,
        input: GetCelebrityRecognitionRequest,
    ) -> Result<GetCelebrityRecognitionResponse, RusotoError<GetCelebrityRecognitionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetCelebrityRecognition");
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
                .deserialize::<GetCelebrityRecognitionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetCelebrityRecognitionError::from_response(response))
        }
    }

    /// <p>Gets the unsafe content analysis results for a Amazon Rekognition Video analysis started by <a>StartContentModeration</a>.</p> <p>Unsafe content analysis of a video is an asynchronous operation. You start analysis by calling <a>StartContentModeration</a> which returns a job identifier (<code>JobId</code>). When analysis finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartContentModeration</code>. To get the results of the unsafe content analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetContentModeration</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Working with Stored Videos in the Amazon Rekognition Devlopers Guide.</p> <p> <code>GetContentModeration</code> returns detected unsafe content labels, and the time they are detected, in an array, <code>ModerationLabels</code>, of <a>ContentModerationDetection</a> objects. </p> <p>By default, the moderated labels are returned sorted by time, in milliseconds from the start of the video. You can also sort them by moderated label by specifying <code>NAME</code> for the <code>SortBy</code> input parameter. </p> <p>Since video analysis can return a large number of results, use the <code>MaxResults</code> parameter to limit the number of labels returned in a single call to <code>GetContentModeration</code>. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetContentModeration</code> and populate the <code>NextToken</code> request parameter with the value of <code>NextToken</code> returned from the previous call to <code>GetContentModeration</code>.</p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    async fn get_content_moderation(
        &self,
        input: GetContentModerationRequest,
    ) -> Result<GetContentModerationResponse, RusotoError<GetContentModerationError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetContentModeration");
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
                .deserialize::<GetContentModerationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetContentModerationError::from_response(response))
        }
    }

    /// <p>Gets face detection results for a Amazon Rekognition Video analysis started by <a>StartFaceDetection</a>.</p> <p>Face detection with Amazon Rekognition Video is an asynchronous operation. You start face detection by calling <a>StartFaceDetection</a> which returns a job identifier (<code>JobId</code>). When the face detection operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceDetection</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p> <code>GetFaceDetection</code> returns an array of detected faces (<code>Faces</code>) sorted by the time the faces were detected. </p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetFaceDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetFaceDetection</code>.</p>
    async fn get_face_detection(
        &self,
        input: GetFaceDetectionRequest,
    ) -> Result<GetFaceDetectionResponse, RusotoError<GetFaceDetectionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetFaceDetection");
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
                .deserialize::<GetFaceDetectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetFaceDetectionError::from_response(response))
        }
    }

    /// <p>Gets the face search results for Amazon Rekognition Video face search started by <a>StartFaceSearch</a>. The search returns faces in a collection that match the faces of persons detected in a video. It also includes the time(s) that faces are matched in the video.</p> <p>Face search in a video is an asynchronous operation. You start face search by calling to <a>StartFaceSearch</a> which returns a job identifier (<code>JobId</code>). When the search operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceSearch</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetFaceSearch</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>.</p> <p>For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>The search results are retured in an array, <code>Persons</code>, of <a>PersonMatch</a> objects. Each<code>PersonMatch</code> element contains details about the matching faces in the input collection, person information (facial attributes, bounding boxes, and person identifer) for the matched person, and the time the person was matched in the video.</p> <note> <p> <code>GetFaceSearch</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Persons</code> array is sorted by the time, in milliseconds from the start of the video, persons are matched. You can also sort by persons by specifying <code>INDEX</code> for the <code>SORTBY</code> input parameter.</p>
    async fn get_face_search(
        &self,
        input: GetFaceSearchRequest,
    ) -> Result<GetFaceSearchResponse, RusotoError<GetFaceSearchError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetFaceSearch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<GetFaceSearchResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetFaceSearchError::from_response(response))
        }
    }

    /// <p>Gets the label detection results of a Amazon Rekognition Video analysis started by <a>StartLabelDetection</a>. </p> <p>The label detection operation is started by a call to <a>StartLabelDetection</a> which returns a job identifier (<code>JobId</code>). When the label detection operation finishes, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartlabelDetection</code>. To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetLabelDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p> <code>GetLabelDetection</code> returns an array of detected labels (<code>Labels</code>) sorted by the time the labels were detected. You can also sort by the label name by specifying <code>NAME</code> for the <code>SortBy</code> input parameter.</p> <p>The labels returned include the label name, the percentage confidence in the accuracy of the detected label, and the time the label was detected in the video.</p> <p>The returned labels also include bounding box information for common objects, a hierarchical taxonomy of detected labels, and the version of the label model used for detection.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetlabelDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetLabelDetection</code>.</p>
    async fn get_label_detection(
        &self,
        input: GetLabelDetectionRequest,
    ) -> Result<GetLabelDetectionResponse, RusotoError<GetLabelDetectionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetLabelDetection");
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
                .deserialize::<GetLabelDetectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetLabelDetectionError::from_response(response))
        }
    }

    /// <p>Gets the path tracking results of a Amazon Rekognition Video analysis started by <a>StartPersonTracking</a>.</p> <p>The person path tracking operation is started by a call to <code>StartPersonTracking</code> which returns a job identifier (<code>JobId</code>). When the operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartPersonTracking</code>.</p> <p>To get the results of the person path tracking operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetPersonTracking</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p> <p> <code>GetPersonTracking</code> returns an array, <code>Persons</code>, of tracked persons and the time(s) their paths were tracked in the video. </p> <note> <p> <code>GetPersonTracking</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. </p> <p>For more information, see FaceDetail in the Amazon Rekognition Developer Guide.</p> </note> <p>By default, the array is sorted by the time(s) a person's path is tracked in the video. You can sort by tracked persons by specifying <code>INDEX</code> for the <code>SortBy</code> input parameter.</p> <p>Use the <code>MaxResults</code> parameter to limit the number of items returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetPersonTracking</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetPersonTracking</code>.</p>
    async fn get_person_tracking(
        &self,
        input: GetPersonTrackingRequest,
    ) -> Result<GetPersonTrackingResponse, RusotoError<GetPersonTrackingError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetPersonTracking");
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
                .deserialize::<GetPersonTrackingResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetPersonTrackingError::from_response(response))
        }
    }

    /// <p>Detects faces in the input image and adds them to the specified collection. </p> <p>Amazon Rekognition doesn't save the actual faces that are detected. Instead, the underlying detection algorithm first detects the faces in the input image. For each face, the algorithm extracts facial features into a feature vector, and stores it in the backend database. Amazon Rekognition uses feature vectors when it performs face match and search operations using the <a>SearchFaces</a> and <a>SearchFacesByImage</a> operations.</p> <p>For more information, see Adding Faces to a Collection in the Amazon Rekognition Developer Guide.</p> <p>To get the number of faces in a collection, call <a>DescribeCollection</a>. </p> <p>If you're using version 1.0 of the face detection model, <code>IndexFaces</code> indexes the 15 largest faces in the input image. Later versions of the face detection model index the 100 largest faces in the input image. </p> <p>If you're using version 4 or later of the face model, image orientation information is not returned in the <code>OrientationCorrection</code> field. </p> <p>To determine which version of the model you're using, call <a>DescribeCollection</a> and supply the collection ID. You can also get the model version from the value of <code>FaceModelVersion</code> in the response from <code>IndexFaces</code> </p> <p>For more information, see Model Versioning in the Amazon Rekognition Developer Guide.</p> <p>If you provide the optional <code>ExternalImageID</code> for the input image you provided, Amazon Rekognition associates this ID with all faces that it detects. When you call the <a>ListFaces</a> operation, the response returns the external ID. You can use this external image ID to create a client-side index to associate the faces with each image. You can then use the index to find all faces in an image.</p> <p>You can specify the maximum number of faces to index with the <code>MaxFaces</code> input parameter. This is useful when you want to index the largest faces in an image and don't want to index smaller faces, such as those belonging to people standing in the background.</p> <p>The <code>QualityFilter</code> input parameter allows you to filter out detected faces that don’t meet a required quality bar. The quality bar is based on a variety of common use cases. By default, <code>IndexFaces</code> chooses the quality bar that's used to filter faces. You can also explicitly choose the quality bar. Use <code>QualityFilter</code>, to set the quality bar by specifying <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>. If you do not want to filter detected faces, specify <code>NONE</code>. </p> <note> <p>To use quality filtering, you need a collection associated with version 3 of the face model or higher. To get the version of the face model associated with a collection, call <a>DescribeCollection</a>. </p> </note> <p>Information about faces detected in an image, but not indexed, is returned in an array of <a>UnindexedFace</a> objects, <code>UnindexedFaces</code>. Faces aren't indexed for reasons such as:</p> <ul> <li> <p>The number of faces detected exceeds the value of the <code>MaxFaces</code> request parameter.</p> </li> <li> <p>The face is too small compared to the image dimensions.</p> </li> <li> <p>The face is too blurry.</p> </li> <li> <p>The image is too dark.</p> </li> <li> <p>The face has an extreme pose.</p> </li> <li> <p>The face doesn’t have enough detail to be suitable for face search.</p> </li> </ul> <p>In response, the <code>IndexFaces</code> operation returns an array of metadata for all detected faces, <code>FaceRecords</code>. This includes: </p> <ul> <li> <p>The bounding box, <code>BoundingBox</code>, of the detected face. </p> </li> <li> <p>A confidence value, <code>Confidence</code>, which indicates the confidence that the bounding box contains a face.</p> </li> <li> <p>A face ID, <code>FaceId</code>, assigned by the service for each face that's detected and stored.</p> </li> <li> <p>An image ID, <code>ImageId</code>, assigned by the service for the input image.</p> </li> </ul> <p>If you request all facial attributes (by using the <code>detectionAttributes</code> parameter), Amazon Rekognition returns detailed facial attributes, such as facial landmarks (for example, location of eye and mouth) and other facial attributes. If you provide the same image, specify the same collection, and use the same external ID in the <code>IndexFaces</code> operation, Amazon Rekognition doesn't save duplicate face metadata.</p> <p/> <p>The input image is passed either as base64-encoded image bytes, or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes isn't supported. The image must be formatted as a PNG or JPEG file. </p> <p>This operation requires permissions to perform the <code>rekognition:IndexFaces</code> action.</p>
    async fn index_faces(
        &self,
        input: IndexFacesRequest,
    ) -> Result<IndexFacesResponse, RusotoError<IndexFacesError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.IndexFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<IndexFacesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(IndexFacesError::from_response(response))
        }
    }

    /// <p>Returns list of collection IDs in your account. If the result is truncated, the response also provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p> <p>For an example, see Listing Collections in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListCollections</code> action.</p>
    async fn list_collections(
        &self,
        input: ListCollectionsRequest,
    ) -> Result<ListCollectionsResponse, RusotoError<ListCollectionsError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListCollections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListCollectionsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListCollectionsError::from_response(response))
        }
    }

    /// <p>Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, see Listing Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListFaces</code> action.</p>
    async fn list_faces(
        &self,
        input: ListFacesRequest,
    ) -> Result<ListFacesResponse, RusotoError<ListFacesError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<ListFacesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListFacesError::from_response(response))
        }
    }

    /// <p>Gets a list of stream processors that you have created with <a>CreateStreamProcessor</a>. </p>
    async fn list_stream_processors(
        &self,
        input: ListStreamProcessorsRequest,
    ) -> Result<ListStreamProcessorsResponse, RusotoError<ListStreamProcessorsError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListStreamProcessors");
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
                .deserialize::<ListStreamProcessorsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListStreamProcessorsError::from_response(response))
        }
    }

    /// <p>Returns an array of celebrities recognized in the input image. For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide. </p> <p> <code>RecognizeCelebrities</code> returns the 100 largest faces in the image. It lists recognized celebrities in the <code>CelebrityFaces</code> array and unrecognized faces in the <code>UnrecognizedFaces</code> array. <code>RecognizeCelebrities</code> doesn't return celebrities whose faces aren't among the largest 100 faces in the image.</p> <p>For each celebrity recognized, <code>RecognizeCelebrities</code> returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p> <p>Amazon Rekognition doesn't retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the <a>GetCelebrityInfo</a> operation.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>For an example, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>
    async fn recognize_celebrities(
        &self,
        input: RecognizeCelebritiesRequest,
    ) -> Result<RecognizeCelebritiesResponse, RusotoError<RecognizeCelebritiesError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.RecognizeCelebrities");
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
                .deserialize::<RecognizeCelebritiesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(RecognizeCelebritiesError::from_response(response))
        }
    }

    /// <p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <a>IndexFaces</a> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p> </note> <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p> <p>For an example, see Searching for a Face Using Its Face ID in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>
    async fn search_faces(
        &self,
        input: SearchFacesRequest,
    ) -> Result<SearchFacesResponse, RusotoError<SearchFacesError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.SearchFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<SearchFacesResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchFacesError::from_response(response))
        }
    }

    /// <p>For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>To search for all faces in an input image, you might first call the <a>IndexFaces</a> operation, and then use the face IDs returned in subsequent calls to the <a>SearchFaces</a> operation. </p> <p> You can also call the <code>DetectFaces</code> operation and use the bounding boxes in the response to make face crops, which then you can pass in to the <code>SearchFacesByImage</code> operation. </p> </note> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> The response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match found. Along with the metadata, the response also includes a <code>similarity</code> indicating how similar the face is to the input face. In the response, the operation also returns the bounding box (and a confidence level that the bounding box contains a face) of the face that Amazon Rekognition used for the input image. </p> <p>For an example, Searching for a Face Using an Image in the Amazon Rekognition Developer Guide.</p> <p>The <code>QualityFilter</code> input parameter allows you to filter out detected faces that don’t meet a required quality bar. The quality bar is based on a variety of common use cases. Use <code>QualityFilter</code> to set the quality bar for filtering by specifying <code>LOW</code>, <code>MEDIUM</code>, or <code>HIGH</code>. If you do not want to filter detected faces, specify <code>NONE</code>. The default value is <code>NONE</code>.</p> <note> <p>To use quality filtering, you need a collection associated with version 3 of the face model or higher. To get the version of the face model associated with a collection, call <a>DescribeCollection</a>. </p> </note> <p>This operation requires permissions to perform the <code>rekognition:SearchFacesByImage</code> action.</p>
    async fn search_faces_by_image(
        &self,
        input: SearchFacesByImageRequest,
    ) -> Result<SearchFacesByImageResponse, RusotoError<SearchFacesByImageError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.SearchFacesByImage");
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
                .deserialize::<SearchFacesByImageResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(SearchFacesByImageError::from_response(response))
        }
    }

    /// <p>Starts asynchronous recognition of celebrities in a stored video.</p> <p>Amazon Rekognition Video can detect celebrities in a video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartCelebrityRecognition</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When celebrity recognition analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetCelebrityRecognition</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityRecognition</code>. </p> <p>For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide.</p>
    async fn start_celebrity_recognition(
        &self,
        input: StartCelebrityRecognitionRequest,
    ) -> Result<StartCelebrityRecognitionResponse, RusotoError<StartCelebrityRecognitionError>>
    {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "RekognitionService.StartCelebrityRecognition",
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
                .deserialize::<StartCelebrityRecognitionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartCelebrityRecognitionError::from_response(response))
        }
    }

    /// <p> Starts asynchronous detection of unsafe content in a stored video.</p> <p>Amazon Rekognition Video can moderate content in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartContentModeration</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When unsafe content analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the unsafe content analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetContentModeration</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    async fn start_content_moderation(
        &self,
        input: StartContentModerationRequest,
    ) -> Result<StartContentModerationResponse, RusotoError<StartContentModerationError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartContentModeration");
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
                .deserialize::<StartContentModerationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartContentModerationError::from_response(response))
        }
    }

    /// <p>Starts asynchronous detection of faces in a stored video.</p> <p>Amazon Rekognition Video can detect faces in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceDetection</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When face detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p>For more information, see Detecting Faces in a Stored Video in the Amazon Rekognition Developer Guide.</p>
    async fn start_face_detection(
        &self,
        input: StartFaceDetectionRequest,
    ) -> Result<StartFaceDetectionResponse, RusotoError<StartFaceDetectionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartFaceDetection");
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
                .deserialize::<StartFaceDetectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartFaceDetectionError::from_response(response))
        }
    }

    /// <p>Starts the asynchronous search for faces in a collection that match the faces of persons detected in a stored video.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceSearch</code> returns a job identifier (<code>JobId</code>) which you use to get the search results once the search has completed. When searching is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceSearch</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>. For more information, see <a>procedure-person-search-videos</a>.</p>
    async fn start_face_search(
        &self,
        input: StartFaceSearchRequest,
    ) -> Result<StartFaceSearchResponse, RusotoError<StartFaceSearchError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartFaceSearch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartFaceSearchResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartFaceSearchError::from_response(response))
        }
    }

    /// <p><p>Starts asynchronous detection of labels in a stored video.</p> <p>Amazon Rekognition Video can detect labels in a video. Labels are instances of real-world entities. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; concepts like landscape, evening, and nature; and activities like a person getting out of a car or a person skiing.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartLabelDetection</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetLabelDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p/></p>
    async fn start_label_detection(
        &self,
        input: StartLabelDetectionRequest,
    ) -> Result<StartLabelDetectionResponse, RusotoError<StartLabelDetectionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartLabelDetection");
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
                .deserialize::<StartLabelDetectionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartLabelDetectionError::from_response(response))
        }
    }

    /// <p>Starts the asynchronous tracking of a person's path in a stored video.</p> <p>Amazon Rekognition Video can track the path of people in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartPersonTracking</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. </p> <p>To get the results of the person detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetPersonTracking</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p>
    async fn start_person_tracking(
        &self,
        input: StartPersonTrackingRequest,
    ) -> Result<StartPersonTrackingResponse, RusotoError<StartPersonTrackingError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartPersonTracking");
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
                .deserialize::<StartPersonTrackingResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartPersonTrackingError::from_response(response))
        }
    }

    /// <p>Starts the running of the version of a model. Starting a model takes a while to complete. To check the current state of the model, use <a>DescribeProjectVersions</a>.</p> <p>Once the model is running, you can detect custom labels in new images by calling <a>DetectCustomLabels</a>.</p> <note> <p>You are charged for the amount of time that the model is running. To stop a running model, call <a>StopProjectVersion</a>.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:StartProjectVersion</code> action.</p>
    async fn start_project_version(
        &self,
        input: StartProjectVersionRequest,
    ) -> Result<StartProjectVersionResponse, RusotoError<StartProjectVersionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartProjectVersion");
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
                .deserialize::<StartProjectVersionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartProjectVersionError::from_response(response))
        }
    }

    /// <p>Starts processing a stream processor. You create a stream processor by calling <a>CreateStreamProcessor</a>. To tell <code>StartStreamProcessor</code> which stream processor to start, use the value of the <code>Name</code> field specified in the call to <code>CreateStreamProcessor</code>.</p>
    async fn start_stream_processor(
        &self,
        input: StartStreamProcessorRequest,
    ) -> Result<StartStreamProcessorResponse, RusotoError<StartStreamProcessorError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartStreamProcessor");
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
                .deserialize::<StartStreamProcessorResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartStreamProcessorError::from_response(response))
        }
    }

    /// <p>Stops a running model. The operation might take a while to complete. To check the current status, call <a>DescribeProjectVersions</a>. </p>
    async fn stop_project_version(
        &self,
        input: StopProjectVersionRequest,
    ) -> Result<StopProjectVersionResponse, RusotoError<StopProjectVersionError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StopProjectVersion");
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
                .deserialize::<StopProjectVersionResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopProjectVersionError::from_response(response))
        }
    }

    /// <p>Stops a running stream processor that was created by <a>CreateStreamProcessor</a>.</p>
    async fn stop_stream_processor(
        &self,
        input: StopStreamProcessorRequest,
    ) -> Result<StopStreamProcessorResponse, RusotoError<StopStreamProcessorError>> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StopStreamProcessor");
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
                .deserialize::<StopStreamProcessorResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopStreamProcessorError::from_response(response))
        }
    }
}
