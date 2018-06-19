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
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use hyper::StatusCode;
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>The address that you want the Snowball or Snowballs associated with a specific job to be shipped to. Addresses are validated at the time of creation. The address you provide must be located within the serviceable area of your region. Although no individual elements of the <code>Address</code> are required, if the address is invalid or unsupported, then an exception is thrown.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// <p>The unique ID for an address.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The city in an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// <p>The name of the company to receive a Snowball at an address.</p>
    #[serde(rename = "Company")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// <p>The country in an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>If the address you are creating is a primary address, then set this option to true. This field is not supported in most regions.</p>
    #[serde(rename = "IsRestricted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_restricted: Option<bool>,
    /// <p>This field is no longer used and the value is ignored.</p>
    #[serde(rename = "Landmark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmark: Option<String>,
    /// <p>The name of a person to receive a Snowball at an address.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The phone number associated with an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "PhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// <p>The postal code in an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "PostalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// <p>This field is no longer used and the value is ignored.</p>
    #[serde(rename = "PrefectureOrDistrict")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefecture_or_district: Option<String>,
    /// <p>The state or province in an address that a Snowball is to be delivered to.</p>
    #[serde(rename = "StateOrProvince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    /// <p>The first line in a street address that a Snowball is to be delivered to.</p>
    #[serde(rename = "Street1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    /// <p>The second line in a street address that a Snowball is to be delivered to.</p>
    #[serde(rename = "Street2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    /// <p>The third line in a street address that a Snowball is to be delivered to.</p>
    #[serde(rename = "Street3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_3: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelClusterRequest {
    /// <p>The 39-character ID for the cluster that you want to cancel, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelClusterResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelJobRequest {
    /// <p>The 39-character job ID for the job that you want to cancel, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CancelJobResult {}

/// <p>Contains a cluster's state, a cluster's ID, and other important information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ClusterListEntry {
    /// <p>The 39-character ID for the cluster that you want to list, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The current state of this cluster. For information about the state of a specific node, see <a>JobListEntry$JobState</a>.</p>
    #[serde(rename = "ClusterState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<String>,
    /// <p>The creation date for this cluster.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Defines an optional description of the cluster, for example <code>Environmental Data Cluster-01</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>Contains metadata about a specific cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ClusterMetadata {
    /// <p>The automatically generated ID for a specific address.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The automatically generated ID for a cluster.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The current status of the cluster.</p>
    #[serde(rename = "ClusterState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<String>,
    /// <p>The creation date for this cluster.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The optional description of the cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the address that you want a cluster shipped to, after it will be shipped to its primary address. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The type of job for this cluster. Currently, the only job type supported for clusters is <code>LOCAL_USE</code>.</p>
    #[serde(rename = "JobType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// <p>The <code>KmsKeyARN</code> Amazon Resource Name (ARN) associated with this cluster. This ARN was created using the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> API action in AWS Key Management Service (AWS KMS).</p>
    #[serde(rename = "KmsKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The Amazon Simple Notification Service (Amazon SNS) notification settings for this cluster.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>The arrays of <a>JobResource</a> objects that can include updated <a>S3Resource</a> objects or <a>LambdaResource</a> objects.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The role ARN associated with this cluster. This ARN was created using the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in AWS Identity and Access Management (IAM).</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The shipping speed for each node in this cluster. This speed doesn&#39;t dictate how soon you&#39;ll get each Snowball Edge appliance, rather it represents how quickly each appliance moves to its destination while in transit. Regional shipping speeds are as follows:</p> <ul> <li> <p>In Australia, you have access to express shipping. Typically, appliances shipped express are delivered in about a day.</p> </li> <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snowball Edges shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li> <li> <p>In India, Snowball Edges are delivered in one to seven days.</p> </li> <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li> </ul></p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
    /// <p>The type of AWS Snowball appliance to use for this cluster. Currently, the only supported appliance type for cluster jobs is <code>EDGE</code>.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAddressRequest {
    /// <p>The address that you want the Snowball shipped to.</p>
    #[serde(rename = "Address")]
    pub address: Address,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateAddressResult {
    /// <p>The automatically generated ID for a specific address. You'll use this ID when you create a job to specify which address you want the Snowball for that job shipped to.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClusterRequest {
    /// <p>The ID for the address that you want the cluster shipped to.</p>
    #[serde(rename = "AddressId")]
    pub address_id: String,
    /// <p>An optional description of this specific cluster, for example <code>Environmental Data Cluster-01</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The forwarding address ID for a cluster. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The type of job for this cluster. Currently, the only job type supported for clusters is <code>LOCAL_USE</code>.</p>
    #[serde(rename = "JobType")]
    pub job_type: String,
    /// <p>The <code>KmsKeyARN</code> value that you want to associate with this cluster. <code>KmsKeyARN</code> values are created by using the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> API action in AWS Key Management Service (AWS KMS). </p>
    #[serde(rename = "KmsKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The Amazon Simple Notification Service (Amazon SNS) notification settings for this cluster.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>The resources associated with the cluster job. These resources include Amazon S3 buckets and optional AWS Lambda functions written in the Python language. </p>
    #[serde(rename = "Resources")]
    pub resources: JobResource,
    /// <p>The <code>RoleARN</code> that you want to associate with this cluster. <code>RoleArn</code> values are created by using the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in AWS Identity and Access Management (IAM).</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p><p>The shipping speed for each node in this cluster. This speed doesn&#39;t dictate how soon you&#39;ll get each Snowball Edge appliance, rather it represents how quickly each appliance moves to its destination while in transit. Regional shipping speeds are as follows:</p> <ul> <li> <p>In Australia, you have access to express shipping. Typically, appliances shipped express are delivered in about a day.</p> </li> <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snowball Edges shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li> <li> <p>In India, Snowball Edges are delivered in one to seven days.</p> </li> <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li> </ul></p>
    #[serde(rename = "ShippingOption")]
    pub shipping_option: String,
    /// <p>The type of AWS Snowball appliance to use for this cluster. Currently, the only supported appliance type for cluster jobs is <code>EDGE</code>.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateClusterResult {
    /// <p>The automatically generated ID for a cluster.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateJobRequest {
    /// <p>The ID for the address that you want the Snowball shipped to.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The ID of a cluster. If you're creating a job for a node in a cluster, you need to provide only this <code>clusterId</code> value. The other job attributes are inherited from the cluster.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>Defines an optional description of this specific job, for example <code>Important Photos 2016-08-11</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The forwarding address ID for a job. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>Defines the type of job that you're creating. </p>
    #[serde(rename = "JobType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// <p>The <code>KmsKeyARN</code> that you want to associate with this job. <code>KmsKeyARN</code>s are created using the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> AWS Key Management Service (KMS) API action.</p>
    #[serde(rename = "KmsKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>Defines the Amazon Simple Notification Service (Amazon SNS) notification settings for this job.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>Defines the Amazon S3 buckets associated with this job.</p> <p>With <code>IMPORT</code> jobs, you specify the bucket or buckets that your transferred data will be imported into.</p> <p>With <code>EXPORT</code> jobs, you specify the bucket or buckets that your transferred data will be exported from. Optionally, you can also specify a <code>KeyRange</code> value. If you choose to export a range, you define the length of the range by providing either an inclusive <code>BeginMarker</code> value, an inclusive <code>EndMarker</code> value, or both. Ranges are UTF-8 binary sorted.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The <code>RoleARN</code> that you want to associate with this job. <code>RoleArn</code>s are created using the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> AWS Identity and Access Management (IAM) API action.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The shipping speed for this job. This speed doesn&#39;t dictate how soon you&#39;ll get the Snowball, rather it represents how quickly the Snowball moves to its destination while in transit. Regional shipping speeds are as follows:</p> <ul> <li> <p>In Australia, you have access to express shipping. Typically, Snowballs shipped express are delivered in about a day.</p> </li> <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snowballs shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li> <li> <p>In India, Snowballs are delivered in one to seven days.</p> </li> <li> <p>In the US, you have access to one-day shipping and two-day shipping.</p> </li> </ul></p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
    /// <p>If your job is being created in one of the US regions, you have the option of specifying what size Snowball you'd like for this job. In all other regions, Snowballs come with 80 TB in storage capacity.</p>
    #[serde(rename = "SnowballCapacityPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_capacity_preference: Option<String>,
    /// <p>The type of AWS Snowball appliance to use for this job. Currently, the only supported appliance type for cluster jobs is <code>EDGE</code>.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateJobResult {
    /// <p>The automatically generated ID for a job, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>Defines the real-time status of a Snowball's data transfer while the appliance is at AWS. This data is only available while a job has a <code>JobState</code> value of <code>InProgress</code>, for both import and export jobs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DataTransfer {
    /// <p>The number of bytes transferred between a Snowball and Amazon S3.</p>
    #[serde(rename = "BytesTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    /// <p>The number of objects transferred between a Snowball and Amazon S3.</p>
    #[serde(rename = "ObjectsTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects_transferred: Option<i64>,
    /// <p>The total bytes of data for a transfer between a Snowball and Amazon S3. This value is set to 0 (zero) until all the keys that will be transferred have been listed.</p>
    #[serde(rename = "TotalBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i64>,
    /// <p>The total number of objects for a transfer between a Snowball and Amazon S3. This value is set to 0 (zero) until all the keys that will be transferred have been listed.</p>
    #[serde(rename = "TotalObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAddressRequest {
    /// <p>The automatically generated ID for a specific address.</p>
    #[serde(rename = "AddressId")]
    pub address_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAddressResult {
    /// <p>The address that you want the Snowball or Snowballs associated with a specific job to be shipped to.</p>
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAddressesRequest {
    /// <p>The number of <code>ADDRESS</code> objects to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>ADDRESS</code> objects, you have the option of specifying a value for <code>NextToken</code> as the starting point for your list of returned addresses.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAddressesResult {
    /// <p>The Snowball shipping addresses that were created for this account.</p>
    #[serde(rename = "Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>DescribeAddresses</code> call, your list of returned addresses will start from this point in the array.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClusterRequest {
    /// <p>The automatically generated ID for a cluster.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeClusterResult {
    /// <p>Information about a specific cluster, including shipping information, cluster status, and other important metadata.</p>
    #[serde(rename = "ClusterMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_metadata: Option<ClusterMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeJobRequest {
    /// <p>The automatically generated ID for a job, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeJobResult {
    /// <p>Information about a specific job, including shipping information, job status, and other important metadata.</p>
    #[serde(rename = "JobMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_metadata: Option<JobMetadata>,
    /// <p>Information about a specific job part (in the case of an export job), including shipping information, job status, and other important metadata.</p>
    #[serde(rename = "SubJobMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_job_metadata: Option<Vec<JobMetadata>>,
}

/// <p>The container for the <a>EventTriggerDefinition$EventResourceARN</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventTriggerDefinition {
    /// <p>The Amazon Resource Name (ARN) for any local Amazon S3 resource that is an AWS Lambda function's event trigger associated with this job.</p>
    #[serde(rename = "EventResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resource_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobManifestRequest {
    /// <p>The ID for a job that you want to get the manifest file for, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetJobManifestResult {
    /// <p>The Amazon S3 presigned URL for the manifest file associated with the specified <code>JobId</code> value.</p>
    #[serde(rename = "ManifestURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobUnlockCodeRequest {
    /// <p>The ID for the job that you want to get the <code>UnlockCode</code> value for, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetJobUnlockCodeResult {
    /// <p>The <code>UnlockCode</code> value for the specified job. The <code>UnlockCode</code> value can be accessed for up to 90 days after the job has been created.</p>
    #[serde(rename = "UnlockCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSnowballUsageRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSnowballUsageResult {
    /// <p>The service limit for number of Snowballs this account can have at once. The default service limit is 1 (one).</p>
    #[serde(rename = "SnowballLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_limit: Option<i64>,
    /// <p>The number of Snowballs that this account is currently using.</p>
    #[serde(rename = "SnowballsInUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowballs_in_use: Option<i64>,
}

/// <p>Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of an export job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct JobListEntry {
    /// <p>The creation date for this job.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The optional description of this specific job, for example <code>Important Photos 2016-08-11</code>.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A value that indicates that this job is a master job. A master job represents a successful request to create an export job. Master jobs aren't associated with any Snowballs. Instead, each master job will have at least one job part, and each job part is associated with a Snowball. It might take some time before the job parts associated with a particular master job are listed, because they are created after the master job is created.</p>
    #[serde(rename = "IsMaster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_master: Option<bool>,
    /// <p>The automatically generated ID for a job, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The current state of this job.</p>
    #[serde(rename = "JobState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_state: Option<String>,
    /// <p>The type of job.</p>
    #[serde(rename = "JobType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// <p>The type of appliance used with this job.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

/// <p>Contains job logs. Whenever Snowball is used to import data into or export data out of Amazon S3, you'll have the option of downloading a PDF job report. Job logs are returned as a part of the response syntax of the <code>DescribeJob</code> action in the <code>JobMetadata</code> data type. The job logs can be accessed for up to 60 minutes after this request has been made. To access any of the job logs after 60 minutes have passed, you'll have to make another call to the <code>DescribeJob</code> action.</p> <p>For import jobs, the PDF job report becomes available at the end of the import process. For export jobs, your job report typically becomes available while the Snowball for your job part is being delivered to you.</p> <p>The job report provides you insight into the state of your Amazon S3 data transfer. The report includes details about your job or job part for your records.</p> <p>For deeper visibility into the status of your transferred objects, you can look at the two associated logs: a success log and a failure log. The logs are saved in comma-separated value (CSV) format, and the name of each log includes the ID of the job or job part that the log describes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct JobLogs {
    /// <p>A link to an Amazon S3 presigned URL where the job completion report is located.</p>
    #[serde(rename = "JobCompletionReportURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_completion_report_uri: Option<String>,
    /// <p>A link to an Amazon S3 presigned URL where the job failure log is located.</p>
    #[serde(rename = "JobFailureLogURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_failure_log_uri: Option<String>,
    /// <p>A link to an Amazon S3 presigned URL where the job success log is located.</p>
    #[serde(rename = "JobSuccessLogURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_success_log_uri: Option<String>,
}

/// <p>Contains information about a specific job including shipping information, job status, and other important metadata. This information is returned as a part of the response syntax of the <code>DescribeJob</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct JobMetadata {
    /// <p>The ID for the address that you want the Snowball shipped to.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The 39-character ID for the cluster, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The creation date for this job.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A value that defines the real-time status of a Snowball's data transfer while the appliance is at AWS. This data is only available while a job has a <code>JobState</code> value of <code>InProgress</code>, for both import and export jobs.</p>
    #[serde(rename = "DataTransferProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_progress: Option<DataTransfer>,
    /// <p>The description of the job, provided at job creation.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the address that you want a job shipped to, after it will be shipped to its primary address. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The automatically generated ID for a job, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>Links to Amazon S3 presigned URLs for the job report and logs. For import jobs, the PDF job report becomes available at the end of the import process. For export jobs, your job report typically becomes available while the Snowball for your job part is being delivered to you.</p>
    #[serde(rename = "JobLogInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_log_info: Option<JobLogs>,
    /// <p>The current status of the jobs.</p>
    #[serde(rename = "JobState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_state: Option<String>,
    /// <p>The type of job.</p>
    #[serde(rename = "JobType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the AWS Key Management Service (AWS KMS) key associated with this job. This ARN was created using the <a href="http://docs.aws.amazon.com/kms/latest/APIReference/API_CreateKey.html">CreateKey</a> API action in AWS KMS.</p>
    #[serde(rename = "KmsKeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The Amazon Simple Notification Service (Amazon SNS) notification settings associated with a specific job. The <code>Notification</code> object is returned as a part of the response syntax of the <code>DescribeJob</code> action in the <code>JobMetadata</code> data type.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>An array of <code>S3Resource</code> objects. Each <code>S3Resource</code> object represents an Amazon S3 bucket that your transferred data will be exported from or imported into.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The role ARN associated with this job. This ARN was created using the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in AWS Identity and Access Management (IAM).</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A job's shipping information, including inbound and outbound tracking numbers and shipping speed options.</p>
    #[serde(rename = "ShippingDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<ShippingDetails>,
    /// <p>The Snowball capacity preference for this job, specified at job creation. In US regions, you can choose between 50 TB and 80 TB Snowballs. All other regions use 80 TB capacity Snowballs.</p>
    #[serde(rename = "SnowballCapacityPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_capacity_preference: Option<String>,
    /// <p>The type of appliance used with this job.</p>
    #[serde(rename = "SnowballType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_type: Option<String>,
}

/// <p>Contains an array of <code>S3Resource</code> objects. Each <code>S3Resource</code> object represents an Amazon S3 bucket that your transferred data will be exported from or imported into.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobResource {
    /// <p>The Python-language Lambda functions for this job.</p>
    #[serde(rename = "LambdaResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_resources: Option<Vec<LambdaResource>>,
    /// <p>An array of <code>S3Resource</code> objects.</p>
    #[serde(rename = "S3Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_resources: Option<Vec<S3Resource>>,
}

/// <p>Contains a key range. For export jobs, a <code>S3Resource</code> object can have an optional <code>KeyRange</code> value. The length of the range is defined at job creation, and has either an inclusive <code>BeginMarker</code>, an inclusive <code>EndMarker</code>, or both. Ranges are UTF-8 binary sorted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyRange {
    /// <p>The key that starts an optional key range for an export job. Ranges are inclusive and UTF-8 binary sorted.</p>
    #[serde(rename = "BeginMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_marker: Option<String>,
    /// <p>The key that ends an optional key range for an export job. Ranges are inclusive and UTF-8 binary sorted.</p>
    #[serde(rename = "EndMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_marker: Option<String>,
}

/// <p>Identifies </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaResource {
    /// <p>The array of ARNs for <a>S3Resource</a> objects to trigger the <a>LambdaResource</a> objects associated with this job.</p>
    #[serde(rename = "EventTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_triggers: Option<Vec<EventTriggerDefinition>>,
    /// <p>An Amazon Resource Name (ARN) that represents an AWS Lambda function to be triggered by PUT object actions on the associated local Amazon S3 resource.</p>
    #[serde(rename = "LambdaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListClusterJobsRequest {
    /// <p>The 39-character ID for the cluster that you want to list, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The number of <code>JobListEntry</code> objects to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>JobListEntry</code> objects, you have the option of specifying <code>NextToken</code> as the starting point for your returned list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListClusterJobsResult {
    /// <p>Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. </p>
    #[serde(rename = "JobListEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_list_entries: Option<Vec<JobListEntry>>,
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>ListClusterJobsResult</code> call, your list of returned jobs will start from this point in the array.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListClustersRequest {
    /// <p>The number of <code>ClusterListEntry</code> objects to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>ClusterListEntry</code> objects, you have the option of specifying <code>NextToken</code> as the starting point for your returned list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListClustersResult {
    /// <p>Each <code>ClusterListEntry</code> object contains a cluster's state, a cluster's ID, and other important status information.</p>
    #[serde(rename = "ClusterListEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_list_entries: Option<Vec<ClusterListEntry>>,
    /// <p>HTTP requests are stateless. If you use the automatically generated <code>NextToken</code> value in your next <code>ClusterListEntry</code> call, your list of returned clusters will start from this point in the array.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobsRequest {
    /// <p>The number of <code>JobListEntry</code> objects to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>HTTP requests are stateless. To identify what object comes "next" in the list of <code>JobListEntry</code> objects, you have the option of specifying <code>NextToken</code> as the starting point for your returned list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListJobsResult {
    /// <p>Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. </p>
    #[serde(rename = "JobListEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_list_entries: Option<Vec<JobListEntry>>,
    /// <p>HTTP requests are stateless. If you use this automatically generated <code>NextToken</code> value in your next <code>ListJobs</code> call, your returned <code>JobListEntry</code> objects will start from this point in the array.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The Amazon Simple Notification Service (Amazon SNS) notification settings associated with a specific job. The <code>Notification</code> object is returned as a part of the response syntax of the <code>DescribeJob</code> action in the <code>JobMetadata</code> data type.</p> <p>When the notification settings are defined during job creation, you can choose to notify based on a specific set of job states using the <code>JobStatesToNotify</code> array of strings, or you can specify that you want to have Amazon SNS notifications sent out for all job states with <code>NotifyAll</code> set to true.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    /// <p>The list of job states that will trigger a notification for this job.</p>
    #[serde(rename = "JobStatesToNotify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_states_to_notify: Option<Vec<String>>,
    /// <p>Any change in job state will trigger a notification for this job.</p>
    #[serde(rename = "NotifyAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_all: Option<bool>,
    /// <p>The new SNS <code>TopicArn</code> that you want to associate with this job. You can create Amazon Resource Names (ARNs) for topics by using the <a href="http://docs.aws.amazon.com/sns/latest/api/API_CreateTopic.html">CreateTopic</a> Amazon SNS API action.</p> <p>You can subscribe email addresses to an Amazon SNS topic through the AWS Management Console, or by using the <a href="http://docs.aws.amazon.com/sns/latest/api/API_Subscribe.html">Subscribe</a> AWS Simple Notification Service (SNS) API action.</p>
    #[serde(rename = "SnsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>Each <code>S3Resource</code> object represents an Amazon S3 bucket that your transferred data will be exported from or imported into. For export jobs, this object can have an optional <code>KeyRange</code> value. The length of the range is defined at job creation, and has either an inclusive <code>BeginMarker</code>, an inclusive <code>EndMarker</code>, or both. Ranges are UTF-8 binary sorted.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Resource {
    /// <p>The Amazon Resource Name (ARN) of an Amazon S3 bucket.</p>
    #[serde(rename = "BucketArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    /// <p>For export jobs, you can provide an optional <code>KeyRange</code> within a specific Amazon S3 bucket. The length of the range is defined at job creation, and has either an inclusive <code>BeginMarker</code>, an inclusive <code>EndMarker</code>, or both. Ranges are UTF-8 binary sorted.</p>
    #[serde(rename = "KeyRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_range: Option<KeyRange>,
}

/// <p>The <code>Status</code> and <code>TrackingNumber</code> information for an inbound or outbound shipment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Shipment {
    /// <p>Status information for a shipment.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tracking number for this job. Using this tracking number with your region's carrier's website, you can track a Snowball as the carrier transports it.</p> <p>For India, the carrier is Amazon Logistics. For all other regions, UPS is the carrier.</p>
    #[serde(rename = "TrackingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// <p>A job's shipping information, including inbound and outbound tracking numbers and shipping speed options.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ShippingDetails {
    /// <p>The <code>Status</code> and <code>TrackingNumber</code> values for a Snowball being returned to AWS for a particular job.</p>
    #[serde(rename = "InboundShipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_shipment: Option<Shipment>,
    /// <p>The <code>Status</code> and <code>TrackingNumber</code> values for a Snowball being delivered to the address that you specified for a particular job.</p>
    #[serde(rename = "OutboundShipment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_shipment: Option<Shipment>,
    /// <p><p>The shipping speed for a particular job. This speed doesn&#39;t dictate how soon you&#39;ll get the Snowball from the job&#39;s creation date. This speed represents how quickly it moves to its destination while in transit. Regional shipping speeds are as follows:</p> <ul> <li> <p>In Australia, you have access to express shipping. Typically, Snowballs shipped express are delivered in about a day.</p> </li> <li> <p>In the European Union (EU), you have access to express shipping. Typically, Snowballs shipped express are delivered in about a day. In addition, most countries in the EU have access to standard shipping, which typically takes less than a week, one way.</p> </li> <li> <p>In India, Snowballs are delivered in one to seven days.</p> </li> <li> <p>In the United States of America (US), you have access to one-day shipping and two-day shipping.</p> </li> </ul></p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateClusterRequest {
    /// <p>The ID of the updated <a>Address</a> object.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The cluster ID of the cluster that you want to update, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The updated description of this cluster.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated ID for the forwarding address for a cluster. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The new or updated <a>Notification</a> object.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>The updated arrays of <a>JobResource</a> objects that can include updated <a>S3Resource</a> objects or <a>LambdaResource</a> objects.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The new role Amazon Resource Name (ARN) that you want to associate with this cluster. To create a role ARN, use the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a> API action in AWS Identity and Access Management (IAM).</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The updated shipping option value of this cluster's <a>ShippingDetails</a> object.</p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateClusterResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateJobRequest {
    /// <p>The ID of the updated <a>Address</a> object.</p>
    #[serde(rename = "AddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// <p>The updated description of this job's <a>JobMetadata</a> object.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The updated ID for the forwarding address for a job. This field is not supported in most regions.</p>
    #[serde(rename = "ForwardingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarding_address_id: Option<String>,
    /// <p>The job ID of the job that you want to update, for example <code>JID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>The new or updated <a>Notification</a> object.</p>
    #[serde(rename = "Notification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// <p>The updated <a>S3Resource</a> object (for a single Amazon S3 bucket or key range), or the updated <a>JobResource</a> object (for multiple buckets or key ranges). </p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobResource>,
    /// <p>The new role Amazon Resource Name (ARN) that you want to associate with this job. To create a role ARN, use the <a href="http://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateRole.html">CreateRole</a>AWS Identity and Access Management (IAM) API action.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The updated shipping option value of this job's <a>ShippingDetails</a> object.</p>
    #[serde(rename = "ShippingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option: Option<String>,
    /// <p>The updated <code>SnowballCapacityPreference</code> of this job's <a>JobMetadata</a> object. The 50 TB Snowballs are only available in the US regions.</p>
    #[serde(rename = "SnowballCapacityPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowball_capacity_preference: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateJobResult {}

/// Errors returned by CancelCluster
#[derive(Debug, PartialEq)]
pub enum CancelClusterError {
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelClusterError {
    pub fn from_body(body: &str) -> CancelClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidJobStateException" => {
                        CancelClusterError::InvalidJobState(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        CancelClusterError::InvalidResource(String::from(error_message))
                    }
                    "KMSRequestFailedException" => {
                        CancelClusterError::KMSRequestFailed(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelClusterError::Validation(error_message.to_string())
                    }
                    _ => CancelClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelClusterError {
    fn from(err: serde_json::error::Error) -> CancelClusterError {
        CancelClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelClusterError {
    fn from(err: CredentialsError) -> CancelClusterError {
        CancelClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelClusterError {
    fn from(err: HttpDispatchError) -> CancelClusterError {
        CancelClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelClusterError {
    fn from(err: io::Error) -> CancelClusterError {
        CancelClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelClusterError {
    fn description(&self) -> &str {
        match *self {
            CancelClusterError::InvalidJobState(ref cause) => cause,
            CancelClusterError::InvalidResource(ref cause) => cause,
            CancelClusterError::KMSRequestFailed(ref cause) => cause,
            CancelClusterError::Validation(ref cause) => cause,
            CancelClusterError::Credentials(ref err) => err.description(),
            CancelClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelJobError {
    pub fn from_body(body: &str) -> CancelJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidJobStateException" => {
                        CancelJobError::InvalidJobState(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        CancelJobError::InvalidResource(String::from(error_message))
                    }
                    "KMSRequestFailedException" => {
                        CancelJobError::KMSRequestFailed(String::from(error_message))
                    }
                    "ValidationException" => CancelJobError::Validation(error_message.to_string()),
                    _ => CancelJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelJobError {
    fn from(err: serde_json::error::Error) -> CancelJobError {
        CancelJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelJobError {
    fn from(err: CredentialsError) -> CancelJobError {
        CancelJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelJobError {
    fn from(err: HttpDispatchError) -> CancelJobError {
        CancelJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelJobError {
    fn from(err: io::Error) -> CancelJobError {
        CancelJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelJobError {
    fn description(&self) -> &str {
        match *self {
            CancelJobError::InvalidJobState(ref cause) => cause,
            CancelJobError::InvalidResource(ref cause) => cause,
            CancelJobError::KMSRequestFailed(ref cause) => cause,
            CancelJobError::Validation(ref cause) => cause,
            CancelJobError::Credentials(ref err) => err.description(),
            CancelJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAddress
#[derive(Debug, PartialEq)]
pub enum CreateAddressError {
    /// <p>The address provided was invalid. Check the address with your region's carrier, and try again.</p>
    InvalidAddress(String),
    /// <p>The address is either outside the serviceable area for your region, or an error occurred. Check the address with your region's carrier and try again. If the issue persists, contact AWS Support.</p>
    UnsupportedAddress(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAddressError {
    pub fn from_body(body: &str) -> CreateAddressError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidAddressException" => {
                        CreateAddressError::InvalidAddress(String::from(error_message))
                    }
                    "UnsupportedAddressException" => {
                        CreateAddressError::UnsupportedAddress(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAddressError::Validation(error_message.to_string())
                    }
                    _ => CreateAddressError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAddressError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAddressError {
    fn from(err: serde_json::error::Error) -> CreateAddressError {
        CreateAddressError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAddressError {
    fn from(err: CredentialsError) -> CreateAddressError {
        CreateAddressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAddressError {
    fn from(err: HttpDispatchError) -> CreateAddressError {
        CreateAddressError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAddressError {
    fn from(err: io::Error) -> CreateAddressError {
        CreateAddressError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAddressError {
    fn description(&self) -> &str {
        match *self {
            CreateAddressError::InvalidAddress(ref cause) => cause,
            CreateAddressError::UnsupportedAddress(ref cause) => cause,
            CreateAddressError::Validation(ref cause) => cause,
            CreateAddressError::Credentials(ref err) => err.description(),
            CreateAddressError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAddressError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>Job or cluster creation failed. One ore more inputs were invalid. Confirm that the <a>CreateClusterRequest$SnowballType</a> value supports your <a>CreateJobRequest$JobType</a>, and try again.</p>
    InvalidInputCombination(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateClusterError {
    pub fn from_body(body: &str) -> CreateClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputCombinationException" => {
                        CreateClusterError::InvalidInputCombination(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        CreateClusterError::InvalidResource(String::from(error_message))
                    }
                    "KMSRequestFailedException" => {
                        CreateClusterError::KMSRequestFailed(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateClusterError::Validation(error_message.to_string())
                    }
                    _ => CreateClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateClusterError {
    fn from(err: serde_json::error::Error) -> CreateClusterError {
        CreateClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateClusterError {
    fn from(err: CredentialsError) -> CreateClusterError {
        CreateClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateClusterError {
    fn from(err: HttpDispatchError) -> CreateClusterError {
        CreateClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateClusterError {
    fn from(err: io::Error) -> CreateClusterError {
        CreateClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterError::InvalidInputCombination(ref cause) => cause,
            CreateClusterError::InvalidResource(ref cause) => cause,
            CreateClusterError::KMSRequestFailed(ref cause) => cause,
            CreateClusterError::Validation(ref cause) => cause,
            CreateClusterError::Credentials(ref err) => err.description(),
            CreateClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>Job creation failed. Currently, clusters support five nodes. If you have less than five nodes for your cluster and you have more nodes to create for this cluster, try again and create jobs until your cluster has exactly five notes.</p>
    ClusterLimitExceeded(String),
    /// <p>Job or cluster creation failed. One ore more inputs were invalid. Confirm that the <a>CreateClusterRequest$SnowballType</a> value supports your <a>CreateJobRequest$JobType</a>, and try again.</p>
    InvalidInputCombination(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateJobError {
    pub fn from_body(body: &str) -> CreateJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClusterLimitExceededException" => {
                        CreateJobError::ClusterLimitExceeded(String::from(error_message))
                    }
                    "InvalidInputCombinationException" => {
                        CreateJobError::InvalidInputCombination(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        CreateJobError::InvalidResource(String::from(error_message))
                    }
                    "KMSRequestFailedException" => {
                        CreateJobError::KMSRequestFailed(String::from(error_message))
                    }
                    "ValidationException" => CreateJobError::Validation(error_message.to_string()),
                    _ => CreateJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateJobError {
    fn from(err: serde_json::error::Error) -> CreateJobError {
        CreateJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateJobError {
    fn from(err: CredentialsError) -> CreateJobError {
        CreateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateJobError {
    fn from(err: HttpDispatchError) -> CreateJobError {
        CreateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateJobError {
    fn from(err: io::Error) -> CreateJobError {
        CreateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobError {
    fn description(&self) -> &str {
        match *self {
            CreateJobError::ClusterLimitExceeded(ref cause) => cause,
            CreateJobError::InvalidInputCombination(ref cause) => cause,
            CreateJobError::InvalidResource(ref cause) => cause,
            CreateJobError::KMSRequestFailed(ref cause) => cause,
            CreateJobError::Validation(ref cause) => cause,
            CreateJobError::Credentials(ref err) => err.description(),
            CreateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAddress
#[derive(Debug, PartialEq)]
pub enum DescribeAddressError {
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAddressError {
    pub fn from_body(body: &str) -> DescribeAddressError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceException" => {
                        DescribeAddressError::InvalidResource(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAddressError::Validation(error_message.to_string())
                    }
                    _ => DescribeAddressError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAddressError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAddressError {
    fn from(err: serde_json::error::Error) -> DescribeAddressError {
        DescribeAddressError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAddressError {
    fn from(err: CredentialsError) -> DescribeAddressError {
        DescribeAddressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAddressError {
    fn from(err: HttpDispatchError) -> DescribeAddressError {
        DescribeAddressError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAddressError {
    fn from(err: io::Error) -> DescribeAddressError {
        DescribeAddressError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAddressError {
    fn description(&self) -> &str {
        match *self {
            DescribeAddressError::InvalidResource(ref cause) => cause,
            DescribeAddressError::Validation(ref cause) => cause,
            DescribeAddressError::Credentials(ref err) => err.description(),
            DescribeAddressError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeAddressError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAddresses
#[derive(Debug, PartialEq)]
pub enum DescribeAddressesError {
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAddressesError {
    pub fn from_body(body: &str) -> DescribeAddressesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        DescribeAddressesError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        DescribeAddressesError::InvalidResource(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAddressesError::Validation(error_message.to_string())
                    }
                    _ => DescribeAddressesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAddressesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAddressesError {
    fn from(err: serde_json::error::Error) -> DescribeAddressesError {
        DescribeAddressesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAddressesError {
    fn from(err: CredentialsError) -> DescribeAddressesError {
        DescribeAddressesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAddressesError {
    fn from(err: HttpDispatchError) -> DescribeAddressesError {
        DescribeAddressesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAddressesError {
    fn from(err: io::Error) -> DescribeAddressesError {
        DescribeAddressesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAddressesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAddressesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAddressesError::InvalidNextToken(ref cause) => cause,
            DescribeAddressesError::InvalidResource(ref cause) => cause,
            DescribeAddressesError::Validation(ref cause) => cause,
            DescribeAddressesError::Credentials(ref err) => err.description(),
            DescribeAddressesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAddressesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCluster
#[derive(Debug, PartialEq)]
pub enum DescribeClusterError {
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClusterError {
    pub fn from_body(body: &str) -> DescribeClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceException" => {
                        DescribeClusterError::InvalidResource(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeClusterError::Validation(error_message.to_string())
                    }
                    _ => DescribeClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeClusterError {
    fn from(err: serde_json::error::Error) -> DescribeClusterError {
        DescribeClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeClusterError {
    fn from(err: CredentialsError) -> DescribeClusterError {
        DescribeClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClusterError {
    fn from(err: HttpDispatchError) -> DescribeClusterError {
        DescribeClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClusterError {
    fn from(err: io::Error) -> DescribeClusterError {
        DescribeClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClusterError {
    fn description(&self) -> &str {
        match *self {
            DescribeClusterError::InvalidResource(ref cause) => cause,
            DescribeClusterError::Validation(ref cause) => cause,
            DescribeClusterError::Credentials(ref err) => err.description(),
            DescribeClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJob
#[derive(Debug, PartialEq)]
pub enum DescribeJobError {
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeJobError {
    pub fn from_body(body: &str) -> DescribeJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceException" => {
                        DescribeJobError::InvalidResource(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeJobError::Validation(error_message.to_string())
                    }
                    _ => DescribeJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeJobError {
    fn from(err: serde_json::error::Error) -> DescribeJobError {
        DescribeJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobError {
    fn from(err: CredentialsError) -> DescribeJobError {
        DescribeJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobError {
    fn from(err: HttpDispatchError) -> DescribeJobError {
        DescribeJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeJobError {
    fn from(err: io::Error) -> DescribeJobError {
        DescribeJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobError::InvalidResource(ref cause) => cause,
            DescribeJobError::Validation(ref cause) => cause,
            DescribeJobError::Credentials(ref err) => err.description(),
            DescribeJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobManifest
#[derive(Debug, PartialEq)]
pub enum GetJobManifestError {
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetJobManifestError {
    pub fn from_body(body: &str) -> GetJobManifestError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidJobStateException" => {
                        GetJobManifestError::InvalidJobState(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        GetJobManifestError::InvalidResource(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetJobManifestError::Validation(error_message.to_string())
                    }
                    _ => GetJobManifestError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobManifestError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobManifestError {
    fn from(err: serde_json::error::Error) -> GetJobManifestError {
        GetJobManifestError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobManifestError {
    fn from(err: CredentialsError) -> GetJobManifestError {
        GetJobManifestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobManifestError {
    fn from(err: HttpDispatchError) -> GetJobManifestError {
        GetJobManifestError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobManifestError {
    fn from(err: io::Error) -> GetJobManifestError {
        GetJobManifestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobManifestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobManifestError {
    fn description(&self) -> &str {
        match *self {
            GetJobManifestError::InvalidJobState(ref cause) => cause,
            GetJobManifestError::InvalidResource(ref cause) => cause,
            GetJobManifestError::Validation(ref cause) => cause,
            GetJobManifestError::Credentials(ref err) => err.description(),
            GetJobManifestError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobManifestError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobUnlockCode
#[derive(Debug, PartialEq)]
pub enum GetJobUnlockCodeError {
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetJobUnlockCodeError {
    pub fn from_body(body: &str) -> GetJobUnlockCodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidJobStateException" => {
                        GetJobUnlockCodeError::InvalidJobState(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        GetJobUnlockCodeError::InvalidResource(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetJobUnlockCodeError::Validation(error_message.to_string())
                    }
                    _ => GetJobUnlockCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobUnlockCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobUnlockCodeError {
    fn from(err: serde_json::error::Error) -> GetJobUnlockCodeError {
        GetJobUnlockCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobUnlockCodeError {
    fn from(err: CredentialsError) -> GetJobUnlockCodeError {
        GetJobUnlockCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobUnlockCodeError {
    fn from(err: HttpDispatchError) -> GetJobUnlockCodeError {
        GetJobUnlockCodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobUnlockCodeError {
    fn from(err: io::Error) -> GetJobUnlockCodeError {
        GetJobUnlockCodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobUnlockCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobUnlockCodeError {
    fn description(&self) -> &str {
        match *self {
            GetJobUnlockCodeError::InvalidJobState(ref cause) => cause,
            GetJobUnlockCodeError::InvalidResource(ref cause) => cause,
            GetJobUnlockCodeError::Validation(ref cause) => cause,
            GetJobUnlockCodeError::Credentials(ref err) => err.description(),
            GetJobUnlockCodeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobUnlockCodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSnowballUsage
#[derive(Debug, PartialEq)]
pub enum GetSnowballUsageError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSnowballUsageError {
    pub fn from_body(body: &str) -> GetSnowballUsageError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        GetSnowballUsageError::Validation(error_message.to_string())
                    }
                    _ => GetSnowballUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSnowballUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSnowballUsageError {
    fn from(err: serde_json::error::Error) -> GetSnowballUsageError {
        GetSnowballUsageError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSnowballUsageError {
    fn from(err: CredentialsError) -> GetSnowballUsageError {
        GetSnowballUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSnowballUsageError {
    fn from(err: HttpDispatchError) -> GetSnowballUsageError {
        GetSnowballUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSnowballUsageError {
    fn from(err: io::Error) -> GetSnowballUsageError {
        GetSnowballUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSnowballUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSnowballUsageError {
    fn description(&self) -> &str {
        match *self {
            GetSnowballUsageError::Validation(ref cause) => cause,
            GetSnowballUsageError::Credentials(ref err) => err.description(),
            GetSnowballUsageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSnowballUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListClusterJobs
#[derive(Debug, PartialEq)]
pub enum ListClusterJobsError {
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListClusterJobsError {
    pub fn from_body(body: &str) -> ListClusterJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        ListClusterJobsError::InvalidNextToken(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        ListClusterJobsError::InvalidResource(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListClusterJobsError::Validation(error_message.to_string())
                    }
                    _ => ListClusterJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListClusterJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListClusterJobsError {
    fn from(err: serde_json::error::Error) -> ListClusterJobsError {
        ListClusterJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListClusterJobsError {
    fn from(err: CredentialsError) -> ListClusterJobsError {
        ListClusterJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListClusterJobsError {
    fn from(err: HttpDispatchError) -> ListClusterJobsError {
        ListClusterJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListClusterJobsError {
    fn from(err: io::Error) -> ListClusterJobsError {
        ListClusterJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListClusterJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClusterJobsError {
    fn description(&self) -> &str {
        match *self {
            ListClusterJobsError::InvalidNextToken(ref cause) => cause,
            ListClusterJobsError::InvalidResource(ref cause) => cause,
            ListClusterJobsError::Validation(ref cause) => cause,
            ListClusterJobsError::Credentials(ref err) => err.description(),
            ListClusterJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListClusterJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListClustersError {
    pub fn from_body(body: &str) -> ListClustersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        ListClustersError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListClustersError::Validation(error_message.to_string())
                    }
                    _ => ListClustersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListClustersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListClustersError {
    fn from(err: serde_json::error::Error) -> ListClustersError {
        ListClustersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListClustersError {
    fn from(err: CredentialsError) -> ListClustersError {
        ListClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListClustersError {
    fn from(err: HttpDispatchError) -> ListClustersError {
        ListClustersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListClustersError {
    fn from(err: io::Error) -> ListClustersError {
        ListClustersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClustersError {
    fn description(&self) -> &str {
        match *self {
            ListClustersError::InvalidNextToken(ref cause) => cause,
            ListClustersError::Validation(ref cause) => cause,
            ListClustersError::Credentials(ref err) => err.description(),
            ListClustersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextToken(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListJobsError {
    pub fn from_body(body: &str) -> ListJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidNextTokenException" => {
                        ListJobsError::InvalidNextToken(String::from(error_message))
                    }
                    "ValidationException" => ListJobsError::Validation(error_message.to_string()),
                    _ => ListJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListJobsError {
    fn from(err: serde_json::error::Error) -> ListJobsError {
        ListJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobsError {
    fn from(err: io::Error) -> ListJobsError {
        ListJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::InvalidNextToken(ref cause) => cause,
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCluster
#[derive(Debug, PartialEq)]
pub enum UpdateClusterError {
    /// <p>Job or cluster creation failed. One ore more inputs were invalid. Confirm that the <a>CreateClusterRequest$SnowballType</a> value supports your <a>CreateJobRequest$JobType</a>, and try again.</p>
    InvalidInputCombination(String),
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateClusterError {
    pub fn from_body(body: &str) -> UpdateClusterError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputCombinationException" => {
                        UpdateClusterError::InvalidInputCombination(String::from(error_message))
                    }
                    "InvalidJobStateException" => {
                        UpdateClusterError::InvalidJobState(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        UpdateClusterError::InvalidResource(String::from(error_message))
                    }
                    "KMSRequestFailedException" => {
                        UpdateClusterError::KMSRequestFailed(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateClusterError::Validation(error_message.to_string())
                    }
                    _ => UpdateClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateClusterError {
    fn from(err: serde_json::error::Error) -> UpdateClusterError {
        UpdateClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateClusterError {
    fn from(err: CredentialsError) -> UpdateClusterError {
        UpdateClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateClusterError {
    fn from(err: HttpDispatchError) -> UpdateClusterError {
        UpdateClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateClusterError {
    fn from(err: io::Error) -> UpdateClusterError {
        UpdateClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClusterError {
    fn description(&self) -> &str {
        match *self {
            UpdateClusterError::InvalidInputCombination(ref cause) => cause,
            UpdateClusterError::InvalidJobState(ref cause) => cause,
            UpdateClusterError::InvalidResource(ref cause) => cause,
            UpdateClusterError::KMSRequestFailed(ref cause) => cause,
            UpdateClusterError::Validation(ref cause) => cause,
            UpdateClusterError::Credentials(ref err) => err.description(),
            UpdateClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateJob
#[derive(Debug, PartialEq)]
pub enum UpdateJobError {
    /// <p>Job creation failed. Currently, clusters support five nodes. If you have less than five nodes for your cluster and you have more nodes to create for this cluster, try again and create jobs until your cluster has exactly five notes.</p>
    ClusterLimitExceeded(String),
    /// <p>Job or cluster creation failed. One ore more inputs were invalid. Confirm that the <a>CreateClusterRequest$SnowballType</a> value supports your <a>CreateJobRequest$JobType</a>, and try again.</p>
    InvalidInputCombination(String),
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobState(String),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResource(String),
    /// <p>The provided AWS Key Management Service key lacks the permissions to perform the specified <a>CreateJob</a> or <a>UpdateJob</a> action.</p>
    KMSRequestFailed(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateJobError {
    pub fn from_body(body: &str) -> UpdateJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ClusterLimitExceededException" => {
                        UpdateJobError::ClusterLimitExceeded(String::from(error_message))
                    }
                    "InvalidInputCombinationException" => {
                        UpdateJobError::InvalidInputCombination(String::from(error_message))
                    }
                    "InvalidJobStateException" => {
                        UpdateJobError::InvalidJobState(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        UpdateJobError::InvalidResource(String::from(error_message))
                    }
                    "KMSRequestFailedException" => {
                        UpdateJobError::KMSRequestFailed(String::from(error_message))
                    }
                    "ValidationException" => UpdateJobError::Validation(error_message.to_string()),
                    _ => UpdateJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateJobError {
    fn from(err: serde_json::error::Error) -> UpdateJobError {
        UpdateJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateJobError {
    fn from(err: CredentialsError) -> UpdateJobError {
        UpdateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateJobError {
    fn from(err: HttpDispatchError) -> UpdateJobError {
        UpdateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateJobError {
    fn from(err: io::Error) -> UpdateJobError {
        UpdateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateJobError {
    fn description(&self) -> &str {
        match *self {
            UpdateJobError::ClusterLimitExceeded(ref cause) => cause,
            UpdateJobError::InvalidInputCombination(ref cause) => cause,
            UpdateJobError::InvalidJobState(ref cause) => cause,
            UpdateJobError::InvalidResource(ref cause) => cause,
            UpdateJobError::KMSRequestFailed(ref cause) => cause,
            UpdateJobError::Validation(ref cause) => cause,
            UpdateJobError::Credentials(ref err) => err.description(),
            UpdateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Snowball API. Amazon Snowball clients implement this trait.
pub trait Snowball {
    /// <p>Cancels a cluster job. You can only cancel a cluster job while it's in the <code>AwaitingQuorum</code> status. You'll have at least an hour after creating a cluster job to cancel it.</p>
    fn cancel_cluster(
        &self,
        input: CancelClusterRequest,
    ) -> RusotoFuture<CancelClusterResult, CancelClusterError>;

    /// <p>Cancels the specified job. You can only cancel a job before its <code>JobState</code> value changes to <code>PreparingAppliance</code>. Requesting the <code>ListJobs</code> or <code>DescribeJob</code> action will return a job's <code>JobState</code> as part of the response element data returned.</p>
    fn cancel_job(&self, input: CancelJobRequest) -> RusotoFuture<CancelJobResult, CancelJobError>;

    /// <p>Creates an address for a Snowball to be shipped to. In most regions, addresses are validated at the time of creation. The address you provide must be located within the serviceable area of your region. If the address is invalid or unsupported, then an exception is thrown.</p>
    fn create_address(
        &self,
        input: CreateAddressRequest,
    ) -> RusotoFuture<CreateAddressResult, CreateAddressError>;

    /// <p>Creates an empty cluster. Each cluster supports five nodes. You use the <a>CreateJob</a> action separately to create the jobs for each of these nodes. The cluster does not ship until these five node jobs have been created.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResult, CreateClusterError>;

    /// <p>Creates a job to import or export data between Amazon S3 and your on-premises data center. Your AWS account must have the right trust policies and permissions in place to create a job for Snowball. If you're creating a job for a node in a cluster, you only need to provide the <code>clusterId</code> value; the other job attributes are inherited from the cluster. </p>
    fn create_job(&self, input: CreateJobRequest) -> RusotoFuture<CreateJobResult, CreateJobError>;

    /// <p>Takes an <code>AddressId</code> and returns specific details about that address in the form of an <code>Address</code> object.</p>
    fn describe_address(
        &self,
        input: DescribeAddressRequest,
    ) -> RusotoFuture<DescribeAddressResult, DescribeAddressError>;

    /// <p>Returns a specified number of <code>ADDRESS</code> objects. Calling this API in one of the US regions will return addresses from the list of all addresses associated with this account in all US regions.</p>
    fn describe_addresses(
        &self,
        input: DescribeAddressesRequest,
    ) -> RusotoFuture<DescribeAddressesResult, DescribeAddressesError>;

    /// <p>Returns information about a specific cluster including shipping information, cluster status, and other important metadata.</p>
    fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> RusotoFuture<DescribeClusterResult, DescribeClusterError>;

    /// <p>Returns information about a specific job including shipping information, job status, and other important metadata. </p>
    fn describe_job(
        &self,
        input: DescribeJobRequest,
    ) -> RusotoFuture<DescribeJobResult, DescribeJobError>;

    /// <p>Returns a link to an Amazon S3 presigned URL for the manifest file associated with the specified <code>JobId</code> value. You can access the manifest file for up to 60 minutes after this request has been made. To access the manifest file after 60 minutes have passed, you'll have to make another call to the <code>GetJobManifest</code> action.</p> <p>The manifest is an encrypted file that you can download after your job enters the <code>WithCustomer</code> status. The manifest is decrypted by using the <code>UnlockCode</code> code value, when you pass both values to the Snowball through the Snowball client when the client is started for the first time.</p> <p>As a best practice, we recommend that you don't save a copy of an <code>UnlockCode</code> value in the same location as the manifest file for that job. Saving these separately helps prevent unauthorized parties from gaining access to the Snowball associated with that job.</p> <p>The credentials of a given job, including its manifest file and unlock code, expire 90 days after the job is created.</p>
    fn get_job_manifest(
        &self,
        input: GetJobManifestRequest,
    ) -> RusotoFuture<GetJobManifestResult, GetJobManifestError>;

    /// <p>Returns the <code>UnlockCode</code> code value for the specified job. A particular <code>UnlockCode</code> value can be accessed for up to 90 days after the associated job has been created.</p> <p>The <code>UnlockCode</code> value is a 29-character code with 25 alphanumeric characters and 4 hyphens. This code is used to decrypt the manifest file when it is passed along with the manifest to the Snowball through the Snowball client when the client is started for the first time.</p> <p>As a best practice, we recommend that you don't save a copy of the <code>UnlockCode</code> in the same location as the manifest file for that job. Saving these separately helps prevent unauthorized parties from gaining access to the Snowball associated with that job.</p>
    fn get_job_unlock_code(
        &self,
        input: GetJobUnlockCodeRequest,
    ) -> RusotoFuture<GetJobUnlockCodeResult, GetJobUnlockCodeError>;

    /// <p>Returns information about the Snowball service limit for your account, and also the number of Snowballs your account has in use.</p> <p>The default service limit for the number of Snowballs that you can have at one time is 1. If you want to increase your service limit, contact AWS Support.</p>
    fn get_snowball_usage(&self) -> RusotoFuture<GetSnowballUsageResult, GetSnowballUsageError>;

    /// <p>Returns an array of <code>JobListEntry</code> objects of the specified length. Each <code>JobListEntry</code> object is for a job in the specified cluster and contains a job's state, a job's ID, and other information.</p>
    fn list_cluster_jobs(
        &self,
        input: ListClusterJobsRequest,
    ) -> RusotoFuture<ListClusterJobsResult, ListClusterJobsError>;

    /// <p>Returns an array of <code>ClusterListEntry</code> objects of the specified length. Each <code>ClusterListEntry</code> object contains a cluster's state, a cluster's ID, and other important status information.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResult, ListClustersError>;

    /// <p>Returns an array of <code>JobListEntry</code> objects of the specified length. Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. Calling this API action in one of the US regions will return jobs from the list of all jobs associated with this account in all US regions.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResult, ListJobsError>;

    /// <p>While a cluster's <code>ClusterState</code> value is in the <code>AwaitingQuorum</code> state, you can update some of the information associated with a cluster. Once the cluster changes to a different job state, usually 60 minutes after the cluster being created, this action is no longer available.</p>
    fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> RusotoFuture<UpdateClusterResult, UpdateClusterError>;

    /// <p>While a job's <code>JobState</code> value is <code>New</code>, you can update some of the information associated with a job. Once the job changes to a different job state, usually within 60 minutes of the job being created, this action is no longer available.</p>
    fn update_job(&self, input: UpdateJobRequest) -> RusotoFuture<UpdateJobResult, UpdateJobError>;
}
/// A client for the Amazon Snowball API.
pub struct SnowballClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl SnowballClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> SnowballClient {
        SnowballClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> SnowballClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        SnowballClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Snowball for SnowballClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Cancels a cluster job. You can only cancel a cluster job while it's in the <code>AwaitingQuorum</code> status. You'll have at least an hour after creating a cluster job to cancel it.</p>
    fn cancel_cluster(
        &self,
        input: CancelClusterRequest,
    ) -> RusotoFuture<CancelClusterResult, CancelClusterError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CancelCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CancelClusterResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Cancels the specified job. You can only cancel a job before its <code>JobState</code> value changes to <code>PreparingAppliance</code>. Requesting the <code>ListJobs</code> or <code>DescribeJob</code> action will return a job's <code>JobState</code> as part of the response element data returned.</p>
    fn cancel_job(&self, input: CancelJobRequest) -> RusotoFuture<CancelJobResult, CancelJobError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CancelJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CancelJobResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CancelJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an address for a Snowball to be shipped to. In most regions, addresses are validated at the time of creation. The address you provide must be located within the serviceable area of your region. If the address is invalid or unsupported, then an exception is thrown.</p>
    fn create_address(
        &self,
        input: CreateAddressRequest,
    ) -> RusotoFuture<CreateAddressResult, CreateAddressError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CreateAddress",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAddressResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateAddressError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an empty cluster. Each cluster supports five nodes. You use the <a>CreateJob</a> action separately to create the jobs for each of these nodes. The cluster does not ship until these five node jobs have been created.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResult, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CreateCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateClusterResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates a job to import or export data between Amazon S3 and your on-premises data center. Your AWS account must have the right trust policies and permissions in place to create a job for Snowball. If you're creating a job for a node in a cluster, you only need to provide the <code>clusterId</code> value; the other job attributes are inherited from the cluster. </p>
    fn create_job(&self, input: CreateJobRequest) -> RusotoFuture<CreateJobResult, CreateJobError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.CreateJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateJobResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Takes an <code>AddressId</code> and returns specific details about that address in the form of an <code>Address</code> object.</p>
    fn describe_address(
        &self,
        input: DescribeAddressRequest,
    ) -> RusotoFuture<DescribeAddressResult, DescribeAddressError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.DescribeAddress",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAddressResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAddressError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a specified number of <code>ADDRESS</code> objects. Calling this API in one of the US regions will return addresses from the list of all addresses associated with this account in all US regions.</p>
    fn describe_addresses(
        &self,
        input: DescribeAddressesRequest,
    ) -> RusotoFuture<DescribeAddressesResult, DescribeAddressesError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.DescribeAddresses",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAddressesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAddressesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific cluster including shipping information, cluster status, and other important metadata.</p>
    fn describe_cluster(
        &self,
        input: DescribeClusterRequest,
    ) -> RusotoFuture<DescribeClusterResult, DescribeClusterError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.DescribeCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeClusterResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific job including shipping information, job status, and other important metadata. </p>
    fn describe_job(
        &self,
        input: DescribeJobRequest,
    ) -> RusotoFuture<DescribeJobResult, DescribeJobError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.DescribeJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeJobResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a link to an Amazon S3 presigned URL for the manifest file associated with the specified <code>JobId</code> value. You can access the manifest file for up to 60 minutes after this request has been made. To access the manifest file after 60 minutes have passed, you'll have to make another call to the <code>GetJobManifest</code> action.</p> <p>The manifest is an encrypted file that you can download after your job enters the <code>WithCustomer</code> status. The manifest is decrypted by using the <code>UnlockCode</code> code value, when you pass both values to the Snowball through the Snowball client when the client is started for the first time.</p> <p>As a best practice, we recommend that you don't save a copy of an <code>UnlockCode</code> value in the same location as the manifest file for that job. Saving these separately helps prevent unauthorized parties from gaining access to the Snowball associated with that job.</p> <p>The credentials of a given job, including its manifest file and unlock code, expire 90 days after the job is created.</p>
    fn get_job_manifest(
        &self,
        input: GetJobManifestRequest,
    ) -> RusotoFuture<GetJobManifestResult, GetJobManifestError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.GetJobManifest",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetJobManifestResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetJobManifestError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the <code>UnlockCode</code> code value for the specified job. A particular <code>UnlockCode</code> value can be accessed for up to 90 days after the associated job has been created.</p> <p>The <code>UnlockCode</code> value is a 29-character code with 25 alphanumeric characters and 4 hyphens. This code is used to decrypt the manifest file when it is passed along with the manifest to the Snowball through the Snowball client when the client is started for the first time.</p> <p>As a best practice, we recommend that you don't save a copy of the <code>UnlockCode</code> in the same location as the manifest file for that job. Saving these separately helps prevent unauthorized parties from gaining access to the Snowball associated with that job.</p>
    fn get_job_unlock_code(
        &self,
        input: GetJobUnlockCodeRequest,
    ) -> RusotoFuture<GetJobUnlockCodeResult, GetJobUnlockCodeError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.GetJobUnlockCode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetJobUnlockCodeResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetJobUnlockCodeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about the Snowball service limit for your account, and also the number of Snowballs your account has in use.</p> <p>The default service limit for the number of Snowballs that you can have at one time is 1. If you want to increase your service limit, contact AWS Support.</p>
    fn get_snowball_usage(&self) -> RusotoFuture<GetSnowballUsageResult, GetSnowballUsageError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.GetSnowballUsage",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSnowballUsageResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetSnowballUsageError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns an array of <code>JobListEntry</code> objects of the specified length. Each <code>JobListEntry</code> object is for a job in the specified cluster and contains a job's state, a job's ID, and other information.</p>
    fn list_cluster_jobs(
        &self,
        input: ListClusterJobsRequest,
    ) -> RusotoFuture<ListClusterJobsResult, ListClusterJobsError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.ListClusterJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListClusterJobsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListClusterJobsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns an array of <code>ClusterListEntry</code> objects of the specified length. Each <code>ClusterListEntry</code> object contains a cluster's state, a cluster's ID, and other important status information.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResult, ListClustersError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.ListClusters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListClustersResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListClustersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns an array of <code>JobListEntry</code> objects of the specified length. Each <code>JobListEntry</code> object contains a job's state, a job's ID, and a value that indicates whether the job is a job part, in the case of export jobs. Calling this API action in one of the US regions will return jobs from the list of all jobs associated with this account in all US regions.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResult, ListJobsError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSIESnowballJobManagementService.ListJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListJobsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListJobsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>While a cluster's <code>ClusterState</code> value is in the <code>AwaitingQuorum</code> state, you can update some of the information associated with a cluster. Once the cluster changes to a different job state, usually 60 minutes after the cluster being created, this action is no longer available.</p>
    fn update_cluster(
        &self,
        input: UpdateClusterRequest,
    ) -> RusotoFuture<UpdateClusterResult, UpdateClusterError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.UpdateCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateClusterResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>While a job's <code>JobState</code> value is <code>New</code>, you can update some of the information associated with a job. Once the job changes to a different job state, usually within 60 minutes of the job being created, this action is no longer available.</p>
    fn update_job(&self, input: UpdateJobRequest) -> RusotoFuture<UpdateJobResult, UpdateJobError> {
        let mut request = SignedRequest::new("POST", "snowball", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSIESnowballJobManagementService.UpdateJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateJobResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
