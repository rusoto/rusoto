
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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Structure containing the estimated age range, in years, for a face.</p> <p>Rekognition estimates an age-range for faces detected in the input image. Estimated age ranges can overlap; a face of a 5 year old may have an estimated range of 4-6 whilst the face of a 6 year old may have an estimated range of 4-8.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AgeRange {
    #[doc="<p>The highest estimated age.</p>"]
    #[serde(rename="High")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub high: Option<UInteger>,
    #[doc="<p>The lowest estimated age.</p>"]
    #[serde(rename="Low")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub low: Option<UInteger>,
}

pub type Attribute = String;
pub type Attributes = Vec<Attribute>;
#[doc="<p>Indicates whether or not the face has a beard, and the confidence level in the determination.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Beard {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Boolean value that indicates whether the face has beard or not.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Boolean>,
}

pub type Boolean = bool;
#[doc="<p>Identifies the bounding box around the object or face. The <code>left</code> (x-coordinate) and <code>top</code> (y-coordinate) are coordinates representing the top and left sides of the bounding box. Note that the upper-left corner of the image is the origin (0,0). </p> <p>The <code>top</code> and <code>left</code> values returned are ratios of the overall image size. For example, if the input image is 700x200 pixels, and the top-left coordinate of the bounding box is 350x50 pixels, the API returns a <code>left</code> value of 0.5 (350/700) and a <code>top</code> value of 0.25 (50/200).</p> <p> The <code>width</code> and <code>height</code> values represent the dimensions of the bounding box as a ratio of the overall image dimension. For example, if the input image is 700x200 pixels, and the bounding box width is 70 pixels, the width returned is 0.1. </p> <note> <p> The bounding box coordinates can have negative values. For example, if Amazon Rekognition is able to detect a face that is at the image edge and is only partially visible, the service can return coordinates that are outside the image bounds and, depending on the image edge, you might get negative values or values greater than 1 for the <code>left</code> or <code>top</code> values. </p> </note>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BoundingBox {
    #[doc="<p>Height of the bounding box as a ratio of the overall image height.</p>"]
    #[serde(rename="Height")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub height: Option<Float>,
    #[doc="<p>Left coordinate of the bounding box as a ratio of overall image width.</p>"]
    #[serde(rename="Left")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub left: Option<Float>,
    #[doc="<p>Top coordinate of the bounding box as a ratio of overall image height.</p>"]
    #[serde(rename="Top")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub top: Option<Float>,
    #[doc="<p>Width of the bounding box as a ratio of the overall image width.</p>"]
    #[serde(rename="Width")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub width: Option<Float>,
}

#[doc="<p>Provides information about a celebrity recognized by the operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Celebrity {
    #[doc="<p>Provides information about the celebrity's face, such as its location on the image.</p>"]
    #[serde(rename="Face")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face: Option<ComparedFace>,
    #[doc="<p>A unique identifier for the celebrity. </p>"]
    #[serde(rename="Id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<RekognitionUniqueId>,
    #[doc="<p>The confidence, in percentage, that Rekognition has that the recognized face is the celebrity.</p>"]
    #[serde(rename="MatchConfidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub match_confidence: Option<Percent>,
    #[doc="<p>The name of the celebrity.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>An array of URLs pointing to additional information about the celebrity. If there is no additional information about the celebrity, this list is empty.</p>"]
    #[serde(rename="Urls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub urls: Option<Urls>,
}

pub type CelebrityList = Vec<Celebrity>;
pub type CollectionId = String;
pub type CollectionIdList = Vec<CollectionId>;
#[doc="<p>Provides information about a face in a target image that matches the source image face analysed by <code>CompareFaces</code>. The <code>Face</code> property contains the bounding box of the face in the target image. The <code>Similarity</code> property is the confidence that the source image face matches the face in the bounding box.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CompareFacesMatch {
    #[doc="<p>Provides face metadata (bounding box and confidence that the bounding box actually contains a face).</p>"]
    #[serde(rename="Face")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face: Option<ComparedFace>,
    #[doc="<p>Level of confidence that the faces match.</p>"]
    #[serde(rename="Similarity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub similarity: Option<Percent>,
}

pub type CompareFacesMatchList = Vec<CompareFacesMatch>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct CompareFacesRequest {
    #[doc="<p>The minimum level of confidence in the face matches that a match must meet to be included in the <code>FaceMatches</code> array.</p>"]
    #[serde(rename="SimilarityThreshold")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub similarity_threshold: Option<Percent>,
    #[doc="<p>The source image, either as bytes or as an S3 object.</p>"]
    #[serde(rename="SourceImage")]
    pub source_image: Image,
    #[doc="<p>The target image, either as bytes or as an S3 object.</p>"]
    #[serde(rename="TargetImage")]
    pub target_image: Image,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CompareFacesResponse {
    #[doc="<p>An array of faces in the target image that match the source image face. Each <code>CompareFacesMatch</code> object provides the bounding box, the confidence level that the bounding box contains a face, and the similarity score for the face in the bounding box and the face in the source image.</p>"]
    #[serde(rename="FaceMatches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_matches: Option<CompareFacesMatchList>,
    #[doc="<p>The face in the source image that was used for comparison.</p>"]
    #[serde(rename="SourceImageFace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_image_face: Option<ComparedSourceImageFace>,
    #[doc="<p> The orientation of the source image (counterclockwise direction). If your application displays the source image, you can use this value to correct image orientation. The bounding box coordinates returned in <code>SourceImageFace</code> represent the location of the face before the image orientation is corrected. </p> <note> <p>If the source image is in .jpeg format, it might contain exchangeable image (Exif) metadata that includes the image's orientation. If the Exif metadata for the source image populates the orientation field, the value of <code>OrientationCorrection</code> is null and the <code>SourceImageFace</code> bounding box coordinates represent the location of the face after Exif metadata is used to correct the orientation. Images in .png format don't contain Exif metadata.</p> </note>"]
    #[serde(rename="SourceImageOrientationCorrection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_image_orientation_correction: Option<OrientationCorrection>,
    #[doc="<p> The orientation of the target image (in counterclockwise direction). If your application displays the target image, you can use this value to correct the orientation of the image. The bounding box coordinates returned in <code>FaceMatches</code> and <code>UnmatchedFaces</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the target image is in .jpg format, it might contain Exif metadata that includes the orientation of the image. If the Exif metadata for the target image populates the orientation field, the value of <code>OrientationCorrection</code> is null and the bounding box coordinates in <code>FaceMatches</code> and <code>UnmatchedFaces</code> represent the location of the face after Exif metadata is used to correct the orientation. Images in .png format don't contain Exif metadata.</p> </note>"]
    #[serde(rename="TargetImageOrientationCorrection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_image_orientation_correction: Option<OrientationCorrection>,
    #[doc="<p>An array of faces in the target image that did not match the source image face.</p>"]
    #[serde(rename="UnmatchedFaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unmatched_faces: Option<CompareFacesUnmatchList>,
}

pub type CompareFacesUnmatchList = Vec<ComparedFace>;
#[doc="<p>Provides face metadata for target image faces that are analysed by <code>CompareFaces</code> and <code>RecognizeCelebrities</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComparedFace {
    #[doc="<p>Bounding box of the face.</p>"]
    #[serde(rename="BoundingBox")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[doc="<p>Level of confidence that what the bounding box contains is a face.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>An array of facial landmarks.</p>"]
    #[serde(rename="Landmarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub landmarks: Option<Landmarks>,
    #[doc="<p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>"]
    #[serde(rename="Pose")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pose: Option<Pose>,
    #[doc="<p>Identifies face image brightness and sharpness. </p>"]
    #[serde(rename="Quality")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub quality: Option<ImageQuality>,
}

pub type ComparedFaceList = Vec<ComparedFace>;
#[doc="<p>Type that describes the face Amazon Rekognition chose to compare with the faces in the target. This contains a bounding box for the selected face and confidence level that the bounding box contains a face. Note that Amazon Rekognition selects the largest face in the source image for this comparison. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ComparedSourceImageFace {
    #[doc="<p>Bounding box of the face.</p>"]
    #[serde(rename="BoundingBox")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[doc="<p>Confidence level that the selected bounding box contains a face.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateCollectionRequest {
    #[doc="<p>ID for the collection that you are creating.</p>"]
    #[serde(rename="CollectionId")]
    pub collection_id: CollectionId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateCollectionResponse {
    #[doc="<p>Amazon Resource Name (ARN) of the collection. You can use this to manage permissions on your resources. </p>"]
    #[serde(rename="CollectionArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collection_arn: Option<String>,
    #[doc="<p>HTTP status code indicating the result of the operation.</p>"]
    #[serde(rename="StatusCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_code: Option<UInteger>,
}

pub type Degree = f32;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteCollectionRequest {
    #[doc="<p>ID of the collection to delete.</p>"]
    #[serde(rename="CollectionId")]
    pub collection_id: CollectionId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteCollectionResponse {
    #[doc="<p>HTTP status code that indicates the result of the operation.</p>"]
    #[serde(rename="StatusCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_code: Option<UInteger>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteFacesRequest {
    #[doc="<p>Collection from which to remove the specific faces.</p>"]
    #[serde(rename="CollectionId")]
    pub collection_id: CollectionId,
    #[doc="<p>An array of face IDs to delete.</p>"]
    #[serde(rename="FaceIds")]
    pub face_ids: FaceIdList,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteFacesResponse {
    #[doc="<p>An array of strings (face IDs) of the faces that were deleted.</p>"]
    #[serde(rename="DeletedFaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_faces: Option<FaceIdList>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DetectFacesRequest {
    #[doc="<p>An array of facial attributes you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>[\"DEFAULT\"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code> and <code>Landmarks</code>. If you provide <code>[\"ALL\"]</code>, all facial attributes are returned but the operation will take longer to complete.</p> <p>If you provide both, <code>[\"ALL\", \"DEFAULT\"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>"]
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<Attributes>,
    #[doc="<p>The image in which you want to detect faces. You can specify a blob or an S3 object. </p>"]
    #[serde(rename="Image")]
    pub image: Image,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DetectFacesResponse {
    #[doc="<p>Details of each face found in the image. </p>"]
    #[serde(rename="FaceDetails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_details: Option<FaceDetailList>,
    #[doc="<p> The orientation of the input image (counter-clockwise direction). If your application displays the image, you can use this value to correct image orientation. The bounding box coordinates returned in <code>FaceDetails</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the input image is in .jpeg format, it might contain exchangeable image (Exif) metadata that includes the image's orientation. If so, and the Exif metadata for the input image populates the orientation field, the value of <code>OrientationCorrection</code> is null and the <code>FaceDetails</code> bounding box coordinates represent face locations after Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> </note>"]
    #[serde(rename="OrientationCorrection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub orientation_correction: Option<OrientationCorrection>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DetectLabelsRequest {
    #[doc="<p>The input image. You can provide a blob of image bytes or an S3 object.</p>"]
    #[serde(rename="Image")]
    pub image: Image,
    #[doc="<p>Maximum number of labels you want the service to return in the response. The service returns the specified number of highest confidence labels. </p>"]
    #[serde(rename="MaxLabels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_labels: Option<UInteger>,
    #[doc="<p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with confidence lower than this specified value.</p> <p>If <code>MinConfidence</code> is not specified, the operation returns labels with a confidence values greater than or equal to 50 percent.</p>"]
    #[serde(rename="MinConfidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_confidence: Option<Percent>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DetectLabelsResponse {
    #[doc="<p>An array of labels for the real-world objects detected. </p>"]
    #[serde(rename="Labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Labels>,
    #[doc="<p> The orientation of the input image (counter-clockwise direction). If your application displays the image, you can use this value to correct the orientation. If Amazon Rekognition detects that the input image was rotated (for example, by 90 degrees), it first corrects the orientation before detecting the labels. </p> <note> <p>If the input image Exif metadata populates the orientation field, Amazon Rekognition does not perform orientation correction and the value of OrientationCorrection will be null.</p> </note>"]
    #[serde(rename="OrientationCorrection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub orientation_correction: Option<OrientationCorrection>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DetectModerationLabelsRequest {
    #[doc="<p>The input image as bytes or an S3 object.</p>"]
    #[serde(rename="Image")]
    pub image: Image,
    #[doc="<p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence level lower than this specified value.</p> <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>"]
    #[serde(rename="MinConfidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_confidence: Option<Percent>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DetectModerationLabelsResponse {
    #[doc="<p>An array of labels for explicit or suggestive adult content found in the image. The list includes the top-level label and each child label detected in the image. This is useful for filtering specific categories of content. </p>"]
    #[serde(rename="ModerationLabels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub moderation_labels: Option<ModerationLabels>,
}

#[doc="<p>The emotions detected on the face, and the confidence level in the determination. For example, HAPPY, SAD, and ANGRY.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Emotion {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Type of emotion detected.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<EmotionName>,
}

pub type EmotionName = String;
pub type Emotions = Vec<Emotion>;
pub type ExternalImageId = String;
#[doc="<p>Indicates whether or not the eyes on the face are open, and the confidence level in the determination.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EyeOpen {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Boolean value that indicates whether the eyes on the face are open.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Boolean>,
}

#[doc="<p>Indicates whether or not the face is wearing eye glasses, and the confidence level in the determination.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Eyeglasses {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Boolean value that indicates whether the face is wearing eye glasses or not.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Boolean>,
}

#[doc="<p>Describes the face properties such as the bounding box, face ID, image ID of the input image, and external image ID that you assigned. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Face {
    #[doc="<p>Bounding box of the face.</p>"]
    #[serde(rename="BoundingBox")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[doc="<p>Confidence level that the bounding box contains a face (and not a different object such as a tree).</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Identifier that you assign to all the faces in the input image.</p>"]
    #[serde(rename="ExternalImageId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_image_id: Option<ExternalImageId>,
    #[doc="<p>Unique identifier that Amazon Rekognition assigns to the face.</p>"]
    #[serde(rename="FaceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_id: Option<FaceId>,
    #[doc="<p>Unique identifier that Amazon Rekognition assigns to the input image.</p>"]
    #[serde(rename="ImageId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_id: Option<ImageId>,
}

#[doc="<p>Structure containing attributes of the face that the algorithm detected.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FaceDetail {
    #[doc="<p>The estimated age range, in years, for the face. Low represents the lowest estimated age and High represents the highest estimated age.</p>"]
    #[serde(rename="AgeRange")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub age_range: Option<AgeRange>,
    #[doc="<p>Indicates whether or not the face has a beard, and the confidence level in the determination.</p>"]
    #[serde(rename="Beard")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub beard: Option<Beard>,
    #[doc="<p>Bounding box of the face.</p>"]
    #[serde(rename="BoundingBox")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[doc="<p>Confidence level that the bounding box contains a face (and not a different object such as a tree).</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>The emotions detected on the face, and the confidence level in the determination. For example, HAPPY, SAD, and ANGRY. </p>"]
    #[serde(rename="Emotions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emotions: Option<Emotions>,
    #[doc="<p>Indicates whether or not the face is wearing eye glasses, and the confidence level in the determination.</p>"]
    #[serde(rename="Eyeglasses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub eyeglasses: Option<Eyeglasses>,
    #[doc="<p>Indicates whether or not the eyes on the face are open, and the confidence level in the determination.</p>"]
    #[serde(rename="EyesOpen")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub eyes_open: Option<EyeOpen>,
    #[doc="<p>Gender of the face and the confidence level in the determination.</p>"]
    #[serde(rename="Gender")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gender: Option<Gender>,
    #[doc="<p>Indicates the location of landmarks on the face.</p>"]
    #[serde(rename="Landmarks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub landmarks: Option<Landmarks>,
    #[doc="<p>Indicates whether or not the mouth on the face is open, and the confidence level in the determination.</p>"]
    #[serde(rename="MouthOpen")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mouth_open: Option<MouthOpen>,
    #[doc="<p>Indicates whether or not the face has a mustache, and the confidence level in the determination.</p>"]
    #[serde(rename="Mustache")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mustache: Option<Mustache>,
    #[doc="<p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>"]
    #[serde(rename="Pose")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pose: Option<Pose>,
    #[doc="<p>Identifies image brightness and sharpness.</p>"]
    #[serde(rename="Quality")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub quality: Option<ImageQuality>,
    #[doc="<p>Indicates whether or not the face is smiling, and the confidence level in the determination.</p>"]
    #[serde(rename="Smile")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub smile: Option<Smile>,
    #[doc="<p>Indicates whether or not the face is wearing sunglasses, and the confidence level in the determination.</p>"]
    #[serde(rename="Sunglasses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sunglasses: Option<Sunglasses>,
}

pub type FaceDetailList = Vec<FaceDetail>;
pub type FaceId = String;
pub type FaceIdList = Vec<FaceId>;
pub type FaceList = Vec<Face>;
#[doc="<p>Provides face metadata. In addition, it also provides the confidence in the match of this face with the input face.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FaceMatch {
    #[doc="<p>Describes the face properties such as the bounding box, face ID, image ID of the source image, and external image ID that you assigned.</p>"]
    #[serde(rename="Face")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face: Option<Face>,
    #[doc="<p>Confidence in the match of this face with the input face.</p>"]
    #[serde(rename="Similarity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub similarity: Option<Percent>,
}

pub type FaceMatchList = Vec<FaceMatch>;
#[doc="<p>Object containing both the face metadata (stored in the back-end database) and facial attributes that are detected but aren't stored in the database.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct FaceRecord {
    #[doc="<p>Describes the face properties such as the bounding box, face ID, image ID of the input image, and external image ID that you assigned. </p>"]
    #[serde(rename="Face")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face: Option<Face>,
    #[doc="<p>Structure containing attributes of the face that the algorithm detected.</p>"]
    #[serde(rename="FaceDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_detail: Option<FaceDetail>,
}

pub type FaceRecordList = Vec<FaceRecord>;
pub type Float = f32;
#[doc="<p>Gender of the face and the confidence level in the determination.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Gender {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Gender of the face.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<GenderType>,
}

pub type GenderType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCelebrityInfoRequest {
    #[doc="<p>The ID for the celebrity. You get the celebrity ID from a call to the operation, which recognizes celebrities in an image. </p>"]
    #[serde(rename="Id")]
    pub id: RekognitionUniqueId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCelebrityInfoResponse {
    #[doc="<p>The name of the celebrity.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>An array of URLs pointing to additional celebrity information. </p>"]
    #[serde(rename="Urls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub urls: Option<Urls>,
}

#[doc="<p>Provides the input image either as bytes or an S3 object.</p> <p>You pass image bytes to a Rekognition API operation by using the <code>Bytes</code> property. For example, you would use the <code>Bytes</code> property to pass an image loaded from a local file system. Image bytes passed by using the <code>Bytes</code> property must be base64-encoded. Your code may not need to encode image bytes if you are using an AWS SDK to call Rekognition API operations. For more information, see <a>example4</a>.</p> <p> You pass images stored in an S3 bucket to a Rekognition API operation by using the <code>S3Object</code> property. Images stored in an S3 bucket do not need to be base64-encoded.</p> <p>The region for the S3 bucket containing the S3 object must match the region you use for Amazon Rekognition operations.</p> <p>If you use the Amazon CLI to call Amazon Rekognition operations, passing image bytes using the Bytes property is not supported. You must first upload the image to an Amazon S3 bucket and then call the operation using the S3Object property.</p> <p>For Amazon Rekognition to process an S3 object, the user must have permission to access the S3 object. For more information, see <a>manage-access-resource-policies</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct Image {
    #[doc="<p>Blob of image bytes up to 5 MBs.</p>"]
    #[serde(rename="Bytes")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub bytes: Option<ImageBlob>,
    #[doc="<p>Identifies an S3 object as the image source.</p>"]
    #[serde(rename="S3Object")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_object: Option<S3Object>,
}

pub type ImageBlob = Vec<u8>;
pub type ImageId = String;
#[doc="<p>Identifies face image brightness and sharpness. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ImageQuality {
    #[doc="<p>Value representing brightness of the face. The service returns a value between 0 and 100 (inclusive). A higher value indicates a brighter face image.</p>"]
    #[serde(rename="Brightness")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub brightness: Option<Float>,
    #[doc="<p>Value representing sharpness of the face. The service returns a value between 0 and 100 (inclusive). A higher value indicates a sharper face image.</p>"]
    #[serde(rename="Sharpness")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sharpness: Option<Float>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct IndexFacesRequest {
    #[doc="<p>The ID of an existing collection to which you want to add the faces that are detected in the input images.</p>"]
    #[serde(rename="CollectionId")]
    pub collection_id: CollectionId,
    #[doc="<p>An array of facial attributes that you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>[\"DEFAULT\"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code> and <code>Landmarks</code>. If you provide <code>[\"ALL\"]</code>, all facial attributes are returned but the operation will take longer to complete.</p> <p>If you provide both, <code>[\"ALL\", \"DEFAULT\"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>"]
    #[serde(rename="DetectionAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detection_attributes: Option<Attributes>,
    #[doc="<p>ID you want to assign to all the faces detected in the image.</p>"]
    #[serde(rename="ExternalImageId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_image_id: Option<ExternalImageId>,
    #[doc="<p>The input image as bytes or an S3 object.</p>"]
    #[serde(rename="Image")]
    pub image: Image,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct IndexFacesResponse {
    #[doc="<p>An array of faces detected and added to the collection. For more information, see <a>howitworks-index-faces</a>. </p>"]
    #[serde(rename="FaceRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_records: Option<FaceRecordList>,
    #[doc="<p>The orientation of the input image (counterclockwise direction). If your application displays the image, you can use this value to correct image orientation. The bounding box coordinates returned in <code>FaceRecords</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the input image is in jpeg format, it might contain exchangeable image (Exif) metadata. If so, and the Exif metadata populates the orientation field, the value of <code>OrientationCorrection</code> is null and the bounding box coordinates in <code>FaceRecords</code> represent face locations after Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> </note>"]
    #[serde(rename="OrientationCorrection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub orientation_correction: Option<OrientationCorrection>,
}

#[doc="<p>Structure containing details about the detected label, including name, and level of confidence.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Label {
    #[doc="<p>Level of confidence.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>The name (label) of the object.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

pub type Labels = Vec<Label>;
#[doc="<p>Indicates the location of the landmark on the face.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Landmark {
    #[doc="<p>Type of the landmark.</p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<LandmarkType>,
    #[doc="<p>x-coordinate from the top left of the landmark expressed as the ration of the width of the image. For example, if the images is 700x200 and the x-coordinate of the landmark is at 350 pixels, this value is 0.5. </p>"]
    #[serde(rename="X")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub x: Option<Float>,
    #[doc="<p>y-coordinate from the top left of the landmark expressed as the ration of the height of the image. For example, if the images is 700x200 and the y-coordinate of the landmark is at 100 pixels, this value is 0.5.</p>"]
    #[serde(rename="Y")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub y: Option<Float>,
}

pub type LandmarkType = String;
pub type Landmarks = Vec<Landmark>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListCollectionsRequest {
    #[doc="<p>Maximum number of collection IDs to return.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<PageSize>,
    #[doc="<p>Pagination token from the previous response.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<PaginationToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListCollectionsResponse {
    #[doc="<p>An array of collection IDs.</p>"]
    #[serde(rename="CollectionIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collection_ids: Option<CollectionIdList>,
    #[doc="<p>If the result is truncated, the response provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<PaginationToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListFacesRequest {
    #[doc="<p>ID of the collection from which to list the faces.</p>"]
    #[serde(rename="CollectionId")]
    pub collection_id: CollectionId,
    #[doc="<p>Maximum number of faces to return.</p>"]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<PageSize>,
    #[doc="<p>If the previous response was incomplete (because there is more data to retrieve), Amazon Rekognition returns a pagination token in the response. You can use this pagination token to retrieve the next set of faces.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<PaginationToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListFacesResponse {
    #[doc="<p>An array of <code>Face</code> objects. </p>"]
    #[serde(rename="Faces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub faces: Option<FaceList>,
    #[doc="<p>If the response is truncated, Amazon Rekognition returns this token that you can use in the subsequent request to retrieve the next set of faces.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

pub type MaxFaces = i64;
#[doc="<p>Provides information about a single type of moderated content found in an image. Each type of moderated content has a label within a hierarchical taxonomy. For more information, see <a>image-moderation</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ModerationLabel {
    #[doc="<p>Specifies the confidence that Amazon Rekognition has that the label has been correctly identified.</p> <p>If you don't specify the <code>MinConfidence</code> parameter in the call to <code>DetectModerationLabels</code>, the operation returns labels with a confidence value greater than or equal to 50 percent.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>The label name for the type of content detected in the image.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[doc="<p>The name for the parent label. Labels at the top-level of the hierarchy have the parent label <code>\"\"</code>.</p>"]
    #[serde(rename="ParentName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_name: Option<String>,
}

pub type ModerationLabels = Vec<ModerationLabel>;
#[doc="<p>Indicates whether or not the mouth on the face is open, and the confidence level in the determination.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct MouthOpen {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Boolean value that indicates whether the mouth on the face is open or not.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Boolean>,
}

#[doc="<p>Indicates whether or not the face has a mustache, and the confidence level in the determination.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Mustache {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Boolean value that indicates whether the face has mustache or not.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Boolean>,
}

pub type OrientationCorrection = String;
pub type PageSize = i64;
pub type PaginationToken = String;
pub type Percent = f32;
#[doc="<p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Pose {
    #[doc="<p>Value representing the face rotation on the pitch axis.</p>"]
    #[serde(rename="Pitch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pitch: Option<Degree>,
    #[doc="<p>Value representing the face rotation on the roll axis.</p>"]
    #[serde(rename="Roll")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub roll: Option<Degree>,
    #[doc="<p>Value representing the face rotation on the yaw axis.</p>"]
    #[serde(rename="Yaw")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub yaw: Option<Degree>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct RecognizeCelebritiesRequest {
    #[doc="<p>The input image to use for celebrity recognition.</p>"]
    #[serde(rename="Image")]
    pub image: Image,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RecognizeCelebritiesResponse {
    #[doc="<p>Details about each celebrity found in the image. Amazon Rekognition can detect a maximum of 15 celebrities in an image.</p>"]
    #[serde(rename="CelebrityFaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub celebrity_faces: Option<CelebrityList>,
    #[doc="<p>The orientation of the input image (counterclockwise direction). If your application displays the image, you can use this value to correct the orientation. The bounding box coordinates returned in <code>CelebrityFaces</code> and <code>UnrecognizedFaces</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the input image is in .jpeg format, it might contain exchangeable image (Exif) metadata that includes the image's orientation. If so, and the Exif metadata for the input image populates the orientation field, the value of <code>OrientationCorrection</code> is null and the <code>CelebrityFaces</code> and <code>UnrecognizedFaces</code> bounding box coordinates represent face locations after Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata. </p> </note>"]
    #[serde(rename="OrientationCorrection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub orientation_correction: Option<OrientationCorrection>,
    #[doc="<p>Details about each unrecognized face in the image.</p>"]
    #[serde(rename="UnrecognizedFaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unrecognized_faces: Option<ComparedFaceList>,
}

pub type RekognitionUniqueId = String;
pub type S3Bucket = String;
#[doc="<p>Provides the S3 bucket name and object name.</p> <p>The region for the S3 bucket containing the S3 object must match the region you use for Amazon Rekognition operations.</p> <p>For Amazon Rekognition to process an S3 object, the user must have permission to access the S3 object. For more information, see <a>manage-access-resource-policies</a>. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct S3Object {
    #[doc="<p>Name of the S3 bucket.</p>"]
    #[serde(rename="Bucket")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bucket: Option<S3Bucket>,
    #[doc="<p>S3 object key name.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<S3ObjectName>,
    #[doc="<p>If the bucket is versioning enabled, you can specify the object version. </p>"]
    #[serde(rename="Version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<S3ObjectVersion>,
}

pub type S3ObjectName = String;
pub type S3ObjectVersion = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct SearchFacesByImageRequest {
    #[doc="<p>ID of the collection to search.</p>"]
    #[serde(rename="CollectionId")]
    pub collection_id: CollectionId,
    #[doc="<p>(Optional) Specifies the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%.</p>"]
    #[serde(rename="FaceMatchThreshold")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_match_threshold: Option<Percent>,
    #[doc="<p>The input image as bytes or an S3 object.</p>"]
    #[serde(rename="Image")]
    pub image: Image,
    #[doc="<p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>"]
    #[serde(rename="MaxFaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_faces: Option<MaxFaces>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct SearchFacesByImageResponse {
    #[doc="<p>An array of faces that match the input face, along with the confidence in the match.</p>"]
    #[serde(rename="FaceMatches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_matches: Option<FaceMatchList>,
    #[doc="<p>The bounding box around the face in the input image that Amazon Rekognition used for the search.</p>"]
    #[serde(rename="SearchedFaceBoundingBox")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub searched_face_bounding_box: Option<BoundingBox>,
    #[doc="<p>The level of confidence that the <code>searchedFaceBoundingBox</code>, contains a face.</p>"]
    #[serde(rename="SearchedFaceConfidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub searched_face_confidence: Option<Percent>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct SearchFacesRequest {
    #[doc="<p>ID of the collection the face belongs to.</p>"]
    #[serde(rename="CollectionId")]
    pub collection_id: CollectionId,
    #[doc="<p>ID of a face to find matches for in the collection.</p>"]
    #[serde(rename="FaceId")]
    pub face_id: FaceId,
    #[doc="<p>Optional value specifying the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%.</p>"]
    #[serde(rename="FaceMatchThreshold")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_match_threshold: Option<Percent>,
    #[doc="<p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>"]
    #[serde(rename="MaxFaces")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_faces: Option<MaxFaces>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct SearchFacesResponse {
    #[doc="<p>An array of faces that matched the input face, along with the confidence in the match.</p>"]
    #[serde(rename="FaceMatches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub face_matches: Option<FaceMatchList>,
    #[doc="<p>ID of the face that was searched for matches in a collection.</p>"]
    #[serde(rename="SearchedFaceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub searched_face_id: Option<FaceId>,
}

#[doc="<p>Indicates whether or not the face is smiling, and the confidence level in the determination.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Smile {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Boolean value that indicates whether the face is smiling or not.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Boolean>,
}

#[doc="<p>Indicates whether or not the face is wearing sunglasses, and the confidence level in the determination.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Sunglasses {
    #[doc="<p>Level of confidence in the determination.</p>"]
    #[serde(rename="Confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<Percent>,
    #[doc="<p>Boolean value that indicates whether the face is wearing sunglasses or not.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Boolean>,
}

pub type UInteger = i64;
pub type Url = String;
pub type Urls = Vec<Url>;
/// Errors returned by CompareFaces
#[derive(Debug, PartialEq)]
pub enum CompareFacesError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>The input image size exceeds the allowed limit. For more information, see <a>limits</a>. </p>
    ImageTooLarge(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CompareFacesError {
    pub fn from_body(body: &str) -> CompareFacesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CompareFacesError::AccessDenied(String::from(error_message))
                    }
                    "ImageTooLargeException" => {
                        CompareFacesError::ImageTooLarge(String::from(error_message))
                    }
                    "InternalServerError" => {
                        CompareFacesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidImageFormatException" => {
                        CompareFacesError::InvalidImageFormat(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CompareFacesError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidS3ObjectException" => {
                        CompareFacesError::InvalidS3Object(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => CompareFacesError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ThrottlingException" => {
                        CompareFacesError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        CompareFacesError::Validation(error_message.to_string())
                    }
                    _ => CompareFacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => CompareFacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CompareFacesError {
    fn from(err: serde_json::error::Error) -> CompareFacesError {
        CompareFacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CompareFacesError {
    fn from(err: CredentialsError) -> CompareFacesError {
        CompareFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CompareFacesError {
    fn from(err: HttpDispatchError) -> CompareFacesError {
        CompareFacesError::HttpDispatch(err)
    }
}
impl fmt::Display for CompareFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompareFacesError {
    fn description(&self) -> &str {
        match *self {
            CompareFacesError::AccessDenied(ref cause) => cause,
            CompareFacesError::ImageTooLarge(ref cause) => cause,
            CompareFacesError::InternalServerError(ref cause) => cause,
            CompareFacesError::InvalidImageFormat(ref cause) => cause,
            CompareFacesError::InvalidParameter(ref cause) => cause,
            CompareFacesError::InvalidS3Object(ref cause) => cause,
            CompareFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            CompareFacesError::Throttling(ref cause) => cause,
            CompareFacesError::Validation(ref cause) => cause,
            CompareFacesError::Credentials(ref err) => err.description(),
            CompareFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CompareFacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCollection
#[derive(Debug, PartialEq)]
pub enum CreateCollectionError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>A collection with the specified ID already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateCollectionError {
    pub fn from_body(body: &str) -> CreateCollectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        CreateCollectionError::AccessDenied(String::from(error_message))
                    }
                    "InternalServerError" => {
                        CreateCollectionError::InternalServerError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateCollectionError::InvalidParameter(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => CreateCollectionError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ResourceAlreadyExistsException" => {
                        CreateCollectionError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateCollectionError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateCollectionError::Validation(error_message.to_string())
                    }
                    _ => CreateCollectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCollectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCollectionError {
    fn from(err: serde_json::error::Error) -> CreateCollectionError {
        CreateCollectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCollectionError {
    fn from(err: CredentialsError) -> CreateCollectionError {
        CreateCollectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCollectionError {
    fn from(err: HttpDispatchError) -> CreateCollectionError {
        CreateCollectionError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCollectionError {
    fn description(&self) -> &str {
        match *self {
            CreateCollectionError::AccessDenied(ref cause) => cause,
            CreateCollectionError::InternalServerError(ref cause) => cause,
            CreateCollectionError::InvalidParameter(ref cause) => cause,
            CreateCollectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            CreateCollectionError::ResourceAlreadyExists(ref cause) => cause,
            CreateCollectionError::Throttling(ref cause) => cause,
            CreateCollectionError::Validation(ref cause) => cause,
            CreateCollectionError::Credentials(ref err) => err.description(),
            CreateCollectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateCollectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCollection
#[derive(Debug, PartialEq)]
pub enum DeleteCollectionError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Collection specified in the request is not found.</p>
    ResourceNotFound(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteCollectionError {
    pub fn from_body(body: &str) -> DeleteCollectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DeleteCollectionError::AccessDenied(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteCollectionError::InternalServerError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteCollectionError::InvalidParameter(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => DeleteCollectionError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        DeleteCollectionError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteCollectionError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCollectionError::Validation(error_message.to_string())
                    }
                    _ => DeleteCollectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCollectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCollectionError {
    fn from(err: serde_json::error::Error) -> DeleteCollectionError {
        DeleteCollectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCollectionError {
    fn from(err: CredentialsError) -> DeleteCollectionError {
        DeleteCollectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCollectionError {
    fn from(err: HttpDispatchError) -> DeleteCollectionError {
        DeleteCollectionError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCollectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteCollectionError::AccessDenied(ref cause) => cause,
            DeleteCollectionError::InternalServerError(ref cause) => cause,
            DeleteCollectionError::InvalidParameter(ref cause) => cause,
            DeleteCollectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteCollectionError::ResourceNotFound(ref cause) => cause,
            DeleteCollectionError::Throttling(ref cause) => cause,
            DeleteCollectionError::Validation(ref cause) => cause,
            DeleteCollectionError::Credentials(ref err) => err.description(),
            DeleteCollectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteCollectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFaces
#[derive(Debug, PartialEq)]
pub enum DeleteFacesError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Collection specified in the request is not found.</p>
    ResourceNotFound(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteFacesError {
    pub fn from_body(body: &str) -> DeleteFacesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DeleteFacesError::AccessDenied(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DeleteFacesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteFacesError::InvalidParameter(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        DeleteFacesError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteFacesError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteFacesError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFacesError::Validation(error_message.to_string())
                    }
                    _ => DeleteFacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFacesError {
    fn from(err: serde_json::error::Error) -> DeleteFacesError {
        DeleteFacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFacesError {
    fn from(err: CredentialsError) -> DeleteFacesError {
        DeleteFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFacesError {
    fn from(err: HttpDispatchError) -> DeleteFacesError {
        DeleteFacesError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFacesError {
    fn description(&self) -> &str {
        match *self {
            DeleteFacesError::AccessDenied(ref cause) => cause,
            DeleteFacesError::InternalServerError(ref cause) => cause,
            DeleteFacesError::InvalidParameter(ref cause) => cause,
            DeleteFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteFacesError::ResourceNotFound(ref cause) => cause,
            DeleteFacesError::Throttling(ref cause) => cause,
            DeleteFacesError::Validation(ref cause) => cause,
            DeleteFacesError::Credentials(ref err) => err.description(),
            DeleteFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetectFaces
#[derive(Debug, PartialEq)]
pub enum DetectFacesError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>The input image size exceeds the allowed limit. For more information, see <a>limits</a>. </p>
    ImageTooLarge(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetectFacesError {
    pub fn from_body(body: &str) -> DetectFacesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DetectFacesError::AccessDenied(String::from(error_message))
                    }
                    "ImageTooLargeException" => {
                        DetectFacesError::ImageTooLarge(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DetectFacesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidImageFormatException" => {
                        DetectFacesError::InvalidImageFormat(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DetectFacesError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidS3ObjectException" => {
                        DetectFacesError::InvalidS3Object(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        DetectFacesError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DetectFacesError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetectFacesError::Validation(error_message.to_string())
                    }
                    _ => DetectFacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetectFacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetectFacesError {
    fn from(err: serde_json::error::Error) -> DetectFacesError {
        DetectFacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectFacesError {
    fn from(err: CredentialsError) -> DetectFacesError {
        DetectFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectFacesError {
    fn from(err: HttpDispatchError) -> DetectFacesError {
        DetectFacesError::HttpDispatch(err)
    }
}
impl fmt::Display for DetectFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectFacesError {
    fn description(&self) -> &str {
        match *self {
            DetectFacesError::AccessDenied(ref cause) => cause,
            DetectFacesError::ImageTooLarge(ref cause) => cause,
            DetectFacesError::InternalServerError(ref cause) => cause,
            DetectFacesError::InvalidImageFormat(ref cause) => cause,
            DetectFacesError::InvalidParameter(ref cause) => cause,
            DetectFacesError::InvalidS3Object(ref cause) => cause,
            DetectFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectFacesError::Throttling(ref cause) => cause,
            DetectFacesError::Validation(ref cause) => cause,
            DetectFacesError::Credentials(ref err) => err.description(),
            DetectFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectFacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetectLabels
#[derive(Debug, PartialEq)]
pub enum DetectLabelsError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>The input image size exceeds the allowed limit. For more information, see <a>limits</a>. </p>
    ImageTooLarge(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetectLabelsError {
    pub fn from_body(body: &str) -> DetectLabelsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DetectLabelsError::AccessDenied(String::from(error_message))
                    }
                    "ImageTooLargeException" => {
                        DetectLabelsError::ImageTooLarge(String::from(error_message))
                    }
                    "InternalServerError" => {
                        DetectLabelsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidImageFormatException" => {
                        DetectLabelsError::InvalidImageFormat(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DetectLabelsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidS3ObjectException" => {
                        DetectLabelsError::InvalidS3Object(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => DetectLabelsError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ThrottlingException" => {
                        DetectLabelsError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetectLabelsError::Validation(error_message.to_string())
                    }
                    _ => DetectLabelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetectLabelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetectLabelsError {
    fn from(err: serde_json::error::Error) -> DetectLabelsError {
        DetectLabelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectLabelsError {
    fn from(err: CredentialsError) -> DetectLabelsError {
        DetectLabelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectLabelsError {
    fn from(err: HttpDispatchError) -> DetectLabelsError {
        DetectLabelsError::HttpDispatch(err)
    }
}
impl fmt::Display for DetectLabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectLabelsError {
    fn description(&self) -> &str {
        match *self {
            DetectLabelsError::AccessDenied(ref cause) => cause,
            DetectLabelsError::ImageTooLarge(ref cause) => cause,
            DetectLabelsError::InternalServerError(ref cause) => cause,
            DetectLabelsError::InvalidImageFormat(ref cause) => cause,
            DetectLabelsError::InvalidParameter(ref cause) => cause,
            DetectLabelsError::InvalidS3Object(ref cause) => cause,
            DetectLabelsError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectLabelsError::Throttling(ref cause) => cause,
            DetectLabelsError::Validation(ref cause) => cause,
            DetectLabelsError::Credentials(ref err) => err.description(),
            DetectLabelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetectLabelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetectModerationLabels
#[derive(Debug, PartialEq)]
pub enum DetectModerationLabelsError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>The input image size exceeds the allowed limit. For more information, see <a>limits</a>. </p>
    ImageTooLarge(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetectModerationLabelsError {
    pub fn from_body(body: &str) -> DetectModerationLabelsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        DetectModerationLabelsError::AccessDenied(String::from(error_message))
                    }
                    "ImageTooLargeException" => {
                        DetectModerationLabelsError::ImageTooLarge(String::from(error_message))
                    }
                    "InternalServerError" => DetectModerationLabelsError::InternalServerError(String::from(error_message)),
                    "InvalidImageFormatException" => {
                        DetectModerationLabelsError::InvalidImageFormat(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DetectModerationLabelsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidS3ObjectException" => {
                        DetectModerationLabelsError::InvalidS3Object(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => DetectModerationLabelsError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ThrottlingException" => {
                        DetectModerationLabelsError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetectModerationLabelsError::Validation(error_message.to_string())
                    }
                    _ => DetectModerationLabelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetectModerationLabelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetectModerationLabelsError {
    fn from(err: serde_json::error::Error) -> DetectModerationLabelsError {
        DetectModerationLabelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetectModerationLabelsError {
    fn from(err: CredentialsError) -> DetectModerationLabelsError {
        DetectModerationLabelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetectModerationLabelsError {
    fn from(err: HttpDispatchError) -> DetectModerationLabelsError {
        DetectModerationLabelsError::HttpDispatch(err)
    }
}
impl fmt::Display for DetectModerationLabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectModerationLabelsError {
    fn description(&self) -> &str {
        match *self {
            DetectModerationLabelsError::AccessDenied(ref cause) => cause,
            DetectModerationLabelsError::ImageTooLarge(ref cause) => cause,
            DetectModerationLabelsError::InternalServerError(ref cause) => cause,
            DetectModerationLabelsError::InvalidImageFormat(ref cause) => cause,
            DetectModerationLabelsError::InvalidParameter(ref cause) => cause,
            DetectModerationLabelsError::InvalidS3Object(ref cause) => cause,
            DetectModerationLabelsError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectModerationLabelsError::Throttling(ref cause) => cause,
            DetectModerationLabelsError::Validation(ref cause) => cause,
            DetectModerationLabelsError::Credentials(ref err) => err.description(),
            DetectModerationLabelsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetectModerationLabelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCelebrityInfo
#[derive(Debug, PartialEq)]
pub enum GetCelebrityInfoError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Collection specified in the request is not found.</p>
    ResourceNotFound(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetCelebrityInfoError {
    pub fn from_body(body: &str) -> GetCelebrityInfoError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        GetCelebrityInfoError::AccessDenied(String::from(error_message))
                    }
                    "InternalServerError" => {
                        GetCelebrityInfoError::InternalServerError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetCelebrityInfoError::InvalidParameter(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => GetCelebrityInfoError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        GetCelebrityInfoError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetCelebrityInfoError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCelebrityInfoError::Validation(error_message.to_string())
                    }
                    _ => GetCelebrityInfoError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCelebrityInfoError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCelebrityInfoError {
    fn from(err: serde_json::error::Error) -> GetCelebrityInfoError {
        GetCelebrityInfoError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCelebrityInfoError {
    fn from(err: CredentialsError) -> GetCelebrityInfoError {
        GetCelebrityInfoError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCelebrityInfoError {
    fn from(err: HttpDispatchError) -> GetCelebrityInfoError {
        GetCelebrityInfoError::HttpDispatch(err)
    }
}
impl fmt::Display for GetCelebrityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCelebrityInfoError {
    fn description(&self) -> &str {
        match *self {
            GetCelebrityInfoError::AccessDenied(ref cause) => cause,
            GetCelebrityInfoError::InternalServerError(ref cause) => cause,
            GetCelebrityInfoError::InvalidParameter(ref cause) => cause,
            GetCelebrityInfoError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetCelebrityInfoError::ResourceNotFound(ref cause) => cause,
            GetCelebrityInfoError::Throttling(ref cause) => cause,
            GetCelebrityInfoError::Validation(ref cause) => cause,
            GetCelebrityInfoError::Credentials(ref err) => err.description(),
            GetCelebrityInfoError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCelebrityInfoError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by IndexFaces
#[derive(Debug, PartialEq)]
pub enum IndexFacesError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>The input image size exceeds the allowed limit. For more information, see <a>limits</a>. </p>
    ImageTooLarge(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Collection specified in the request is not found.</p>
    ResourceNotFound(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl IndexFacesError {
    pub fn from_body(body: &str) -> IndexFacesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        IndexFacesError::AccessDenied(String::from(error_message))
                    }
                    "ImageTooLargeException" => {
                        IndexFacesError::ImageTooLarge(String::from(error_message))
                    }
                    "InternalServerError" => {
                        IndexFacesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidImageFormatException" => {
                        IndexFacesError::InvalidImageFormat(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        IndexFacesError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidS3ObjectException" => {
                        IndexFacesError::InvalidS3Object(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        IndexFacesError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        IndexFacesError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        IndexFacesError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => IndexFacesError::Validation(error_message.to_string()),
                    _ => IndexFacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => IndexFacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for IndexFacesError {
    fn from(err: serde_json::error::Error) -> IndexFacesError {
        IndexFacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for IndexFacesError {
    fn from(err: CredentialsError) -> IndexFacesError {
        IndexFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for IndexFacesError {
    fn from(err: HttpDispatchError) -> IndexFacesError {
        IndexFacesError::HttpDispatch(err)
    }
}
impl fmt::Display for IndexFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IndexFacesError {
    fn description(&self) -> &str {
        match *self {
            IndexFacesError::AccessDenied(ref cause) => cause,
            IndexFacesError::ImageTooLarge(ref cause) => cause,
            IndexFacesError::InternalServerError(ref cause) => cause,
            IndexFacesError::InvalidImageFormat(ref cause) => cause,
            IndexFacesError::InvalidParameter(ref cause) => cause,
            IndexFacesError::InvalidS3Object(ref cause) => cause,
            IndexFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            IndexFacesError::ResourceNotFound(ref cause) => cause,
            IndexFacesError::Throttling(ref cause) => cause,
            IndexFacesError::Validation(ref cause) => cause,
            IndexFacesError::Credentials(ref err) => err.description(),
            IndexFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            IndexFacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCollections
#[derive(Debug, PartialEq)]
pub enum ListCollectionsError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Collection specified in the request is not found.</p>
    ResourceNotFound(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListCollectionsError {
    pub fn from_body(body: &str) -> ListCollectionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        ListCollectionsError::AccessDenied(String::from(error_message))
                    }
                    "InternalServerError" => {
                        ListCollectionsError::InternalServerError(String::from(error_message))
                    }
                    "InvalidPaginationTokenException" => {
                        ListCollectionsError::InvalidPaginationToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListCollectionsError::InvalidParameter(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => ListCollectionsError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        ListCollectionsError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListCollectionsError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCollectionsError::Validation(error_message.to_string())
                    }
                    _ => ListCollectionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCollectionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCollectionsError {
    fn from(err: serde_json::error::Error) -> ListCollectionsError {
        ListCollectionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCollectionsError {
    fn from(err: CredentialsError) -> ListCollectionsError {
        ListCollectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCollectionsError {
    fn from(err: HttpDispatchError) -> ListCollectionsError {
        ListCollectionsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListCollectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCollectionsError {
    fn description(&self) -> &str {
        match *self {
            ListCollectionsError::AccessDenied(ref cause) => cause,
            ListCollectionsError::InternalServerError(ref cause) => cause,
            ListCollectionsError::InvalidPaginationToken(ref cause) => cause,
            ListCollectionsError::InvalidParameter(ref cause) => cause,
            ListCollectionsError::ProvisionedThroughputExceeded(ref cause) => cause,
            ListCollectionsError::ResourceNotFound(ref cause) => cause,
            ListCollectionsError::Throttling(ref cause) => cause,
            ListCollectionsError::Validation(ref cause) => cause,
            ListCollectionsError::Credentials(ref err) => err.description(),
            ListCollectionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListCollectionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFaces
#[derive(Debug, PartialEq)]
pub enum ListFacesError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Collection specified in the request is not found.</p>
    ResourceNotFound(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListFacesError {
    pub fn from_body(body: &str) -> ListFacesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        ListFacesError::AccessDenied(String::from(error_message))
                    }
                    "InternalServerError" => {
                        ListFacesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidPaginationTokenException" => {
                        ListFacesError::InvalidPaginationToken(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListFacesError::InvalidParameter(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        ListFacesError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListFacesError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListFacesError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => ListFacesError::Validation(error_message.to_string()),
                    _ => ListFacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFacesError {
    fn from(err: serde_json::error::Error) -> ListFacesError {
        ListFacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFacesError {
    fn from(err: CredentialsError) -> ListFacesError {
        ListFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFacesError {
    fn from(err: HttpDispatchError) -> ListFacesError {
        ListFacesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFacesError {
    fn description(&self) -> &str {
        match *self {
            ListFacesError::AccessDenied(ref cause) => cause,
            ListFacesError::InternalServerError(ref cause) => cause,
            ListFacesError::InvalidPaginationToken(ref cause) => cause,
            ListFacesError::InvalidParameter(ref cause) => cause,
            ListFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            ListFacesError::ResourceNotFound(ref cause) => cause,
            ListFacesError::Throttling(ref cause) => cause,
            ListFacesError::Validation(ref cause) => cause,
            ListFacesError::Credentials(ref err) => err.description(),
            ListFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RecognizeCelebrities
#[derive(Debug, PartialEq)]
pub enum RecognizeCelebritiesError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>The input image size exceeds the allowed limit. For more information, see <a>limits</a>. </p>
    ImageTooLarge(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RecognizeCelebritiesError {
    pub fn from_body(body: &str) -> RecognizeCelebritiesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        RecognizeCelebritiesError::AccessDenied(String::from(error_message))
                    }
                    "ImageTooLargeException" => {
                        RecognizeCelebritiesError::ImageTooLarge(String::from(error_message))
                    }
                    "InternalServerError" => {
                        RecognizeCelebritiesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidImageFormatException" => {
                        RecognizeCelebritiesError::InvalidImageFormat(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RecognizeCelebritiesError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidS3ObjectException" => {
                        RecognizeCelebritiesError::InvalidS3Object(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => RecognizeCelebritiesError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ThrottlingException" => {
                        RecognizeCelebritiesError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        RecognizeCelebritiesError::Validation(error_message.to_string())
                    }
                    _ => RecognizeCelebritiesError::Unknown(String::from(body)),
                }
            }
            Err(_) => RecognizeCelebritiesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RecognizeCelebritiesError {
    fn from(err: serde_json::error::Error) -> RecognizeCelebritiesError {
        RecognizeCelebritiesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RecognizeCelebritiesError {
    fn from(err: CredentialsError) -> RecognizeCelebritiesError {
        RecognizeCelebritiesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RecognizeCelebritiesError {
    fn from(err: HttpDispatchError) -> RecognizeCelebritiesError {
        RecognizeCelebritiesError::HttpDispatch(err)
    }
}
impl fmt::Display for RecognizeCelebritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RecognizeCelebritiesError {
    fn description(&self) -> &str {
        match *self {
            RecognizeCelebritiesError::AccessDenied(ref cause) => cause,
            RecognizeCelebritiesError::ImageTooLarge(ref cause) => cause,
            RecognizeCelebritiesError::InternalServerError(ref cause) => cause,
            RecognizeCelebritiesError::InvalidImageFormat(ref cause) => cause,
            RecognizeCelebritiesError::InvalidParameter(ref cause) => cause,
            RecognizeCelebritiesError::InvalidS3Object(ref cause) => cause,
            RecognizeCelebritiesError::ProvisionedThroughputExceeded(ref cause) => cause,
            RecognizeCelebritiesError::Throttling(ref cause) => cause,
            RecognizeCelebritiesError::Validation(ref cause) => cause,
            RecognizeCelebritiesError::Credentials(ref err) => err.description(),
            RecognizeCelebritiesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RecognizeCelebritiesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchFaces
#[derive(Debug, PartialEq)]
pub enum SearchFacesError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Collection specified in the request is not found.</p>
    ResourceNotFound(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SearchFacesError {
    pub fn from_body(body: &str) -> SearchFacesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        SearchFacesError::AccessDenied(String::from(error_message))
                    }
                    "InternalServerError" => {
                        SearchFacesError::InternalServerError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        SearchFacesError::InvalidParameter(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => {
                        SearchFacesError::ProvisionedThroughputExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SearchFacesError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        SearchFacesError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        SearchFacesError::Validation(error_message.to_string())
                    }
                    _ => SearchFacesError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchFacesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchFacesError {
    fn from(err: serde_json::error::Error) -> SearchFacesError {
        SearchFacesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchFacesError {
    fn from(err: CredentialsError) -> SearchFacesError {
        SearchFacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchFacesError {
    fn from(err: HttpDispatchError) -> SearchFacesError {
        SearchFacesError::HttpDispatch(err)
    }
}
impl fmt::Display for SearchFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchFacesError {
    fn description(&self) -> &str {
        match *self {
            SearchFacesError::AccessDenied(ref cause) => cause,
            SearchFacesError::InternalServerError(ref cause) => cause,
            SearchFacesError::InvalidParameter(ref cause) => cause,
            SearchFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            SearchFacesError::ResourceNotFound(ref cause) => cause,
            SearchFacesError::Throttling(ref cause) => cause,
            SearchFacesError::Validation(ref cause) => cause,
            SearchFacesError::Credentials(ref err) => err.description(),
            SearchFacesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchFacesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchFacesByImage
#[derive(Debug, PartialEq)]
pub enum SearchFacesByImageError {
    ///<p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    ///<p>The input image size exceeds the allowed limit. For more information, see <a>limits</a>. </p>
    ImageTooLarge(String),
    ///<p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    ///<p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    ///<p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    ///<p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    ///<p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    ///<p>Collection specified in the request is not found.</p>
    ResourceNotFound(String),
    ///<p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SearchFacesByImageError {
    pub fn from_body(body: &str) -> SearchFacesByImageError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedException" => {
                        SearchFacesByImageError::AccessDenied(String::from(error_message))
                    }
                    "ImageTooLargeException" => {
                        SearchFacesByImageError::ImageTooLarge(String::from(error_message))
                    }
                    "InternalServerError" => {
                        SearchFacesByImageError::InternalServerError(String::from(error_message))
                    }
                    "InvalidImageFormatException" => {
                        SearchFacesByImageError::InvalidImageFormat(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        SearchFacesByImageError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidS3ObjectException" => {
                        SearchFacesByImageError::InvalidS3Object(String::from(error_message))
                    }
                    "ProvisionedThroughputExceededException" => SearchFacesByImageError::ProvisionedThroughputExceeded(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        SearchFacesByImageError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        SearchFacesByImageError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        SearchFacesByImageError::Validation(error_message.to_string())
                    }
                    _ => SearchFacesByImageError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchFacesByImageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchFacesByImageError {
    fn from(err: serde_json::error::Error) -> SearchFacesByImageError {
        SearchFacesByImageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchFacesByImageError {
    fn from(err: CredentialsError) -> SearchFacesByImageError {
        SearchFacesByImageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchFacesByImageError {
    fn from(err: HttpDispatchError) -> SearchFacesByImageError {
        SearchFacesByImageError::HttpDispatch(err)
    }
}
impl fmt::Display for SearchFacesByImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchFacesByImageError {
    fn description(&self) -> &str {
        match *self {
            SearchFacesByImageError::AccessDenied(ref cause) => cause,
            SearchFacesByImageError::ImageTooLarge(ref cause) => cause,
            SearchFacesByImageError::InternalServerError(ref cause) => cause,
            SearchFacesByImageError::InvalidImageFormat(ref cause) => cause,
            SearchFacesByImageError::InvalidParameter(ref cause) => cause,
            SearchFacesByImageError::InvalidS3Object(ref cause) => cause,
            SearchFacesByImageError::ProvisionedThroughputExceeded(ref cause) => cause,
            SearchFacesByImageError::ResourceNotFound(ref cause) => cause,
            SearchFacesByImageError::Throttling(ref cause) => cause,
            SearchFacesByImageError::Validation(ref cause) => cause,
            SearchFacesByImageError::Credentials(ref err) => err.description(),
            SearchFacesByImageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SearchFacesByImageError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Rekognition API. Amazon Rekognition clients implement this trait.
pub trait Rekognition {
    #[doc="<p>Compares a face in the <i>source</i> input image with each face detected in the <i>target</i> input image. </p> <note> <p> If the source image contains multiple faces, the service detects the largest face and compares it with each face detected in the target image. </p> </note> <p>In response, the operation returns an array of face matches ordered by similarity score in descending order. For each face match, the response provides a bounding box of the face, facial landmarks, pose details (pitch, role, and yaw), quality (brightness and sharpness), and confidence value (indicating the level of confidence that the bounding box contains a face). The response also provides a similarity score, which indicates how closely the faces match. </p> <note> <p>By default, only faces with a similarity score of greater than or equal to 80% are returned in the response. You can change this value by specifying the <code>SimilarityThreshold</code> parameter.</p> </note> <p> <code>CompareFaces</code> also returns an array of faces that don't match the source image. For each face, it returns a bounding box, confidence value, landmarks, pose details, and quality. The response also returns information about the face in the source image, including the bounding box of the face and confidence value.</p> <p>If the image doesn't contain Exif metadata, <code>CompareFaces</code> returns orientation information for the source and target images. Use these values to display the images with the correct image orientation.</p> <note> <p> This is a stateless API operation. That is, data returned by this operation doesn't persist.</p> </note> <p>For an example, see <a>get-started-exercise-compare-faces</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:CompareFaces</code> action.</p>"]
    fn compare_faces(&self,
                     input: &CompareFacesRequest)
                     -> Result<CompareFacesResponse, CompareFacesError>;


    #[doc="<p>Creates a collection in an AWS Region. You can add faces to the collection using the operation. </p> <p>For example, you might create collections, one for each of your application users. A user can then index faces using the <code>IndexFaces</code> operation and persist results in a specific collection. Then, a user can search the collection for faces in the user-specific container. </p> <note> <p>Collection names are case-sensitive.</p> </note> <p>For an example, see <a>example1</a>. </p> <p>This operation requires permissions to perform the <code>rekognition:CreateCollection</code> action.</p>"]
    fn create_collection(&self,
                         input: &CreateCollectionRequest)
                         -> Result<CreateCollectionResponse, CreateCollectionError>;


    #[doc="<p>Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see <a>example1</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteCollection</code> action.</p>"]
    fn delete_collection(&self,
                         input: &DeleteCollectionRequest)
                         -> Result<DeleteCollectionResponse, DeleteCollectionError>;


    #[doc="<p>Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteFaces</code> action.</p>"]
    fn delete_faces(&self,
                    input: &DeleteFacesRequest)
                    -> Result<DeleteFacesResponse, DeleteFacesError>;


    #[doc="<p>Detects faces within an image (JPEG or PNG) that is provided as input.</p> <p> For each face detected, the operation returns face details including a bounding box of the face, a confidence value (that the bounding box contains a face), and a fixed set of attributes such as facial landmarks (for example, coordinates of eye and mouth), gender, presence of beard, sunglasses, etc. </p> <p>The face-detection algorithm is most effective on frontal faces. For non-frontal or obscured faces, the algorithm may not detect the faces or might detect faces with lower confidence. </p> <note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> </note> <p>For an example, see <a>get-started-exercise-detect-faces</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectFaces</code> action. </p>"]
    fn detect_faces(&self,
                    input: &DetectFacesRequest)
                    -> Result<DetectFacesResponse, DetectFacesError>;


    #[doc="<p>Detects instances of real-world labels within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like landscape, evening, and nature. For an example, see <a>get-started-exercise-detect-labels</a>.</p> <p> For each object, scene, and concept the API returns one or more labels. Each label provides the object name, and the level of confidence that the image contains the object. For example, suppose the input image has a lighthouse, the sea, and a rock. The response will include all three labels, one for each object. </p> <p> <code>{Name: lighthouse, Confidence: 98.4629}</code> </p> <p> <code>{Name: rock,Confidence: 79.2097}</code> </p> <p> <code> {Name: sea,Confidence: 75.061}</code> </p> <p> In the preceding example, the operation returns one label for each of the three objects. The operation can also return multiple labels for the same object in the image. For example, if the input image shows a flower (for example, a tulip), the operation might return the following three labels. </p> <p> <code>{Name: flower,Confidence: 99.0562}</code> </p> <p> <code>{Name: plant,Confidence: 99.0562}</code> </p> <p> <code>{Name: tulip,Confidence: 99.0562}</code> </p> <p>In this example, the detection algorithm more precisely identifies the flower as a tulip.</p> <p>You can provide the input image as an S3 object or as base64-encoded bytes. In response, the API returns an array of labels. In addition, the response also includes the orientation correction. Optionally, you can specify <code>MinConfidence</code> to control the confidence threshold for the labels returned. The default is 50%. You can also add the <code>MaxLabels</code> parameter to limit the number of labels returned. </p> <note> <p>If the object detected is a person, the operation doesn't provide the same facial details that the <a>DetectFaces</a> operation provides.</p> </note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectLabels</code> action. </p>"]
    fn detect_labels(&self,
                     input: &DetectLabelsRequest)
                     -> Result<DetectLabelsResponse, DetectLabelsError>;


    #[doc="<p>Detects explicit or suggestive adult content in a specified JPEG or PNG format image. Use <code>DetectModerationLabels</code> to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not images containing suggestive content.</p> <p>To filter images, use the labels returned by <code>DetectModerationLabels</code> to determine which types of content are appropriate. For information about moderation labels, see <a>image-moderation</a>.</p>"]
    fn detect_moderation_labels
        (&self,
         input: &DetectModerationLabelsRequest)
         -> Result<DetectModerationLabelsResponse, DetectModerationLabelsError>;


    #[doc="<p>Gets the name and additional information about a celebrity based on his or her Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, this list is empty. For more information, see <a>celebrity-recognition</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:GetCelebrityInfo</code> action. </p>"]
    fn get_celebrity_info(&self,
                          input: &GetCelebrityInfoRequest)
                          -> Result<GetCelebrityInfoResponse, GetCelebrityInfoError>;


    #[doc="<p>Detects faces in the input image and adds them to the specified collection. </p> <p> Amazon Rekognition does not save the actual faces detected. Instead, the underlying detection algorithm first detects the faces in the input image, and for each face extracts facial features into a feature vector, and stores it in the back-end database. Amazon Rekognition uses feature vectors when performing face match and search operations using the and operations. </p> <p>If you provide the optional <code>externalImageID</code> for the input image you provided, Amazon Rekognition associates this ID with all faces that it detects. When you call the operation, the response returns the external ID. You can use this external image ID to create a client-side index to associate the faces with each image. You can then use the index to find all faces in an image. </p> <p>In response, the operation returns an array of metadata for all detected faces. This includes, the bounding box of the detected face, confidence value (indicating the bounding box contains a face), a face ID assigned by the service for each face that is detected and stored, and an image ID assigned by the service for the input image. If you request all facial attributes (using the <code>detectionAttributes</code> parameter, Amazon Rekognition returns detailed facial attributes such as facial landmarks (for example, location of eye and mount) and other facial attributes such gender. If you provide the same image, specify the same collection, and use the same external ID in the <code>IndexFaces</code> operation, Amazon Rekognition doesn't save duplicate face metadata. </p> <p>For an example, see <a>example2</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:IndexFaces</code> action.</p>"]
    fn index_faces(&self,
                   input: &IndexFacesRequest)
                   -> Result<IndexFacesResponse, IndexFacesError>;


    #[doc="<p>Returns list of collection IDs in your account. If the result is truncated, the response also provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p> <p>For an example, see <a>example1</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:ListCollections</code> action.</p>"]
    fn list_collections(&self,
                        input: &ListCollectionsRequest)
                        -> Result<ListCollectionsResponse, ListCollectionsError>;


    #[doc="<p>Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, see <a>example3</a>. </p> <p>This operation requires permissions to perform the <code>rekognition:ListFaces</code> action.</p>"]
    fn list_faces(&self, input: &ListFacesRequest) -> Result<ListFacesResponse, ListFacesError>;


    #[doc="<p>Returns an array of celebrities recognized in the input image. The image is passed either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. The image must be either a PNG or JPEG formatted file. For more information, see <a>celebrity-recognition</a>. </p> <p> <code>RecognizeCelebrities</code> returns the 15 largest faces in the image. It lists recognized celebrities in the <code>CelebrityFaces</code> list and unrecognized faces in the <code>UnrecognizedFaces</code> list. The operation doesn't return celebrities whose face sizes are smaller than the largest 15 faces in the image.</p> <p>For each celebrity recognized, the API returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p> <p>Rekognition does not retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the operation.</p> <p>For an example, see <a>recognize-celebrities-tutorial</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>"]
    fn recognize_celebrities(&self,
                             input: &RecognizeCelebritiesRequest)
                             -> Result<RecognizeCelebritiesResponse, RecognizeCelebritiesError>;


    #[doc="<p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <a>IndexFaces</a> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p> </note> <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p> <p>For an example, see <a>example3</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>"]
    fn search_faces(&self,
                    input: &SearchFacesRequest)
                    -> Result<SearchFacesResponse, SearchFacesError>;


    #[doc="<p>For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p> To search for all faces in an input image, you might first call the operation, and then use the face IDs returned in subsequent calls to the operation. </p> <p> You can also call the <code>DetectFaces</code> operation and use the bounding boxes in the response to make face crops, which then you can pass in to the <code>SearchFacesByImage</code> operation. </p> </note> <p> The response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match found. Along with the metadata, the response also includes a <code>similarity</code> indicating how similar the face is to the input face. In the response, the operation also returns the bounding box (and a confidence level that the bounding box contains a face) of the face that Amazon Rekognition used for the input image. </p> <p>For an example, see <a>example3</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFacesByImage</code> action.</p>"]
    fn search_faces_by_image(&self,
                             input: &SearchFacesByImageRequest)
                             -> Result<SearchFacesByImageResponse, SearchFacesByImageError>;
}
/// A client for the Amazon Rekognition API.
pub struct RekognitionClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> RekognitionClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        RekognitionClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Rekognition for RekognitionClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Compares a face in the <i>source</i> input image with each face detected in the <i>target</i> input image. </p> <note> <p> If the source image contains multiple faces, the service detects the largest face and compares it with each face detected in the target image. </p> </note> <p>In response, the operation returns an array of face matches ordered by similarity score in descending order. For each face match, the response provides a bounding box of the face, facial landmarks, pose details (pitch, role, and yaw), quality (brightness and sharpness), and confidence value (indicating the level of confidence that the bounding box contains a face). The response also provides a similarity score, which indicates how closely the faces match. </p> <note> <p>By default, only faces with a similarity score of greater than or equal to 80% are returned in the response. You can change this value by specifying the <code>SimilarityThreshold</code> parameter.</p> </note> <p> <code>CompareFaces</code> also returns an array of faces that don't match the source image. For each face, it returns a bounding box, confidence value, landmarks, pose details, and quality. The response also returns information about the face in the source image, including the bounding box of the face and confidence value.</p> <p>If the image doesn't contain Exif metadata, <code>CompareFaces</code> returns orientation information for the source and target images. Use these values to display the images with the correct image orientation.</p> <note> <p> This is a stateless API operation. That is, data returned by this operation doesn't persist.</p> </note> <p>For an example, see <a>get-started-exercise-compare-faces</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:CompareFaces</code> action.</p>"]
    fn compare_faces(&self,
                     input: &CompareFacesRequest)
                     -> Result<CompareFacesResponse, CompareFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CompareFaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CompareFacesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CompareFacesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a collection in an AWS Region. You can add faces to the collection using the operation. </p> <p>For example, you might create collections, one for each of your application users. A user can then index faces using the <code>IndexFaces</code> operation and persist results in a specific collection. Then, a user can search the collection for faces in the user-specific container. </p> <note> <p>Collection names are case-sensitive.</p> </note> <p>For an example, see <a>example1</a>. </p> <p>This operation requires permissions to perform the <code>rekognition:CreateCollection</code> action.</p>"]
    fn create_collection(&self,
                         input: &CreateCollectionRequest)
                         -> Result<CreateCollectionResponse, CreateCollectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateCollection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateCollectionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateCollectionError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see <a>example1</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteCollection</code> action.</p>"]
    fn delete_collection(&self,
                         input: &DeleteCollectionRequest)
                         -> Result<DeleteCollectionResponse, DeleteCollectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteCollection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteCollectionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteCollectionError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteFaces</code> action.</p>"]
    fn delete_faces(&self,
                    input: &DeleteFacesRequest)
                    -> Result<DeleteFacesResponse, DeleteFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteFaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteFacesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteFacesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Detects faces within an image (JPEG or PNG) that is provided as input.</p> <p> For each face detected, the operation returns face details including a bounding box of the face, a confidence value (that the bounding box contains a face), and a fixed set of attributes such as facial landmarks (for example, coordinates of eye and mouth), gender, presence of beard, sunglasses, etc. </p> <p>The face-detection algorithm is most effective on frontal faces. For non-frontal or obscured faces, the algorithm may not detect the faces or might detect faces with lower confidence. </p> <note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> </note> <p>For an example, see <a>get-started-exercise-detect-faces</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectFaces</code> action. </p>"]
    fn detect_faces(&self,
                    input: &DetectFacesRequest)
                    -> Result<DetectFacesResponse, DetectFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectFaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DetectFacesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DetectFacesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Detects instances of real-world labels within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like landscape, evening, and nature. For an example, see <a>get-started-exercise-detect-labels</a>.</p> <p> For each object, scene, and concept the API returns one or more labels. Each label provides the object name, and the level of confidence that the image contains the object. For example, suppose the input image has a lighthouse, the sea, and a rock. The response will include all three labels, one for each object. </p> <p> <code>{Name: lighthouse, Confidence: 98.4629}</code> </p> <p> <code>{Name: rock,Confidence: 79.2097}</code> </p> <p> <code> {Name: sea,Confidence: 75.061}</code> </p> <p> In the preceding example, the operation returns one label for each of the three objects. The operation can also return multiple labels for the same object in the image. For example, if the input image shows a flower (for example, a tulip), the operation might return the following three labels. </p> <p> <code>{Name: flower,Confidence: 99.0562}</code> </p> <p> <code>{Name: plant,Confidence: 99.0562}</code> </p> <p> <code>{Name: tulip,Confidence: 99.0562}</code> </p> <p>In this example, the detection algorithm more precisely identifies the flower as a tulip.</p> <p>You can provide the input image as an S3 object or as base64-encoded bytes. In response, the API returns an array of labels. In addition, the response also includes the orientation correction. Optionally, you can specify <code>MinConfidence</code> to control the confidence threshold for the labels returned. The default is 50%. You can also add the <code>MaxLabels</code> parameter to limit the number of labels returned. </p> <note> <p>If the object detected is a person, the operation doesn't provide the same facial details that the <a>DetectFaces</a> operation provides.</p> </note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectLabels</code> action. </p>"]
    fn detect_labels(&self,
                     input: &DetectLabelsRequest)
                     -> Result<DetectLabelsResponse, DetectLabelsError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectLabels");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DetectLabelsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DetectLabelsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Detects explicit or suggestive adult content in a specified JPEG or PNG format image. Use <code>DetectModerationLabels</code> to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not images containing suggestive content.</p> <p>To filter images, use the labels returned by <code>DetectModerationLabels</code> to determine which types of content are appropriate. For information about moderation labels, see <a>image-moderation</a>.</p>"]
    fn detect_moderation_labels
        (&self,
         input: &DetectModerationLabelsRequest)
         -> Result<DetectModerationLabelsResponse, DetectModerationLabelsError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectModerationLabels");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DetectModerationLabelsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DetectModerationLabelsError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Gets the name and additional information about a celebrity based on his or her Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, this list is empty. For more information, see <a>celebrity-recognition</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:GetCelebrityInfo</code> action. </p>"]
    fn get_celebrity_info(&self,
                          input: &GetCelebrityInfoRequest)
                          -> Result<GetCelebrityInfoResponse, GetCelebrityInfoError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetCelebrityInfo");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetCelebrityInfoResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetCelebrityInfoError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Detects faces in the input image and adds them to the specified collection. </p> <p> Amazon Rekognition does not save the actual faces detected. Instead, the underlying detection algorithm first detects the faces in the input image, and for each face extracts facial features into a feature vector, and stores it in the back-end database. Amazon Rekognition uses feature vectors when performing face match and search operations using the and operations. </p> <p>If you provide the optional <code>externalImageID</code> for the input image you provided, Amazon Rekognition associates this ID with all faces that it detects. When you call the operation, the response returns the external ID. You can use this external image ID to create a client-side index to associate the faces with each image. You can then use the index to find all faces in an image. </p> <p>In response, the operation returns an array of metadata for all detected faces. This includes, the bounding box of the detected face, confidence value (indicating the bounding box contains a face), a face ID assigned by the service for each face that is detected and stored, and an image ID assigned by the service for the input image. If you request all facial attributes (using the <code>detectionAttributes</code> parameter, Amazon Rekognition returns detailed facial attributes such as facial landmarks (for example, location of eye and mount) and other facial attributes such gender. If you provide the same image, specify the same collection, and use the same external ID in the <code>IndexFaces</code> operation, Amazon Rekognition doesn't save duplicate face metadata. </p> <p>For an example, see <a>example2</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:IndexFaces</code> action.</p>"]
    fn index_faces(&self,
                   input: &IndexFacesRequest)
                   -> Result<IndexFacesResponse, IndexFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.IndexFaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<IndexFacesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(IndexFacesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns list of collection IDs in your account. If the result is truncated, the response also provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p> <p>For an example, see <a>example1</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:ListCollections</code> action.</p>"]
    fn list_collections(&self,
                        input: &ListCollectionsRequest)
                        -> Result<ListCollectionsResponse, ListCollectionsError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListCollections");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListCollectionsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListCollectionsError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, see <a>example3</a>. </p> <p>This operation requires permissions to perform the <code>rekognition:ListFaces</code> action.</p>"]
    fn list_faces(&self, input: &ListFacesRequest) -> Result<ListFacesResponse, ListFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListFaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListFacesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListFacesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns an array of celebrities recognized in the input image. The image is passed either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. The image must be either a PNG or JPEG formatted file. For more information, see <a>celebrity-recognition</a>. </p> <p> <code>RecognizeCelebrities</code> returns the 15 largest faces in the image. It lists recognized celebrities in the <code>CelebrityFaces</code> list and unrecognized faces in the <code>UnrecognizedFaces</code> list. The operation doesn't return celebrities whose face sizes are smaller than the largest 15 faces in the image.</p> <p>For each celebrity recognized, the API returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p> <p>Rekognition does not retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the operation.</p> <p>For an example, see <a>recognize-celebrities-tutorial</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>"]
    fn recognize_celebrities(&self,
                             input: &RecognizeCelebritiesRequest)
                             -> Result<RecognizeCelebritiesResponse, RecognizeCelebritiesError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.RecognizeCelebrities");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RecognizeCelebritiesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RecognizeCelebritiesError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <a>IndexFaces</a> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p> </note> <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p> <p>For an example, see <a>example3</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>"]
    fn search_faces(&self,
                    input: &SearchFacesRequest)
                    -> Result<SearchFacesResponse, SearchFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.SearchFaces");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<SearchFacesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(SearchFacesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p> To search for all faces in an input image, you might first call the operation, and then use the face IDs returned in subsequent calls to the operation. </p> <p> You can also call the <code>DetectFaces</code> operation and use the bounding boxes in the response to make face crops, which then you can pass in to the <code>SearchFacesByImage</code> operation. </p> </note> <p> The response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match found. Along with the metadata, the response also includes a <code>similarity</code> indicating how similar the face is to the input face. In the response, the operation also returns the bounding box (and a confidence level that the bounding box contains a face) of the face that Amazon Rekognition used for the input image. </p> <p>For an example, see <a>example3</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFacesByImage</code> action.</p>"]
    fn search_faces_by_image(&self,
                             input: &SearchFacesByImageRequest)
                             -> Result<SearchFacesByImageResponse, SearchFacesByImageError> {
        let mut request = SignedRequest::new("POST", "rekognition", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.SearchFacesByImage");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<SearchFacesByImageResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(SearchFacesByImageError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
