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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownArrayJobDependency {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ArrayJobDependency {
    NToN,
    Sequential,
    #[doc(hidden)]
    UnknownVariant(UnknownArrayJobDependency),
}

impl Default for ArrayJobDependency {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ArrayJobDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ArrayJobDependency {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ArrayJobDependency {
    fn into(self) -> String {
        match self {
            ArrayJobDependency::NToN => "N_TO_N".to_string(),
            ArrayJobDependency::Sequential => "SEQUENTIAL".to_string(),
            ArrayJobDependency::UnknownVariant(UnknownArrayJobDependency { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ArrayJobDependency {
    fn into(self) -> &'a str {
        match self {
            ArrayJobDependency::NToN => &"N_TO_N",
            ArrayJobDependency::Sequential => &"SEQUENTIAL",
            ArrayJobDependency::UnknownVariant(UnknownArrayJobDependency { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ArrayJobDependency {
    fn from(name: &str) -> Self {
        match name {
            "N_TO_N" => ArrayJobDependency::NToN,
            "SEQUENTIAL" => ArrayJobDependency::Sequential,
            _ => ArrayJobDependency::UnknownVariant(UnknownArrayJobDependency {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ArrayJobDependency {
    fn from(name: String) -> Self {
        match &*name {
            "N_TO_N" => ArrayJobDependency::NToN,
            "SEQUENTIAL" => ArrayJobDependency::Sequential,
            _ => ArrayJobDependency::UnknownVariant(UnknownArrayJobDependency { name }),
        }
    }
}

impl ::std::str::FromStr for ArrayJobDependency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ArrayJobDependency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ArrayJobDependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>An object representing an AWS Batch array job.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ArrayProperties {
    /// <p>The size of the array job.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>An object representing the array properties of a job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArrayPropertiesDetail {
    /// <p>The job index within the array that's associated with this job. This parameter is returned for array job children.</p>
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The size of the array job. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// <p>A summary of the number of array job children in each available job status. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "statusSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>An object representing the array properties of a job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ArrayPropertiesSummary {
    /// <p>The job index within the array that's associated with this job. This parameter is returned for children of array jobs.</p>
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// <p>The size of the array job. This parameter is returned for parent array jobs.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownAssignPublicIp {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AssignPublicIp {
    Disabled,
    Enabled,
    #[doc(hidden)]
    UnknownVariant(UnknownAssignPublicIp),
}

impl Default for AssignPublicIp {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for AssignPublicIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for AssignPublicIp {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for AssignPublicIp {
    fn into(self) -> String {
        match self {
            AssignPublicIp::Disabled => "DISABLED".to_string(),
            AssignPublicIp::Enabled => "ENABLED".to_string(),
            AssignPublicIp::UnknownVariant(UnknownAssignPublicIp { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a AssignPublicIp {
    fn into(self) -> &'a str {
        match self {
            AssignPublicIp::Disabled => &"DISABLED",
            AssignPublicIp::Enabled => &"ENABLED",
            AssignPublicIp::UnknownVariant(UnknownAssignPublicIp { name: original }) => original,
        }
    }
}

impl From<&str> for AssignPublicIp {
    fn from(name: &str) -> Self {
        match name {
            "DISABLED" => AssignPublicIp::Disabled,
            "ENABLED" => AssignPublicIp::Enabled,
            _ => AssignPublicIp::UnknownVariant(UnknownAssignPublicIp {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for AssignPublicIp {
    fn from(name: String) -> Self {
        match &*name {
            "DISABLED" => AssignPublicIp::Disabled,
            "ENABLED" => AssignPublicIp::Enabled,
            _ => AssignPublicIp::UnknownVariant(UnknownAssignPublicIp { name }),
        }
    }
}

impl ::std::str::FromStr for AssignPublicIp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for AssignPublicIp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AssignPublicIp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>An object representing the details of a container that's part of a job attempt.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttemptContainerDetail {
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS container instance that hosts the job attempt.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The exit code for the job attempt. A non-zero exit code is considered a failure.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The name of the CloudWatch Logs log stream associated with the container. The log group for AWS Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>The network interfaces associated with the job attempt.</p>
    #[serde(rename = "networkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that's associated with the job attempt. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>An object representing a job attempt.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttemptDetail {
    /// <p>Details about the container in this job attempt.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<AttemptContainerDetail>,
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was started (when the attempt transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job attempt.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was stopped (when the attempt transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCEState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CEState {
    Disabled,
    Enabled,
    #[doc(hidden)]
    UnknownVariant(UnknownCEState),
}

impl Default for CEState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CEState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CEState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CEState {
    fn into(self) -> String {
        match self {
            CEState::Disabled => "DISABLED".to_string(),
            CEState::Enabled => "ENABLED".to_string(),
            CEState::UnknownVariant(UnknownCEState { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CEState {
    fn into(self) -> &'a str {
        match self {
            CEState::Disabled => &"DISABLED",
            CEState::Enabled => &"ENABLED",
            CEState::UnknownVariant(UnknownCEState { name: original }) => original,
        }
    }
}

impl From<&str> for CEState {
    fn from(name: &str) -> Self {
        match name {
            "DISABLED" => CEState::Disabled,
            "ENABLED" => CEState::Enabled,
            _ => CEState::UnknownVariant(UnknownCEState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CEState {
    fn from(name: String) -> Self {
        match &*name {
            "DISABLED" => CEState::Disabled,
            "ENABLED" => CEState::Enabled,
            _ => CEState::UnknownVariant(UnknownCEState { name }),
        }
    }
}

impl ::std::str::FromStr for CEState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CEState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CEState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCEStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CEStatus {
    Creating,
    Deleted,
    Deleting,
    Invalid,
    Updating,
    Valid,
    #[doc(hidden)]
    UnknownVariant(UnknownCEStatus),
}

impl Default for CEStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CEStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CEStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CEStatus {
    fn into(self) -> String {
        match self {
            CEStatus::Creating => "CREATING".to_string(),
            CEStatus::Deleted => "DELETED".to_string(),
            CEStatus::Deleting => "DELETING".to_string(),
            CEStatus::Invalid => "INVALID".to_string(),
            CEStatus::Updating => "UPDATING".to_string(),
            CEStatus::Valid => "VALID".to_string(),
            CEStatus::UnknownVariant(UnknownCEStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CEStatus {
    fn into(self) -> &'a str {
        match self {
            CEStatus::Creating => &"CREATING",
            CEStatus::Deleted => &"DELETED",
            CEStatus::Deleting => &"DELETING",
            CEStatus::Invalid => &"INVALID",
            CEStatus::Updating => &"UPDATING",
            CEStatus::Valid => &"VALID",
            CEStatus::UnknownVariant(UnknownCEStatus { name: original }) => original,
        }
    }
}

impl From<&str> for CEStatus {
    fn from(name: &str) -> Self {
        match name {
            "CREATING" => CEStatus::Creating,
            "DELETED" => CEStatus::Deleted,
            "DELETING" => CEStatus::Deleting,
            "INVALID" => CEStatus::Invalid,
            "UPDATING" => CEStatus::Updating,
            "VALID" => CEStatus::Valid,
            _ => CEStatus::UnknownVariant(UnknownCEStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CEStatus {
    fn from(name: String) -> Self {
        match &*name {
            "CREATING" => CEStatus::Creating,
            "DELETED" => CEStatus::Deleted,
            "DELETING" => CEStatus::Deleting,
            "INVALID" => CEStatus::Invalid,
            "UPDATING" => CEStatus::Updating,
            "VALID" => CEStatus::Valid,
            _ => CEStatus::UnknownVariant(UnknownCEStatus { name }),
        }
    }
}

impl ::std::str::FromStr for CEStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for CEStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CEStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCEType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CEType {
    Managed,
    Unmanaged,
    #[doc(hidden)]
    UnknownVariant(UnknownCEType),
}

impl Default for CEType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CEType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CEType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CEType {
    fn into(self) -> String {
        match self {
            CEType::Managed => "MANAGED".to_string(),
            CEType::Unmanaged => "UNMANAGED".to_string(),
            CEType::UnknownVariant(UnknownCEType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CEType {
    fn into(self) -> &'a str {
        match self {
            CEType::Managed => &"MANAGED",
            CEType::Unmanaged => &"UNMANAGED",
            CEType::UnknownVariant(UnknownCEType { name: original }) => original,
        }
    }
}

impl From<&str> for CEType {
    fn from(name: &str) -> Self {
        match name {
            "MANAGED" => CEType::Managed,
            "UNMANAGED" => CEType::Unmanaged,
            _ => CEType::UnknownVariant(UnknownCEType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CEType {
    fn from(name: String) -> Self {
        match &*name {
            "MANAGED" => CEType::Managed,
            "UNMANAGED" => CEType::Unmanaged,
            _ => CEType::UnknownVariant(UnknownCEType { name }),
        }
    }
}

impl ::std::str::FromStr for CEType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CEType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CEType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCRAllocationStrategy {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CRAllocationStrategy {
    BestFit,
    BestFitProgressive,
    SpotCapacityOptimized,
    #[doc(hidden)]
    UnknownVariant(UnknownCRAllocationStrategy),
}

impl Default for CRAllocationStrategy {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CRAllocationStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CRAllocationStrategy {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CRAllocationStrategy {
    fn into(self) -> String {
        match self {
            CRAllocationStrategy::BestFit => "BEST_FIT".to_string(),
            CRAllocationStrategy::BestFitProgressive => "BEST_FIT_PROGRESSIVE".to_string(),
            CRAllocationStrategy::SpotCapacityOptimized => "SPOT_CAPACITY_OPTIMIZED".to_string(),
            CRAllocationStrategy::UnknownVariant(UnknownCRAllocationStrategy {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CRAllocationStrategy {
    fn into(self) -> &'a str {
        match self {
            CRAllocationStrategy::BestFit => &"BEST_FIT",
            CRAllocationStrategy::BestFitProgressive => &"BEST_FIT_PROGRESSIVE",
            CRAllocationStrategy::SpotCapacityOptimized => &"SPOT_CAPACITY_OPTIMIZED",
            CRAllocationStrategy::UnknownVariant(UnknownCRAllocationStrategy {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for CRAllocationStrategy {
    fn from(name: &str) -> Self {
        match name {
            "BEST_FIT" => CRAllocationStrategy::BestFit,
            "BEST_FIT_PROGRESSIVE" => CRAllocationStrategy::BestFitProgressive,
            "SPOT_CAPACITY_OPTIMIZED" => CRAllocationStrategy::SpotCapacityOptimized,
            _ => CRAllocationStrategy::UnknownVariant(UnknownCRAllocationStrategy {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CRAllocationStrategy {
    fn from(name: String) -> Self {
        match &*name {
            "BEST_FIT" => CRAllocationStrategy::BestFit,
            "BEST_FIT_PROGRESSIVE" => CRAllocationStrategy::BestFitProgressive,
            "SPOT_CAPACITY_OPTIMIZED" => CRAllocationStrategy::SpotCapacityOptimized,
            _ => CRAllocationStrategy::UnknownVariant(UnknownCRAllocationStrategy { name }),
        }
    }
}

impl ::std::str::FromStr for CRAllocationStrategy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CRAllocationStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CRAllocationStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCRType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CRType {
    Ec2,
    Fargate,
    FargateSpot,
    Spot,
    #[doc(hidden)]
    UnknownVariant(UnknownCRType),
}

impl Default for CRType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CRType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CRType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CRType {
    fn into(self) -> String {
        match self {
            CRType::Ec2 => "EC2".to_string(),
            CRType::Fargate => "FARGATE".to_string(),
            CRType::FargateSpot => "FARGATE_SPOT".to_string(),
            CRType::Spot => "SPOT".to_string(),
            CRType::UnknownVariant(UnknownCRType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a CRType {
    fn into(self) -> &'a str {
        match self {
            CRType::Ec2 => &"EC2",
            CRType::Fargate => &"FARGATE",
            CRType::FargateSpot => &"FARGATE_SPOT",
            CRType::Spot => &"SPOT",
            CRType::UnknownVariant(UnknownCRType { name: original }) => original,
        }
    }
}

impl From<&str> for CRType {
    fn from(name: &str) -> Self {
        match name {
            "EC2" => CRType::Ec2,
            "FARGATE" => CRType::Fargate,
            "FARGATE_SPOT" => CRType::FargateSpot,
            "SPOT" => CRType::Spot,
            _ => CRType::UnknownVariant(UnknownCRType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CRType {
    fn from(name: String) -> Self {
        match &*name {
            "EC2" => CRType::Ec2,
            "FARGATE" => CRType::Fargate,
            "FARGATE_SPOT" => CRType::FargateSpot,
            "SPOT" => CRType::Spot,
            _ => CRType::UnknownVariant(UnknownCRType { name }),
        }
    }
}

impl ::std::str::FromStr for CRType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CRType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CRType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Contains the parameters for <code>CancelJob</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelJobRequest {
    /// <p>The AWS Batch job ID of the job to cancel.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <a>DescribeJobs</a> operations on the job. This message is also recorded in the AWS Batch activity logs.</p>
    #[serde(rename = "reason")]
    pub reason: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelJobResponse {}

/// <p>An object representing an AWS Batch compute environment.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComputeEnvironmentDetail {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironmentArn")]
    pub compute_environment_arn: String,
    /// <p>The name of the compute environment. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "computeEnvironmentName")]
    pub compute_environment_name: String,
    /// <p>The compute resources defined for the compute environment. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    /// <p>The Amazon Resource Name (ARN) of the underlying Amazon ECS cluster used by the compute environment.</p>
    #[serde(rename = "ecsClusterArn")]
    pub ecs_cluster_arn: String,
    /// <p>The service role associated with the compute environment that allows AWS Batch to make calls to AWS API operations on your behalf. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/service_IAM_role.html">AWS Batch service IAM role</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The state of the compute environment. The valid values are <code>ENABLED</code> or <code>DISABLED</code>.</p> <p>If the state is <code>ENABLED</code>, then the AWS Batch scheduler can attempt to place jobs from an associated job queue on the compute resources within the environment. If the compute environment is managed, then it can scale its instances out or in automatically, based on the job queue demand.</p> <p>If the state is <code>DISABLED</code>, then the AWS Batch scheduler doesn't attempt to place jobs within the environment. Jobs in a <code>STARTING</code> or <code>RUNNING</code> state continue to progress normally. Managed compute environments in the <code>DISABLED</code> state don't scale out. However, they scale in to <code>minvCpus</code> value after instances become idle.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CEState>,
    /// <p>The current status of the compute environment (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CEStatus>,
    /// <p>A short, human-readable string to provide additional details about the current status of the compute environment.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The tags applied to the compute environment.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the compute environment: <code>MANAGED</code> or <code>UNMANAGED</code>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CEType>,
}

/// <p><p>The order in which compute environments are tried for job placement within a queue. Compute environments are tried in ascending order. For example, if two compute environments are associated with a job queue, the compute environment with a lower order integer value is tried for job placement first. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can&#39;t be mixed.</p> <note> <p>All compute environments that are associated with a job queue must share the same architecture. AWS Batch doesn&#39;t support mixing compute environment architecture types in a single job queue.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ComputeEnvironmentOrder {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
    /// <p>The order of the compute environment. Compute environments are tried in ascending order. For example, if two compute environments are associated with a job queue, the compute environment with a lower <code>order</code> integer value is tried for job placement first.</p>
    #[serde(rename = "order")]
    pub order: i64,
}

/// <p>An object representing an AWS Batch compute resource. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ComputeResource {
    /// <p>The allocation strategy to use for the compute resource if not enough instances of the best fitting instance type can be allocated. This might be because of availability of the instance type in the Region or <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-resource-limits.html">Amazon EC2 service limits</a>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/allocation-strategies.html">Allocation Strategies</a> in the <i>AWS Batch User Guide</i>.</p> <note> <p>This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.</p> </note> <dl> <dt>BEST_FIT (default)</dt> <dd> <p>AWS Batch selects an instance type that best fits the needs of the jobs with a preference for the lowest-cost instance type. If additional instances of the selected instance type aren't available, AWS Batch will wait for the additional instances to be available. If there are not enough instances available, or if the user is hitting <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-resource-limits.html">Amazon EC2 service limits</a> then additional jobs aren't run until currently running jobs have completed. This allocation strategy keeps costs lower but can limit scaling. If you are using Spot Fleets with <code>BEST_FIT</code> then the Spot Fleet IAM Role must be specified.</p> </dd> <dt>BEST_FIT_PROGRESSIVE</dt> <dd> <p>AWS Batch will select additional instance types that are large enough to meet the requirements of the jobs in the queue, with a preference for instance types with a lower cost per unit vCPU. If additional instances of the previously selected instance types aren't available, AWS Batch will select new instance types.</p> </dd> <dt>SPOT_CAPACITY_OPTIMIZED</dt> <dd> <p>AWS Batch will select one or more instance types that are large enough to meet the requirements of the jobs in the queue, with a preference for instance types that are less likely to be interrupted. This allocation strategy is only available for Spot Instance compute resources.</p> </dd> </dl> <p>With both <code>BEST_FIT_PROGRESSIVE</code> and <code>SPOT_CAPACITY_OPTIMIZED</code> strategies, AWS Batch might need to go above <code>maxvCpus</code> to meet your capacity requirements. In this event, AWS Batch never exceeds <code>maxvCpus</code> by more than a single instance.</p>
    #[serde(rename = "allocationStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_strategy: Option<CRAllocationStrategy>,
    /// <p><p>The maximum percentage that a Spot Instance price can be when compared with the On-Demand price for that instance type before instances are launched. For example, if your maximum percentage is 20%, then the Spot price must be less than 20% of the current On-Demand price for that Amazon EC2 instance. You always pay the lowest (market) price and never more than your maximum percentage. If you leave this field empty, the default value is 100% of the On-Demand price.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "bidPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_percentage: Option<i64>,
    /// <p><p>The desired number of Amazon EC2 vCPUS in the compute environment. AWS Batch modifies this value between the minimum and maximum values, based on job queue demand.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "desiredvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desiredv_cpus: Option<i64>,
    /// <p><p>Provides information used to select Amazon Machine Images (AMIs) for EC2 instances in the compute environment. If <code>Ec2Configuration</code> isn&#39;t specified, the default is <code>ECS_AL1</code>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "ec2Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_configuration: Option<Vec<Ec2Configuration>>,
    /// <p><p>The Amazon EC2 key pair that&#39;s used for instances launched in the compute environment. You can use this key pair to log in to your instances with SSH.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "ec2KeyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_pair: Option<String>,
    /// <p><p>The Amazon ECS instance profile applied to Amazon EC2 instances in a compute environment. You can specify the short name or full Amazon Resource Name (ARN) of an instance profile. For example, <code> <i>ecsInstanceRole</i> </code> or <code>arn:aws:iam::<i>&lt;aws<em>account</em>id&gt;</i>:instance-profile/<i>ecsInstanceRole</i> </code>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/instance_IAM_role.html">Amazon ECS Instance Role</a> in the <i>AWS Batch User Guide</i>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "instanceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    /// <p><p>The instances types that can be launched. You can specify instance families to launch any instance type within those families (for example, <code>c5</code> or <code>p3</code>), or you can specify specific sizes within a family (such as <code>c5.8xlarge</code>). You can also choose <code>optimal</code> to select instance types (from the C4, M4, and R4 instance families) on the fly that match the demand of your job queues.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note> <note> <p>When you create a compute environment, the instance types that you select for the compute environment must share the same architecture. For example, you can&#39;t mix x86 and ARM instances in the same compute environment.</p> </note> <note> <p>Currently, <code>optimal</code> uses instance types from the C4, M4, and R4 instance families. In Regions that don&#39;t have instance types from those instance families, instance types from the C5, M5. and R5 instance families are used.</p> </note></p>
    #[serde(rename = "instanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// <p><p>The launch template to use for your compute resources. Any other compute resource parameters that you specify in a <a>CreateComputeEnvironment</a> API operation override the same parameters in the launch template. You must specify either the launch template ID or launch template name in the request, but not both. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/launch-templates.html">Launch Template Support</a> in the <i>AWS Batch User Guide</i>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "launchTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// <p><p>The maximum number of Amazon EC2 vCPUs that a compute environment can reach.</p> <note> <p>With both <code>BEST<em>FIT</em>PROGRESSIVE</code> and <code>SPOT<em>CAPACITY</em>OPTIMIZED</code> allocation strategies, AWS Batch might need to go above <code>maxvCpus</code> to meet your capacity requirements. In this event, AWS Batch will never go above <code>maxvCpus</code> by more than a single instance (e.g., no more than a single instance from among those specified in your compute environment).</p> </note></p>
    #[serde(rename = "maxvCpus")]
    pub maxv_cpus: i64,
    /// <p><p>The minimum number of Amazon EC2 vCPUs that an environment should maintain (even if the compute environment is <code>DISABLED</code>).</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "minvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minv_cpus: Option<i64>,
    /// <p><p>The Amazon EC2 placement group to associate with your compute resources. If you intend to submit multi-node parallel jobs to your compute environment, you should consider creating a cluster placement group and associate it with your compute resources. This keeps your multi-node parallel job on a logical grouping of instances within a single Availability Zone with high network flow potential. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement Groups</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "placementGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
    /// <p>The Amazon EC2 security groups associated with instances launched in the compute environment. One or more security groups must be specified, either in <code>securityGroupIds</code> or using a launch template referenced in <code>launchTemplate</code>. This parameter is required for jobs running on Fargate resources and must contain at least one security group. (Fargate does not support launch templates.) If security groups are specified using both <code>securityGroupIds</code> and <code>launchTemplate</code>, the values in <code>securityGroupIds</code> will be used.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon EC2 Spot Fleet IAM role applied to a <code>SPOT</code> compute environment. This role is required if the allocation strategy set to <code>BEST<em>FIT</code> or if the allocation strategy isn&#39;t specified. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/batch/latest/userguide/spot</em>fleet<em>IAM</em>role.html&quot;&gt;Amazon EC2 Spot Fleet Role</a> in the <i>AWS Batch User Guide</i>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note> <important> <p>To tag your Spot Instances on creation, the Spot Fleet IAM role specified here must use the newer <b>AmazonEC2SpotFleetTaggingRole</b> managed policy. The previously recommended <b>AmazonEC2SpotFleetRole</b> managed policy doesn&#39;t have the required permissions to tag Spot Instances. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/troubleshooting.html#spot-instance-no-tag">Spot Instances not tagged on creation</a> in the <i>AWS Batch User Guide</i>.</p> </important></p>
    #[serde(rename = "spotIamFleetRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_iam_fleet_role: Option<String>,
    /// <p>The VPC subnets into which the compute resources are launched. These subnets must be within the same VPC. This parameter is required for jobs running on Fargate resources, where it can contain up to 16 subnets. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">VPCs and Subnets</a> in the <i>Amazon VPC User Guide</i>.</p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
    /// <p><p>Key-value pair tags to be applied to EC2 resources that are launched in the compute environment. For AWS Batch, these take the form of &quot;String1&quot;: &quot;String2&quot;, where String1 is the tag key and String2 is the tag value−for example, { &quot;Name&quot;: &quot;AWS Batch Instance - C4OnDemand&quot; }. This is helpful for recognizing your AWS Batch instances in the Amazon EC2 console. These tags can&#39;t be updated or removed after the compute environment has been created; any changes require creating a new compute environment and removing the old compute environment. These tags are not seen when using the AWS Batch <code>ListTagsForResource</code> API operation.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of compute environment: <code>EC2</code>, <code>SPOT</code>, <code>FARGATE</code>, or <code>FARGATE_SPOT</code>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p> <p> If you choose <code>SPOT</code>, you must also specify an Amazon EC2 Spot Fleet role with the <code>spotIamFleetRole</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/spot_fleet_IAM_role.html">Amazon EC2 Spot Fleet role</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "type")]
    pub type_: CRType,
}

/// <p>An object representing the attributes of a compute environment that can be updated. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ComputeResourceUpdate {
    /// <p><p>The desired number of Amazon EC2 vCPUS in the compute environment.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "desiredvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desiredv_cpus: Option<i64>,
    /// <p><p>The maximum number of Amazon EC2 vCPUs that an environment can reach.</p> <note> <p>With both <code>BEST<em>FIT</em>PROGRESSIVE</code> and <code>SPOT<em>CAPACITY</em>OPTIMIZED</code> allocation strategies, AWS Batch might need to go above <code>maxvCpus</code> to meet your capacity requirements. In this event, AWS Batch will never go above <code>maxvCpus</code> by more than a single instance (e.g., no more than a single instance from among those specified in your compute environment).</p> </note></p>
    #[serde(rename = "maxvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxv_cpus: Option<i64>,
    /// <p><p>The minimum number of Amazon EC2 vCPUs that an environment should maintain.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources, and shouldn&#39;t be specified.</p> </note></p>
    #[serde(rename = "minvCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minv_cpus: Option<i64>,
    /// <p>The Amazon EC2 security groups associated with instances launched in the compute environment. This parameter is required for Fargate compute resources, where it can contain up to 5 security groups. This can't be specified for EC2 compute resources. Providing an empty list is handled as if this parameter wasn't specified and no change is made.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The VPC subnets that the compute resources are launched into. This parameter is required for jobs running on Fargate compute resources, where it can contain up to 16 subnets. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Subnets.html">VPCs and Subnets</a> in the <i>Amazon VPC User Guide</i>. This can't be specified for EC2 compute resources. Providing an empty list will be handled as if this parameter wasn't specified and no change is made.</p>
    #[serde(rename = "subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

/// <p>An object representing the details of a container that's part of a job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContainerDetail {
    /// <p>The command that's passed to the container.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the container instance that the container is running on.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p><p>The environment variables to pass to a container.</p> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>The Amazon Resource Name (ARN) of the execution role that AWS Batch can assume. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/execution-IAM-role.html">AWS Batch execution IAM role</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The exit code to return upon completion.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The platform configuration for jobs running on Fargate resources. Jobs running on EC2 resources must not specify this parameter.</p>
    #[serde(rename = "fargatePlatformConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_platform_configuration: Option<FargatePlatformConfiguration>,
    /// <p>The image used to start the container.</p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p><p>The instance type of the underlying host infrastructure of a multi-node parallel job.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources.</p> </note></p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) associated with the job upon execution.</p>
    #[serde(rename = "jobRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,
    /// <p>Linux-specific modifications that are applied to the container, such as details for device mappings.</p>
    #[serde(rename = "linuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    /// <p><p>The log configuration specification for the container.</p> <p>This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. By default, containers use the same logging driver that the Docker daemon uses. However the container might use a different logging driver than the Docker daemon by specifying a log driver with this parameter in the container definition. To use a different logging driver for a container, the log system must be configured properly on the container instance. Or, alternatively, it must be configured on a different log server for remote logging options. For more information on the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p> <note> <p>AWS Batch currently supports a subset of the logging drivers available to the Docker daemon (shown in the <a>LogConfiguration</a> data type). Additional log drivers might be available in future releases of the Amazon ECS container agent.</p> </note> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep &quot;Server API version&quot;</code> </p> <note> <p>The Amazon ECS container agent running on a container instance must register the logging drivers available on that instance with the <code>ECS<em>AVAILABLE</em>LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    #[serde(rename = "logConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    /// <p>The name of the CloudWatch Logs log stream associated with the container. The log group for AWS Batch jobs is <code>/aws/batch/job</code>. Each container attempt receives a log stream name when they reach the <code>RUNNING</code> status.</p>
    #[serde(rename = "logStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    /// <p>For jobs run on EC2 resources that didn't specify memory requirements using <code>ResourceRequirement</code>, the number of MiB of memory reserved for the job. For other jobs, including all run on Fargate resources, see <code>resourceRequirements</code>.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The mount points for data volumes in your container.</p>
    #[serde(rename = "mountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    /// <p>The network configuration for jobs running on Fargate resources. Jobs running on EC2 resources must not specify this parameter.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The network interfaces associated with the job.</p>
    #[serde(rename = "networkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p><p>When this parameter is true, the container is given elevated permissions on the host container instance (similar to the <code>root</code> user). The default value is false.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided, or specified as false.</p> </note></p>
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>When this parameter is true, the container is given read-only access to its root file system. This parameter maps to <code>ReadonlyRootfs</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--read-only</code> option to <a href="https://docs.docker.com/engine/reference/commandline/run/"> <code>docker run</code> </a>.</p>
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The type and amount of resources to assign to a container. The supported resources include <code>GPU</code>, <code>MEMORY</code>, and <code>VCPU</code>.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    /// <p>The secrets to pass to the container. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/specifying-sensitive-data.html">Specifying sensitive data</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon ECS task that's associated with the container job. Each container attempt receives a task ARN when they reach the <code>STARTING</code> status.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p><p>A list of <code>ulimit</code> values to set in the container. This parameter maps to <code>Ulimits</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--ulimit</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources.</p> </note></p>
    #[serde(rename = "ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    /// <p>The user name to use inside the container. This parameter maps to <code>User</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--user</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p><p>The number of vCPUs reserved for the container. Jobs running on EC2 resources can specify the vCPU requirement for the job using <code>resourceRequirements</code> but the vCPU requirements can&#39;t be specified both here and in the <code>resourceRequirement</code> object. This parameter maps to <code>CpuShares</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--cpu-shares</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. Each vCPU is equivalent to 1,024 CPU shares. You must specify at least one vCPU. This is required but can be specified in several places. It must be specified for each node at least once.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources. Jobs running on Fargate resources must specify the vCPU requirement for the job using <code>resourceRequirements</code>.</p> </note></p>
    #[serde(rename = "vcpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,
    /// <p>A list of volumes associated with the job.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

/// <p>The overrides that should be sent to a container.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ContainerOverrides {
    /// <p>The command to send to the container that overrides the default command from the Docker image or the job definition.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p><p>The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the job definition.</p> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p><p>The instance type to use for a multi-node parallel job.</p> <note> <p>This parameter isn&#39;t applicable to single-node container jobs or for jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The type and amount of resources to assign to a container. This overrides the settings in the job definition. The supported resources include <code>GPU</code>, <code>MEMORY</code>, and <code>VCPU</code>.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
}

/// <p>Container properties are used in job definitions to describe the container that's launched as part of a job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerProperties {
    /// <p>The command that's passed to the container. This parameter maps to <code>Cmd</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>COMMAND</code> parameter to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. For more information, see <a href="https://docs.docker.com/engine/reference/builder/#cmd">https://docs.docker.com/engine/reference/builder/#cmd</a>.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p><p>The environment variables to pass to a container. This parameter maps to <code>Env</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--env</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <important> <p>We don&#39;t recommend using plaintext environment variables for sensitive information, such as credential data.</p> </important> <note> <p>Environment variables must not start with <code>AWS_BATCH</code>; this naming convention is reserved for variables that are set by the AWS Batch service.</p> </note></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>The Amazon Resource Name (ARN) of the execution role that AWS Batch can assume. Jobs running on Fargate resources must provide an execution role. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/execution-IAM-role.html">AWS Batch execution IAM role</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The platform configuration for jobs running on Fargate resources. Jobs running on EC2 resources must not specify this parameter.</p>
    #[serde(rename = "fargatePlatformConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_platform_configuration: Option<FargatePlatformConfiguration>,
    /// <p><p>The image used to start a container. This string is passed directly to the Docker daemon. Images in the Docker Hub registry are available by default. Other repositories are specified with <code> <i>repository-url</i>/<i>image</i>:<i>tag</i> </code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>IMAGE</code> parameter of <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>Docker image architecture must match the processor architecture of the compute resources that they&#39;re scheduled on. For example, ARM-based Docker images can only run on ARM-based compute resources.</p> </note> <ul> <li> <p>Images in Amazon ECR repositories use the full registry and repository URI (for example, <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;</code>).</p> </li> <li> <p>Images in official repositories on Docker Hub use a single name (for example, <code>ubuntu</code> or <code>mongo</code>).</p> </li> <li> <p>Images in other repositories on Docker Hub are qualified with an organization name (for example, <code>amazon/amazon-ecs-agent</code>).</p> </li> <li> <p>Images in other online repositories are qualified further by a domain name (for example, <code>quay.io/assemblyline/ubuntu</code>).</p> </li> </ul></p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p><p>The instance type to use for a multi-node parallel job. All node groups in a multi-node parallel job must use the same instance type.</p> <note> <p>This parameter isn&#39;t applicable to single-node container jobs or for jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that the container can assume for AWS permissions. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">IAM Roles for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "jobRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,
    /// <p>Linux-specific modifications that are applied to the container, such as details for device mappings.</p>
    #[serde(rename = "linuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    /// <p><p>The log configuration specification for the container.</p> <p>This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. By default, containers use the same logging driver that the Docker daemon uses. However the container might use a different logging driver than the Docker daemon by specifying a log driver with this parameter in the container definition. To use a different logging driver for a container, the log system must be configured properly on the container instance (or on a different log server for remote logging options). For more information on the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p> <note> <p>AWS Batch currently supports a subset of the logging drivers available to the Docker daemon (shown in the <a>LogConfiguration</a> data type).</p> </note> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep &quot;Server API version&quot;</code> </p> <note> <p>The Amazon ECS container agent running on a container instance must register the logging drivers available on that instance with the <code>ECS<em>AVAILABLE</em>LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    #[serde(rename = "logConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    /// <p>The mount points for data volumes in your container. This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "mountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    /// <p>The network configuration for jobs running on Fargate resources. Jobs running on EC2 resources must not specify this parameter.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p><p>When this parameter is true, the container is given elevated permissions on the host container instance (similar to the <code>root</code> user). This parameter maps to <code>Privileged</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--privileged</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. The default value is false.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided, or specified as false.</p> </note></p>
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>When this parameter is true, the container is given read-only access to its root file system. This parameter maps to <code>ReadonlyRootfs</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--read-only</code> option to <code>docker run</code>.</p>
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>The type and amount of resources to assign to a container. The supported resources include <code>GPU</code>, <code>MEMORY</code>, and <code>VCPU</code>.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    /// <p>The secrets for the container. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/specifying-sensitive-data.html">Specifying sensitive data</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    /// <p><p>A list of <code>ulimits</code> to set in the container. This parameter maps to <code>Ulimits</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--ulimit</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    /// <p>The user name to use inside the container. This parameter maps to <code>User</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--user</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p>A list of data volumes used in a job.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

/// <p>An object representing summary details of a container within a job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContainerSummary {
    /// <p>The exit code to return upon completion.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p>Contains the parameters for <code>CreateComputeEnvironment</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateComputeEnvironmentRequest {
    /// <p>The name for your compute environment. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "computeEnvironmentName")]
    pub compute_environment_name: String,
    /// <p>Details about the compute resources managed by the compute environment. This parameter is required for managed compute environments. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/service_IAM_role.html">AWS Batch service IAM role</a> in the <i>AWS Batch User Guide</i>.</p> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path.</p> <note> <p>Depending on how you created your AWS Batch service role, its ARN might contain the <code>service-role</code> path prefix. When you only specify the name of the service role, AWS Batch assumes that your ARN doesn&#39;t use the <code>service-role</code> path prefix. Because of this, we recommend that you specify the full ARN of your service role when you create compute environments.</p> </note></p>
    #[serde(rename = "serviceRole")]
    pub service_role: String,
    /// <p>The state of the compute environment. If the state is <code>ENABLED</code>, then the compute environment accepts jobs from a queue and can scale out automatically based on queues.</p> <p>If the state is <code>ENABLED</code>, then the AWS Batch scheduler can attempt to place jobs from an associated job queue on the compute resources within the environment. If the compute environment is managed, then it can scale its instances out or in automatically, based on the job queue demand.</p> <p>If the state is <code>DISABLED</code>, then the AWS Batch scheduler doesn't attempt to place jobs within the environment. Jobs in a <code>STARTING</code> or <code>RUNNING</code> state continue to progress normally. Managed compute environments in the <code>DISABLED</code> state don't scale out. However, they scale in to <code>minvCpus</code> value after instances become idle.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CEState>,
    /// <p>The tags that you apply to the compute environment to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in <i>AWS General Reference</i>.</p> <p>These tags can be updated or removed using the <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_TagResource.html">TagResource</a> and <a href="https://docs.aws.amazon.com/batch/latest/APIReference/API_UntagResource.html">UntagResource</a> API operations. These tags don't propagate to the underlying compute resources.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the compute environment: <code>MANAGED</code> or <code>UNMANAGED</code>. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "type")]
    pub type_: CEType,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateComputeEnvironmentResponse {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    /// <p>The name of the compute environment. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "computeEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
}

/// <p>Contains the parameters for <code>CreateJobQueue</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJobQueueRequest {
    /// <p><p>The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment should run a specific job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. You can associate up to three compute environments with a job queue. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can&#39;t be mixed.</p> <note> <p>All compute environments that are associated with a job queue must share the same architecture. AWS Batch doesn&#39;t support mixing compute environment architecture types in a single job queue.</p> </note></p>
    #[serde(rename = "computeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    /// <p>The name of the job queue. Up to 128 letters (uppercase and lowercase), numbers, and underscores are allowed.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments cannot be mixed.</p>
    #[serde(rename = "priority")]
    pub priority: i64,
    /// <p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<JQState>,
    /// <p>The tags that you apply to the job queue to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your AWS Batch resources</a> in <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobQueueResponse {
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    pub job_queue_arn: String,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
}

/// <p>Contains the parameters for <code>DeleteComputeEnvironment</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteComputeEnvironmentRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the compute environment to delete.</p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteComputeEnvironmentResponse {}

/// <p>Contains the parameters for <code>DeleteJobQueue</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteJobQueueRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the queue to delete.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteJobQueueResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterJobDefinitionRequest {
    /// <p>The name and revision (<code>name:revision</code>) or full Amazon Resource Name (ARN) of the job definition to deregister.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterJobDefinitionResponse {}

/// <p>Contains the parameters for <code>DescribeComputeEnvironments</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComputeEnvironmentsRequest {
    /// <p>A list of up to 100 compute environment names or full Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "computeEnvironments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environments: Option<Vec<String>>,
    /// <p>The maximum number of cluster results returned by <code>DescribeComputeEnvironments</code> in paginated output. When this parameter is used, <code>DescribeComputeEnvironments</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeComputeEnvironments</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>DescribeComputeEnvironments</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeComputeEnvironments</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that&#39;s only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeComputeEnvironmentsResponse {
    /// <p>The list of compute environments.</p>
    #[serde(rename = "computeEnvironments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environments: Option<Vec<ComputeEnvironmentDetail>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeComputeEnvironments</code> request. When the results of a <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the parameters for <code>DescribeJobDefinitions</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobDefinitionsRequest {
    /// <p>The name of the job definition to describe.</p>
    #[serde(rename = "jobDefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<String>,
    /// <p>A list of up to 100 job definition names or full Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "jobDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definitions: Option<Vec<String>>,
    /// <p>The maximum number of results returned by <code>DescribeJobDefinitions</code> in paginated output. When this parameter is used, <code>DescribeJobDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeJobDefinitions</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>DescribeJobDefinitions</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeJobDefinitions</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that&#39;s only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status used to filter job definitions.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobDefinitionsResponse {
    /// <p>The list of job definitions.</p>
    #[serde(rename = "jobDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definitions: Option<Vec<JobDefinition>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeJobDefinitions</code> request. When the results of a <code>DescribeJobDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the parameters for <code>DescribeJobQueues</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobQueuesRequest {
    /// <p>A list of up to 100 queue names or full queue Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "jobQueues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queues: Option<Vec<String>>,
    /// <p>The maximum number of results returned by <code>DescribeJobQueues</code> in paginated output. When this parameter is used, <code>DescribeJobQueues</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>DescribeJobQueues</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>DescribeJobQueues</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeJobQueues</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that&#39;s only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobQueuesResponse {
    /// <p>The list of job queues.</p>
    #[serde(rename = "jobQueues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queues: Option<Vec<JobQueueDetail>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeJobQueues</code> request. When the results of a <code>DescribeJobQueues</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the parameters for <code>DescribeJobs</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobsRequest {
    /// <p>A list of up to 100 job IDs.</p>
    #[serde(rename = "jobs")]
    pub jobs: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobsResponse {
    /// <p>The list of jobs.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobDetail>>,
}

/// <p><p>An object representing a container instance host device.</p> <note> <p>This object isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Device {
    /// <p>The path inside the container used to expose the host device. By default the <code>hostPath</code> value is used.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>The path for the device on the host container instance.</p>
    #[serde(rename = "hostPath")]
    pub host_path: String,
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<DeviceCgroupPermission>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDeviceCgroupPermission {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DeviceCgroupPermission {
    Mknod,
    Read,
    Write,
    #[doc(hidden)]
    UnknownVariant(UnknownDeviceCgroupPermission),
}

impl Default for DeviceCgroupPermission {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DeviceCgroupPermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DeviceCgroupPermission {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DeviceCgroupPermission {
    fn into(self) -> String {
        match self {
            DeviceCgroupPermission::Mknod => "MKNOD".to_string(),
            DeviceCgroupPermission::Read => "READ".to_string(),
            DeviceCgroupPermission::Write => "WRITE".to_string(),
            DeviceCgroupPermission::UnknownVariant(UnknownDeviceCgroupPermission {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DeviceCgroupPermission {
    fn into(self) -> &'a str {
        match self {
            DeviceCgroupPermission::Mknod => &"MKNOD",
            DeviceCgroupPermission::Read => &"READ",
            DeviceCgroupPermission::Write => &"WRITE",
            DeviceCgroupPermission::UnknownVariant(UnknownDeviceCgroupPermission {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DeviceCgroupPermission {
    fn from(name: &str) -> Self {
        match name {
            "MKNOD" => DeviceCgroupPermission::Mknod,
            "READ" => DeviceCgroupPermission::Read,
            "WRITE" => DeviceCgroupPermission::Write,
            _ => DeviceCgroupPermission::UnknownVariant(UnknownDeviceCgroupPermission {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DeviceCgroupPermission {
    fn from(name: String) -> Self {
        match &*name {
            "MKNOD" => DeviceCgroupPermission::Mknod,
            "READ" => DeviceCgroupPermission::Read,
            "WRITE" => DeviceCgroupPermission::Write,
            _ => DeviceCgroupPermission::UnknownVariant(UnknownDeviceCgroupPermission { name }),
        }
    }
}

impl ::std::str::FromStr for DeviceCgroupPermission {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DeviceCgroupPermission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DeviceCgroupPermission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p><p>Provides information used to select Amazon Machine Images (AMIs) for instances in the compute environment. If the <code>Ec2Configuration</code> isn&#39;t specified, the default is <code>ECS_AL1</code>.</p> <note> <p>This object isn&#39;t applicable to jobs running on Fargate resources.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ec2Configuration {
    /// <p>The AMI ID used for instances launched in the compute environment that match the image type. This setting overrides the <code>imageId</code> set in the <code>computeResource</code> object.</p>
    #[serde(rename = "imageIdOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id_override: Option<String>,
    /// <p><p>The image type to match with the instance type to select an AMI. If the <code>imageIdOverride</code> parameter isn&#39;t specified, then a recent <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized AMI</a> is used.</p> <dl> <dt>ECS<em>AL2</dt> <dd> <p> &lt;a href=&quot;https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized</em>AMI.html#al2ami&quot;&gt;Amazon Linux 2</a>− Default for all AWS Graviton-based instance families (for example, <code>C6g</code>, <code>M6g</code>, <code>R6g</code>, and <code>T4g</code>) and can be used for all non-GPU instance types.</p> </dd> <dt>ECS<em>AL2</em>NVIDIA</dt> <dd> <p> <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html#gpuami">Amazon Linux 2 (GPU)</a>−Default for all GPU instance families (for example <code>P4</code> and <code>G4</code>) and can be used for all non-AWS Graviton-based instance types.</p> </dd> <dt>ECS<em>AL1</dt> <dd> <p> &lt;a href=&quot;https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized</em>AMI.html#alami&quot;&gt;Amazon Linux</a>−Default for all non-GPU, non-AWS Graviton instance families. Amazon Linux is reaching the end-of-life of standard support. For more information, see <a href="http://aws.amazon.com/amazon-linux-ami/">Amazon Linux AMI</a>.</p> </dd> </dl></p>
    #[serde(rename = "imageType")]
    pub image_type: String,
}

/// <p>Specifies a set of conditions to be met, and an action to take (<code>RETRY</code> or <code>EXIT</code>) if all conditions are met.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EvaluateOnExit {
    /// <p>Specifies the action to take if all of the specified conditions (<code>onStatusReason</code>, <code>onReason</code>, and <code>onExitCode</code>) are met. The values are not case sensitive.</p>
    #[serde(rename = "action")]
    pub action: RetryAction,
    /// <p>Contains a glob pattern to match against the decimal representation of the <code>ExitCode</code> returned for a job. The patten can be up to 512 characters long, can contain only numbers, and can optionally end with an asterisk (*) so that only the start of the string needs to be an exact match.</p>
    #[serde(rename = "onExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit_code: Option<String>,
    /// <p>Contains a glob pattern to match against the <code>Reason</code> returned for a job. The patten can be up to 512 characters long, can contain letters, numbers, periods (.), colons (:), and white space (spaces, tabs), and can optionally end with an asterisk (*) so that only the start of the string needs to be an exact match.</p>
    #[serde(rename = "onReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_reason: Option<String>,
    /// <p>Contains a glob pattern to match against the <code>StatusReason</code> returned for a job. The patten can be up to 512 characters long, can contain letters, numbers, periods (.), colons (:), and white space (spaces, tabs). and can optionally end with an asterisk (*) so that only the start of the string needs to be an exact match.</p>
    #[serde(rename = "onStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_status_reason: Option<String>,
}

/// <p>The platform configuration for jobs running on Fargate resources. Jobs running on EC2 resources must not specify this parameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FargatePlatformConfiguration {
    /// <p>The AWS Fargate platform version on which the jobs are running. A platform version is specified only for jobs running on Fargate resources. If one isn't specified, the <code>LATEST</code> platform version is used by default. This will use a recent, approved version of the AWS Fargate platform for compute resources. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate platform versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
}

/// <p>Determine whether your data volume persists on the host container instance and where it is stored. If this parameter is empty, then the Docker daemon assigns a host path for your data volume, but the data isn't guaranteed to persist after the containers associated with it stop running.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Host {
    /// <p><p>The path on the host container instance that&#39;s presented to the container. If this parameter is empty, then the Docker daemon has assigned a host path for you. If this parameter contains a file location, then the data volume persists at the specified location on the host container instance until you delete it manually. If the source path location does not exist on the host container instance, the Docker daemon creates it. If the location does exist, the contents of the source path folder are exported.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "sourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownJQState {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum JQState {
    Disabled,
    Enabled,
    #[doc(hidden)]
    UnknownVariant(UnknownJQState),
}

impl Default for JQState {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for JQState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for JQState {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for JQState {
    fn into(self) -> String {
        match self {
            JQState::Disabled => "DISABLED".to_string(),
            JQState::Enabled => "ENABLED".to_string(),
            JQState::UnknownVariant(UnknownJQState { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a JQState {
    fn into(self) -> &'a str {
        match self {
            JQState::Disabled => &"DISABLED",
            JQState::Enabled => &"ENABLED",
            JQState::UnknownVariant(UnknownJQState { name: original }) => original,
        }
    }
}

impl From<&str> for JQState {
    fn from(name: &str) -> Self {
        match name {
            "DISABLED" => JQState::Disabled,
            "ENABLED" => JQState::Enabled,
            _ => JQState::UnknownVariant(UnknownJQState {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for JQState {
    fn from(name: String) -> Self {
        match &*name {
            "DISABLED" => JQState::Disabled,
            "ENABLED" => JQState::Enabled,
            _ => JQState::UnknownVariant(UnknownJQState { name }),
        }
    }
}

impl ::std::str::FromStr for JQState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for JQState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for JQState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownJQStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum JQStatus {
    Creating,
    Deleted,
    Deleting,
    Invalid,
    Updating,
    Valid,
    #[doc(hidden)]
    UnknownVariant(UnknownJQStatus),
}

impl Default for JQStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for JQStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for JQStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for JQStatus {
    fn into(self) -> String {
        match self {
            JQStatus::Creating => "CREATING".to_string(),
            JQStatus::Deleted => "DELETED".to_string(),
            JQStatus::Deleting => "DELETING".to_string(),
            JQStatus::Invalid => "INVALID".to_string(),
            JQStatus::Updating => "UPDATING".to_string(),
            JQStatus::Valid => "VALID".to_string(),
            JQStatus::UnknownVariant(UnknownJQStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a JQStatus {
    fn into(self) -> &'a str {
        match self {
            JQStatus::Creating => &"CREATING",
            JQStatus::Deleted => &"DELETED",
            JQStatus::Deleting => &"DELETING",
            JQStatus::Invalid => &"INVALID",
            JQStatus::Updating => &"UPDATING",
            JQStatus::Valid => &"VALID",
            JQStatus::UnknownVariant(UnknownJQStatus { name: original }) => original,
        }
    }
}

impl From<&str> for JQStatus {
    fn from(name: &str) -> Self {
        match name {
            "CREATING" => JQStatus::Creating,
            "DELETED" => JQStatus::Deleted,
            "DELETING" => JQStatus::Deleting,
            "INVALID" => JQStatus::Invalid,
            "UPDATING" => JQStatus::Updating,
            "VALID" => JQStatus::Valid,
            _ => JQStatus::UnknownVariant(UnknownJQStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for JQStatus {
    fn from(name: String) -> Self {
        match &*name {
            "CREATING" => JQStatus::Creating,
            "DELETED" => JQStatus::Deleted,
            "DELETING" => JQStatus::Deleting,
            "INVALID" => JQStatus::Invalid,
            "UPDATING" => JQStatus::Updating,
            "VALID" => JQStatus::Valid,
            _ => JQStatus::UnknownVariant(UnknownJQStatus { name }),
        }
    }
}

impl ::std::str::FromStr for JQStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for JQStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for JQStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>An object representing an AWS Batch job definition.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobDefinition {
    /// <p>An object with various properties specific to container-based jobs.</p>
    #[serde(rename = "containerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    /// <p>The Amazon Resource Name (ARN) for the job definition.</p>
    #[serde(rename = "jobDefinitionArn")]
    pub job_definition_arn: String,
    /// <p>The name of the job definition.</p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p><p>An object with various properties specific to multi-node parallel jobs.</p> <note> <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use <code>containerProperties</code> instead.</p> </note></p>
    #[serde(rename = "nodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    /// <p>Default parameters or parameter substitution placeholders that are set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition. For more information about specifying parameters, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_definition_parameters.html">Job Definition Parameters</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The platform capabilities required by the job definition. If no value is specified, it defaults to <code>EC2</code>. Jobs run on Fargate resources specify <code>FARGATE</code>.</p>
    #[serde(rename = "platformCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_capabilities: Option<Vec<PlatformCapability>>,
    /// <p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags aren't propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,
    /// <p>The retry strategy to use for failed jobs that are submitted with this job definition.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The revision of the job definition.</p>
    #[serde(rename = "revision")]
    pub revision: i64,
    /// <p>The status of the job definition.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tags applied to the job definition.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The timeout configuration for jobs that are submitted with this job definition. You can specify a timeout duration after which AWS Batch terminates your jobs if they haven't finished.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
    /// <p>The type of job definition. If the job is run on Fargate resources, then <code>multinode</code> isn't supported. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownJobDefinitionType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum JobDefinitionType {
    Container,
    Multinode,
    #[doc(hidden)]
    UnknownVariant(UnknownJobDefinitionType),
}

impl Default for JobDefinitionType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for JobDefinitionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for JobDefinitionType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for JobDefinitionType {
    fn into(self) -> String {
        match self {
            JobDefinitionType::Container => "container".to_string(),
            JobDefinitionType::Multinode => "multinode".to_string(),
            JobDefinitionType::UnknownVariant(UnknownJobDefinitionType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a JobDefinitionType {
    fn into(self) -> &'a str {
        match self {
            JobDefinitionType::Container => &"container",
            JobDefinitionType::Multinode => &"multinode",
            JobDefinitionType::UnknownVariant(UnknownJobDefinitionType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for JobDefinitionType {
    fn from(name: &str) -> Self {
        match name {
            "container" => JobDefinitionType::Container,
            "multinode" => JobDefinitionType::Multinode,
            _ => JobDefinitionType::UnknownVariant(UnknownJobDefinitionType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for JobDefinitionType {
    fn from(name: String) -> Self {
        match &*name {
            "container" => JobDefinitionType::Container,
            "multinode" => JobDefinitionType::Multinode,
            _ => JobDefinitionType::UnknownVariant(UnknownJobDefinitionType { name }),
        }
    }
}

impl ::std::str::FromStr for JobDefinitionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for JobDefinitionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "deserialize_structs")]
impl<'de> Deserialize<'de> for JobDefinitionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>An object representing an AWS Batch job dependency.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JobDependency {
    /// <p>The job ID of the AWS Batch job associated with this dependency.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The type of the job dependency.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ArrayJobDependency>,
}

/// <p>An object representing an AWS Batch job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobDetail {
    /// <p>The array properties of the job, if it is an array job.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayPropertiesDetail>,
    /// <p>A list of job attempts associated with this job.</p>
    #[serde(rename = "attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Vec<AttemptDetail>>,
    /// <p>An object representing the details of the container that's associated with the job.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerDetail>,
    /// <p>The Unix timestamp (in milliseconds) for when the job was created. For non-array jobs and parent array jobs, this is when the job entered the <code>SUBMITTED</code> state (at the time <a>SubmitJob</a> was called). For array child jobs, this is when the child job was spawned by its parent and entered the <code>PENDING</code> state.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// <p>A list of job IDs that this job depends on.</p>
    #[serde(rename = "dependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    /// <p>The Amazon Resource Name (ARN) of the job.</p>
    #[serde(rename = "jobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    /// <p>The job definition that's used by this job.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
    /// <p>The ID for the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The Amazon Resource Name (ARN) of the job queue that the job is associated with.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p>An object representing the details of a node that's associated with a multi-node parallel job.</p>
    #[serde(rename = "nodeDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_details: Option<NodeDetails>,
    /// <p><p>An object representing the node properties of a multi-node parallel job.</p> <note> <p>This isn&#39;t applicable to jobs running on Fargate resources.</p> </note></p>
    #[serde(rename = "nodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    /// <p>Additional parameters passed to the job that replace parameter substitution placeholders or override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The platform capabilities required by the job definition. If no value is specified, it defaults to <code>EC2</code>. Jobs run on Fargate resources specify <code>FARGATE</code>.</p>
    #[serde(rename = "platformCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_capabilities: Option<Vec<PlatformCapability>>,
    /// <p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags are not propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,
    /// <p>The retry strategy to use for this job if an attempt fails.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The Unix timestamp (in milliseconds) for when the job was started (when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state). This parameter isn't provided for child jobs of array jobs or multi-node parallel jobs.</p>
    #[serde(rename = "startedAt")]
    pub started_at: Option<i64>,
    /// <p><p>The current status for the job.</p> <note> <p>If your jobs don&#39;t progress to <code>STARTING</code>, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/troubleshooting.html#job_stuck_in_runnable">Jobs Stuck in RUNNABLE Status</a> in the troubleshooting section of the <i>AWS Batch User Guide</i>.</p> </note></p>
    #[serde(rename = "status")]
    pub status: JobStatus,
    /// <p>A short, human-readable string to provide additional details about the current status of the job.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix timestamp (in milliseconds) for when the job was stopped (when the job transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
    /// <p>The tags applied to the job.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The timeout configuration for the job.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
}

/// <p>An object representing the details of an AWS Batch job queue.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobQueueDetail {
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    #[serde(rename = "computeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    pub job_queue_arn: String,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    pub job_queue_name: String,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order, for example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments cannot be mixed.</p>
    #[serde(rename = "priority")]
    pub priority: i64,
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it's able to accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    #[serde(rename = "state")]
    pub state: JQState,
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JQStatus>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job queue.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The tags applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your AWS Batch resources</a> in <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownJobStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum JobStatus {
    Failed,
    Pending,
    Runnable,
    Running,
    Starting,
    Submitted,
    Succeeded,
    #[doc(hidden)]
    UnknownVariant(UnknownJobStatus),
}

impl Default for JobStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for JobStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for JobStatus {
    fn into(self) -> String {
        match self {
            JobStatus::Failed => "FAILED".to_string(),
            JobStatus::Pending => "PENDING".to_string(),
            JobStatus::Runnable => "RUNNABLE".to_string(),
            JobStatus::Running => "RUNNING".to_string(),
            JobStatus::Starting => "STARTING".to_string(),
            JobStatus::Submitted => "SUBMITTED".to_string(),
            JobStatus::Succeeded => "SUCCEEDED".to_string(),
            JobStatus::UnknownVariant(UnknownJobStatus { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a JobStatus {
    fn into(self) -> &'a str {
        match self {
            JobStatus::Failed => &"FAILED",
            JobStatus::Pending => &"PENDING",
            JobStatus::Runnable => &"RUNNABLE",
            JobStatus::Running => &"RUNNING",
            JobStatus::Starting => &"STARTING",
            JobStatus::Submitted => &"SUBMITTED",
            JobStatus::Succeeded => &"SUCCEEDED",
            JobStatus::UnknownVariant(UnknownJobStatus { name: original }) => original,
        }
    }
}

impl From<&str> for JobStatus {
    fn from(name: &str) -> Self {
        match name {
            "FAILED" => JobStatus::Failed,
            "PENDING" => JobStatus::Pending,
            "RUNNABLE" => JobStatus::Runnable,
            "RUNNING" => JobStatus::Running,
            "STARTING" => JobStatus::Starting,
            "SUBMITTED" => JobStatus::Submitted,
            "SUCCEEDED" => JobStatus::Succeeded,
            _ => JobStatus::UnknownVariant(UnknownJobStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for JobStatus {
    fn from(name: String) -> Self {
        match &*name {
            "FAILED" => JobStatus::Failed,
            "PENDING" => JobStatus::Pending,
            "RUNNABLE" => JobStatus::Runnable,
            "RUNNING" => JobStatus::Running,
            "STARTING" => JobStatus::Starting,
            "SUBMITTED" => JobStatus::Submitted,
            "SUCCEEDED" => JobStatus::Succeeded,
            _ => JobStatus::UnknownVariant(UnknownJobStatus { name }),
        }
    }
}

impl ::std::str::FromStr for JobStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for JobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for JobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>An object representing summary details of a job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobSummary {
    /// <p>The array properties of the job, if it is an array job.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayPropertiesSummary>,
    /// <p>An object representing the details of the container that's associated with the job.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerSummary>,
    /// <p>The Unix timestamp for when the job was created. For non-array jobs and parent array jobs, this is when the job entered the <code>SUBMITTED</code> state (at the time <a>SubmitJob</a> was called). For array child jobs, this is when the child job was spawned by its parent and entered the <code>PENDING</code> state.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the job.</p>
    #[serde(rename = "jobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    /// <p>The ID of the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p><p>The node properties for a single node in a job summary list.</p> <note> <p>This isn&#39;t applicable to jobs running on Fargate resources.</p> </note></p>
    #[serde(rename = "nodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodePropertiesSummary>,
    /// <p>The Unix timestamp for when the job was started (when the job transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// <p>The current status for the job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JobStatus>,
    /// <p>A short, human-readable string to provide additional details about the current status of the job.</p>
    #[serde(rename = "statusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>The Unix timestamp for when the job was stopped (when the job transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

/// <p>An object representing a job timeout configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JobTimeout {
    /// <p>The time duration in seconds (measured from the job attempt's <code>startedAt</code> timestamp) after which AWS Batch terminates your jobs if they have not finished. The minimum value for the timeout is 60 seconds.</p>
    #[serde(rename = "attemptDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_duration_seconds: Option<i64>,
}

/// <p>A key-value pair object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KeyValuePair {
    /// <p>The name of the key-value pair. For environment variables, this is the name of the environment variable.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the key-value pair. For environment variables, this is the value of the environment variable.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p><p>An object representing a launch template associated with a compute resource. You must specify either the launch template ID or launch template name in the request, but not both.</p> <p>If security groups are specified using both the <code>securityGroupIds</code> parameter of <code>CreateComputeEnvironment</code> and the launch template, the values in the <code>securityGroupIds</code> parameter of <code>CreateComputeEnvironment</code> will be used.</p> <note> <p>This object isn&#39;t applicable to jobs running on Fargate resources.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LaunchTemplateSpecification {
    /// <p>The ID of the launch template.</p>
    #[serde(rename = "launchTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,
    /// <p>The name of the launch template.</p>
    #[serde(rename = "launchTemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
    /// <p>The version number of the launch template, <code>$Latest</code>, or <code>$Default</code>.</p> <p>If the value is <code>$Latest</code>, the latest version of the launch template is used. If the value is <code>$Default</code>, the default version of the launch template is used.</p> <p>Default: <code>$Default</code>.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Linux-specific modifications that are applied to the container, such as details for device mappings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LinuxParameters {
    /// <p><p>Any host devices to expose to the container. This parameter maps to <code>Devices</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--device</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    /// <p>If true, run an <code>init</code> process inside the container that forwards signals and reaps processes. This parameter maps to the <code>--init</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. This parameter requires version 1.25 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep "Server API version"</code> </p>
    #[serde(rename = "initProcessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,
    /// <p><p>The total amount of swap memory (in MiB) a container can use. This parameter is translated to the <code>--memory-swap</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a> where the value is the sum of the container memory plus the <code>maxSwap</code> value. For more information, see <a href="https://docs.docker.com/config/containers/resource_constraints/#--memory-swap-details"> <code>--memory-swap</code> details</a> in the Docker documentation.</p> <p>If a <code>maxSwap</code> value of <code>0</code> is specified, the container doesn&#39;t use swap. Accepted values are <code>0</code> or any positive integer. If the <code>maxSwap</code> parameter is omitted, the container doesn&#39;t use the swap configuration for the container instance it is running on. A <code>maxSwap</code> value must be set for the <code>swappiness</code> parameter to be used.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "maxSwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_swap: Option<i64>,
    /// <p><p>The value for the size (in MiB) of the <code>/dev/shm</code> volume. This parameter maps to the <code>--shm-size</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "sharedMemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i64>,
    /// <p><p>This allows you to tune a container&#39;s memory swappiness behavior. A <code>swappiness</code> value of <code>0</code> causes swapping not to happen unless absolutely necessary. A <code>swappiness</code> value of <code>100</code> causes pages to be swapped very aggressively. Accepted values are whole numbers between <code>0</code> and <code>100</code>. If the <code>swappiness</code> parameter isn&#39;t specified, a default value of <code>60</code> is used. If a value isn&#39;t specified for <code>maxSwap</code> then this parameter is ignored. If <code>maxSwap</code> is set to 0, the container doesn&#39;t use swap. This parameter maps to the <code>--memory-swappiness</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <p>Consider the following when you use a per-container swap configuration.</p> <ul> <li> <p>Swap space must be enabled and allocated on the container instance for the containers to use.</p> <note> <p>The Amazon ECS optimized AMIs don&#39;t have swap enabled by default. You must enable swap on the instance to use this feature. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-store-swap-volumes.html">Instance Store Swap Volumes</a> in the <i>Amazon EC2 User Guide for Linux Instances</i> or <a href="http://aws.amazon.com/premiumsupport/knowledge-center/ec2-memory-swap-file/">How do I allocate memory to work as swap space in an Amazon EC2 instance by using a swap file?</a> </p> </note> </li> <li> <p>The swap space parameters are only supported for job definitions using EC2 resources.</p> </li> <li> <p>If the <code>maxSwap</code> and <code>swappiness</code> parameters are omitted from a job definition, each container will have a default <code>swappiness</code> value of 60 and the total swap usage will be limited to two times the memory reservation of the container.</p> </li> </ul> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "swappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i64>,
    /// <p><p>The container path, mount options, and size (in MiB) of the tmpfs mount. This parameter maps to the <code>--tmpfs</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<Tmpfs>>,
}

/// <p>Contains the parameters for <code>ListJobs</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsRequest {
    /// <p>The job ID for an array job. Specifying an array job ID with this parameter lists all child jobs from within the specified array.</p>
    #[serde(rename = "arrayJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_job_id: Option<String>,
    /// <p>The name or full Amazon Resource Name (ARN) of the job queue used to list jobs.</p>
    #[serde(rename = "jobQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue: Option<String>,
    /// <p>The job status used to filter jobs in the specified queue. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p>
    #[serde(rename = "jobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    /// <p>The maximum number of results returned by <code>ListJobs</code> in paginated output. When this parameter is used, <code>ListJobs</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobs</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListJobs</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The job ID for a multi-node parallel job. Specifying a multi-node parallel job ID with this parameter lists all nodes that are associated with the specified job.</p>
    #[serde(rename = "multiNodeJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_node_job_id: Option<String>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobs</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note> <p>This token should be treated as an opaque identifier that&#39;s only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobsResponse {
    /// <p>A list of job summaries that match the request.</p>
    #[serde(rename = "jobSummaryList")]
    pub job_summary_list: Vec<JobSummary>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListJobs</code> request. When the results of a <code>ListJobs</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource that tags are listed for. AWS Batch resources that support tags are compute environments, jobs, job definitions, and job queues. ARNs for child jobs of array and multi-node parallel (MNP) jobs are not supported.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Log configuration options to send to a custom log driver for the container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LogConfiguration {
    /// <p>The log driver to use for the container. The valid values listed for this parameter are log drivers that the Amazon ECS container agent can communicate with by default.</p> <p>The supported log drivers are <code>awslogs</code>, <code>fluentd</code>, <code>gelf</code>, <code>json-file</code>, <code>journald</code>, <code>logentries</code>, <code>syslog</code>, and <code>splunk</code>.</p> <note> <p>Jobs running on Fargate resources are restricted to the <code>awslogs</code> and <code>splunk</code> log drivers.</p> </note> <dl> <dt>awslogs</dt> <dd> <p>Specifies the Amazon CloudWatch Logs logging driver. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using_awslogs.html">Using the awslogs Log Driver</a> in the <i>AWS Batch User Guide</i> and <a href="https://docs.docker.com/config/containers/logging/awslogs/">Amazon CloudWatch Logs logging driver</a> in the Docker documentation.</p> </dd> <dt>fluentd</dt> <dd> <p>Specifies the Fluentd logging driver. For more information, including usage and options, see <a href="https://docs.docker.com/config/containers/logging/fluentd/">Fluentd logging driver</a> in the Docker documentation.</p> </dd> <dt>gelf</dt> <dd> <p>Specifies the Graylog Extended Format (GELF) logging driver. For more information, including usage and options, see <a href="https://docs.docker.com/config/containers/logging/gelf/">Graylog Extended Format logging driver</a> in the Docker documentation.</p> </dd> <dt>journald</dt> <dd> <p>Specifies the journald logging driver. For more information, including usage and options, see <a href="https://docs.docker.com/config/containers/logging/journald/">Journald logging driver</a> in the Docker documentation.</p> </dd> <dt>json-file</dt> <dd> <p>Specifies the JSON file logging driver. For more information, including usage and options, see <a href="https://docs.docker.com/config/containers/logging/json-file/">JSON File logging driver</a> in the Docker documentation.</p> </dd> <dt>splunk</dt> <dd> <p>Specifies the Splunk logging driver. For more information, including usage and options, see <a href="https://docs.docker.com/config/containers/logging/splunk/">Splunk logging driver</a> in the Docker documentation.</p> </dd> <dt>syslog</dt> <dd> <p>Specifies the syslog logging driver. For more information, including usage and options, see <a href="https://docs.docker.com/config/containers/logging/syslog/">Syslog logging driver</a> in the Docker documentation.</p> </dd> </dl> <note> <p>If you have a custom driver that'sn't listed earlier that you want to work with the Amazon ECS container agent, you can fork the Amazon ECS container agent project that's <a href="https://github.com/aws/amazon-ecs-agent">available on GitHub</a> and customize it to work with that driver. We encourage you to submit pull requests for changes that you want to have included. However, Amazon Web Services doesn't currently support running modified copies of this software.</p> </note> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep "Server API version"</code> </p>
    #[serde(rename = "logDriver")]
    pub log_driver: LogDriver,
    /// <p>The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log into your container instance and run the following command: <code>sudo docker version | grep "Server API version"</code> </p>
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The secrets to pass to the log configuration. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/specifying-sensitive-data.html">Specifying Sensitive Data</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "secretOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options: Option<Vec<Secret>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownLogDriver {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum LogDriver {
    Awslogs,
    Fluentd,
    Gelf,
    Journald,
    JsonFile,
    Splunk,
    Syslog,
    #[doc(hidden)]
    UnknownVariant(UnknownLogDriver),
}

impl Default for LogDriver {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for LogDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for LogDriver {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for LogDriver {
    fn into(self) -> String {
        match self {
            LogDriver::Awslogs => "awslogs".to_string(),
            LogDriver::Fluentd => "fluentd".to_string(),
            LogDriver::Gelf => "gelf".to_string(),
            LogDriver::Journald => "journald".to_string(),
            LogDriver::JsonFile => "json-file".to_string(),
            LogDriver::Splunk => "splunk".to_string(),
            LogDriver::Syslog => "syslog".to_string(),
            LogDriver::UnknownVariant(UnknownLogDriver { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a LogDriver {
    fn into(self) -> &'a str {
        match self {
            LogDriver::Awslogs => &"awslogs",
            LogDriver::Fluentd => &"fluentd",
            LogDriver::Gelf => &"gelf",
            LogDriver::Journald => &"journald",
            LogDriver::JsonFile => &"json-file",
            LogDriver::Splunk => &"splunk",
            LogDriver::Syslog => &"syslog",
            LogDriver::UnknownVariant(UnknownLogDriver { name: original }) => original,
        }
    }
}

impl From<&str> for LogDriver {
    fn from(name: &str) -> Self {
        match name {
            "awslogs" => LogDriver::Awslogs,
            "fluentd" => LogDriver::Fluentd,
            "gelf" => LogDriver::Gelf,
            "journald" => LogDriver::Journald,
            "json-file" => LogDriver::JsonFile,
            "splunk" => LogDriver::Splunk,
            "syslog" => LogDriver::Syslog,
            _ => LogDriver::UnknownVariant(UnknownLogDriver {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for LogDriver {
    fn from(name: String) -> Self {
        match &*name {
            "awslogs" => LogDriver::Awslogs,
            "fluentd" => LogDriver::Fluentd,
            "gelf" => LogDriver::Gelf,
            "journald" => LogDriver::Journald,
            "json-file" => LogDriver::JsonFile,
            "splunk" => LogDriver::Splunk,
            "syslog" => LogDriver::Syslog,
            _ => LogDriver::UnknownVariant(UnknownLogDriver { name }),
        }
    }
}

impl ::std::str::FromStr for LogDriver {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for LogDriver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LogDriver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Details on a Docker volume mount point that's used in a job's container properties. This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/reference/api/docker_remote_api_v1.19/#create-a-container">Create a container</a> section of the Docker Remote API and the <code>--volume</code> option to docker run.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MountPoint {
    /// <p>The path on the container where the host volume is mounted.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. Otherwise, the container can write to the volume. The default value is <code>false</code>.</p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of the volume to mount.</p>
    #[serde(rename = "sourceVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

/// <p>The network configuration for jobs running on Fargate resources. Jobs running on EC2 resources must not specify this parameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkConfiguration {
    /// <p>Indicates whether the job should have a public IP address. For a job running on Fargate resources in a private subnet to send outbound traffic to the internet (for example, in order to pull container images), the private subnet requires a NAT gateway be attached to route requests to the internet. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Amazon ECS task networking</a>. The default value is "DISABLED".</p>
    #[serde(rename = "assignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<AssignPublicIp>,
}

/// <p>An object representing the elastic network interface for a multi-node parallel job node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkInterface {
    /// <p>The attachment ID for the network interface.</p>
    #[serde(rename = "attachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>The private IPv6 address for the network interface.</p>
    #[serde(rename = "ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_address: Option<String>,
    /// <p>The private IPv4 address for the network interface.</p>
    #[serde(rename = "privateIpv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ipv_4_address: Option<String>,
}

/// <p>An object representing the details of a multi-node parallel job node.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodeDetails {
    /// <p>Specifies whether the current node is the main node for a multi-node parallel job.</p>
    #[serde(rename = "isMainNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_node: Option<bool>,
    /// <p>The node index for the node. Node index numbering begins at zero. This index is also available on the node with the <code>AWS_BATCH_JOB_NODE_INDEX</code> environment variable.</p>
    #[serde(rename = "nodeIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_index: Option<i64>,
}

/// <p><p>Object representing any node overrides to a job definition that&#39;s used in a <a>SubmitJob</a> API operation.</p> <note> <p>This isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided; use <code>containerOverrides</code> instead.</p> </note></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NodeOverrides {
    /// <p>The node property overrides for the job.</p>
    #[serde(rename = "nodePropertyOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_property_overrides: Option<Vec<NodePropertyOverride>>,
    /// <p><p>The number of nodes to use with a multi-node parallel job. This value overrides the number of nodes that are specified in the job definition. To use this override:</p> <ul> <li> <p>There must be at least one node range in your job definition that has an open upper boundary (such as <code>:</code> or <code>n:</code>).</p> </li> <li> <p>The lower boundary of the node range specified in the job definition must be fewer than the number of nodes specified in the override.</p> </li> <li> <p>The main node index specified in the job definition must be fewer than the number of nodes specified in the override.</p> </li> </ul></p>
    #[serde(rename = "numNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_nodes: Option<i64>,
}

/// <p>An object representing the node properties of a multi-node parallel job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NodeProperties {
    /// <p>Specifies the node index for the main node of a multi-node parallel job. This node index value must be fewer than the number of nodes.</p>
    #[serde(rename = "mainNode")]
    pub main_node: i64,
    /// <p>A list of node ranges and their properties associated with a multi-node parallel job.</p>
    #[serde(rename = "nodeRangeProperties")]
    pub node_range_properties: Vec<NodeRangeProperty>,
    /// <p>The number of nodes associated with a multi-node parallel job.</p>
    #[serde(rename = "numNodes")]
    pub num_nodes: i64,
}

/// <p>An object representing the properties of a node that's associated with a multi-node parallel job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NodePropertiesSummary {
    /// <p>Specifies whether the current node is the main node for a multi-node parallel job.</p>
    #[serde(rename = "isMainNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_node: Option<bool>,
    /// <p>The node index for the node. Node index numbering begins at zero. This index is also available on the node with the <code>AWS_BATCH_JOB_NODE_INDEX</code> environment variable.</p>
    #[serde(rename = "nodeIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_index: Option<i64>,
    /// <p>The number of nodes associated with a multi-node parallel job.</p>
    #[serde(rename = "numNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_nodes: Option<i64>,
}

/// <p>Object representing any node overrides to a job definition that's used in a <a>SubmitJob</a> API operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NodePropertyOverride {
    /// <p>The overrides that should be sent to a node range.</p>
    #[serde(rename = "containerOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<ContainerOverrides>,
    /// <p>The range of nodes, using node index values, that's used to override. A range of <code>0:3</code> indicates nodes with index values of <code>0</code> through <code>3</code>. If the starting range value is omitted (<code>:n</code>), then <code>0</code> is used to start the range. If the ending range value is omitted (<code>n:</code>), then the highest possible node index is used to end the range.</p>
    #[serde(rename = "targetNodes")]
    pub target_nodes: String,
}

/// <p>An object representing the properties of the node range for a multi-node parallel job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NodeRangeProperty {
    /// <p>The container details for the node range.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerProperties>,
    /// <p>The range of nodes, using node index values. A range of <code>0:3</code> indicates nodes with index values of <code>0</code> through <code>3</code>. If the starting range value is omitted (<code>:n</code>), then <code>0</code> is used to start the range. If the ending range value is omitted (<code>n:</code>), then the highest possible node index is used to end the range. Your accumulative node ranges must account for all nodes (<code>0:n</code>). You can nest node ranges, for example <code>0:10</code> and <code>4:5</code>, in which case the <code>4:5</code> range properties override the <code>0:10</code> properties.</p>
    #[serde(rename = "targetNodes")]
    pub target_nodes: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownPlatformCapability {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum PlatformCapability {
    Ec2,
    Fargate,
    #[doc(hidden)]
    UnknownVariant(UnknownPlatformCapability),
}

impl Default for PlatformCapability {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for PlatformCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for PlatformCapability {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for PlatformCapability {
    fn into(self) -> String {
        match self {
            PlatformCapability::Ec2 => "EC2".to_string(),
            PlatformCapability::Fargate => "FARGATE".to_string(),
            PlatformCapability::UnknownVariant(UnknownPlatformCapability { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a PlatformCapability {
    fn into(self) -> &'a str {
        match self {
            PlatformCapability::Ec2 => &"EC2",
            PlatformCapability::Fargate => &"FARGATE",
            PlatformCapability::UnknownVariant(UnknownPlatformCapability { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for PlatformCapability {
    fn from(name: &str) -> Self {
        match name {
            "EC2" => PlatformCapability::Ec2,
            "FARGATE" => PlatformCapability::Fargate,
            _ => PlatformCapability::UnknownVariant(UnknownPlatformCapability {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for PlatformCapability {
    fn from(name: String) -> Self {
        match &*name {
            "EC2" => PlatformCapability::Ec2,
            "FARGATE" => PlatformCapability::Fargate,
            _ => PlatformCapability::UnknownVariant(UnknownPlatformCapability { name }),
        }
    }
}

impl ::std::str::FromStr for PlatformCapability {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for PlatformCapability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PlatformCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Contains the parameters for <code>RegisterJobDefinition</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterJobDefinitionRequest {
    /// <p><p>An object with various properties specific to single-node container-based jobs. If the job definition&#39;s <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>.</p> <note> <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use only <code>containerProperties</code>.</p> </note></p>
    #[serde(rename = "containerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    /// <p>The name of the job definition to register. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p><p>An object with various properties specific to multi-node parallel jobs. If you specify node properties for a job, it becomes a multi-node parallel job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-parallel-jobs.html">Multi-node Parallel Jobs</a> in the <i>AWS Batch User Guide</i>. If the job definition&#39;s <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>.</p> <note> <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use <code>containerProperties</code> instead.</p> </note></p>
    #[serde(rename = "nodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    /// <p>Default parameter substitution placeholders to set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The platform capabilities required by the job definition. If no value is specified, it defaults to <code>EC2</code>. To run the job on Fargate resources, specify <code>FARGATE</code>.</p>
    #[serde(rename = "platformCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_capabilities: Option<Vec<PlatformCapability>>,
    /// <p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags are not propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,
    /// <p>The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that's specified during a <a>SubmitJob</a> operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it isn't retried.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The tags that you apply to the job definition to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging AWS Resources</a> in <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The timeout configuration for jobs that are submitted with this job definition, after which AWS Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. Any timeout configuration that's specified during a <a>SubmitJob</a> operation overrides the timeout configuration defined here. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_timeouts.html">Job Timeouts</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
    /// <p><p>The type of job definition. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>AWS Batch User Guide</i>.</p> <note> <p>If the job is run on Fargate resources, then <code>multinode</code> isn&#39;t supported.</p> </note></p>
    #[serde(rename = "type")]
    pub type_: JobDefinitionType,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterJobDefinitionResponse {
    /// <p>The Amazon Resource Name (ARN) of the job definition.</p>
    #[serde(rename = "jobDefinitionArn")]
    pub job_definition_arn: String,
    /// <p>The name of the job definition.</p>
    #[serde(rename = "jobDefinitionName")]
    pub job_definition_name: String,
    /// <p>The revision of the job definition.</p>
    #[serde(rename = "revision")]
    pub revision: i64,
}

/// <p>The type and amount of a resource to assign to a container. The supported resources include <code>GPU</code>, <code>MEMORY</code>, and <code>VCPU</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceRequirement {
    /// <p>The type of resource to assign to a container. The supported resources include <code>GPU</code>, <code>MEMORY</code>, and <code>VCPU</code>.</p>
    #[serde(rename = "type")]
    pub type_: ResourceType,
    /// <p><p>The quantity of the specified resource to reserve for the container. The values vary based on the <code>type</code> specified.</p> <dl> <dt>type=&quot;GPU&quot;</dt> <dd> <p>The number of physical GPUs to reserve for the container. The number of GPUs reserved for all containers in a job shouldn&#39;t exceed the number of available GPUs on the compute resource that the job is launched on.</p> <note> <p>GPUs are not available for jobs running on Fargate resources.</p> </note> </dd> <dt>type=&quot;MEMORY&quot;</dt> <dd> <p>For jobs running on EC2 resources, the hard limit (in MiB) of memory to present to the container. If your container attempts to exceed the memory specified here, the container is killed. This parameter maps to <code>Memory</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--memory</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. You must specify at least 4 MiB of memory for a job. This is required but can be specified in several places for multi-node parallel (MNP) jobs. It must be specified for each node at least once. This parameter maps to <code>Memory</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--memory</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>If you&#39;re trying to maximize your resource utilization by providing your jobs as much memory as possible for a particular instance type, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/memory-management.html">Memory Management</a> in the <i>AWS Batch User Guide</i>.</p> </note> <p>For jobs running on Fargate resources, then <code>value</code> is the hard limit (in MiB), and must match one of the supported values and the <code>VCPU</code> values must be one of the values supported for that memory value.</p> <dl> <dt>value = 512</dt> <dd> <p> <code>VCPU</code> = 0.25</p> </dd> <dt>value = 1024</dt> <dd> <p> <code>VCPU</code> = 0.25 or 0.5</p> </dd> <dt>value = 2048</dt> <dd> <p> <code>VCPU</code> = 0.25, 0.5, or 1</p> </dd> <dt>value = 3072</dt> <dd> <p> <code>VCPU</code> = 0.5, or 1</p> </dd> <dt>value = 4096</dt> <dd> <p> <code>VCPU</code> = 0.5, 1, or 2</p> </dd> <dt>value = 5120, 6144, or 7168</dt> <dd> <p> <code>VCPU</code> = 1 or 2</p> </dd> <dt>value = 8192</dt> <dd> <p> <code>VCPU</code> = 1, 2, or 4</p> </dd> <dt>value = 9216, 10240, 11264, 12288, 13312, 14336, 15360, or 16384</dt> <dd> <p> <code>VCPU</code> = 2 or 4</p> </dd> <dt>value = 17408, 18432, 19456, 20480, 21504, 22528, 23552, 24576, 25600, 26624, 27648, 28672, 29696, or 30720</dt> <dd> <p> <code>VCPU</code> = 4</p> </dd> </dl> </dd> <dt>type=&quot;VCPU&quot;</dt> <dd> <p>The number of vCPUs reserved for the container. This parameter maps to <code>CpuShares</code> in the <a href="https://docs.docker.com/engine/api/v1.23/#create-a-container">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.23/">Docker Remote API</a> and the <code>--cpu-shares</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. Each vCPU is equivalent to 1,024 CPU shares. For EC2 resources, you must specify at least one vCPU. This is required but can be specified in several places; it must be specified for each node at least once.</p> <p>For jobs running on Fargate resources, then <code>value</code> must match one of the supported values and the <code>MEMORY</code> values must be one of the values supported for that VCPU value. The supported values are 0.25, 0.5, 1, 2, and 4</p> <dl> <dt>value = 0.25</dt> <dd> <p> <code>MEMORY</code> = 512, 1024, or 2048</p> </dd> <dt>value = 0.5</dt> <dd> <p> <code>MEMORY</code> = 1024, 2048, 3072, or 4096</p> </dd> <dt>value = 1</dt> <dd> <p> <code>MEMORY</code> = 2048, 3072, 4096, 5120, 6144, 7168, or 8192</p> </dd> <dt>value = 2</dt> <dd> <p> <code>MEMORY</code> = 4096, 5120, 6144, 7168, 8192, 9216, 10240, 11264, 12288, 13312, 14336, 15360, or 16384</p> </dd> <dt>value = 4</dt> <dd> <p> <code>MEMORY</code> = 8192, 9216, 10240, 11264, 12288, 13312, 14336, 15360, 16384, 17408, 18432, 19456, 20480, 21504, 22528, 23552, 24576, 25600, 26624, 27648, 28672, 29696, or 30720</p> </dd> </dl> </dd> </dl></p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownResourceType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResourceType {
    Gpu,
    Memory,
    Vcpu,
    #[doc(hidden)]
    UnknownVariant(UnknownResourceType),
}

impl Default for ResourceType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ResourceType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ResourceType {
    fn into(self) -> String {
        match self {
            ResourceType::Gpu => "GPU".to_string(),
            ResourceType::Memory => "MEMORY".to_string(),
            ResourceType::Vcpu => "VCPU".to_string(),
            ResourceType::UnknownVariant(UnknownResourceType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ResourceType {
    fn into(self) -> &'a str {
        match self {
            ResourceType::Gpu => &"GPU",
            ResourceType::Memory => &"MEMORY",
            ResourceType::Vcpu => &"VCPU",
            ResourceType::UnknownVariant(UnknownResourceType { name: original }) => original,
        }
    }
}

impl From<&str> for ResourceType {
    fn from(name: &str) -> Self {
        match name {
            "GPU" => ResourceType::Gpu,
            "MEMORY" => ResourceType::Memory,
            "VCPU" => ResourceType::Vcpu,
            _ => ResourceType::UnknownVariant(UnknownResourceType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ResourceType {
    fn from(name: String) -> Self {
        match &*name {
            "GPU" => ResourceType::Gpu,
            "MEMORY" => ResourceType::Memory,
            "VCPU" => ResourceType::Vcpu,
            _ => ResourceType::UnknownVariant(UnknownResourceType { name }),
        }
    }
}

impl ::std::str::FromStr for ResourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRetryAction {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RetryAction {
    Exit,
    Retry,
    #[doc(hidden)]
    UnknownVariant(UnknownRetryAction),
}

impl Default for RetryAction {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RetryAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RetryAction {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RetryAction {
    fn into(self) -> String {
        match self {
            RetryAction::Exit => "EXIT".to_string(),
            RetryAction::Retry => "RETRY".to_string(),
            RetryAction::UnknownVariant(UnknownRetryAction { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RetryAction {
    fn into(self) -> &'a str {
        match self {
            RetryAction::Exit => &"EXIT",
            RetryAction::Retry => &"RETRY",
            RetryAction::UnknownVariant(UnknownRetryAction { name: original }) => original,
        }
    }
}

impl From<&str> for RetryAction {
    fn from(name: &str) -> Self {
        match name {
            "EXIT" => RetryAction::Exit,
            "RETRY" => RetryAction::Retry,
            _ => RetryAction::UnknownVariant(UnknownRetryAction {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RetryAction {
    fn from(name: String) -> Self {
        match &*name {
            "EXIT" => RetryAction::Exit,
            "RETRY" => RetryAction::Retry,
            _ => RetryAction::UnknownVariant(UnknownRetryAction { name }),
        }
    }
}

impl ::std::str::FromStr for RetryAction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for RetryAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RetryAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The retry strategy associated with a job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_retries.html">Automated job retries</a> in the <i>AWS Batch User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RetryStrategy {
    /// <p>The number of times to move a job to the <code>RUNNABLE</code> status. You can specify between 1 and 10 attempts. If the value of <code>attempts</code> is greater than one, the job is retried on failure the same number of attempts as the value.</p>
    #[serde(rename = "attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
    /// <p>Array of up to 5 objects that specify conditions under which the job should be retried or failed. If this parameter is specified, then the <code>attempts</code> parameter must also be specified.</p>
    #[serde(rename = "evaluateOnExit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_on_exit: Option<Vec<EvaluateOnExit>>,
}

/// <p>An object representing the secret to expose to your container. Secrets can be exposed to a container in the following ways:</p> <ul> <li> <p>To inject sensitive data into your containers as environment variables, use the <code>secrets</code> container definition parameter.</p> </li> <li> <p>To reference sensitive information in the log configuration of a container, use the <code>secretOptions</code> container definition parameter.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/specifying-sensitive-data.html">Specifying sensitive data</a> in the <i>AWS Batch User Guide</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Secret {
    /// <p>The name of the secret.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The secret to expose to the container. The supported values are either the full ARN of the AWS Secrets Manager secret or the full ARN of the parameter in the AWS Systems Manager Parameter Store.</p> <note> <p>If the AWS Systems Manager Parameter Store parameter exists in the same Region as the job you are launching, then you can use either the full ARN or name of the parameter. If the parameter exists in a different Region, then the full ARN must be specified.</p> </note></p>
    #[serde(rename = "valueFrom")]
    pub value_from: String,
}

/// <p>Contains the parameters for <code>SubmitJob</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SubmitJobRequest {
    /// <p>The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/array_jobs.html">Array Jobs</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "arrayProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayProperties>,
    /// <p>A list of container overrides in JSON format that specify the name of a container in the specified job definition and the overrides it should receive. You can override the default command for a container (that's specified in the job definition or the Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the job definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p>
    #[serde(rename = "containerOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<ContainerOverrides>,
    /// <p>A list of dependencies for the job. A job can depend upon a maximum of 20 jobs. You can specify a <code>SEQUENTIAL</code> type dependency without specifying a job ID for array jobs so that each child array job completes sequentially, starting at index 0. You can also specify an <code>N_TO_N</code> type dependency with a job ID for array jobs. In that case, each index child of this job must wait for the corresponding index child of each dependency to complete before it can begin.</p>
    #[serde(rename = "dependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    /// <p>The job definition used by this job. This value can be one of <code>name</code>, <code>name:revision</code>, or the Amazon Resource Name (ARN) for the job definition. If <code>name</code> is specified without a revision then the latest active revision is used.</p>
    #[serde(rename = "jobDefinition")]
    pub job_definition: String,
    /// <p>The name of the job. The first character must be alphanumeric, and up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
    /// <p>The job queue into which the job is submitted. You can specify either the name or the Amazon Resource Name (ARN) of the queue.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p><p>A list of node overrides in JSON format that specify the node range to target and the container overrides for that node range.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources; use <code>containerOverrides</code> instead.</p> </note></p>
    #[serde(rename = "nodeOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_overrides: Option<NodeOverrides>,
    /// <p>Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags aren't propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state. When specified, this overrides the tag propagation setting in the job definition.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,
    /// <p>The retry strategy to use for failed jobs from this <a>SubmitJob</a> operation. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition.</p>
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// <p>The tags that you apply to the job request to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in <i>AWS General Reference</i>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The timeout configuration for this <a>SubmitJob</a> operation. You can specify a timeout duration after which AWS Batch terminates your jobs if they haven't finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. This configuration overrides any timeout configuration specified in the job definition. For array jobs, child jobs have the same timeout configuration as the parent job. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/job_timeouts.html">Job Timeouts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubmitJobResponse {
    /// <p>The Amazon Resource Name (ARN) for the job.</p>
    #[serde(rename = "jobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    /// <p>The unique identifier for the job.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the job.</p>
    #[serde(rename = "jobName")]
    pub job_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that tags are added to. AWS Batch resources that support tags are compute environments, jobs, job definitions, and job queues. ARNs for child jobs of array and multi-node parallel (MNP) jobs are not supported.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags that you apply to the resource to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in <i>AWS General Reference</i>.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Contains the parameters for <code>TerminateJob</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TerminateJobRequest {
    /// <p>The AWS Batch job ID of the job to terminate.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A message to attach to the job that explains the reason for canceling it. This message is returned by future <a>DescribeJobs</a> operations on the job. This message is also recorded in the AWS Batch activity logs.</p>
    #[serde(rename = "reason")]
    pub reason: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminateJobResponse {}

/// <p><p>The container path, mount options, and size of the tmpfs mount.</p> <note> <p>This object isn&#39;t applicable to jobs running on Fargate resources.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tmpfs {
    /// <p>The absolute file path in the container where the tmpfs volume is mounted.</p>
    #[serde(rename = "containerPath")]
    pub container_path: String,
    /// <p>The list of tmpfs volume mount options.</p> <p>Valid values: "<code>defaults</code>" | "<code>ro</code>" | "<code>rw</code>" | "<code>suid</code>" | "<code>nosuid</code>" | "<code>dev</code>" | "<code>nodev</code>" | "<code>exec</code>" | "<code>noexec</code>" | "<code>sync</code>" | "<code>async</code>" | "<code>dirsync</code>" | "<code>remount</code>" | "<code>mand</code>" | "<code>nomand</code>" | "<code>atime</code>" | "<code>noatime</code>" | "<code>diratime</code>" | "<code>nodiratime</code>" | "<code>bind</code>" | "<code>rbind" | "unbindable" | "runbindable" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave" | "relatime</code>" | "<code>norelatime</code>" | "<code>strictatime</code>" | "<code>nostrictatime</code>" | "<code>mode</code>" | "<code>uid</code>" | "<code>gid</code>" | "<code>nr_inodes</code>" | "<code>nr_blocks</code>" | "<code>mpol</code>"</p>
    #[serde(rename = "mountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,
    /// <p>The size (in MiB) of the tmpfs volume.</p>
    #[serde(rename = "size")]
    pub size: i64,
}

/// <p><p>The <code>ulimit</code> settings to pass to the container.</p> <note> <p>This object isn&#39;t applicable to jobs running on Fargate resources.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ulimit {
    /// <p>The hard limit for the <code>ulimit</code> type.</p>
    #[serde(rename = "hardLimit")]
    pub hard_limit: i64,
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The soft limit for the <code>ulimit</code> type.</p>
    #[serde(rename = "softLimit")]
    pub soft_limit: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource from which to delete tags. AWS Batch resources that support tags are compute environments, jobs, job definitions, and job queues. ARNs for child jobs of array and multi-node parallel (MNP) jobs are not supported.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>Contains the parameters for <code>UpdateComputeEnvironment</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateComputeEnvironmentRequest {
    /// <p>The name or full Amazon Resource Name (ARN) of the compute environment to update.</p>
    #[serde(rename = "computeEnvironment")]
    pub compute_environment: String,
    /// <p>Details of the compute resources managed by the compute environment. Required for a managed compute environment. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/compute_environments.html">Compute Environments</a> in the <i>AWS Batch User Guide</i>.</p>
    #[serde(rename = "computeResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResourceUpdate>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/service_IAM_role.html">AWS Batch service IAM role</a> in the <i>AWS Batch User Guide</i>.</p> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path.</p> <note> <p>Depending on how you created your AWS Batch service role, its ARN might contain the <code>service-role</code> path prefix. When you only specify the name of the service role, AWS Batch assumes that your ARN does not use the <code>service-role</code> path prefix. Because of this, we recommend that you specify the full ARN of your service role when you create compute environments.</p> </note></p>
    #[serde(rename = "serviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    /// <p>The state of the compute environment. Compute environments in the <code>ENABLED</code> state can accept jobs from a queue and scale in or out automatically based on the workload demand of its associated queues.</p> <p>If the state is <code>ENABLED</code>, then the AWS Batch scheduler can attempt to place jobs from an associated job queue on the compute resources within the environment. If the compute environment is managed, then it can scale its instances out or in automatically, based on the job queue demand.</p> <p>If the state is <code>DISABLED</code>, then the AWS Batch scheduler doesn't attempt to place jobs within the environment. Jobs in a <code>STARTING</code> or <code>RUNNING</code> state continue to progress normally. Managed compute environments in the <code>DISABLED</code> state don't scale out. However, they scale in to <code>minvCpus</code> value after instances become idle.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CEState>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateComputeEnvironmentResponse {
    /// <p>The Amazon Resource Name (ARN) of the compute environment.</p>
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    /// <p>The name of the compute environment. Up to 128 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "computeEnvironmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
}

/// <p>Contains the parameters for <code>UpdateJobQueue</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateJobQueueRequest {
    /// <p><p>Details the set of compute environments mapped to a job queue and their order relative to each other. This is one of the parameters used by the job scheduler to determine which compute environment should run a given job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can&#39;t be mixed.</p> <note> <p>All compute environments that are associated with a job queue must share the same architecture. AWS Batch doesn&#39;t support mixing compute environment architecture types in a single job queue.</p> </note></p>
    #[serde(rename = "computeEnvironmentOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_order: Option<Vec<ComputeEnvironmentOrder>>,
    /// <p>The name or the Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueue")]
    pub job_queue: String,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order, for example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments cannot be mixed.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Describes the queue's ability to accept new jobs. If the job queue state is <code>ENABLED</code>, it is able to accept jobs. If the job queue state is <code>DISABLED</code>, new jobs cannot be added to the queue, but jobs already in the queue can finish.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<JQState>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateJobQueueResponse {
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[serde(rename = "jobQueueArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_arn: Option<String>,
    /// <p>The name of the job queue.</p>
    #[serde(rename = "jobQueueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_name: Option<String>,
}

/// <p>A data volume used in a job's container properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Volume {
    /// <p><p>The contents of the <code>host</code> parameter determine whether your data volume persists on the host container instance and where it is stored. If the host parameter is empty, then the Docker daemon assigns a host path for your data volume. However, the data isn&#39;t guaranteed to persist after the containers associated with it stop running.</p> <note> <p>This parameter isn&#39;t applicable to jobs running on Fargate resources and shouldn&#39;t be provided.</p> </note></p>
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Host>,
    /// <p>The name of the volume. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. This name is referenced in the <code>sourceVolume</code> parameter of container definition <code>mountPoints</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl CancelJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(CancelJobError::Client(err.msg)),
                "ServerException" => return RusotoError::Service(CancelJobError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelJobError::Client(ref cause) => write!(f, "{}", cause),
            CancelJobError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelJobError {}
/// Errors returned by CreateComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum CreateComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl CreateComputeEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateComputeEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateComputeEnvironmentError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateComputeEnvironmentError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateComputeEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateComputeEnvironmentError::Client(ref cause) => write!(f, "{}", cause),
            CreateComputeEnvironmentError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateComputeEnvironmentError {}
/// Errors returned by CreateJobQueue
#[derive(Debug, PartialEq)]
pub enum CreateJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl CreateJobQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateJobQueueError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateJobQueueError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateJobQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJobQueueError::Client(ref cause) => write!(f, "{}", cause),
            CreateJobQueueError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJobQueueError {}
/// Errors returned by DeleteComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum DeleteComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeleteComputeEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteComputeEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteComputeEnvironmentError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteComputeEnvironmentError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteComputeEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteComputeEnvironmentError::Client(ref cause) => write!(f, "{}", cause),
            DeleteComputeEnvironmentError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteComputeEnvironmentError {}
/// Errors returned by DeleteJobQueue
#[derive(Debug, PartialEq)]
pub enum DeleteJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeleteJobQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJobQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteJobQueueError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteJobQueueError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteJobQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteJobQueueError::Client(ref cause) => write!(f, "{}", cause),
            DeleteJobQueueError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteJobQueueError {}
/// Errors returned by DeregisterJobDefinition
#[derive(Debug, PartialEq)]
pub enum DeregisterJobDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeregisterJobDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterJobDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeregisterJobDefinitionError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeregisterJobDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterJobDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterJobDefinitionError::Client(ref cause) => write!(f, "{}", cause),
            DeregisterJobDefinitionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterJobDefinitionError {}
/// Errors returned by DescribeComputeEnvironments
#[derive(Debug, PartialEq)]
pub enum DescribeComputeEnvironmentsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeComputeEnvironmentsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComputeEnvironmentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeComputeEnvironmentsError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeComputeEnvironmentsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComputeEnvironmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComputeEnvironmentsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeComputeEnvironmentsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeComputeEnvironmentsError {}
/// Errors returned by DescribeJobDefinitions
#[derive(Debug, PartialEq)]
pub enum DescribeJobDefinitionsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeJobDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobDefinitionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeJobDefinitionsError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeJobDefinitionsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeJobDefinitionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobDefinitionsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeJobDefinitionsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobDefinitionsError {}
/// Errors returned by DescribeJobQueues
#[derive(Debug, PartialEq)]
pub enum DescribeJobQueuesError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeJobQueuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeJobQueuesError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeJobQueuesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeJobQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobQueuesError::Client(ref cause) => write!(f, "{}", cause),
            DescribeJobQueuesError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobQueuesError {}
/// Errors returned by DescribeJobs
#[derive(Debug, PartialEq)]
pub enum DescribeJobsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeJobsError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeJobsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobsError::Client(ref cause) => write!(f, "{}", cause),
            DescribeJobsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobsError {}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(ListJobsError::Client(err.msg)),
                "ServerException" => return RusotoError::Service(ListJobsError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobsError::Client(ref cause) => write!(f, "{}", cause),
            ListJobsError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTagsForResourceError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::Server(err.msg))
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
            ListTagsForResourceError::Client(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RegisterJobDefinition
#[derive(Debug, PartialEq)]
pub enum RegisterJobDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl RegisterJobDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterJobDefinitionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RegisterJobDefinitionError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(RegisterJobDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterJobDefinitionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterJobDefinitionError::Client(ref cause) => write!(f, "{}", cause),
            RegisterJobDefinitionError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterJobDefinitionError {}
/// Errors returned by SubmitJob
#[derive(Debug, PartialEq)]
pub enum SubmitJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl SubmitJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubmitJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(SubmitJobError::Client(err.msg)),
                "ServerException" => return RusotoError::Service(SubmitJobError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SubmitJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubmitJobError::Client(ref cause) => write!(f, "{}", cause),
            SubmitJobError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubmitJobError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(TagResourceError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(TagResourceError::Server(err.msg))
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
            TagResourceError::Client(ref cause) => write!(f, "{}", cause),
            TagResourceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by TerminateJob
#[derive(Debug, PartialEq)]
pub enum TerminateJobError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl TerminateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TerminateJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(TerminateJobError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(TerminateJobError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TerminateJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TerminateJobError::Client(ref cause) => write!(f, "{}", cause),
            TerminateJobError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TerminateJobError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UntagResourceError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UntagResourceError::Server(err.msg))
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
            UntagResourceError::Client(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateComputeEnvironment
#[derive(Debug, PartialEq)]
pub enum UpdateComputeEnvironmentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateComputeEnvironmentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateComputeEnvironmentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateComputeEnvironmentError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateComputeEnvironmentError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateComputeEnvironmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateComputeEnvironmentError::Client(ref cause) => write!(f, "{}", cause),
            UpdateComputeEnvironmentError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateComputeEnvironmentError {}
/// Errors returned by UpdateJobQueue
#[derive(Debug, PartialEq)]
pub enum UpdateJobQueueError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that's not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateJobQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateJobQueueError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateJobQueueError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateJobQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateJobQueueError::Client(ref cause) => write!(f, "{}", cause),
            UpdateJobQueueError::Server(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateJobQueueError {}
/// Trait representing the capabilities of the AWS Batch API. AWS Batch clients implement this trait.
#[async_trait]
pub trait Batch {
    /// <p>Cancels a job in an AWS Batch job queue. Jobs that are in the <code>SUBMITTED</code>, <code>PENDING</code>, or <code>RUNNABLE</code> state are canceled. Jobs that have progressed to <code>STARTING</code> or <code>RUNNING</code> are not canceled (but the API operation still succeeds, even if no job is canceled); these jobs must be terminated with the <a>TerminateJob</a> operation.</p>
    async fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> Result<CancelJobResponse, RusotoError<CancelJobError>>;

    /// <p><p>Creates an AWS Batch compute environment. You can create <code>MANAGED</code> or <code>UNMANAGED</code> compute environments. <code>MANAGED</code> compute environments can use Amazon EC2 or AWS Fargate resources. <code>UNMANAGED</code> compute environments can only use EC2 resources.</p> <p>In a managed compute environment, AWS Batch manages the capacity and instance types of the compute resources within the environment. This is based on the compute resource specification that you define or the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html">launch template</a> that you specify when you create the compute environment. You can choose either to use EC2 On-Demand Instances and EC2 Spot Instances, or to use Fargate and Fargate Spot capacity in your managed compute environment. You can optionally set a maximum price so that Spot Instances only launch when the Spot Instance price is less than a specified percentage of the On-Demand price.</p> <note> <p>Multi-node parallel jobs are not supported on Spot Instances.</p> </note> <p>In an unmanaged compute environment, you can manage your own EC2 compute resources and have a lot of flexibility with how you configure your compute resources. For example, you can use custom AMI. However, you need to verify that your AMI meets the Amazon ECS container instance AMI specification. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container_instance_AMIs.html">container instance AMIs</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. After you have created your unmanaged compute environment, you can use the <a>DescribeComputeEnvironments</a> operation to find the Amazon ECS cluster that&#39;s associated with it. Then, manually launch your container instances into that Amazon ECS cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_container_instance.html">Launching an Amazon ECS container instance</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>AWS Batch doesn&#39;t upgrade the AMIs in a compute environment after it&#39;s created. For example, it doesn&#39;t update the AMIs when a newer version of the Amazon ECS-optimized AMI is available. Therefore, you&#39;re responsible for the management of the guest operating system (including updates and security patches) and any additional application software or utilities that you install on the compute resources. To use a new AMI for your AWS Batch jobs, complete these steps:</p> <ol> <li> <p>Create a new compute environment with the new AMI.</p> </li> <li> <p>Add the compute environment to an existing job queue.</p> </li> <li> <p>Remove the earlier compute environment from your job queue.</p> </li> <li> <p>Delete the earlier compute environment.</p> </li> </ol> </note></p>
    async fn create_compute_environment(
        &self,
        input: CreateComputeEnvironmentRequest,
    ) -> Result<CreateComputeEnvironmentResponse, RusotoError<CreateComputeEnvironmentError>>;

    /// <p>Creates an AWS Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments.</p> <p>You also set a priority to the job queue that determines the order in which the AWS Batch scheduler places jobs onto its associated compute environments. For example, if a compute environment is associated with more than one job queue, the job queue with a higher priority is given preference for scheduling jobs to that compute environment.</p>
    async fn create_job_queue(
        &self,
        input: CreateJobQueueRequest,
    ) -> Result<CreateJobQueueResponse, RusotoError<CreateJobQueueError>>;

    /// <p>Deletes an AWS Batch compute environment.</p> <p>Before you can delete a compute environment, you must set its state to <code>DISABLED</code> with the <a>UpdateComputeEnvironment</a> API operation and disassociate it from any job queues with the <a>UpdateJobQueue</a> API operation. Compute environments that use AWS Fargate resources must terminate all active jobs on that compute environment before deleting the compute environment. If this isn't done, the compute environment will end up in an invalid state.</p>
    async fn delete_compute_environment(
        &self,
        input: DeleteComputeEnvironmentRequest,
    ) -> Result<DeleteComputeEnvironmentResponse, RusotoError<DeleteComputeEnvironmentError>>;

    /// <p>Deletes the specified job queue. You must first disable submissions for a queue with the <a>UpdateJobQueue</a> operation. All jobs in the queue are eventually terminated when you delete a job queue. The jobs are terminated at a rate of about 16 jobs each second.</p> <p>It's not necessary to disassociate compute environments from a queue before submitting a <code>DeleteJobQueue</code> request.</p>
    async fn delete_job_queue(
        &self,
        input: DeleteJobQueueRequest,
    ) -> Result<DeleteJobQueueResponse, RusotoError<DeleteJobQueueError>>;

    /// <p>Deregisters an AWS Batch job definition. Job definitions are permanently deleted after 180 days.</p>
    async fn deregister_job_definition(
        &self,
        input: DeregisterJobDefinitionRequest,
    ) -> Result<DeregisterJobDefinitionResponse, RusotoError<DeregisterJobDefinitionError>>;

    /// <p>Describes one or more of your compute environments.</p> <p>If you're using an unmanaged compute environment, you can use the <code>DescribeComputeEnvironment</code> operation to determine the <code>ecsClusterArn</code> that you should launch your Amazon ECS container instances into.</p>
    async fn describe_compute_environments(
        &self,
        input: DescribeComputeEnvironmentsRequest,
    ) -> Result<DescribeComputeEnvironmentsResponse, RusotoError<DescribeComputeEnvironmentsError>>;

    /// <p>Describes a list of job definitions. You can specify a <code>status</code> (such as <code>ACTIVE</code>) to only return job definitions that match that status.</p>
    async fn describe_job_definitions(
        &self,
        input: DescribeJobDefinitionsRequest,
    ) -> Result<DescribeJobDefinitionsResponse, RusotoError<DescribeJobDefinitionsError>>;

    /// <p>Describes one or more of your job queues.</p>
    async fn describe_job_queues(
        &self,
        input: DescribeJobQueuesRequest,
    ) -> Result<DescribeJobQueuesResponse, RusotoError<DescribeJobQueuesError>>;

    /// <p>Describes a list of AWS Batch jobs.</p>
    async fn describe_jobs(
        &self,
        input: DescribeJobsRequest,
    ) -> Result<DescribeJobsResponse, RusotoError<DescribeJobsError>>;

    /// <p>Returns a list of AWS Batch jobs.</p> <p>You must specify only one of the following items:</p> <ul> <li> <p>A job queue ID to return a list of jobs in that job queue</p> </li> <li> <p>A multi-node parallel job ID to return a list of that job's nodes</p> </li> <li> <p>An array job ID to return a list of that job's children</p> </li> </ul> <p>You can filter the results by job status with the <code>jobStatus</code> parameter. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>>;

    /// <p>Lists the tags for an AWS Batch resource. AWS Batch resources that support tags are compute environments, jobs, job definitions, and job queues. ARNs for child jobs of array and multi-node parallel (MNP) jobs are not supported.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Registers an AWS Batch job definition.</p>
    async fn register_job_definition(
        &self,
        input: RegisterJobDefinitionRequest,
    ) -> Result<RegisterJobDefinitionResponse, RusotoError<RegisterJobDefinitionError>>;

    /// <p><p>Submits an AWS Batch job from a job definition. Parameters specified during <a>SubmitJob</a> override parameters defined in the job definition.</p> <important> <p>Jobs run on Fargate resources don&#39;t run for more than 14 days. After 14 days, the Fargate resources might no longer be available and the job is terminated.</p> </important></p>
    async fn submit_job(
        &self,
        input: SubmitJobRequest,
    ) -> Result<SubmitJobResponse, RusotoError<SubmitJobError>>;

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags associated with that resource are deleted as well. AWS Batch resources that support tags are compute environments, jobs, job definitions, and job queues. ARNs for child jobs of array and multi-node parallel (MNP) jobs are not supported.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Terminates a job in a job queue. Jobs that are in the <code>STARTING</code> or <code>RUNNING</code> state are terminated, which causes them to transition to <code>FAILED</code>. Jobs that have not progressed to the <code>STARTING</code> state are cancelled.</p>
    async fn terminate_job(
        &self,
        input: TerminateJobRequest,
    ) -> Result<TerminateJobResponse, RusotoError<TerminateJobError>>;

    /// <p>Deletes specified tags from an AWS Batch resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates an AWS Batch compute environment.</p>
    async fn update_compute_environment(
        &self,
        input: UpdateComputeEnvironmentRequest,
    ) -> Result<UpdateComputeEnvironmentResponse, RusotoError<UpdateComputeEnvironmentError>>;

    /// <p>Updates a job queue.</p>
    async fn update_job_queue(
        &self,
        input: UpdateJobQueueRequest,
    ) -> Result<UpdateJobQueueResponse, RusotoError<UpdateJobQueueError>>;
}
/// A client for the AWS Batch API.
#[derive(Clone)]
pub struct BatchClient {
    client: Client,
    region: region::Region,
}

impl BatchClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> BatchClient {
        BatchClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> BatchClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        BatchClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> BatchClient {
        BatchClient { client, region }
    }
}

#[async_trait]
impl Batch for BatchClient {
    /// <p>Cancels a job in an AWS Batch job queue. Jobs that are in the <code>SUBMITTED</code>, <code>PENDING</code>, or <code>RUNNABLE</code> state are canceled. Jobs that have progressed to <code>STARTING</code> or <code>RUNNING</code> are not canceled (but the API operation still succeeds, even if no job is canceled); these jobs must be terminated with the <a>TerminateJob</a> operation.</p>
    #[allow(unused_mut)]
    async fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> Result<CancelJobResponse, RusotoError<CancelJobError>> {
        let request_uri = "/v1/canceljob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelJobError::from_response(response))
        }
    }

    /// <p><p>Creates an AWS Batch compute environment. You can create <code>MANAGED</code> or <code>UNMANAGED</code> compute environments. <code>MANAGED</code> compute environments can use Amazon EC2 or AWS Fargate resources. <code>UNMANAGED</code> compute environments can only use EC2 resources.</p> <p>In a managed compute environment, AWS Batch manages the capacity and instance types of the compute resources within the environment. This is based on the compute resource specification that you define or the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html">launch template</a> that you specify when you create the compute environment. You can choose either to use EC2 On-Demand Instances and EC2 Spot Instances, or to use Fargate and Fargate Spot capacity in your managed compute environment. You can optionally set a maximum price so that Spot Instances only launch when the Spot Instance price is less than a specified percentage of the On-Demand price.</p> <note> <p>Multi-node parallel jobs are not supported on Spot Instances.</p> </note> <p>In an unmanaged compute environment, you can manage your own EC2 compute resources and have a lot of flexibility with how you configure your compute resources. For example, you can use custom AMI. However, you need to verify that your AMI meets the Amazon ECS container instance AMI specification. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container_instance_AMIs.html">container instance AMIs</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. After you have created your unmanaged compute environment, you can use the <a>DescribeComputeEnvironments</a> operation to find the Amazon ECS cluster that&#39;s associated with it. Then, manually launch your container instances into that Amazon ECS cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_container_instance.html">Launching an Amazon ECS container instance</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>AWS Batch doesn&#39;t upgrade the AMIs in a compute environment after it&#39;s created. For example, it doesn&#39;t update the AMIs when a newer version of the Amazon ECS-optimized AMI is available. Therefore, you&#39;re responsible for the management of the guest operating system (including updates and security patches) and any additional application software or utilities that you install on the compute resources. To use a new AMI for your AWS Batch jobs, complete these steps:</p> <ol> <li> <p>Create a new compute environment with the new AMI.</p> </li> <li> <p>Add the compute environment to an existing job queue.</p> </li> <li> <p>Remove the earlier compute environment from your job queue.</p> </li> <li> <p>Delete the earlier compute environment.</p> </li> </ol> </note></p>
    #[allow(unused_mut)]
    async fn create_compute_environment(
        &self,
        input: CreateComputeEnvironmentRequest,
    ) -> Result<CreateComputeEnvironmentResponse, RusotoError<CreateComputeEnvironmentError>> {
        let request_uri = "/v1/createcomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateComputeEnvironmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateComputeEnvironmentError::from_response(response))
        }
    }

    /// <p>Creates an AWS Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments.</p> <p>You also set a priority to the job queue that determines the order in which the AWS Batch scheduler places jobs onto its associated compute environments. For example, if a compute environment is associated with more than one job queue, the job queue with a higher priority is given preference for scheduling jobs to that compute environment.</p>
    #[allow(unused_mut)]
    async fn create_job_queue(
        &self,
        input: CreateJobQueueRequest,
    ) -> Result<CreateJobQueueResponse, RusotoError<CreateJobQueueError>> {
        let request_uri = "/v1/createjobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateJobQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateJobQueueError::from_response(response))
        }
    }

    /// <p>Deletes an AWS Batch compute environment.</p> <p>Before you can delete a compute environment, you must set its state to <code>DISABLED</code> with the <a>UpdateComputeEnvironment</a> API operation and disassociate it from any job queues with the <a>UpdateJobQueue</a> API operation. Compute environments that use AWS Fargate resources must terminate all active jobs on that compute environment before deleting the compute environment. If this isn't done, the compute environment will end up in an invalid state.</p>
    #[allow(unused_mut)]
    async fn delete_compute_environment(
        &self,
        input: DeleteComputeEnvironmentRequest,
    ) -> Result<DeleteComputeEnvironmentResponse, RusotoError<DeleteComputeEnvironmentError>> {
        let request_uri = "/v1/deletecomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteComputeEnvironmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteComputeEnvironmentError::from_response(response))
        }
    }

    /// <p>Deletes the specified job queue. You must first disable submissions for a queue with the <a>UpdateJobQueue</a> operation. All jobs in the queue are eventually terminated when you delete a job queue. The jobs are terminated at a rate of about 16 jobs each second.</p> <p>It's not necessary to disassociate compute environments from a queue before submitting a <code>DeleteJobQueue</code> request.</p>
    #[allow(unused_mut)]
    async fn delete_job_queue(
        &self,
        input: DeleteJobQueueRequest,
    ) -> Result<DeleteJobQueueResponse, RusotoError<DeleteJobQueueError>> {
        let request_uri = "/v1/deletejobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteJobQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteJobQueueError::from_response(response))
        }
    }

    /// <p>Deregisters an AWS Batch job definition. Job definitions are permanently deleted after 180 days.</p>
    #[allow(unused_mut)]
    async fn deregister_job_definition(
        &self,
        input: DeregisterJobDefinitionRequest,
    ) -> Result<DeregisterJobDefinitionResponse, RusotoError<DeregisterJobDefinitionError>> {
        let request_uri = "/v1/deregisterjobdefinition";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeregisterJobDefinitionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeregisterJobDefinitionError::from_response(response))
        }
    }

    /// <p>Describes one or more of your compute environments.</p> <p>If you're using an unmanaged compute environment, you can use the <code>DescribeComputeEnvironment</code> operation to determine the <code>ecsClusterArn</code> that you should launch your Amazon ECS container instances into.</p>
    #[allow(unused_mut)]
    async fn describe_compute_environments(
        &self,
        input: DescribeComputeEnvironmentsRequest,
    ) -> Result<DescribeComputeEnvironmentsResponse, RusotoError<DescribeComputeEnvironmentsError>>
    {
        let request_uri = "/v1/describecomputeenvironments";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeComputeEnvironmentsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeComputeEnvironmentsError::from_response(response))
        }
    }

    /// <p>Describes a list of job definitions. You can specify a <code>status</code> (such as <code>ACTIVE</code>) to only return job definitions that match that status.</p>
    #[allow(unused_mut)]
    async fn describe_job_definitions(
        &self,
        input: DescribeJobDefinitionsRequest,
    ) -> Result<DescribeJobDefinitionsResponse, RusotoError<DescribeJobDefinitionsError>> {
        let request_uri = "/v1/describejobdefinitions";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeJobDefinitionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobDefinitionsError::from_response(response))
        }
    }

    /// <p>Describes one or more of your job queues.</p>
    #[allow(unused_mut)]
    async fn describe_job_queues(
        &self,
        input: DescribeJobQueuesRequest,
    ) -> Result<DescribeJobQueuesResponse, RusotoError<DescribeJobQueuesError>> {
        let request_uri = "/v1/describejobqueues";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeJobQueuesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobQueuesError::from_response(response))
        }
    }

    /// <p>Describes a list of AWS Batch jobs.</p>
    #[allow(unused_mut)]
    async fn describe_jobs(
        &self,
        input: DescribeJobsRequest,
    ) -> Result<DescribeJobsResponse, RusotoError<DescribeJobsError>> {
        let request_uri = "/v1/describejobs";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobsError::from_response(response))
        }
    }

    /// <p>Returns a list of AWS Batch jobs.</p> <p>You must specify only one of the following items:</p> <ul> <li> <p>A job queue ID to return a list of jobs in that job queue</p> </li> <li> <p>A multi-node parallel job ID to return a list of that job's nodes</p> </li> <li> <p>An array job ID to return a list of that job's children</p> </li> </ul> <p>You can filter the results by job status with the <code>jobStatus</code> parameter. If you don't specify a status, only <code>RUNNING</code> jobs are returned.</p>
    #[allow(unused_mut)]
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>> {
        let request_uri = "/v1/listjobs";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobsError::from_response(response))
        }
    }

    /// <p>Lists the tags for an AWS Batch resource. AWS Batch resources that support tags are compute environments, jobs, job definitions, and job queues. ARNs for child jobs of array and multi-node parallel (MNP) jobs are not supported.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Registers an AWS Batch job definition.</p>
    #[allow(unused_mut)]
    async fn register_job_definition(
        &self,
        input: RegisterJobDefinitionRequest,
    ) -> Result<RegisterJobDefinitionResponse, RusotoError<RegisterJobDefinitionError>> {
        let request_uri = "/v1/registerjobdefinition";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RegisterJobDefinitionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterJobDefinitionError::from_response(response))
        }
    }

    /// <p><p>Submits an AWS Batch job from a job definition. Parameters specified during <a>SubmitJob</a> override parameters defined in the job definition.</p> <important> <p>Jobs run on Fargate resources don&#39;t run for more than 14 days. After 14 days, the Fargate resources might no longer be available and the job is terminated.</p> </important></p>
    #[allow(unused_mut)]
    async fn submit_job(
        &self,
        input: SubmitJobRequest,
    ) -> Result<SubmitJobResponse, RusotoError<SubmitJobError>> {
        let request_uri = "/v1/submitjob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SubmitJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SubmitJobError::from_response(response))
        }
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags associated with that resource are deleted as well. AWS Batch resources that support tags are compute environments, jobs, job definitions, and job queues. ARNs for child jobs of array and multi-node parallel (MNP) jobs are not supported.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Terminates a job in a job queue. Jobs that are in the <code>STARTING</code> or <code>RUNNING</code> state are terminated, which causes them to transition to <code>FAILED</code>. Jobs that have not progressed to the <code>STARTING</code> state are cancelled.</p>
    #[allow(unused_mut)]
    async fn terminate_job(
        &self,
        input: TerminateJobRequest,
    ) -> Result<TerminateJobResponse, RusotoError<TerminateJobError>> {
        let request_uri = "/v1/terminatejob";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TerminateJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TerminateJobError::from_response(response))
        }
    }

    /// <p>Deletes specified tags from an AWS Batch resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/v1/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "batch", &self.region, &request_uri);
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates an AWS Batch compute environment.</p>
    #[allow(unused_mut)]
    async fn update_compute_environment(
        &self,
        input: UpdateComputeEnvironmentRequest,
    ) -> Result<UpdateComputeEnvironmentResponse, RusotoError<UpdateComputeEnvironmentError>> {
        let request_uri = "/v1/updatecomputeenvironment";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateComputeEnvironmentResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateComputeEnvironmentError::from_response(response))
        }
    }

    /// <p>Updates a job queue.</p>
    #[allow(unused_mut)]
    async fn update_job_queue(
        &self,
        input: UpdateJobQueueRequest,
    ) -> Result<UpdateJobQueueResponse, RusotoError<UpdateJobQueueError>> {
        let request_uri = "/v1/updatejobqueue";

        let mut request = SignedRequest::new("POST", "batch", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateJobQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateJobQueueError::from_response(response))
        }
    }
}
