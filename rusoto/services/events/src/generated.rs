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
pub type Boolean = bool;
#[doc="<p>Container for the parameters to the <a>DeleteRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteRuleRequest {
                #[doc="<p>The name of the rule to be deleted.</p>"]
#[serde(rename="Name")]
pub name: RuleName,
            }
            
#[doc="<p>Container for the parameters to the <a>DescribeRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeRuleRequest {
                #[doc="<p>The name of the rule you want to describe details for.</p>"]
#[serde(rename="Name")]
pub name: RuleName,
            }
            
#[doc="<p>The result of the <a>DescribeRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeRuleResponse {
                #[doc="<p>The Amazon Resource Name (ARN) associated with the rule.</p>"]
#[serde(rename="Arn")]
pub arn: Option<RuleArn>,
#[doc="<p>The rule's description.</p>"]
#[serde(rename="Description")]
pub description: Option<RuleDescription>,
#[doc="<p>The event pattern.</p>"]
#[serde(rename="EventPattern")]
pub event_pattern: Option<EventPattern>,
#[doc="<p>The rule's name.</p>"]
#[serde(rename="Name")]
pub name: Option<RuleName>,
#[doc="<p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>"]
#[serde(rename="RoleArn")]
pub role_arn: Option<RoleArn>,
#[doc="<p>The scheduling expression. For example, \"cron(0 20 * * ? *)\", \"rate(5 minutes)\".</p>"]
#[serde(rename="ScheduleExpression")]
pub schedule_expression: Option<ScheduleExpression>,
#[doc="<p>Specifies whether the rule is enabled or disabled.</p>"]
#[serde(rename="State")]
pub state: Option<RuleState>,
            }
            
#[doc="<p>Container for the parameters to the <a>DisableRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DisableRuleRequest {
                #[doc="<p>The name of the rule you want to disable.</p>"]
#[serde(rename="Name")]
pub name: RuleName,
            }
            
#[doc="<p>Container for the parameters to the <a>EnableRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct EnableRuleRequest {
                #[doc="<p>The name of the rule that you want to enable.</p>"]
#[serde(rename="Name")]
pub name: RuleName,
            }
            
pub type ErrorCode = String;
pub type ErrorMessage = String;
pub type EventId = String;
pub type EventPattern = String;
pub type EventResource = String;
pub type EventResourceList = Vec<EventResource>;
pub type EventTime = f64;
pub type Integer = i64;
pub type LimitMax100 = i64;
#[doc="<p>Container for the parameters to the <a>ListRuleNamesByTarget</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListRuleNamesByTargetRequest {
                #[doc="<p>The maximum number of results to return.</p>"]
#[serde(rename="Limit")]
pub limit: Option<LimitMax100>,
#[doc="<p>The token returned by a previous call to indicate that there is more data available.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The Amazon Resource Name (ARN) of the target resource that you want to list the rules for.</p>"]
#[serde(rename="TargetArn")]
pub target_arn: TargetArn,
            }
            
#[doc="<p>The result of the <a>ListRuleNamesByTarget</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListRuleNamesByTargetResponse {
                #[doc="<p>Indicates that there are additional results to retrieve.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>List of rules names that can invoke the given target.</p>"]
#[serde(rename="RuleNames")]
pub rule_names: Option<RuleNameList>,
            }
            
#[doc="<p>Container for the parameters to the <a>ListRules</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListRulesRequest {
                #[doc="<p>The maximum number of results to return.</p>"]
#[serde(rename="Limit")]
pub limit: Option<LimitMax100>,
#[doc="<p>The prefix matching the rule name.</p>"]
#[serde(rename="NamePrefix")]
pub name_prefix: Option<RuleName>,
#[doc="<p>The token returned by a previous call to indicate that there is more data available.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[doc="<p>The result of the <a>ListRules</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListRulesResponse {
                #[doc="<p>Indicates that there are additional results to retrieve.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>List of rules matching the specified criteria.</p>"]
#[serde(rename="Rules")]
pub rules: Option<RuleResponseList>,
            }
            
#[doc="<p>Container for the parameters to the <a>ListTargetsByRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListTargetsByRuleRequest {
                #[doc="<p>The maximum number of results to return.</p>"]
#[serde(rename="Limit")]
pub limit: Option<LimitMax100>,
#[doc="<p>The token returned by a previous call to indicate that there is more data available.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The name of the rule whose targets you want to list.</p>"]
#[serde(rename="Rule")]
pub rule: RuleName,
            }
            
#[doc="<p>The result of the <a>ListTargetsByRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListTargetsByRuleResponse {
                #[doc="<p>Indicates that there are additional results to retrieve.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Lists the targets assigned to the rule.</p>"]
#[serde(rename="Targets")]
pub targets: Option<TargetList>,
            }
            
pub type NextToken = String;
#[doc="<p>Container for the parameters to the <a>PutEvents</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutEventsRequest {
                #[doc="<p>The entry that defines an event in your system. You can specify several parameters for the entry such as the source and type of the event, resources associated with the event, and so on.</p>"]
#[serde(rename="Entries")]
pub entries: PutEventsRequestEntryList,
            }
            
#[doc="<p>Contains information about the event to be used in PutEvents.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutEventsRequestEntry {
                #[doc="<p>In the JSON sense, an object containing fields, which may also contain nested sub-objects. No constraints are imposed on its contents.</p>"]
#[serde(rename="Detail")]
pub detail: Option<String>,
#[doc="<p>Free-form string used to decide what fields to expect in the event detail.</p>"]
#[serde(rename="DetailType")]
pub detail_type: Option<String>,
#[doc="<p>AWS resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>"]
#[serde(rename="Resources")]
pub resources: Option<EventResourceList>,
#[doc="<p>The source of the event.</p>"]
#[serde(rename="Source")]
pub source: Option<String>,
#[doc="<p>Timestamp of event, per <a href=\"https://www.rfc-editor.org/rfc/rfc3339.txt\">RFC3339</a>. If no timestamp is provided, the timestamp of the <a>PutEvents</a> call will be used.</p>"]
#[serde(rename="Time")]
pub time: Option<EventTime>,
            }
            
pub type PutEventsRequestEntryList = Vec<PutEventsRequestEntry>;
#[doc="<p>The result of the <a>PutEvents</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutEventsResponse {
                #[doc="<p>A list of successfully and unsuccessfully ingested events results. If the ingestion was successful, the entry will have the event ID in it. If not, then the ErrorCode and ErrorMessage can be used to identify the problem with the entry.</p>"]
#[serde(rename="Entries")]
pub entries: Option<PutEventsResultEntryList>,
#[doc="<p>The number of failed entries.</p>"]
#[serde(rename="FailedEntryCount")]
pub failed_entry_count: Option<Integer>,
            }
            
#[doc="<p>A PutEventsResult contains a list of PutEventsResultEntry.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutEventsResultEntry {
                #[doc="<p>The error code representing why the event submission failed on this entry.</p>"]
#[serde(rename="ErrorCode")]
pub error_code: Option<ErrorCode>,
#[doc="<p>The error message explaining why the event submission failed on this entry.</p>"]
#[serde(rename="ErrorMessage")]
pub error_message: Option<ErrorMessage>,
#[doc="<p>The ID of the event submitted to Amazon CloudWatch Events.</p>"]
#[serde(rename="EventId")]
pub event_id: Option<EventId>,
            }
            
pub type PutEventsResultEntryList = Vec<PutEventsResultEntry>;
#[doc="<p>Container for the parameters to the <a>PutRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutRuleRequest {
                #[doc="<p>A description of the rule.</p>"]
#[serde(rename="Description")]
pub description: Option<RuleDescription>,
#[doc="<p>The event pattern.</p>"]
#[serde(rename="EventPattern")]
pub event_pattern: Option<EventPattern>,
#[doc="<p>The name of the rule that you are creating or updating.</p>"]
#[serde(rename="Name")]
pub name: RuleName,
#[doc="<p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>"]
#[serde(rename="RoleArn")]
pub role_arn: Option<RoleArn>,
#[doc="<p>The scheduling expression. For example, \"cron(0 20 * * ? *)\", \"rate(5 minutes)\".</p>"]
#[serde(rename="ScheduleExpression")]
pub schedule_expression: Option<ScheduleExpression>,
#[doc="<p>Indicates whether the rule is enabled or disabled.</p>"]
#[serde(rename="State")]
pub state: Option<RuleState>,
            }
            
#[doc="<p>The result of the <a>PutRule</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutRuleResponse {
                #[doc="<p>The Amazon Resource Name (ARN) that identifies the rule.</p>"]
#[serde(rename="RuleArn")]
pub rule_arn: Option<RuleArn>,
            }
            
#[doc="<p>Container for the parameters to the <a>PutTargets</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutTargetsRequest {
                #[doc="<p>The name of the rule you want to add targets to.</p>"]
#[serde(rename="Rule")]
pub rule: RuleName,
#[doc="<p>List of targets you want to update or add to the rule.</p>"]
#[serde(rename="Targets")]
pub targets: TargetList,
            }
            
#[doc="<p>The result of the <a>PutTargets</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutTargetsResponse {
                #[doc="<p>An array of failed target entries.</p>"]
#[serde(rename="FailedEntries")]
pub failed_entries: Option<PutTargetsResultEntryList>,
#[doc="<p>The number of failed entries.</p>"]
#[serde(rename="FailedEntryCount")]
pub failed_entry_count: Option<Integer>,
            }
            
#[doc="<p>A PutTargetsResult contains a list of PutTargetsResultEntry.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutTargetsResultEntry {
                #[doc="<p>The error code representing why the target submission failed on this entry.</p>"]
#[serde(rename="ErrorCode")]
pub error_code: Option<ErrorCode>,
#[doc="<p>The error message explaining why the target submission failed on this entry.</p>"]
#[serde(rename="ErrorMessage")]
pub error_message: Option<ErrorMessage>,
#[doc="<p>The ID of the target submitted to Amazon CloudWatch Events.</p>"]
#[serde(rename="TargetId")]
pub target_id: Option<TargetId>,
            }
            
pub type PutTargetsResultEntryList = Vec<PutTargetsResultEntry>;
#[doc="<p>Container for the parameters to the <a>RemoveTargets</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RemoveTargetsRequest {
                #[doc="<p>The list of target IDs to remove from the rule.</p>"]
#[serde(rename="Ids")]
pub ids: TargetIdList,
#[doc="<p>The name of the rule you want to remove targets from.</p>"]
#[serde(rename="Rule")]
pub rule: RuleName,
            }
            
#[doc="<p>The result of the <a>RemoveTargets</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RemoveTargetsResponse {
                #[doc="<p>An array of failed target entries.</p>"]
#[serde(rename="FailedEntries")]
pub failed_entries: Option<RemoveTargetsResultEntryList>,
#[doc="<p>The number of failed entries.</p>"]
#[serde(rename="FailedEntryCount")]
pub failed_entry_count: Option<Integer>,
            }
            
#[doc="<p>The ID of the target requested to be removed from the rule by Amazon CloudWatch Events.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RemoveTargetsResultEntry {
                #[doc="<p>The error code representing why the target removal failed on this entry.</p>"]
#[serde(rename="ErrorCode")]
pub error_code: Option<ErrorCode>,
#[doc="<p>The error message explaining why the target removal failed on this entry.</p>"]
#[serde(rename="ErrorMessage")]
pub error_message: Option<ErrorMessage>,
#[doc="<p>The ID of the target requested to be removed by Amazon CloudWatch Events.</p>"]
#[serde(rename="TargetId")]
pub target_id: Option<TargetId>,
            }
            
pub type RemoveTargetsResultEntryList = Vec<RemoveTargetsResultEntry>;
pub type RoleArn = String;
#[doc="<p>Contains information about a rule in Amazon CloudWatch Events. A ListRulesResult contains a list of Rules.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Rule {
                #[doc="<p>The Amazon Resource Name (ARN) of the rule.</p>"]
#[serde(rename="Arn")]
pub arn: Option<RuleArn>,
#[doc="<p>The description of the rule.</p>"]
#[serde(rename="Description")]
pub description: Option<RuleDescription>,
#[doc="<p>The event pattern of the rule.</p>"]
#[serde(rename="EventPattern")]
pub event_pattern: Option<EventPattern>,
#[doc="<p>The rule's name.</p>"]
#[serde(rename="Name")]
pub name: Option<RuleName>,
#[doc="<p>The Amazon Resource Name (ARN) associated with the role that is used for target invocation.</p>"]
#[serde(rename="RoleArn")]
pub role_arn: Option<RoleArn>,
#[doc="<p>The scheduling expression. For example, \"cron(0 20 * * ? *)\", \"rate(5 minutes)\".</p>"]
#[serde(rename="ScheduleExpression")]
pub schedule_expression: Option<ScheduleExpression>,
#[doc="<p>The rule's state.</p>"]
#[serde(rename="State")]
pub state: Option<RuleState>,
            }
            
pub type RuleArn = String;
pub type RuleDescription = String;
pub type RuleName = String;
pub type RuleNameList = Vec<RuleName>;
pub type RuleResponseList = Vec<Rule>;
pub type RuleState = String;
pub type ScheduleExpression = String;
#[doc="<p>Targets are the resources that can be invoked when a rule is triggered. For example, AWS Lambda functions, Amazon Kinesis streams, and built-in targets.</p> <p><b>Input</b> and <b>InputPath</b> are mutually-exclusive and optional parameters of a target. When a rule is triggered due to a matched event, if for a target:</p> <ul> <li>Neither <b>Input</b> nor <b>InputPath</b> is specified, then the entire event is passed to the target in JSON form.</li> <li> <b>InputPath</b> is specified in the form of JSONPath (e.g. <b>$.detail</b>), then only the part of the event specified in the path is passed to the target (e.g. only the detail part of the event is passed). </li> <li> <b>Input</b> is specified in the form of a valid JSON, then the matched event is overridden with this constant.</li> </ul>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Target {
                #[doc="<p>The Amazon Resource Name (ARN) associated of the target.</p>"]
#[serde(rename="Arn")]
pub arn: TargetArn,
#[doc="<p>The unique target assignment ID.</p>"]
#[serde(rename="Id")]
pub id: TargetId,
#[doc="<p>Valid JSON text passed to the target. For more information about JSON text, see <a href=\"http://www.rfc-editor.org/rfc/rfc7159.txt\">The JavaScript Object Notation (JSON) Data Interchange Format</a>.</p>"]
#[serde(rename="Input")]
pub input: Option<TargetInput>,
#[doc="<p>The value of the JSONPath that is used for extracting part of the matched event when passing it to the target. For more information about JSON paths, see <a href=\"http://goessner.net/articles/JsonPath/\">JSONPath</a>.</p>"]
#[serde(rename="InputPath")]
pub input_path: Option<TargetInputPath>,
            }
            
pub type TargetArn = String;
pub type TargetId = String;
pub type TargetIdList = Vec<TargetId>;
pub type TargetInput = String;
pub type TargetInputPath = String;
pub type TargetList = Vec<Target>;
#[doc="<p>Container for the parameters to the <a>TestEventPattern</a> operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct TestEventPatternRequest {
                #[doc="<p>The event in the JSON format to test against the event pattern.</p>"]
#[serde(rename="Event")]
pub event: String,
#[doc="<p>The event pattern you want to test.</p>"]
#[serde(rename="EventPattern")]
pub event_pattern: EventPattern,
            }
            
#[doc="<p>The result of the <a>TestEventPattern</a> operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct TestEventPatternResponse {
                #[doc="<p>Indicates whether the event matches the event pattern.</p>"]
#[serde(rename="Result")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub result: Option<Boolean>,
            }
            
/// Errors returned by DeleteRule
                #[derive(Debug, PartialEq)]
                pub enum DeleteRuleError {
                    
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),
///<p>This exception occurs if there is concurrent modification on rule or target.</p>
ConcurrentModification(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteRuleError {
                    pub fn from_body(body: &str) -> DeleteRuleError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ConcurrentModificationException" => DeleteRuleError::ConcurrentModification(String::from(error_message)),"InternalException" => DeleteRuleError::Internal(String::from(error_message)),"ValidationException" => DeleteRuleError::Validation(error_message.to_string()),_ => DeleteRuleError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteRuleError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteRuleError {
                    fn from(err: serde_json::error::Error) -> DeleteRuleError {
                        DeleteRuleError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteRuleError {
                    fn from(err: CredentialsError) -> DeleteRuleError {
                        DeleteRuleError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteRuleError {
                    fn from(err: HttpDispatchError) -> DeleteRuleError {
                        DeleteRuleError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteRuleError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteRuleError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteRuleError::Internal(ref cause) => cause,DeleteRuleError::ConcurrentModification(ref cause) => cause,DeleteRuleError::Validation(ref cause) => cause,DeleteRuleError::Credentials(ref err) => err.description(),DeleteRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteRuleError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeRule
                #[derive(Debug, PartialEq)]
                pub enum DescribeRuleError {
                    
///<p>The rule does not exist.</p>
ResourceNotFound(String),
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeRuleError {
                    pub fn from_body(body: &str) -> DescribeRuleError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalException" => DescribeRuleError::Internal(String::from(error_message)),"ResourceNotFoundException" => DescribeRuleError::ResourceNotFound(String::from(error_message)),"ValidationException" => DescribeRuleError::Validation(error_message.to_string()),_ => DescribeRuleError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeRuleError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeRuleError {
                    fn from(err: serde_json::error::Error) -> DescribeRuleError {
                        DescribeRuleError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeRuleError {
                    fn from(err: CredentialsError) -> DescribeRuleError {
                        DescribeRuleError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeRuleError {
                    fn from(err: HttpDispatchError) -> DescribeRuleError {
                        DescribeRuleError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeRuleError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeRuleError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeRuleError::ResourceNotFound(ref cause) => cause,DescribeRuleError::Internal(ref cause) => cause,DescribeRuleError::Validation(ref cause) => cause,DescribeRuleError::Credentials(ref err) => err.description(),DescribeRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeRuleError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DisableRule
                #[derive(Debug, PartialEq)]
                pub enum DisableRuleError {
                    
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),
///<p>The rule does not exist.</p>
ResourceNotFound(String),
///<p>This exception occurs if there is concurrent modification on rule or target.</p>
ConcurrentModification(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DisableRuleError {
                    pub fn from_body(body: &str) -> DisableRuleError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceNotFoundException" => DisableRuleError::ResourceNotFound(String::from(error_message)),"ConcurrentModificationException" => DisableRuleError::ConcurrentModification(String::from(error_message)),"InternalException" => DisableRuleError::Internal(String::from(error_message)),"ValidationException" => DisableRuleError::Validation(error_message.to_string()),_ => DisableRuleError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DisableRuleError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DisableRuleError {
                    fn from(err: serde_json::error::Error) -> DisableRuleError {
                        DisableRuleError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DisableRuleError {
                    fn from(err: CredentialsError) -> DisableRuleError {
                        DisableRuleError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DisableRuleError {
                    fn from(err: HttpDispatchError) -> DisableRuleError {
                        DisableRuleError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DisableRuleError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DisableRuleError {
                    fn description(&self) -> &str {
                        match *self {
                            DisableRuleError::ConcurrentModification(ref cause) => cause,DisableRuleError::Internal(ref cause) => cause,DisableRuleError::ResourceNotFound(ref cause) => cause,DisableRuleError::Validation(ref cause) => cause,DisableRuleError::Credentials(ref err) => err.description(),DisableRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DisableRuleError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by EnableRule
                #[derive(Debug, PartialEq)]
                pub enum EnableRuleError {
                    
///<p>This exception occurs if there is concurrent modification on rule or target.</p>
ConcurrentModification(String),
///<p>The rule does not exist.</p>
ResourceNotFound(String),
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl EnableRuleError {
                    pub fn from_body(body: &str) -> EnableRuleError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalException" => EnableRuleError::Internal(String::from(error_message)),"ConcurrentModificationException" => EnableRuleError::ConcurrentModification(String::from(error_message)),"ResourceNotFoundException" => EnableRuleError::ResourceNotFound(String::from(error_message)),"ValidationException" => EnableRuleError::Validation(error_message.to_string()),_ => EnableRuleError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => EnableRuleError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for EnableRuleError {
                    fn from(err: serde_json::error::Error) -> EnableRuleError {
                        EnableRuleError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for EnableRuleError {
                    fn from(err: CredentialsError) -> EnableRuleError {
                        EnableRuleError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for EnableRuleError {
                    fn from(err: HttpDispatchError) -> EnableRuleError {
                        EnableRuleError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for EnableRuleError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for EnableRuleError {
                    fn description(&self) -> &str {
                        match *self {
                            EnableRuleError::ResourceNotFound(ref cause) => cause,EnableRuleError::Internal(ref cause) => cause,EnableRuleError::ConcurrentModification(ref cause) => cause,EnableRuleError::Validation(ref cause) => cause,EnableRuleError::Credentials(ref err) => err.description(),EnableRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),EnableRuleError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListRuleNamesByTarget
                #[derive(Debug, PartialEq)]
                pub enum ListRuleNamesByTargetError {
                    
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListRuleNamesByTargetError {
                    pub fn from_body(body: &str) -> ListRuleNamesByTargetError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalException" => ListRuleNamesByTargetError::Internal(String::from(error_message)),"ValidationException" => ListRuleNamesByTargetError::Validation(error_message.to_string()),_ => ListRuleNamesByTargetError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListRuleNamesByTargetError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListRuleNamesByTargetError {
                    fn from(err: serde_json::error::Error) -> ListRuleNamesByTargetError {
                        ListRuleNamesByTargetError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListRuleNamesByTargetError {
                    fn from(err: CredentialsError) -> ListRuleNamesByTargetError {
                        ListRuleNamesByTargetError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListRuleNamesByTargetError {
                    fn from(err: HttpDispatchError) -> ListRuleNamesByTargetError {
                        ListRuleNamesByTargetError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListRuleNamesByTargetError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListRuleNamesByTargetError {
                    fn description(&self) -> &str {
                        match *self {
                            ListRuleNamesByTargetError::Internal(ref cause) => cause,ListRuleNamesByTargetError::Validation(ref cause) => cause,ListRuleNamesByTargetError::Credentials(ref err) => err.description(),ListRuleNamesByTargetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListRuleNamesByTargetError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListRules
                #[derive(Debug, PartialEq)]
                pub enum ListRulesError {
                    
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListRulesError {
                    pub fn from_body(body: &str) -> ListRulesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalException" => ListRulesError::Internal(String::from(error_message)),"ValidationException" => ListRulesError::Validation(error_message.to_string()),_ => ListRulesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListRulesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListRulesError {
                    fn from(err: serde_json::error::Error) -> ListRulesError {
                        ListRulesError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListRulesError {
                    fn from(err: CredentialsError) -> ListRulesError {
                        ListRulesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListRulesError {
                    fn from(err: HttpDispatchError) -> ListRulesError {
                        ListRulesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListRulesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListRulesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListRulesError::Internal(ref cause) => cause,ListRulesError::Validation(ref cause) => cause,ListRulesError::Credentials(ref err) => err.description(),ListRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListRulesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTargetsByRule
                #[derive(Debug, PartialEq)]
                pub enum ListTargetsByRuleError {
                    
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),
///<p>The rule does not exist.</p>
ResourceNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTargetsByRuleError {
                    pub fn from_body(body: &str) -> ListTargetsByRuleError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceNotFoundException" => ListTargetsByRuleError::ResourceNotFound(String::from(error_message)),"InternalException" => ListTargetsByRuleError::Internal(String::from(error_message)),"ValidationException" => ListTargetsByRuleError::Validation(error_message.to_string()),_ => ListTargetsByRuleError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListTargetsByRuleError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListTargetsByRuleError {
                    fn from(err: serde_json::error::Error) -> ListTargetsByRuleError {
                        ListTargetsByRuleError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListTargetsByRuleError {
                    fn from(err: CredentialsError) -> ListTargetsByRuleError {
                        ListTargetsByRuleError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTargetsByRuleError {
                    fn from(err: HttpDispatchError) -> ListTargetsByRuleError {
                        ListTargetsByRuleError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTargetsByRuleError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTargetsByRuleError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTargetsByRuleError::Internal(ref cause) => cause,ListTargetsByRuleError::ResourceNotFound(ref cause) => cause,ListTargetsByRuleError::Validation(ref cause) => cause,ListTargetsByRuleError::Credentials(ref err) => err.description(),ListTargetsByRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTargetsByRuleError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutEvents
                #[derive(Debug, PartialEq)]
                pub enum PutEventsError {
                    
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutEventsError {
                    pub fn from_body(body: &str) -> PutEventsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalException" => PutEventsError::Internal(String::from(error_message)),"ValidationException" => PutEventsError::Validation(error_message.to_string()),_ => PutEventsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => PutEventsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for PutEventsError {
                    fn from(err: serde_json::error::Error) -> PutEventsError {
                        PutEventsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for PutEventsError {
                    fn from(err: CredentialsError) -> PutEventsError {
                        PutEventsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for PutEventsError {
                    fn from(err: HttpDispatchError) -> PutEventsError {
                        PutEventsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for PutEventsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutEventsError {
                    fn description(&self) -> &str {
                        match *self {
                            PutEventsError::Internal(ref cause) => cause,PutEventsError::Validation(ref cause) => cause,PutEventsError::Credentials(ref err) => err.description(),PutEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),PutEventsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutRule
                #[derive(Debug, PartialEq)]
                pub enum PutRuleError {
                    
///<p>This exception occurs if you try to create more rules or add more targets to a rule than allowed by default.</p>
LimitExceeded(String),
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),
///<p>The event pattern is invalid.</p>
InvalidEventPattern(String),
///<p>This exception occurs if there is concurrent modification on rule or target.</p>
ConcurrentModification(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutRuleError {
                    pub fn from_body(body: &str) -> PutRuleError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidEventPatternException" => PutRuleError::InvalidEventPattern(String::from(error_message)),"LimitExceededException" => PutRuleError::LimitExceeded(String::from(error_message)),"ConcurrentModificationException" => PutRuleError::ConcurrentModification(String::from(error_message)),"InternalException" => PutRuleError::Internal(String::from(error_message)),"ValidationException" => PutRuleError::Validation(error_message.to_string()),_ => PutRuleError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => PutRuleError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for PutRuleError {
                    fn from(err: serde_json::error::Error) -> PutRuleError {
                        PutRuleError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for PutRuleError {
                    fn from(err: CredentialsError) -> PutRuleError {
                        PutRuleError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for PutRuleError {
                    fn from(err: HttpDispatchError) -> PutRuleError {
                        PutRuleError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for PutRuleError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutRuleError {
                    fn description(&self) -> &str {
                        match *self {
                            PutRuleError::LimitExceeded(ref cause) => cause,PutRuleError::InvalidEventPattern(ref cause) => cause,PutRuleError::ConcurrentModification(ref cause) => cause,PutRuleError::Internal(ref cause) => cause,PutRuleError::Validation(ref cause) => cause,PutRuleError::Credentials(ref err) => err.description(),PutRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),PutRuleError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutTargets
                #[derive(Debug, PartialEq)]
                pub enum PutTargetsError {
                    
///<p>This exception occurs if there is concurrent modification on rule or target.</p>
ConcurrentModification(String),
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),
///<p>The rule does not exist.</p>
ResourceNotFound(String),
///<p>This exception occurs if you try to create more rules or add more targets to a rule than allowed by default.</p>
LimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutTargetsError {
                    pub fn from_body(body: &str) -> PutTargetsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "LimitExceededException" => PutTargetsError::LimitExceeded(String::from(error_message)),"InternalException" => PutTargetsError::Internal(String::from(error_message)),"ResourceNotFoundException" => PutTargetsError::ResourceNotFound(String::from(error_message)),"ConcurrentModificationException" => PutTargetsError::ConcurrentModification(String::from(error_message)),"ValidationException" => PutTargetsError::Validation(error_message.to_string()),_ => PutTargetsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => PutTargetsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for PutTargetsError {
                    fn from(err: serde_json::error::Error) -> PutTargetsError {
                        PutTargetsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for PutTargetsError {
                    fn from(err: CredentialsError) -> PutTargetsError {
                        PutTargetsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for PutTargetsError {
                    fn from(err: HttpDispatchError) -> PutTargetsError {
                        PutTargetsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for PutTargetsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutTargetsError {
                    fn description(&self) -> &str {
                        match *self {
                            PutTargetsError::LimitExceeded(ref cause) => cause,PutTargetsError::ConcurrentModification(ref cause) => cause,PutTargetsError::Internal(ref cause) => cause,PutTargetsError::ResourceNotFound(ref cause) => cause,PutTargetsError::Validation(ref cause) => cause,PutTargetsError::Credentials(ref err) => err.description(),PutTargetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),PutTargetsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RemoveTargets
                #[derive(Debug, PartialEq)]
                pub enum RemoveTargetsError {
                    
///<p>The rule does not exist.</p>
ResourceNotFound(String),
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),
///<p>This exception occurs if there is concurrent modification on rule or target.</p>
ConcurrentModification(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RemoveTargetsError {
                    pub fn from_body(body: &str) -> RemoveTargetsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ConcurrentModificationException" => RemoveTargetsError::ConcurrentModification(String::from(error_message)),"ResourceNotFoundException" => RemoveTargetsError::ResourceNotFound(String::from(error_message)),"InternalException" => RemoveTargetsError::Internal(String::from(error_message)),"ValidationException" => RemoveTargetsError::Validation(error_message.to_string()),_ => RemoveTargetsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RemoveTargetsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RemoveTargetsError {
                    fn from(err: serde_json::error::Error) -> RemoveTargetsError {
                        RemoveTargetsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RemoveTargetsError {
                    fn from(err: CredentialsError) -> RemoveTargetsError {
                        RemoveTargetsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RemoveTargetsError {
                    fn from(err: HttpDispatchError) -> RemoveTargetsError {
                        RemoveTargetsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RemoveTargetsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RemoveTargetsError {
                    fn description(&self) -> &str {
                        match *self {
                            RemoveTargetsError::ResourceNotFound(ref cause) => cause,RemoveTargetsError::ConcurrentModification(ref cause) => cause,RemoveTargetsError::Internal(ref cause) => cause,RemoveTargetsError::Validation(ref cause) => cause,RemoveTargetsError::Credentials(ref err) => err.description(),RemoveTargetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RemoveTargetsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by TestEventPattern
                #[derive(Debug, PartialEq)]
                pub enum TestEventPatternError {
                    
///<p>This exception occurs due to unexpected causes.</p>
Internal(String),
///<p>The event pattern is invalid.</p>
InvalidEventPattern(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl TestEventPatternError {
                    pub fn from_body(body: &str) -> TestEventPatternError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidEventPatternException" => TestEventPatternError::InvalidEventPattern(String::from(error_message)),"InternalException" => TestEventPatternError::Internal(String::from(error_message)),"ValidationException" => TestEventPatternError::Validation(error_message.to_string()),_ => TestEventPatternError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => TestEventPatternError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for TestEventPatternError {
                    fn from(err: serde_json::error::Error) -> TestEventPatternError {
                        TestEventPatternError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for TestEventPatternError {
                    fn from(err: CredentialsError) -> TestEventPatternError {
                        TestEventPatternError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for TestEventPatternError {
                    fn from(err: HttpDispatchError) -> TestEventPatternError {
                        TestEventPatternError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for TestEventPatternError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for TestEventPatternError {
                    fn description(&self) -> &str {
                        match *self {
                            TestEventPatternError::Internal(ref cause) => cause,TestEventPatternError::InvalidEventPattern(ref cause) => cause,TestEventPatternError::Validation(ref cause) => cause,TestEventPatternError::Credentials(ref err) => err.description(),TestEventPatternError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),TestEventPatternError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Trait representing the capabilities of the Amazon CloudWatch Events API. Amazon CloudWatch Events clients implement this trait.
        pub trait CloudWatchEvents {
        

                #[doc="<p>Deletes a rule. You must remove all targets from a rule using <a>RemoveTargets</a> before you can delete the rule.</p> <p> <b>Note:</b> When you delete a rule, incoming events might still continue to match to the deleted rule. Please allow a short period of time for changes to take effect. </p>"]
                fn delete_rule(&self, input: &DeleteRuleRequest)  -> Result<(), DeleteRuleError>;
                

                #[doc="<p>Describes the details of the specified rule.</p>"]
                fn describe_rule(&self, input: &DescribeRuleRequest)  -> Result<DescribeRuleResponse, DescribeRuleError>;
                

                #[doc="<p>Disables a rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p> <b>Note:</b> When you disable a rule, incoming events might still continue to match to the disabled rule. Please allow a short period of time for changes to take effect. </p>"]
                fn disable_rule(&self, input: &DisableRuleRequest)  -> Result<(), DisableRuleError>;
                

                #[doc="<p>Enables a rule. If the rule does not exist, the operation fails.</p> <p> <b>Note:</b> When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Please allow a short period of time for changes to take effect. </p>"]
                fn enable_rule(&self, input: &EnableRuleRequest)  -> Result<(), EnableRuleError>;
                

                #[doc="<p>Lists the names of the rules that the given target is put to. You can see which of the rules in Amazon CloudWatch Events can invoke a specific target in your account. If you have more rules in your account than the given limit, the results will be paginated. In that case, use the next token returned in the response and repeat ListRulesByTarget until the NextToken in the response is returned as null.</p>"]
                fn list_rule_names_by_target(&self, input: &ListRuleNamesByTargetRequest)  -> Result<ListRuleNamesByTargetResponse, ListRuleNamesByTargetError>;
                

                #[doc="<p>Lists the Amazon CloudWatch Events rules in your account. You can either list all the rules or you can provide a prefix to match to the rule names. If you have more rules in your account than the given limit, the results will be paginated. In that case, use the next token returned in the response and repeat ListRules until the NextToken in the response is returned as null.</p>"]
                fn list_rules(&self, input: &ListRulesRequest)  -> Result<ListRulesResponse, ListRulesError>;
                

                #[doc="<p>Lists of targets assigned to the rule.</p>"]
                fn list_targets_by_rule(&self, input: &ListTargetsByRuleRequest)  -> Result<ListTargetsByRuleResponse, ListTargetsByRuleError>;
                

                #[doc="<p>Sends custom events to Amazon CloudWatch Events so that they can be matched to rules.</p>"]
                fn put_events(&self, input: &PutEventsRequest)  -> Result<PutEventsResponse, PutEventsError>;
                

                #[doc="<p>Creates or updates a rule. Rules are enabled by default, or based on value of the State parameter. You can disable a rule using <a>DisableRule</a>.</p> <p> <b>Note:</b> When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Please allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule will trigger on matching events as well as on a schedule.</p> <p> <b>Note:</b> Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match. </p>"]
                fn put_rule(&self, input: &PutRuleRequest)  -> Result<PutRuleResponse, PutRuleError>;
                

                #[doc="<p>Adds target(s) to a rule. Targets are the resources that can be invoked when a rule is triggered. For example, AWS Lambda functions, Amazon Kinesis streams, and built-in targets. Updates the target(s) if they are already associated with the role. In other words, if there is already a target with the given target ID, then the target associated with that ID is updated.</p> <p>In order to be able to make API calls against the resources you own, Amazon CloudWatch Events needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, CloudWatch Events relies on resource-based policies. For Amazon Kinesis streams, CloudWatch Events relies on IAM roles. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/EventsTargetPermissions.html\">Permissions for Sending Events to Targets</a> in the <b><i>Amazon CloudWatch Developer Guide</i></b>.</p> <p><b>Input</b> and <b>InputPath</b> are mutually-exclusive and optional parameters of a target. When a rule is triggered due to a matched event, if for a target:</p> <ul> <li>Neither <b>Input</b> nor <b>InputPath</b> is specified, then the entire event is passed to the target in JSON form.</li> <li> <b>InputPath</b> is specified in the form of JSONPath (e.g. <b>$.detail</b>), then only the part of the event specified in the path is passed to the target (e.g. only the detail part of the event is passed). </li> <li> <b>Input</b> is specified in the form of a valid JSON, then the matched event is overridden with this constant.</li> </ul> <p> <b>Note:</b> When you add targets to a rule, when the associated rule triggers, new or updated targets might not be immediately invoked. Please allow a short period of time for changes to take effect. </p>"]
                fn put_targets(&self, input: &PutTargetsRequest)  -> Result<PutTargetsResponse, PutTargetsError>;
                

                #[doc="<p>Removes target(s) from a rule so that when the rule is triggered, those targets will no longer be invoked.</p> <p> <b>Note:</b> When you remove a target, when the associated rule triggers, removed targets might still continue to be invoked. Please allow a short period of time for changes to take effect. </p>"]
                fn remove_targets(&self, input: &RemoveTargetsRequest)  -> Result<RemoveTargetsResponse, RemoveTargetsError>;
                

                #[doc="<p>Tests whether an event pattern matches the provided event.</p> <p> <b>Note:</b> Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match. </p>"]
                fn test_event_pattern(&self, input: &TestEventPatternRequest)  -> Result<TestEventPatternResponse, TestEventPatternError>;
                
}
/// A client for the Amazon CloudWatch Events API.
        pub struct CloudWatchEventsClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> CloudWatchEventsClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  CloudWatchEventsClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }
        }

        impl<P, D> CloudWatchEvents for CloudWatchEventsClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
        

                #[doc="<p>Deletes a rule. You must remove all targets from a rule using <a>RemoveTargets</a> before you can delete the rule.</p> <p> <b>Note:</b> When you delete a rule, incoming events might still continue to match to the deleted rule. Please allow a short period of time for changes to take effect. </p>"]
                fn delete_rule(&self, input: &DeleteRuleRequest)  -> Result<(), DeleteRuleError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.DeleteRule");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(DeleteRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Describes the details of the specified rule.</p>"]
                fn describe_rule(&self, input: &DescribeRuleRequest)  -> Result<DescribeRuleResponse, DescribeRuleError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.DescribeRule");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Disables a rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p> <b>Note:</b> When you disable a rule, incoming events might still continue to match to the disabled rule. Please allow a short period of time for changes to take effect. </p>"]
                fn disable_rule(&self, input: &DisableRuleRequest)  -> Result<(), DisableRuleError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.DisableRule");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(DisableRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Enables a rule. If the rule does not exist, the operation fails.</p> <p> <b>Note:</b> When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Please allow a short period of time for changes to take effect. </p>"]
                fn enable_rule(&self, input: &EnableRuleRequest)  -> Result<(), EnableRuleError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.EnableRule");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(EnableRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the names of the rules that the given target is put to. You can see which of the rules in Amazon CloudWatch Events can invoke a specific target in your account. If you have more rules in your account than the given limit, the results will be paginated. In that case, use the next token returned in the response and repeat ListRulesByTarget until the NextToken in the response is returned as null.</p>"]
                fn list_rule_names_by_target(&self, input: &ListRuleNamesByTargetRequest)  -> Result<ListRuleNamesByTargetResponse, ListRuleNamesByTargetError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.ListRuleNamesByTarget");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListRuleNamesByTargetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListRuleNamesByTargetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the Amazon CloudWatch Events rules in your account. You can either list all the rules or you can provide a prefix to match to the rule names. If you have more rules in your account than the given limit, the results will be paginated. In that case, use the next token returned in the response and repeat ListRules until the NextToken in the response is returned as null.</p>"]
                fn list_rules(&self, input: &ListRulesRequest)  -> Result<ListRulesResponse, ListRulesError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.ListRules");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListRulesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListRulesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists of targets assigned to the rule.</p>"]
                fn list_targets_by_rule(&self, input: &ListTargetsByRuleRequest)  -> Result<ListTargetsByRuleResponse, ListTargetsByRuleError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.ListTargetsByRule");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTargetsByRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListTargetsByRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Sends custom events to Amazon CloudWatch Events so that they can be matched to rules.</p>"]
                fn put_events(&self, input: &PutEventsRequest)  -> Result<PutEventsResponse, PutEventsError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.PutEvents");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutEventsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(PutEventsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates or updates a rule. Rules are enabled by default, or based on value of the State parameter. You can disable a rule using <a>DisableRule</a>.</p> <p> <b>Note:</b> When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Please allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule will trigger on matching events as well as on a schedule.</p> <p> <b>Note:</b> Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match. </p>"]
                fn put_rule(&self, input: &PutRuleRequest)  -> Result<PutRuleResponse, PutRuleError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.PutRule");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(PutRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Adds target(s) to a rule. Targets are the resources that can be invoked when a rule is triggered. For example, AWS Lambda functions, Amazon Kinesis streams, and built-in targets. Updates the target(s) if they are already associated with the role. In other words, if there is already a target with the given target ID, then the target associated with that ID is updated.</p> <p>In order to be able to make API calls against the resources you own, Amazon CloudWatch Events needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, CloudWatch Events relies on resource-based policies. For Amazon Kinesis streams, CloudWatch Events relies on IAM roles. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/EventsTargetPermissions.html\">Permissions for Sending Events to Targets</a> in the <b><i>Amazon CloudWatch Developer Guide</i></b>.</p> <p><b>Input</b> and <b>InputPath</b> are mutually-exclusive and optional parameters of a target. When a rule is triggered due to a matched event, if for a target:</p> <ul> <li>Neither <b>Input</b> nor <b>InputPath</b> is specified, then the entire event is passed to the target in JSON form.</li> <li> <b>InputPath</b> is specified in the form of JSONPath (e.g. <b>$.detail</b>), then only the part of the event specified in the path is passed to the target (e.g. only the detail part of the event is passed). </li> <li> <b>Input</b> is specified in the form of a valid JSON, then the matched event is overridden with this constant.</li> </ul> <p> <b>Note:</b> When you add targets to a rule, when the associated rule triggers, new or updated targets might not be immediately invoked. Please allow a short period of time for changes to take effect. </p>"]
                fn put_targets(&self, input: &PutTargetsRequest)  -> Result<PutTargetsResponse, PutTargetsError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.PutTargets");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutTargetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(PutTargetsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes target(s) from a rule so that when the rule is triggered, those targets will no longer be invoked.</p> <p> <b>Note:</b> When you remove a target, when the associated rule triggers, removed targets might still continue to be invoked. Please allow a short period of time for changes to take effect. </p>"]
                fn remove_targets(&self, input: &RemoveTargetsRequest)  -> Result<RemoveTargetsResponse, RemoveTargetsError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.RemoveTargets");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveTargetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RemoveTargetsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Tests whether an event pattern matches the provided event.</p> <p> <b>Note:</b> Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match. </p>"]
                fn test_event_pattern(&self, input: &TestEventPatternRequest)  -> Result<TestEventPatternResponse, TestEventPatternError> {
                    let mut request = SignedRequest::new("POST", "events", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AWSEvents.TestEventPattern");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<TestEventPatternResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(TestEventPatternError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                
}

            #[cfg(test)]
            mod protocol_tests {
                
            }
            
