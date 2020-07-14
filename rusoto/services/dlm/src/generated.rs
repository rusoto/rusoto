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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLifecyclePolicyRequest {
    /// <p>A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are supported.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by the lifecycle policy.</p>
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// <p>The configuration details of the lifecycle policy.</p>
    #[serde(rename = "PolicyDetails")]
    pub policy_details: PolicyDetails,
    /// <p>The desired activation state of the lifecycle policy after creation.</p>
    #[serde(rename = "State")]
    pub state: String,
    /// <p>The tags to apply to the lifecycle policy during creation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLifecyclePolicyResponse {
    /// <p>The identifier of the lifecycle policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}

/// <p>Specifies when to create snapshots of EBS volumes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CreateRule {
    /// <p>The interval between snapshots. The supported values are 1, 2, 3, 4, 6, 8, 12, and 24.</p>
    #[serde(rename = "Interval")]
    pub interval: i64,
    /// <p>The interval unit.</p>
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: String,
    /// <p>The time, in UTC, to start the operation. The supported format is hh:mm.</p> <p>The operation occurs within a one-hour window following the specified time.</p>
    #[serde(rename = "Times")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times: Option<Vec<String>>,
}

/// <p>Specifies the retention rule for cross-Region snapshot copies.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CrossRegionCopyRetainRule {
    /// <p>The amount of time to retain each snapshot. The maximum is 100 years. This is equivalent to 1200 months, 5200 weeks, or 36500 days.</p>
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The unit of time for time-based retention.</p>
    #[serde(rename = "IntervalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

/// <p>Specifies a rule for cross-Region snapshot copies.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CrossRegionCopyRule {
    /// <p>The Amazon Resource Name (ARN) of the AWS KMS customer master key (CMK) to use for EBS encryption. If this parameter is not specified, your AWS managed CMK for EBS is used.</p>
    #[serde(rename = "CmkArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_arn: Option<String>,
    /// <p>Copy all user-defined tags from the source snapshot to the copied snapshot.</p>
    #[serde(rename = "CopyTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    /// <p>To encrypt a copy of an unencrypted snapshot if encryption by default is not enabled, enable encryption using this parameter. Copies of encrypted snapshots are encrypted, even if this parameter is false or if encryption by default is not enabled.</p>
    #[serde(rename = "Encrypted")]
    pub encrypted: bool,
    /// <p>The retention rule.</p>
    #[serde(rename = "RetainRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_rule: Option<CrossRegionCopyRetainRule>,
    /// <p>The target Region.</p>
    #[serde(rename = "TargetRegion")]
    pub target_region: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLifecyclePolicyRequest {
    /// <p>The identifier of the lifecycle policy.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLifecyclePolicyResponse {}

/// <p>Specifies a rule for enabling fast snapshot restore. You can enable fast snapshot restore based on either a count or a time interval.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FastRestoreRule {
    /// <p>The Availability Zones in which to enable fast snapshot restore.</p>
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Vec<String>,
    /// <p>The number of snapshots to be enabled with fast snapshot restore.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The amount of time to enable fast snapshot restore. The maximum is 100 years. This is equivalent to 1200 months, 5200 weeks, or 36500 days.</p>
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The unit of time for enabling fast snapshot restore.</p>
    #[serde(rename = "IntervalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLifecyclePoliciesRequest {
    /// <p>The identifiers of the data lifecycle policies.</p>
    #[serde(rename = "PolicyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_ids: Option<Vec<String>>,
    /// <p>The resource type.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
    /// <p>The activation state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The tags to add to objects created by the policy.</p> <p>Tags are strings in the format <code>key=value</code>.</p> <p>These user-defined tags are added in addition to the AWS-added lifecycle tags.</p>
    #[serde(rename = "TagsToAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_to_add: Option<Vec<String>>,
    /// <p>The target tag for a policy.</p> <p>Tags are strings in the format <code>key=value</code>.</p>
    #[serde(rename = "TargetTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLifecyclePoliciesResponse {
    /// <p>Summary information about the lifecycle policies.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<LifecyclePolicySummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLifecyclePolicyRequest {
    /// <p>The identifier of the lifecycle policy.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLifecyclePolicyResponse {
    /// <p>Detailed information about the lifecycle policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<LifecyclePolicy>,
}

/// <p>Detailed information about a lifecycle policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LifecyclePolicy {
    /// <p>The local date and time when the lifecycle policy was created.</p>
    #[serde(rename = "DateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<f64>,
    /// <p>The local date and time when the lifecycle policy was last modified.</p>
    #[serde(rename = "DateModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<f64>,
    /// <p>The description of the lifecycle policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by the lifecycle policy.</p>
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the policy.</p>
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The configuration of the lifecycle policy</p>
    #[serde(rename = "PolicyDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    /// <p>The identifier of the lifecycle policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The activation state of the lifecycle policy.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The description of the status.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Summary information about a lifecycle policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LifecyclePolicySummary {
    /// <p>The description of the lifecycle policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the lifecycle policy.</p>
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// <p>The activation state of the lifecycle policy.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Specifies optional parameters to add to a policy. The set of valid parameters depends on the combination of policy type and resource type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Parameters {
    /// <p>[EBS Snapshot Management – Instance policies only] Indicates whether to exclude the root volume from snapshots created using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateSnapshots.html">CreateSnapshots</a>. The default is false.</p>
    #[serde(rename = "ExcludeBootVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_boot_volume: Option<bool>,
}

/// <p>Specifies the configuration of a lifecycle policy.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PolicyDetails {
    /// <p>A set of optional parameters for the policy. </p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    /// <p>The valid target resource types and actions a policy can manage. The default is EBS_SNAPSHOT_MANAGEMENT.</p>
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// <p>The resource type.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
    /// <p>The schedule of policy-defined actions.</p>
    #[serde(rename = "Schedules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<Schedule>>,
    /// <p>The single tag that identifies targeted resources for this policy.</p>
    #[serde(rename = "TargetTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tags: Option<Vec<Tag>>,
}

/// <p>Specifies the retention rule for a lifecycle policy. You can retain snapshots based on either a count or a time interval.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetainRule {
    /// <p>The number of snapshots to retain for each volume, up to a maximum of 1000.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The amount of time to retain each snapshot. The maximum is 100 years. This is equivalent to 1200 months, 5200 weeks, or 36500 days.</p>
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The unit of time for time-based retention.</p>
    #[serde(rename = "IntervalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

/// <p>Specifies a backup schedule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Schedule {
    /// <p>Copy all user-defined tags on a source volume to snapshots of the volume created by this policy.</p>
    #[serde(rename = "CopyTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    /// <p>The creation rule.</p>
    #[serde(rename = "CreateRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_rule: Option<CreateRule>,
    /// <p>The rule for cross-Region snapshot copies.</p>
    #[serde(rename = "CrossRegionCopyRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_copy_rules: Option<Vec<CrossRegionCopyRule>>,
    /// <p>The rule for enabling fast snapshot restore.</p>
    #[serde(rename = "FastRestoreRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_restore_rule: Option<FastRestoreRule>,
    /// <p>The name of the schedule.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The retention rule.</p>
    #[serde(rename = "RetainRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_rule: Option<RetainRule>,
    /// <p>The tags to apply to policy-created resources. These user-defined tags are in addition to the AWS-added lifecycle tags.</p>
    #[serde(rename = "TagsToAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_to_add: Option<Vec<Tag>>,
    /// <p>A collection of key/value pairs with values determined dynamically when the policy is executed. Keys may be any valid Amazon EC2 tag key. Values must be in one of the two following formats: <code>$(instance-id)</code> or <code>$(timestamp)</code>. Variable tags are only valid for EBS Snapshot Management – Instance policies.</p>
    #[serde(rename = "VariableTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_tags: Option<Vec<Tag>>,
}

/// <p>Specifies a tag for a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The tag key.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The tag value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLifecyclePolicyRequest {
    /// <p>A description of the lifecycle policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by the lifecycle policy.</p>
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The configuration of the lifecycle policy. You cannot update the policy type or the resource type.</p>
    #[serde(rename = "PolicyDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    /// <p>The identifier of the lifecycle policy.</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p>The desired activation state of the lifecycle policy after creation.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLifecyclePolicyResponse {}

/// Errors returned by CreateLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum CreateLifecyclePolicyError {
    /// <p>The service failed in an unexpected way.</p>
    InternalServer(String),
    /// <p>Bad request. The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request failed because a limit was exceeded.</p>
    LimitExceeded(String),
}

impl CreateLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateLifecyclePolicyError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateLifecyclePolicyError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateLifecyclePolicyError::LimitExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLifecyclePolicyError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateLifecyclePolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateLifecyclePolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLifecyclePolicyError {}
/// Errors returned by DeleteLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum DeleteLifecyclePolicyError {
    /// <p>The service failed in an unexpected way.</p>
    InternalServer(String),
    /// <p>The request failed because a limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>A requested resource was not found.</p>
    ResourceNotFound(String),
}

impl DeleteLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::InternalServer(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteLifecyclePolicyError::ResourceNotFound(
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
impl fmt::Display for DeleteLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLifecyclePolicyError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteLifecyclePolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            DeleteLifecyclePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLifecyclePolicyError {}
/// Errors returned by GetLifecyclePolicies
#[derive(Debug, PartialEq)]
pub enum GetLifecyclePoliciesError {
    /// <p>The service failed in an unexpected way.</p>
    InternalServer(String),
    /// <p>Bad request. The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request failed because a limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>A requested resource was not found.</p>
    ResourceNotFound(String),
}

impl GetLifecyclePoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLifecyclePoliciesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetLifecyclePoliciesError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetLifecyclePoliciesError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetLifecyclePoliciesError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLifecyclePoliciesError::ResourceNotFound(
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
impl fmt::Display for GetLifecyclePoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLifecyclePoliciesError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetLifecyclePoliciesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetLifecyclePoliciesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetLifecyclePoliciesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLifecyclePoliciesError {}
/// Errors returned by GetLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum GetLifecyclePolicyError {
    /// <p>The service failed in an unexpected way.</p>
    InternalServer(String),
    /// <p>The request failed because a limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>A requested resource was not found.</p>
    ResourceNotFound(String),
}

impl GetLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::InternalServer(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLifecyclePolicyError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLifecyclePolicyError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            GetLifecyclePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLifecyclePolicyError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The service failed in an unexpected way.</p>
    InternalServer(String),
    /// <p>Bad request. The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>A requested resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
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
        RusotoError::Unknown(res)
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
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The service failed in an unexpected way.</p>
    InternalServer(String),
    /// <p>Bad request. The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>A requested resource was not found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The service failed in an unexpected way.</p>
    InternalServer(String),
    /// <p>Bad request. The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>A requested resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateLifecyclePolicy
#[derive(Debug, PartialEq)]
pub enum UpdateLifecyclePolicyError {
    /// <p>The service failed in an unexpected way.</p>
    InternalServer(String),
    /// <p>Bad request. The request is missing required parameters or has invalid parameters.</p>
    InvalidRequest(String),
    /// <p>The request failed because a limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>A requested resource was not found.</p>
    ResourceNotFound(String),
}

impl UpdateLifecyclePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLifecyclePolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateLifecyclePolicyError::InternalServer(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateLifecyclePolicyError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateLifecyclePolicyError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateLifecyclePolicyError::ResourceNotFound(
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
impl fmt::Display for UpdateLifecyclePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLifecyclePolicyError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateLifecyclePolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateLifecyclePolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateLifecyclePolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLifecyclePolicyError {}
/// Trait representing the capabilities of the Amazon DLM API. Amazon DLM clients implement this trait.
#[async_trait]
pub trait Dlm {
    /// <p>Creates a policy to manage the lifecycle of the specified AWS resources. You can create up to 100 lifecycle policies.</p>
    async fn create_lifecycle_policy(
        &self,
        input: CreateLifecyclePolicyRequest,
    ) -> Result<CreateLifecyclePolicyResponse, RusotoError<CreateLifecyclePolicyError>>;

    /// <p>Deletes the specified lifecycle policy and halts the automated operations that the policy specified.</p>
    async fn delete_lifecycle_policy(
        &self,
        input: DeleteLifecyclePolicyRequest,
    ) -> Result<DeleteLifecyclePolicyResponse, RusotoError<DeleteLifecyclePolicyError>>;

    /// <p>Gets summary information about all or the specified data lifecycle policies.</p> <p>To get complete information about a policy, use <a>GetLifecyclePolicy</a>.</p>
    async fn get_lifecycle_policies(
        &self,
        input: GetLifecyclePoliciesRequest,
    ) -> Result<GetLifecyclePoliciesResponse, RusotoError<GetLifecyclePoliciesError>>;

    /// <p>Gets detailed information about the specified lifecycle policy.</p>
    async fn get_lifecycle_policy(
        &self,
        input: GetLifecyclePolicyRequest,
    ) -> Result<GetLifecyclePolicyResponse, RusotoError<GetLifecyclePolicyError>>;

    /// <p>Lists the tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds the specified tags to the specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes the specified tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the specified lifecycle policy.</p>
    async fn update_lifecycle_policy(
        &self,
        input: UpdateLifecyclePolicyRequest,
    ) -> Result<UpdateLifecyclePolicyResponse, RusotoError<UpdateLifecyclePolicyError>>;
}
/// A client for the Amazon DLM API.
#[derive(Clone)]
pub struct DlmClient {
    client: Client,
    region: region::Region,
}

impl DlmClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DlmClient {
        DlmClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DlmClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DlmClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DlmClient {
        DlmClient { client, region }
    }
}

#[async_trait]
impl Dlm for DlmClient {
    /// <p>Creates a policy to manage the lifecycle of the specified AWS resources. You can create up to 100 lifecycle policies.</p>
    async fn create_lifecycle_policy(
        &self,
        input: CreateLifecyclePolicyRequest,
    ) -> Result<CreateLifecyclePolicyResponse, RusotoError<CreateLifecyclePolicyError>> {
        let request_uri = "/policies";

        let mut request = SignedRequest::new("POST", "dlm", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateLifecyclePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Deletes the specified lifecycle policy and halts the automated operations that the policy specified.</p>
    async fn delete_lifecycle_policy(
        &self,
        input: DeleteLifecyclePolicyRequest,
    ) -> Result<DeleteLifecyclePolicyResponse, RusotoError<DeleteLifecyclePolicyError>> {
        let request_uri = format!("/policies/{policy_id}/", policy_id = input.policy_id);

        let mut request = SignedRequest::new("DELETE", "dlm", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteLifecyclePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Gets summary information about all or the specified data lifecycle policies.</p> <p>To get complete information about a policy, use <a>GetLifecyclePolicy</a>.</p>
    async fn get_lifecycle_policies(
        &self,
        input: GetLifecyclePoliciesRequest,
    ) -> Result<GetLifecyclePoliciesResponse, RusotoError<GetLifecyclePoliciesError>> {
        let request_uri = "/policies";

        let mut request = SignedRequest::new("GET", "dlm", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.policy_ids {
            for item in x.iter() {
                params.put("policyIds", item);
            }
        }
        if let Some(ref x) = input.resource_types {
            for item in x.iter() {
                params.put("resourceTypes", item);
            }
        }
        if let Some(ref x) = input.state {
            params.put("state", x);
        }
        if let Some(ref x) = input.tags_to_add {
            for item in x.iter() {
                params.put("tagsToAdd", item);
            }
        }
        if let Some(ref x) = input.target_tags {
            for item in x.iter() {
                params.put("targetTags", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLifecyclePoliciesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetLifecyclePoliciesError::from_response(response))
        }
    }

    /// <p>Gets detailed information about the specified lifecycle policy.</p>
    async fn get_lifecycle_policy(
        &self,
        input: GetLifecyclePolicyRequest,
    ) -> Result<GetLifecyclePolicyResponse, RusotoError<GetLifecyclePolicyError>> {
        let request_uri = format!("/policies/{policy_id}/", policy_id = input.policy_id);

        let mut request = SignedRequest::new("GET", "dlm", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLifecyclePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetLifecyclePolicyError::from_response(response))
        }
    }

    /// <p>Lists the tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "dlm", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Adds the specified tags to the specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "dlm", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes the specified tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "dlm", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the specified lifecycle policy.</p>
    async fn update_lifecycle_policy(
        &self,
        input: UpdateLifecyclePolicyRequest,
    ) -> Result<UpdateLifecyclePolicyResponse, RusotoError<UpdateLifecyclePolicyError>> {
        let request_uri = format!("/policies/{policy_id}", policy_id = input.policy_id);

        let mut request = SignedRequest::new("PATCH", "dlm", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateLifecyclePolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateLifecyclePolicyError::from_response(response))
        }
    }
}
