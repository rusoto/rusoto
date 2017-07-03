
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

use std::fmt;
use std::error::Error;

use rusoto_core::region;
use rusoto_core::request::{DispatchSignedRequest, HttpDispatchError};
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[doc="<p>The input for the AcceptCertificateTransfer operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AcceptCertificateTransferRequest {
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
    #[doc="<p>Specifies whether the certificate is active.</p>"]
    #[serde(rename="setAsActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub set_as_active: Option<bool>,
}

#[doc="<p>Describes the actions associated with a rule.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Action {
    #[doc="<p>Change the state of a CloudWatch alarm.</p>"]
    #[serde(rename="cloudwatchAlarm")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloudwatch_alarm: Option<CloudwatchAlarmAction>,
    #[doc="<p>Capture a CloudWatch metric.</p>"]
    #[serde(rename="cloudwatchMetric")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloudwatch_metric: Option<CloudwatchMetricAction>,
    #[doc="<p>Write to a DynamoDB table.</p>"]
    #[serde(rename="dynamoDB")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dynamo_db: Option<DynamoDBAction>,
    #[doc="<p>Write to a DynamoDB table. This is a new version of the DynamoDB action. It allows you to write each attribute in an MQTT message payload into a separate DynamoDB column.</p>"]
    #[serde(rename="dynamoDBv2")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dynamo_d_bv_2: Option<DynamoDBv2Action>,
    #[doc="<p>Write data to an Amazon Elasticsearch Service domain.</p>"]
    #[serde(rename="elasticsearch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elasticsearch: Option<ElasticsearchAction>,
    #[doc="<p>Write to an Amazon Kinesis Firehose stream.</p>"]
    #[serde(rename="firehose")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub firehose: Option<FirehoseAction>,
    #[doc="<p>Write data to an Amazon Kinesis stream.</p>"]
    #[serde(rename="kinesis")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis: Option<KinesisAction>,
    #[doc="<p>Invoke a Lambda function.</p>"]
    #[serde(rename="lambda")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lambda: Option<LambdaAction>,
    #[doc="<p>Publish to another MQTT topic.</p>"]
    #[serde(rename="republish")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub republish: Option<RepublishAction>,
    #[doc="<p>Write to an Amazon S3 bucket.</p>"]
    #[serde(rename="s3")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s_3: Option<S3Action>,
    #[doc="<p>Send a message to a Salesforce IoT Cloud Input Stream.</p>"]
    #[serde(rename="salesforce")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub salesforce: Option<SalesforceAction>,
    #[doc="<p>Publish to an Amazon SNS topic.</p>"]
    #[serde(rename="sns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sns: Option<SnsAction>,
    #[doc="<p>Publish to an Amazon SQS queue.</p>"]
    #[serde(rename="sqs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sqs: Option<SqsAction>,
}

#[doc="<p>The input for the AttachPrincipalPolicy operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AttachPrincipalPolicyRequest {
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
    #[doc="<p>The principal, which can be a certificate ARN (as returned from the CreateCertificate operation) or an Amazon Cognito ID.</p>"]
    #[serde(rename="principal")]
    pub principal: String,
}

#[doc="<p>The input for the AttachThingPrincipal operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AttachThingPrincipalRequest {
    #[doc="<p>The principal, such as a certificate or other credential.</p>"]
    #[serde(rename="principal")]
    pub principal: String,
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
}

#[doc="<p>The output from the AttachThingPrincipal operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AttachThingPrincipalResponse;

#[doc="<p>The attribute payload.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AttributePayload {
    #[doc="<p>A JSON string containing up to three key-value pair in JSON format. For example:</p> <p> <code>{\\\"attributes\\\":{\\\"string1\\\":\\\"string2\\\"}}</code> </p>"]
    #[serde(rename="attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>Specifies whether the list of attributes provided in the <code>AttributePayload</code> is merged with the attributes stored in the registry, instead of overwriting them.</p> <p>To remove an attribute, call <code>UpdateThing</code> with an empty attribute value.</p> <note> <p>The <code>merge</code> attribute is only valid when calling <code>UpdateThing</code>.</p> </note>"]
    #[serde(rename="merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merge: Option<bool>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum AutoRegistrationStatus {
    Disable,
    Enable,
}

impl Into<String> for AutoRegistrationStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for AutoRegistrationStatus {
    fn into(self) -> &'static str {
        match self {
            AutoRegistrationStatus::Disable => "DISABLE",
            AutoRegistrationStatus::Enable => "ENABLE",
        }
    }
}

impl ::std::str::FromStr for AutoRegistrationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DISABLE" => Ok(AutoRegistrationStatus::Disable),
            "ENABLE" => Ok(AutoRegistrationStatus::Enable),
            _ => Err(()),
        }
    }
}

#[doc="<p>A CA certificate.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CACertificate {
    #[doc="<p>The ARN of the CA certificate.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The ID of the CA certificate.</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc="<p>The date the CA certificate was created.</p>"]
    #[serde(rename="creationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The status of the CA certificate.</p> <p>The status value REGISTER_INACTIVE is deprecated and should not be used.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[doc="<p>Describes a CA certificate.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CACertificateDescription {
    #[doc="<p>Whether the CA certificate configured for auto registration of device certificates. Valid values are \"ENABLE\" and \"DISABLE\"</p>"]
    #[serde(rename="autoRegistrationStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_registration_status: Option<String>,
    #[doc="<p>The CA certificate ARN.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The CA certificate ID.</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc="<p>The CA certificate data, in PEM format.</p>"]
    #[serde(rename="certificatePem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_pem: Option<String>,
    #[doc="<p>The date the CA certificate was created.</p>"]
    #[serde(rename="creationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The owner of the CA certificate.</p>"]
    #[serde(rename="ownedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owned_by: Option<String>,
    #[doc="<p>The status of a CA certificate.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum CACertificateStatus {
    Active,
    Inactive,
}

impl Into<String> for CACertificateStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for CACertificateStatus {
    fn into(self) -> &'static str {
        match self {
            CACertificateStatus::Active => "ACTIVE",
            CACertificateStatus::Inactive => "INACTIVE",
        }
    }
}

impl ::std::str::FromStr for CACertificateStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTIVE" => Ok(CACertificateStatus::Active),
            "INACTIVE" => Ok(CACertificateStatus::Inactive),
            _ => Err(()),
        }
    }
}

#[doc="<p>The input for the CancelCertificateTransfer operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CancelCertificateTransferRequest {
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum CannedAccessControlList {
    AuthenticatedRead,
    AwsExecRead,
    BucketOwnerFullControl,
    BucketOwnerRead,
    LogDeliveryWrite,
    Private,
    PublicRead,
    PublicReadWrite,
}

impl Into<String> for CannedAccessControlList {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for CannedAccessControlList {
    fn into(self) -> &'static str {
        match self {
            CannedAccessControlList::AuthenticatedRead => "authenticated-read",
            CannedAccessControlList::AwsExecRead => "aws-exec-read",
            CannedAccessControlList::BucketOwnerFullControl => "bucket-owner-full-control",
            CannedAccessControlList::BucketOwnerRead => "bucket-owner-read",
            CannedAccessControlList::LogDeliveryWrite => "log-delivery-write",
            CannedAccessControlList::Private => "private",
            CannedAccessControlList::PublicRead => "public-read",
            CannedAccessControlList::PublicReadWrite => "public-read-write",
        }
    }
}

impl ::std::str::FromStr for CannedAccessControlList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authenticated-read" => Ok(CannedAccessControlList::AuthenticatedRead),
            "aws-exec-read" => Ok(CannedAccessControlList::AwsExecRead),
            "bucket-owner-full-control" => Ok(CannedAccessControlList::BucketOwnerFullControl),
            "bucket-owner-read" => Ok(CannedAccessControlList::BucketOwnerRead),
            "log-delivery-write" => Ok(CannedAccessControlList::LogDeliveryWrite),
            "private" => Ok(CannedAccessControlList::Private),
            "public-read" => Ok(CannedAccessControlList::PublicRead),
            "public-read-write" => Ok(CannedAccessControlList::PublicReadWrite),
            _ => Err(()),
        }
    }
}

#[doc="<p>Information about a certificate.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Certificate {
    #[doc="<p>The ARN of the certificate.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc="<p>The date and time the certificate was created.</p>"]
    #[serde(rename="creationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The status of the certificate.</p> <p>The status value REGISTER_INACTIVE is deprecated and should not be used.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[doc="<p>Describes a certificate.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CertificateDescription {
    #[doc="<p>The certificate ID of the CA certificate used to sign this certificate.</p>"]
    #[serde(rename="caCertificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ca_certificate_id: Option<String>,
    #[doc="<p>The ARN of the certificate.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc="<p>The certificate data, in PEM format.</p>"]
    #[serde(rename="certificatePem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_pem: Option<String>,
    #[doc="<p>The date and time the certificate was created.</p>"]
    #[serde(rename="creationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The date and time the certificate was last modified.</p>"]
    #[serde(rename="lastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="<p>The ID of the AWS account that owns the certificate.</p>"]
    #[serde(rename="ownedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owned_by: Option<String>,
    #[doc="<p>The ID of the AWS account of the previous owner of the certificate.</p>"]
    #[serde(rename="previousOwnedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_owned_by: Option<String>,
    #[doc="<p>The status of the certificate.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The transfer data.</p>"]
    #[serde(rename="transferData")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transfer_data: Option<TransferData>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum CertificateStatus {
    Active,
    Inactive,
    PendingActivation,
    PendingTransfer,
    RegisterInactive,
    Revoked,
}

impl Into<String> for CertificateStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for CertificateStatus {
    fn into(self) -> &'static str {
        match self {
            CertificateStatus::Active => "ACTIVE",
            CertificateStatus::Inactive => "INACTIVE",
            CertificateStatus::PendingActivation => "PENDING_ACTIVATION",
            CertificateStatus::PendingTransfer => "PENDING_TRANSFER",
            CertificateStatus::RegisterInactive => "REGISTER_INACTIVE",
            CertificateStatus::Revoked => "REVOKED",
        }
    }
}

impl ::std::str::FromStr for CertificateStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACTIVE" => Ok(CertificateStatus::Active),
            "INACTIVE" => Ok(CertificateStatus::Inactive),
            "PENDING_ACTIVATION" => Ok(CertificateStatus::PendingActivation),
            "PENDING_TRANSFER" => Ok(CertificateStatus::PendingTransfer),
            "REGISTER_INACTIVE" => Ok(CertificateStatus::RegisterInactive),
            "REVOKED" => Ok(CertificateStatus::Revoked),
            _ => Err(()),
        }
    }
}

#[doc="<p>Describes an action that updates a CloudWatch alarm.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CloudwatchAlarmAction {
    #[doc="<p>The CloudWatch alarm name.</p>"]
    #[serde(rename="alarmName")]
    pub alarm_name: String,
    #[doc="<p>The IAM role that allows access to the CloudWatch alarm.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
    #[doc="<p>The reason for the alarm change.</p>"]
    #[serde(rename="stateReason")]
    pub state_reason: String,
    #[doc="<p>The value of the alarm state. Acceptable values are: OK, ALARM, INSUFFICIENT_DATA.</p>"]
    #[serde(rename="stateValue")]
    pub state_value: String,
}

#[doc="<p>Describes an action that captures a CloudWatch metric.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CloudwatchMetricAction {
    #[doc="<p>The CloudWatch metric name.</p>"]
    #[serde(rename="metricName")]
    pub metric_name: String,
    #[doc="<p>The CloudWatch metric namespace name.</p>"]
    #[serde(rename="metricNamespace")]
    pub metric_namespace: String,
    #[doc="<p>An optional <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/cloudwatch_concepts.html#about_timestamp\">Unix timestamp</a>.</p>"]
    #[serde(rename="metricTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_timestamp: Option<String>,
    #[doc="<p>The <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/cloudwatch_concepts.html#Unit\">metric unit</a> supported by CloudWatch.</p>"]
    #[serde(rename="metricUnit")]
    pub metric_unit: String,
    #[doc="<p>The CloudWatch metric value.</p>"]
    #[serde(rename="metricValue")]
    pub metric_value: String,
    #[doc="<p>The IAM role that allows access to the CloudWatch metric.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
}

#[doc="<p>The input for the CreateCertificateFromCsr operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateCertificateFromCsrRequest {
    #[doc="<p>The certificate signing request (CSR).</p>"]
    #[serde(rename="certificateSigningRequest")]
    pub certificate_signing_request: String,
    #[doc="<p>Specifies whether the certificate is active.</p>"]
    #[serde(rename="setAsActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub set_as_active: Option<bool>,
}

#[doc="<p>The output from the CreateCertificateFromCsr operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateCertificateFromCsrResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the certificate. You can use the ARN as a principal for policy operations.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The ID of the certificate. Certificate management operations only take a certificateId.</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc="<p>The certificate data, in PEM format.</p>"]
    #[serde(rename="certificatePem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_pem: Option<String>,
}

#[doc="<p>The input for the CreateKeysAndCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateKeysAndCertificateRequest {
    #[doc="<p>Specifies whether the certificate is active.</p>"]
    #[serde(rename="setAsActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub set_as_active: Option<bool>,
}

#[doc="<p>The output of the CreateKeysAndCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateKeysAndCertificateResponse {
    #[doc="<p>The ARN of the certificate.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The ID of the certificate. AWS IoT issues a default subject name for the certificate (for example, AWS IoT Certificate).</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc="<p>The certificate data, in PEM format.</p>"]
    #[serde(rename="certificatePem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_pem: Option<String>,
    #[doc="<p>The generated key pair.</p>"]
    #[serde(rename="keyPair")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_pair: Option<KeyPair>,
}

#[doc="<p>The input for the CreatePolicy operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePolicyRequest {
    #[doc="<p>The JSON document that describes the policy. <b>policyDocument</b> must have a minimum length of 1, with a maximum length of 2048, excluding whitespace.</p>"]
    #[serde(rename="policyDocument")]
    pub policy_document: String,
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
}

#[doc="<p>The output from the CreatePolicy operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreatePolicyResponse {
    #[doc="<p>The policy ARN.</p>"]
    #[serde(rename="policyArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_arn: Option<String>,
    #[doc="<p>The JSON document that describes the policy.</p>"]
    #[serde(rename="policyDocument")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_document: Option<String>,
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_name: Option<String>,
    #[doc="<p>The policy version ID.</p>"]
    #[serde(rename="policyVersionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_version_id: Option<String>,
}

#[doc="<p>The input for the CreatePolicyVersion operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreatePolicyVersionRequest {
    #[doc="<p>The JSON document that describes the policy. Minimum length of 1. Maximum length of 2048, excluding whitespaces</p>"]
    #[serde(rename="policyDocument")]
    pub policy_document: String,
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
    #[doc="<p>Specifies whether the policy version is set as the default. When this parameter is true, the new policy version becomes the operative version (that is, the version that is in effect for the certificates to which the policy is attached).</p>"]
    #[serde(rename="setAsDefault")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub set_as_default: Option<bool>,
}

#[doc="<p>The output of the CreatePolicyVersion operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreatePolicyVersionResponse {
    #[doc="<p>Specifies whether the policy version is the default.</p>"]
    #[serde(rename="isDefaultVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_default_version: Option<bool>,
    #[doc="<p>The policy ARN.</p>"]
    #[serde(rename="policyArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_arn: Option<String>,
    #[doc="<p>The JSON document that describes the policy.</p>"]
    #[serde(rename="policyDocument")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_document: Option<String>,
    #[doc="<p>The policy version ID.</p>"]
    #[serde(rename="policyVersionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_version_id: Option<String>,
}

#[doc="<p>The input for the CreateThing operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateThingRequest {
    #[doc="<p>The attribute payload, which consists of up to three name/value pairs in a JSON document. For example:</p> <p> <code>{\\\"attributes\\\":{\\\"string1\\\":\\\"string2\\\"}}</code> </p>"]
    #[serde(rename="attributePayload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,
    #[doc="<p>The name of the thing to create.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
    #[doc="<p>The name of the thing type associated with the new thing.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[doc="<p>The output of the CreateThing operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateThingResponse {
    #[doc="<p>The ARN of the new thing.</p>"]
    #[serde(rename="thingArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_arn: Option<String>,
    #[doc="<p>The name of the new thing.</p>"]
    #[serde(rename="thingName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_name: Option<String>,
}

#[doc="<p>The input for the CreateThingType operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateThingTypeRequest {
    #[doc="<p>The name of the thing type.</p>"]
    #[serde(rename="thingTypeName")]
    pub thing_type_name: String,
    #[doc="<p>The ThingTypeProperties for the thing type to create. It contains information about the new thing type including a description, and a list of searchable thing attribute names.</p>"]
    #[serde(rename="thingTypeProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

#[doc="<p>The output of the CreateThingType operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateThingTypeResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the thing type.</p>"]
    #[serde(rename="thingTypeArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_arn: Option<String>,
    #[doc="<p>The name of the thing type.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[doc="<p>The input for the CreateTopicRule operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateTopicRuleRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="ruleName")]
    pub rule_name: String,
    #[doc="<p>The rule payload.</p>"]
    #[serde(rename="topicRulePayload")]
    pub topic_rule_payload: TopicRulePayload,
}

#[doc="<p>Input for the DeleteCACertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteCACertificateRequest {
    #[doc="<p>The ID of the certificate to delete.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
}

#[doc="<p>The output for the DeleteCACertificate operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteCACertificateResponse;

#[doc="<p>The input for the DeleteCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteCertificateRequest {
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
}

#[doc="<p>The input for the DeletePolicy operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeletePolicyRequest {
    #[doc="<p>The name of the policy to delete.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
}

#[doc="<p>The input for the DeletePolicyVersion operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeletePolicyVersionRequest {
    #[doc="<p>The name of the policy.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
    #[doc="<p>The policy version ID.</p>"]
    #[serde(rename="policyVersionId")]
    pub policy_version_id: String,
}

#[doc="<p>The input for the DeleteRegistrationCode operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteRegistrationCodeRequest;

#[doc="<p>The output for the DeleteRegistrationCode operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteRegistrationCodeResponse;

#[doc="<p>The input for the DeleteThing operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteThingRequest {
    #[doc="<p>The expected version of the thing record in the registry. If the version of the record in the registry does not match the expected version specified in the request, the <code>DeleteThing</code> request is rejected with a <code>VersionConflictException</code>.</p>"]
    #[serde(rename="expectedVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expected_version: Option<i64>,
    #[doc="<p>The name of the thing to delete.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
}

#[doc="<p>The output of the DeleteThing operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteThingResponse;

#[doc="<p>The input for the DeleteThingType operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteThingTypeRequest {
    #[doc="<p>The name of the thing type.</p>"]
    #[serde(rename="thingTypeName")]
    pub thing_type_name: String,
}

#[doc="<p>The output for the DeleteThingType operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteThingTypeResponse;

#[doc="<p>The input for the DeleteTopicRule operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteTopicRuleRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="ruleName")]
    pub rule_name: String,
}

#[doc="<p>The input for the DeprecateThingType operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeprecateThingTypeRequest {
    #[doc="<p>The name of the thing type to deprecate.</p>"]
    #[serde(rename="thingTypeName")]
    pub thing_type_name: String,
    #[doc="<p>Whether to undeprecate a deprecated thing type. If <b>true</b>, the thing type will not be deprecated anymore and you can associate it with things.</p>"]
    #[serde(rename="undoDeprecate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub undo_deprecate: Option<bool>,
}

#[doc="<p>The output for the DeprecateThingType operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeprecateThingTypeResponse;

#[doc="<p>The input for the DescribeCACertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeCACertificateRequest {
    #[doc="<p>The CA certificate identifier.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
}

#[doc="<p>The output from the DescribeCACertificate operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeCACertificateResponse {
    #[doc="<p>The CA certificate description.</p>"]
    #[serde(rename="certificateDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_description: Option<CACertificateDescription>,
}

#[doc="<p>The input for the DescribeCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeCertificateRequest {
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
}

#[doc="<p>The output of the DescribeCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeCertificateResponse {
    #[doc="<p>The description of the certificate.</p>"]
    #[serde(rename="certificateDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_description: Option<CertificateDescription>,
}

#[doc="<p>The input for the DescribeEndpoint operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEndpointRequest;

#[doc="<p>The output from the DescribeEndpoint operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEndpointResponse {
    #[doc="<p>The endpoint. The format of the endpoint is as follows: <i>identifier</i>.iot.<i>region</i>.amazonaws.com.</p>"]
    #[serde(rename="endpointAddress")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_address: Option<String>,
}

#[doc="<p>The input for the DescribeThing operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeThingRequest {
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
}

#[doc="<p>The output from the DescribeThing operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeThingResponse {
    #[doc="<p>The thing attributes.</p>"]
    #[serde(rename="attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The default client ID.</p>"]
    #[serde(rename="defaultClientId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_client_id: Option<String>,
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_name: Option<String>,
    #[doc="<p>The thing type name.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
    #[doc="<p>The current version of the thing record in the registry.</p> <note> <p>To avoid unintentional changes to the information in the registry, you can pass the version information in the <code>expectedVersion</code> parameter of the <code>UpdateThing</code> and <code>DeleteThing</code> calls.</p> </note>"]
    #[serde(rename="version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="<p>The input for the DescribeThingType operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeThingTypeRequest {
    #[doc="<p>The name of the thing type.</p>"]
    #[serde(rename="thingTypeName")]
    pub thing_type_name: String,
}

#[doc="<p>The output for the DescribeThingType operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeThingTypeResponse {
    #[doc="<p>The ThingTypeMetadata contains additional information about the thing type including: creation date and time, a value indicating whether the thing type is deprecated, and a date and time when it was deprecated.</p>"]
    #[serde(rename="thingTypeMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_metadata: Option<ThingTypeMetadata>,
    #[doc="<p>The name of the thing type.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
    #[doc="<p>The ThingTypeProperties contains information about the thing type including description, and a list of searchable thing attribute names.</p>"]
    #[serde(rename="thingTypeProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

#[doc="<p>The input for the DetachPrincipalPolicy operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DetachPrincipalPolicyRequest {
    #[doc="<p>The name of the policy to detach.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
    #[doc="<p>The principal.</p> <p>If the principal is a certificate, specify the certificate ARN. If the principal is an Amazon Cognito identity, specify the identity ID.</p>"]
    #[serde(rename="principal")]
    pub principal: String,
}

#[doc="<p>The input for the DetachThingPrincipal operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DetachThingPrincipalRequest {
    #[doc="<p>If the principal is a certificate, this value must be ARN of the certificate. If the principal is an Amazon Cognito identity, this value must be the ID of the Amazon Cognito identity.</p>"]
    #[serde(rename="principal")]
    pub principal: String,
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
}

#[doc="<p>The output from the DetachThingPrincipal operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DetachThingPrincipalResponse;

#[doc="<p>The input for the DisableTopicRuleRequest operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DisableTopicRuleRequest {
    #[doc="<p>The name of the rule to disable.</p>"]
    #[serde(rename="ruleName")]
    pub rule_name: String,
}

#[doc="<p>Describes an action to write to a DynamoDB table.</p> <p>The <code>tableName</code>, <code>hashKeyField</code>, and <code>rangeKeyField</code> values must match the values used when you created the table.</p> <p>The <code>hashKeyValue</code> and <code>rangeKeyvalue</code> fields use a substitution template syntax. These templates provide data at runtime. The syntax is as follows: ${<i>sql-expression</i>}.</p> <p>You can specify any valid expression in a WHERE or SELECT clause, including JSON properties, comparisons, calculations, and functions. For example, the following field uses the third level of the topic:</p> <p> <code>\"hashKeyValue\": \"${topic(3)}\"</code> </p> <p>The following field uses the timestamp:</p> <p> <code>\"rangeKeyValue\": \"${timestamp()}\"</code> </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DynamoDBAction {
    #[doc="<p>The hash key name.</p>"]
    #[serde(rename="hashKeyField")]
    pub hash_key_field: String,
    #[doc="<p>The hash key type. Valid values are \"STRING\" or \"NUMBER\"</p>"]
    #[serde(rename="hashKeyType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hash_key_type: Option<String>,
    #[doc="<p>The hash key value.</p>"]
    #[serde(rename="hashKeyValue")]
    pub hash_key_value: String,
    #[doc="<p>The type of operation to be performed. This follows the substitution template, so it can be <code>${operation}</code>, but the substitution must result in one of the following: <code>INSERT</code>, <code>UPDATE</code>, or <code>DELETE</code>.</p>"]
    #[serde(rename="operation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operation: Option<String>,
    #[doc="<p>The action payload. This name can be customized.</p>"]
    #[serde(rename="payloadField")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload_field: Option<String>,
    #[doc="<p>The range key name.</p>"]
    #[serde(rename="rangeKeyField")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub range_key_field: Option<String>,
    #[doc="<p>The range key type. Valid values are \"STRING\" or \"NUMBER\"</p>"]
    #[serde(rename="rangeKeyType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub range_key_type: Option<String>,
    #[doc="<p>The range key value.</p>"]
    #[serde(rename="rangeKeyValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub range_key_value: Option<String>,
    #[doc="<p>The ARN of the IAM role that grants access to the DynamoDB table.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
    #[doc="<p>The name of the DynamoDB table.</p>"]
    #[serde(rename="tableName")]
    pub table_name: String,
}

#[doc="<p>Describes an action to write to a DynamoDB table.</p> <p>This DynamoDB action writes each attribute in the message payload into it's own column in the DynamoDB table.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DynamoDBv2Action {
    #[doc="<p>Specifies the DynamoDB table to which the message data will be written. For example:</p> <p> <code>{ \"dynamoDBv2\": { \"roleArn\": \"aws:iam:12341251:my-role\" \"putItem\": { \"tableName\": \"my-table\" } } }</code> </p> <p>Each attribute in the message payload will be written to a separate column in the DynamoDB database.</p>"]
    #[serde(rename="putItem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub put_item: Option<PutItemInput>,
    #[doc="<p>The ARN of the IAM role that grants access to the DynamoDB table.</p>"]
    #[serde(rename="roleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum DynamoKeyType {
    Number,
    String,
}

impl Into<String> for DynamoKeyType {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for DynamoKeyType {
    fn into(self) -> &'static str {
        match self {
            DynamoKeyType::Number => "NUMBER",
            DynamoKeyType::String => "STRING",
        }
    }
}

impl ::std::str::FromStr for DynamoKeyType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NUMBER" => Ok(DynamoKeyType::Number),
            "STRING" => Ok(DynamoKeyType::String),
            _ => Err(()),
        }
    }
}

#[doc="<p>Describes an action that writes data to an Amazon Elasticsearch Service domain.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ElasticsearchAction {
    #[doc="<p>The endpoint of your Elasticsearch domain.</p>"]
    #[serde(rename="endpoint")]
    pub endpoint: String,
    #[doc="<p>The unique identifier for the document you are storing.</p>"]
    #[serde(rename="id")]
    pub id: String,
    #[doc="<p>The Elasticsearch index where you want to store your data.</p>"]
    #[serde(rename="index")]
    pub index: String,
    #[doc="<p>The IAM role ARN that has access to Elasticsearch.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
    #[doc="<p>The type of document you are storing.</p>"]
    #[serde(rename="type")]
    pub type_: String,
}

#[doc="<p>The input for the EnableTopicRuleRequest operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct EnableTopicRuleRequest {
    #[doc="<p>The name of the topic rule to enable.</p>"]
    #[serde(rename="ruleName")]
    pub rule_name: String,
}

#[doc="<p>Describes an action that writes data to an Amazon Kinesis Firehose stream.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct FirehoseAction {
    #[doc="<p>The delivery stream name.</p>"]
    #[serde(rename="deliveryStreamName")]
    pub delivery_stream_name: String,
    #[doc="<p>The IAM role that grants access to the Amazon Kinesis Firehost stream.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
    #[doc="<p>A character separator that will be used to separate records written to the Firehose stream. Valid values are: '\\n' (newline), '\\t' (tab), '\\r\\n' (Windows newline), ',' (comma).</p>"]
    #[serde(rename="separator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub separator: Option<String>,
}

#[doc="<p>The input for the GetLoggingOptions operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetLoggingOptionsRequest;

#[doc="<p>The output from the GetLoggingOptions operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetLoggingOptionsResponse {
    #[doc="<p>The logging level.</p>"]
    #[serde(rename="logLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_level: Option<String>,
    #[doc="<p>The ARN of the IAM role that grants access.</p>"]
    #[serde(rename="roleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="<p>The input for the GetPolicy operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetPolicyRequest {
    #[doc="<p>The name of the policy.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
}

#[doc="<p>The output from the GetPolicy operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetPolicyResponse {
    #[doc="<p>The default policy version ID.</p>"]
    #[serde(rename="defaultVersionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_version_id: Option<String>,
    #[doc="<p>The policy ARN.</p>"]
    #[serde(rename="policyArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_arn: Option<String>,
    #[doc="<p>The JSON document that describes the policy.</p>"]
    #[serde(rename="policyDocument")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_document: Option<String>,
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_name: Option<String>,
}

#[doc="<p>The input for the GetPolicyVersion operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetPolicyVersionRequest {
    #[doc="<p>The name of the policy.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
    #[doc="<p>The policy version ID.</p>"]
    #[serde(rename="policyVersionId")]
    pub policy_version_id: String,
}

#[doc="<p>The output from the GetPolicyVersion operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetPolicyVersionResponse {
    #[doc="<p>Specifies whether the policy version is the default.</p>"]
    #[serde(rename="isDefaultVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_default_version: Option<bool>,
    #[doc="<p>The policy ARN.</p>"]
    #[serde(rename="policyArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_arn: Option<String>,
    #[doc="<p>The JSON document that describes the policy.</p>"]
    #[serde(rename="policyDocument")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_document: Option<String>,
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_name: Option<String>,
    #[doc="<p>The policy version ID.</p>"]
    #[serde(rename="policyVersionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_version_id: Option<String>,
}

#[doc="<p>The input to the GetRegistrationCode operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetRegistrationCodeRequest;

#[doc="<p>The output from the GetRegistrationCode operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetRegistrationCodeResponse {
    #[doc="<p>The CA certificate registration code.</p>"]
    #[serde(rename="registrationCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub registration_code: Option<String>,
}

#[doc="<p>The input for the GetTopicRule operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetTopicRuleRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="ruleName")]
    pub rule_name: String,
}

#[doc="<p>The output from the GetTopicRule operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetTopicRuleResponse {
    #[doc="<p>The rule.</p>"]
    #[serde(rename="rule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule: Option<TopicRule>,
    #[doc="<p>The rule ARN.</p>"]
    #[serde(rename="ruleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_arn: Option<String>,
}

#[doc="<p>Describes a key pair.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct KeyPair {
    #[doc="<p>The private key.</p>"]
    #[serde(rename="PrivateKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_key: Option<String>,
    #[doc="<p>The public key.</p>"]
    #[serde(rename="PublicKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_key: Option<String>,
}

#[doc="<p>Describes an action to write data to an Amazon Kinesis stream.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct KinesisAction {
    #[doc="<p>The partition key.</p>"]
    #[serde(rename="partitionKey")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition_key: Option<String>,
    #[doc="<p>The ARN of the IAM role that grants access to the Amazon Kinesis stream.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
    #[doc="<p>The name of the Amazon Kinesis stream.</p>"]
    #[serde(rename="streamName")]
    pub stream_name: String,
}

#[doc="<p>Describes an action to invoke a Lambda function.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct LambdaAction {
    #[doc="<p>The ARN of the Lambda function.</p>"]
    #[serde(rename="functionArn")]
    pub function_arn: String,
}

#[doc="<p>Input for the ListCACertificates operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListCACertificatesRequest {
    #[doc="<p>Determines the order of the results.</p>"]
    #[serde(rename="ascendingOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ascending_order: Option<bool>,
    #[doc="<p>The marker for the next set of results.</p>"]
    #[serde(rename="marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The result page size.</p>"]
    #[serde(rename="pageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<i64>,
}

#[doc="<p>The output from the ListCACertificates operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListCACertificatesResponse {
    #[doc="<p>The CA certificates registered in your AWS account.</p>"]
    #[serde(rename="certificates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificates: Option<Vec<CACertificate>>,
    #[doc="<p>The current position within the list of CA certificates.</p>"]
    #[serde(rename="nextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<String>,
}

#[doc="<p>The input to the ListCertificatesByCA operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListCertificatesByCARequest {
    #[doc="<p>Specifies the order for results. If True, the results are returned in ascending order, based on the creation date.</p>"]
    #[serde(rename="ascendingOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ascending_order: Option<bool>,
    #[doc="<p>The ID of the CA certificate. This operation will list all registered device certificate that were signed by this CA certificate.</p>"]
    #[serde(rename="caCertificateId")]
    pub ca_certificate_id: String,
    #[doc="<p>The marker for the next set of results.</p>"]
    #[serde(rename="marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The result page size.</p>"]
    #[serde(rename="pageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<i64>,
}

#[doc="<p>The output of the ListCertificatesByCA operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListCertificatesByCAResponse {
    #[doc="<p>The device certificates signed by the specified CA certificate.</p>"]
    #[serde(rename="certificates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    #[doc="<p>The marker for the next set of results, or null if there are no additional results.</p>"]
    #[serde(rename="nextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<String>,
}

#[doc="<p>The input for the ListCertificates operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListCertificatesRequest {
    #[doc="<p>Specifies the order for results. If True, the results are returned in ascending order, based on the creation date.</p>"]
    #[serde(rename="ascendingOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ascending_order: Option<bool>,
    #[doc="<p>The marker for the next set of results.</p>"]
    #[serde(rename="marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The result page size.</p>"]
    #[serde(rename="pageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<i64>,
}

#[doc="<p>The output of the ListCertificates operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListCertificatesResponse {
    #[doc="<p>The descriptions of the certificates.</p>"]
    #[serde(rename="certificates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    #[doc="<p>The marker for the next set of results, or null if there are no additional results.</p>"]
    #[serde(rename="nextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<String>,
}

#[doc="<p>The input to the ListOutgoingCertificates operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListOutgoingCertificatesRequest {
    #[doc="<p>Specifies the order for results. If True, the results are returned in ascending order, based on the creation date.</p>"]
    #[serde(rename="ascendingOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ascending_order: Option<bool>,
    #[doc="<p>The marker for the next set of results.</p>"]
    #[serde(rename="marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The result page size.</p>"]
    #[serde(rename="pageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<i64>,
}

#[doc="<p>The output from the ListOutgoingCertificates operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListOutgoingCertificatesResponse {
    #[doc="<p>The marker for the next set of results.</p>"]
    #[serde(rename="nextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<String>,
    #[doc="<p>The certificates that are being transfered but not yet accepted.</p>"]
    #[serde(rename="outgoingCertificates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub outgoing_certificates: Option<Vec<OutgoingCertificate>>,
}

#[doc="<p>The input for the ListPolicies operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPoliciesRequest {
    #[doc="<p>Specifies the order for results. If true, the results are returned in ascending creation order.</p>"]
    #[serde(rename="ascendingOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ascending_order: Option<bool>,
    #[doc="<p>The marker for the next set of results.</p>"]
    #[serde(rename="marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The result page size.</p>"]
    #[serde(rename="pageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<i64>,
}

#[doc="<p>The output from the ListPolicies operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPoliciesResponse {
    #[doc="<p>The marker for the next set of results, or null if there are no additional results.</p>"]
    #[serde(rename="nextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<String>,
    #[doc="<p>The descriptions of the policies.</p>"]
    #[serde(rename="policies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[doc="<p>The input for the ListPolicyPrincipals operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPolicyPrincipalsRequest {
    #[doc="<p>Specifies the order for results. If true, the results are returned in ascending creation order.</p>"]
    #[serde(rename="ascendingOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ascending_order: Option<bool>,
    #[doc="<p>The marker for the next set of results.</p>"]
    #[serde(rename="marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The result page size.</p>"]
    #[serde(rename="pageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<i64>,
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
}

#[doc="<p>The output from the ListPolicyPrincipals operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPolicyPrincipalsResponse {
    #[doc="<p>The marker for the next set of results, or null if there are no additional results.</p>"]
    #[serde(rename="nextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<String>,
    #[doc="<p>The descriptions of the principals.</p>"]
    #[serde(rename="principals")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub principals: Option<Vec<String>>,
}

#[doc="<p>The input for the ListPolicyVersions operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPolicyVersionsRequest {
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
}

#[doc="<p>The output from the ListPolicyVersions operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPolicyVersionsResponse {
    #[doc="<p>The policy versions.</p>"]
    #[serde(rename="policyVersions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_versions: Option<Vec<PolicyVersion>>,
}

#[doc="<p>The input for the ListPrincipalPolicies operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPrincipalPoliciesRequest {
    #[doc="<p>Specifies the order for results. If true, results are returned in ascending creation order.</p>"]
    #[serde(rename="ascendingOrder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ascending_order: Option<bool>,
    #[doc="<p>The marker for the next set of results.</p>"]
    #[serde(rename="marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The result page size.</p>"]
    #[serde(rename="pageSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_size: Option<i64>,
    #[doc="<p>The principal.</p>"]
    #[serde(rename="principal")]
    pub principal: String,
}

#[doc="<p>The output from the ListPrincipalPolicies operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPrincipalPoliciesResponse {
    #[doc="<p>The marker for the next set of results, or null if there are no additional results.</p>"]
    #[serde(rename="nextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<String>,
    #[doc="<p>The policies.</p>"]
    #[serde(rename="policies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[doc="<p>The input for the ListPrincipalThings operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListPrincipalThingsRequest {
    #[doc="<p>The maximum number of results to return in this operation.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The principal.</p>"]
    #[serde(rename="principal")]
    pub principal: String,
}

#[doc="<p>The output from the ListPrincipalThings operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListPrincipalThingsResponse {
    #[doc="<p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The things.</p>"]
    #[serde(rename="things")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub things: Option<Vec<String>>,
}

#[doc="<p>The input for the ListThingPrincipal operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListThingPrincipalsRequest {
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
}

#[doc="<p>The output from the ListThingPrincipals operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListThingPrincipalsResponse {
    #[doc="<p>The principals associated with the thing.</p>"]
    #[serde(rename="principals")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub principals: Option<Vec<String>>,
}

#[doc="<p>The input for the ListThingTypes operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListThingTypesRequest {
    #[doc="<p>The maximum number of results to return in this operation.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the thing type.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[doc="<p>The output for the ListThingTypes operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListThingTypesResponse {
    #[doc="<p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The thing types.</p>"]
    #[serde(rename="thingTypes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_types: Option<Vec<ThingTypeDefinition>>,
}

#[doc="<p>The input for the ListThings operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListThingsRequest {
    #[doc="<p>The attribute name used to search for things.</p>"]
    #[serde(rename="attributeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_name: Option<String>,
    #[doc="<p>The attribute value used to search for things.</p>"]
    #[serde(rename="attributeValue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_value: Option<String>,
    #[doc="<p>The maximum number of results to return in this operation.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the thing type used to search for things.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[doc="<p>The output from the ListThings operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListThingsResponse {
    #[doc="<p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The things.</p>"]
    #[serde(rename="things")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub things: Option<Vec<ThingAttribute>>,
}

#[doc="<p>The input for the ListTopicRules operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTopicRulesRequest {
    #[doc="<p>The maximum number of results to return.</p>"]
    #[serde(rename="maxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="<p>A token used to retrieve the next value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>Specifies whether the rule is disabled.</p>"]
    #[serde(rename="ruleDisabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_disabled: Option<bool>,
    #[doc="<p>The topic.</p>"]
    #[serde(rename="topic")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic: Option<String>,
}

#[doc="<p>The output from the ListTopicRules operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTopicRulesResponse {
    #[doc="<p>A token used to retrieve the next value.</p>"]
    #[serde(rename="nextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="<p>The rules.</p>"]
    #[serde(rename="rules")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rules: Option<Vec<TopicRuleListItem>>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum LogLevel {
    Debug,
    Disabled,
    Error,
    Info,
    Warn,
}

impl Into<String> for LogLevel {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for LogLevel {
    fn into(self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Disabled => "DISABLED",
            LogLevel::Error => "ERROR",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
        }
    }
}

impl ::std::str::FromStr for LogLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEBUG" => Ok(LogLevel::Debug),
            "DISABLED" => Ok(LogLevel::Disabled),
            "ERROR" => Ok(LogLevel::Error),
            "INFO" => Ok(LogLevel::Info),
            "WARN" => Ok(LogLevel::Warn),
            _ => Err(()),
        }
    }
}

#[doc="<p>Describes the logging options payload.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct LoggingOptionsPayload {
    #[doc="<p>The logging level.</p>"]
    #[serde(rename="logLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_level: Option<String>,
    #[doc="<p>The ARN of the IAM role that grants access.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum MessageFormat {
    Json,
    Raw,
}

impl Into<String> for MessageFormat {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for MessageFormat {
    fn into(self) -> &'static str {
        match self {
            MessageFormat::Json => "JSON",
            MessageFormat::Raw => "RAW",
        }
    }
}

impl ::std::str::FromStr for MessageFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JSON" => Ok(MessageFormat::Json),
            "RAW" => Ok(MessageFormat::Raw),
            _ => Err(()),
        }
    }
}

#[doc="<p>A certificate that has been transfered but not yet accepted.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct OutgoingCertificate {
    #[doc="<p>The certificate ARN.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The certificate ID.</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc="<p>The certificate creation date.</p>"]
    #[serde(rename="creationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>The date the transfer was initiated.</p>"]
    #[serde(rename="transferDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transfer_date: Option<f64>,
    #[doc="<p>The transfer message.</p>"]
    #[serde(rename="transferMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transfer_message: Option<String>,
    #[doc="<p>The AWS account to which the transfer was made.</p>"]
    #[serde(rename="transferredTo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transferred_to: Option<String>,
}

#[doc="<p>Describes an AWS IoT policy.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Policy {
    #[doc="<p>The policy ARN.</p>"]
    #[serde(rename="policyArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_arn: Option<String>,
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy_name: Option<String>,
}

#[doc="<p>Describes a policy version.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PolicyVersion {
    #[doc="<p>The date and time the policy was created.</p>"]
    #[serde(rename="createDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_date: Option<f64>,
    #[doc="<p>Specifies whether the policy version is the default.</p>"]
    #[serde(rename="isDefaultVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_default_version: Option<bool>,
    #[doc="<p>The policy version ID.</p>"]
    #[serde(rename="versionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version_id: Option<String>,
}

#[doc="<p>The input for the DynamoActionVS action that specifies the DynamoDB table to which the message data will be written.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PutItemInput {
    #[doc="<p>The table where the message data will be written</p>"]
    #[serde(rename="tableName")]
    pub table_name: String,
}

#[doc="<p>The input to the RegisterCACertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterCACertificateRequest {
    #[doc="<p>Allows this CA certificate to be used for auto registration of device certificates.</p>"]
    #[serde(rename="allowAutoRegistration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_auto_registration: Option<bool>,
    #[doc="<p>The CA certificate.</p>"]
    #[serde(rename="caCertificate")]
    pub ca_certificate: String,
    #[doc="<p>A boolean value that specifies if the CA certificate is set to active.</p>"]
    #[serde(rename="setAsActive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub set_as_active: Option<bool>,
    #[doc="<p>The private key verification certificate.</p>"]
    #[serde(rename="verificationCertificate")]
    pub verification_certificate: String,
}

#[doc="<p>The output from the RegisterCACertificateResponse operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterCACertificateResponse {
    #[doc="<p>The CA certificate ARN.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The CA certificate identifier.</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
}

#[doc="<p>The input to the RegisterCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterCertificateRequest {
    #[doc="<p>The CA certificate used to sign the device certificate being registered.</p>"]
    #[serde(rename="caCertificatePem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ca_certificate_pem: Option<String>,
    #[doc="<p>The certificate data, in PEM format.</p>"]
    #[serde(rename="certificatePem")]
    pub certificate_pem: String,
    #[doc="<p>The status of the register certificate request.</p>"]
    #[serde(rename="status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

#[doc="<p>The output from the RegisterCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterCertificateResponse {
    #[doc="<p>The certificate ARN.</p>"]
    #[serde(rename="certificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The certificate identifier.</p>"]
    #[serde(rename="certificateId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_id: Option<String>,
}

#[doc="<p>The input for the RejectCertificateTransfer operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RejectCertificateTransferRequest {
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
    #[doc="<p>The reason the certificate transfer was rejected.</p>"]
    #[serde(rename="rejectReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reject_reason: Option<String>,
}

#[doc="<p>The input for the ReplaceTopicRule operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ReplaceTopicRuleRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="ruleName")]
    pub rule_name: String,
    #[doc="<p>The rule payload.</p>"]
    #[serde(rename="topicRulePayload")]
    pub topic_rule_payload: TopicRulePayload,
}

#[doc="<p>Describes an action to republish to another topic.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RepublishAction {
    #[doc="<p>The ARN of the IAM role that grants access.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
    #[doc="<p>The name of the MQTT topic.</p>"]
    #[serde(rename="topic")]
    pub topic: String,
}

#[doc="<p>Describes an action to write data to an Amazon S3 bucket.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct S3Action {
    #[doc="<p>The Amazon S3 bucket.</p>"]
    #[serde(rename="bucketName")]
    pub bucket_name: String,
    #[doc="<p>The Amazon S3 canned ACL that controls access to the object identified by the object key. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl\">S3 canned ACLs</a>.</p>"]
    #[serde(rename="cannedAcl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub canned_acl: Option<String>,
    #[doc="<p>The object key.</p>"]
    #[serde(rename="key")]
    pub key: String,
    #[doc="<p>The ARN of the IAM role that grants access.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
}

#[doc="<p>Describes an action to write a message to a Salesforce IoT Cloud Input Stream.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SalesforceAction {
    #[doc="<p>The token used to authenticate access to the Salesforce IoT Cloud Input Stream. The token is available from the Salesforce IoT Cloud platform after creation of the Input Stream.</p>"]
    #[serde(rename="token")]
    pub token: String,
    #[doc="<p>The URL exposed by the Salesforce IoT Cloud Input Stream. The URL is available from the Salesforce IoT Cloud platform after creation of the Input Stream.</p>"]
    #[serde(rename="url")]
    pub url: String,
}

#[doc="<p>The input for the SetDefaultPolicyVersion operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SetDefaultPolicyVersionRequest {
    #[doc="<p>The policy name.</p>"]
    #[serde(rename="policyName")]
    pub policy_name: String,
    #[doc="<p>The policy version ID.</p>"]
    #[serde(rename="policyVersionId")]
    pub policy_version_id: String,
}

#[doc="<p>The input for the SetLoggingOptions operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SetLoggingOptionsRequest {
    #[doc="<p>The logging options payload.</p>"]
    #[serde(rename="loggingOptionsPayload")]
    pub logging_options_payload: LoggingOptionsPayload,
}

#[doc="<p>Describes an action to publish to an Amazon SNS topic.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SnsAction {
    #[doc="<p>The message format of the message to publish. Optional. Accepted values are \"JSON\" and \"RAW\". The default value of the attribute is \"RAW\". SNS uses this setting to determine if the payload should be parsed and relevant platform-specific bits of the payload should be extracted. To read more about SNS message formats, see <a href=\"http://docs.aws.amazon.com/sns/latest/dg/json-formats.html\">http://docs.aws.amazon.com/sns/latest/dg/json-formats.html</a> refer to their official documentation.</p>"]
    #[serde(rename="messageFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message_format: Option<String>,
    #[doc="<p>The ARN of the IAM role that grants access.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
    #[doc="<p>The ARN of the SNS topic.</p>"]
    #[serde(rename="targetArn")]
    pub target_arn: String,
}

#[doc="<p>Describes an action to publish data to an Amazon SQS queue.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SqsAction {
    #[doc="<p>The URL of the Amazon SQS queue.</p>"]
    #[serde(rename="queueUrl")]
    pub queue_url: String,
    #[doc="<p>The ARN of the IAM role that grants access.</p>"]
    #[serde(rename="roleArn")]
    pub role_arn: String,
    #[doc="<p>Specifies whether to use Base64 encoding.</p>"]
    #[serde(rename="useBase64")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_base_64: Option<bool>,
}

#[doc="<p>The properties of the thing, including thing name, thing type name, and a list of thing attributes.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ThingAttribute {
    #[doc="<p>A list of thing attributes which are name-value pairs.</p>"]
    #[serde(rename="attributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[doc="<p>The name of the thing.</p>"]
    #[serde(rename="thingName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_name: Option<String>,
    #[doc="<p>The name of the thing type, if the thing has been associated with a type.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
    #[doc="<p>The version of the thing record in the registry.</p>"]
    #[serde(rename="version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<i64>,
}

#[doc="<p>The definition of the thing type, including thing type name and description.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ThingTypeDefinition {
    #[doc="<p>The ThingTypeMetadata contains additional information about the thing type including: creation date and time, a value indicating whether the thing type is deprecated, and a date and time when it was deprecated.</p>"]
    #[serde(rename="thingTypeMetadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_metadata: Option<ThingTypeMetadata>,
    #[doc="<p>The name of the thing type.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
    #[doc="<p>The ThingTypeProperties for the thing type.</p>"]
    #[serde(rename="thingTypeProperties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

#[doc="<p>The ThingTypeMetadata contains additional information about the thing type including: creation date and time, a value indicating whether the thing type is deprecated, and a date and time when time was deprecated.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ThingTypeMetadata {
    #[doc="<p>The date and time when the thing type was created.</p>"]
    #[serde(rename="creationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="<p>Whether the thing type is deprecated. If <b>true</b>, no new things could be associated with this type.</p>"]
    #[serde(rename="deprecated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deprecated: Option<bool>,
    #[doc="<p>The date and time when the thing type was deprecated.</p>"]
    #[serde(rename="deprecationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deprecation_date: Option<f64>,
}

#[doc="<p>The ThingTypeProperties contains information about the thing type including: a thing type description, and a list of searchable thing attribute names.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ThingTypeProperties {
    #[doc="<p>A list of searchable thing attribute names.</p>"]
    #[serde(rename="searchableAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    #[doc="<p>The description of the thing type.</p>"]
    #[serde(rename="thingTypeDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_description: Option<String>,
}

#[doc="<p>Describes a rule.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TopicRule {
    #[doc="<p>The actions associated with the rule.</p>"]
    #[serde(rename="actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[doc="<p>The version of the SQL rules engine to use when evaluating the rule.</p>"]
    #[serde(rename="awsIotSqlVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aws_iot_sql_version: Option<String>,
    #[doc="<p>The date and time the rule was created.</p>"]
    #[serde(rename="createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p>The description of the rule.</p>"]
    #[serde(rename="description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Specifies whether the rule is disabled.</p>"]
    #[serde(rename="ruleDisabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_disabled: Option<bool>,
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="ruleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_name: Option<String>,
    #[doc="<p>The SQL statement used to query the topic. When using a SQL query with multiple lines, be sure to escape the newline characters.</p>"]
    #[serde(rename="sql")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sql: Option<String>,
}

#[doc="<p>Describes a rule.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TopicRuleListItem {
    #[doc="<p>The date and time the rule was created.</p>"]
    #[serde(rename="createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<f64>,
    #[doc="<p>The rule ARN.</p>"]
    #[serde(rename="ruleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_arn: Option<String>,
    #[doc="<p>Specifies whether the rule is disabled.</p>"]
    #[serde(rename="ruleDisabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_disabled: Option<bool>,
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="ruleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_name: Option<String>,
    #[doc="<p>The pattern for the topic names that apply.</p>"]
    #[serde(rename="topicPattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_pattern: Option<String>,
}

#[doc="<p>Describes a rule.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TopicRulePayload {
    #[doc="<p>The actions associated with the rule.</p>"]
    #[serde(rename="actions")]
    pub actions: Vec<Action>,
    #[doc="<p>The version of the SQL rules engine to use when evaluating the rule.</p>"]
    #[serde(rename="awsIotSqlVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aws_iot_sql_version: Option<String>,
    #[doc="<p>The description of the rule.</p>"]
    #[serde(rename="description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[doc="<p>Specifies whether the rule is disabled.</p>"]
    #[serde(rename="ruleDisabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_disabled: Option<bool>,
    #[doc="<p>The SQL statement used to query the topic. For more information, see <a href=\"http://docs.aws.amazon.com/iot/latest/developerguide/iot-rules.html#aws-iot-sql-reference\">AWS IoT SQL Reference</a> in the <i>AWS IoT Developer Guide</i>.</p>"]
    #[serde(rename="sql")]
    pub sql: String,
}

#[doc="<p>The input for the TransferCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TransferCertificateRequest {
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
    #[doc="<p>The AWS account.</p>"]
    #[serde(rename="targetAwsAccount")]
    pub target_aws_account: String,
    #[doc="<p>The transfer message.</p>"]
    #[serde(rename="transferMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transfer_message: Option<String>,
}

#[doc="<p>The output from the TransferCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TransferCertificateResponse {
    #[doc="<p>The ARN of the certificate.</p>"]
    #[serde(rename="transferredCertificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transferred_certificate_arn: Option<String>,
}

#[doc="<p>Data used to transfer a certificate to an AWS account.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TransferData {
    #[doc="<p>The date the transfer was accepted.</p>"]
    #[serde(rename="acceptDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accept_date: Option<f64>,
    #[doc="<p>The date the transfer was rejected.</p>"]
    #[serde(rename="rejectDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reject_date: Option<f64>,
    #[doc="<p>The reason why the transfer was rejected.</p>"]
    #[serde(rename="rejectReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reject_reason: Option<String>,
    #[doc="<p>The date the transfer took place.</p>"]
    #[serde(rename="transferDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transfer_date: Option<f64>,
    #[doc="<p>The transfer message.</p>"]
    #[serde(rename="transferMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transfer_message: Option<String>,
}

#[doc="<p>The input to the UpdateCACertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateCACertificateRequest {
    #[doc="<p>The CA certificate identifier.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
    #[doc="<p>The new value for the auto registration status. Valid values are: \"ENABLE\" or \"DISABLE\".</p>"]
    #[serde(rename="newAutoRegistrationStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_auto_registration_status: Option<String>,
    #[doc="<p>The updated status of the CA certificate.</p> <p> <b>Note:</b> The status value REGISTER_INACTIVE is deprecated and should not be used.</p>"]
    #[serde(rename="newStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_status: Option<String>,
}

#[doc="<p>The input for the UpdateCertificate operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateCertificateRequest {
    #[doc="<p>The ID of the certificate.</p>"]
    #[serde(rename="certificateId")]
    pub certificate_id: String,
    #[doc="<p>The new status.</p> <p> <b>Note:</b> Setting the status to PENDING_TRANSFER will result in an exception being thrown. PENDING_TRANSFER is a status used internally by AWS IoT. It is not intended for developer use.</p> <p> <b>Note:</b> The status value REGISTER_INACTIVE is deprecated and should not be used.</p>"]
    #[serde(rename="newStatus")]
    pub new_status: String,
}

#[doc="<p>The input for the UpdateThing operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateThingRequest {
    #[doc="<p>A list of thing attributes, a JSON string containing name-value pairs. For example:</p> <p> <code>{\\\"attributes\\\":{\\\"name1\\\":\\\"value2\\\"}}</code> </p> <p>This data is used to add new attributes or update existing attributes.</p>"]
    #[serde(rename="attributePayload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,
    #[doc="<p>The expected version of the thing record in the registry. If the version of the record in the registry does not match the expected version specified in the request, the <code>UpdateThing</code> request is rejected with a <code>VersionConflictException</code>.</p>"]
    #[serde(rename="expectedVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expected_version: Option<i64>,
    #[doc="<p>Remove a thing type association. If <b>true</b>, the assocation is removed.</p>"]
    #[serde(rename="removeThingType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_thing_type: Option<bool>,
    #[doc="<p>The name of the thing to update.</p>"]
    #[serde(rename="thingName")]
    pub thing_name: String,
    #[doc="<p>The name of the thing type.</p>"]
    #[serde(rename="thingTypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[doc="<p>The output from the UpdateThing operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateThingResponse;

/// Errors returned by AcceptCertificateTransfer
#[derive(Debug, PartialEq)]
pub enum AcceptCertificateTransferError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You can't revert the certificate transfer because the transfer is already complete.</p>
    TransferAlreadyCompleted(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AcceptCertificateTransferError {
    pub fn from_body(body: &str) -> AcceptCertificateTransferError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        AcceptCertificateTransferError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AcceptCertificateTransferError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => AcceptCertificateTransferError::ResourceNotFound(String::from(error_message)),
                    "ServiceUnavailableException" => AcceptCertificateTransferError::ServiceUnavailable(String::from(error_message)),
                    "ThrottlingException" => {
                        AcceptCertificateTransferError::Throttling(String::from(error_message))
                    }
                    "TransferAlreadyCompletedException" => AcceptCertificateTransferError::TransferAlreadyCompleted(String::from(error_message)),
                    "UnauthorizedException" => {
                        AcceptCertificateTransferError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        AcceptCertificateTransferError::Validation(error_message.to_string())
                    }
                    _ => AcceptCertificateTransferError::Unknown(String::from(body)),
                }
            }
            Err(_) => AcceptCertificateTransferError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AcceptCertificateTransferError {
    fn from(err: serde_json::error::Error) -> AcceptCertificateTransferError {
        AcceptCertificateTransferError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AcceptCertificateTransferError {
    fn from(err: CredentialsError) -> AcceptCertificateTransferError {
        AcceptCertificateTransferError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcceptCertificateTransferError {
    fn from(err: HttpDispatchError) -> AcceptCertificateTransferError {
        AcceptCertificateTransferError::HttpDispatch(err)
    }
}
impl fmt::Display for AcceptCertificateTransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptCertificateTransferError {
    fn description(&self) -> &str {
        match *self {
            AcceptCertificateTransferError::InternalFailure(ref cause) => cause,
            AcceptCertificateTransferError::InvalidRequest(ref cause) => cause,
            AcceptCertificateTransferError::ResourceNotFound(ref cause) => cause,
            AcceptCertificateTransferError::ServiceUnavailable(ref cause) => cause,
            AcceptCertificateTransferError::Throttling(ref cause) => cause,
            AcceptCertificateTransferError::TransferAlreadyCompleted(ref cause) => cause,
            AcceptCertificateTransferError::Unauthorized(ref cause) => cause,
            AcceptCertificateTransferError::Validation(ref cause) => cause,
            AcceptCertificateTransferError::Credentials(ref err) => err.description(),
            AcceptCertificateTransferError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AcceptCertificateTransferError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachPrincipalPolicy
#[derive(Debug, PartialEq)]
pub enum AttachPrincipalPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AttachPrincipalPolicyError {
    pub fn from_body(body: &str) -> AttachPrincipalPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        AttachPrincipalPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AttachPrincipalPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AttachPrincipalPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachPrincipalPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AttachPrincipalPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        AttachPrincipalPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        AttachPrincipalPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachPrincipalPolicyError::Validation(error_message.to_string())
                    }
                    _ => AttachPrincipalPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachPrincipalPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachPrincipalPolicyError {
    fn from(err: serde_json::error::Error) -> AttachPrincipalPolicyError {
        AttachPrincipalPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachPrincipalPolicyError {
    fn from(err: CredentialsError) -> AttachPrincipalPolicyError {
        AttachPrincipalPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachPrincipalPolicyError {
    fn from(err: HttpDispatchError) -> AttachPrincipalPolicyError {
        AttachPrincipalPolicyError::HttpDispatch(err)
    }
}
impl fmt::Display for AttachPrincipalPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachPrincipalPolicyError {
    fn description(&self) -> &str {
        match *self {
            AttachPrincipalPolicyError::InternalFailure(ref cause) => cause,
            AttachPrincipalPolicyError::InvalidRequest(ref cause) => cause,
            AttachPrincipalPolicyError::LimitExceeded(ref cause) => cause,
            AttachPrincipalPolicyError::ResourceNotFound(ref cause) => cause,
            AttachPrincipalPolicyError::ServiceUnavailable(ref cause) => cause,
            AttachPrincipalPolicyError::Throttling(ref cause) => cause,
            AttachPrincipalPolicyError::Unauthorized(ref cause) => cause,
            AttachPrincipalPolicyError::Validation(ref cause) => cause,
            AttachPrincipalPolicyError::Credentials(ref err) => err.description(),
            AttachPrincipalPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachPrincipalPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachThingPrincipal
#[derive(Debug, PartialEq)]
pub enum AttachThingPrincipalError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AttachThingPrincipalError {
    pub fn from_body(body: &str) -> AttachThingPrincipalError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        AttachThingPrincipalError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AttachThingPrincipalError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachThingPrincipalError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AttachThingPrincipalError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        AttachThingPrincipalError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        AttachThingPrincipalError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachThingPrincipalError::Validation(error_message.to_string())
                    }
                    _ => AttachThingPrincipalError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachThingPrincipalError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachThingPrincipalError {
    fn from(err: serde_json::error::Error) -> AttachThingPrincipalError {
        AttachThingPrincipalError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachThingPrincipalError {
    fn from(err: CredentialsError) -> AttachThingPrincipalError {
        AttachThingPrincipalError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachThingPrincipalError {
    fn from(err: HttpDispatchError) -> AttachThingPrincipalError {
        AttachThingPrincipalError::HttpDispatch(err)
    }
}
impl fmt::Display for AttachThingPrincipalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachThingPrincipalError {
    fn description(&self) -> &str {
        match *self {
            AttachThingPrincipalError::InternalFailure(ref cause) => cause,
            AttachThingPrincipalError::InvalidRequest(ref cause) => cause,
            AttachThingPrincipalError::ResourceNotFound(ref cause) => cause,
            AttachThingPrincipalError::ServiceUnavailable(ref cause) => cause,
            AttachThingPrincipalError::Throttling(ref cause) => cause,
            AttachThingPrincipalError::Unauthorized(ref cause) => cause,
            AttachThingPrincipalError::Validation(ref cause) => cause,
            AttachThingPrincipalError::Credentials(ref err) => err.description(),
            AttachThingPrincipalError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachThingPrincipalError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelCertificateTransfer
#[derive(Debug, PartialEq)]
pub enum CancelCertificateTransferError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You can't revert the certificate transfer because the transfer is already complete.</p>
    TransferAlreadyCompleted(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CancelCertificateTransferError {
    pub fn from_body(body: &str) -> CancelCertificateTransferError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CancelCertificateTransferError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CancelCertificateTransferError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => CancelCertificateTransferError::ResourceNotFound(String::from(error_message)),
                    "ServiceUnavailableException" => CancelCertificateTransferError::ServiceUnavailable(String::from(error_message)),
                    "ThrottlingException" => {
                        CancelCertificateTransferError::Throttling(String::from(error_message))
                    }
                    "TransferAlreadyCompletedException" => CancelCertificateTransferError::TransferAlreadyCompleted(String::from(error_message)),
                    "UnauthorizedException" => {
                        CancelCertificateTransferError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelCertificateTransferError::Validation(error_message.to_string())
                    }
                    _ => CancelCertificateTransferError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelCertificateTransferError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelCertificateTransferError {
    fn from(err: serde_json::error::Error) -> CancelCertificateTransferError {
        CancelCertificateTransferError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelCertificateTransferError {
    fn from(err: CredentialsError) -> CancelCertificateTransferError {
        CancelCertificateTransferError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelCertificateTransferError {
    fn from(err: HttpDispatchError) -> CancelCertificateTransferError {
        CancelCertificateTransferError::HttpDispatch(err)
    }
}
impl fmt::Display for CancelCertificateTransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelCertificateTransferError {
    fn description(&self) -> &str {
        match *self {
            CancelCertificateTransferError::InternalFailure(ref cause) => cause,
            CancelCertificateTransferError::InvalidRequest(ref cause) => cause,
            CancelCertificateTransferError::ResourceNotFound(ref cause) => cause,
            CancelCertificateTransferError::ServiceUnavailable(ref cause) => cause,
            CancelCertificateTransferError::Throttling(ref cause) => cause,
            CancelCertificateTransferError::TransferAlreadyCompleted(ref cause) => cause,
            CancelCertificateTransferError::Unauthorized(ref cause) => cause,
            CancelCertificateTransferError::Validation(ref cause) => cause,
            CancelCertificateTransferError::Credentials(ref err) => err.description(),
            CancelCertificateTransferError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CancelCertificateTransferError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCertificateFromCsr
#[derive(Debug, PartialEq)]
pub enum CreateCertificateFromCsrError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateCertificateFromCsrError {
    pub fn from_body(body: &str) -> CreateCertificateFromCsrError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateCertificateFromCsrError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateCertificateFromCsrError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => CreateCertificateFromCsrError::ServiceUnavailable(String::from(error_message)),
                    "ThrottlingException" => {
                        CreateCertificateFromCsrError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateCertificateFromCsrError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateCertificateFromCsrError::Validation(error_message.to_string())
                    }
                    _ => CreateCertificateFromCsrError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCertificateFromCsrError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCertificateFromCsrError {
    fn from(err: serde_json::error::Error) -> CreateCertificateFromCsrError {
        CreateCertificateFromCsrError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCertificateFromCsrError {
    fn from(err: CredentialsError) -> CreateCertificateFromCsrError {
        CreateCertificateFromCsrError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCertificateFromCsrError {
    fn from(err: HttpDispatchError) -> CreateCertificateFromCsrError {
        CreateCertificateFromCsrError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateCertificateFromCsrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCertificateFromCsrError {
    fn description(&self) -> &str {
        match *self {
            CreateCertificateFromCsrError::InternalFailure(ref cause) => cause,
            CreateCertificateFromCsrError::InvalidRequest(ref cause) => cause,
            CreateCertificateFromCsrError::ServiceUnavailable(ref cause) => cause,
            CreateCertificateFromCsrError::Throttling(ref cause) => cause,
            CreateCertificateFromCsrError::Unauthorized(ref cause) => cause,
            CreateCertificateFromCsrError::Validation(ref cause) => cause,
            CreateCertificateFromCsrError::Credentials(ref err) => err.description(),
            CreateCertificateFromCsrError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCertificateFromCsrError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateKeysAndCertificate
#[derive(Debug, PartialEq)]
pub enum CreateKeysAndCertificateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateKeysAndCertificateError {
    pub fn from_body(body: &str) -> CreateKeysAndCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateKeysAndCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateKeysAndCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => CreateKeysAndCertificateError::ServiceUnavailable(String::from(error_message)),
                    "ThrottlingException" => {
                        CreateKeysAndCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateKeysAndCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateKeysAndCertificateError::Validation(error_message.to_string())
                    }
                    _ => CreateKeysAndCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateKeysAndCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateKeysAndCertificateError {
    fn from(err: serde_json::error::Error) -> CreateKeysAndCertificateError {
        CreateKeysAndCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateKeysAndCertificateError {
    fn from(err: CredentialsError) -> CreateKeysAndCertificateError {
        CreateKeysAndCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateKeysAndCertificateError {
    fn from(err: HttpDispatchError) -> CreateKeysAndCertificateError {
        CreateKeysAndCertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateKeysAndCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateKeysAndCertificateError {
    fn description(&self) -> &str {
        match *self {
            CreateKeysAndCertificateError::InternalFailure(ref cause) => cause,
            CreateKeysAndCertificateError::InvalidRequest(ref cause) => cause,
            CreateKeysAndCertificateError::ServiceUnavailable(ref cause) => cause,
            CreateKeysAndCertificateError::Throttling(ref cause) => cause,
            CreateKeysAndCertificateError::Unauthorized(ref cause) => cause,
            CreateKeysAndCertificateError::Validation(ref cause) => cause,
            CreateKeysAndCertificateError::Credentials(ref err) => err.description(),
            CreateKeysAndCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateKeysAndCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePolicy
#[derive(Debug, PartialEq)]
pub enum CreatePolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The policy documentation is not valid.</p>
    MalformedPolicy(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePolicyError {
    pub fn from_body(body: &str) -> CreatePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreatePolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreatePolicyError::InvalidRequest(String::from(error_message))
                    }
                    "MalformedPolicyException" => {
                        CreatePolicyError::MalformedPolicy(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreatePolicyError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreatePolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreatePolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreatePolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePolicyError::Validation(error_message.to_string())
                    }
                    _ => CreatePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePolicyError {
    fn from(err: serde_json::error::Error) -> CreatePolicyError {
        CreatePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePolicyError {
    fn from(err: CredentialsError) -> CreatePolicyError {
        CreatePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePolicyError {
    fn from(err: HttpDispatchError) -> CreatePolicyError {
        CreatePolicyError::HttpDispatch(err)
    }
}
impl fmt::Display for CreatePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePolicyError {
    fn description(&self) -> &str {
        match *self {
            CreatePolicyError::InternalFailure(ref cause) => cause,
            CreatePolicyError::InvalidRequest(ref cause) => cause,
            CreatePolicyError::MalformedPolicy(ref cause) => cause,
            CreatePolicyError::ResourceAlreadyExists(ref cause) => cause,
            CreatePolicyError::ServiceUnavailable(ref cause) => cause,
            CreatePolicyError::Throttling(ref cause) => cause,
            CreatePolicyError::Unauthorized(ref cause) => cause,
            CreatePolicyError::Validation(ref cause) => cause,
            CreatePolicyError::Credentials(ref err) => err.description(),
            CreatePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePolicyVersion
#[derive(Debug, PartialEq)]
pub enum CreatePolicyVersionError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The policy documentation is not valid.</p>
    MalformedPolicy(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>The number of policy versions exceeds the limit.</p>
    VersionsLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreatePolicyVersionError {
    pub fn from_body(body: &str) -> CreatePolicyVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreatePolicyVersionError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreatePolicyVersionError::InvalidRequest(String::from(error_message))
                    }
                    "MalformedPolicyException" => {
                        CreatePolicyVersionError::MalformedPolicy(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreatePolicyVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreatePolicyVersionError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreatePolicyVersionError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreatePolicyVersionError::Unauthorized(String::from(error_message))
                    }
                    "VersionsLimitExceededException" => {
                        CreatePolicyVersionError::VersionsLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePolicyVersionError::Validation(error_message.to_string())
                    }
                    _ => CreatePolicyVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePolicyVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePolicyVersionError {
    fn from(err: serde_json::error::Error) -> CreatePolicyVersionError {
        CreatePolicyVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePolicyVersionError {
    fn from(err: CredentialsError) -> CreatePolicyVersionError {
        CreatePolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePolicyVersionError {
    fn from(err: HttpDispatchError) -> CreatePolicyVersionError {
        CreatePolicyVersionError::HttpDispatch(err)
    }
}
impl fmt::Display for CreatePolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            CreatePolicyVersionError::InternalFailure(ref cause) => cause,
            CreatePolicyVersionError::InvalidRequest(ref cause) => cause,
            CreatePolicyVersionError::MalformedPolicy(ref cause) => cause,
            CreatePolicyVersionError::ResourceNotFound(ref cause) => cause,
            CreatePolicyVersionError::ServiceUnavailable(ref cause) => cause,
            CreatePolicyVersionError::Throttling(ref cause) => cause,
            CreatePolicyVersionError::Unauthorized(ref cause) => cause,
            CreatePolicyVersionError::VersionsLimitExceeded(ref cause) => cause,
            CreatePolicyVersionError::Validation(ref cause) => cause,
            CreatePolicyVersionError::Credentials(ref err) => err.description(),
            CreatePolicyVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateThing
#[derive(Debug, PartialEq)]
pub enum CreateThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateThingError {
    pub fn from_body(body: &str) -> CreateThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateThingError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateThingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateThingError::Validation(error_message.to_string())
                    }
                    _ => CreateThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateThingError {
    fn from(err: serde_json::error::Error) -> CreateThingError {
        CreateThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateThingError {
    fn from(err: CredentialsError) -> CreateThingError {
        CreateThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateThingError {
    fn from(err: HttpDispatchError) -> CreateThingError {
        CreateThingError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateThingError {
    fn description(&self) -> &str {
        match *self {
            CreateThingError::InternalFailure(ref cause) => cause,
            CreateThingError::InvalidRequest(ref cause) => cause,
            CreateThingError::ResourceAlreadyExists(ref cause) => cause,
            CreateThingError::ResourceNotFound(ref cause) => cause,
            CreateThingError::ServiceUnavailable(ref cause) => cause,
            CreateThingError::Throttling(ref cause) => cause,
            CreateThingError::Unauthorized(ref cause) => cause,
            CreateThingError::Validation(ref cause) => cause,
            CreateThingError::Credentials(ref err) => err.description(),
            CreateThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateThingType
#[derive(Debug, PartialEq)]
pub enum CreateThingTypeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateThingTypeError {
    pub fn from_body(body: &str) -> CreateThingTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateThingTypeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateThingTypeError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateThingTypeError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateThingTypeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateThingTypeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateThingTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateThingTypeError::Validation(error_message.to_string())
                    }
                    _ => CreateThingTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateThingTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateThingTypeError {
    fn from(err: serde_json::error::Error) -> CreateThingTypeError {
        CreateThingTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateThingTypeError {
    fn from(err: CredentialsError) -> CreateThingTypeError {
        CreateThingTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateThingTypeError {
    fn from(err: HttpDispatchError) -> CreateThingTypeError {
        CreateThingTypeError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateThingTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateThingTypeError {
    fn description(&self) -> &str {
        match *self {
            CreateThingTypeError::InternalFailure(ref cause) => cause,
            CreateThingTypeError::InvalidRequest(ref cause) => cause,
            CreateThingTypeError::ResourceAlreadyExists(ref cause) => cause,
            CreateThingTypeError::ServiceUnavailable(ref cause) => cause,
            CreateThingTypeError::Throttling(ref cause) => cause,
            CreateThingTypeError::Unauthorized(ref cause) => cause,
            CreateThingTypeError::Validation(ref cause) => cause,
            CreateThingTypeError::Credentials(ref err) => err.description(),
            CreateThingTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateThingTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTopicRule
#[derive(Debug, PartialEq)]
pub enum CreateTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The Rule-SQL expression can't be parsed correctly.</p>
    SqlParse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateTopicRuleError {
    pub fn from_body(body: &str) -> CreateTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        CreateTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateTopicRuleError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "SqlParseException" => {
                        CreateTopicRuleError::SqlParse(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => CreateTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTopicRuleError {
    fn from(err: serde_json::error::Error) -> CreateTopicRuleError {
        CreateTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTopicRuleError {
    fn from(err: CredentialsError) -> CreateTopicRuleError {
        CreateTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTopicRuleError {
    fn from(err: HttpDispatchError) -> CreateTopicRuleError {
        CreateTopicRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            CreateTopicRuleError::Internal(ref cause) => cause,
            CreateTopicRuleError::InvalidRequest(ref cause) => cause,
            CreateTopicRuleError::ResourceAlreadyExists(ref cause) => cause,
            CreateTopicRuleError::ServiceUnavailable(ref cause) => cause,
            CreateTopicRuleError::SqlParse(ref cause) => cause,
            CreateTopicRuleError::Validation(ref cause) => cause,
            CreateTopicRuleError::Credentials(ref err) => err.description(),
            CreateTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCACertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCACertificateError {
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteCACertificateError {
    pub fn from_body(body: &str) -> DeleteCACertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateStateException" => {
                        DeleteCACertificateError::CertificateState(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeleteCACertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteCACertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteCACertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteCACertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteCACertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteCACertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCACertificateError::Validation(error_message.to_string())
                    }
                    _ => DeleteCACertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCACertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCACertificateError {
    fn from(err: serde_json::error::Error) -> DeleteCACertificateError {
        DeleteCACertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCACertificateError {
    fn from(err: CredentialsError) -> DeleteCACertificateError {
        DeleteCACertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCACertificateError {
    fn from(err: HttpDispatchError) -> DeleteCACertificateError {
        DeleteCACertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteCACertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCACertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteCACertificateError::CertificateState(ref cause) => cause,
            DeleteCACertificateError::InternalFailure(ref cause) => cause,
            DeleteCACertificateError::InvalidRequest(ref cause) => cause,
            DeleteCACertificateError::ResourceNotFound(ref cause) => cause,
            DeleteCACertificateError::ServiceUnavailable(ref cause) => cause,
            DeleteCACertificateError::Throttling(ref cause) => cause,
            DeleteCACertificateError::Unauthorized(ref cause) => cause,
            DeleteCACertificateError::Validation(ref cause) => cause,
            DeleteCACertificateError::Credentials(ref err) => err.description(),
            DeleteCACertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCACertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCertificateError {
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteCertificateError {
    pub fn from_body(body: &str) -> DeleteCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateStateException" => {
                        DeleteCertificateError::CertificateState(String::from(error_message))
                    }
                    "DeleteConflictException" => {
                        DeleteCertificateError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeleteCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCertificateError::Validation(error_message.to_string())
                    }
                    _ => DeleteCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCertificateError {
    fn from(err: serde_json::error::Error) -> DeleteCertificateError {
        DeleteCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCertificateError {
    fn from(err: CredentialsError) -> DeleteCertificateError {
        DeleteCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCertificateError {
    fn from(err: HttpDispatchError) -> DeleteCertificateError {
        DeleteCertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteCertificateError::CertificateState(ref cause) => cause,
            DeleteCertificateError::DeleteConflict(ref cause) => cause,
            DeleteCertificateError::InternalFailure(ref cause) => cause,
            DeleteCertificateError::InvalidRequest(ref cause) => cause,
            DeleteCertificateError::ResourceNotFound(ref cause) => cause,
            DeleteCertificateError::ServiceUnavailable(ref cause) => cause,
            DeleteCertificateError::Throttling(ref cause) => cause,
            DeleteCertificateError::Unauthorized(ref cause) => cause,
            DeleteCertificateError::Validation(ref cause) => cause,
            DeleteCertificateError::Credentials(ref err) => err.description(),
            DeleteCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeletePolicyError {
    pub fn from_body(body: &str) -> DeletePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DeleteConflictException" => {
                        DeletePolicyError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeletePolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeletePolicyError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeletePolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeletePolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeletePolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeletePolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePolicyError::Validation(error_message.to_string())
                    }
                    _ => DeletePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePolicyError {
    fn from(err: serde_json::error::Error) -> DeletePolicyError {
        DeletePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePolicyError {
    fn from(err: CredentialsError) -> DeletePolicyError {
        DeletePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePolicyError {
    fn from(err: HttpDispatchError) -> DeletePolicyError {
        DeletePolicyError::HttpDispatch(err)
    }
}
impl fmt::Display for DeletePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeletePolicyError::DeleteConflict(ref cause) => cause,
            DeletePolicyError::InternalFailure(ref cause) => cause,
            DeletePolicyError::InvalidRequest(ref cause) => cause,
            DeletePolicyError::ResourceNotFound(ref cause) => cause,
            DeletePolicyError::ServiceUnavailable(ref cause) => cause,
            DeletePolicyError::Throttling(ref cause) => cause,
            DeletePolicyError::Unauthorized(ref cause) => cause,
            DeletePolicyError::Validation(ref cause) => cause,
            DeletePolicyError::Credentials(ref err) => err.description(),
            DeletePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePolicyVersion
#[derive(Debug, PartialEq)]
pub enum DeletePolicyVersionError {
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeletePolicyVersionError {
    pub fn from_body(body: &str) -> DeletePolicyVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DeleteConflictException" => {
                        DeletePolicyVersionError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeletePolicyVersionError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeletePolicyVersionError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeletePolicyVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeletePolicyVersionError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeletePolicyVersionError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeletePolicyVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePolicyVersionError::Validation(error_message.to_string())
                    }
                    _ => DeletePolicyVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePolicyVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePolicyVersionError {
    fn from(err: serde_json::error::Error) -> DeletePolicyVersionError {
        DeletePolicyVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePolicyVersionError {
    fn from(err: CredentialsError) -> DeletePolicyVersionError {
        DeletePolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePolicyVersionError {
    fn from(err: HttpDispatchError) -> DeletePolicyVersionError {
        DeletePolicyVersionError::HttpDispatch(err)
    }
}
impl fmt::Display for DeletePolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            DeletePolicyVersionError::DeleteConflict(ref cause) => cause,
            DeletePolicyVersionError::InternalFailure(ref cause) => cause,
            DeletePolicyVersionError::InvalidRequest(ref cause) => cause,
            DeletePolicyVersionError::ResourceNotFound(ref cause) => cause,
            DeletePolicyVersionError::ServiceUnavailable(ref cause) => cause,
            DeletePolicyVersionError::Throttling(ref cause) => cause,
            DeletePolicyVersionError::Unauthorized(ref cause) => cause,
            DeletePolicyVersionError::Validation(ref cause) => cause,
            DeletePolicyVersionError::Credentials(ref err) => err.description(),
            DeletePolicyVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeletePolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRegistrationCode
#[derive(Debug, PartialEq)]
pub enum DeleteRegistrationCodeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteRegistrationCodeError {
    pub fn from_body(body: &str) -> DeleteRegistrationCodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteRegistrationCodeError::InternalFailure(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteRegistrationCodeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteRegistrationCodeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteRegistrationCodeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteRegistrationCodeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRegistrationCodeError::Validation(error_message.to_string())
                    }
                    _ => DeleteRegistrationCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRegistrationCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRegistrationCodeError {
    fn from(err: serde_json::error::Error) -> DeleteRegistrationCodeError {
        DeleteRegistrationCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRegistrationCodeError {
    fn from(err: CredentialsError) -> DeleteRegistrationCodeError {
        DeleteRegistrationCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRegistrationCodeError {
    fn from(err: HttpDispatchError) -> DeleteRegistrationCodeError {
        DeleteRegistrationCodeError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteRegistrationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRegistrationCodeError {
    fn description(&self) -> &str {
        match *self {
            DeleteRegistrationCodeError::InternalFailure(ref cause) => cause,
            DeleteRegistrationCodeError::ResourceNotFound(ref cause) => cause,
            DeleteRegistrationCodeError::ServiceUnavailable(ref cause) => cause,
            DeleteRegistrationCodeError::Throttling(ref cause) => cause,
            DeleteRegistrationCodeError::Unauthorized(ref cause) => cause,
            DeleteRegistrationCodeError::Validation(ref cause) => cause,
            DeleteRegistrationCodeError::Credentials(ref err) => err.description(),
            DeleteRegistrationCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRegistrationCodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteThing
#[derive(Debug, PartialEq)]
pub enum DeleteThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>An exception thrown when the version of a thing passed to a command is different than the version specified with the --version parameter.</p>
    VersionConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteThingError {
    pub fn from_body(body: &str) -> DeleteThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteThingError::Unauthorized(String::from(error_message))
                    }
                    "VersionConflictException" => {
                        DeleteThingError::VersionConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteThingError::Validation(error_message.to_string())
                    }
                    _ => DeleteThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteThingError {
    fn from(err: serde_json::error::Error) -> DeleteThingError {
        DeleteThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteThingError {
    fn from(err: CredentialsError) -> DeleteThingError {
        DeleteThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteThingError {
    fn from(err: HttpDispatchError) -> DeleteThingError {
        DeleteThingError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteThingError {
    fn description(&self) -> &str {
        match *self {
            DeleteThingError::InternalFailure(ref cause) => cause,
            DeleteThingError::InvalidRequest(ref cause) => cause,
            DeleteThingError::ResourceNotFound(ref cause) => cause,
            DeleteThingError::ServiceUnavailable(ref cause) => cause,
            DeleteThingError::Throttling(ref cause) => cause,
            DeleteThingError::Unauthorized(ref cause) => cause,
            DeleteThingError::VersionConflict(ref cause) => cause,
            DeleteThingError::Validation(ref cause) => cause,
            DeleteThingError::Credentials(ref err) => err.description(),
            DeleteThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteThingType
#[derive(Debug, PartialEq)]
pub enum DeleteThingTypeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteThingTypeError {
    pub fn from_body(body: &str) -> DeleteThingTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteThingTypeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteThingTypeError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteThingTypeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteThingTypeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteThingTypeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteThingTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteThingTypeError::Validation(error_message.to_string())
                    }
                    _ => DeleteThingTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteThingTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteThingTypeError {
    fn from(err: serde_json::error::Error) -> DeleteThingTypeError {
        DeleteThingTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteThingTypeError {
    fn from(err: CredentialsError) -> DeleteThingTypeError {
        DeleteThingTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteThingTypeError {
    fn from(err: HttpDispatchError) -> DeleteThingTypeError {
        DeleteThingTypeError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteThingTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteThingTypeError {
    fn description(&self) -> &str {
        match *self {
            DeleteThingTypeError::InternalFailure(ref cause) => cause,
            DeleteThingTypeError::InvalidRequest(ref cause) => cause,
            DeleteThingTypeError::ResourceNotFound(ref cause) => cause,
            DeleteThingTypeError::ServiceUnavailable(ref cause) => cause,
            DeleteThingTypeError::Throttling(ref cause) => cause,
            DeleteThingTypeError::Unauthorized(ref cause) => cause,
            DeleteThingTypeError::Validation(ref cause) => cause,
            DeleteThingTypeError::Credentials(ref err) => err.description(),
            DeleteThingTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteThingTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTopicRule
#[derive(Debug, PartialEq)]
pub enum DeleteTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteTopicRuleError {
    pub fn from_body(body: &str) -> DeleteTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        DeleteTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => DeleteTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTopicRuleError {
    fn from(err: serde_json::error::Error) -> DeleteTopicRuleError {
        DeleteTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTopicRuleError {
    fn from(err: CredentialsError) -> DeleteTopicRuleError {
        DeleteTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTopicRuleError {
    fn from(err: HttpDispatchError) -> DeleteTopicRuleError {
        DeleteTopicRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteTopicRuleError::Internal(ref cause) => cause,
            DeleteTopicRuleError::InvalidRequest(ref cause) => cause,
            DeleteTopicRuleError::ServiceUnavailable(ref cause) => cause,
            DeleteTopicRuleError::Unauthorized(ref cause) => cause,
            DeleteTopicRuleError::Validation(ref cause) => cause,
            DeleteTopicRuleError::Credentials(ref err) => err.description(),
            DeleteTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeprecateThingType
#[derive(Debug, PartialEq)]
pub enum DeprecateThingTypeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeprecateThingTypeError {
    pub fn from_body(body: &str) -> DeprecateThingTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeprecateThingTypeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeprecateThingTypeError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeprecateThingTypeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeprecateThingTypeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeprecateThingTypeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeprecateThingTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeprecateThingTypeError::Validation(error_message.to_string())
                    }
                    _ => DeprecateThingTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeprecateThingTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeprecateThingTypeError {
    fn from(err: serde_json::error::Error) -> DeprecateThingTypeError {
        DeprecateThingTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeprecateThingTypeError {
    fn from(err: CredentialsError) -> DeprecateThingTypeError {
        DeprecateThingTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeprecateThingTypeError {
    fn from(err: HttpDispatchError) -> DeprecateThingTypeError {
        DeprecateThingTypeError::HttpDispatch(err)
    }
}
impl fmt::Display for DeprecateThingTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeprecateThingTypeError {
    fn description(&self) -> &str {
        match *self {
            DeprecateThingTypeError::InternalFailure(ref cause) => cause,
            DeprecateThingTypeError::InvalidRequest(ref cause) => cause,
            DeprecateThingTypeError::ResourceNotFound(ref cause) => cause,
            DeprecateThingTypeError::ServiceUnavailable(ref cause) => cause,
            DeprecateThingTypeError::Throttling(ref cause) => cause,
            DeprecateThingTypeError::Unauthorized(ref cause) => cause,
            DeprecateThingTypeError::Validation(ref cause) => cause,
            DeprecateThingTypeError::Credentials(ref err) => err.description(),
            DeprecateThingTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeprecateThingTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCACertificate
#[derive(Debug, PartialEq)]
pub enum DescribeCACertificateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCACertificateError {
    pub fn from_body(body: &str) -> DescribeCACertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeCACertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeCACertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeCACertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeCACertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeCACertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeCACertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCACertificateError::Validation(error_message.to_string())
                    }
                    _ => DescribeCACertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCACertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCACertificateError {
    fn from(err: serde_json::error::Error) -> DescribeCACertificateError {
        DescribeCACertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCACertificateError {
    fn from(err: CredentialsError) -> DescribeCACertificateError {
        DescribeCACertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCACertificateError {
    fn from(err: HttpDispatchError) -> DescribeCACertificateError {
        DescribeCACertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCACertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCACertificateError {
    fn description(&self) -> &str {
        match *self {
            DescribeCACertificateError::InternalFailure(ref cause) => cause,
            DescribeCACertificateError::InvalidRequest(ref cause) => cause,
            DescribeCACertificateError::ResourceNotFound(ref cause) => cause,
            DescribeCACertificateError::ServiceUnavailable(ref cause) => cause,
            DescribeCACertificateError::Throttling(ref cause) => cause,
            DescribeCACertificateError::Unauthorized(ref cause) => cause,
            DescribeCACertificateError::Validation(ref cause) => cause,
            DescribeCACertificateError::Credentials(ref err) => err.description(),
            DescribeCACertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCACertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCertificate
#[derive(Debug, PartialEq)]
pub enum DescribeCertificateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCertificateError {
    pub fn from_body(body: &str) -> DescribeCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCertificateError::Validation(error_message.to_string())
                    }
                    _ => DescribeCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCertificateError {
    fn from(err: serde_json::error::Error) -> DescribeCertificateError {
        DescribeCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCertificateError {
    fn from(err: CredentialsError) -> DescribeCertificateError {
        DescribeCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCertificateError {
    fn from(err: HttpDispatchError) -> DescribeCertificateError {
        DescribeCertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCertificateError {
    fn description(&self) -> &str {
        match *self {
            DescribeCertificateError::InternalFailure(ref cause) => cause,
            DescribeCertificateError::InvalidRequest(ref cause) => cause,
            DescribeCertificateError::ResourceNotFound(ref cause) => cause,
            DescribeCertificateError::ServiceUnavailable(ref cause) => cause,
            DescribeCertificateError::Throttling(ref cause) => cause,
            DescribeCertificateError::Unauthorized(ref cause) => cause,
            DescribeCertificateError::Validation(ref cause) => cause,
            DescribeCertificateError::Credentials(ref err) => err.description(),
            DescribeCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEndpoint
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEndpointError {
    pub fn from_body(body: &str) -> DescribeEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeEndpointError::InternalFailure(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeEndpointError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeEndpointError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEndpointError::Validation(error_message.to_string())
                    }
                    _ => DescribeEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEndpointError {
    fn from(err: serde_json::error::Error) -> DescribeEndpointError {
        DescribeEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEndpointError {
    fn from(err: CredentialsError) -> DescribeEndpointError {
        DescribeEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEndpointError {
    fn from(err: HttpDispatchError) -> DescribeEndpointError {
        DescribeEndpointError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointError {
    fn description(&self) -> &str {
        match *self {
            DescribeEndpointError::InternalFailure(ref cause) => cause,
            DescribeEndpointError::Throttling(ref cause) => cause,
            DescribeEndpointError::Unauthorized(ref cause) => cause,
            DescribeEndpointError::Validation(ref cause) => cause,
            DescribeEndpointError::Credentials(ref err) => err.description(),
            DescribeEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeThing
#[derive(Debug, PartialEq)]
pub enum DescribeThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeThingError {
    pub fn from_body(body: &str) -> DescribeThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeThingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeThingError::Validation(error_message.to_string())
                    }
                    _ => DescribeThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeThingError {
    fn from(err: serde_json::error::Error) -> DescribeThingError {
        DescribeThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeThingError {
    fn from(err: CredentialsError) -> DescribeThingError {
        DescribeThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeThingError {
    fn from(err: HttpDispatchError) -> DescribeThingError {
        DescribeThingError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeThingError {
    fn description(&self) -> &str {
        match *self {
            DescribeThingError::InternalFailure(ref cause) => cause,
            DescribeThingError::InvalidRequest(ref cause) => cause,
            DescribeThingError::ResourceNotFound(ref cause) => cause,
            DescribeThingError::ServiceUnavailable(ref cause) => cause,
            DescribeThingError::Throttling(ref cause) => cause,
            DescribeThingError::Unauthorized(ref cause) => cause,
            DescribeThingError::Validation(ref cause) => cause,
            DescribeThingError::Credentials(ref err) => err.description(),
            DescribeThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeThingType
#[derive(Debug, PartialEq)]
pub enum DescribeThingTypeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeThingTypeError {
    pub fn from_body(body: &str) -> DescribeThingTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeThingTypeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeThingTypeError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeThingTypeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeThingTypeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeThingTypeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeThingTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeThingTypeError::Validation(error_message.to_string())
                    }
                    _ => DescribeThingTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeThingTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeThingTypeError {
    fn from(err: serde_json::error::Error) -> DescribeThingTypeError {
        DescribeThingTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeThingTypeError {
    fn from(err: CredentialsError) -> DescribeThingTypeError {
        DescribeThingTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeThingTypeError {
    fn from(err: HttpDispatchError) -> DescribeThingTypeError {
        DescribeThingTypeError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeThingTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeThingTypeError {
    fn description(&self) -> &str {
        match *self {
            DescribeThingTypeError::InternalFailure(ref cause) => cause,
            DescribeThingTypeError::InvalidRequest(ref cause) => cause,
            DescribeThingTypeError::ResourceNotFound(ref cause) => cause,
            DescribeThingTypeError::ServiceUnavailable(ref cause) => cause,
            DescribeThingTypeError::Throttling(ref cause) => cause,
            DescribeThingTypeError::Unauthorized(ref cause) => cause,
            DescribeThingTypeError::Validation(ref cause) => cause,
            DescribeThingTypeError::Credentials(ref err) => err.description(),
            DescribeThingTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeThingTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachPrincipalPolicy
#[derive(Debug, PartialEq)]
pub enum DetachPrincipalPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetachPrincipalPolicyError {
    pub fn from_body(body: &str) -> DetachPrincipalPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DetachPrincipalPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DetachPrincipalPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DetachPrincipalPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DetachPrincipalPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DetachPrincipalPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DetachPrincipalPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachPrincipalPolicyError::Validation(error_message.to_string())
                    }
                    _ => DetachPrincipalPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachPrincipalPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachPrincipalPolicyError {
    fn from(err: serde_json::error::Error) -> DetachPrincipalPolicyError {
        DetachPrincipalPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachPrincipalPolicyError {
    fn from(err: CredentialsError) -> DetachPrincipalPolicyError {
        DetachPrincipalPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachPrincipalPolicyError {
    fn from(err: HttpDispatchError) -> DetachPrincipalPolicyError {
        DetachPrincipalPolicyError::HttpDispatch(err)
    }
}
impl fmt::Display for DetachPrincipalPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachPrincipalPolicyError {
    fn description(&self) -> &str {
        match *self {
            DetachPrincipalPolicyError::InternalFailure(ref cause) => cause,
            DetachPrincipalPolicyError::InvalidRequest(ref cause) => cause,
            DetachPrincipalPolicyError::ResourceNotFound(ref cause) => cause,
            DetachPrincipalPolicyError::ServiceUnavailable(ref cause) => cause,
            DetachPrincipalPolicyError::Throttling(ref cause) => cause,
            DetachPrincipalPolicyError::Unauthorized(ref cause) => cause,
            DetachPrincipalPolicyError::Validation(ref cause) => cause,
            DetachPrincipalPolicyError::Credentials(ref err) => err.description(),
            DetachPrincipalPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachPrincipalPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachThingPrincipal
#[derive(Debug, PartialEq)]
pub enum DetachThingPrincipalError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DetachThingPrincipalError {
    pub fn from_body(body: &str) -> DetachThingPrincipalError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DetachThingPrincipalError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DetachThingPrincipalError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DetachThingPrincipalError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DetachThingPrincipalError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DetachThingPrincipalError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DetachThingPrincipalError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachThingPrincipalError::Validation(error_message.to_string())
                    }
                    _ => DetachThingPrincipalError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachThingPrincipalError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachThingPrincipalError {
    fn from(err: serde_json::error::Error) -> DetachThingPrincipalError {
        DetachThingPrincipalError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachThingPrincipalError {
    fn from(err: CredentialsError) -> DetachThingPrincipalError {
        DetachThingPrincipalError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachThingPrincipalError {
    fn from(err: HttpDispatchError) -> DetachThingPrincipalError {
        DetachThingPrincipalError::HttpDispatch(err)
    }
}
impl fmt::Display for DetachThingPrincipalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachThingPrincipalError {
    fn description(&self) -> &str {
        match *self {
            DetachThingPrincipalError::InternalFailure(ref cause) => cause,
            DetachThingPrincipalError::InvalidRequest(ref cause) => cause,
            DetachThingPrincipalError::ResourceNotFound(ref cause) => cause,
            DetachThingPrincipalError::ServiceUnavailable(ref cause) => cause,
            DetachThingPrincipalError::Throttling(ref cause) => cause,
            DetachThingPrincipalError::Unauthorized(ref cause) => cause,
            DetachThingPrincipalError::Validation(ref cause) => cause,
            DetachThingPrincipalError::Credentials(ref err) => err.description(),
            DetachThingPrincipalError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachThingPrincipalError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableTopicRule
#[derive(Debug, PartialEq)]
pub enum DisableTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisableTopicRuleError {
    pub fn from_body(body: &str) -> DisableTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        DisableTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DisableTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DisableTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DisableTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => DisableTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableTopicRuleError {
    fn from(err: serde_json::error::Error) -> DisableTopicRuleError {
        DisableTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableTopicRuleError {
    fn from(err: CredentialsError) -> DisableTopicRuleError {
        DisableTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableTopicRuleError {
    fn from(err: HttpDispatchError) -> DisableTopicRuleError {
        DisableTopicRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for DisableTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            DisableTopicRuleError::Internal(ref cause) => cause,
            DisableTopicRuleError::InvalidRequest(ref cause) => cause,
            DisableTopicRuleError::ServiceUnavailable(ref cause) => cause,
            DisableTopicRuleError::Unauthorized(ref cause) => cause,
            DisableTopicRuleError::Validation(ref cause) => cause,
            DisableTopicRuleError::Credentials(ref err) => err.description(),
            DisableTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableTopicRule
#[derive(Debug, PartialEq)]
pub enum EnableTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EnableTopicRuleError {
    pub fn from_body(body: &str) -> EnableTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        EnableTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        EnableTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        EnableTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        EnableTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => EnableTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableTopicRuleError {
    fn from(err: serde_json::error::Error) -> EnableTopicRuleError {
        EnableTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableTopicRuleError {
    fn from(err: CredentialsError) -> EnableTopicRuleError {
        EnableTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableTopicRuleError {
    fn from(err: HttpDispatchError) -> EnableTopicRuleError {
        EnableTopicRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for EnableTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            EnableTopicRuleError::Internal(ref cause) => cause,
            EnableTopicRuleError::InvalidRequest(ref cause) => cause,
            EnableTopicRuleError::ServiceUnavailable(ref cause) => cause,
            EnableTopicRuleError::Unauthorized(ref cause) => cause,
            EnableTopicRuleError::Validation(ref cause) => cause,
            EnableTopicRuleError::Credentials(ref err) => err.description(),
            EnableTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoggingOptions
#[derive(Debug, PartialEq)]
pub enum GetLoggingOptionsError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetLoggingOptionsError {
    pub fn from_body(body: &str) -> GetLoggingOptionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        GetLoggingOptionsError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetLoggingOptionsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetLoggingOptionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLoggingOptionsError::Validation(error_message.to_string())
                    }
                    _ => GetLoggingOptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLoggingOptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLoggingOptionsError {
    fn from(err: serde_json::error::Error) -> GetLoggingOptionsError {
        GetLoggingOptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLoggingOptionsError {
    fn from(err: CredentialsError) -> GetLoggingOptionsError {
        GetLoggingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLoggingOptionsError {
    fn from(err: HttpDispatchError) -> GetLoggingOptionsError {
        GetLoggingOptionsError::HttpDispatch(err)
    }
}
impl fmt::Display for GetLoggingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoggingOptionsError {
    fn description(&self) -> &str {
        match *self {
            GetLoggingOptionsError::Internal(ref cause) => cause,
            GetLoggingOptionsError::InvalidRequest(ref cause) => cause,
            GetLoggingOptionsError::ServiceUnavailable(ref cause) => cause,
            GetLoggingOptionsError::Validation(ref cause) => cause,
            GetLoggingOptionsError::Credentials(ref err) => err.description(),
            GetLoggingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLoggingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPolicy
#[derive(Debug, PartialEq)]
pub enum GetPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetPolicyError {
    pub fn from_body(body: &str) -> GetPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetPolicyError::Validation(error_message.to_string()),
                    _ => GetPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPolicyError {
    fn from(err: serde_json::error::Error) -> GetPolicyError {
        GetPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPolicyError {
    fn from(err: CredentialsError) -> GetPolicyError {
        GetPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPolicyError {
    fn from(err: HttpDispatchError) -> GetPolicyError {
        GetPolicyError::HttpDispatch(err)
    }
}
impl fmt::Display for GetPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetPolicyError::InternalFailure(ref cause) => cause,
            GetPolicyError::InvalidRequest(ref cause) => cause,
            GetPolicyError::ResourceNotFound(ref cause) => cause,
            GetPolicyError::ServiceUnavailable(ref cause) => cause,
            GetPolicyError::Throttling(ref cause) => cause,
            GetPolicyError::Unauthorized(ref cause) => cause,
            GetPolicyError::Validation(ref cause) => cause,
            GetPolicyError::Credentials(ref err) => err.description(),
            GetPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPolicyVersion
#[derive(Debug, PartialEq)]
pub enum GetPolicyVersionError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetPolicyVersionError {
    pub fn from_body(body: &str) -> GetPolicyVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetPolicyVersionError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetPolicyVersionError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetPolicyVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetPolicyVersionError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetPolicyVersionError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetPolicyVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetPolicyVersionError::Validation(error_message.to_string())
                    }
                    _ => GetPolicyVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPolicyVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPolicyVersionError {
    fn from(err: serde_json::error::Error) -> GetPolicyVersionError {
        GetPolicyVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPolicyVersionError {
    fn from(err: CredentialsError) -> GetPolicyVersionError {
        GetPolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPolicyVersionError {
    fn from(err: HttpDispatchError) -> GetPolicyVersionError {
        GetPolicyVersionError::HttpDispatch(err)
    }
}
impl fmt::Display for GetPolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            GetPolicyVersionError::InternalFailure(ref cause) => cause,
            GetPolicyVersionError::InvalidRequest(ref cause) => cause,
            GetPolicyVersionError::ResourceNotFound(ref cause) => cause,
            GetPolicyVersionError::ServiceUnavailable(ref cause) => cause,
            GetPolicyVersionError::Throttling(ref cause) => cause,
            GetPolicyVersionError::Unauthorized(ref cause) => cause,
            GetPolicyVersionError::Validation(ref cause) => cause,
            GetPolicyVersionError::Credentials(ref err) => err.description(),
            GetPolicyVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRegistrationCode
#[derive(Debug, PartialEq)]
pub enum GetRegistrationCodeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetRegistrationCodeError {
    pub fn from_body(body: &str) -> GetRegistrationCodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetRegistrationCodeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetRegistrationCodeError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetRegistrationCodeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetRegistrationCodeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetRegistrationCodeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRegistrationCodeError::Validation(error_message.to_string())
                    }
                    _ => GetRegistrationCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRegistrationCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRegistrationCodeError {
    fn from(err: serde_json::error::Error) -> GetRegistrationCodeError {
        GetRegistrationCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRegistrationCodeError {
    fn from(err: CredentialsError) -> GetRegistrationCodeError {
        GetRegistrationCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRegistrationCodeError {
    fn from(err: HttpDispatchError) -> GetRegistrationCodeError {
        GetRegistrationCodeError::HttpDispatch(err)
    }
}
impl fmt::Display for GetRegistrationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRegistrationCodeError {
    fn description(&self) -> &str {
        match *self {
            GetRegistrationCodeError::InternalFailure(ref cause) => cause,
            GetRegistrationCodeError::InvalidRequest(ref cause) => cause,
            GetRegistrationCodeError::ServiceUnavailable(ref cause) => cause,
            GetRegistrationCodeError::Throttling(ref cause) => cause,
            GetRegistrationCodeError::Unauthorized(ref cause) => cause,
            GetRegistrationCodeError::Validation(ref cause) => cause,
            GetRegistrationCodeError::Credentials(ref err) => err.description(),
            GetRegistrationCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRegistrationCodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTopicRule
#[derive(Debug, PartialEq)]
pub enum GetTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetTopicRuleError {
    pub fn from_body(body: &str) -> GetTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => GetTopicRuleError::Internal(String::from(error_message)),
                    "InvalidRequestException" => {
                        GetTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => GetTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTopicRuleError {
    fn from(err: serde_json::error::Error) -> GetTopicRuleError {
        GetTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTopicRuleError {
    fn from(err: CredentialsError) -> GetTopicRuleError {
        GetTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTopicRuleError {
    fn from(err: HttpDispatchError) -> GetTopicRuleError {
        GetTopicRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for GetTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            GetTopicRuleError::Internal(ref cause) => cause,
            GetTopicRuleError::InvalidRequest(ref cause) => cause,
            GetTopicRuleError::ServiceUnavailable(ref cause) => cause,
            GetTopicRuleError::Unauthorized(ref cause) => cause,
            GetTopicRuleError::Validation(ref cause) => cause,
            GetTopicRuleError::Credentials(ref err) => err.description(),
            GetTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCACertificates
#[derive(Debug, PartialEq)]
pub enum ListCACertificatesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListCACertificatesError {
    pub fn from_body(body: &str) -> ListCACertificatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListCACertificatesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListCACertificatesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListCACertificatesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListCACertificatesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListCACertificatesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCACertificatesError::Validation(error_message.to_string())
                    }
                    _ => ListCACertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCACertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCACertificatesError {
    fn from(err: serde_json::error::Error) -> ListCACertificatesError {
        ListCACertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCACertificatesError {
    fn from(err: CredentialsError) -> ListCACertificatesError {
        ListCACertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCACertificatesError {
    fn from(err: HttpDispatchError) -> ListCACertificatesError {
        ListCACertificatesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListCACertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCACertificatesError {
    fn description(&self) -> &str {
        match *self {
            ListCACertificatesError::InternalFailure(ref cause) => cause,
            ListCACertificatesError::InvalidRequest(ref cause) => cause,
            ListCACertificatesError::ServiceUnavailable(ref cause) => cause,
            ListCACertificatesError::Throttling(ref cause) => cause,
            ListCACertificatesError::Unauthorized(ref cause) => cause,
            ListCACertificatesError::Validation(ref cause) => cause,
            ListCACertificatesError::Credentials(ref err) => err.description(),
            ListCACertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCACertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCertificates
#[derive(Debug, PartialEq)]
pub enum ListCertificatesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListCertificatesError {
    pub fn from_body(body: &str) -> ListCertificatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListCertificatesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListCertificatesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListCertificatesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListCertificatesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListCertificatesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCertificatesError::Validation(error_message.to_string())
                    }
                    _ => ListCertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCertificatesError {
    fn from(err: serde_json::error::Error) -> ListCertificatesError {
        ListCertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCertificatesError {
    fn from(err: CredentialsError) -> ListCertificatesError {
        ListCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCertificatesError {
    fn from(err: HttpDispatchError) -> ListCertificatesError {
        ListCertificatesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCertificatesError {
    fn description(&self) -> &str {
        match *self {
            ListCertificatesError::InternalFailure(ref cause) => cause,
            ListCertificatesError::InvalidRequest(ref cause) => cause,
            ListCertificatesError::ServiceUnavailable(ref cause) => cause,
            ListCertificatesError::Throttling(ref cause) => cause,
            ListCertificatesError::Unauthorized(ref cause) => cause,
            ListCertificatesError::Validation(ref cause) => cause,
            ListCertificatesError::Credentials(ref err) => err.description(),
            ListCertificatesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCertificatesByCA
#[derive(Debug, PartialEq)]
pub enum ListCertificatesByCAError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListCertificatesByCAError {
    pub fn from_body(body: &str) -> ListCertificatesByCAError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListCertificatesByCAError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListCertificatesByCAError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListCertificatesByCAError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListCertificatesByCAError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListCertificatesByCAError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCertificatesByCAError::Validation(error_message.to_string())
                    }
                    _ => ListCertificatesByCAError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCertificatesByCAError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCertificatesByCAError {
    fn from(err: serde_json::error::Error) -> ListCertificatesByCAError {
        ListCertificatesByCAError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCertificatesByCAError {
    fn from(err: CredentialsError) -> ListCertificatesByCAError {
        ListCertificatesByCAError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCertificatesByCAError {
    fn from(err: HttpDispatchError) -> ListCertificatesByCAError {
        ListCertificatesByCAError::HttpDispatch(err)
    }
}
impl fmt::Display for ListCertificatesByCAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCertificatesByCAError {
    fn description(&self) -> &str {
        match *self {
            ListCertificatesByCAError::InternalFailure(ref cause) => cause,
            ListCertificatesByCAError::InvalidRequest(ref cause) => cause,
            ListCertificatesByCAError::ServiceUnavailable(ref cause) => cause,
            ListCertificatesByCAError::Throttling(ref cause) => cause,
            ListCertificatesByCAError::Unauthorized(ref cause) => cause,
            ListCertificatesByCAError::Validation(ref cause) => cause,
            ListCertificatesByCAError::Credentials(ref err) => err.description(),
            ListCertificatesByCAError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCertificatesByCAError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOutgoingCertificates
#[derive(Debug, PartialEq)]
pub enum ListOutgoingCertificatesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListOutgoingCertificatesError {
    pub fn from_body(body: &str) -> ListOutgoingCertificatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListOutgoingCertificatesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListOutgoingCertificatesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => ListOutgoingCertificatesError::ServiceUnavailable(String::from(error_message)),
                    "ThrottlingException" => {
                        ListOutgoingCertificatesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListOutgoingCertificatesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListOutgoingCertificatesError::Validation(error_message.to_string())
                    }
                    _ => ListOutgoingCertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOutgoingCertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOutgoingCertificatesError {
    fn from(err: serde_json::error::Error) -> ListOutgoingCertificatesError {
        ListOutgoingCertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOutgoingCertificatesError {
    fn from(err: CredentialsError) -> ListOutgoingCertificatesError {
        ListOutgoingCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOutgoingCertificatesError {
    fn from(err: HttpDispatchError) -> ListOutgoingCertificatesError {
        ListOutgoingCertificatesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListOutgoingCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOutgoingCertificatesError {
    fn description(&self) -> &str {
        match *self {
            ListOutgoingCertificatesError::InternalFailure(ref cause) => cause,
            ListOutgoingCertificatesError::InvalidRequest(ref cause) => cause,
            ListOutgoingCertificatesError::ServiceUnavailable(ref cause) => cause,
            ListOutgoingCertificatesError::Throttling(ref cause) => cause,
            ListOutgoingCertificatesError::Unauthorized(ref cause) => cause,
            ListOutgoingCertificatesError::Validation(ref cause) => cause,
            ListOutgoingCertificatesError::Credentials(ref err) => err.description(),
            ListOutgoingCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOutgoingCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicies
#[derive(Debug, PartialEq)]
pub enum ListPoliciesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPoliciesError {
    pub fn from_body(body: &str) -> ListPoliciesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPoliciesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPoliciesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPoliciesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPoliciesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPoliciesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPoliciesError::Validation(error_message.to_string())
                    }
                    _ => ListPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPoliciesError {
    fn from(err: serde_json::error::Error) -> ListPoliciesError {
        ListPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPoliciesError {
    fn from(err: CredentialsError) -> ListPoliciesError {
        ListPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPoliciesError {
    fn from(err: HttpDispatchError) -> ListPoliciesError {
        ListPoliciesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListPoliciesError::InternalFailure(ref cause) => cause,
            ListPoliciesError::InvalidRequest(ref cause) => cause,
            ListPoliciesError::ServiceUnavailable(ref cause) => cause,
            ListPoliciesError::Throttling(ref cause) => cause,
            ListPoliciesError::Unauthorized(ref cause) => cause,
            ListPoliciesError::Validation(ref cause) => cause,
            ListPoliciesError::Credentials(ref err) => err.description(),
            ListPoliciesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicyPrincipals
#[derive(Debug, PartialEq)]
pub enum ListPolicyPrincipalsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPolicyPrincipalsError {
    pub fn from_body(body: &str) -> ListPolicyPrincipalsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPolicyPrincipalsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPolicyPrincipalsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPolicyPrincipalsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPolicyPrincipalsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPolicyPrincipalsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPolicyPrincipalsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPolicyPrincipalsError::Validation(error_message.to_string())
                    }
                    _ => ListPolicyPrincipalsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPolicyPrincipalsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPolicyPrincipalsError {
    fn from(err: serde_json::error::Error) -> ListPolicyPrincipalsError {
        ListPolicyPrincipalsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPolicyPrincipalsError {
    fn from(err: CredentialsError) -> ListPolicyPrincipalsError {
        ListPolicyPrincipalsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPolicyPrincipalsError {
    fn from(err: HttpDispatchError) -> ListPolicyPrincipalsError {
        ListPolicyPrincipalsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPolicyPrincipalsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPolicyPrincipalsError {
    fn description(&self) -> &str {
        match *self {
            ListPolicyPrincipalsError::InternalFailure(ref cause) => cause,
            ListPolicyPrincipalsError::InvalidRequest(ref cause) => cause,
            ListPolicyPrincipalsError::ResourceNotFound(ref cause) => cause,
            ListPolicyPrincipalsError::ServiceUnavailable(ref cause) => cause,
            ListPolicyPrincipalsError::Throttling(ref cause) => cause,
            ListPolicyPrincipalsError::Unauthorized(ref cause) => cause,
            ListPolicyPrincipalsError::Validation(ref cause) => cause,
            ListPolicyPrincipalsError::Credentials(ref err) => err.description(),
            ListPolicyPrincipalsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPolicyPrincipalsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicyVersions
#[derive(Debug, PartialEq)]
pub enum ListPolicyVersionsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPolicyVersionsError {
    pub fn from_body(body: &str) -> ListPolicyVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPolicyVersionsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPolicyVersionsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPolicyVersionsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPolicyVersionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPolicyVersionsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPolicyVersionsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPolicyVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListPolicyVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPolicyVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPolicyVersionsError {
    fn from(err: serde_json::error::Error) -> ListPolicyVersionsError {
        ListPolicyVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPolicyVersionsError {
    fn from(err: CredentialsError) -> ListPolicyVersionsError {
        ListPolicyVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPolicyVersionsError {
    fn from(err: HttpDispatchError) -> ListPolicyVersionsError {
        ListPolicyVersionsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPolicyVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPolicyVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListPolicyVersionsError::InternalFailure(ref cause) => cause,
            ListPolicyVersionsError::InvalidRequest(ref cause) => cause,
            ListPolicyVersionsError::ResourceNotFound(ref cause) => cause,
            ListPolicyVersionsError::ServiceUnavailable(ref cause) => cause,
            ListPolicyVersionsError::Throttling(ref cause) => cause,
            ListPolicyVersionsError::Unauthorized(ref cause) => cause,
            ListPolicyVersionsError::Validation(ref cause) => cause,
            ListPolicyVersionsError::Credentials(ref err) => err.description(),
            ListPolicyVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPolicyVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPrincipalPolicies
#[derive(Debug, PartialEq)]
pub enum ListPrincipalPoliciesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPrincipalPoliciesError {
    pub fn from_body(body: &str) -> ListPrincipalPoliciesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPrincipalPoliciesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPrincipalPoliciesError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPrincipalPoliciesError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPrincipalPoliciesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPrincipalPoliciesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPrincipalPoliciesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPrincipalPoliciesError::Validation(error_message.to_string())
                    }
                    _ => ListPrincipalPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPrincipalPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPrincipalPoliciesError {
    fn from(err: serde_json::error::Error) -> ListPrincipalPoliciesError {
        ListPrincipalPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPrincipalPoliciesError {
    fn from(err: CredentialsError) -> ListPrincipalPoliciesError {
        ListPrincipalPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPrincipalPoliciesError {
    fn from(err: HttpDispatchError) -> ListPrincipalPoliciesError {
        ListPrincipalPoliciesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPrincipalPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPrincipalPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListPrincipalPoliciesError::InternalFailure(ref cause) => cause,
            ListPrincipalPoliciesError::InvalidRequest(ref cause) => cause,
            ListPrincipalPoliciesError::ResourceNotFound(ref cause) => cause,
            ListPrincipalPoliciesError::ServiceUnavailable(ref cause) => cause,
            ListPrincipalPoliciesError::Throttling(ref cause) => cause,
            ListPrincipalPoliciesError::Unauthorized(ref cause) => cause,
            ListPrincipalPoliciesError::Validation(ref cause) => cause,
            ListPrincipalPoliciesError::Credentials(ref err) => err.description(),
            ListPrincipalPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPrincipalPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPrincipalThings
#[derive(Debug, PartialEq)]
pub enum ListPrincipalThingsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListPrincipalThingsError {
    pub fn from_body(body: &str) -> ListPrincipalThingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPrincipalThingsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPrincipalThingsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPrincipalThingsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPrincipalThingsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPrincipalThingsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPrincipalThingsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPrincipalThingsError::Validation(error_message.to_string())
                    }
                    _ => ListPrincipalThingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPrincipalThingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPrincipalThingsError {
    fn from(err: serde_json::error::Error) -> ListPrincipalThingsError {
        ListPrincipalThingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPrincipalThingsError {
    fn from(err: CredentialsError) -> ListPrincipalThingsError {
        ListPrincipalThingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPrincipalThingsError {
    fn from(err: HttpDispatchError) -> ListPrincipalThingsError {
        ListPrincipalThingsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListPrincipalThingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPrincipalThingsError {
    fn description(&self) -> &str {
        match *self {
            ListPrincipalThingsError::InternalFailure(ref cause) => cause,
            ListPrincipalThingsError::InvalidRequest(ref cause) => cause,
            ListPrincipalThingsError::ResourceNotFound(ref cause) => cause,
            ListPrincipalThingsError::ServiceUnavailable(ref cause) => cause,
            ListPrincipalThingsError::Throttling(ref cause) => cause,
            ListPrincipalThingsError::Unauthorized(ref cause) => cause,
            ListPrincipalThingsError::Validation(ref cause) => cause,
            ListPrincipalThingsError::Credentials(ref err) => err.description(),
            ListPrincipalThingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPrincipalThingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingPrincipals
#[derive(Debug, PartialEq)]
pub enum ListThingPrincipalsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListThingPrincipalsError {
    pub fn from_body(body: &str) -> ListThingPrincipalsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingPrincipalsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingPrincipalsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListThingPrincipalsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListThingPrincipalsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListThingPrincipalsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListThingPrincipalsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListThingPrincipalsError::Validation(error_message.to_string())
                    }
                    _ => ListThingPrincipalsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingPrincipalsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingPrincipalsError {
    fn from(err: serde_json::error::Error) -> ListThingPrincipalsError {
        ListThingPrincipalsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingPrincipalsError {
    fn from(err: CredentialsError) -> ListThingPrincipalsError {
        ListThingPrincipalsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingPrincipalsError {
    fn from(err: HttpDispatchError) -> ListThingPrincipalsError {
        ListThingPrincipalsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListThingPrincipalsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingPrincipalsError {
    fn description(&self) -> &str {
        match *self {
            ListThingPrincipalsError::InternalFailure(ref cause) => cause,
            ListThingPrincipalsError::InvalidRequest(ref cause) => cause,
            ListThingPrincipalsError::ResourceNotFound(ref cause) => cause,
            ListThingPrincipalsError::ServiceUnavailable(ref cause) => cause,
            ListThingPrincipalsError::Throttling(ref cause) => cause,
            ListThingPrincipalsError::Unauthorized(ref cause) => cause,
            ListThingPrincipalsError::Validation(ref cause) => cause,
            ListThingPrincipalsError::Credentials(ref err) => err.description(),
            ListThingPrincipalsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListThingPrincipalsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingTypes
#[derive(Debug, PartialEq)]
pub enum ListThingTypesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListThingTypesError {
    pub fn from_body(body: &str) -> ListThingTypesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingTypesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingTypesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListThingTypesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListThingTypesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListThingTypesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListThingTypesError::Validation(error_message.to_string())
                    }
                    _ => ListThingTypesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingTypesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingTypesError {
    fn from(err: serde_json::error::Error) -> ListThingTypesError {
        ListThingTypesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingTypesError {
    fn from(err: CredentialsError) -> ListThingTypesError {
        ListThingTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingTypesError {
    fn from(err: HttpDispatchError) -> ListThingTypesError {
        ListThingTypesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListThingTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingTypesError {
    fn description(&self) -> &str {
        match *self {
            ListThingTypesError::InternalFailure(ref cause) => cause,
            ListThingTypesError::InvalidRequest(ref cause) => cause,
            ListThingTypesError::ServiceUnavailable(ref cause) => cause,
            ListThingTypesError::Throttling(ref cause) => cause,
            ListThingTypesError::Unauthorized(ref cause) => cause,
            ListThingTypesError::Validation(ref cause) => cause,
            ListThingTypesError::Credentials(ref err) => err.description(),
            ListThingTypesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListThingTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThings
#[derive(Debug, PartialEq)]
pub enum ListThingsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListThingsError {
    pub fn from_body(body: &str) -> ListThingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListThingsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListThingsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListThingsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => ListThingsError::Validation(error_message.to_string()),
                    _ => ListThingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingsError {
    fn from(err: serde_json::error::Error) -> ListThingsError {
        ListThingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingsError {
    fn from(err: CredentialsError) -> ListThingsError {
        ListThingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingsError {
    fn from(err: HttpDispatchError) -> ListThingsError {
        ListThingsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListThingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingsError {
    fn description(&self) -> &str {
        match *self {
            ListThingsError::InternalFailure(ref cause) => cause,
            ListThingsError::InvalidRequest(ref cause) => cause,
            ListThingsError::ServiceUnavailable(ref cause) => cause,
            ListThingsError::Throttling(ref cause) => cause,
            ListThingsError::Unauthorized(ref cause) => cause,
            ListThingsError::Validation(ref cause) => cause,
            ListThingsError::Credentials(ref err) => err.description(),
            ListThingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListThingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTopicRules
#[derive(Debug, PartialEq)]
pub enum ListTopicRulesError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTopicRulesError {
    pub fn from_body(body: &str) -> ListTopicRulesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        ListTopicRulesError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListTopicRulesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListTopicRulesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTopicRulesError::Validation(error_message.to_string())
                    }
                    _ => ListTopicRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTopicRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTopicRulesError {
    fn from(err: serde_json::error::Error) -> ListTopicRulesError {
        ListTopicRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTopicRulesError {
    fn from(err: CredentialsError) -> ListTopicRulesError {
        ListTopicRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTopicRulesError {
    fn from(err: HttpDispatchError) -> ListTopicRulesError {
        ListTopicRulesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListTopicRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTopicRulesError {
    fn description(&self) -> &str {
        match *self {
            ListTopicRulesError::Internal(ref cause) => cause,
            ListTopicRulesError::InvalidRequest(ref cause) => cause,
            ListTopicRulesError::ServiceUnavailable(ref cause) => cause,
            ListTopicRulesError::Validation(ref cause) => cause,
            ListTopicRulesError::Credentials(ref err) => err.description(),
            ListTopicRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTopicRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterCACertificate
#[derive(Debug, PartialEq)]
pub enum RegisterCACertificateError {
    ///<p>The certificate is invalid.</p>
    CertificateValidation(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The registration code is invalid.</p>
    RegistrationCodeValidation(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterCACertificateError {
    pub fn from_body(body: &str) -> RegisterCACertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateValidationException" => RegisterCACertificateError::CertificateValidation(String::from(error_message)),
                    "InternalFailureException" => {
                        RegisterCACertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RegisterCACertificateError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        RegisterCACertificateError::LimitExceeded(String::from(error_message))
                    }
                    "RegistrationCodeValidationException" => RegisterCACertificateError::RegistrationCodeValidation(String::from(error_message)),
                    "ResourceAlreadyExistsException" => RegisterCACertificateError::ResourceAlreadyExists(String::from(error_message)),
                    "ServiceUnavailableException" => {
                        RegisterCACertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        RegisterCACertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        RegisterCACertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterCACertificateError::Validation(error_message.to_string())
                    }
                    _ => RegisterCACertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterCACertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterCACertificateError {
    fn from(err: serde_json::error::Error) -> RegisterCACertificateError {
        RegisterCACertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterCACertificateError {
    fn from(err: CredentialsError) -> RegisterCACertificateError {
        RegisterCACertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterCACertificateError {
    fn from(err: HttpDispatchError) -> RegisterCACertificateError {
        RegisterCACertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for RegisterCACertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterCACertificateError {
    fn description(&self) -> &str {
        match *self {
            RegisterCACertificateError::CertificateValidation(ref cause) => cause,
            RegisterCACertificateError::InternalFailure(ref cause) => cause,
            RegisterCACertificateError::InvalidRequest(ref cause) => cause,
            RegisterCACertificateError::LimitExceeded(ref cause) => cause,
            RegisterCACertificateError::RegistrationCodeValidation(ref cause) => cause,
            RegisterCACertificateError::ResourceAlreadyExists(ref cause) => cause,
            RegisterCACertificateError::ServiceUnavailable(ref cause) => cause,
            RegisterCACertificateError::Throttling(ref cause) => cause,
            RegisterCACertificateError::Unauthorized(ref cause) => cause,
            RegisterCACertificateError::Validation(ref cause) => cause,
            RegisterCACertificateError::Credentials(ref err) => err.description(),
            RegisterCACertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterCACertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterCertificate
#[derive(Debug, PartialEq)]
pub enum RegisterCertificateError {
    ///<p>Unable to verify the CA certificate used to sign the device certificate you are attempting to register. This is happens when you have registered more than one CA certificate that has the same subject field and public key.</p>
    CertificateConflict(String),
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>The certificate is invalid.</p>
    CertificateValidation(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterCertificateError {
    pub fn from_body(body: &str) -> RegisterCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateConflictException" => {
                        RegisterCertificateError::CertificateConflict(String::from(error_message))
                    }
                    "CertificateStateException" => {
                        RegisterCertificateError::CertificateState(String::from(error_message))
                    }
                    "CertificateValidationException" => {
                        RegisterCertificateError::CertificateValidation(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        RegisterCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RegisterCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        RegisterCertificateError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        RegisterCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        RegisterCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        RegisterCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterCertificateError::Validation(error_message.to_string())
                    }
                    _ => RegisterCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterCertificateError {
    fn from(err: serde_json::error::Error) -> RegisterCertificateError {
        RegisterCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterCertificateError {
    fn from(err: CredentialsError) -> RegisterCertificateError {
        RegisterCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterCertificateError {
    fn from(err: HttpDispatchError) -> RegisterCertificateError {
        RegisterCertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for RegisterCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterCertificateError {
    fn description(&self) -> &str {
        match *self {
            RegisterCertificateError::CertificateConflict(ref cause) => cause,
            RegisterCertificateError::CertificateState(ref cause) => cause,
            RegisterCertificateError::CertificateValidation(ref cause) => cause,
            RegisterCertificateError::InternalFailure(ref cause) => cause,
            RegisterCertificateError::InvalidRequest(ref cause) => cause,
            RegisterCertificateError::ResourceAlreadyExists(ref cause) => cause,
            RegisterCertificateError::ServiceUnavailable(ref cause) => cause,
            RegisterCertificateError::Throttling(ref cause) => cause,
            RegisterCertificateError::Unauthorized(ref cause) => cause,
            RegisterCertificateError::Validation(ref cause) => cause,
            RegisterCertificateError::Credentials(ref err) => err.description(),
            RegisterCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RejectCertificateTransfer
#[derive(Debug, PartialEq)]
pub enum RejectCertificateTransferError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You can't revert the certificate transfer because the transfer is already complete.</p>
    TransferAlreadyCompleted(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RejectCertificateTransferError {
    pub fn from_body(body: &str) -> RejectCertificateTransferError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        RejectCertificateTransferError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RejectCertificateTransferError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => RejectCertificateTransferError::ResourceNotFound(String::from(error_message)),
                    "ServiceUnavailableException" => RejectCertificateTransferError::ServiceUnavailable(String::from(error_message)),
                    "ThrottlingException" => {
                        RejectCertificateTransferError::Throttling(String::from(error_message))
                    }
                    "TransferAlreadyCompletedException" => RejectCertificateTransferError::TransferAlreadyCompleted(String::from(error_message)),
                    "UnauthorizedException" => {
                        RejectCertificateTransferError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RejectCertificateTransferError::Validation(error_message.to_string())
                    }
                    _ => RejectCertificateTransferError::Unknown(String::from(body)),
                }
            }
            Err(_) => RejectCertificateTransferError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RejectCertificateTransferError {
    fn from(err: serde_json::error::Error) -> RejectCertificateTransferError {
        RejectCertificateTransferError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RejectCertificateTransferError {
    fn from(err: CredentialsError) -> RejectCertificateTransferError {
        RejectCertificateTransferError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RejectCertificateTransferError {
    fn from(err: HttpDispatchError) -> RejectCertificateTransferError {
        RejectCertificateTransferError::HttpDispatch(err)
    }
}
impl fmt::Display for RejectCertificateTransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectCertificateTransferError {
    fn description(&self) -> &str {
        match *self {
            RejectCertificateTransferError::InternalFailure(ref cause) => cause,
            RejectCertificateTransferError::InvalidRequest(ref cause) => cause,
            RejectCertificateTransferError::ResourceNotFound(ref cause) => cause,
            RejectCertificateTransferError::ServiceUnavailable(ref cause) => cause,
            RejectCertificateTransferError::Throttling(ref cause) => cause,
            RejectCertificateTransferError::TransferAlreadyCompleted(ref cause) => cause,
            RejectCertificateTransferError::Unauthorized(ref cause) => cause,
            RejectCertificateTransferError::Validation(ref cause) => cause,
            RejectCertificateTransferError::Credentials(ref err) => err.description(),
            RejectCertificateTransferError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RejectCertificateTransferError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReplaceTopicRule
#[derive(Debug, PartialEq)]
pub enum ReplaceTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The Rule-SQL expression can't be parsed correctly.</p>
    SqlParse(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ReplaceTopicRuleError {
    pub fn from_body(body: &str) -> ReplaceTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        ReplaceTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ReplaceTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ReplaceTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "SqlParseException" => {
                        ReplaceTopicRuleError::SqlParse(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ReplaceTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ReplaceTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => ReplaceTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => ReplaceTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ReplaceTopicRuleError {
    fn from(err: serde_json::error::Error) -> ReplaceTopicRuleError {
        ReplaceTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ReplaceTopicRuleError {
    fn from(err: CredentialsError) -> ReplaceTopicRuleError {
        ReplaceTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReplaceTopicRuleError {
    fn from(err: HttpDispatchError) -> ReplaceTopicRuleError {
        ReplaceTopicRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for ReplaceTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReplaceTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            ReplaceTopicRuleError::Internal(ref cause) => cause,
            ReplaceTopicRuleError::InvalidRequest(ref cause) => cause,
            ReplaceTopicRuleError::ServiceUnavailable(ref cause) => cause,
            ReplaceTopicRuleError::SqlParse(ref cause) => cause,
            ReplaceTopicRuleError::Unauthorized(ref cause) => cause,
            ReplaceTopicRuleError::Validation(ref cause) => cause,
            ReplaceTopicRuleError::Credentials(ref err) => err.description(),
            ReplaceTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ReplaceTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetDefaultPolicyVersion
#[derive(Debug, PartialEq)]
pub enum SetDefaultPolicyVersionError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetDefaultPolicyVersionError {
    pub fn from_body(body: &str) -> SetDefaultPolicyVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        SetDefaultPolicyVersionError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetDefaultPolicyVersionError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SetDefaultPolicyVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => SetDefaultPolicyVersionError::ServiceUnavailable(String::from(error_message)),
                    "ThrottlingException" => {
                        SetDefaultPolicyVersionError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        SetDefaultPolicyVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetDefaultPolicyVersionError::Validation(error_message.to_string())
                    }
                    _ => SetDefaultPolicyVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetDefaultPolicyVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetDefaultPolicyVersionError {
    fn from(err: serde_json::error::Error) -> SetDefaultPolicyVersionError {
        SetDefaultPolicyVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetDefaultPolicyVersionError {
    fn from(err: CredentialsError) -> SetDefaultPolicyVersionError {
        SetDefaultPolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetDefaultPolicyVersionError {
    fn from(err: HttpDispatchError) -> SetDefaultPolicyVersionError {
        SetDefaultPolicyVersionError::HttpDispatch(err)
    }
}
impl fmt::Display for SetDefaultPolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetDefaultPolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            SetDefaultPolicyVersionError::InternalFailure(ref cause) => cause,
            SetDefaultPolicyVersionError::InvalidRequest(ref cause) => cause,
            SetDefaultPolicyVersionError::ResourceNotFound(ref cause) => cause,
            SetDefaultPolicyVersionError::ServiceUnavailable(ref cause) => cause,
            SetDefaultPolicyVersionError::Throttling(ref cause) => cause,
            SetDefaultPolicyVersionError::Unauthorized(ref cause) => cause,
            SetDefaultPolicyVersionError::Validation(ref cause) => cause,
            SetDefaultPolicyVersionError::Credentials(ref err) => err.description(),
            SetDefaultPolicyVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetDefaultPolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetLoggingOptions
#[derive(Debug, PartialEq)]
pub enum SetLoggingOptionsError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetLoggingOptionsError {
    pub fn from_body(body: &str) -> SetLoggingOptionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        SetLoggingOptionsError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetLoggingOptionsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SetLoggingOptionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetLoggingOptionsError::Validation(error_message.to_string())
                    }
                    _ => SetLoggingOptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetLoggingOptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetLoggingOptionsError {
    fn from(err: serde_json::error::Error) -> SetLoggingOptionsError {
        SetLoggingOptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetLoggingOptionsError {
    fn from(err: CredentialsError) -> SetLoggingOptionsError {
        SetLoggingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetLoggingOptionsError {
    fn from(err: HttpDispatchError) -> SetLoggingOptionsError {
        SetLoggingOptionsError::HttpDispatch(err)
    }
}
impl fmt::Display for SetLoggingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetLoggingOptionsError {
    fn description(&self) -> &str {
        match *self {
            SetLoggingOptionsError::Internal(ref cause) => cause,
            SetLoggingOptionsError::InvalidRequest(ref cause) => cause,
            SetLoggingOptionsError::ServiceUnavailable(ref cause) => cause,
            SetLoggingOptionsError::Validation(ref cause) => cause,
            SetLoggingOptionsError::Credentials(ref err) => err.description(),
            SetLoggingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetLoggingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TransferCertificate
#[derive(Debug, PartialEq)]
pub enum TransferCertificateError {
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You can't transfer the certificate because authorization policies are still attached.</p>
    TransferConflict(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TransferCertificateError {
    pub fn from_body(body: &str) -> TransferCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateStateException" => {
                        TransferCertificateError::CertificateState(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        TransferCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        TransferCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TransferCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        TransferCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        TransferCertificateError::Throttling(String::from(error_message))
                    }
                    "TransferConflictException" => {
                        TransferCertificateError::TransferConflict(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        TransferCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        TransferCertificateError::Validation(error_message.to_string())
                    }
                    _ => TransferCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => TransferCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TransferCertificateError {
    fn from(err: serde_json::error::Error) -> TransferCertificateError {
        TransferCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TransferCertificateError {
    fn from(err: CredentialsError) -> TransferCertificateError {
        TransferCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TransferCertificateError {
    fn from(err: HttpDispatchError) -> TransferCertificateError {
        TransferCertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for TransferCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TransferCertificateError {
    fn description(&self) -> &str {
        match *self {
            TransferCertificateError::CertificateState(ref cause) => cause,
            TransferCertificateError::InternalFailure(ref cause) => cause,
            TransferCertificateError::InvalidRequest(ref cause) => cause,
            TransferCertificateError::ResourceNotFound(ref cause) => cause,
            TransferCertificateError::ServiceUnavailable(ref cause) => cause,
            TransferCertificateError::Throttling(ref cause) => cause,
            TransferCertificateError::TransferConflict(ref cause) => cause,
            TransferCertificateError::Unauthorized(ref cause) => cause,
            TransferCertificateError::Validation(ref cause) => cause,
            TransferCertificateError::Credentials(ref err) => err.description(),
            TransferCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TransferCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCACertificate
#[derive(Debug, PartialEq)]
pub enum UpdateCACertificateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateCACertificateError {
    pub fn from_body(body: &str) -> UpdateCACertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateCACertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateCACertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateCACertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateCACertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateCACertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateCACertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCACertificateError::Validation(error_message.to_string())
                    }
                    _ => UpdateCACertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCACertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCACertificateError {
    fn from(err: serde_json::error::Error) -> UpdateCACertificateError {
        UpdateCACertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCACertificateError {
    fn from(err: CredentialsError) -> UpdateCACertificateError {
        UpdateCACertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCACertificateError {
    fn from(err: HttpDispatchError) -> UpdateCACertificateError {
        UpdateCACertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateCACertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCACertificateError {
    fn description(&self) -> &str {
        match *self {
            UpdateCACertificateError::InternalFailure(ref cause) => cause,
            UpdateCACertificateError::InvalidRequest(ref cause) => cause,
            UpdateCACertificateError::ResourceNotFound(ref cause) => cause,
            UpdateCACertificateError::ServiceUnavailable(ref cause) => cause,
            UpdateCACertificateError::Throttling(ref cause) => cause,
            UpdateCACertificateError::Unauthorized(ref cause) => cause,
            UpdateCACertificateError::Validation(ref cause) => cause,
            UpdateCACertificateError::Credentials(ref err) => err.description(),
            UpdateCACertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCACertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCertificate
#[derive(Debug, PartialEq)]
pub enum UpdateCertificateError {
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateCertificateError {
    pub fn from_body(body: &str) -> UpdateCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateStateException" => {
                        UpdateCertificateError::CertificateState(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        UpdateCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCertificateError::Validation(error_message.to_string())
                    }
                    _ => UpdateCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCertificateError {
    fn from(err: serde_json::error::Error) -> UpdateCertificateError {
        UpdateCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCertificateError {
    fn from(err: CredentialsError) -> UpdateCertificateError {
        UpdateCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCertificateError {
    fn from(err: HttpDispatchError) -> UpdateCertificateError {
        UpdateCertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCertificateError {
    fn description(&self) -> &str {
        match *self {
            UpdateCertificateError::CertificateState(ref cause) => cause,
            UpdateCertificateError::InternalFailure(ref cause) => cause,
            UpdateCertificateError::InvalidRequest(ref cause) => cause,
            UpdateCertificateError::ResourceNotFound(ref cause) => cause,
            UpdateCertificateError::ServiceUnavailable(ref cause) => cause,
            UpdateCertificateError::Throttling(ref cause) => cause,
            UpdateCertificateError::Unauthorized(ref cause) => cause,
            UpdateCertificateError::Validation(ref cause) => cause,
            UpdateCertificateError::Credentials(ref err) => err.description(),
            UpdateCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateThing
#[derive(Debug, PartialEq)]
pub enum UpdateThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>An exception thrown when the version of a thing passed to a command is different than the version specified with the --version parameter.</p>
    VersionConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateThingError {
    pub fn from_body(body: &str) -> UpdateThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateThingError::Unauthorized(String::from(error_message))
                    }
                    "VersionConflictException" => {
                        UpdateThingError::VersionConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateThingError::Validation(error_message.to_string())
                    }
                    _ => UpdateThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateThingError {
    fn from(err: serde_json::error::Error) -> UpdateThingError {
        UpdateThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateThingError {
    fn from(err: CredentialsError) -> UpdateThingError {
        UpdateThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateThingError {
    fn from(err: HttpDispatchError) -> UpdateThingError {
        UpdateThingError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateThingError {
    fn description(&self) -> &str {
        match *self {
            UpdateThingError::InternalFailure(ref cause) => cause,
            UpdateThingError::InvalidRequest(ref cause) => cause,
            UpdateThingError::ResourceNotFound(ref cause) => cause,
            UpdateThingError::ServiceUnavailable(ref cause) => cause,
            UpdateThingError::Throttling(ref cause) => cause,
            UpdateThingError::Unauthorized(ref cause) => cause,
            UpdateThingError::VersionConflict(ref cause) => cause,
            UpdateThingError::Validation(ref cause) => cause,
            UpdateThingError::Credentials(ref err) => err.description(),
            UpdateThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS IoT API. AWS IoT clients implement this trait.
pub trait Iot {
    #[doc="<p>Accepts a pending certificate transfer. The default state of the certificate is INACTIVE.</p> <p>To check for pending certificate transfers, call <a>ListCertificates</a> to enumerate your certificates.</p>"]
    fn accept_certificate_transfer(&self,
                                   input: &AcceptCertificateTransferRequest)
                                   -> Result<(), AcceptCertificateTransferError>;


    #[doc="<p>Attaches the specified policy to the specified principal (certificate or other credential).</p>"]
    fn attach_principal_policy(&self,
                               input: &AttachPrincipalPolicyRequest)
                               -> Result<(), AttachPrincipalPolicyError>;


    #[doc="<p>Attaches the specified principal to the specified thing.</p>"]
    fn attach_thing_principal
        (&self,
         input: &AttachThingPrincipalRequest)
         -> Result<AttachThingPrincipalResponse, AttachThingPrincipalError>;


    #[doc="<p>Cancels a pending transfer for the specified certificate.</p> <p> <b>Note</b> Only the transfer source account can use this operation to cancel a transfer. (Transfer destinations can use <a>RejectCertificateTransfer</a> instead.) After transfer, AWS IoT returns the certificate to the source account in the INACTIVE state. After the destination account has accepted the transfer, the transfer cannot be cancelled.</p> <p>After a certificate transfer is cancelled, the status of the certificate changes from PENDING_TRANSFER to INACTIVE.</p>"]
    fn cancel_certificate_transfer(&self,
                                   input: &CancelCertificateTransferRequest)
                                   -> Result<(), CancelCertificateTransferError>;


    #[doc="<p>Creates an X.509 certificate using the specified certificate signing request.</p> <p> <b>Note:</b> The CSR must include a public key that is either an RSA key with a length of at least 2048 bits or an ECC key from NIST P-256 or NIST P-384 curves. </p> <p> <b>Note:</b> Reusing the same certificate signing request (CSR) results in a distinct certificate.</p> <p>You can create multiple certificates in a batch by creating a directory, copying multiple .csr files into that directory, and then specifying that directory on the command line. The following commands show how to create a batch of certificates given a batch of CSRs.</p> <p>Assuming a set of CSRs are located inside of the directory my-csr-directory:</p> <p>On Linux and OS X, the command is:</p> <p>$ ls my-csr-directory/ | xargs -I {} aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/{}</p> <p>This command lists all of the CSRs in my-csr-directory and pipes each CSR file name to the aws iot create-certificate-from-csr AWS CLI command to create a certificate for the corresponding CSR.</p> <p>The aws iot create-certificate-from-csr part of the command can also be run in parallel to speed up the certificate creation process:</p> <p>$ ls my-csr-directory/ | xargs -P 10 -I {} aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/{}</p> <p>On Windows PowerShell, the command to create certificates for all CSRs in my-csr-directory is:</p> <p>&gt; ls -Name my-csr-directory | %{aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/$_}</p> <p>On a Windows command prompt, the command to create certificates for all CSRs in my-csr-directory is:</p> <p>&gt; forfiles /p my-csr-directory /c \"cmd /c aws iot create-certificate-from-csr --certificate-signing-request file://@path\"</p>"]
    fn create_certificate_from_csr
        (&self,
         input: &CreateCertificateFromCsrRequest)
         -> Result<CreateCertificateFromCsrResponse, CreateCertificateFromCsrError>;


    #[doc="<p>Creates a 2048-bit RSA key pair and issues an X.509 certificate using the issued public key.</p> <p> <b>Note</b> This is the only time AWS IoT issues the private key for this certificate, so it is important to keep it in a secure location.</p>"]
    fn create_keys_and_certificate
        (&self,
         input: &CreateKeysAndCertificateRequest)
         -> Result<CreateKeysAndCertificateResponse, CreateKeysAndCertificateError>;


    #[doc="<p>Creates an AWS IoT policy.</p> <p>The created policy is the default version for the policy. This operation creates a policy version with a version identifier of <b>1</b> and sets <b>1</b> as the policy's default version.</p>"]
    fn create_policy(&self,
                     input: &CreatePolicyRequest)
                     -> Result<CreatePolicyResponse, CreatePolicyError>;


    #[doc="<p>Creates a new version of the specified AWS IoT policy. To update a policy, create a new policy version. A managed policy can have up to five versions. If the policy has five versions, you must use <a>DeletePolicyVersion</a> to delete an existing version before you create a new one.</p> <p>Optionally, you can set the new version as the policy's default version. The default version is the operative version (that is, the version that is in effect for the certificates to which the policy is attached).</p>"]
    fn create_policy_version(&self,
                             input: &CreatePolicyVersionRequest)
                             -> Result<CreatePolicyVersionResponse, CreatePolicyVersionError>;


    #[doc="<p>Creates a thing record in the thing registry.</p>"]
    fn create_thing(&self,
                    input: &CreateThingRequest)
                    -> Result<CreateThingResponse, CreateThingError>;


    #[doc="<p>Creates a new thing type.</p>"]
    fn create_thing_type(&self,
                         input: &CreateThingTypeRequest)
                         -> Result<CreateThingTypeResponse, CreateThingTypeError>;


    #[doc="<p>Creates a rule. Creating rules is an administrator-level action. Any user who has permission to create rules will be able to access data processed by the rule.</p>"]
    fn create_topic_rule(&self,
                         input: &CreateTopicRuleRequest)
                         -> Result<(), CreateTopicRuleError>;


    #[doc="<p>Deletes a registered CA certificate.</p>"]
    fn delete_ca_certificate(&self,
                             input: &DeleteCACertificateRequest)
                             -> Result<DeleteCACertificateResponse, DeleteCACertificateError>;


    #[doc="<p>Deletes the specified certificate.</p> <p>A certificate cannot be deleted if it has a policy attached to it or if its status is set to ACTIVE. To delete a certificate, first use the <a>DetachPrincipalPolicy</a> API to detach all policies. Next, use the <a>UpdateCertificate</a> API to set the certificate to the INACTIVE status.</p>"]
    fn delete_certificate(&self,
                          input: &DeleteCertificateRequest)
                          -> Result<(), DeleteCertificateError>;


    #[doc="<p>Deletes the specified policy.</p> <p>A policy cannot be deleted if it has non-default versions or it is attached to any certificate.</p> <p>To delete a policy, use the DeletePolicyVersion API to delete all non-default versions of the policy; use the DetachPrincipalPolicy API to detach the policy from any certificate; and then use the DeletePolicy API to delete the policy.</p> <p>When a policy is deleted using DeletePolicy, its default version is deleted with it.</p>"]
    fn delete_policy(&self, input: &DeletePolicyRequest) -> Result<(), DeletePolicyError>;


    #[doc="<p>Deletes the specified version of the specified policy. You cannot delete the default version of a policy using this API. To delete the default version of a policy, use <a>DeletePolicy</a>. To find out which version of a policy is marked as the default version, use ListPolicyVersions.</p>"]
    fn delete_policy_version(&self,
                             input: &DeletePolicyVersionRequest)
                             -> Result<(), DeletePolicyVersionError>;


    #[doc="<p>Deletes a CA certificate registration code.</p>"]
    fn delete_registration_code
        (&self)
         -> Result<DeleteRegistrationCodeResponse, DeleteRegistrationCodeError>;


    #[doc="<p>Deletes the specified thing.</p>"]
    fn delete_thing(&self,
                    input: &DeleteThingRequest)
                    -> Result<DeleteThingResponse, DeleteThingError>;


    #[doc="<p>Deletes the specified thing type . You cannot delete a thing type if it has things associated with it. To delete a thing type, first mark it as deprecated by calling <a>DeprecateThingType</a>, then remove any associated things by calling <a>UpdateThing</a> to change the thing type on any associated thing, and finally use <a>DeleteThingType</a> to delete the thing type.</p>"]
    fn delete_thing_type(&self,
                         input: &DeleteThingTypeRequest)
                         -> Result<DeleteThingTypeResponse, DeleteThingTypeError>;


    #[doc="<p>Deletes the specified rule.</p>"]
    fn delete_topic_rule(&self,
                         input: &DeleteTopicRuleRequest)
                         -> Result<(), DeleteTopicRuleError>;


    #[doc="<p>Deprecates a thing type. You can not associate new things with deprecated thing type.</p>"]
    fn deprecate_thing_type(&self,
                            input: &DeprecateThingTypeRequest)
                            -> Result<DeprecateThingTypeResponse, DeprecateThingTypeError>;


    #[doc="<p>Describes a registered CA certificate.</p>"]
    fn describe_ca_certificate
        (&self,
         input: &DescribeCACertificateRequest)
         -> Result<DescribeCACertificateResponse, DescribeCACertificateError>;


    #[doc="<p>Gets information about the specified certificate.</p>"]
    fn describe_certificate(&self,
                            input: &DescribeCertificateRequest)
                            -> Result<DescribeCertificateResponse, DescribeCertificateError>;


    #[doc="<p>Returns a unique endpoint specific to the AWS account making the call.</p>"]
    fn describe_endpoint(&self) -> Result<DescribeEndpointResponse, DescribeEndpointError>;


    #[doc="<p>Gets information about the specified thing.</p>"]
    fn describe_thing(&self,
                      input: &DescribeThingRequest)
                      -> Result<DescribeThingResponse, DescribeThingError>;


    #[doc="<p>Gets information about the specified thing type.</p>"]
    fn describe_thing_type(&self,
                           input: &DescribeThingTypeRequest)
                           -> Result<DescribeThingTypeResponse, DescribeThingTypeError>;


    #[doc="<p>Removes the specified policy from the specified certificate.</p>"]
    fn detach_principal_policy(&self,
                               input: &DetachPrincipalPolicyRequest)
                               -> Result<(), DetachPrincipalPolicyError>;


    #[doc="<p>Detaches the specified principal from the specified thing.</p>"]
    fn detach_thing_principal
        (&self,
         input: &DetachThingPrincipalRequest)
         -> Result<DetachThingPrincipalResponse, DetachThingPrincipalError>;


    #[doc="<p>Disables the specified rule.</p>"]
    fn disable_topic_rule(&self,
                          input: &DisableTopicRuleRequest)
                          -> Result<(), DisableTopicRuleError>;


    #[doc="<p>Enables the specified rule.</p>"]
    fn enable_topic_rule(&self,
                         input: &EnableTopicRuleRequest)
                         -> Result<(), EnableTopicRuleError>;


    #[doc="<p>Gets the logging options.</p>"]
    fn get_logging_options(&self) -> Result<GetLoggingOptionsResponse, GetLoggingOptionsError>;


    #[doc="<p>Gets information about the specified policy with the policy document of the default version.</p>"]
    fn get_policy(&self, input: &GetPolicyRequest) -> Result<GetPolicyResponse, GetPolicyError>;


    #[doc="<p>Gets information about the specified policy version.</p>"]
    fn get_policy_version(&self,
                          input: &GetPolicyVersionRequest)
                          -> Result<GetPolicyVersionResponse, GetPolicyVersionError>;


    #[doc="<p>Gets a registration code used to register a CA certificate with AWS IoT.</p>"]
    fn get_registration_code(&self)
                             -> Result<GetRegistrationCodeResponse, GetRegistrationCodeError>;


    #[doc="<p>Gets information about the specified rule.</p>"]
    fn get_topic_rule(&self,
                      input: &GetTopicRuleRequest)
                      -> Result<GetTopicRuleResponse, GetTopicRuleError>;


    #[doc="<p>Lists the CA certificates registered for your AWS account.</p> <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>"]
    fn list_ca_certificates(&self,
                            input: &ListCACertificatesRequest)
                            -> Result<ListCACertificatesResponse, ListCACertificatesError>;


    #[doc="<p>Lists the certificates registered in your AWS account.</p> <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>"]
    fn list_certificates(&self,
                         input: &ListCertificatesRequest)
                         -> Result<ListCertificatesResponse, ListCertificatesError>;


    #[doc="<p>List the device certificates signed by the specified CA certificate.</p>"]
    fn list_certificates_by_ca
        (&self,
         input: &ListCertificatesByCARequest)
         -> Result<ListCertificatesByCAResponse, ListCertificatesByCAError>;


    #[doc="<p>Lists certificates that are being transfered but not yet accepted.</p>"]
    fn list_outgoing_certificates
        (&self,
         input: &ListOutgoingCertificatesRequest)
         -> Result<ListOutgoingCertificatesResponse, ListOutgoingCertificatesError>;


    #[doc="<p>Lists your policies.</p>"]
    fn list_policies(&self,
                     input: &ListPoliciesRequest)
                     -> Result<ListPoliciesResponse, ListPoliciesError>;


    #[doc="<p>Lists the principals associated with the specified policy.</p>"]
    fn list_policy_principals
        (&self,
         input: &ListPolicyPrincipalsRequest)
         -> Result<ListPolicyPrincipalsResponse, ListPolicyPrincipalsError>;


    #[doc="<p>Lists the versions of the specified policy and identifies the default version.</p>"]
    fn list_policy_versions(&self,
                            input: &ListPolicyVersionsRequest)
                            -> Result<ListPolicyVersionsResponse, ListPolicyVersionsError>;


    #[doc="<p>Lists the policies attached to the specified principal. If you use an Cognito identity, the ID must be in <a href=\"http://docs.aws.amazon.com/cognitoidentity/latest/APIReference/API_GetCredentialsForIdentity.html#API_GetCredentialsForIdentity_RequestSyntax\">AmazonCognito Identity format</a>.</p>"]
    fn list_principal_policies
        (&self,
         input: &ListPrincipalPoliciesRequest)
         -> Result<ListPrincipalPoliciesResponse, ListPrincipalPoliciesError>;


    #[doc="<p>Lists the things associated with the specified principal.</p>"]
    fn list_principal_things(&self,
                             input: &ListPrincipalThingsRequest)
                             -> Result<ListPrincipalThingsResponse, ListPrincipalThingsError>;


    #[doc="<p>Lists the principals associated with the specified thing.</p>"]
    fn list_thing_principals(&self,
                             input: &ListThingPrincipalsRequest)
                             -> Result<ListThingPrincipalsResponse, ListThingPrincipalsError>;


    #[doc="<p>Lists the existing thing types.</p>"]
    fn list_thing_types(&self,
                        input: &ListThingTypesRequest)
                        -> Result<ListThingTypesResponse, ListThingTypesError>;


    #[doc="<p>Lists your things. Use the <b>attributeName</b> and <b>attributeValue</b> parameters to filter your things. For example, calling <code>ListThings</code> with attributeName=Color and attributeValue=Red retrieves all things in the registry that contain an attribute <b>Color</b> with the value <b>Red</b>. </p>"]
    fn list_things(&self,
                   input: &ListThingsRequest)
                   -> Result<ListThingsResponse, ListThingsError>;


    #[doc="<p>Lists the rules for the specific topic.</p>"]
    fn list_topic_rules(&self,
                        input: &ListTopicRulesRequest)
                        -> Result<ListTopicRulesResponse, ListTopicRulesError>;


    #[doc="<p>Registers a CA certificate with AWS IoT. This CA certificate can then be used to sign device certificates, which can be then registered with AWS IoT. You can register up to 10 CA certificates per AWS account that have the same subject field. This enables you to have up to 10 certificate authorities sign your device certificates. If you have more than one CA certificate registered, make sure you pass the CA certificate when you register your device certificates with the RegisterCertificate API.</p>"]
    fn register_ca_certificate
        (&self,
         input: &RegisterCACertificateRequest)
         -> Result<RegisterCACertificateResponse, RegisterCACertificateError>;


    #[doc="<p>Registers a device certificate with AWS IoT. If you have more than one CA certificate that has the same subject field, you must specify the CA certificate that was used to sign the device certificate being registered.</p>"]
    fn register_certificate(&self,
                            input: &RegisterCertificateRequest)
                            -> Result<RegisterCertificateResponse, RegisterCertificateError>;


    #[doc="<p>Rejects a pending certificate transfer. After AWS IoT rejects a certificate transfer, the certificate status changes from <b>PENDING_TRANSFER</b> to <b>INACTIVE</b>.</p> <p>To check for pending certificate transfers, call <a>ListCertificates</a> to enumerate your certificates.</p> <p>This operation can only be called by the transfer destination. After it is called, the certificate will be returned to the source's account in the INACTIVE state.</p>"]
    fn reject_certificate_transfer(&self,
                                   input: &RejectCertificateTransferRequest)
                                   -> Result<(), RejectCertificateTransferError>;


    #[doc="<p>Replaces the specified rule. You must specify all parameters for the new rule. Creating rules is an administrator-level action. Any user who has permission to create rules will be able to access data processed by the rule.</p>"]
    fn replace_topic_rule(&self,
                          input: &ReplaceTopicRuleRequest)
                          -> Result<(), ReplaceTopicRuleError>;


    #[doc="<p>Sets the specified version of the specified policy as the policy's default (operative) version. This action affects all certificates to which the policy is attached. To list the principals the policy is attached to, use the ListPrincipalPolicy API.</p>"]
    fn set_default_policy_version(&self,
                                  input: &SetDefaultPolicyVersionRequest)
                                  -> Result<(), SetDefaultPolicyVersionError>;


    #[doc="<p>Sets the logging options.</p>"]
    fn set_logging_options(&self,
                           input: &SetLoggingOptionsRequest)
                           -> Result<(), SetLoggingOptionsError>;


    #[doc="<p>Transfers the specified certificate to the specified AWS account.</p> <p>You can cancel the transfer until it is acknowledged by the recipient.</p> <p>No notification is sent to the transfer destination's account. It is up to the caller to notify the transfer target.</p> <p>The certificate being transferred must not be in the ACTIVE state. You can use the UpdateCertificate API to deactivate it.</p> <p>The certificate must not have any policies attached to it. You can use the DetachPrincipalPolicy API to detach them.</p>"]
    fn transfer_certificate(&self,
                            input: &TransferCertificateRequest)
                            -> Result<TransferCertificateResponse, TransferCertificateError>;


    #[doc="<p>Updates a registered CA certificate.</p>"]
    fn update_ca_certificate(&self,
                             input: &UpdateCACertificateRequest)
                             -> Result<(), UpdateCACertificateError>;


    #[doc="<p>Updates the status of the specified certificate. This operation is idempotent.</p> <p>Moving a certificate from the ACTIVE state (including REVOKED) will not disconnect currently connected devices, but these devices will be unable to reconnect.</p> <p>The ACTIVE state is required to authenticate devices connecting to AWS IoT using a certificate.</p>"]
    fn update_certificate(&self,
                          input: &UpdateCertificateRequest)
                          -> Result<(), UpdateCertificateError>;


    #[doc="<p>Updates the data for a thing.</p>"]
    fn update_thing(&self,
                    input: &UpdateThingRequest)
                    -> Result<UpdateThingResponse, UpdateThingError>;
}
/// A client for the AWS IoT API.
pub struct IotClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> IotClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        IotClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Iot for IotClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Accepts a pending certificate transfer. The default state of the certificate is INACTIVE.</p> <p>To check for pending certificate transfers, call <a>ListCertificates</a> to enumerate your certificates.</p>"]
    fn accept_certificate_transfer(&self,
                                   input: &AcceptCertificateTransferRequest)
                                   -> Result<(), AcceptCertificateTransferError> {


        let request_uri = format!("/accept-certificate-transfer/{certificate_id}",
                                  certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("PATCH", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.set_as_active {
            params.put("setAsActive", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => Err(AcceptCertificateTransferError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Attaches the specified policy to the specified principal (certificate or other credential).</p>"]
    fn attach_principal_policy(&self,
                               input: &AttachPrincipalPolicyRequest)
                               -> Result<(), AttachPrincipalPolicyError> {


        let request_uri = format!("/principal-policies/{policy_name}",
                                  policy_name = input.policy_name);

        let mut request = SignedRequest::new("PUT", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(AttachPrincipalPolicyError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Attaches the specified principal to the specified thing.</p>"]
    fn attach_thing_principal
        (&self,
         input: &AttachThingPrincipalRequest)
         -> Result<AttachThingPrincipalResponse, AttachThingPrincipalError> {


        let request_uri = format!("/things/{thing_name}/principals",
                                  thing_name = input.thing_name);

        let mut request = SignedRequest::new("PUT", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AttachThingPrincipalResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(AttachThingPrincipalError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Cancels a pending transfer for the specified certificate.</p> <p> <b>Note</b> Only the transfer source account can use this operation to cancel a transfer. (Transfer destinations can use <a>RejectCertificateTransfer</a> instead.) After transfer, AWS IoT returns the certificate to the source account in the INACTIVE state. After the destination account has accepted the transfer, the transfer cannot be cancelled.</p> <p>After a certificate transfer is cancelled, the status of the certificate changes from PENDING_TRANSFER to INACTIVE.</p>"]
    fn cancel_certificate_transfer(&self,
                                   input: &CancelCertificateTransferRequest)
                                   -> Result<(), CancelCertificateTransferError> {


        let request_uri = format!("/cancel-certificate-transfer/{certificate_id}",
                                  certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("PATCH", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => Err(CancelCertificateTransferError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates an X.509 certificate using the specified certificate signing request.</p> <p> <b>Note:</b> The CSR must include a public key that is either an RSA key with a length of at least 2048 bits or an ECC key from NIST P-256 or NIST P-384 curves. </p> <p> <b>Note:</b> Reusing the same certificate signing request (CSR) results in a distinct certificate.</p> <p>You can create multiple certificates in a batch by creating a directory, copying multiple .csr files into that directory, and then specifying that directory on the command line. The following commands show how to create a batch of certificates given a batch of CSRs.</p> <p>Assuming a set of CSRs are located inside of the directory my-csr-directory:</p> <p>On Linux and OS X, the command is:</p> <p>$ ls my-csr-directory/ | xargs -I {} aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/{}</p> <p>This command lists all of the CSRs in my-csr-directory and pipes each CSR file name to the aws iot create-certificate-from-csr AWS CLI command to create a certificate for the corresponding CSR.</p> <p>The aws iot create-certificate-from-csr part of the command can also be run in parallel to speed up the certificate creation process:</p> <p>$ ls my-csr-directory/ | xargs -P 10 -I {} aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/{}</p> <p>On Windows PowerShell, the command to create certificates for all CSRs in my-csr-directory is:</p> <p>&gt; ls -Name my-csr-directory | %{aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/$_}</p> <p>On a Windows command prompt, the command to create certificates for all CSRs in my-csr-directory is:</p> <p>&gt; forfiles /p my-csr-directory /c \"cmd /c aws iot create-certificate-from-csr --certificate-signing-request file://@path\"</p>"]
    fn create_certificate_from_csr
        (&self,
         input: &CreateCertificateFromCsrRequest)
         -> Result<CreateCertificateFromCsrResponse, CreateCertificateFromCsrError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/certificates";

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));
        let mut params = Params::new();
        if let Some(ref x) = input.set_as_active {
            params.put("setAsActive", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateCertificateFromCsrResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => Err(CreateCertificateFromCsrError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a 2048-bit RSA key pair and issues an X.509 certificate using the issued public key.</p> <p> <b>Note</b> This is the only time AWS IoT issues the private key for this certificate, so it is important to keep it in a secure location.</p>"]
    fn create_keys_and_certificate
        (&self,
         input: &CreateKeysAndCertificateRequest)
         -> Result<CreateKeysAndCertificateResponse, CreateKeysAndCertificateError> {


        let request_uri = "/keys-and-certificate";

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.set_as_active {
            params.put("setAsActive", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateKeysAndCertificateResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => Err(CreateKeysAndCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates an AWS IoT policy.</p> <p>The created policy is the default version for the policy. This operation creates a policy version with a version identifier of <b>1</b> and sets <b>1</b> as the policy's default version.</p>"]
    fn create_policy(&self,
                     input: &CreatePolicyRequest)
                     -> Result<CreatePolicyResponse, CreatePolicyError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/policies/{policy_name}", policy_name = input.policy_name);

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreatePolicyResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(CreatePolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new version of the specified AWS IoT policy. To update a policy, create a new policy version. A managed policy can have up to five versions. If the policy has five versions, you must use <a>DeletePolicyVersion</a> to delete an existing version before you create a new one.</p> <p>Optionally, you can set the new version as the policy's default version. The default version is the operative version (that is, the version that is in effect for the certificates to which the policy is attached).</p>"]
    fn create_policy_version(&self,
                             input: &CreatePolicyVersionRequest)
                             -> Result<CreatePolicyVersionResponse, CreatePolicyVersionError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/policies/{policy_name}/version",
                                  policy_name = input.policy_name);

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));
        let mut params = Params::new();
        if let Some(ref x) = input.set_as_default {
            params.put("setAsDefault", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreatePolicyVersionResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(CreatePolicyVersionError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a thing record in the thing registry.</p>"]
    fn create_thing(&self,
                    input: &CreateThingRequest)
                    -> Result<CreateThingResponse, CreateThingError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/things/{thing_name}", thing_name = input.thing_name);

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateThingResponse>(&body).unwrap();



                Ok(result)
            }
            _ => Err(CreateThingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a new thing type.</p>"]
    fn create_thing_type(&self,
                         input: &CreateThingTypeRequest)
                         -> Result<CreateThingTypeResponse, CreateThingTypeError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/thing-types/{thing_type_name}",
                                  thing_type_name = input.thing_type_name);

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateThingTypeResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(CreateThingTypeError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a rule. Creating rules is an administrator-level action. Any user who has permission to create rules will be able to access data processed by the rule.</p>"]
    fn create_topic_rule(&self,
                         input: &CreateTopicRuleRequest)
                         -> Result<(), CreateTopicRuleError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/rules/{rule_name}", rule_name = input.rule_name);

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(CreateTopicRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a registered CA certificate.</p>"]
    fn delete_ca_certificate(&self,
                             input: &DeleteCACertificateRequest)
                             -> Result<DeleteCACertificateResponse, DeleteCACertificateError> {


        let request_uri = format!("/cacertificate/{ca_certificate_id}",
                                  ca_certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteCACertificateResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DeleteCACertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified certificate.</p> <p>A certificate cannot be deleted if it has a policy attached to it or if its status is set to ACTIVE. To delete a certificate, first use the <a>DetachPrincipalPolicy</a> API to detach all policies. Next, use the <a>UpdateCertificate</a> API to set the certificate to the INACTIVE status.</p>"]
    fn delete_certificate(&self,
                          input: &DeleteCertificateRequest)
                          -> Result<(), DeleteCertificateError> {


        let request_uri = format!("/certificates/{certificate_id}",
                                  certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(DeleteCertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified policy.</p> <p>A policy cannot be deleted if it has non-default versions or it is attached to any certificate.</p> <p>To delete a policy, use the DeletePolicyVersion API to delete all non-default versions of the policy; use the DetachPrincipalPolicy API to detach the policy from any certificate; and then use the DeletePolicy API to delete the policy.</p> <p>When a policy is deleted using DeletePolicy, its default version is deleted with it.</p>"]
    fn delete_policy(&self, input: &DeletePolicyRequest) -> Result<(), DeletePolicyError> {


        let request_uri = format!("/policies/{policy_name}", policy_name = input.policy_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(DeletePolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified version of the specified policy. You cannot delete the default version of a policy using this API. To delete the default version of a policy, use <a>DeletePolicy</a>. To find out which version of a policy is marked as the default version, use ListPolicyVersions.</p>"]
    fn delete_policy_version(&self,
                             input: &DeletePolicyVersionRequest)
                             -> Result<(), DeletePolicyVersionError> {


        let request_uri = format!("/policies/{policy_name}/version/{policy_version_id}",
                                  policy_name = input.policy_name,
                                  policy_version_id = input.policy_version_id);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(DeletePolicyVersionError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a CA certificate registration code.</p>"]
    fn delete_registration_code
        (&self)
         -> Result<DeleteRegistrationCodeResponse, DeleteRegistrationCodeError> {


        let request_uri = "/registrationcode";

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteRegistrationCodeResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                Err(DeleteRegistrationCodeError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified thing.</p>"]
    fn delete_thing(&self,
                    input: &DeleteThingRequest)
                    -> Result<DeleteThingResponse, DeleteThingError> {


        let request_uri = format!("/things/{thing_name}", thing_name = input.thing_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.expected_version {
            params.put("expectedVersion", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteThingResponse>(&body).unwrap();



                Ok(result)
            }
            _ => Err(DeleteThingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes the specified thing type . You cannot delete a thing type if it has things associated with it. To delete a thing type, first mark it as deprecated by calling <a>DeprecateThingType</a>, then remove any associated things by calling <a>UpdateThing</a> to change the thing type on any associated thing, and finally use <a>DeleteThingType</a> to delete the thing type.</p>"]
    fn delete_thing_type(&self,
                         input: &DeleteThingTypeRequest)
                         -> Result<DeleteThingTypeResponse, DeleteThingTypeError> {


        let request_uri = format!("/thing-types/{thing_type_name}",
                                  thing_type_name = input.thing_type_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteThingTypeResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DeleteThingTypeError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified rule.</p>"]
    fn delete_topic_rule(&self,
                         input: &DeleteTopicRuleRequest)
                         -> Result<(), DeleteTopicRuleError> {


        let request_uri = format!("/rules/{rule_name}", rule_name = input.rule_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(DeleteTopicRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Deprecates a thing type. You can not associate new things with deprecated thing type.</p>"]
    fn deprecate_thing_type(&self,
                            input: &DeprecateThingTypeRequest)
                            -> Result<DeprecateThingTypeResponse, DeprecateThingTypeError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/thing-types/{thing_type_name}/deprecate",
                                  thing_type_name = input.thing_type_name);

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeprecateThingTypeResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DeprecateThingTypeError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Describes a registered CA certificate.</p>"]
    fn describe_ca_certificate
        (&self,
         input: &DescribeCACertificateRequest)
         -> Result<DescribeCACertificateResponse, DescribeCACertificateError> {


        let request_uri = format!("/cacertificate/{ca_certificate_id}",
                                  ca_certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeCACertificateResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                Err(DescribeCACertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about the specified certificate.</p>"]
    fn describe_certificate(&self,
                            input: &DescribeCertificateRequest)
                            -> Result<DescribeCertificateResponse, DescribeCertificateError> {


        let request_uri = format!("/certificates/{certificate_id}",
                                  certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeCertificateResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DescribeCertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Returns a unique endpoint specific to the AWS account making the call.</p>"]
    fn describe_endpoint(&self) -> Result<DescribeEndpointResponse, DescribeEndpointError> {


        let request_uri = "/endpoint";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeEndpointResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DescribeEndpointError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about the specified thing.</p>"]
    fn describe_thing(&self,
                      input: &DescribeThingRequest)
                      -> Result<DescribeThingResponse, DescribeThingError> {


        let request_uri = format!("/things/{thing_name}", thing_name = input.thing_name);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeThingResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DescribeThingError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about the specified thing type.</p>"]
    fn describe_thing_type(&self,
                           input: &DescribeThingTypeRequest)
                           -> Result<DescribeThingTypeResponse, DescribeThingTypeError> {


        let request_uri = format!("/thing-types/{thing_type_name}",
                                  thing_type_name = input.thing_type_name);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeThingTypeResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DescribeThingTypeError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Removes the specified policy from the specified certificate.</p>"]
    fn detach_principal_policy(&self,
                               input: &DetachPrincipalPolicyRequest)
                               -> Result<(), DetachPrincipalPolicyError> {


        let request_uri = format!("/principal-policies/{policy_name}",
                                  policy_name = input.policy_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(DetachPrincipalPolicyError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Detaches the specified principal from the specified thing.</p>"]
    fn detach_thing_principal
        (&self,
         input: &DetachThingPrincipalRequest)
         -> Result<DetachThingPrincipalResponse, DetachThingPrincipalError> {


        let request_uri = format!("/things/{thing_name}/principals",
                                  thing_name = input.thing_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DetachThingPrincipalResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(DetachThingPrincipalError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Disables the specified rule.</p>"]
    fn disable_topic_rule(&self,
                          input: &DisableTopicRuleRequest)
                          -> Result<(), DisableTopicRuleError> {


        let request_uri = format!("/rules/{rule_name}/disable", rule_name = input.rule_name);

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(DisableTopicRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Enables the specified rule.</p>"]
    fn enable_topic_rule(&self,
                         input: &EnableTopicRuleRequest)
                         -> Result<(), EnableTopicRuleError> {


        let request_uri = format!("/rules/{rule_name}/enable", rule_name = input.rule_name);

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(EnableTopicRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Gets the logging options.</p>"]
    fn get_logging_options(&self) -> Result<GetLoggingOptionsResponse, GetLoggingOptionsError> {


        let request_uri = "/loggingOptions";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetLoggingOptionsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(GetLoggingOptionsError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about the specified policy with the policy document of the default version.</p>"]
    fn get_policy(&self, input: &GetPolicyRequest) -> Result<GetPolicyResponse, GetPolicyError> {


        let request_uri = format!("/policies/{policy_name}", policy_name = input.policy_name);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetPolicyResponse>(&body).unwrap();



                Ok(result)
            }
            _ => Err(GetPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets information about the specified policy version.</p>"]
    fn get_policy_version(&self,
                          input: &GetPolicyVersionRequest)
                          -> Result<GetPolicyVersionResponse, GetPolicyVersionError> {


        let request_uri = format!("/policies/{policy_name}/version/{policy_version_id}",
                                  policy_name = input.policy_name,
                                  policy_version_id = input.policy_version_id);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetPolicyVersionResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(GetPolicyVersionError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Gets a registration code used to register a CA certificate with AWS IoT.</p>"]
    fn get_registration_code(&self)
                             -> Result<GetRegistrationCodeResponse, GetRegistrationCodeError> {


        let request_uri = "/registrationcode";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetRegistrationCodeResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(GetRegistrationCodeError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about the specified rule.</p>"]
    fn get_topic_rule(&self,
                      input: &GetTopicRuleRequest)
                      -> Result<GetTopicRuleResponse, GetTopicRuleError> {


        let request_uri = format!("/rules/{rule_name}", rule_name = input.rule_name);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetTopicRuleResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(GetTopicRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists the CA certificates registered for your AWS account.</p> <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>"]
    fn list_ca_certificates(&self,
                            input: &ListCACertificatesRequest)
                            -> Result<ListCACertificatesResponse, ListCACertificatesError> {


        let request_uri = "/cacertificates";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("ascendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListCACertificatesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListCACertificatesError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the certificates registered in your AWS account.</p> <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>"]
    fn list_certificates(&self,
                         input: &ListCertificatesRequest)
                         -> Result<ListCertificatesResponse, ListCertificatesError> {


        let request_uri = "/certificates";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("ascendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListCertificatesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListCertificatesError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>List the device certificates signed by the specified CA certificate.</p>"]
    fn list_certificates_by_ca
        (&self,
         input: &ListCertificatesByCARequest)
         -> Result<ListCertificatesByCAResponse, ListCertificatesByCAError> {


        let request_uri = format!("/certificates-by-ca/{ca_certificate_id}",
                                  ca_certificate_id = input.ca_certificate_id);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("ascendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListCertificatesByCAResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListCertificatesByCAError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Lists certificates that are being transfered but not yet accepted.</p>"]
    fn list_outgoing_certificates
        (&self,
         input: &ListOutgoingCertificatesRequest)
         -> Result<ListOutgoingCertificatesResponse, ListOutgoingCertificatesError> {


        let request_uri = "/certificates-out-going";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("ascendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListOutgoingCertificatesResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => Err(ListOutgoingCertificatesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists your policies.</p>"]
    fn list_policies(&self,
                     input: &ListPoliciesRequest)
                     -> Result<ListPoliciesResponse, ListPoliciesError> {


        let request_uri = "/policies";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("ascendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPoliciesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListPoliciesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists the principals associated with the specified policy.</p>"]
    fn list_policy_principals
        (&self,
         input: &ListPolicyPrincipalsRequest)
         -> Result<ListPolicyPrincipalsResponse, ListPolicyPrincipalsError> {


        let request_uri = "/policy-principals";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("ascendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPolicyPrincipalsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListPolicyPrincipalsError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the versions of the specified policy and identifies the default version.</p>"]
    fn list_policy_versions(&self,
                            input: &ListPolicyVersionsRequest)
                            -> Result<ListPolicyVersionsResponse, ListPolicyVersionsError> {


        let request_uri = format!("/policies/{policy_name}/version",
                                  policy_name = input.policy_name);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPolicyVersionsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListPolicyVersionsError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the policies attached to the specified principal. If you use an Cognito identity, the ID must be in <a href=\"http://docs.aws.amazon.com/cognitoidentity/latest/APIReference/API_GetCredentialsForIdentity.html#API_GetCredentialsForIdentity_RequestSyntax\">AmazonCognito Identity format</a>.</p>"]
    fn list_principal_policies
        (&self,
         input: &ListPrincipalPoliciesRequest)
         -> Result<ListPrincipalPoliciesResponse, ListPrincipalPoliciesError> {


        let request_uri = "/principal-policies";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("ascendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPrincipalPoliciesResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                Err(ListPrincipalPoliciesError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the things associated with the specified principal.</p>"]
    fn list_principal_things(&self,
                             input: &ListPrincipalThingsRequest)
                             -> Result<ListPrincipalThingsResponse, ListPrincipalThingsError> {


        let request_uri = "/principals/things";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPrincipalThingsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListPrincipalThingsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the principals associated with the specified thing.</p>"]
    fn list_thing_principals(&self,
                             input: &ListThingPrincipalsRequest)
                             -> Result<ListThingPrincipalsResponse, ListThingPrincipalsError> {


        let request_uri = format!("/things/{thing_name}/principals",
                                  thing_name = input.thing_name);

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListThingPrincipalsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListThingPrincipalsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Lists the existing thing types.</p>"]
    fn list_thing_types(&self,
                        input: &ListThingTypesRequest)
                        -> Result<ListThingTypesResponse, ListThingTypesError> {


        let request_uri = "/thing-types";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.thing_type_name {
            params.put("thingTypeName", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListThingTypesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListThingTypesError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Lists your things. Use the <b>attributeName</b> and <b>attributeValue</b> parameters to filter your things. For example, calling <code>ListThings</code> with attributeName=Color and attributeValue=Red retrieves all things in the registry that contain an attribute <b>Color</b> with the value <b>Red</b>. </p>"]
    fn list_things(&self,
                   input: &ListThingsRequest)
                   -> Result<ListThingsResponse, ListThingsError> {


        let request_uri = "/things";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.attribute_name {
            params.put("attributeName", x);
        }
        if let Some(ref x) = input.attribute_value {
            params.put("attributeValue", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.thing_type_name {
            params.put("thingTypeName", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListThingsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => Err(ListThingsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists the rules for the specific topic.</p>"]
    fn list_topic_rules(&self,
                        input: &ListTopicRulesRequest)
                        -> Result<ListTopicRulesResponse, ListTopicRulesError> {


        let request_uri = "/rules";

        let mut request = SignedRequest::new("GET", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.rule_disabled {
            params.put("ruleDisabled", x);
        }
        if let Some(ref x) = input.topic {
            params.put("topic", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListTopicRulesResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(ListTopicRulesError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Registers a CA certificate with AWS IoT. This CA certificate can then be used to sign device certificates, which can be then registered with AWS IoT. You can register up to 10 CA certificates per AWS account that have the same subject field. This enables you to have up to 10 certificate authorities sign your device certificates. If you have more than one CA certificate registered, make sure you pass the CA certificate when you register your device certificates with the RegisterCertificate API.</p>"]
    fn register_ca_certificate
        (&self,
         input: &RegisterCACertificateRequest)
         -> Result<RegisterCACertificateResponse, RegisterCACertificateError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/cacertificate";

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));
        let mut params = Params::new();
        if let Some(ref x) = input.allow_auto_registration {
            params.put("allowAutoRegistration", x);
        }
        if let Some(ref x) = input.set_as_active {
            params.put("setAsActive", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<RegisterCACertificateResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                Err(RegisterCACertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Registers a device certificate with AWS IoT. If you have more than one CA certificate that has the same subject field, you must specify the CA certificate that was used to sign the device certificate being registered.</p>"]
    fn register_certificate(&self,
                            input: &RegisterCertificateRequest)
                            -> Result<RegisterCertificateResponse, RegisterCertificateError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/certificate/register";

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<RegisterCertificateResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(RegisterCertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Rejects a pending certificate transfer. After AWS IoT rejects a certificate transfer, the certificate status changes from <b>PENDING_TRANSFER</b> to <b>INACTIVE</b>.</p> <p>To check for pending certificate transfers, call <a>ListCertificates</a> to enumerate your certificates.</p> <p>This operation can only be called by the transfer destination. After it is called, the certificate will be returned to the source's account in the INACTIVE state.</p>"]
    fn reject_certificate_transfer(&self,
                                   input: &RejectCertificateTransferRequest)
                                   -> Result<(), RejectCertificateTransferError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/reject-certificate-transfer/{certificate_id}",
                                  certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("PATCH", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => Err(RejectCertificateTransferError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Replaces the specified rule. You must specify all parameters for the new rule. Creating rules is an administrator-level action. Any user who has permission to create rules will be able to access data processed by the rule.</p>"]
    fn replace_topic_rule(&self,
                          input: &ReplaceTopicRuleRequest)
                          -> Result<(), ReplaceTopicRuleError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/rules/{rule_name}", rule_name = input.rule_name);

        let mut request = SignedRequest::new("PATCH", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(ReplaceTopicRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Sets the specified version of the specified policy as the policy's default (operative) version. This action affects all certificates to which the policy is attached. To list the principals the policy is attached to, use the ListPrincipalPolicy API.</p>"]
    fn set_default_policy_version(&self,
                                  input: &SetDefaultPolicyVersionRequest)
                                  -> Result<(), SetDefaultPolicyVersionError> {


        let request_uri = format!("/policies/{policy_name}/version/{policy_version_id}",
                                  policy_name = input.policy_name,
                                  policy_version_id = input.policy_version_id);

        let mut request = SignedRequest::new("PATCH", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());



        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => Err(SetDefaultPolicyVersionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Sets the logging options.</p>"]
    fn set_logging_options(&self,
                           input: &SetLoggingOptionsRequest)
                           -> Result<(), SetLoggingOptionsError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = "/loggingOptions";

        let mut request = SignedRequest::new("POST", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(SetLoggingOptionsError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Transfers the specified certificate to the specified AWS account.</p> <p>You can cancel the transfer until it is acknowledged by the recipient.</p> <p>No notification is sent to the transfer destination's account. It is up to the caller to notify the transfer target.</p> <p>The certificate being transferred must not be in the ACTIVE state. You can use the UpdateCertificate API to deactivate it.</p> <p>The certificate must not have any policies attached to it. You can use the DetachPrincipalPolicy API to detach them.</p>"]
    fn transfer_certificate(&self,
                            input: &TransferCertificateRequest)
                            -> Result<TransferCertificateResponse, TransferCertificateError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/transfer-certificate/{certificate_id}",
                                  certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("PATCH", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));
        let mut params = Params::new();
        params.put("targetAwsAccount", &input.target_aws_account);
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<TransferCertificateResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                Err(TransferCertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Updates a registered CA certificate.</p>"]
    fn update_ca_certificate(&self,
                             input: &UpdateCACertificateRequest)
                             -> Result<(), UpdateCACertificateError> {


        let request_uri = format!("/cacertificate/{ca_certificate_id}",
                                  ca_certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("PUT", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.new_auto_registration_status {
            params.put("newAutoRegistrationStatus", x);
        }
        if let Some(ref x) = input.new_status {
            params.put("newStatus", x);
        }
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(UpdateCACertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Updates the status of the specified certificate. This operation is idempotent.</p> <p>Moving a certificate from the ACTIVE state (including REVOKED) will not disconnect currently connected devices, but these devices will be unable to reconnect.</p> <p>The ACTIVE state is required to authenticate devices connecting to AWS IoT using a certificate.</p>"]
    fn update_certificate(&self,
                          input: &UpdateCertificateRequest)
                          -> Result<(), UpdateCertificateError> {


        let request_uri = format!("/certificates/{certificate_id}",
                                  certificate_id = input.certificate_id);

        let mut request = SignedRequest::new("PUT", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        params.put("newStatus", &input.new_status);
        request.set_params(params);

        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                Err(UpdateCertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Updates the data for a thing.</p>"]
    fn update_thing(&self,
                    input: &UpdateThingRequest)
                    -> Result<UpdateThingResponse, UpdateThingError> {
        let encoded = serde_json::to_string(input).unwrap();

        let request_uri = format!("/things/{thing_name}", thing_name = input.thing_name);

        let mut request = SignedRequest::new("PATCH", "execute-api", self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        request.set_payload(Some(encoded.into_bytes()));


        request.sign(&self.credentials_provider.credentials()?);

        let response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body = response.body;

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateThingResponse>(&body).unwrap();



                Ok(result)
            }
            _ => Err(UpdateThingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
