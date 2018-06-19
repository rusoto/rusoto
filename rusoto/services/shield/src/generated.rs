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
/// <p>The details of a DDoS attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttackDetail {
    /// <p>List of counters that describe the attack for the specified time period.</p>
    #[serde(rename = "AttackCounters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_counters: Option<Vec<SummarizedCounter>>,
    /// <p>The unique identifier (ID) of the attack.</p>
    #[serde(rename = "AttackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_id: Option<String>,
    /// <p>The array of <a>AttackProperty</a> objects.</p>
    #[serde(rename = "AttackProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_properties: Option<Vec<AttackProperty>>,
    /// <p>The time the attack ended, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>List of mitigation actions taken for the attack.</p>
    #[serde(rename = "Mitigations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigations: Option<Vec<Mitigation>>,
    /// <p>The ARN (Amazon Resource Name) of the resource that was attacked.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The time the attack started, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>If applicable, additional detail about the resource being attacked, for example, IP address or URL.</p>
    #[serde(rename = "SubResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_resources: Option<Vec<SubResourceSummary>>,
}

/// <p>Details of the described attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttackProperty {
    /// <p>The type of DDoS event that was observed. <code>NETWORK</code> indicates layer 3 and layer 4 events and <code>APPLICATION</code> indicates layer 7 events.</p>
    #[serde(rename = "AttackLayer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_layer: Option<String>,
    /// <p>Defines the DDoS attack property information that is provided.</p>
    #[serde(rename = "AttackPropertyIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_property_identifier: Option<String>,
    /// <p>The array of <a>Contributor</a> objects that includes the top five contributors to an attack. </p>
    #[serde(rename = "TopContributors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_contributors: Option<Vec<Contributor>>,
    /// <p>The total contributions made to this attack by all contributors, not just the five listed in the <code>TopContributors</code> list.</p>
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// <p>The unit of the <code>Value</code> of the contributions.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Summarizes all DDoS attacks for a specified time period.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttackSummary {
    /// <p>The unique identifier (ID) of the attack.</p>
    #[serde(rename = "AttackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_id: Option<String>,
    /// <p>The list of attacks for a specified time period.</p>
    #[serde(rename = "AttackVectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_vectors: Option<Vec<AttackVectorDescription>>,
    /// <p>The end time of the attack, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The ARN (Amazon Resource Name) of the resource that was attacked.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The start time of the attack, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p>Describes the attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AttackVectorDescription {
    /// <p><p>The attack type. Valid values:</p> <ul> <li> <p>UDP<em>TRAFFIC</p> </li> <li> <p>UDP</em>FRAGMENT</p> </li> <li> <p>GENERIC<em>UDP</em>REFLECTION</p> </li> <li> <p>DNS<em>REFLECTION</p> </li> <li> <p>NTP</em>REFLECTION</p> </li> <li> <p>CHARGEN<em>REFLECTION</p> </li> <li> <p>SSDP</em>REFLECTION</p> </li> <li> <p>PORT<em>MAPPER</p> </li> <li> <p>RIP</em>REFLECTION</p> </li> <li> <p>SNMP<em>REFLECTION</p> </li> <li> <p>MSSQL</em>REFLECTION</p> </li> <li> <p>NET<em>BIOS</em>REFLECTION</p> </li> <li> <p>SYN<em>FLOOD</p> </li> <li> <p>ACK</em>FLOOD</p> </li> <li> <p>REQUEST_FLOOD</p> </li> </ul></p>
    #[serde(rename = "VectorType")]
    pub vector_type: String,
}

/// <p>A contributor to the attack and their contribution.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Contributor {
    /// <p>The name of the contributor. This is dependent on the <code>AttackPropertyIdentifier</code>. For example, if the <code>AttackPropertyIdentifier</code> is <code>SOURCE_COUNTRY</code>, the <code>Name</code> could be <code>United States</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The contribution of this contributor expressed in <a>Protection</a> units. For example <code>10,000</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProtectionRequest {
    /// <p>Friendly name for the <code>Protection</code> you are creating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The ARN (Amazon Resource Name) of the resource to be protected.</p> <p>The ARN should be in one of the following formats:</p> <ul> <li> <p>For an Application Load Balancer: <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/app/<i>load-balancer-name</i>/<i>load-balancer-id</i> </code> </p> </li> <li> <p>For an Elastic Load Balancer (Classic Load Balancer): <code>arn:aws:elasticloadbalancing:<i>region</i>:<i>account-id</i>:loadbalancer/<i>load-balancer-name</i> </code> </p> </li> <li> <p>For AWS CloudFront distribution: <code>arn:aws:cloudfront::<i>account-id</i>:distribution/<i>distribution-id</i> </code> </p> </li> <li> <p>For Amazon Route 53: <code>arn:aws:route53::<i>account-id</i>:hostedzone/<i>hosted-zone-id</i> </code> </p> </li> <li> <p>For an Elastic IP address: <code>arn:aws:ec2:<i>region</i>:<i>account-id</i>:eip-allocation/<i>allocation-id</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateProtectionResponse {
    /// <p>The unique identifier (ID) for the <a>Protection</a> object that is created.</p>
    #[serde(rename = "ProtectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateSubscriptionRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateSubscriptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProtectionRequest {
    /// <p>The unique identifier (ID) for the <a>Protection</a> object to be deleted.</p>
    #[serde(rename = "ProtectionId")]
    pub protection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteProtectionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteSubscriptionRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteSubscriptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAttackRequest {
    /// <p>The unique identifier (ID) for the attack that to be described.</p>
    #[serde(rename = "AttackId")]
    pub attack_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeAttackResponse {
    /// <p>The attack that is described.</p>
    #[serde(rename = "Attack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<AttackDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeProtectionRequest {
    /// <p>The unique identifier (ID) for the <a>Protection</a> object that is described.</p>
    #[serde(rename = "ProtectionId")]
    pub protection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeProtectionResponse {
    /// <p>The <a>Protection</a> object that is described.</p>
    #[serde(rename = "Protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<Protection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSubscriptionRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeSubscriptionResponse {
    /// <p>The AWS Shield Advanced subscription details for an account.</p>
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSubscriptionStateRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSubscriptionStateResponse {
    /// <p>The status of the subscription.</p>
    #[serde(rename = "SubscriptionState")]
    pub subscription_state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAttacksRequest {
    /// <p>The end of the time period for the attacks. This is a <code>timestamp</code> type. The sample request above indicates a <code>number</code> type because the default used by WAF is Unix time in seconds. However any valid <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp format</a> is allowed. </p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeRange>,
    /// <p>The maximum number of <a>AttackSummary</a> objects to be returned. If this is left blank, the first 20 results will be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>ListAttacksRequest.NextMarker</code> value from a previous call to <code>ListAttacksRequest</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN (Amazon Resource Name) of the resource that was attacked. If this is left blank, all applicable resources for this account will be included.</p>
    #[serde(rename = "ResourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    /// <p>The start of the time period for the attacks. This is a <code>timestamp</code> type. The sample request above indicates a <code>number</code> type because the default used by WAF is Unix time in seconds. However any valid <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp format</a> is allowed. </p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeRange>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAttacksResponse {
    /// <p>The attack information for the specified time range.</p>
    #[serde(rename = "AttackSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_summaries: Option<Vec<AttackSummary>>,
    /// <p>The token returned by a previous call to indicate that there is more data available. If not null, more results are available. Pass this value for the <code>NextMarker</code> parameter in a subsequent call to <code>ListAttacks</code> to retrieve the next set of items.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProtectionsRequest {
    /// <p>The maximum number of <a>Protection</a> objects to be returned. If this is left blank the first 20 results will be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>ListProtectionsRequest.NextToken</code> value from a previous call to <code>ListProtections</code>. Pass null if this is the first call.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListProtectionsResponse {
    /// <p>If you specify a value for <code>MaxResults</code> and you have more Protections than the value of MaxResults, AWS Shield Advanced returns a NextToken value in the response that allows you to list another group of Protections. For the second and subsequent ListProtections requests, specify the value of NextToken from the previous response to get information about another batch of Protections.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The array of enabled <a>Protection</a> objects.</p>
    #[serde(rename = "Protections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protections: Option<Vec<Protection>>,
}

/// <p>The mitigation applied to a DDoS attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Mitigation {
    /// <p>The name of the mitigation taken for this attack.</p>
    #[serde(rename = "MitigationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_name: Option<String>,
}

/// <p>An object that represents a resource that is under DDoS protection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Protection {
    /// <p>The unique identifier (ID) of the protection.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of the protection. For example, <code>My CloudFront distributions</code>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN (Amazon Resource Name) of the AWS resource that is protected.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

/// <p>The attack information for the specified SubResource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SubResourceSummary {
    /// <p>The list of attack types and associated counters.</p>
    #[serde(rename = "AttackVectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_vectors: Option<Vec<SummarizedAttackVector>>,
    /// <p>The counters that describe the details of the attack.</p>
    #[serde(rename = "Counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<SummarizedCounter>>,
    /// <p>The unique identifier (ID) of the <code>SubResource</code>.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The <code>SubResource</code> type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information about the AWS Shield Advanced subscription for an account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Subscription {
    /// <p>The start time of the subscription, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The length, in seconds, of the AWS Shield Advanced subscription for the account.</p>
    #[serde(rename = "TimeCommitmentInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_commitment_in_seconds: Option<i64>,
}

/// <p>A summary of information about the attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SummarizedAttackVector {
    /// <p>The list of counters that describe the details of the attack.</p>
    #[serde(rename = "VectorCounters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_counters: Option<Vec<SummarizedCounter>>,
    /// <p>The attack type, for example, SNMP reflection or SYN flood.</p>
    #[serde(rename = "VectorType")]
    pub vector_type: String,
}

/// <p>The counter that describes a DDoS attack.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SummarizedCounter {
    /// <p>The average value of the counter for a specified time period.</p>
    #[serde(rename = "Average")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    /// <p>The maximum value of the counter for a specified time period.</p>
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// <p>The number of counters for a specified time period.</p>
    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// <p>The counter name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The total of counter values for a specified time period.</p>
    #[serde(rename = "Sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    /// <p>The unit of the counters.</p>
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>The time range.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TimeRange {
    /// <p>The start time, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "FromInclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_inclusive: Option<f64>,
    /// <p>The end time, in Unix time in seconds. For more information see <a href="http://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#parameter-types">timestamp</a>.</p>
    #[serde(rename = "ToExclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_exclusive: Option<f64>,
}

/// Errors returned by CreateProtection
#[derive(Debug, PartialEq)]
pub enum CreateProtectionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the resource is invalid. You might not have access to the resource, or the resource might not exist.</p>
    InvalidResource(String),
    /// <p>Exception that indicates that the operation would exceed a limit.</p> <p> <code>Type</code> is the type of limit that would be exceeded.</p> <p> <code>Limit</code> is the threshold that would be exceeded.</p>
    LimitsExceeded(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateProtectionError {
    pub fn from_body(body: &str) -> CreateProtectionError {
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
                    "InternalErrorException" => {
                        CreateProtectionError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        CreateProtectionError::InvalidOperation(String::from(error_message))
                    }
                    "InvalidResourceException" => {
                        CreateProtectionError::InvalidResource(String::from(error_message))
                    }
                    "LimitsExceededException" => {
                        CreateProtectionError::LimitsExceeded(String::from(error_message))
                    }
                    "OptimisticLockException" => {
                        CreateProtectionError::OptimisticLock(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateProtectionError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateProtectionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateProtectionError::Validation(error_message.to_string())
                    }
                    _ => CreateProtectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateProtectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateProtectionError {
    fn from(err: serde_json::error::Error) -> CreateProtectionError {
        CreateProtectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProtectionError {
    fn from(err: CredentialsError) -> CreateProtectionError {
        CreateProtectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProtectionError {
    fn from(err: HttpDispatchError) -> CreateProtectionError {
        CreateProtectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateProtectionError {
    fn from(err: io::Error) -> CreateProtectionError {
        CreateProtectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateProtectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProtectionError {
    fn description(&self) -> &str {
        match *self {
            CreateProtectionError::InternalError(ref cause) => cause,
            CreateProtectionError::InvalidOperation(ref cause) => cause,
            CreateProtectionError::InvalidResource(ref cause) => cause,
            CreateProtectionError::LimitsExceeded(ref cause) => cause,
            CreateProtectionError::OptimisticLock(ref cause) => cause,
            CreateProtectionError::ResourceAlreadyExists(ref cause) => cause,
            CreateProtectionError::ResourceNotFound(ref cause) => cause,
            CreateProtectionError::Validation(ref cause) => cause,
            CreateProtectionError::Credentials(ref err) => err.description(),
            CreateProtectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProtectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSubscription
#[derive(Debug, PartialEq)]
pub enum CreateSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateSubscriptionError {
    pub fn from_body(body: &str) -> CreateSubscriptionError {
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
                    "InternalErrorException" => {
                        CreateSubscriptionError::InternalError(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateSubscriptionError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSubscriptionError::Validation(error_message.to_string())
                    }
                    _ => CreateSubscriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSubscriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSubscriptionError {
    fn from(err: serde_json::error::Error) -> CreateSubscriptionError {
        CreateSubscriptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSubscriptionError {
    fn from(err: CredentialsError) -> CreateSubscriptionError {
        CreateSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSubscriptionError {
    fn from(err: HttpDispatchError) -> CreateSubscriptionError {
        CreateSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateSubscriptionError {
    fn from(err: io::Error) -> CreateSubscriptionError {
        CreateSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            CreateSubscriptionError::InternalError(ref cause) => cause,
            CreateSubscriptionError::ResourceAlreadyExists(ref cause) => cause,
            CreateSubscriptionError::Validation(ref cause) => cause,
            CreateSubscriptionError::Credentials(ref err) => err.description(),
            CreateSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProtection
#[derive(Debug, PartialEq)]
pub enum DeleteProtectionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the protection state has been modified by another client. You can retry the request.</p>
    OptimisticLock(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteProtectionError {
    pub fn from_body(body: &str) -> DeleteProtectionError {
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
                    "InternalErrorException" => {
                        DeleteProtectionError::InternalError(String::from(error_message))
                    }
                    "OptimisticLockException" => {
                        DeleteProtectionError::OptimisticLock(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteProtectionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteProtectionError::Validation(error_message.to_string())
                    }
                    _ => DeleteProtectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteProtectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteProtectionError {
    fn from(err: serde_json::error::Error) -> DeleteProtectionError {
        DeleteProtectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProtectionError {
    fn from(err: CredentialsError) -> DeleteProtectionError {
        DeleteProtectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProtectionError {
    fn from(err: HttpDispatchError) -> DeleteProtectionError {
        DeleteProtectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteProtectionError {
    fn from(err: io::Error) -> DeleteProtectionError {
        DeleteProtectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteProtectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProtectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteProtectionError::InternalError(ref cause) => cause,
            DeleteProtectionError::OptimisticLock(ref cause) => cause,
            DeleteProtectionError::ResourceNotFound(ref cause) => cause,
            DeleteProtectionError::Validation(ref cause) => cause,
            DeleteProtectionError::Credentials(ref err) => err.description(),
            DeleteProtectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProtectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the subscription you are trying to delete has not yet completed the 1-year commitment. You cannot delete this subscription.</p>
    LockedSubscription(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteSubscriptionError {
    pub fn from_body(body: &str) -> DeleteSubscriptionError {
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
                    "InternalErrorException" => {
                        DeleteSubscriptionError::InternalError(String::from(error_message))
                    }
                    "LockedSubscriptionException" => {
                        DeleteSubscriptionError::LockedSubscription(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteSubscriptionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSubscriptionError::Validation(error_message.to_string())
                    }
                    _ => DeleteSubscriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSubscriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSubscriptionError {
    fn from(err: serde_json::error::Error) -> DeleteSubscriptionError {
        DeleteSubscriptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSubscriptionError {
    fn from(err: CredentialsError) -> DeleteSubscriptionError {
        DeleteSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSubscriptionError {
    fn from(err: HttpDispatchError) -> DeleteSubscriptionError {
        DeleteSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteSubscriptionError {
    fn from(err: io::Error) -> DeleteSubscriptionError {
        DeleteSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteSubscriptionError::InternalError(ref cause) => cause,
            DeleteSubscriptionError::LockedSubscription(ref cause) => cause,
            DeleteSubscriptionError::ResourceNotFound(ref cause) => cause,
            DeleteSubscriptionError::Validation(ref cause) => cause,
            DeleteSubscriptionError::Credentials(ref err) => err.description(),
            DeleteSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAttack
#[derive(Debug, PartialEq)]
pub enum DescribeAttackError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. </p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAttackError {
    pub fn from_body(body: &str) -> DescribeAttackError {
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
                    "InternalErrorException" => {
                        DescribeAttackError::InternalError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeAttackError::InvalidParameter(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAttackError::Validation(error_message.to_string())
                    }
                    _ => DescribeAttackError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAttackError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAttackError {
    fn from(err: serde_json::error::Error) -> DescribeAttackError {
        DescribeAttackError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAttackError {
    fn from(err: CredentialsError) -> DescribeAttackError {
        DescribeAttackError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAttackError {
    fn from(err: HttpDispatchError) -> DescribeAttackError {
        DescribeAttackError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAttackError {
    fn from(err: io::Error) -> DescribeAttackError {
        DescribeAttackError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAttackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAttackError {
    fn description(&self) -> &str {
        match *self {
            DescribeAttackError::InternalError(ref cause) => cause,
            DescribeAttackError::InvalidParameter(ref cause) => cause,
            DescribeAttackError::Validation(ref cause) => cause,
            DescribeAttackError::Credentials(ref err) => err.description(),
            DescribeAttackError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeAttackError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeProtection
#[derive(Debug, PartialEq)]
pub enum DescribeProtectionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeProtectionError {
    pub fn from_body(body: &str) -> DescribeProtectionError {
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
                    "InternalErrorException" => {
                        DescribeProtectionError::InternalError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeProtectionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeProtectionError::Validation(error_message.to_string())
                    }
                    _ => DescribeProtectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeProtectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeProtectionError {
    fn from(err: serde_json::error::Error) -> DescribeProtectionError {
        DescribeProtectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeProtectionError {
    fn from(err: CredentialsError) -> DescribeProtectionError {
        DescribeProtectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeProtectionError {
    fn from(err: HttpDispatchError) -> DescribeProtectionError {
        DescribeProtectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeProtectionError {
    fn from(err: io::Error) -> DescribeProtectionError {
        DescribeProtectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeProtectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeProtectionError {
    fn description(&self) -> &str {
        match *self {
            DescribeProtectionError::InternalError(ref cause) => cause,
            DescribeProtectionError::ResourceNotFound(ref cause) => cause,
            DescribeProtectionError::Validation(ref cause) => cause,
            DescribeProtectionError::Credentials(ref err) => err.description(),
            DescribeProtectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeProtectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSubscription
#[derive(Debug, PartialEq)]
pub enum DescribeSubscriptionError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeSubscriptionError {
    pub fn from_body(body: &str) -> DescribeSubscriptionError {
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
                    "InternalErrorException" => {
                        DescribeSubscriptionError::InternalError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeSubscriptionError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeSubscriptionError::Validation(error_message.to_string())
                    }
                    _ => DescribeSubscriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSubscriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSubscriptionError {
    fn from(err: serde_json::error::Error) -> DescribeSubscriptionError {
        DescribeSubscriptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSubscriptionError {
    fn from(err: CredentialsError) -> DescribeSubscriptionError {
        DescribeSubscriptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSubscriptionError {
    fn from(err: HttpDispatchError) -> DescribeSubscriptionError {
        DescribeSubscriptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeSubscriptionError {
    fn from(err: io::Error) -> DescribeSubscriptionError {
        DescribeSubscriptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            DescribeSubscriptionError::InternalError(ref cause) => cause,
            DescribeSubscriptionError::ResourceNotFound(ref cause) => cause,
            DescribeSubscriptionError::Validation(ref cause) => cause,
            DescribeSubscriptionError::Credentials(ref err) => err.description(),
            DescribeSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSubscriptionState
#[derive(Debug, PartialEq)]
pub enum GetSubscriptionStateError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSubscriptionStateError {
    pub fn from_body(body: &str) -> GetSubscriptionStateError {
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
                    "InternalErrorException" => {
                        GetSubscriptionStateError::InternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSubscriptionStateError::Validation(error_message.to_string())
                    }
                    _ => GetSubscriptionStateError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSubscriptionStateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSubscriptionStateError {
    fn from(err: serde_json::error::Error) -> GetSubscriptionStateError {
        GetSubscriptionStateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSubscriptionStateError {
    fn from(err: CredentialsError) -> GetSubscriptionStateError {
        GetSubscriptionStateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSubscriptionStateError {
    fn from(err: HttpDispatchError) -> GetSubscriptionStateError {
        GetSubscriptionStateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSubscriptionStateError {
    fn from(err: io::Error) -> GetSubscriptionStateError {
        GetSubscriptionStateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSubscriptionStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSubscriptionStateError {
    fn description(&self) -> &str {
        match *self {
            GetSubscriptionStateError::InternalError(ref cause) => cause,
            GetSubscriptionStateError::Validation(ref cause) => cause,
            GetSubscriptionStateError::Credentials(ref err) => err.description(),
            GetSubscriptionStateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSubscriptionStateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAttacks
#[derive(Debug, PartialEq)]
pub enum ListAttacksError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception that indicates that the operation would not cause any change to occur.</p>
    InvalidOperation(String),
    /// <p>Exception that indicates that the parameters passed to the API are invalid. </p>
    InvalidParameter(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAttacksError {
    pub fn from_body(body: &str) -> ListAttacksError {
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
                    "InternalErrorException" => {
                        ListAttacksError::InternalError(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        ListAttacksError::InvalidOperation(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListAttacksError::InvalidParameter(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAttacksError::Validation(error_message.to_string())
                    }
                    _ => ListAttacksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAttacksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAttacksError {
    fn from(err: serde_json::error::Error) -> ListAttacksError {
        ListAttacksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAttacksError {
    fn from(err: CredentialsError) -> ListAttacksError {
        ListAttacksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAttacksError {
    fn from(err: HttpDispatchError) -> ListAttacksError {
        ListAttacksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAttacksError {
    fn from(err: io::Error) -> ListAttacksError {
        ListAttacksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAttacksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAttacksError {
    fn description(&self) -> &str {
        match *self {
            ListAttacksError::InternalError(ref cause) => cause,
            ListAttacksError::InvalidOperation(ref cause) => cause,
            ListAttacksError::InvalidParameter(ref cause) => cause,
            ListAttacksError::Validation(ref cause) => cause,
            ListAttacksError::Credentials(ref err) => err.description(),
            ListAttacksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAttacksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProtections
#[derive(Debug, PartialEq)]
pub enum ListProtectionsError {
    /// <p>Exception that indicates that a problem occurred with the service infrastructure. You can retry the request.</p>
    InternalError(String),
    /// <p>Exception indicating the specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListProtectionsError {
    pub fn from_body(body: &str) -> ListProtectionsError {
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
                    "InternalErrorException" => {
                        ListProtectionsError::InternalError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListProtectionsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListProtectionsError::Validation(error_message.to_string())
                    }
                    _ => ListProtectionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListProtectionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListProtectionsError {
    fn from(err: serde_json::error::Error) -> ListProtectionsError {
        ListProtectionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListProtectionsError {
    fn from(err: CredentialsError) -> ListProtectionsError {
        ListProtectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListProtectionsError {
    fn from(err: HttpDispatchError) -> ListProtectionsError {
        ListProtectionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListProtectionsError {
    fn from(err: io::Error) -> ListProtectionsError {
        ListProtectionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListProtectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProtectionsError {
    fn description(&self) -> &str {
        match *self {
            ListProtectionsError::InternalError(ref cause) => cause,
            ListProtectionsError::ResourceNotFound(ref cause) => cause,
            ListProtectionsError::Validation(ref cause) => cause,
            ListProtectionsError::Credentials(ref err) => err.description(),
            ListProtectionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListProtectionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Shield API. AWS Shield clients implement this trait.
pub trait Shield {
    /// <p>Enables AWS Shield Advanced for a specific AWS resource. The resource can be an Amazon CloudFront distribution, Elastic Load Balancing load balancer, Elastic IP Address, or an Amazon Route 53 hosted zone.</p>
    fn create_protection(
        &self,
        input: CreateProtectionRequest,
    ) -> RusotoFuture<CreateProtectionResponse, CreateProtectionError>;

    /// <p>Activates AWS Shield Advanced for an account.</p>
    fn create_subscription(
        &self,
    ) -> RusotoFuture<CreateSubscriptionResponse, CreateSubscriptionError>;

    /// <p>Deletes an AWS Shield Advanced <a>Protection</a>.</p>
    fn delete_protection(
        &self,
        input: DeleteProtectionRequest,
    ) -> RusotoFuture<DeleteProtectionResponse, DeleteProtectionError>;

    /// <p>Removes AWS Shield Advanced from an account. AWS Shield Advanced requires a 1-year subscription commitment. You cannot delete a subscription prior to the completion of that commitment. </p>
    fn delete_subscription(
        &self,
    ) -> RusotoFuture<DeleteSubscriptionResponse, DeleteSubscriptionError>;

    /// <p>Describes the details of a DDoS attack. </p>
    fn describe_attack(
        &self,
        input: DescribeAttackRequest,
    ) -> RusotoFuture<DescribeAttackResponse, DescribeAttackError>;

    /// <p>Lists the details of a <a>Protection</a> object.</p>
    fn describe_protection(
        &self,
        input: DescribeProtectionRequest,
    ) -> RusotoFuture<DescribeProtectionResponse, DescribeProtectionError>;

    /// <p>Provides details about the AWS Shield Advanced subscription for an account.</p>
    fn describe_subscription(
        &self,
    ) -> RusotoFuture<DescribeSubscriptionResponse, DescribeSubscriptionError>;

    /// <p>Returns the <code>SubscriptionState</code>, either <code>Active</code> or <code>Inactive</code>.</p>
    fn get_subscription_state(
        &self,
    ) -> RusotoFuture<GetSubscriptionStateResponse, GetSubscriptionStateError>;

    /// <p>Returns all ongoing DDoS attacks or all DDoS attacks during a specified time period.</p>
    fn list_attacks(
        &self,
        input: ListAttacksRequest,
    ) -> RusotoFuture<ListAttacksResponse, ListAttacksError>;

    /// <p>Lists all <a>Protection</a> objects for the account.</p>
    fn list_protections(
        &self,
        input: ListProtectionsRequest,
    ) -> RusotoFuture<ListProtectionsResponse, ListProtectionsError>;
}
/// A client for the AWS Shield API.
pub struct ShieldClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ShieldClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ShieldClient {
        ShieldClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ShieldClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ShieldClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Shield for ShieldClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Enables AWS Shield Advanced for a specific AWS resource. The resource can be an Amazon CloudFront distribution, Elastic Load Balancing load balancer, Elastic IP Address, or an Amazon Route 53 hosted zone.</p>
    fn create_protection(
        &self,
        input: CreateProtectionRequest,
    ) -> RusotoFuture<CreateProtectionResponse, CreateProtectionError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.CreateProtection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProtectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateProtectionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Activates AWS Shield Advanced for an account.</p>
    fn create_subscription(
        &self,
    ) -> RusotoFuture<CreateSubscriptionResponse, CreateSubscriptionError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.CreateSubscription");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateSubscriptionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an AWS Shield Advanced <a>Protection</a>.</p>
    fn delete_protection(
        &self,
        input: DeleteProtectionRequest,
    ) -> RusotoFuture<DeleteProtectionResponse, DeleteProtectionError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DeleteProtection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProtectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProtectionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Removes AWS Shield Advanced from an account. AWS Shield Advanced requires a 1-year subscription commitment. You cannot delete a subscription prior to the completion of that commitment. </p>
    fn delete_subscription(
        &self,
    ) -> RusotoFuture<DeleteSubscriptionResponse, DeleteSubscriptionError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DeleteSubscription");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteSubscriptionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Describes the details of a DDoS attack. </p>
    fn describe_attack(
        &self,
        input: DescribeAttackRequest,
    ) -> RusotoFuture<DescribeAttackResponse, DescribeAttackError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeAttack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeAttackResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAttackError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists the details of a <a>Protection</a> object.</p>
    fn describe_protection(
        &self,
        input: DescribeProtectionRequest,
    ) -> RusotoFuture<DescribeProtectionResponse, DescribeProtectionError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeProtection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeProtectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeProtectionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Provides details about the AWS Shield Advanced subscription for an account.</p>
    fn describe_subscription(
        &self,
    ) -> RusotoFuture<DescribeSubscriptionResponse, DescribeSubscriptionError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.DescribeSubscription");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeSubscriptionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeSubscriptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns the <code>SubscriptionState</code>, either <code>Active</code> or <code>Inactive</code>.</p>
    fn get_subscription_state(
        &self,
    ) -> RusotoFuture<GetSubscriptionStateResponse, GetSubscriptionStateError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.GetSubscriptionState");
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSubscriptionStateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetSubscriptionStateError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns all ongoing DDoS attacks or all DDoS attacks during a specified time period.</p>
    fn list_attacks(
        &self,
        input: ListAttacksRequest,
    ) -> RusotoFuture<ListAttacksResponse, ListAttacksError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.ListAttacks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAttacksResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListAttacksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists all <a>Protection</a> objects for the account.</p>
    fn list_protections(
        &self,
        input: ListProtectionsRequest,
    ) -> RusotoFuture<ListProtectionsResponse, ListProtectionsError> {
        let mut request = SignedRequest::new("POST", "shield", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSShield_20160616.ListProtections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListProtectionsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListProtectionsError::from_body(
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
