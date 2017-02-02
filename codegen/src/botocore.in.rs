use std::collections::{HashSet, BTreeMap};

use serialization::{ShapesMap, ShapeName};

#[derive(Debug, Deserialize)]
pub struct Service {
    pub documentation: Option<String>,
    pub examples: Option<BTreeMap<String, String>>,
    pub metadata: Metadata,
    pub operations: BTreeMap<String, Operation>,
    #[serde(deserialize_with="ShapesMap::deserialize_shapes_map")]
    pub shapes: BTreeMap<String, Shape>,
    pub version: Option<String>,
}

impl Service {
    pub fn client_type_name(&self) -> String {
        format!("{}Client", self.service_type_name())
    }

    pub fn service_type_name(&self) -> &str {
        match &self.metadata.service_full_name[..] {
            "AWS Application Discovery Service" => "ApplicationDiscovery",
            "AWS Budgets" => "Budgets",
            "AWS Certificate Manager" => "Acm",
            "AWS CloudFormation" => "CloudFormation",
            "AWS CloudTrail" => "CloudTrail",
            "AWS CodeBuild" => "CodeBuild",
            "AWS CodeCommit" => "CodeCommit",
            "AWS CodeDeploy" => "CodeDeploy",
            "AWS CodePipeline" => "CodePipeline",
            "AWS Config" => "ConfigService",
            "AWS Cost and Usage Report Service" => "CostAndUsageReport",
            "AWS Data Pipeline" => "DataPipeline",
            "AWS Database Migration Service" => "DatabaseMigration",
            "AWS Device Farm" => "DeviceFarm",
            "AWS Direct Connect" => "DirectConnect",
            "AWS Directory Service" => "DirectoryService",
            "AWS Elastic Beanstalk" => "ElasticBeanstalk",
            "AWS Health APIs and Notifications" => "Health",
            "AWS Identity and Access Management" => "Iam",
            "AWS Import/Export" => "ImportExport",
            "AWS IoT Data Plane" => "IotDataPlane",
            "AWS IoT" => "Iot",
            "AWS Key Management Service" => "Kms",
            "AWS Lambda" => "Lambda",
            "AWS Marketplace Commerce Analytics" => "MarketplaceCommerceAnalytics",
            "AWS OpsWorks" => "OpsWorks",
            "AWS OpsWorks for Chef Automate" => "OpsWorksCm",
            "AWS Security Token Service" => "Sts",
            "AWS Service Catalog" => "ServiceCatalog",
            "AWS Shield" => "Shield",
            "AWS Storage Gateway" => "StorageGateway",
            "AWS Support" => "Support",
            "AWS WAF" => "Waf",
            "AWSMarketplace Metering" => "MarketplaceMetering",
            "Amazon API Gateway" => "ApiGateway",
            "Amazon AppStream" => "AppStream",
            "Amazon CloudFront" => "CloudFront",
            "Amazon CloudHSM" => "CloudHsm",
            "Amazon CloudSearch Domain" => "CloudSearchDomain",
            "Amazon CloudSearch" => "CloudSearch",
            "Amazon CloudWatch Events" => "CloudWatchEvents",
            "Amazon CloudWatch Logs" => "CloudWatchLogs",
            "Amazon CloudWatch" => "CloudWatch",
            "Amazon Cognito Identity" => "CognitoIdentity",
            "Amazon Cognito Identity Provider" => "CognitoIdentityProvider",
            "Amazon Cognito Sync" => "CognitoSync",
            "Amazon DynamoDB Streams" => "DynamoDbStreams",
            "Amazon DynamoDB" => "DynamoDb",
            "Amazon EC2 Container Registry" => "Ecr",
            "Amazon EC2 Container Service" => "Ecs",
            "Amazon ElastiCache" => "ElastiCache",
            "Amazon Elastic Compute Cloud" => "Ec2",
            "Amazon Elastic File System" => "Efs",
            "Amazon Elastic MapReduce" => "Emr",
            "Amazon Elastic Transcoder" => "Ets",
            "Amazon Elasticsearch Service" => "ElasticsearchService",
            "Amazon GameLift" => "GameLift",
            "Amazon Glacier" => "Glacier",
            "Amazon Import/Export Snowball" => "Snowball",
            "Amazon Inspector" => "Inspector",
            "Amazon Kinesis Firehose" => "KinesisFirehose",
            "Amazon Kinesis" => "Kinesis",
            "Amazon Kinesis Analytics" => "KinesisAnalytics",
            "Amazon Lightsail" => "Lightsail",
            "Amazon Machine Learning" => "MachineLearning",
            "Amazon Redshift" => "Redshift",
            "Amazon Rekognition" => "Rekognition",
            "Amazon Relational Database Service" => "Rds",
            "Amazon Route 53 Domains" => "Route53Domains",
            "Amazon Route 53" => "Route53",
            "Amazon Simple Email Service" => "Ses",
            "Amazon Simple Notification Service" => "Sns",
            "Amazon Simple Queue Service" => "Sqs",
            "Amazon Simple Storage Service" => "S3",
            "Amazon Simple Systems Manager (SSM)" => "Ssm",
            "Amazon Simple Workflow Service" => "Swf",
            "Amazon SimpleDB" => "SimpleDb",
            "Amazon WorkSpaces" => "Workspaces",
            "Application Auto Scaling" => "ApplicationAutoScaling",
            "Auto Scaling" => "Autoscaling",
            "Elastic Load Balancing" => "Elb",
            name => panic!("Unknown service full name: {}", name),
        }
    }

    pub fn shape_for_value<'a>(&'a self, value: &Value) -> Option<&'a Shape> {
        self.shapes.get(&value.shape)
    }

    pub fn shape_for_member<'a>(&'a self, member: &Member) -> Option<&'a Shape> {
        self.shapes.get(&member.shape)
    }

    pub fn shape_type_for_member<'a>(&'a self, member: &Member) -> Option<ShapeType> {
        self.shapes.get(&member.shape).map(|ref shape| shape.shape_type)
    }

    pub fn signing_name(&self) -> String {
        match self.metadata.signing_name {
            Some(ref signing_name) => signing_name.to_string(),
            None => self.metadata.endpoint_prefix.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    #[serde(rename="requestUri")]
    pub request_uri: String,
    #[serde(rename="responseCode")]
    pub response_code: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub documentation: Option<String>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub documentation: Option<String>,
    #[serde(rename="resultWrapper")]
    pub result_wrapper: Option<String>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Error {
    pub documentation: Option<String>,
    pub error: Option<HttpError>,
    pub exception: Option<bool>,
    pub fault: Option<bool>,
    pub shape: String,
}

impl Error {
    pub fn idiomatic_error_name(&self) -> String {
        self.shape.replace("Exception", "")
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct HttpError {
    pub code: Option<String>,
    #[serde(rename="httpStatusCode")]
    pub http_status_code: i32,
    #[serde(rename="senderFault")]
    pub sender_fault: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub deprecated: Option<bool>,
    pub documentation: Option<String>,
    pub flattened: Option<bool>,
    pub location: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
    pub streaming: Option<bool>,
    #[serde(rename="xmlAttribute")]
    pub xml_attribute: Option<bool>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl Member {
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    pub fn streaming(&self) -> bool {
        self.streaming.unwrap_or(false)
    }
}

#[derive(Debug, Deserialize)]
pub struct XmlNamespace {
    pub prefix: Option<String>,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    pub documentation: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub required: Option<bool>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
}

impl Key {
    pub fn tag_name(&self) -> String {
        self.location_name.as_ref().map(String::as_ref).unwrap_or_else(|| "key").to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct Value {
    pub documentation: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
}

impl Value {
    pub fn tag_name(&self) -> String {
        self.location_name.as_ref().map(String::as_ref).unwrap_or_else(|| "value").to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct Shape {
    #[serde(rename="box")]
    pub aws_box: Option<bool>,
    pub documentation: Option<String>,
    pub error: Option<HttpError>,
    pub exception: Option<bool>,
    pub fault: Option<bool>,
    pub flattened: Option<bool>,
    pub key: Option<Key>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub max: Option<u64>,
    pub member: Option<Member>,
    pub members: Option<BTreeMap<String, Member>>,
    pub min: Option<i32>,
    pub pattern: Option<String>,
    pub payload: Option<String>,
    pub required: Option<Vec<String>>,
    #[serde(rename="enum")]
    pub shape_enum: Option<Vec<String>>,
    #[serde(rename="type")]
    pub shape_type: ShapeType,
    pub sensitive: Option<bool>,
    #[serde(rename="timestampFormat")]
    pub timestamp_format: Option<String>,
    pub value: Option<Value>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl Shape {
    pub fn is_primitive(&self) -> bool {
        is_primitive(&self.shape_type)
    }
}

pub fn is_primitive(shape_type: &ShapeType) -> bool {
    match *shape_type {
        ShapeType::Structure | ShapeType::Map | ShapeType::List => false,
        _ => true,
    }
}

impl<'a> Shape {
    pub fn key_type(&'a self) -> &'a str {
        &self.key.as_ref().expect("Key shape undefined").shape
    }

    pub fn value_type(&'a self) -> &'a str {
        &self.value.as_ref().expect("Value shape undefined").shape
    }

    pub fn member_type(&'a self) -> &'a str {
        &self.member.as_ref().expect("Member shape undefined").shape
    }

    pub fn required(&self, field: &'a str) -> bool {
        self.required.is_some() && self.required.as_ref().unwrap().contains(&String::from(field))
    }

    pub fn exception(&self) -> bool {
        self.exception.unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ShapeType {
    #[serde(rename="blob")]
    Blob,
    #[serde(rename="boolean")]
    Boolean,
    #[serde(rename="double")]
    Double,
    #[serde(rename="float")]
    Float,
    #[serde(rename="integer")]
    Integer,
    #[serde(rename="list")]
    List,
    #[serde(rename="long")]
    Long,
    #[serde(rename="map")]
    Map,
    #[serde(rename="string")]
    String,
    #[serde(rename="structure")]
    Structure,
    #[serde(rename="timestamp")]
    Timestamp,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub alias: Option<String>,
    pub deprecated: Option<bool>,
    pub documentation: Option<String>,
    #[serde(rename="documentationUrl")]
    pub documentation_url: Option<String>,
    pub errors: Option<Vec<Error>>,
    pub http: HttpRequest,
    pub input: Option<Input>,
    pub name: String,
    pub output: Option<Output>,
}

impl<'a> Operation {
    pub fn input_shape(&'a self) -> &'a str {
        &self.input.as_ref().expect("Operation input undefined").shape
    }

    pub fn output_shape_or(&'a self, default: &'a str) -> &'a str {
        match self.output.as_ref() {
            Some(output) => &output.shape,
            None => default,
        }
    }

    // botocore duplicates errors in a few places
    // return a unique set
    pub fn errors(&'a self) -> HashSet<&'a Error> {
        self.errors.as_ref().unwrap().iter().collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename="apiVersion")]
    pub api_version: String,
    #[serde(rename="checksumFormat")]
    pub checksum_format: Option<String>,
    #[serde(rename="endpointPrefix")]
    pub endpoint_prefix: String,
    #[serde(rename="globalEndpoint")]
    pub global_endpoint: Option<String>,
    #[serde(rename="jsonVersion")]
    pub json_version: Option<String>,
    pub protocol: String,
    #[serde(rename="serviceAbbreviation")]
    pub service_abbreviation: Option<String>,
    #[serde(rename="serviceFullName")]
    pub service_full_name: String,
    #[serde(rename="signatureVersion")]
    pub signature_version: String,
    #[serde(rename="signingName")]
    pub signing_name: Option<String>,
    #[serde(rename="targetPrefix")]
    pub target_prefix: Option<String>,
    #[serde(rename="timestampFormat")]
    pub timestamp_format: Option<String>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<String>,
}
