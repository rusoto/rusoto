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
/// <p>Contains information about a backup of an AWS CloudHSM cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Backup {
    /// <p>The identifier (ID) of the backup.</p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>The state of the backup.</p>
    #[serde(rename = "BackupState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_state: Option<String>,
    /// <p>The identifier (ID) of the cluster that was backed up.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The date and time when the backup was created.</p>
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
}

/// <p>Contains one or more certificates or a certificate signing request (CSR).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Certificates {
    /// <p>The HSM hardware certificate issued (signed) by AWS CloudHSM.</p>
    #[serde(rename = "AwsHardwareCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_hardware_certificate: Option<String>,
    /// <p>The cluster certificate issued (signed) by the issuing certificate authority (CA) of the cluster's owner.</p>
    #[serde(rename = "ClusterCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_certificate: Option<String>,
    /// <p>The cluster's certificate signing request (CSR). The CSR exists only when the cluster's state is <code>UNINITIALIZED</code>.</p>
    #[serde(rename = "ClusterCsr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_csr: Option<String>,
    /// <p>The HSM certificate issued (signed) by the HSM hardware.</p>
    #[serde(rename = "HsmCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_certificate: Option<String>,
    /// <p>The HSM hardware certificate issued (signed) by the hardware manufacturer.</p>
    #[serde(rename = "ManufacturerHardwareCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_hardware_certificate: Option<String>,
}

/// <p>Contains information about an AWS CloudHSM cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Cluster {
    /// <p>The cluster's backup policy.</p>
    #[serde(rename = "BackupPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<String>,
    /// <p>Contains one or more certificates or a certificate signing request (CSR).</p>
    #[serde(rename = "Certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Certificates>,
    /// <p>The cluster's identifier (ID).</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The date and time when the cluster was created.</p>
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The type of HSM that the cluster contains.</p>
    #[serde(rename = "HsmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
    /// <p>Contains information about the HSMs in the cluster.</p>
    #[serde(rename = "Hsms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsms: Option<Vec<Hsm>>,
    /// <p>The default password for the cluster's Pre-Crypto Officer (PRECO) user.</p>
    #[serde(rename = "PreCoPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_co_password: Option<String>,
    /// <p>The identifier (ID) of the cluster's security group.</p>
    #[serde(rename = "SecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    /// <p>The identifier (ID) of the backup used to create the cluster. This value exists only when the cluster was created from a backup.</p>
    #[serde(rename = "SourceBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    /// <p>The cluster's state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the cluster's state.</p>
    #[serde(rename = "StateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    /// <p>A map of the cluster's subnets and their corresponding Availability Zones.</p>
    #[serde(rename = "SubnetMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mapping: Option<::std::collections::HashMap<String, String>>,
    /// <p>The identifier (ID) of the virtual private cloud (VPC) that contains the cluster.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClusterRequest {
    /// <p>The type of HSM to use in the cluster. Currently the only allowed value is <code>hsm1.medium</code>.</p>
    #[serde(rename = "HsmType")]
    pub hsm_type: String,
    /// <p>The identifier (ID) of the cluster backup to restore. Use this value to restore the cluster from a backup instead of creating a new cluster. To find the backup ID, use <a>DescribeBackups</a>.</p>
    #[serde(rename = "SourceBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    /// <p><p>The identifiers (IDs) of the subnets where you are creating the cluster. You must specify at least one subnet. If you specify multiple subnets, they must meet the following criteria:</p> <ul> <li> <p>All subnets must be in the same virtual private cloud (VPC).</p> </li> <li> <p>You can specify only one subnet per Availability Zone.</p> </li> </ul></p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateClusterResponse {
    /// <p>Information about the cluster that was created.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateHsmRequest {
    /// <p>The Availability Zone where you are creating the HSM. To find the cluster's Availability Zones, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// <p>The identifier (ID) of the HSM's cluster. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The HSM's IP address. If you specify an IP address, use an available address from the subnet that maps to the Availability Zone where you are creating the HSM. If you don't specify an IP address, one is chosen for you from that subnet.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateHsmResponse {
    /// <p>Information about the HSM that was created.</p>
    #[serde(rename = "Hsm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm: Option<Hsm>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClusterRequest {
    /// <p>The identifier (ID) of the cluster that you are deleting. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteClusterResponse {
    /// <p>Information about the cluster that was deleted.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteHsmRequest {
    /// <p>The identifier (ID) of the cluster that contains the HSM that you are deleting.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The identifier (ID) of the elastic network interface (ENI) of the HSM that you are deleting.</p>
    #[serde(rename = "EniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    /// <p>The IP address of the elastic network interface (ENI) of the HSM that you are deleting.</p>
    #[serde(rename = "EniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The identifier (ID) of the HSM that you are deleting.</p>
    #[serde(rename = "HsmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteHsmResponse {
    /// <p>The identifier (ID) of the HSM that was deleted.</p>
    #[serde(rename = "HsmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBackupsRequest {
    /// <p>One or more filters to limit the items returned in the response.</p> <p>Use the <code>backupIds</code> filter to return only the specified backups. Specify backups by their backup identifier (ID).</p> <p>Use the <code>clusterIds</code> filter to return only the backups for the specified clusters. Specify clusters by their cluster identifier (ID).</p> <p>Use the <code>states</code> filter to return only backups that match the specified state.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of backups to return in the response. When there are more backups than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more backups.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeBackupsResponse {
    /// <p>A list of backups.</p>
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    /// <p>An opaque string that indicates that the response contains only a subset of backups. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClustersRequest {
    /// <p>One or more filters to limit the items returned in the response.</p> <p>Use the <code>clusterIds</code> filter to return only the specified clusters. Specify clusters by their cluster identifier (ID).</p> <p>Use the <code>vpcIds</code> filter to return only the clusters in the specified virtual private clouds (VPCs). Specify VPCs by their VPC identifier (ID).</p> <p>Use the <code>states</code> filter to return only clusters that match the specified state.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of clusters to return in the response. When there are more clusters than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more clusters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeClustersResponse {
    /// <p>A list of clusters.</p>
    #[serde(rename = "Clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    /// <p>An opaque string that indicates that the response contains only a subset of clusters. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains information about a hardware security module (HSM) in an AWS CloudHSM cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Hsm {
    /// <p>The Availability Zone that contains the HSM.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The identifier (ID) of the cluster that contains the HSM.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The identifier (ID) of the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "EniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    /// <p>The IP address of the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "EniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The HSM's identifier (ID).</p>
    #[serde(rename = "HsmId")]
    pub hsm_id: String,
    /// <p>The HSM's state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the HSM's state.</p>
    #[serde(rename = "StateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    /// <p>The subnet that contains the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InitializeClusterRequest {
    /// <p>The identifier (ID) of the cluster that you are claiming. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The cluster certificate issued (signed) by your issuing certificate authority (CA). The certificate must be in PEM format and can contain a maximum of 5000 characters.</p>
    #[serde(rename = "SignedCert")]
    pub signed_cert: String,
    /// <p>The issuing certificate of the issuing certificate authority (CA) that issued (signed) the cluster certificate. This can be a root (self-signed) certificate or a certificate chain that begins with the certificate that issued the cluster certificate and ends with a root certificate. The certificate or certificate chain must be in PEM format and can contain a maximum of 5000 characters.</p>
    #[serde(rename = "TrustAnchor")]
    pub trust_anchor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InitializeClusterResponse {
    /// <p>The cluster's state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the cluster's state.</p>
    #[serde(rename = "StateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>The maximum number of tags to return in the response. When there are more tags than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more tags.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The cluster identifier (ID) for the cluster whose tags you are getting. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsResponse {
    /// <p>An opaque string that indicates that the response contains only a subset of tags. Use this value in a subsequent <code>ListTags</code> request to get more tags.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tags.</p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

/// <p>Contains a tag. A tag is a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The cluster identifier (ID) for the cluster that you are tagging. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>A list of one or more tags.</p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The cluster identifier (ID) for the cluster whose tags you are removing. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>A list of one or more tag keys for the tags that you are removing. Specify only the tag keys, not the tag values.</p>
    #[serde(rename = "TagKeyList")]
    pub tag_key_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UntagResourceResponse {}

/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
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
                    "CloudHsmAccessDeniedException" => {
                        CreateClusterError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        CreateClusterError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        CreateClusterError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        CreateClusterError::CloudHsmResourceNotFound(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        CreateClusterError::CloudHsmService(String::from(error_message))
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
            CreateClusterError::CloudHsmAccessDenied(ref cause) => cause,
            CreateClusterError::CloudHsmInternalFailure(ref cause) => cause,
            CreateClusterError::CloudHsmInvalidRequest(ref cause) => cause,
            CreateClusterError::CloudHsmResourceNotFound(ref cause) => cause,
            CreateClusterError::CloudHsmService(ref cause) => cause,
            CreateClusterError::Validation(ref cause) => cause,
            CreateClusterError::Credentials(ref err) => err.description(),
            CreateClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHsm
#[derive(Debug, PartialEq)]
pub enum CreateHsmError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateHsmError {
    pub fn from_body(body: &str) -> CreateHsmError {
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
                    "CloudHsmAccessDeniedException" => {
                        CreateHsmError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        CreateHsmError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        CreateHsmError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        CreateHsmError::CloudHsmResourceNotFound(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        CreateHsmError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => CreateHsmError::Validation(error_message.to_string()),
                    _ => CreateHsmError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateHsmError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateHsmError {
    fn from(err: serde_json::error::Error) -> CreateHsmError {
        CreateHsmError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateHsmError {
    fn from(err: CredentialsError) -> CreateHsmError {
        CreateHsmError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateHsmError {
    fn from(err: HttpDispatchError) -> CreateHsmError {
        CreateHsmError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateHsmError {
    fn from(err: io::Error) -> CreateHsmError {
        CreateHsmError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateHsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHsmError {
    fn description(&self) -> &str {
        match *self {
            CreateHsmError::CloudHsmAccessDenied(ref cause) => cause,
            CreateHsmError::CloudHsmInternalFailure(ref cause) => cause,
            CreateHsmError::CloudHsmInvalidRequest(ref cause) => cause,
            CreateHsmError::CloudHsmResourceNotFound(ref cause) => cause,
            CreateHsmError::CloudHsmService(ref cause) => cause,
            CreateHsmError::Validation(ref cause) => cause,
            CreateHsmError::Credentials(ref err) => err.description(),
            CreateHsmError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateHsmError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteClusterError {
    pub fn from_body(body: &str) -> DeleteClusterError {
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
                    "CloudHsmAccessDeniedException" => {
                        DeleteClusterError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        DeleteClusterError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        DeleteClusterError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        DeleteClusterError::CloudHsmResourceNotFound(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        DeleteClusterError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteClusterError::Validation(error_message.to_string())
                    }
                    _ => DeleteClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteClusterError {
    fn from(err: serde_json::error::Error) -> DeleteClusterError {
        DeleteClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteClusterError {
    fn from(err: CredentialsError) -> DeleteClusterError {
        DeleteClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteClusterError {
    fn from(err: HttpDispatchError) -> DeleteClusterError {
        DeleteClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteClusterError {
    fn from(err: io::Error) -> DeleteClusterError {
        DeleteClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterError::CloudHsmAccessDenied(ref cause) => cause,
            DeleteClusterError::CloudHsmInternalFailure(ref cause) => cause,
            DeleteClusterError::CloudHsmInvalidRequest(ref cause) => cause,
            DeleteClusterError::CloudHsmResourceNotFound(ref cause) => cause,
            DeleteClusterError::CloudHsmService(ref cause) => cause,
            DeleteClusterError::Validation(ref cause) => cause,
            DeleteClusterError::Credentials(ref err) => err.description(),
            DeleteClusterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHsm
#[derive(Debug, PartialEq)]
pub enum DeleteHsmError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteHsmError {
    pub fn from_body(body: &str) -> DeleteHsmError {
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
                    "CloudHsmAccessDeniedException" => {
                        DeleteHsmError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        DeleteHsmError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        DeleteHsmError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        DeleteHsmError::CloudHsmResourceNotFound(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        DeleteHsmError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => DeleteHsmError::Validation(error_message.to_string()),
                    _ => DeleteHsmError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteHsmError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteHsmError {
    fn from(err: serde_json::error::Error) -> DeleteHsmError {
        DeleteHsmError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteHsmError {
    fn from(err: CredentialsError) -> DeleteHsmError {
        DeleteHsmError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteHsmError {
    fn from(err: HttpDispatchError) -> DeleteHsmError {
        DeleteHsmError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteHsmError {
    fn from(err: io::Error) -> DeleteHsmError {
        DeleteHsmError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteHsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHsmError {
    fn description(&self) -> &str {
        match *self {
            DeleteHsmError::CloudHsmAccessDenied(ref cause) => cause,
            DeleteHsmError::CloudHsmInternalFailure(ref cause) => cause,
            DeleteHsmError::CloudHsmInvalidRequest(ref cause) => cause,
            DeleteHsmError::CloudHsmResourceNotFound(ref cause) => cause,
            DeleteHsmError::CloudHsmService(ref cause) => cause,
            DeleteHsmError::Validation(ref cause) => cause,
            DeleteHsmError::Credentials(ref err) => err.description(),
            DeleteHsmError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteHsmError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBackups
#[derive(Debug, PartialEq)]
pub enum DescribeBackupsError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeBackupsError {
    pub fn from_body(body: &str) -> DescribeBackupsError {
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
                    "CloudHsmAccessDeniedException" => {
                        DescribeBackupsError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        DescribeBackupsError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        DescribeBackupsError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        DescribeBackupsError::CloudHsmResourceNotFound(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        DescribeBackupsError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeBackupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeBackupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeBackupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeBackupsError {
    fn from(err: serde_json::error::Error) -> DescribeBackupsError {
        DescribeBackupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeBackupsError {
    fn from(err: CredentialsError) -> DescribeBackupsError {
        DescribeBackupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeBackupsError {
    fn from(err: HttpDispatchError) -> DescribeBackupsError {
        DescribeBackupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeBackupsError {
    fn from(err: io::Error) -> DescribeBackupsError {
        DescribeBackupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeBackupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBackupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeBackupsError::CloudHsmAccessDenied(ref cause) => cause,
            DescribeBackupsError::CloudHsmInternalFailure(ref cause) => cause,
            DescribeBackupsError::CloudHsmInvalidRequest(ref cause) => cause,
            DescribeBackupsError::CloudHsmResourceNotFound(ref cause) => cause,
            DescribeBackupsError::CloudHsmService(ref cause) => cause,
            DescribeBackupsError::Validation(ref cause) => cause,
            DescribeBackupsError::Credentials(ref err) => err.description(),
            DescribeBackupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeBackupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeClustersError {
    pub fn from_body(body: &str) -> DescribeClustersError {
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
                    "CloudHsmAccessDeniedException" => {
                        DescribeClustersError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        DescribeClustersError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        DescribeClustersError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        DescribeClustersError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeClustersError::Validation(error_message.to_string())
                    }
                    _ => DescribeClustersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeClustersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeClustersError {
    fn from(err: serde_json::error::Error) -> DescribeClustersError {
        DescribeClustersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeClustersError {
    fn from(err: CredentialsError) -> DescribeClustersError {
        DescribeClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeClustersError {
    fn from(err: HttpDispatchError) -> DescribeClustersError {
        DescribeClustersError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeClustersError {
    fn from(err: io::Error) -> DescribeClustersError {
        DescribeClustersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeClustersError::CloudHsmAccessDenied(ref cause) => cause,
            DescribeClustersError::CloudHsmInternalFailure(ref cause) => cause,
            DescribeClustersError::CloudHsmInvalidRequest(ref cause) => cause,
            DescribeClustersError::CloudHsmService(ref cause) => cause,
            DescribeClustersError::Validation(ref cause) => cause,
            DescribeClustersError::Credentials(ref err) => err.description(),
            DescribeClustersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InitializeCluster
#[derive(Debug, PartialEq)]
pub enum InitializeClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InitializeClusterError {
    pub fn from_body(body: &str) -> InitializeClusterError {
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
                    "CloudHsmAccessDeniedException" => {
                        InitializeClusterError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        InitializeClusterError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        InitializeClusterError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        InitializeClusterError::CloudHsmResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "CloudHsmServiceException" => {
                        InitializeClusterError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => {
                        InitializeClusterError::Validation(error_message.to_string())
                    }
                    _ => InitializeClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => InitializeClusterError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InitializeClusterError {
    fn from(err: serde_json::error::Error) -> InitializeClusterError {
        InitializeClusterError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InitializeClusterError {
    fn from(err: CredentialsError) -> InitializeClusterError {
        InitializeClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InitializeClusterError {
    fn from(err: HttpDispatchError) -> InitializeClusterError {
        InitializeClusterError::HttpDispatch(err)
    }
}
impl From<io::Error> for InitializeClusterError {
    fn from(err: io::Error) -> InitializeClusterError {
        InitializeClusterError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InitializeClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitializeClusterError {
    fn description(&self) -> &str {
        match *self {
            InitializeClusterError::CloudHsmAccessDenied(ref cause) => cause,
            InitializeClusterError::CloudHsmInternalFailure(ref cause) => cause,
            InitializeClusterError::CloudHsmInvalidRequest(ref cause) => cause,
            InitializeClusterError::CloudHsmResourceNotFound(ref cause) => cause,
            InitializeClusterError::CloudHsmService(ref cause) => cause,
            InitializeClusterError::Validation(ref cause) => cause,
            InitializeClusterError::Credentials(ref err) => err.description(),
            InitializeClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InitializeClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsError {
    pub fn from_body(body: &str) -> ListTagsError {
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
                    "CloudHsmAccessDeniedException" => {
                        ListTagsError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        ListTagsError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        ListTagsError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        ListTagsError::CloudHsmResourceNotFound(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        ListTagsError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => ListTagsError::Validation(error_message.to_string()),
                    _ => ListTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsError {
    fn from(err: CredentialsError) -> ListTagsError {
        ListTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsError {
    fn from(err: HttpDispatchError) -> ListTagsError {
        ListTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsError {
    fn from(err: io::Error) -> ListTagsError {
        ListTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::CloudHsmAccessDenied(ref cause) => cause,
            ListTagsError::CloudHsmInternalFailure(ref cause) => cause,
            ListTagsError::CloudHsmInvalidRequest(ref cause) => cause,
            ListTagsError::CloudHsmResourceNotFound(ref cause) => cause,
            ListTagsError::CloudHsmService(ref cause) => cause,
            ListTagsError::Validation(ref cause) => cause,
            ListTagsError::Credentials(ref err) => err.description(),
            ListTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
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
                    "CloudHsmAccessDeniedException" => {
                        TagResourceError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        TagResourceError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        TagResourceError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        TagResourceError::CloudHsmResourceNotFound(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        TagResourceError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::CloudHsmAccessDenied(ref cause) => cause,
            TagResourceError::CloudHsmInternalFailure(ref cause) => cause,
            TagResourceError::CloudHsmInvalidRequest(ref cause) => cause,
            TagResourceError::CloudHsmResourceNotFound(ref cause) => cause,
            TagResourceError::CloudHsmService(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
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
                    "CloudHsmAccessDeniedException" => {
                        UntagResourceError::CloudHsmAccessDenied(String::from(error_message))
                    }
                    "CloudHsmInternalFailureException" => {
                        UntagResourceError::CloudHsmInternalFailure(String::from(error_message))
                    }
                    "CloudHsmInvalidRequestException" => {
                        UntagResourceError::CloudHsmInvalidRequest(String::from(error_message))
                    }
                    "CloudHsmResourceNotFoundException" => {
                        UntagResourceError::CloudHsmResourceNotFound(String::from(error_message))
                    }
                    "CloudHsmServiceException" => {
                        UntagResourceError::CloudHsmService(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::CloudHsmAccessDenied(ref cause) => cause,
            UntagResourceError::CloudHsmInternalFailure(ref cause) => cause,
            UntagResourceError::CloudHsmInvalidRequest(ref cause) => cause,
            UntagResourceError::CloudHsmResourceNotFound(ref cause) => cause,
            UntagResourceError::CloudHsmService(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CloudHSM V2 API. CloudHSM V2 clients implement this trait.
pub trait CloudHsmv2 {
    /// <p>Creates a new AWS CloudHSM cluster.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError>;

    /// <p>Creates a new hardware security module (HSM) in the specified AWS CloudHSM cluster.</p>
    fn create_hsm(
        &self,
        input: CreateHsmRequest,
    ) -> RusotoFuture<CreateHsmResponse, CreateHsmError>;

    /// <p>Deletes the specified AWS CloudHSM cluster. Before you can delete a cluster, you must delete all HSMs in the cluster. To see if the cluster contains any HSMs, use <a>DescribeClusters</a>. To delete an HSM, use <a>DeleteHsm</a>.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError>;

    /// <p>Deletes the specified HSM. To specify an HSM, you can use its identifier (ID), the IP address of the HSM's elastic network interface (ENI), or the ID of the HSM's ENI. You need to specify only one of these values. To find these values, use <a>DescribeClusters</a>.</p>
    fn delete_hsm(
        &self,
        input: DeleteHsmRequest,
    ) -> RusotoFuture<DeleteHsmResponse, DeleteHsmError>;

    /// <p>Gets information about backups of AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the backups. When the response contains only a subset of backups, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more backups to get.</p>
    fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> RusotoFuture<DescribeBackupsResponse, DescribeBackupsError>;

    /// <p>Gets information about AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the clusters. When the response contains only a subset of clusters, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more clusters to get.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError>;

    /// <p>Claims an AWS CloudHSM cluster by submitting the cluster certificate issued by your issuing certificate authority (CA) and the CA's root certificate. Before you can claim a cluster, you must sign the cluster's certificate signing request (CSR) with your issuing CA. To get the cluster's CSR, use <a>DescribeClusters</a>.</p>
    fn initialize_cluster(
        &self,
        input: InitializeClusterRequest,
    ) -> RusotoFuture<InitializeClusterResponse, InitializeClusterError>;

    /// <p>Gets a list of tags for the specified AWS CloudHSM cluster.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the tags. When the response contains only a subset of tags, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>ListTags</code> request to get more tags. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more tags to get.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Adds or overwrites one or more tags for the specified AWS CloudHSM cluster.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes the specified tag or tags from the specified AWS CloudHSM cluster.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;
}
/// A client for the CloudHSM V2 API.
pub struct CloudHsmv2Client<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl CloudHsmv2Client {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> CloudHsmv2Client {
        CloudHsmv2Client::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> CloudHsmv2Client<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudHsmv2Client {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> CloudHsmv2 for CloudHsmv2Client<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Creates a new AWS CloudHSM cluster.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.CreateCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateClusterResponse>(
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

    /// <p>Creates a new hardware security module (HSM) in the specified AWS CloudHSM cluster.</p>
    fn create_hsm(
        &self,
        input: CreateHsmRequest,
    ) -> RusotoFuture<CreateHsmResponse, CreateHsmError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.CreateHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateHsmResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateHsmError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified AWS CloudHSM cluster. Before you can delete a cluster, you must delete all HSMs in the cluster. To see if the cluster contains any HSMs, use <a>DescribeClusters</a>. To delete an HSM, use <a>DeleteHsm</a>.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DeleteCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteClusterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified HSM. To specify an HSM, you can use its identifier (ID), the IP address of the HSM's elastic network interface (ENI), or the ID of the HSM's ENI. You need to specify only one of these values. To find these values, use <a>DescribeClusters</a>.</p>
    fn delete_hsm(
        &self,
        input: DeleteHsmRequest,
    ) -> RusotoFuture<DeleteHsmResponse, DeleteHsmError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DeleteHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteHsmResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteHsmError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about backups of AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the backups. When the response contains only a subset of backups, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more backups to get.</p>
    fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> RusotoFuture<DescribeBackupsResponse, DescribeBackupsError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DescribeBackups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeBackupsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeBackupsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets information about AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the clusters. When the response contains only a subset of clusters, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more clusters to get.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DescribeClusters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeClustersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeClustersError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Claims an AWS CloudHSM cluster by submitting the cluster certificate issued by your issuing certificate authority (CA) and the CA's root certificate. Before you can claim a cluster, you must sign the cluster's certificate signing request (CSR) with your issuing CA. To get the cluster's CSR, use <a>DescribeClusters</a>.</p>
    fn initialize_cluster(
        &self,
        input: InitializeClusterRequest,
    ) -> RusotoFuture<InitializeClusterResponse, InitializeClusterError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.InitializeCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<InitializeClusterResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(InitializeClusterError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets a list of tags for the specified AWS CloudHSM cluster.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the tags. When the response contains only a subset of tags, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>ListTags</code> request to get more tags. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more tags to get.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds or overwrites one or more tags for the specified AWS CloudHSM cluster.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes the specified tag or tags from the specified AWS CloudHSM cluster.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
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
