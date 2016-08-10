use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Service {
    pub documentation: Option<String>,
    pub examples: Option<BTreeMap<String, String>>,
    pub metadata: Metadata,
    pub operations: BTreeMap<String, Operation>,
    pub shapes: BTreeMap<String, Shape>,
    pub version: String,
}

impl Service {
    pub fn client_type_name(&self) -> String {
        format!("{}Client", self.service_type_name())
    }

    pub fn service_type_name(&self) -> &str {
        match &self.metadata.service_full_name[..] {
            "AWS Certificate Manager" => "Acm",
            "AWS CloudFormation" => "CloudFormation",
            "AWS CloudTrail" => "CloudTrail",
            "AWS CodeCommit" => "CodeCommit",
            "AWS CodeDeploy" => "CodeDeploy",
            "AWS CodePipeline" => "CodePipeline",
            "AWS Config" => "ConfigService",
            "AWS Data Pipeline" => "DataPipeline",
            "AWS Device Farm" => "DeviceFarm",
            "AWS Direct Connect" => "DirectConnect",
            "AWS Directory Service" => "DirectoryService",
            "AWS Elastic Beanstalk" => "ElasticBeanstalk",
            "AWS Identity and Access Management" => "Iam",
            "AWS Import/Export" => "ImportExport",
            "AWS IoT Data Plane" => "IotDataPlane",
            "AWS IoT" => "Iot",
            "AWS Key Management Service" => "Kms",
            "AWS Lambda" => "Lambda",
            "AWS Marketplace Commerce Analytics" => "MarketplaceCommerceAnalytics",
            "AWS OpsWorks" => "OpsWorks",
            "AWS Security Token Service" => "Sts",
            "AWS Storage Gateway" => "StorageGateway",
            "AWS Support" => "Support",
            "AWS WAF" => "Waf",
            "Amazon API Gateway" => "ApiGateway",
            "Amazon CloudFront" => "CloudFront",
            "Amazon CloudHSM" => "CloudHsm",
            "Amazon CloudSearch Domain" => "CloudSearchDomain",
            "Amazon CloudSearch" => "CloudSearch",
            "Amazon CloudWatch Events" => "CloudWatchEvents",
            "Amazon CloudWatch Logs" => "CloudWatchLogs",
            "Amazon CloudWatch" => "CloudWatch",
            "Amazon Cognito Identity" => "CognitoIdentity",
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
            "Amazon Inspector" => "Inspector",
            "Amazon Kinesis Firehose" => "KinesisFirehose",
            "Amazon Kinesis" => "Kinesis",
            "Amazon Machine Learning" => "MachineLearning",
            "Amazon Redshift" => "Redshift",
            "Amazon Relational Database Service" => "Rds",
            "Amazon Route 53 Domains" => "Route53Domains",
            "Amazon Route 53" => "Route53",
            "Amazon Simple Email Service" => "Ses",
            "Amazon Simple Notification Service" => "Sns",
            "Amazon Simple Queue Service" => "Sqs",
            "Amazon Simple Storage Service" => "S3",
            "Amazon Simple Systems Management Service" => "Ssm",
            "Amazon Simple Workflow Service" => "Swf",
            "Amazon SimpleDB" => "SimpleDb",
            "Amazon WorkSpaces" => "Workspaces",
            "Auto Scaling" => "Autoscaling",
            "Elastic Load Balancing" => "Elb",
            name => panic!("Unknown service full name: {}", name),
        }
    }

    pub fn shape_for_member<'a>(&'a self, member: &Member) -> Option<&'a Shape> {
        self.shapes.get(&member.shape).map(|shape| shape)
    }

    pub fn shape_type_for_member<'a>(&'a self, member: &Member) -> Option<&'a str> {
        self.shapes.get(&member.shape).map(|ref shape| &shape.shape_type[..])
    }

    pub fn signing_name(&self) -> String {
        match self.metadata.signing_name {
            Some(ref signing_name) => signing_name.to_string(),
            None => self.metadata.endpoint_prefix.to_string()
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
    pub shape: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub documentation: Option<String>,
    #[serde(rename="resultWrapper")]
    pub result_wrapper: Option<String>,
    pub shape: String,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub documentation: Option<String>,
    pub error: Option<HttpError>,
    pub exception: Option<bool>,
    pub fault: Option<bool>,
    pub shape: String,
}

impl Error {
    pub fn idiomatic_error_name(&self) -> String {
        self.shape.replace("Exception","")
    }
}

#[derive(Debug, Deserialize)]
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
    pub shape: String,
    pub streaming: Option<bool>,
    #[serde(rename="xmlAttribute")]
    pub xml_attribute: Option<bool>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl Member {
    pub fn tag_name(&self) -> String {
        self.location_name.clone().unwrap_or(self.shape.clone())
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
    pub shape: String,
}

impl Key {
    pub fn tag_name(&self) -> String {
        self.location_name.clone().unwrap_or(self.shape.clone())
    }
}

#[derive(Debug, Deserialize)]
pub struct Value {
    pub documentation: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub shape: String,
}

impl Value {
    pub fn tag_name(&self) -> String {
        self.location_name.clone().unwrap_or(self.shape.clone())
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
    pub shape_type: String,
    pub sensitive: Option<bool>,
    #[serde(rename="timestampFormat")]
    pub timestamp_format: Option<String>,
    pub value: Option<Value>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl<'a> Shape {
    pub fn key(&'a self) -> &'a str {
        &self.key.as_ref().expect("Key shape undefined").shape
    }

    pub fn value(&'a self) -> &'a str {
        &self.value.as_ref().expect("Value shape undefined").shape
    }

    pub fn member(&'a self) -> &'a str {
        &self.member.as_ref().expect("Member shape undefined").shape
    }

    pub fn required(&self, field: &'a str) -> bool {
        self.required.is_some() && self.required.as_ref().unwrap().contains(&String::from(field))
    }

    pub fn exception(&self) -> bool {
        self.exception.unwrap_or(false)
    }
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
            None => default
        }
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
    pub xml_namespace: Option<String>
}
