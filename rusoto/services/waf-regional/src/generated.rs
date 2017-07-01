
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
pub type Action = String;
#[doc="<p>The <code>ActivatedRule</code> object in an <a>UpdateWebACL</a> request specifies a <code>Rule</code> that you want to insert or delete, the priority of the <code>Rule</code> in the <code>WebACL</code>, and the action that you want AWS WAF to take when a web request matches the <code>Rule</code> (<code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>).</p> <p>To specify whether to insert or delete a <code>Rule</code>, use the <code>Action</code> parameter in the <a>WebACLUpdate</a> data type.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ActivatedRule {
    #[doc="<p>Specifies the action that CloudFront or AWS WAF takes when a web request matches the conditions in the <code>Rule</code>. Valid values for <code>Action</code> include the following:</p> <ul> <li> <p> <code>ALLOW</code>: CloudFront responds with the requested object.</p> </li> <li> <p> <code>BLOCK</code>: CloudFront responds with an HTTP 403 (Forbidden) status code.</p> </li> <li> <p> <code>COUNT</code>: AWS WAF increments a counter of requests that match the conditions in the rule and then continues to inspect the web request based on the remaining rules in the web ACL. </p> </li> </ul>"]
    #[serde(rename="Action")]
    pub action: WafAction,
    #[doc="<p>Specifies the order in which the <code>Rules</code> in a <code>WebACL</code> are evaluated. Rules with a lower value for <code>Priority</code> are evaluated before <code>Rules</code> with a higher value. The value must be a unique integer. If you add multiple <code>Rules</code> to a <code>WebACL</code>, the values don't need to be consecutive.</p>"]
    #[serde(rename="Priority")]
    pub priority: RulePriority,
    #[doc="<p>The <code>RuleId</code> for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete a one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
    #[doc="<p>The rule type, either <code>REGULAR</code>, as defined by <a>Rule</a>, or <code>RATE_BASED</code>, as defined by <a>RateBasedRule</a>. The default is REGULAR. Although this field is optional, be aware that if you try to add a RATE_BASED rule to a web ACL without setting the type, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule with the specified ID, which does not exist. </p>"]
    #[serde(rename="Type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_: Option<WafRuleType>,
}

pub type ActivatedRules = Vec<ActivatedRule>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct AssociateWebACLRequest {
    #[doc="<p>The ARN (Amazon Resource Name) of the resource to be protected.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: ResourceArn,
    #[doc="<p>A unique identifier (ID) for the web ACL.</p>"]
    #[serde(rename="WebACLId")]
    pub web_acl_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct AssociateWebACLResponse;

#[doc="<p>In a <a>GetByteMatchSet</a> request, <code>ByteMatchSet</code> is a complex type that contains the <code>ByteMatchSetId</code> and <code>Name</code> of a <code>ByteMatchSet</code>, and the values that you specified when you updated the <code>ByteMatchSet</code>. </p> <p>A complex type that contains <code>ByteMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect and the values that you want AWS WAF to search for. If a <code>ByteMatchSet</code> contains more than one <code>ByteMatchTuple</code> object, a request needs to match the settings in only one <code>ByteMatchTuple</code> to be considered a match.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ByteMatchSet {
    #[doc="<p>The <code>ByteMatchSetId</code> for a <code>ByteMatchSet</code>. You use <code>ByteMatchSetId</code> to get information about a <code>ByteMatchSet</code> (see <a>GetByteMatchSet</a>), update a <code>ByteMatchSet</code> (see <a>UpdateByteMatchSet</a>), insert a <code>ByteMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>ByteMatchSet</code> from AWS WAF (see <a>DeleteByteMatchSet</a>).</p> <p> <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>"]
    #[serde(rename="ByteMatchSetId")]
    pub byte_match_set_id: ResourceId,
    #[doc="<p>Specifies the bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings.</p>"]
    #[serde(rename="ByteMatchTuples")]
    pub byte_match_tuples: ByteMatchTuples,
    #[doc="<p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ResourceName>,
}

pub type ByteMatchSetSummaries = Vec<ByteMatchSetSummary>;
#[doc="<p>Returned by <a>ListByteMatchSets</a>. Each <code>ByteMatchSetSummary</code> object includes the <code>Name</code> and <code>ByteMatchSetId</code> for one <a>ByteMatchSet</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ByteMatchSetSummary {
    #[doc="<p>The <code>ByteMatchSetId</code> for a <code>ByteMatchSet</code>. You use <code>ByteMatchSetId</code> to get information about a <code>ByteMatchSet</code>, update a <code>ByteMatchSet</code>, remove a <code>ByteMatchSet</code> from a <code>Rule</code>, and delete a <code>ByteMatchSet</code> from AWS WAF.</p> <p> <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>"]
    #[serde(rename="ByteMatchSetId")]
    pub byte_match_set_id: ResourceId,
    #[doc="<p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[doc="<p>In an <a>UpdateByteMatchSet</a> request, <code>ByteMatchSetUpdate</code> specifies whether to insert or delete a <a>ByteMatchTuple</a> and includes the settings for the <code>ByteMatchTuple</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ByteMatchSetUpdate {
    #[doc="<p>Specifies whether to insert or delete a <a>ByteMatchTuple</a>.</p>"]
    #[serde(rename="Action")]
    pub action: ChangeAction,
    #[doc="<p>Information about the part of a web request that you want AWS WAF to inspect and the value that you want AWS WAF to search for. If you specify <code>DELETE</code> for the value of <code>Action</code>, the <code>ByteMatchTuple</code> values must exactly match the values in the <code>ByteMatchTuple</code> that you want to delete from the <code>ByteMatchSet</code>.</p>"]
    #[serde(rename="ByteMatchTuple")]
    pub byte_match_tuple: ByteMatchTuple,
}

pub type ByteMatchSetUpdates = Vec<ByteMatchSetUpdate>;
pub type ByteMatchTargetString = Vec<u8>;
#[doc="<p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ByteMatchTuple {
    #[doc="<p>The part of a web request that you want AWS WAF to search, such as a specified header or a query string. For more information, see <a>FieldToMatch</a>.</p>"]
    #[serde(rename="FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[doc="<p>Within the portion of a web request that you want to search (for example, in the query string, if any), specify where you want AWS WAF to search. Valid values include the following:</p> <p> <b>CONTAINS</b> </p> <p>The specified part of the web request must include the value of <code>TargetString</code>, but the location doesn't matter.</p> <p> <b>CONTAINS_WORD</b> </p> <p>The specified part of the web request must include the value of <code>TargetString</code>, and <code>TargetString</code> must contain only alphanumeric characters or underscore (A-Z, a-z, 0-9, or _). In addition, <code>TargetString</code> must be a word, which means one of the following:</p> <ul> <li> <p> <code>TargetString</code> exactly matches the value of the specified part of the web request, such as the value of a header.</p> </li> <li> <p> <code>TargetString</code> is at the beginning of the specified part of the web request and is followed by a character other than an alphanumeric character or underscore (_), for example, <code>BadBot;</code>.</p> </li> <li> <p> <code>TargetString</code> is at the end of the specified part of the web request and is preceded by a character other than an alphanumeric character or underscore (_), for example, <code>;BadBot</code>.</p> </li> <li> <p> <code>TargetString</code> is in the middle of the specified part of the web request and is preceded and followed by characters other than alphanumeric characters or underscore (_), for example, <code>-BadBot;</code>.</p> </li> </ul> <p> <b>EXACTLY</b> </p> <p>The value of the specified part of the web request must exactly match the value of <code>TargetString</code>.</p> <p> <b>STARTS_WITH</b> </p> <p>The value of <code>TargetString</code> must appear at the beginning of the specified part of the web request.</p> <p> <b>ENDS_WITH</b> </p> <p>The value of <code>TargetString</code> must appear at the end of the specified part of the web request.</p>"]
    #[serde(rename="PositionalConstraint")]
    pub positional_constraint: PositionalConstraint,
    #[doc="<p>The value that you want AWS WAF to search for. AWS WAF searches for the specified string in the part of web requests that you specified in <code>FieldToMatch</code>. The maximum length of the value is 50 bytes.</p> <p>Valid values depend on the values that you specified for <code>FieldToMatch</code>:</p> <ul> <li> <p> <code>HEADER</code>: The value that you want AWS WAF to search for in the request header that you specified in <a>FieldToMatch</a>, for example, the value of the <code>User-Agent</code> or <code>Referer</code> header.</p> </li> <li> <p> <code>METHOD</code>: The HTTP method, which indicates the type of operation specified in the request. CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>.</p> </li> <li> <p> <code>QUERY_STRING</code>: The value that you want AWS WAF to search for in the query string, which is the part of a URL that appears after a <code>?</code> character.</p> </li> <li> <p> <code>URI</code>: The value that you want AWS WAF to search for in the part of a URL that identifies a resource, for example, <code>/images/daily-ad.jpg</code>.</p> </li> <li> <p> <code>BODY</code>: The part of a request that contains any additional data that you want to send to your web server as the HTTP request body, such as data from a form. The request body immediately follows the request headers. Note that only the first <code>8192</code> bytes of the request body are forwarded to AWS WAF for inspection. To allow or block requests based on the length of the body, you can create a size constraint set. For more information, see <a>CreateSizeConstraintSet</a>. </p> </li> </ul> <p>If <code>TargetString</code> includes alphabetic characters A-Z and a-z, note that the value is case sensitive.</p> <p> <b>If you're using the AWS WAF API</b> </p> <p>Specify a base64-encoded version of the value. The maximum length of the value before you base64-encode it is 50 bytes.</p> <p>For example, suppose the value of <code>Type</code> is <code>HEADER</code> and the value of <code>Data</code> is <code>User-Agent</code>. If you want to search the <code>User-Agent</code> header for the value <code>BadBot</code>, you base64-encode <code>BadBot</code> using MIME base64 encoding and include the resulting value, <code>QmFkQm90</code>, in the value of <code>TargetString</code>.</p> <p> <b>If you're using the AWS CLI or one of the AWS SDKs</b> </p> <p>The value that you want AWS WAF to search for. The SDK automatically base64 encodes the value.</p>"]
    #[serde(rename="TargetString")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub target_string: ByteMatchTargetString,
    #[doc="<p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>TargetString</code> before inspecting a request for a match.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system commandline command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \\ \" ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\\f, formfeed, decimal 12</p> </li> <li> <p>\\t, tab, decimal 9</p> </li> <li> <p>\\n, newline, decimal 10</p> </li> <li> <p>\\r, carriage return, decimal 13</p> </li> <li> <p>\\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>\"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a \"less than\" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>"]
    #[serde(rename="TextTransformation")]
    pub text_transformation: TextTransformation,
}

pub type ByteMatchTuples = Vec<ByteMatchTuple>;
pub type ChangeAction = String;
pub type ChangeToken = String;
pub type ChangeTokenStatus = String;
pub type ComparisonOperator = String;
pub type Country = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateByteMatchSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateByteMatchSetResponse {
    #[doc="<p>A <a>ByteMatchSet</a> that contains no <code>ByteMatchTuple</code> objects.</p>"]
    #[serde(rename="ByteMatchSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub byte_match_set: Option<ByteMatchSet>,
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateIPSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>A friendly name or description of the <a>IPSet</a>. You can't change <code>Name</code> after you create the <code>IPSet</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateIPSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
    #[doc="<p>The <a>IPSet</a> returned in the <code>CreateIPSet</code> response.</p>"]
    #[serde(rename="IPSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_set: Option<IPSet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateRateBasedRuleRequest {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>A friendly name or description for the metrics for this <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>"]
    #[serde(rename="MetricName")]
    pub metric_name: MetricName,
    #[doc="<p>A friendly name or description of the <a>RateBasedRule</a>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
    #[doc="<p>The field that AWS WAF uses to determine if requests are likely arriving from a single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests that arrive from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>"]
    #[serde(rename="RateKey")]
    pub rate_key: RateKey,
    #[doc="<p>The maximum number of requests, which have an identical value in the field that is specified by <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>"]
    #[serde(rename="RateLimit")]
    pub rate_limit: RateLimit,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateRateBasedRuleResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
    #[doc="<p>The <a>RateBasedRule</a> that is returned in the <code>CreateRateBasedRule</code> response.</p>"]
    #[serde(rename="Rule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule: Option<RateBasedRule>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateRuleRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>A friendly name or description for the metrics for this <code>Rule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change the name of the metric after you create the <code>Rule</code>.</p>"]
    #[serde(rename="MetricName")]
    pub metric_name: MetricName,
    #[doc="<p>A friendly name or description of the <a>Rule</a>. You can't change the name of a <code>Rule</code> after you create it.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateRuleResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
    #[doc="<p>The <a>Rule</a> returned in the <code>CreateRule</code> response.</p>"]
    #[serde(rename="Rule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateSizeConstraintSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>A friendly name or description of the <a>SizeConstraintSet</a>. You can't change <code>Name</code> after you create a <code>SizeConstraintSet</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateSizeConstraintSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
    #[doc="<p>A <a>SizeConstraintSet</a> that contains no <code>SizeConstraint</code> objects.</p>"]
    #[serde(rename="SizeConstraintSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_constraint_set: Option<SizeConstraintSet>,
}

#[doc="<p>A request to create a <a>SqlInjectionMatchSet</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateSqlInjectionMatchSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>A friendly name or description for the <a>SqlInjectionMatchSet</a> that you're creating. You can't change <code>Name</code> after you create the <code>SqlInjectionMatchSet</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[doc="<p>The response to a <code>CreateSqlInjectionMatchSet</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateSqlInjectionMatchSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
    #[doc="<p>A <a>SqlInjectionMatchSet</a>.</p>"]
    #[serde(rename="SqlInjectionMatchSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sql_injection_match_set: Option<SqlInjectionMatchSet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateWebACLRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The action that you want AWS WAF to take when a request doesn't match the criteria specified in any of the <code>Rule</code> objects that are associated with the <code>WebACL</code>.</p>"]
    #[serde(rename="DefaultAction")]
    pub default_action: WafAction,
    #[doc="<p>A friendly name or description for the metrics for this <code>WebACL</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change <code>MetricName</code> after you create the <code>WebACL</code>.</p>"]
    #[serde(rename="MetricName")]
    pub metric_name: MetricName,
    #[doc="<p>A friendly name or description of the <a>WebACL</a>. You can't change <code>Name</code> after you create the <code>WebACL</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateWebACLResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
    #[doc="<p>The <a>WebACL</a> returned in the <code>CreateWebACL</code> response.</p>"]
    #[serde(rename="WebACL")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub web_acl: Option<WebACL>,
}

#[doc="<p>A request to create an <a>XssMatchSet</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateXssMatchSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>A friendly name or description for the <a>XssMatchSet</a> that you're creating. You can't change <code>Name</code> after you create the <code>XssMatchSet</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[doc="<p>The response to a <code>CreateXssMatchSet</code> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateXssMatchSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>CreateXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
    #[doc="<p>An <a>XssMatchSet</a>.</p>"]
    #[serde(rename="XssMatchSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub xss_match_set: Option<XssMatchSet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteByteMatchSetRequest {
    #[doc="<p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to delete. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>"]
    #[serde(rename="ByteMatchSetId")]
    pub byte_match_set_id: ResourceId,
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteByteMatchSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>DeleteByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteIPSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to delete. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>"]
    #[serde(rename="IPSetId")]
    pub ip_set_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteIPSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>DeleteIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteRateBasedRuleRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>RuleId</code> of the <a>RateBasedRule</a> that you want to delete. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteRateBasedRuleResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteRuleRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>RuleId</code> of the <a>Rule</a> that you want to delete. <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteRuleResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteSizeConstraintSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to delete. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>"]
    #[serde(rename="SizeConstraintSetId")]
    pub size_constraint_set_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteSizeConstraintSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>DeleteSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[doc="<p>A request to delete a <a>SqlInjectionMatchSet</a> from AWS WAF.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteSqlInjectionMatchSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>SqlInjectionMatchSetId</code> of the <a>SqlInjectionMatchSet</a> that you want to delete. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>"]
    #[serde(rename="SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: ResourceId,
}

#[doc="<p>The response to a request to delete a <a>SqlInjectionMatchSet</a> from AWS WAF.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteSqlInjectionMatchSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>DeleteSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteWebACLRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to delete. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>"]
    #[serde(rename="WebACLId")]
    pub web_acl_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteWebACLResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>DeleteWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[doc="<p>A request to delete an <a>XssMatchSet</a> from AWS WAF.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteXssMatchSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>XssMatchSetId</code> of the <a>XssMatchSet</a> that you want to delete. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>"]
    #[serde(rename="XssMatchSetId")]
    pub xss_match_set_id: ResourceId,
}

#[doc="<p>The response to a request to delete an <a>XssMatchSet</a> from AWS WAF.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteXssMatchSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>DeleteXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DisassociateWebACLRequest {
    #[doc="<p>The ARN (Amazon Resource Name) of the resource from which the web ACL is being removed.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: ResourceArn,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DisassociateWebACLResponse;

pub type ErrorMessage = String;
#[doc="<p>Specifies where in a web request to look for <code>TargetString</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct FieldToMatch {
    #[doc="<p>When the value of <code>Type</code> is <code>HEADER</code>, enter the name of the header that you want AWS WAF to search, for example, <code>User-Agent</code> or <code>Referer</code>. If the value of <code>Type</code> is any other value, omit <code>Data</code>.</p> <p>The name of the header is not case sensitive.</p>"]
    #[serde(rename="Data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<MatchFieldData>,
    #[doc="<p>The part of the web request that you want AWS WAF to search for a specified string. Parts of a request that you can search include the following:</p> <ul> <li> <p> <code>HEADER</code>: A specified request header, for example, the value of the <code>User-Agent</code> or <code>Referer</code> header. If you choose <code>HEADER</code> for the type, specify the name of the header in <code>Data</code>.</p> </li> <li> <p> <code>METHOD</code>: The HTTP method, which indicated the type of operation that the request is asking the origin to perform. Amazon CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>.</p> </li> <li> <p> <code>QUERY_STRING</code>: A query string, which is the part of a URL that appears after a <code>?</code> character, if any.</p> </li> <li> <p> <code>URI</code>: The part of a web request that identifies a resource, for example, <code>/images/daily-ad.jpg</code>.</p> </li> <li> <p> <code>BODY</code>: The part of a request that contains any additional data that you want to send to your web server as the HTTP request body, such as data from a form. The request body immediately follows the request headers. Note that only the first <code>8192</code> bytes of the request body are forwarded to AWS WAF for inspection. To allow or block requests based on the length of the body, you can create a size constraint set. For more information, see <a>CreateSizeConstraintSet</a>. </p> </li> </ul>"]
    #[serde(rename="Type")]
    pub type_: MatchFieldType,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetByteMatchSetRequest {
    #[doc="<p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to get. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>"]
    #[serde(rename="ByteMatchSetId")]
    pub byte_match_set_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetByteMatchSetResponse {
    #[doc="<p>Information about the <a>ByteMatchSet</a> that you specified in the <code>GetByteMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>ByteMatchSet</a>: Contains <code>ByteMatchSetId</code>, <code>ByteMatchTuples</code>, and <code>Name</code> </p> </li> <li> <p> <code>ByteMatchTuples</code>: Contains an array of <a>ByteMatchTuple</a> objects. Each <code>ByteMatchTuple</code> object contains <a>FieldToMatch</a>, <code>PositionalConstraint</code>, <code>TargetString</code>, and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="ByteMatchSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub byte_match_set: Option<ByteMatchSet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetChangeTokenRequest;

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetChangeTokenResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used in the request. Use this value in a <code>GetChangeTokenStatus</code> request to get the current status of the request. </p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetChangeTokenStatusRequest {
    #[doc="<p>The change token for which you want to get the status. This change token was previously returned in the <code>GetChangeToken</code> response.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetChangeTokenStatusResponse {
    #[doc="<p>The status of the change token.</p>"]
    #[serde(rename="ChangeTokenStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token_status: Option<ChangeTokenStatus>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetIPSetRequest {
    #[doc="<p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to get. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>"]
    #[serde(rename="IPSetId")]
    pub ip_set_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetIPSetResponse {
    #[doc="<p>Information about the <a>IPSet</a> that you specified in the <code>GetIPSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>IPSet</a>: Contains <code>IPSetDescriptors</code>, <code>IPSetId</code>, and <code>Name</code> </p> </li> <li> <p> <code>IPSetDescriptors</code>: Contains an array of <a>IPSetDescriptor</a> objects. Each <code>IPSetDescriptor</code> object contains <code>Type</code> and <code>Value</code> </p> </li> </ul>"]
    #[serde(rename="IPSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_set: Option<IPSet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetRateBasedRuleManagedKeysRequest {
    #[doc="<p>A null value and not currently used. Do not include this in your request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
    #[doc="<p>The <code>RuleId</code> of the <a>RateBasedRule</a> for which you want to get a list of <code>ManagedKeys</code>. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetRateBasedRuleManagedKeysResponse {
    #[doc="<p>An array of IP addresses that currently are blocked by the specified <a>RateBasedRule</a>. </p>"]
    #[serde(rename="ManagedKeys")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub managed_keys: Option<ManagedKeys>,
    #[doc="<p>A null value and not currently used.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetRateBasedRuleRequest {
    #[doc="<p>The <code>RuleId</code> of the <a>RateBasedRule</a> that you want to get. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetRateBasedRuleResponse {
    #[doc="<p>Information about the <a>RateBasedRule</a> that you specified in the <code>GetRateBasedRule</code> request.</p>"]
    #[serde(rename="Rule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule: Option<RateBasedRule>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetRuleRequest {
    #[doc="<p>The <code>RuleId</code> of the <a>Rule</a> that you want to get. <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetRuleResponse {
    #[doc="<p>Information about the <a>Rule</a> that you specified in the <code>GetRule</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>Rule</a>: Contains <code>MetricName</code>, <code>Name</code>, an array of <code>Predicate</code> objects, and <code>RuleId</code> </p> </li> <li> <p> <a>Predicate</a>: Each <code>Predicate</code> object contains <code>DataId</code>, <code>Negated</code>, and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="Rule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule: Option<Rule>,
}

pub type GetSampledRequestsMaxItems = i64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSampledRequestsRequest {
    #[doc="<p>The number of requests that you want AWS WAF to return from among the first 5,000 requests that your AWS resource received during the time range. If your resource received fewer requests than the value of <code>MaxItems</code>, <code>GetSampledRequests</code> returns information about all of them. </p>"]
    #[serde(rename="MaxItems")]
    pub max_items: GetSampledRequestsMaxItems,
    #[doc="<p> <code>RuleId</code> is one of two values:</p> <ul> <li> <p>The <code>RuleId</code> of the <code>Rule</code> for which you want <code>GetSampledRequests</code> to return a sample of requests.</p> </li> <li> <p> <code>Default_Action</code>, which causes <code>GetSampledRequests</code> to return a sample of the requests that didn't match any of the rules in the specified <code>WebACL</code>.</p> </li> </ul>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
    #[doc="<p>The start date and time and the end date and time of the range for which you want <code>GetSampledRequests</code> to return a sample of requests. Specify the date and time in the following format: <code>\"2016-09-27T14:50Z\"</code>. You can specify any time range in the previous three hours.</p>"]
    #[serde(rename="TimeWindow")]
    pub time_window: TimeWindow,
    #[doc="<p>The <code>WebACLId</code> of the <code>WebACL</code> for which you want <code>GetSampledRequests</code> to return a sample of requests.</p>"]
    #[serde(rename="WebAclId")]
    pub web_acl_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSampledRequestsResponse {
    #[doc="<p>The total number of requests from which <code>GetSampledRequests</code> got a sample of <code>MaxItems</code> requests. If <code>PopulationSize</code> is less than <code>MaxItems</code>, the sample includes every request that your AWS resource received during the specified time range.</p>"]
    #[serde(rename="PopulationSize")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub population_size: Option<PopulationSize>,
    #[doc="<p>A complex type that contains detailed information about each of the requests in the sample.</p>"]
    #[serde(rename="SampledRequests")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sampled_requests: Option<SampledHTTPRequests>,
    #[doc="<p>Usually, <code>TimeWindow</code> is the time range that you specified in the <code>GetSampledRequests</code> request. However, if your AWS resource received more than 5,000 requests during the time range that you specified in the request, <code>GetSampledRequests</code> returns the time range for the first 5,000 requests.</p>"]
    #[serde(rename="TimeWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub time_window: Option<TimeWindow>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSizeConstraintSetRequest {
    #[doc="<p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to get. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>"]
    #[serde(rename="SizeConstraintSetId")]
    pub size_constraint_set_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSizeConstraintSetResponse {
    #[doc="<p>Information about the <a>SizeConstraintSet</a> that you specified in the <code>GetSizeConstraintSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>SizeConstraintSet</a>: Contains <code>SizeConstraintSetId</code>, <code>SizeConstraints</code>, and <code>Name</code> </p> </li> <li> <p> <code>SizeConstraints</code>: Contains an array of <a>SizeConstraint</a> objects. Each <code>SizeConstraint</code> object contains <a>FieldToMatch</a>, <code>TextTransformation</code>, <code>ComparisonOperator</code>, and <code>Size</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="SizeConstraintSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_constraint_set: Option<SizeConstraintSet>,
}

#[doc="<p>A request to get a <a>SqlInjectionMatchSet</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetSqlInjectionMatchSetRequest {
    #[doc="<p>The <code>SqlInjectionMatchSetId</code> of the <a>SqlInjectionMatchSet</a> that you want to get. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>"]
    #[serde(rename="SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: ResourceId,
}

#[doc="<p>The response to a <a>GetSqlInjectionMatchSet</a> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetSqlInjectionMatchSetResponse {
    #[doc="<p>Information about the <a>SqlInjectionMatchSet</a> that you specified in the <code>GetSqlInjectionMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>SqlInjectionMatchSet</a>: Contains <code>Name</code>, <code>SqlInjectionMatchSetId</code>, and an array of <code>SqlInjectionMatchTuple</code> objects</p> </li> <li> <p> <a>SqlInjectionMatchTuple</a>: Each <code>SqlInjectionMatchTuple</code> object contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="SqlInjectionMatchSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sql_injection_match_set: Option<SqlInjectionMatchSet>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetWebACLForResourceRequest {
    #[doc="<p>The ARN (Amazon Resource Name) of the resource for which to get the web ACL.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: ResourceArn,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetWebACLForResourceResponse {
    #[doc="<p>Information about the web ACL that you specified in the <code>GetWebACLForResource</code> request. If there is no associated resource, a null WebACLSummary is returned.</p>"]
    #[serde(rename="WebACLSummary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub web_acl_summary: Option<WebACLSummary>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct GetWebACLRequest {
    #[doc="<p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to get. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>"]
    #[serde(rename="WebACLId")]
    pub web_acl_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetWebACLResponse {
    #[doc="<p>Information about the <a>WebACL</a> that you specified in the <code>GetWebACL</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>WebACL</a>: Contains <code>DefaultAction</code>, <code>MetricName</code>, <code>Name</code>, an array of <code>Rule</code> objects, and <code>WebACLId</code> </p> </li> <li> <p> <code>DefaultAction</code> (Data type is <a>WafAction</a>): Contains <code>Type</code> </p> </li> <li> <p> <code>Rules</code>: Contains an array of <code>ActivatedRule</code> objects, which contain <code>Action</code>, <code>Priority</code>, and <code>RuleId</code> </p> </li> <li> <p> <code>Action</code>: Contains <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="WebACL")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub web_acl: Option<WebACL>,
}

#[doc="<p>A request to get an <a>XssMatchSet</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetXssMatchSetRequest {
    #[doc="<p>The <code>XssMatchSetId</code> of the <a>XssMatchSet</a> that you want to get. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>"]
    #[serde(rename="XssMatchSetId")]
    pub xss_match_set_id: ResourceId,
}

#[doc="<p>The response to a <a>GetXssMatchSet</a> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetXssMatchSetResponse {
    #[doc="<p>Information about the <a>XssMatchSet</a> that you specified in the <code>GetXssMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>XssMatchSet</a>: Contains <code>Name</code>, <code>XssMatchSetId</code>, and an array of <code>XssMatchTuple</code> objects</p> </li> <li> <p> <a>XssMatchTuple</a>: Each <code>XssMatchTuple</code> object contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="XssMatchSet")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub xss_match_set: Option<XssMatchSet>,
}

#[doc="<p>The response from a <a>GetSampledRequests</a> request includes an <code>HTTPHeader</code> complex type that appears as <code>Headers</code> in the response syntax. <code>HTTPHeader</code> contains the names and values of all of the headers that appear in one of the web requests that were returned by <code>GetSampledRequests</code>. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct HTTPHeader {
    #[doc="<p>The name of one of the headers in the sampled web request.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<HeaderName>,
    #[doc="<p>The value of one of the headers in the sampled web request.</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<HeaderValue>,
}

pub type HTTPHeaders = Vec<HTTPHeader>;
pub type HTTPMethod = String;
#[doc="<p>The response from a <a>GetSampledRequests</a> request includes an <code>HTTPRequest</code> complex type that appears as <code>Request</code> in the response syntax. <code>HTTPRequest</code> contains information about one of the web requests that were returned by <code>GetSampledRequests</code>. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct HTTPRequest {
    #[doc="<p>The IP address that the request originated from. If the <code>WebACL</code> is associated with a CloudFront distribution, this is the value of one of the following fields in CloudFront access logs:</p> <ul> <li> <p> <code>c-ip</code>, if the viewer did not use an HTTP proxy or a load balancer to send the request</p> </li> <li> <p> <code>x-forwarded-for</code>, if the viewer did use an HTTP proxy or a load balancer to send the request</p> </li> </ul>"]
    #[serde(rename="ClientIP")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_ip: Option<IPString>,
    #[doc="<p>The two-letter country code for the country that the request originated from. For a current list of country codes, see the Wikipedia entry <a href=\"https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2\">ISO 3166-1 alpha-2</a>.</p>"]
    #[serde(rename="Country")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub country: Option<Country>,
    #[doc="<p>The HTTP version specified in the sampled web request, for example, <code>HTTP/1.1</code>.</p>"]
    #[serde(rename="HTTPVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub http_version: Option<HTTPVersion>,
    #[doc="<p>A complex type that contains two values for each header in the sampled web request: the name of the header and the value of the header.</p>"]
    #[serde(rename="Headers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub headers: Option<HTTPHeaders>,
    #[doc="<p>The HTTP method specified in the sampled web request. CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>. </p>"]
    #[serde(rename="Method")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub method: Option<HTTPMethod>,
    #[doc="<p>The part of a web request that identifies the resource, for example, <code>/images/daily-ad.jpg</code>.</p>"]
    #[serde(rename="URI")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uri: Option<URIString>,
}

pub type HTTPVersion = String;
pub type HeaderName = String;
pub type HeaderValue = String;
#[doc="<p>Contains one or more IP addresses or blocks of IP addresses specified in Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports /8, /16, /24, and /32 IP address ranges for IPv4, and /24, /32, /48, /56, /64 and /128 for IPv6.</p> <p>To specify an individual IP address, you specify the four-part IP address followed by a <code>/32</code>, for example, 192.0.2.0/31. To block a range of IP addresses, you can specify a <code>/128</code>, <code>/64</code>, <code>/56</code>, <code>/48</code>, <code>/32</code>, <code>/24</code>, <code>/16</code>, or <code>/8</code> CIDR. For more information about CIDR notation, see the Wikipedia entry <a href=\"https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing\">Classless Inter-Domain Routing</a>. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct IPSet {
    #[doc="<p>The IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR notation) that web requests originate from. If the <code>WebACL</code> is associated with a CloudFront distribution and the viewer did not use an HTTP proxy or a load balancer to send the request, this is the value of the c-ip field in the CloudFront access logs.</p>"]
    #[serde(rename="IPSetDescriptors")]
    pub ip_set_descriptors: IPSetDescriptors,
    #[doc="<p>The <code>IPSetId</code> for an <code>IPSet</code>. You use <code>IPSetId</code> to get information about an <code>IPSet</code> (see <a>GetIPSet</a>), update an <code>IPSet</code> (see <a>UpdateIPSet</a>), insert an <code>IPSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>IPSet</code> from AWS WAF (see <a>DeleteIPSet</a>).</p> <p> <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>"]
    #[serde(rename="IPSetId")]
    pub ip_set_id: ResourceId,
    #[doc="<p>A friendly name or description of the <a>IPSet</a>. You can't change the name of an <code>IPSet</code> after you create it.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ResourceName>,
}

#[doc="<p>Specifies the IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR format) that web requests originate from.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct IPSetDescriptor {
    #[doc="<p>Specify <code>IPV4</code> or <code>IPV6</code>.</p>"]
    #[serde(rename="Type")]
    pub type_: IPSetDescriptorType,
    #[doc="<p>Specify an IPv4 address by using CIDR notation. For example:</p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li> </ul> <p>For more information about CIDR notation, see the Wikipedia entry <a href=\"https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing\">Classless Inter-Domain Routing</a>.</p> <p>Specify an IPv6 address by using CIDR notation. For example:</p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li> </ul>"]
    #[serde(rename="Value")]
    pub value: IPSetDescriptorValue,
}

pub type IPSetDescriptorType = String;
pub type IPSetDescriptorValue = String;
pub type IPSetDescriptors = Vec<IPSetDescriptor>;
pub type IPSetSummaries = Vec<IPSetSummary>;
#[doc="<p>Contains the identifier and the name of the <code>IPSet</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct IPSetSummary {
    #[doc="<p>The <code>IPSetId</code> for an <a>IPSet</a>. You can use <code>IPSetId</code> in a <a>GetIPSet</a> request to get detailed information about an <a>IPSet</a>.</p>"]
    #[serde(rename="IPSetId")]
    pub ip_set_id: ResourceId,
    #[doc="<p>A friendly name or description of the <a>IPSet</a>. You can't change the name of an <code>IPSet</code> after you create it.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
}

#[doc="<p>Specifies the type of update to perform to an <a>IPSet</a> with <a>UpdateIPSet</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct IPSetUpdate {
    #[doc="<p>Specifies whether to insert or delete an IP address with <a>UpdateIPSet</a>.</p>"]
    #[serde(rename="Action")]
    pub action: ChangeAction,
    #[doc="<p>The IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR notation) that web requests originate from.</p>"]
    #[serde(rename="IPSetDescriptor")]
    pub ip_set_descriptor: IPSetDescriptor,
}

pub type IPSetUpdates = Vec<IPSetUpdate>;
pub type IPString = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListByteMatchSetsRequest {
    #[doc="<p>Specifies the number of <code>ByteMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>ByteMatchSets</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>ByteMatchSet</code> objects.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<PaginationLimit>,
    #[doc="<p>If you specify a value for <code>Limit</code> and you have more <code>ByteMatchSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ByteMatchSets</code>. For the second and subsequent <code>ListByteMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ByteMatchSets</code>.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListByteMatchSetsResponse {
    #[doc="<p>An array of <a>ByteMatchSetSummary</a> objects.</p>"]
    #[serde(rename="ByteMatchSets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub byte_match_sets: Option<ByteMatchSetSummaries>,
    #[doc="<p>If you have more <code>ByteMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>ByteMatchSet</code> objects, submit another <code>ListByteMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListIPSetsRequest {
    #[doc="<p>Specifies the number of <code>IPSet</code> objects that you want AWS WAF to return for this request. If you have more <code>IPSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>IPSet</code> objects.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<PaginationLimit>,
    #[doc="<p>If you specify a value for <code>Limit</code> and you have more <code>IPSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>IPSets</code>. For the second and subsequent <code>ListIPSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ByteMatchSets</code>.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListIPSetsResponse {
    #[doc="<p>An array of <a>IPSetSummary</a> objects.</p>"]
    #[serde(rename="IPSets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_sets: Option<IPSetSummaries>,
    #[doc="<p>If you have more <code>IPSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>IPSet</code> objects, submit another <code>ListIPSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRateBasedRulesRequest {
    #[doc="<p>Specifies the number of <code>Rules</code> that you want AWS WAF to return for this request. If you have more <code>Rules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<PaginationLimit>,
    #[doc="<p>If you specify a value for <code>Limit</code> and you have more <code>Rules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>Rules</code>. For the second and subsequent <code>ListRateBasedRules</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>Rules</code>.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListRateBasedRulesResponse {
    #[doc="<p>If you have more <code>Rules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>Rules</code>, submit another <code>ListRateBasedRules</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
    #[doc="<p>An array of <a>RuleSummary</a> objects.</p>"]
    #[serde(rename="Rules")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rules: Option<RuleSummaries>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListResourcesForWebACLRequest {
    #[doc="<p>The unique identifier (ID) of the web ACL for which to list the associated resources.</p>"]
    #[serde(rename="WebACLId")]
    pub web_acl_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListResourcesForWebACLResponse {
    #[doc="<p>An array of ARNs (Amazon Resource Names) of the resources associated with the specified web ACL. An array with zero elements is returned if there are no resources associated with the web ACL.</p>"]
    #[serde(rename="ResourceArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_arns: Option<ResourceArns>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRulesRequest {
    #[doc="<p>Specifies the number of <code>Rules</code> that you want AWS WAF to return for this request. If you have more <code>Rules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<PaginationLimit>,
    #[doc="<p>If you specify a value for <code>Limit</code> and you have more <code>Rules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>Rules</code>. For the second and subsequent <code>ListRules</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>Rules</code>.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListRulesResponse {
    #[doc="<p>If you have more <code>Rules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>Rules</code>, submit another <code>ListRules</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
    #[doc="<p>An array of <a>RuleSummary</a> objects.</p>"]
    #[serde(rename="Rules")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rules: Option<RuleSummaries>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListSizeConstraintSetsRequest {
    #[doc="<p>Specifies the number of <code>SizeConstraintSet</code> objects that you want AWS WAF to return for this request. If you have more <code>SizeConstraintSets</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>SizeConstraintSet</code> objects.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<PaginationLimit>,
    #[doc="<p>If you specify a value for <code>Limit</code> and you have more <code>SizeConstraintSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>SizeConstraintSets</code>. For the second and subsequent <code>ListSizeConstraintSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>SizeConstraintSets</code>.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListSizeConstraintSetsResponse {
    #[doc="<p>If you have more <code>SizeConstraintSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>SizeConstraintSet</code> objects, submit another <code>ListSizeConstraintSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
    #[doc="<p>An array of <a>SizeConstraintSetSummary</a> objects.</p>"]
    #[serde(rename="SizeConstraintSets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_constraint_sets: Option<SizeConstraintSetSummaries>,
}

#[doc="<p>A request to list the <a>SqlInjectionMatchSet</a> objects created by the current AWS account.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListSqlInjectionMatchSetsRequest {
    #[doc="<p>Specifies the number of <a>SqlInjectionMatchSet</a> objects that you want AWS WAF to return for this request. If you have more <code>SqlInjectionMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<PaginationLimit>,
    #[doc="<p>If you specify a value for <code>Limit</code> and you have more <a>SqlInjectionMatchSet</a> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>SqlInjectionMatchSets</code>. For the second and subsequent <code>ListSqlInjectionMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>SqlInjectionMatchSets</code>.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[doc="<p>The response to a <a>ListSqlInjectionMatchSets</a> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListSqlInjectionMatchSetsResponse {
    #[doc="<p>If you have more <a>SqlInjectionMatchSet</a> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>SqlInjectionMatchSet</code> objects, submit another <code>ListSqlInjectionMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
    #[doc="<p>An array of <a>SqlInjectionMatchSetSummary</a> objects.</p>"]
    #[serde(rename="SqlInjectionMatchSets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sql_injection_match_sets: Option<SqlInjectionMatchSetSummaries>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListWebACLsRequest {
    #[doc="<p>Specifies the number of <code>WebACL</code> objects that you want AWS WAF to return for this request. If you have more <code>WebACL</code> objects than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>WebACL</code> objects.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<PaginationLimit>,
    #[doc="<p>If you specify a value for <code>Limit</code> and you have more <code>WebACL</code> objects than the number that you specify for <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>WebACL</code> objects. For the second and subsequent <code>ListWebACLs</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>WebACL</code> objects.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListWebACLsResponse {
    #[doc="<p>If you have more <code>WebACL</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>WebACL</code> objects, submit another <code>ListWebACLs</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
    #[doc="<p>An array of <a>WebACLSummary</a> objects.</p>"]
    #[serde(rename="WebACLs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub web_ac_ls: Option<WebACLSummaries>,
}

#[doc="<p>A request to list the <a>XssMatchSet</a> objects created by the current AWS account.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListXssMatchSetsRequest {
    #[doc="<p>Specifies the number of <a>XssMatchSet</a> objects that you want AWS WAF to return for this request. If you have more <code>XssMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<PaginationLimit>,
    #[doc="<p>If you specify a value for <code>Limit</code> and you have more <a>XssMatchSet</a> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>XssMatchSets</code>. For the second and subsequent <code>ListXssMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>XssMatchSets</code>.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
}

#[doc="<p>The response to a <a>ListXssMatchSets</a> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListXssMatchSetsResponse {
    #[doc="<p>If you have more <a>XssMatchSet</a> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>XssMatchSet</code> objects, submit another <code>ListXssMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>"]
    #[serde(rename="NextMarker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_marker: Option<NextMarker>,
    #[doc="<p>An array of <a>XssMatchSetSummary</a> objects.</p>"]
    #[serde(rename="XssMatchSets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub xss_match_sets: Option<XssMatchSetSummaries>,
}

pub type ManagedKey = String;
pub type ManagedKeys = Vec<ManagedKey>;
pub type MatchFieldData = String;
pub type MatchFieldType = String;
pub type MetricName = String;
pub type Negated = bool;
pub type NextMarker = String;
pub type PaginationLimit = i64;
pub type ParameterExceptionField = String;
pub type ParameterExceptionParameter = String;
pub type ParameterExceptionReason = String;
pub type PopulationSize = i64;
pub type PositionalConstraint = String;
#[doc="<p>Specifies the <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, and <a>SizeConstraintSet</a> objects that you want to add to a <code>Rule</code> and, for each object, indicates whether you want to negate the settings, for example, requests that do NOT originate from the IP address 192.0.2.44. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Predicate {
    #[doc="<p>A unique identifier for a predicate in a <code>Rule</code>, such as <code>ByteMatchSetId</code> or <code>IPSetId</code>. The ID is returned by the corresponding <code>Create</code> or <code>List</code> command.</p>"]
    #[serde(rename="DataId")]
    pub data_id: ResourceId,
    #[doc="<p>Set <code>Negated</code> to <code>False</code> if you want AWS WAF to allow, block, or count requests based on the settings in the specified <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, or <a>SizeConstraintSet</a>. For example, if an <code>IPSet</code> includes the IP address <code>192.0.2.44</code>, AWS WAF will allow or block requests based on that IP address.</p> <p>Set <code>Negated</code> to <code>True</code> if you want AWS WAF to allow or block a request based on the negation of the settings in the <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, or <a>SizeConstraintSet</a>. For example, if an <code>IPSet</code> includes the IP address <code>192.0.2.44</code>, AWS WAF will allow, block, or count requests based on all IP addresses <i>except</i> <code>192.0.2.44</code>.</p>"]
    #[serde(rename="Negated")]
    pub negated: Negated,
    #[doc="<p>The type of predicate in a <code>Rule</code>, such as <code>ByteMatchSet</code> or <code>IPSet</code>.</p>"]
    #[serde(rename="Type")]
    pub type_: PredicateType,
}

pub type PredicateType = String;
pub type Predicates = Vec<Predicate>;
#[doc="<p>A <code>RateBasedRule</code> is identical to a regular <a>Rule</a>, with one addition: a <code>RateBasedRule</code> counts the number of requests that arrive from a specified IP address every five minutes. For example, based on recent requests that you've seen from an attacker, you might create a <code>RateBasedRule</code> that includes the following conditions: </p> <ul> <li> <p>The requests come from 192.0.2.44.</p> </li> <li> <p>They contain the value <code>BadBot</code> in the <code>User-Agent</code> header.</p> </li> </ul> <p>In the rule, you also define the rate limit as 15,000.</p> <p>Requests that meet both of these conditions and exceed 15,000 requests every five minutes trigger the rule's action (block or count), which is defined in the web ACL.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RateBasedRule {
    #[doc="<p>The <code>Predicates</code> object contains one <code>Predicate</code> element for each <a>ByteMatchSet</a>, <a>IPSet</a>, or <a>SqlInjectionMatchSet</a> object that you want to include in a <code>RateBasedRule</code>.</p>"]
    #[serde(rename="MatchPredicates")]
    pub match_predicates: Predicates,
    #[doc="<p>A friendly name or description for the metrics for a <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>"]
    #[serde(rename="MetricName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_name: Option<MetricName>,
    #[doc="<p>A friendly name or description for a <code>RateBasedRule</code>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc="<p>The field that AWS WAF uses to determine if requests are likely arriving from single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests arriving from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>"]
    #[serde(rename="RateKey")]
    pub rate_key: RateKey,
    #[doc="<p>The maximum number of requests, which have an identical value in the field specified by the <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>"]
    #[serde(rename="RateLimit")]
    pub rate_limit: RateLimit,
    #[doc="<p>A unique identifier for a <code>RateBasedRule</code>. You use <code>RuleId</code> to get more information about a <code>RateBasedRule</code> (see <a>GetRateBasedRule</a>), update a <code>RateBasedRule</code> (see <a>UpdateRateBasedRule</a>), insert a <code>RateBasedRule</code> into a <code>WebACL</code> or delete one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>RateBasedRule</code> from AWS WAF (see <a>DeleteRateBasedRule</a>).</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
}

pub type RateKey = String;
pub type RateLimit = i64;
pub type ResourceArn = String;
pub type ResourceArns = Vec<ResourceArn>;
pub type ResourceId = String;
pub type ResourceName = String;
#[doc="<p>A combination of <a>ByteMatchSet</a>, <a>IPSet</a>, and/or <a>SqlInjectionMatchSet</a> objects that identify the web requests that you want to allow, block, or count. For example, you might create a <code>Rule</code> that includes the following predicates:</p> <ul> <li> <p>An <code>IPSet</code> that causes AWS WAF to search for web requests that originate from the IP address <code>192.0.2.44</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that causes AWS WAF to search for web requests for which the value of the <code>User-Agent</code> header is <code>BadBot</code>.</p> </li> </ul> <p>To match the settings in this <code>Rule</code>, a request must originate from <code>192.0.2.44</code> AND include a <code>User-Agent</code> header for which the value is <code>BadBot</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Rule {
    #[doc="<p>A friendly name or description for the metrics for this <code>Rule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change <code>MetricName</code> after you create the <code>Rule</code>.</p>"]
    #[serde(rename="MetricName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_name: Option<MetricName>,
    #[doc="<p>The friendly name or description for the <code>Rule</code>. You can't change the name of a <code>Rule</code> after you create it.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc="<p>The <code>Predicates</code> object contains one <code>Predicate</code> element for each <a>ByteMatchSet</a>, <a>IPSet</a>, or <a>SqlInjectionMatchSet</a> object that you want to include in a <code>Rule</code>.</p>"]
    #[serde(rename="Predicates")]
    pub predicates: Predicates,
    #[doc="<p>A unique identifier for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete a one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
}

pub type RulePriority = i64;
pub type RuleSummaries = Vec<RuleSummary>;
#[doc="<p>Contains the identifier and the friendly name or description of the <code>Rule</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RuleSummary {
    #[doc="<p>A friendly name or description of the <a>Rule</a>. You can't change the name of a <code>Rule</code> after you create it.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
    #[doc="<p>A unique identifier for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
}

#[doc="<p>Specifies a <code>Predicate</code> (such as an <code>IPSet</code>) and indicates whether you want to add it to a <code>Rule</code> or delete it from a <code>Rule</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RuleUpdate {
    #[doc="<p>Specify <code>INSERT</code> to add a <code>Predicate</code> to a <code>Rule</code>. Use <code>DELETE</code> to remove a <code>Predicate</code> from a <code>Rule</code>.</p>"]
    #[serde(rename="Action")]
    pub action: ChangeAction,
    #[doc="<p>The ID of the <code>Predicate</code> (such as an <code>IPSet</code>) that you want to add to a <code>Rule</code>.</p>"]
    #[serde(rename="Predicate")]
    pub predicate: Predicate,
}

pub type RuleUpdates = Vec<RuleUpdate>;
pub type SampleWeight = i64;
#[doc="<p>The response from a <a>GetSampledRequests</a> request includes a <code>SampledHTTPRequests</code> complex type that appears as <code>SampledRequests</code> in the response syntax. <code>SampledHTTPRequests</code> contains one <code>SampledHTTPRequest</code> object for each web request that is returned by <code>GetSampledRequests</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SampledHTTPRequest {
    #[doc="<p>The action for the <code>Rule</code> that the request matched: <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p>"]
    #[serde(rename="Action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<Action>,
    #[doc="<p>A complex type that contains detailed information about the request.</p>"]
    #[serde(rename="Request")]
    pub request: HTTPRequest,
    #[doc="<p>The time at which AWS WAF received the request from your AWS resource, in Unix time format (in seconds).</p>"]
    #[serde(rename="Timestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timestamp: Option<Timestamp>,
    #[doc="<p>A value that indicates how one result in the response relates proportionally to other results in the response. A result that has a weight of <code>2</code> represents roughly twice as many CloudFront web requests as a result that has a weight of <code>1</code>.</p>"]
    #[serde(rename="Weight")]
    pub weight: SampleWeight,
}

pub type SampledHTTPRequests = Vec<SampledHTTPRequest>;
pub type Size = i64;
#[doc="<p>Specifies a constraint on the size of a part of the web request. AWS WAF uses the <code>Size</code>, <code>ComparisonOperator</code>, and <code>FieldToMatch</code> to build an expression in the form of \"<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>\". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SizeConstraint {
    #[doc="<p>The type of comparison you want AWS WAF to perform. AWS WAF uses this in combination with the provided <code>Size</code> and <code>FieldToMatch</code> to build an expression in the form of \"<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>\". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p> <p> <b>EQ</b>: Used to test if the <code>Size</code> is equal to the size of the <code>FieldToMatch</code> </p> <p> <b>NE</b>: Used to test if the <code>Size</code> is not equal to the size of the <code>FieldToMatch</code> </p> <p> <b>LE</b>: Used to test if the <code>Size</code> is less than or equal to the size of the <code>FieldToMatch</code> </p> <p> <b>LT</b>: Used to test if the <code>Size</code> is strictly less than the size of the <code>FieldToMatch</code> </p> <p> <b>GE</b>: Used to test if the <code>Size</code> is greater than or equal to the size of the <code>FieldToMatch</code> </p> <p> <b>GT</b>: Used to test if the <code>Size</code> is strictly greater than the size of the <code>FieldToMatch</code> </p>"]
    #[serde(rename="ComparisonOperator")]
    pub comparison_operator: ComparisonOperator,
    #[doc="<p>Specifies where in a web request to look for the size constraint.</p>"]
    #[serde(rename="FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[doc="<p>The size in bytes that you want AWS WAF to compare against the size of the specified <code>FieldToMatch</code>. AWS WAF uses this in combination with <code>ComparisonOperator</code> and <code>FieldToMatch</code> to build an expression in the form of \"<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>\". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p> <p>Valid values for size are 0 - 21474836480 bytes (0 - 20 GB).</p> <p>If you specify <code>URI</code> for the value of <code>Type</code>, the / in the URI counts as one character. For example, the URI <code>/logo.jpg</code> is nine characters long.</p>"]
    #[serde(rename="Size")]
    pub size: Size,
    #[doc="<p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting a request for a match.</p> <p>Note that if you choose <code>BODY</code> for the value of <code>Type</code>, you must choose <code>NONE</code> for <code>TextTransformation</code> because CloudFront forwards only the first 8192 bytes for inspection. </p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \\ \" ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\\f, formfeed, decimal 12</p> </li> <li> <p>\\t, tab, decimal 9</p> </li> <li> <p>\\n, newline, decimal 10</p> </li> <li> <p>\\r, carriage return, decimal 13</p> </li> <li> <p>\\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>\"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a \"less than\" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p>"]
    #[serde(rename="TextTransformation")]
    pub text_transformation: TextTransformation,
}

#[doc="<p>A complex type that contains <code>SizeConstraint</code> objects, which specify the parts of web requests that you want AWS WAF to inspect the size of. If a <code>SizeConstraintSet</code> contains more than one <code>SizeConstraint</code> object, a request only needs to match one constraint to be considered a match.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SizeConstraintSet {
    #[doc="<p>The name, if any, of the <code>SizeConstraintSet</code>.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc="<p>A unique identifier for a <code>SizeConstraintSet</code>. You use <code>SizeConstraintSetId</code> to get information about a <code>SizeConstraintSet</code> (see <a>GetSizeConstraintSet</a>), update a <code>SizeConstraintSet</code> (see <a>UpdateSizeConstraintSet</a>), insert a <code>SizeConstraintSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SizeConstraintSet</code> from AWS WAF (see <a>DeleteSizeConstraintSet</a>).</p> <p> <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>"]
    #[serde(rename="SizeConstraintSetId")]
    pub size_constraint_set_id: ResourceId,
    #[doc="<p>Specifies the parts of web requests that you want to inspect the size of.</p>"]
    #[serde(rename="SizeConstraints")]
    pub size_constraints: SizeConstraints,
}

pub type SizeConstraintSetSummaries = Vec<SizeConstraintSetSummary>;
#[doc="<p>The <code>Id</code> and <code>Name</code> of a <code>SizeConstraintSet</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SizeConstraintSetSummary {
    #[doc="<p>The name of the <code>SizeConstraintSet</code>, if any.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
    #[doc="<p>A unique identifier for a <code>SizeConstraintSet</code>. You use <code>SizeConstraintSetId</code> to get information about a <code>SizeConstraintSet</code> (see <a>GetSizeConstraintSet</a>), update a <code>SizeConstraintSet</code> (see <a>UpdateSizeConstraintSet</a>), insert a <code>SizeConstraintSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SizeConstraintSet</code> from AWS WAF (see <a>DeleteSizeConstraintSet</a>).</p> <p> <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>"]
    #[serde(rename="SizeConstraintSetId")]
    pub size_constraint_set_id: ResourceId,
}

#[doc="<p>Specifies the part of a web request that you want to inspect the size of and indicates whether you want to add the specification to a <a>SizeConstraintSet</a> or delete it from a <code>SizeConstraintSet</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SizeConstraintSetUpdate {
    #[doc="<p>Specify <code>INSERT</code> to add a <a>SizeConstraintSetUpdate</a> to a <a>SizeConstraintSet</a>. Use <code>DELETE</code> to remove a <code>SizeConstraintSetUpdate</code> from a <code>SizeConstraintSet</code>.</p>"]
    #[serde(rename="Action")]
    pub action: ChangeAction,
    #[doc="<p>Specifies a constraint on the size of a part of the web request. AWS WAF uses the <code>Size</code>, <code>ComparisonOperator</code>, and <code>FieldToMatch</code> to build an expression in the form of \"<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>\". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p>"]
    #[serde(rename="SizeConstraint")]
    pub size_constraint: SizeConstraint,
}

pub type SizeConstraintSetUpdates = Vec<SizeConstraintSetUpdate>;
pub type SizeConstraints = Vec<SizeConstraint>;
#[doc="<p>A complex type that contains <code>SqlInjectionMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header. If a <code>SqlInjectionMatchSet</code> contains more than one <code>SqlInjectionMatchTuple</code> object, a request needs to include snippets of SQL code in only one of the specified parts of the request to be considered a match.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SqlInjectionMatchSet {
    #[doc="<p>The name, if any, of the <code>SqlInjectionMatchSet</code>.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc="<p>A unique identifier for a <code>SqlInjectionMatchSet</code>. You use <code>SqlInjectionMatchSetId</code> to get information about a <code>SqlInjectionMatchSet</code> (see <a>GetSqlInjectionMatchSet</a>), update a <code>SqlInjectionMatchSet</code> (see <a>UpdateSqlInjectionMatchSet</a>), insert a <code>SqlInjectionMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SqlInjectionMatchSet</code> from AWS WAF (see <a>DeleteSqlInjectionMatchSet</a>).</p> <p> <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>"]
    #[serde(rename="SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: ResourceId,
    #[doc="<p>Specifies the parts of web requests that you want to inspect for snippets of malicious SQL code.</p>"]
    #[serde(rename="SqlInjectionMatchTuples")]
    pub sql_injection_match_tuples: SqlInjectionMatchTuples,
}

pub type SqlInjectionMatchSetSummaries = Vec<SqlInjectionMatchSetSummary>;
#[doc="<p>The <code>Id</code> and <code>Name</code> of a <code>SqlInjectionMatchSet</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SqlInjectionMatchSetSummary {
    #[doc="<p>The name of the <code>SqlInjectionMatchSet</code>, if any, specified by <code>Id</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
    #[doc="<p>A unique identifier for a <code>SqlInjectionMatchSet</code>. You use <code>SqlInjectionMatchSetId</code> to get information about a <code>SqlInjectionMatchSet</code> (see <a>GetSqlInjectionMatchSet</a>), update a <code>SqlInjectionMatchSet</code> (see <a>UpdateSqlInjectionMatchSet</a>), insert a <code>SqlInjectionMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SqlInjectionMatchSet</code> from AWS WAF (see <a>DeleteSqlInjectionMatchSet</a>).</p> <p> <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>"]
    #[serde(rename="SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: ResourceId,
}

#[doc="<p>Specifies the part of a web request that you want to inspect for snippets of malicious SQL code and indicates whether you want to add the specification to a <a>SqlInjectionMatchSet</a> or delete it from a <code>SqlInjectionMatchSet</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SqlInjectionMatchSetUpdate {
    #[doc="<p>Specify <code>INSERT</code> to add a <a>SqlInjectionMatchSetUpdate</a> to a <a>SqlInjectionMatchSet</a>. Use <code>DELETE</code> to remove a <code>SqlInjectionMatchSetUpdate</code> from a <code>SqlInjectionMatchSet</code>.</p>"]
    #[serde(rename="Action")]
    pub action: ChangeAction,
    #[doc="<p>Specifies the part of a web request that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.</p>"]
    #[serde(rename="SqlInjectionMatchTuple")]
    pub sql_injection_match_tuple: SqlInjectionMatchTuple,
}

pub type SqlInjectionMatchSetUpdates = Vec<SqlInjectionMatchSetUpdate>;
#[doc="<p>Specifies the part of a web request that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SqlInjectionMatchTuple {
    #[doc="<p>Specifies where in a web request to look for snippets of malicious SQL code.</p>"]
    #[serde(rename="FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[doc="<p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting a request for a match.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system commandline command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \\ \" ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\\f, formfeed, decimal 12</p> </li> <li> <p>\\t, tab, decimal 9</p> </li> <li> <p>\\n, newline, decimal 10</p> </li> <li> <p>\\r, carriage return, decimal 13</p> </li> <li> <p>\\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>\"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a \"less than\" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>"]
    #[serde(rename="TextTransformation")]
    pub text_transformation: TextTransformation,
}

pub type SqlInjectionMatchTuples = Vec<SqlInjectionMatchTuple>;
pub type TextTransformation = String;
#[doc="<p>In a <a>GetSampledRequests</a> request, the <code>StartTime</code> and <code>EndTime</code> objects specify the time range for which you want AWS WAF to return a sample of web requests.</p> <p>In a <a>GetSampledRequests</a> response, the <code>StartTime</code> and <code>EndTime</code> objects specify the time range for which AWS WAF actually returned a sample of web requests. AWS WAF gets the specified number of requests from among the first 5,000 requests that your AWS resource receives during the specified time period. If your resource receives more than 5,000 requests during that period, AWS WAF stops sampling after the 5,000th request. In that case, <code>EndTime</code> is the time that AWS WAF received the 5,000th request. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct TimeWindow {
    #[doc="<p>The end of the time range from which you want <code>GetSampledRequests</code> to return a sample of the requests that your AWS resource received. Specify the date and time in the following format: <code>\"2016-09-27T14:50Z\"</code>. You can specify any time range in the previous three hours.</p>"]
    #[serde(rename="EndTime")]
    pub end_time: Timestamp,
    #[doc="<p>The beginning of the time range from which you want <code>GetSampledRequests</code> to return a sample of the requests that your AWS resource received. Specify the date and time in the following format: <code>\"2016-09-27T14:50Z\"</code>. You can specify any time range in the previous three hours.</p>"]
    #[serde(rename="StartTime")]
    pub start_time: Timestamp,
}

pub type Timestamp = f64;
pub type URIString = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateByteMatchSetRequest {
    #[doc="<p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to update. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>"]
    #[serde(rename="ByteMatchSetId")]
    pub byte_match_set_id: ResourceId,
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>An array of <code>ByteMatchSetUpdate</code> objects that you want to insert into or delete from a <a>ByteMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>ByteMatchSetUpdate</a>: Contains <code>Action</code> and <code>ByteMatchTuple</code> </p> </li> <li> <p> <a>ByteMatchTuple</a>: Contains <code>FieldToMatch</code>, <code>PositionalConstraint</code>, <code>TargetString</code>, and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="Updates")]
    pub updates: ByteMatchSetUpdates,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateByteMatchSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>UpdateByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateIPSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to update. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>"]
    #[serde(rename="IPSetId")]
    pub ip_set_id: ResourceId,
    #[doc="<p>An array of <code>IPSetUpdate</code> objects that you want to insert into or delete from an <a>IPSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>IPSetUpdate</a>: Contains <code>Action</code> and <code>IPSetDescriptor</code> </p> </li> <li> <p> <a>IPSetDescriptor</a>: Contains <code>Type</code> and <code>Value</code> </p> </li> </ul>"]
    #[serde(rename="Updates")]
    pub updates: IPSetUpdates,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateIPSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>UpdateIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateRateBasedRuleRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The maximum number of requests, which have an identical value in the field specified by the <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>"]
    #[serde(rename="RateLimit")]
    pub rate_limit: RateLimit,
    #[doc="<p>The <code>RuleId</code> of the <code>RateBasedRule</code> that you want to update. <code>RuleId</code> is returned by <code>CreateRateBasedRule</code> and by <a>ListRateBasedRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
    #[doc="<p>An array of <code>RuleUpdate</code> objects that you want to insert into or delete from a <a>RateBasedRule</a>. </p>"]
    #[serde(rename="Updates")]
    pub updates: RuleUpdates,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateRateBasedRuleResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateRuleRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>RuleId</code> of the <code>Rule</code> that you want to update. <code>RuleId</code> is returned by <code>CreateRule</code> and by <a>ListRules</a>.</p>"]
    #[serde(rename="RuleId")]
    pub rule_id: ResourceId,
    #[doc="<p>An array of <code>RuleUpdate</code> objects that you want to insert into or delete from a <a>Rule</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>RuleUpdate</a>: Contains <code>Action</code> and <code>Predicate</code> </p> </li> <li> <p> <a>Predicate</a>: Contains <code>DataId</code>, <code>Negated</code>, and <code>Type</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="Updates")]
    pub updates: RuleUpdates,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateRuleResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateSizeConstraintSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to update. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>"]
    #[serde(rename="SizeConstraintSetId")]
    pub size_constraint_set_id: ResourceId,
    #[doc="<p>An array of <code>SizeConstraintSetUpdate</code> objects that you want to insert into or delete from a <a>SizeConstraintSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>SizeConstraintSetUpdate</a>: Contains <code>Action</code> and <code>SizeConstraint</code> </p> </li> <li> <p> <a>SizeConstraint</a>: Contains <code>FieldToMatch</code>, <code>TextTransformation</code>, <code>ComparisonOperator</code>, and <code>Size</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="Updates")]
    pub updates: SizeConstraintSetUpdates,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateSizeConstraintSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>UpdateSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[doc="<p>A request to update a <a>SqlInjectionMatchSet</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateSqlInjectionMatchSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>The <code>SqlInjectionMatchSetId</code> of the <code>SqlInjectionMatchSet</code> that you want to update. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>"]
    #[serde(rename="SqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: ResourceId,
    #[doc="<p>An array of <code>SqlInjectionMatchSetUpdate</code> objects that you want to insert into or delete from a <a>SqlInjectionMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>SqlInjectionMatchSetUpdate</a>: Contains <code>Action</code> and <code>SqlInjectionMatchTuple</code> </p> </li> <li> <p> <a>SqlInjectionMatchTuple</a>: Contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="Updates")]
    pub updates: SqlInjectionMatchSetUpdates,
}

#[doc="<p>The response to an <a>UpdateSqlInjectionMatchSets</a> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateSqlInjectionMatchSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>UpdateSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateWebACLRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>A default action for the web ACL, either ALLOW or BLOCK. AWS WAF performs the default action if a request doesn't match the criteria in any of the rules in a web ACL.</p>"]
    #[serde(rename="DefaultAction")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_action: Option<WafAction>,
    #[doc="<p>An array of updates to make to the <a>WebACL</a>.</p> <p>An array of <code>WebACLUpdate</code> objects that you want to insert into or delete from a <a>WebACL</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>WebACLUpdate</a>: Contains <code>Action</code> and <code>ActivatedRule</code> </p> </li> <li> <p> <a>ActivatedRule</a>: Contains <code>Action</code>, <code>Priority</code>, <code>RuleId</code>, and <code>Type</code> </p> </li> <li> <p> <a>WafAction</a>: Contains <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="Updates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updates: Option<WebACLUpdates>,
    #[doc="<p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to update. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>"]
    #[serde(rename="WebACLId")]
    pub web_acl_id: ResourceId,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateWebACLResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>UpdateWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[doc="<p>A request to update an <a>XssMatchSet</a>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateXssMatchSetRequest {
    #[doc="<p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>"]
    #[serde(rename="ChangeToken")]
    pub change_token: ChangeToken,
    #[doc="<p>An array of <code>XssMatchSetUpdate</code> objects that you want to insert into or delete from a <a>XssMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>XssMatchSetUpdate</a>: Contains <code>Action</code> and <code>XssMatchTuple</code> </p> </li> <li> <p> <a>XssMatchTuple</a>: Contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul>"]
    #[serde(rename="Updates")]
    pub updates: XssMatchSetUpdates,
    #[doc="<p>The <code>XssMatchSetId</code> of the <code>XssMatchSet</code> that you want to update. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>"]
    #[serde(rename="XssMatchSetId")]
    pub xss_match_set_id: ResourceId,
}

#[doc="<p>The response to an <a>UpdateXssMatchSets</a> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateXssMatchSetResponse {
    #[doc="<p>The <code>ChangeToken</code> that you used to submit the <code>UpdateXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>"]
    #[serde(rename="ChangeToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_token: Option<ChangeToken>,
}

#[doc="<p>For the action that is associated with a rule in a <code>WebACL</code>, specifies the action that you want AWS WAF to perform when a web request matches all of the conditions in a rule. For the default action in a <code>WebACL</code>, specifies the action that you want AWS WAF to take when a web request doesn't match all of the conditions in any of the rules in a <code>WebACL</code>. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct WafAction {
    #[doc="<p>Specifies how you want AWS WAF to respond to requests that match the settings in a <code>Rule</code>. Valid settings include the following:</p> <ul> <li> <p> <code>ALLOW</code>: AWS WAF allows requests</p> </li> <li> <p> <code>BLOCK</code>: AWS WAF blocks requests</p> </li> <li> <p> <code>COUNT</code>: AWS WAF increments a counter of the requests that match all of the conditions in the rule. AWS WAF then continues to inspect the web request based on the remaining rules in the web ACL. You can't specify <code>COUNT</code> for the default action for a <code>WebACL</code>.</p> </li> </ul>"]
    #[serde(rename="Type")]
    pub type_: WafActionType,
}

pub type WafActionType = String;
pub type WafRuleType = String;
#[doc="<p>Contains the <code>Rules</code> that identify the requests that you want to allow, block, or count. In a <code>WebACL</code>, you also specify a default action (<code>ALLOW</code> or <code>BLOCK</code>), and the action for each <code>Rule</code> that you add to a <code>WebACL</code>, for example, block requests from specified IP addresses or block requests from specified referrers. You also associate the <code>WebACL</code> with a CloudFront distribution to identify the requests that you want AWS WAF to filter. If you add more than one <code>Rule</code> to a <code>WebACL</code>, a request needs to match only one of the specifications to be allowed, blocked, or counted. For more information, see <a>UpdateWebACL</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct WebACL {
    #[doc="<p>The action to perform if none of the <code>Rules</code> contained in the <code>WebACL</code> match. The action is specified by the <a>WafAction</a> object.</p>"]
    #[serde(rename="DefaultAction")]
    pub default_action: WafAction,
    #[doc="<p>A friendly name or description for the metrics for this <code>WebACL</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace. You can't change <code>MetricName</code> after you create the <code>WebACL</code>.</p>"]
    #[serde(rename="MetricName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric_name: Option<MetricName>,
    #[doc="<p>A friendly name or description of the <code>WebACL</code>. You can't change the name of a <code>WebACL</code> after you create it.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc="<p>An array that contains the action for each <code>Rule</code> in a <code>WebACL</code>, the priority of the <code>Rule</code>, and the ID of the <code>Rule</code>.</p>"]
    #[serde(rename="Rules")]
    pub rules: ActivatedRules,
    #[doc="<p>A unique identifier for a <code>WebACL</code>. You use <code>WebACLId</code> to get information about a <code>WebACL</code> (see <a>GetWebACL</a>), update a <code>WebACL</code> (see <a>UpdateWebACL</a>), and delete a <code>WebACL</code> from AWS WAF (see <a>DeleteWebACL</a>).</p> <p> <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>"]
    #[serde(rename="WebACLId")]
    pub web_acl_id: ResourceId,
}

pub type WebACLSummaries = Vec<WebACLSummary>;
#[doc="<p>Contains the identifier and the name or description of the <a>WebACL</a>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct WebACLSummary {
    #[doc="<p>A friendly name or description of the <a>WebACL</a>. You can't change the name of a <code>WebACL</code> after you create it.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
    #[doc="<p>A unique identifier for a <code>WebACL</code>. You use <code>WebACLId</code> to get information about a <code>WebACL</code> (see <a>GetWebACL</a>), update a <code>WebACL</code> (see <a>UpdateWebACL</a>), and delete a <code>WebACL</code> from AWS WAF (see <a>DeleteWebACL</a>).</p> <p> <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>"]
    #[serde(rename="WebACLId")]
    pub web_acl_id: ResourceId,
}

#[doc="<p>Specifies whether to insert a <code>Rule</code> into or delete a <code>Rule</code> from a <code>WebACL</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct WebACLUpdate {
    #[doc="<p>Specifies whether to insert a <code>Rule</code> into or delete a <code>Rule</code> from a <code>WebACL</code>.</p>"]
    #[serde(rename="Action")]
    pub action: ChangeAction,
    #[doc="<p>The <code>ActivatedRule</code> object in an <a>UpdateWebACL</a> request specifies a <code>Rule</code> that you want to insert or delete, the priority of the <code>Rule</code> in the <code>WebACL</code>, and the action that you want AWS WAF to take when a web request matches the <code>Rule</code> (<code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>).</p>"]
    #[serde(rename="ActivatedRule")]
    pub activated_rule: ActivatedRule,
}

pub type WebACLUpdates = Vec<WebACLUpdate>;
#[doc="<p>A complex type that contains <code>XssMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header. If a <code>XssMatchSet</code> contains more than one <code>XssMatchTuple</code> object, a request needs to include cross-site scripting attacks in only one of the specified parts of the request to be considered a match.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct XssMatchSet {
    #[doc="<p>The name, if any, of the <code>XssMatchSet</code>.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc="<p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about an <code>XssMatchSet</code> (see <a>GetXssMatchSet</a>), update an <code>XssMatchSet</code> (see <a>UpdateXssMatchSet</a>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>XssMatchSet</code> from AWS WAF (see <a>DeleteXssMatchSet</a>).</p> <p> <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>"]
    #[serde(rename="XssMatchSetId")]
    pub xss_match_set_id: ResourceId,
    #[doc="<p>Specifies the parts of web requests that you want to inspect for cross-site scripting attacks.</p>"]
    #[serde(rename="XssMatchTuples")]
    pub xss_match_tuples: XssMatchTuples,
}

pub type XssMatchSetSummaries = Vec<XssMatchSetSummary>;
#[doc="<p>The <code>Id</code> and <code>Name</code> of an <code>XssMatchSet</code>.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct XssMatchSetSummary {
    #[doc="<p>The name of the <code>XssMatchSet</code>, if any, specified by <code>Id</code>.</p>"]
    #[serde(rename="Name")]
    pub name: ResourceName,
    #[doc="<p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about a <code>XssMatchSet</code> (see <a>GetXssMatchSet</a>), update an <code>XssMatchSet</code> (see <a>UpdateXssMatchSet</a>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>XssMatchSet</code> from AWS WAF (see <a>DeleteXssMatchSet</a>).</p> <p> <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>"]
    #[serde(rename="XssMatchSetId")]
    pub xss_match_set_id: ResourceId,
}

#[doc="<p>Specifies the part of a web request that you want to inspect for cross-site scripting attacks and indicates whether you want to add the specification to an <a>XssMatchSet</a> or delete it from an <code>XssMatchSet</code>.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct XssMatchSetUpdate {
    #[doc="<p>Specify <code>INSERT</code> to add a <a>XssMatchSetUpdate</a> to an <a>XssMatchSet</a>. Use <code>DELETE</code> to remove a <code>XssMatchSetUpdate</code> from an <code>XssMatchSet</code>.</p>"]
    #[serde(rename="Action")]
    pub action: ChangeAction,
    #[doc="<p>Specifies the part of a web request that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header.</p>"]
    #[serde(rename="XssMatchTuple")]
    pub xss_match_tuple: XssMatchTuple,
}

pub type XssMatchSetUpdates = Vec<XssMatchSetUpdate>;
#[doc="<p>Specifies the part of a web request that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct XssMatchTuple {
    #[doc="<p>Specifies where in a web request to look for cross-site scripting attacks.</p>"]
    #[serde(rename="FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[doc="<p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting a request for a match.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system commandline command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \\ \" ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\\f, formfeed, decimal 12</p> </li> <li> <p>\\t, tab, decimal 9</p> </li> <li> <p>\\n, newline, decimal 10</p> </li> <li> <p>\\r, carriage return, decimal 13</p> </li> <li> <p>\\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>\"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a \"less than\" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>"]
    #[serde(rename="TextTransformation")]
    pub text_transformation: TextTransformation,
}

pub type XssMatchTuples = Vec<XssMatchTuple>;
/// Errors returned by AssociateWebACL
#[derive(Debug, PartialEq)]
pub enum AssociateWebACLError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because the entity referenced is temporarily unavailable. Retry your request.</p>
    WAFUnavailableEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AssociateWebACLError {
    pub fn from_body(body: &str) -> AssociateWebACLError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        AssociateWebACLError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        AssociateWebACLError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        AssociateWebACLError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        AssociateWebACLError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFUnavailableEntityException" => {
                        AssociateWebACLError::WAFUnavailableEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateWebACLError::Validation(error_message.to_string())
                    }
                    _ => AssociateWebACLError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateWebACLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateWebACLError {
    fn from(err: serde_json::error::Error) -> AssociateWebACLError {
        AssociateWebACLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateWebACLError {
    fn from(err: CredentialsError) -> AssociateWebACLError {
        AssociateWebACLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateWebACLError {
    fn from(err: HttpDispatchError) -> AssociateWebACLError {
        AssociateWebACLError::HttpDispatch(err)
    }
}
impl fmt::Display for AssociateWebACLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateWebACLError {
    fn description(&self) -> &str {
        match *self {
            AssociateWebACLError::WAFInternalError(ref cause) => cause,
            AssociateWebACLError::WAFInvalidAccount(ref cause) => cause,
            AssociateWebACLError::WAFInvalidParameter(ref cause) => cause,
            AssociateWebACLError::WAFNonexistentItem(ref cause) => cause,
            AssociateWebACLError::WAFUnavailableEntity(ref cause) => cause,
            AssociateWebACLError::Validation(ref cause) => cause,
            AssociateWebACLError::Credentials(ref err) => err.description(),
            AssociateWebACLError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AssociateWebACLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateByteMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateByteMatchSetError {
    ///<p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateByteMatchSetError {
    pub fn from_body(body: &str) -> CreateByteMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFDisallowedNameException" => {
                        CreateByteMatchSetError::WAFDisallowedName(String::from(error_message))
                    }
                    "WAFInternalErrorException" => {
                        CreateByteMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        CreateByteMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        CreateByteMatchSetError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        CreateByteMatchSetError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        CreateByteMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateByteMatchSetError::Validation(error_message.to_string())
                    }
                    _ => CreateByteMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateByteMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateByteMatchSetError {
    fn from(err: serde_json::error::Error) -> CreateByteMatchSetError {
        CreateByteMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateByteMatchSetError {
    fn from(err: CredentialsError) -> CreateByteMatchSetError {
        CreateByteMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateByteMatchSetError {
    fn from(err: HttpDispatchError) -> CreateByteMatchSetError {
        CreateByteMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateByteMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateByteMatchSetError {
    fn description(&self) -> &str {
        match *self {
            CreateByteMatchSetError::WAFDisallowedName(ref cause) => cause,
            CreateByteMatchSetError::WAFInternalError(ref cause) => cause,
            CreateByteMatchSetError::WAFInvalidAccount(ref cause) => cause,
            CreateByteMatchSetError::WAFInvalidParameter(ref cause) => cause,
            CreateByteMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            CreateByteMatchSetError::WAFStaleData(ref cause) => cause,
            CreateByteMatchSetError::Validation(ref cause) => cause,
            CreateByteMatchSetError::Credentials(ref err) => err.description(),
            CreateByteMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateByteMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateIPSet
#[derive(Debug, PartialEq)]
pub enum CreateIPSetError {
    ///<p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateIPSetError {
    pub fn from_body(body: &str) -> CreateIPSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFDisallowedNameException" => {
                        CreateIPSetError::WAFDisallowedName(String::from(error_message))
                    }
                    "WAFInternalErrorException" => {
                        CreateIPSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        CreateIPSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        CreateIPSetError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        CreateIPSetError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        CreateIPSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateIPSetError::Validation(error_message.to_string())
                    }
                    _ => CreateIPSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateIPSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateIPSetError {
    fn from(err: serde_json::error::Error) -> CreateIPSetError {
        CreateIPSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateIPSetError {
    fn from(err: CredentialsError) -> CreateIPSetError {
        CreateIPSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateIPSetError {
    fn from(err: HttpDispatchError) -> CreateIPSetError {
        CreateIPSetError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateIPSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateIPSetError {
    fn description(&self) -> &str {
        match *self {
            CreateIPSetError::WAFDisallowedName(ref cause) => cause,
            CreateIPSetError::WAFInternalError(ref cause) => cause,
            CreateIPSetError::WAFInvalidAccount(ref cause) => cause,
            CreateIPSetError::WAFInvalidParameter(ref cause) => cause,
            CreateIPSetError::WAFLimitsExceeded(ref cause) => cause,
            CreateIPSetError::WAFStaleData(ref cause) => cause,
            CreateIPSetError::Validation(ref cause) => cause,
            CreateIPSetError::Credentials(ref err) => err.description(),
            CreateIPSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateIPSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRateBasedRule
#[derive(Debug, PartialEq)]
pub enum CreateRateBasedRuleError {
    ///<p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateRateBasedRuleError {
    pub fn from_body(body: &str) -> CreateRateBasedRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFDisallowedNameException" => {
                        CreateRateBasedRuleError::WAFDisallowedName(String::from(error_message))
                    }
                    "WAFInternalErrorException" => {
                        CreateRateBasedRuleError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        CreateRateBasedRuleError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        CreateRateBasedRuleError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        CreateRateBasedRuleError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateRateBasedRuleError::Validation(error_message.to_string())
                    }
                    _ => CreateRateBasedRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRateBasedRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRateBasedRuleError {
    fn from(err: serde_json::error::Error) -> CreateRateBasedRuleError {
        CreateRateBasedRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRateBasedRuleError {
    fn from(err: CredentialsError) -> CreateRateBasedRuleError {
        CreateRateBasedRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRateBasedRuleError {
    fn from(err: HttpDispatchError) -> CreateRateBasedRuleError {
        CreateRateBasedRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateRateBasedRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRateBasedRuleError {
    fn description(&self) -> &str {
        match *self {
            CreateRateBasedRuleError::WAFDisallowedName(ref cause) => cause,
            CreateRateBasedRuleError::WAFInternalError(ref cause) => cause,
            CreateRateBasedRuleError::WAFInvalidParameter(ref cause) => cause,
            CreateRateBasedRuleError::WAFLimitsExceeded(ref cause) => cause,
            CreateRateBasedRuleError::WAFStaleData(ref cause) => cause,
            CreateRateBasedRuleError::Validation(ref cause) => cause,
            CreateRateBasedRuleError::Credentials(ref err) => err.description(),
            CreateRateBasedRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateRateBasedRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRule
#[derive(Debug, PartialEq)]
pub enum CreateRuleError {
    ///<p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateRuleError {
    pub fn from_body(body: &str) -> CreateRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFDisallowedNameException" => {
                        CreateRuleError::WAFDisallowedName(String::from(error_message))
                    }
                    "WAFInternalErrorException" => {
                        CreateRuleError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        CreateRuleError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        CreateRuleError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        CreateRuleError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => CreateRuleError::Validation(error_message.to_string()),
                    _ => CreateRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRuleError {
    fn from(err: serde_json::error::Error) -> CreateRuleError {
        CreateRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRuleError {
    fn from(err: CredentialsError) -> CreateRuleError {
        CreateRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRuleError {
    fn from(err: HttpDispatchError) -> CreateRuleError {
        CreateRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRuleError {
    fn description(&self) -> &str {
        match *self {
            CreateRuleError::WAFDisallowedName(ref cause) => cause,
            CreateRuleError::WAFInternalError(ref cause) => cause,
            CreateRuleError::WAFInvalidParameter(ref cause) => cause,
            CreateRuleError::WAFLimitsExceeded(ref cause) => cause,
            CreateRuleError::WAFStaleData(ref cause) => cause,
            CreateRuleError::Validation(ref cause) => cause,
            CreateRuleError::Credentials(ref err) => err.description(),
            CreateRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum CreateSizeConstraintSetError {
    ///<p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateSizeConstraintSetError {
    pub fn from_body(body: &str) -> CreateSizeConstraintSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFDisallowedNameException" => {
                        CreateSizeConstraintSetError::WAFDisallowedName(String::from(error_message))
                    }
                    "WAFInternalErrorException" => {
                        CreateSizeConstraintSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        CreateSizeConstraintSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => CreateSizeConstraintSetError::WAFInvalidParameter(String::from(error_message)),
                    "WAFLimitsExceededException" => {
                        CreateSizeConstraintSetError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        CreateSizeConstraintSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSizeConstraintSetError::Validation(error_message.to_string())
                    }
                    _ => CreateSizeConstraintSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSizeConstraintSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSizeConstraintSetError {
    fn from(err: serde_json::error::Error) -> CreateSizeConstraintSetError {
        CreateSizeConstraintSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSizeConstraintSetError {
    fn from(err: CredentialsError) -> CreateSizeConstraintSetError {
        CreateSizeConstraintSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSizeConstraintSetError {
    fn from(err: HttpDispatchError) -> CreateSizeConstraintSetError {
        CreateSizeConstraintSetError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateSizeConstraintSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSizeConstraintSetError {
    fn description(&self) -> &str {
        match *self {
            CreateSizeConstraintSetError::WAFDisallowedName(ref cause) => cause,
            CreateSizeConstraintSetError::WAFInternalError(ref cause) => cause,
            CreateSizeConstraintSetError::WAFInvalidAccount(ref cause) => cause,
            CreateSizeConstraintSetError::WAFInvalidParameter(ref cause) => cause,
            CreateSizeConstraintSetError::WAFLimitsExceeded(ref cause) => cause,
            CreateSizeConstraintSetError::WAFStaleData(ref cause) => cause,
            CreateSizeConstraintSetError::Validation(ref cause) => cause,
            CreateSizeConstraintSetError::Credentials(ref err) => err.description(),
            CreateSizeConstraintSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSizeConstraintSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateSqlInjectionMatchSetError {
    ///<p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateSqlInjectionMatchSetError {
    pub fn from_body(body: &str) -> CreateSqlInjectionMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFDisallowedNameException" => CreateSqlInjectionMatchSetError::WAFDisallowedName(String::from(error_message)),
                    "WAFInternalErrorException" => CreateSqlInjectionMatchSetError::WAFInternalError(String::from(error_message)),
                    "WAFInvalidAccountException" => CreateSqlInjectionMatchSetError::WAFInvalidAccount(String::from(error_message)),
                    "WAFInvalidParameterException" => CreateSqlInjectionMatchSetError::WAFInvalidParameter(String::from(error_message)),
                    "WAFLimitsExceededException" => CreateSqlInjectionMatchSetError::WAFLimitsExceeded(String::from(error_message)),
                    "WAFStaleDataException" => {
                        CreateSqlInjectionMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateSqlInjectionMatchSetError::Validation(error_message.to_string())
                    }
                    _ => CreateSqlInjectionMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSqlInjectionMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateSqlInjectionMatchSetError {
    fn from(err: serde_json::error::Error) -> CreateSqlInjectionMatchSetError {
        CreateSqlInjectionMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateSqlInjectionMatchSetError {
    fn from(err: CredentialsError) -> CreateSqlInjectionMatchSetError {
        CreateSqlInjectionMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSqlInjectionMatchSetError {
    fn from(err: HttpDispatchError) -> CreateSqlInjectionMatchSetError {
        CreateSqlInjectionMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateSqlInjectionMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSqlInjectionMatchSetError {
    fn description(&self) -> &str {
        match *self {
            CreateSqlInjectionMatchSetError::WAFDisallowedName(ref cause) => cause,
            CreateSqlInjectionMatchSetError::WAFInternalError(ref cause) => cause,
            CreateSqlInjectionMatchSetError::WAFInvalidAccount(ref cause) => cause,
            CreateSqlInjectionMatchSetError::WAFInvalidParameter(ref cause) => cause,
            CreateSqlInjectionMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            CreateSqlInjectionMatchSetError::WAFStaleData(ref cause) => cause,
            CreateSqlInjectionMatchSetError::Validation(ref cause) => cause,
            CreateSqlInjectionMatchSetError::Credentials(ref err) => err.description(),
            CreateSqlInjectionMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateSqlInjectionMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateWebACL
#[derive(Debug, PartialEq)]
pub enum CreateWebACLError {
    ///<p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateWebACLError {
    pub fn from_body(body: &str) -> CreateWebACLError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFDisallowedNameException" => {
                        CreateWebACLError::WAFDisallowedName(String::from(error_message))
                    }
                    "WAFInternalErrorException" => {
                        CreateWebACLError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        CreateWebACLError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        CreateWebACLError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        CreateWebACLError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        CreateWebACLError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateWebACLError::Validation(error_message.to_string())
                    }
                    _ => CreateWebACLError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateWebACLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateWebACLError {
    fn from(err: serde_json::error::Error) -> CreateWebACLError {
        CreateWebACLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateWebACLError {
    fn from(err: CredentialsError) -> CreateWebACLError {
        CreateWebACLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateWebACLError {
    fn from(err: HttpDispatchError) -> CreateWebACLError {
        CreateWebACLError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateWebACLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateWebACLError {
    fn description(&self) -> &str {
        match *self {
            CreateWebACLError::WAFDisallowedName(ref cause) => cause,
            CreateWebACLError::WAFInternalError(ref cause) => cause,
            CreateWebACLError::WAFInvalidAccount(ref cause) => cause,
            CreateWebACLError::WAFInvalidParameter(ref cause) => cause,
            CreateWebACLError::WAFLimitsExceeded(ref cause) => cause,
            CreateWebACLError::WAFStaleData(ref cause) => cause,
            CreateWebACLError::Validation(ref cause) => cause,
            CreateWebACLError::Credentials(ref err) => err.description(),
            CreateWebACLError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateWebACLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateXssMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateXssMatchSetError {
    ///<p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateXssMatchSetError {
    pub fn from_body(body: &str) -> CreateXssMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFDisallowedNameException" => {
                        CreateXssMatchSetError::WAFDisallowedName(String::from(error_message))
                    }
                    "WAFInternalErrorException" => {
                        CreateXssMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        CreateXssMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        CreateXssMatchSetError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        CreateXssMatchSetError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        CreateXssMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateXssMatchSetError::Validation(error_message.to_string())
                    }
                    _ => CreateXssMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateXssMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateXssMatchSetError {
    fn from(err: serde_json::error::Error) -> CreateXssMatchSetError {
        CreateXssMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateXssMatchSetError {
    fn from(err: CredentialsError) -> CreateXssMatchSetError {
        CreateXssMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateXssMatchSetError {
    fn from(err: HttpDispatchError) -> CreateXssMatchSetError {
        CreateXssMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateXssMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateXssMatchSetError {
    fn description(&self) -> &str {
        match *self {
            CreateXssMatchSetError::WAFDisallowedName(ref cause) => cause,
            CreateXssMatchSetError::WAFInternalError(ref cause) => cause,
            CreateXssMatchSetError::WAFInvalidAccount(ref cause) => cause,
            CreateXssMatchSetError::WAFInvalidParameter(ref cause) => cause,
            CreateXssMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            CreateXssMatchSetError::WAFStaleData(ref cause) => cause,
            CreateXssMatchSetError::Validation(ref cause) => cause,
            CreateXssMatchSetError::Credentials(ref err) => err.description(),
            CreateXssMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateXssMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteByteMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteByteMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because you tried to delete an object that isn't empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul>
    WAFNonEmptyEntity(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteByteMatchSetError {
    pub fn from_body(body: &str) -> DeleteByteMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        DeleteByteMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        DeleteByteMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonEmptyEntityException" => {
                        DeleteByteMatchSetError::WAFNonEmptyEntity(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        DeleteByteMatchSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        DeleteByteMatchSetError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        DeleteByteMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteByteMatchSetError::Validation(error_message.to_string())
                    }
                    _ => DeleteByteMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteByteMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteByteMatchSetError {
    fn from(err: serde_json::error::Error) -> DeleteByteMatchSetError {
        DeleteByteMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteByteMatchSetError {
    fn from(err: CredentialsError) -> DeleteByteMatchSetError {
        DeleteByteMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteByteMatchSetError {
    fn from(err: HttpDispatchError) -> DeleteByteMatchSetError {
        DeleteByteMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteByteMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteByteMatchSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteByteMatchSetError::WAFInternalError(ref cause) => cause,
            DeleteByteMatchSetError::WAFInvalidAccount(ref cause) => cause,
            DeleteByteMatchSetError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteByteMatchSetError::WAFNonexistentItem(ref cause) => cause,
            DeleteByteMatchSetError::WAFReferencedItem(ref cause) => cause,
            DeleteByteMatchSetError::WAFStaleData(ref cause) => cause,
            DeleteByteMatchSetError::Validation(ref cause) => cause,
            DeleteByteMatchSetError::Credentials(ref err) => err.description(),
            DeleteByteMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteByteMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIPSet
#[derive(Debug, PartialEq)]
pub enum DeleteIPSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because you tried to delete an object that isn't empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul>
    WAFNonEmptyEntity(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteIPSetError {
    pub fn from_body(body: &str) -> DeleteIPSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        DeleteIPSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        DeleteIPSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonEmptyEntityException" => {
                        DeleteIPSetError::WAFNonEmptyEntity(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        DeleteIPSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        DeleteIPSetError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        DeleteIPSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteIPSetError::Validation(error_message.to_string())
                    }
                    _ => DeleteIPSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteIPSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteIPSetError {
    fn from(err: serde_json::error::Error) -> DeleteIPSetError {
        DeleteIPSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteIPSetError {
    fn from(err: CredentialsError) -> DeleteIPSetError {
        DeleteIPSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteIPSetError {
    fn from(err: HttpDispatchError) -> DeleteIPSetError {
        DeleteIPSetError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteIPSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIPSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteIPSetError::WAFInternalError(ref cause) => cause,
            DeleteIPSetError::WAFInvalidAccount(ref cause) => cause,
            DeleteIPSetError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteIPSetError::WAFNonexistentItem(ref cause) => cause,
            DeleteIPSetError::WAFReferencedItem(ref cause) => cause,
            DeleteIPSetError::WAFStaleData(ref cause) => cause,
            DeleteIPSetError::Validation(ref cause) => cause,
            DeleteIPSetError::Credentials(ref err) => err.description(),
            DeleteIPSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteIPSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRateBasedRule
#[derive(Debug, PartialEq)]
pub enum DeleteRateBasedRuleError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because you tried to delete an object that isn't empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul>
    WAFNonEmptyEntity(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteRateBasedRuleError {
    pub fn from_body(body: &str) -> DeleteRateBasedRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        DeleteRateBasedRuleError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        DeleteRateBasedRuleError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonEmptyEntityException" => {
                        DeleteRateBasedRuleError::WAFNonEmptyEntity(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        DeleteRateBasedRuleError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        DeleteRateBasedRuleError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        DeleteRateBasedRuleError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRateBasedRuleError::Validation(error_message.to_string())
                    }
                    _ => DeleteRateBasedRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRateBasedRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRateBasedRuleError {
    fn from(err: serde_json::error::Error) -> DeleteRateBasedRuleError {
        DeleteRateBasedRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRateBasedRuleError {
    fn from(err: CredentialsError) -> DeleteRateBasedRuleError {
        DeleteRateBasedRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRateBasedRuleError {
    fn from(err: HttpDispatchError) -> DeleteRateBasedRuleError {
        DeleteRateBasedRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteRateBasedRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRateBasedRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteRateBasedRuleError::WAFInternalError(ref cause) => cause,
            DeleteRateBasedRuleError::WAFInvalidAccount(ref cause) => cause,
            DeleteRateBasedRuleError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteRateBasedRuleError::WAFNonexistentItem(ref cause) => cause,
            DeleteRateBasedRuleError::WAFReferencedItem(ref cause) => cause,
            DeleteRateBasedRuleError::WAFStaleData(ref cause) => cause,
            DeleteRateBasedRuleError::Validation(ref cause) => cause,
            DeleteRateBasedRuleError::Credentials(ref err) => err.description(),
            DeleteRateBasedRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRateBasedRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because you tried to delete an object that isn't empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul>
    WAFNonEmptyEntity(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteRuleError {
    pub fn from_body(body: &str) -> DeleteRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        DeleteRuleError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        DeleteRuleError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonEmptyEntityException" => {
                        DeleteRuleError::WAFNonEmptyEntity(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        DeleteRuleError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        DeleteRuleError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        DeleteRuleError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => DeleteRuleError::Validation(error_message.to_string()),
                    _ => DeleteRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRuleError::Unknown(String::from(body)),
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
            DeleteRuleError::WAFInternalError(ref cause) => cause,
            DeleteRuleError::WAFInvalidAccount(ref cause) => cause,
            DeleteRuleError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteRuleError::WAFNonexistentItem(ref cause) => cause,
            DeleteRuleError::WAFReferencedItem(ref cause) => cause,
            DeleteRuleError::WAFStaleData(ref cause) => cause,
            DeleteRuleError::Validation(ref cause) => cause,
            DeleteRuleError::Credentials(ref err) => err.description(),
            DeleteRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum DeleteSizeConstraintSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because you tried to delete an object that isn't empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul>
    WAFNonEmptyEntity(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteSizeConstraintSetError {
    pub fn from_body(body: &str) -> DeleteSizeConstraintSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        DeleteSizeConstraintSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        DeleteSizeConstraintSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonEmptyEntityException" => {
                        DeleteSizeConstraintSetError::WAFNonEmptyEntity(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => DeleteSizeConstraintSetError::WAFNonexistentItem(String::from(error_message)),
                    "WAFReferencedItemException" => {
                        DeleteSizeConstraintSetError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        DeleteSizeConstraintSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSizeConstraintSetError::Validation(error_message.to_string())
                    }
                    _ => DeleteSizeConstraintSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSizeConstraintSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSizeConstraintSetError {
    fn from(err: serde_json::error::Error) -> DeleteSizeConstraintSetError {
        DeleteSizeConstraintSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSizeConstraintSetError {
    fn from(err: CredentialsError) -> DeleteSizeConstraintSetError {
        DeleteSizeConstraintSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSizeConstraintSetError {
    fn from(err: HttpDispatchError) -> DeleteSizeConstraintSetError {
        DeleteSizeConstraintSetError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteSizeConstraintSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSizeConstraintSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteSizeConstraintSetError::WAFInternalError(ref cause) => cause,
            DeleteSizeConstraintSetError::WAFInvalidAccount(ref cause) => cause,
            DeleteSizeConstraintSetError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteSizeConstraintSetError::WAFNonexistentItem(ref cause) => cause,
            DeleteSizeConstraintSetError::WAFReferencedItem(ref cause) => cause,
            DeleteSizeConstraintSetError::WAFStaleData(ref cause) => cause,
            DeleteSizeConstraintSetError::Validation(ref cause) => cause,
            DeleteSizeConstraintSetError::Credentials(ref err) => err.description(),
            DeleteSizeConstraintSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSizeConstraintSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteSqlInjectionMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because you tried to delete an object that isn't empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul>
    WAFNonEmptyEntity(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteSqlInjectionMatchSetError {
    pub fn from_body(body: &str) -> DeleteSqlInjectionMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => DeleteSqlInjectionMatchSetError::WAFInternalError(String::from(error_message)),
                    "WAFInvalidAccountException" => DeleteSqlInjectionMatchSetError::WAFInvalidAccount(String::from(error_message)),
                    "WAFNonEmptyEntityException" => DeleteSqlInjectionMatchSetError::WAFNonEmptyEntity(String::from(error_message)),
                    "WAFNonexistentItemException" => DeleteSqlInjectionMatchSetError::WAFNonexistentItem(String::from(error_message)),
                    "WAFReferencedItemException" => DeleteSqlInjectionMatchSetError::WAFReferencedItem(String::from(error_message)),
                    "WAFStaleDataException" => {
                        DeleteSqlInjectionMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteSqlInjectionMatchSetError::Validation(error_message.to_string())
                    }
                    _ => DeleteSqlInjectionMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSqlInjectionMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteSqlInjectionMatchSetError {
    fn from(err: serde_json::error::Error) -> DeleteSqlInjectionMatchSetError {
        DeleteSqlInjectionMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteSqlInjectionMatchSetError {
    fn from(err: CredentialsError) -> DeleteSqlInjectionMatchSetError {
        DeleteSqlInjectionMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSqlInjectionMatchSetError {
    fn from(err: HttpDispatchError) -> DeleteSqlInjectionMatchSetError {
        DeleteSqlInjectionMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteSqlInjectionMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSqlInjectionMatchSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteSqlInjectionMatchSetError::WAFInternalError(ref cause) => cause,
            DeleteSqlInjectionMatchSetError::WAFInvalidAccount(ref cause) => cause,
            DeleteSqlInjectionMatchSetError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteSqlInjectionMatchSetError::WAFNonexistentItem(ref cause) => cause,
            DeleteSqlInjectionMatchSetError::WAFReferencedItem(ref cause) => cause,
            DeleteSqlInjectionMatchSetError::WAFStaleData(ref cause) => cause,
            DeleteSqlInjectionMatchSetError::Validation(ref cause) => cause,
            DeleteSqlInjectionMatchSetError::Credentials(ref err) => err.description(),
            DeleteSqlInjectionMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteSqlInjectionMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteWebACL
#[derive(Debug, PartialEq)]
pub enum DeleteWebACLError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because you tried to delete an object that isn't empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul>
    WAFNonEmptyEntity(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteWebACLError {
    pub fn from_body(body: &str) -> DeleteWebACLError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        DeleteWebACLError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        DeleteWebACLError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonEmptyEntityException" => {
                        DeleteWebACLError::WAFNonEmptyEntity(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        DeleteWebACLError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        DeleteWebACLError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        DeleteWebACLError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteWebACLError::Validation(error_message.to_string())
                    }
                    _ => DeleteWebACLError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteWebACLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteWebACLError {
    fn from(err: serde_json::error::Error) -> DeleteWebACLError {
        DeleteWebACLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteWebACLError {
    fn from(err: CredentialsError) -> DeleteWebACLError {
        DeleteWebACLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteWebACLError {
    fn from(err: HttpDispatchError) -> DeleteWebACLError {
        DeleteWebACLError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteWebACLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteWebACLError {
    fn description(&self) -> &str {
        match *self {
            DeleteWebACLError::WAFInternalError(ref cause) => cause,
            DeleteWebACLError::WAFInvalidAccount(ref cause) => cause,
            DeleteWebACLError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteWebACLError::WAFNonexistentItem(ref cause) => cause,
            DeleteWebACLError::WAFReferencedItem(ref cause) => cause,
            DeleteWebACLError::WAFStaleData(ref cause) => cause,
            DeleteWebACLError::Validation(ref cause) => cause,
            DeleteWebACLError::Credentials(ref err) => err.description(),
            DeleteWebACLError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteWebACLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteXssMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteXssMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because you tried to delete an object that isn't empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul>
    WAFNonEmptyEntity(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteXssMatchSetError {
    pub fn from_body(body: &str) -> DeleteXssMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        DeleteXssMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        DeleteXssMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonEmptyEntityException" => {
                        DeleteXssMatchSetError::WAFNonEmptyEntity(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        DeleteXssMatchSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        DeleteXssMatchSetError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        DeleteXssMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteXssMatchSetError::Validation(error_message.to_string())
                    }
                    _ => DeleteXssMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteXssMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteXssMatchSetError {
    fn from(err: serde_json::error::Error) -> DeleteXssMatchSetError {
        DeleteXssMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteXssMatchSetError {
    fn from(err: CredentialsError) -> DeleteXssMatchSetError {
        DeleteXssMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteXssMatchSetError {
    fn from(err: HttpDispatchError) -> DeleteXssMatchSetError {
        DeleteXssMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteXssMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteXssMatchSetError {
    fn description(&self) -> &str {
        match *self {
            DeleteXssMatchSetError::WAFInternalError(ref cause) => cause,
            DeleteXssMatchSetError::WAFInvalidAccount(ref cause) => cause,
            DeleteXssMatchSetError::WAFNonEmptyEntity(ref cause) => cause,
            DeleteXssMatchSetError::WAFNonexistentItem(ref cause) => cause,
            DeleteXssMatchSetError::WAFReferencedItem(ref cause) => cause,
            DeleteXssMatchSetError::WAFStaleData(ref cause) => cause,
            DeleteXssMatchSetError::Validation(ref cause) => cause,
            DeleteXssMatchSetError::Credentials(ref err) => err.description(),
            DeleteXssMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteXssMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateWebACL
#[derive(Debug, PartialEq)]
pub enum DisassociateWebACLError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisassociateWebACLError {
    pub fn from_body(body: &str) -> DisassociateWebACLError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        DisassociateWebACLError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        DisassociateWebACLError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        DisassociateWebACLError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        DisassociateWebACLError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisassociateWebACLError::Validation(error_message.to_string())
                    }
                    _ => DisassociateWebACLError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisassociateWebACLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisassociateWebACLError {
    fn from(err: serde_json::error::Error) -> DisassociateWebACLError {
        DisassociateWebACLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateWebACLError {
    fn from(err: CredentialsError) -> DisassociateWebACLError {
        DisassociateWebACLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateWebACLError {
    fn from(err: HttpDispatchError) -> DisassociateWebACLError {
        DisassociateWebACLError::HttpDispatch(err)
    }
}
impl fmt::Display for DisassociateWebACLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateWebACLError {
    fn description(&self) -> &str {
        match *self {
            DisassociateWebACLError::WAFInternalError(ref cause) => cause,
            DisassociateWebACLError::WAFInvalidAccount(ref cause) => cause,
            DisassociateWebACLError::WAFInvalidParameter(ref cause) => cause,
            DisassociateWebACLError::WAFNonexistentItem(ref cause) => cause,
            DisassociateWebACLError::Validation(ref cause) => cause,
            DisassociateWebACLError::Credentials(ref err) => err.description(),
            DisassociateWebACLError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateWebACLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetByteMatchSet
#[derive(Debug, PartialEq)]
pub enum GetByteMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetByteMatchSetError {
    pub fn from_body(body: &str) -> GetByteMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetByteMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetByteMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetByteMatchSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetByteMatchSetError::Validation(error_message.to_string())
                    }
                    _ => GetByteMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetByteMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetByteMatchSetError {
    fn from(err: serde_json::error::Error) -> GetByteMatchSetError {
        GetByteMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetByteMatchSetError {
    fn from(err: CredentialsError) -> GetByteMatchSetError {
        GetByteMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetByteMatchSetError {
    fn from(err: HttpDispatchError) -> GetByteMatchSetError {
        GetByteMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for GetByteMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetByteMatchSetError {
    fn description(&self) -> &str {
        match *self {
            GetByteMatchSetError::WAFInternalError(ref cause) => cause,
            GetByteMatchSetError::WAFInvalidAccount(ref cause) => cause,
            GetByteMatchSetError::WAFNonexistentItem(ref cause) => cause,
            GetByteMatchSetError::Validation(ref cause) => cause,
            GetByteMatchSetError::Credentials(ref err) => err.description(),
            GetByteMatchSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetByteMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetChangeToken
#[derive(Debug, PartialEq)]
pub enum GetChangeTokenError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetChangeTokenError {
    pub fn from_body(body: &str) -> GetChangeTokenError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetChangeTokenError::WAFInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetChangeTokenError::Validation(error_message.to_string())
                    }
                    _ => GetChangeTokenError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetChangeTokenError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetChangeTokenError {
    fn from(err: serde_json::error::Error) -> GetChangeTokenError {
        GetChangeTokenError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetChangeTokenError {
    fn from(err: CredentialsError) -> GetChangeTokenError {
        GetChangeTokenError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetChangeTokenError {
    fn from(err: HttpDispatchError) -> GetChangeTokenError {
        GetChangeTokenError::HttpDispatch(err)
    }
}
impl fmt::Display for GetChangeTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetChangeTokenError {
    fn description(&self) -> &str {
        match *self {
            GetChangeTokenError::WAFInternalError(ref cause) => cause,
            GetChangeTokenError::Validation(ref cause) => cause,
            GetChangeTokenError::Credentials(ref err) => err.description(),
            GetChangeTokenError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetChangeTokenError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetChangeTokenStatus
#[derive(Debug, PartialEq)]
pub enum GetChangeTokenStatusError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetChangeTokenStatusError {
    pub fn from_body(body: &str) -> GetChangeTokenStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetChangeTokenStatusError::WAFInternalError(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetChangeTokenStatusError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetChangeTokenStatusError::Validation(error_message.to_string())
                    }
                    _ => GetChangeTokenStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetChangeTokenStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetChangeTokenStatusError {
    fn from(err: serde_json::error::Error) -> GetChangeTokenStatusError {
        GetChangeTokenStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetChangeTokenStatusError {
    fn from(err: CredentialsError) -> GetChangeTokenStatusError {
        GetChangeTokenStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetChangeTokenStatusError {
    fn from(err: HttpDispatchError) -> GetChangeTokenStatusError {
        GetChangeTokenStatusError::HttpDispatch(err)
    }
}
impl fmt::Display for GetChangeTokenStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetChangeTokenStatusError {
    fn description(&self) -> &str {
        match *self {
            GetChangeTokenStatusError::WAFInternalError(ref cause) => cause,
            GetChangeTokenStatusError::WAFNonexistentItem(ref cause) => cause,
            GetChangeTokenStatusError::Validation(ref cause) => cause,
            GetChangeTokenStatusError::Credentials(ref err) => err.description(),
            GetChangeTokenStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetChangeTokenStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIPSet
#[derive(Debug, PartialEq)]
pub enum GetIPSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetIPSetError {
    pub fn from_body(body: &str) -> GetIPSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetIPSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetIPSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetIPSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => GetIPSetError::Validation(error_message.to_string()),
                    _ => GetIPSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetIPSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetIPSetError {
    fn from(err: serde_json::error::Error) -> GetIPSetError {
        GetIPSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIPSetError {
    fn from(err: CredentialsError) -> GetIPSetError {
        GetIPSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIPSetError {
    fn from(err: HttpDispatchError) -> GetIPSetError {
        GetIPSetError::HttpDispatch(err)
    }
}
impl fmt::Display for GetIPSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIPSetError {
    fn description(&self) -> &str {
        match *self {
            GetIPSetError::WAFInternalError(ref cause) => cause,
            GetIPSetError::WAFInvalidAccount(ref cause) => cause,
            GetIPSetError::WAFNonexistentItem(ref cause) => cause,
            GetIPSetError::Validation(ref cause) => cause,
            GetIPSetError::Credentials(ref err) => err.description(),
            GetIPSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetIPSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRateBasedRule
#[derive(Debug, PartialEq)]
pub enum GetRateBasedRuleError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetRateBasedRuleError {
    pub fn from_body(body: &str) -> GetRateBasedRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetRateBasedRuleError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetRateBasedRuleError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetRateBasedRuleError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRateBasedRuleError::Validation(error_message.to_string())
                    }
                    _ => GetRateBasedRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRateBasedRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRateBasedRuleError {
    fn from(err: serde_json::error::Error) -> GetRateBasedRuleError {
        GetRateBasedRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRateBasedRuleError {
    fn from(err: CredentialsError) -> GetRateBasedRuleError {
        GetRateBasedRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRateBasedRuleError {
    fn from(err: HttpDispatchError) -> GetRateBasedRuleError {
        GetRateBasedRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for GetRateBasedRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRateBasedRuleError {
    fn description(&self) -> &str {
        match *self {
            GetRateBasedRuleError::WAFInternalError(ref cause) => cause,
            GetRateBasedRuleError::WAFInvalidAccount(ref cause) => cause,
            GetRateBasedRuleError::WAFNonexistentItem(ref cause) => cause,
            GetRateBasedRuleError::Validation(ref cause) => cause,
            GetRateBasedRuleError::Credentials(ref err) => err.description(),
            GetRateBasedRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRateBasedRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRateBasedRuleManagedKeys
#[derive(Debug, PartialEq)]
pub enum GetRateBasedRuleManagedKeysError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetRateBasedRuleManagedKeysError {
    pub fn from_body(body: &str) -> GetRateBasedRuleManagedKeysError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => GetRateBasedRuleManagedKeysError::WAFInternalError(String::from(error_message)),
                    "WAFInvalidAccountException" => GetRateBasedRuleManagedKeysError::WAFInvalidAccount(String::from(error_message)),
                    "WAFInvalidParameterException" => GetRateBasedRuleManagedKeysError::WAFInvalidParameter(String::from(error_message)),
                    "WAFNonexistentItemException" => GetRateBasedRuleManagedKeysError::WAFNonexistentItem(String::from(error_message)),
                    "ValidationException" => {
                        GetRateBasedRuleManagedKeysError::Validation(error_message.to_string())
                    }
                    _ => GetRateBasedRuleManagedKeysError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRateBasedRuleManagedKeysError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRateBasedRuleManagedKeysError {
    fn from(err: serde_json::error::Error) -> GetRateBasedRuleManagedKeysError {
        GetRateBasedRuleManagedKeysError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRateBasedRuleManagedKeysError {
    fn from(err: CredentialsError) -> GetRateBasedRuleManagedKeysError {
        GetRateBasedRuleManagedKeysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRateBasedRuleManagedKeysError {
    fn from(err: HttpDispatchError) -> GetRateBasedRuleManagedKeysError {
        GetRateBasedRuleManagedKeysError::HttpDispatch(err)
    }
}
impl fmt::Display for GetRateBasedRuleManagedKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRateBasedRuleManagedKeysError {
    fn description(&self) -> &str {
        match *self {
            GetRateBasedRuleManagedKeysError::WAFInternalError(ref cause) => cause,
            GetRateBasedRuleManagedKeysError::WAFInvalidAccount(ref cause) => cause,
            GetRateBasedRuleManagedKeysError::WAFInvalidParameter(ref cause) => cause,
            GetRateBasedRuleManagedKeysError::WAFNonexistentItem(ref cause) => cause,
            GetRateBasedRuleManagedKeysError::Validation(ref cause) => cause,
            GetRateBasedRuleManagedKeysError::Credentials(ref err) => err.description(),
            GetRateBasedRuleManagedKeysError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRateBasedRuleManagedKeysError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRule
#[derive(Debug, PartialEq)]
pub enum GetRuleError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetRuleError {
    pub fn from_body(body: &str) -> GetRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetRuleError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetRuleError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetRuleError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => GetRuleError::Validation(error_message.to_string()),
                    _ => GetRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRuleError {
    fn from(err: serde_json::error::Error) -> GetRuleError {
        GetRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRuleError {
    fn from(err: CredentialsError) -> GetRuleError {
        GetRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRuleError {
    fn from(err: HttpDispatchError) -> GetRuleError {
        GetRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for GetRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRuleError {
    fn description(&self) -> &str {
        match *self {
            GetRuleError::WAFInternalError(ref cause) => cause,
            GetRuleError::WAFInvalidAccount(ref cause) => cause,
            GetRuleError::WAFNonexistentItem(ref cause) => cause,
            GetRuleError::Validation(ref cause) => cause,
            GetRuleError::Credentials(ref err) => err.description(),
            GetRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSampledRequests
#[derive(Debug, PartialEq)]
pub enum GetSampledRequestsError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetSampledRequestsError {
    pub fn from_body(body: &str) -> GetSampledRequestsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetSampledRequestsError::WAFInternalError(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetSampledRequestsError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSampledRequestsError::Validation(error_message.to_string())
                    }
                    _ => GetSampledRequestsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSampledRequestsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSampledRequestsError {
    fn from(err: serde_json::error::Error) -> GetSampledRequestsError {
        GetSampledRequestsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSampledRequestsError {
    fn from(err: CredentialsError) -> GetSampledRequestsError {
        GetSampledRequestsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSampledRequestsError {
    fn from(err: HttpDispatchError) -> GetSampledRequestsError {
        GetSampledRequestsError::HttpDispatch(err)
    }
}
impl fmt::Display for GetSampledRequestsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSampledRequestsError {
    fn description(&self) -> &str {
        match *self {
            GetSampledRequestsError::WAFInternalError(ref cause) => cause,
            GetSampledRequestsError::WAFNonexistentItem(ref cause) => cause,
            GetSampledRequestsError::Validation(ref cause) => cause,
            GetSampledRequestsError::Credentials(ref err) => err.description(),
            GetSampledRequestsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSampledRequestsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum GetSizeConstraintSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetSizeConstraintSetError {
    pub fn from_body(body: &str) -> GetSizeConstraintSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetSizeConstraintSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetSizeConstraintSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetSizeConstraintSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetSizeConstraintSetError::Validation(error_message.to_string())
                    }
                    _ => GetSizeConstraintSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSizeConstraintSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSizeConstraintSetError {
    fn from(err: serde_json::error::Error) -> GetSizeConstraintSetError {
        GetSizeConstraintSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSizeConstraintSetError {
    fn from(err: CredentialsError) -> GetSizeConstraintSetError {
        GetSizeConstraintSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSizeConstraintSetError {
    fn from(err: HttpDispatchError) -> GetSizeConstraintSetError {
        GetSizeConstraintSetError::HttpDispatch(err)
    }
}
impl fmt::Display for GetSizeConstraintSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSizeConstraintSetError {
    fn description(&self) -> &str {
        match *self {
            GetSizeConstraintSetError::WAFInternalError(ref cause) => cause,
            GetSizeConstraintSetError::WAFInvalidAccount(ref cause) => cause,
            GetSizeConstraintSetError::WAFNonexistentItem(ref cause) => cause,
            GetSizeConstraintSetError::Validation(ref cause) => cause,
            GetSizeConstraintSetError::Credentials(ref err) => err.description(),
            GetSizeConstraintSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSizeConstraintSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum GetSqlInjectionMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetSqlInjectionMatchSetError {
    pub fn from_body(body: &str) -> GetSqlInjectionMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetSqlInjectionMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetSqlInjectionMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => GetSqlInjectionMatchSetError::WAFNonexistentItem(String::from(error_message)),
                    "ValidationException" => {
                        GetSqlInjectionMatchSetError::Validation(error_message.to_string())
                    }
                    _ => GetSqlInjectionMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSqlInjectionMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSqlInjectionMatchSetError {
    fn from(err: serde_json::error::Error) -> GetSqlInjectionMatchSetError {
        GetSqlInjectionMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSqlInjectionMatchSetError {
    fn from(err: CredentialsError) -> GetSqlInjectionMatchSetError {
        GetSqlInjectionMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSqlInjectionMatchSetError {
    fn from(err: HttpDispatchError) -> GetSqlInjectionMatchSetError {
        GetSqlInjectionMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for GetSqlInjectionMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSqlInjectionMatchSetError {
    fn description(&self) -> &str {
        match *self {
            GetSqlInjectionMatchSetError::WAFInternalError(ref cause) => cause,
            GetSqlInjectionMatchSetError::WAFInvalidAccount(ref cause) => cause,
            GetSqlInjectionMatchSetError::WAFNonexistentItem(ref cause) => cause,
            GetSqlInjectionMatchSetError::Validation(ref cause) => cause,
            GetSqlInjectionMatchSetError::Credentials(ref err) => err.description(),
            GetSqlInjectionMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetSqlInjectionMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWebACL
#[derive(Debug, PartialEq)]
pub enum GetWebACLError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetWebACLError {
    pub fn from_body(body: &str) -> GetWebACLError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetWebACLError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetWebACLError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetWebACLError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => GetWebACLError::Validation(error_message.to_string()),
                    _ => GetWebACLError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetWebACLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetWebACLError {
    fn from(err: serde_json::error::Error) -> GetWebACLError {
        GetWebACLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetWebACLError {
    fn from(err: CredentialsError) -> GetWebACLError {
        GetWebACLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetWebACLError {
    fn from(err: HttpDispatchError) -> GetWebACLError {
        GetWebACLError::HttpDispatch(err)
    }
}
impl fmt::Display for GetWebACLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWebACLError {
    fn description(&self) -> &str {
        match *self {
            GetWebACLError::WAFInternalError(ref cause) => cause,
            GetWebACLError::WAFInvalidAccount(ref cause) => cause,
            GetWebACLError::WAFNonexistentItem(ref cause) => cause,
            GetWebACLError::Validation(ref cause) => cause,
            GetWebACLError::Credentials(ref err) => err.description(),
            GetWebACLError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetWebACLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetWebACLForResource
#[derive(Debug, PartialEq)]
pub enum GetWebACLForResourceError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because the entity referenced is temporarily unavailable. Retry your request.</p>
    WAFUnavailableEntity(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetWebACLForResourceError {
    pub fn from_body(body: &str) -> GetWebACLForResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetWebACLForResourceError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetWebACLForResourceError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        GetWebACLForResourceError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetWebACLForResourceError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFUnavailableEntityException" => {
                        GetWebACLForResourceError::WAFUnavailableEntity(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetWebACLForResourceError::Validation(error_message.to_string())
                    }
                    _ => GetWebACLForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetWebACLForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetWebACLForResourceError {
    fn from(err: serde_json::error::Error) -> GetWebACLForResourceError {
        GetWebACLForResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetWebACLForResourceError {
    fn from(err: CredentialsError) -> GetWebACLForResourceError {
        GetWebACLForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetWebACLForResourceError {
    fn from(err: HttpDispatchError) -> GetWebACLForResourceError {
        GetWebACLForResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for GetWebACLForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetWebACLForResourceError {
    fn description(&self) -> &str {
        match *self {
            GetWebACLForResourceError::WAFInternalError(ref cause) => cause,
            GetWebACLForResourceError::WAFInvalidAccount(ref cause) => cause,
            GetWebACLForResourceError::WAFInvalidParameter(ref cause) => cause,
            GetWebACLForResourceError::WAFNonexistentItem(ref cause) => cause,
            GetWebACLForResourceError::WAFUnavailableEntity(ref cause) => cause,
            GetWebACLForResourceError::Validation(ref cause) => cause,
            GetWebACLForResourceError::Credentials(ref err) => err.description(),
            GetWebACLForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetWebACLForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetXssMatchSet
#[derive(Debug, PartialEq)]
pub enum GetXssMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetXssMatchSetError {
    pub fn from_body(body: &str) -> GetXssMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        GetXssMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        GetXssMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        GetXssMatchSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetXssMatchSetError::Validation(error_message.to_string())
                    }
                    _ => GetXssMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetXssMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetXssMatchSetError {
    fn from(err: serde_json::error::Error) -> GetXssMatchSetError {
        GetXssMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetXssMatchSetError {
    fn from(err: CredentialsError) -> GetXssMatchSetError {
        GetXssMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetXssMatchSetError {
    fn from(err: HttpDispatchError) -> GetXssMatchSetError {
        GetXssMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for GetXssMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetXssMatchSetError {
    fn description(&self) -> &str {
        match *self {
            GetXssMatchSetError::WAFInternalError(ref cause) => cause,
            GetXssMatchSetError::WAFInvalidAccount(ref cause) => cause,
            GetXssMatchSetError::WAFNonexistentItem(ref cause) => cause,
            GetXssMatchSetError::Validation(ref cause) => cause,
            GetXssMatchSetError::Credentials(ref err) => err.description(),
            GetXssMatchSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetXssMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListByteMatchSets
#[derive(Debug, PartialEq)]
pub enum ListByteMatchSetsError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListByteMatchSetsError {
    pub fn from_body(body: &str) -> ListByteMatchSetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        ListByteMatchSetsError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        ListByteMatchSetsError::WAFInvalidAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListByteMatchSetsError::Validation(error_message.to_string())
                    }
                    _ => ListByteMatchSetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListByteMatchSetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListByteMatchSetsError {
    fn from(err: serde_json::error::Error) -> ListByteMatchSetsError {
        ListByteMatchSetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListByteMatchSetsError {
    fn from(err: CredentialsError) -> ListByteMatchSetsError {
        ListByteMatchSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListByteMatchSetsError {
    fn from(err: HttpDispatchError) -> ListByteMatchSetsError {
        ListByteMatchSetsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListByteMatchSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListByteMatchSetsError {
    fn description(&self) -> &str {
        match *self {
            ListByteMatchSetsError::WAFInternalError(ref cause) => cause,
            ListByteMatchSetsError::WAFInvalidAccount(ref cause) => cause,
            ListByteMatchSetsError::Validation(ref cause) => cause,
            ListByteMatchSetsError::Credentials(ref err) => err.description(),
            ListByteMatchSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListByteMatchSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIPSets
#[derive(Debug, PartialEq)]
pub enum ListIPSetsError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListIPSetsError {
    pub fn from_body(body: &str) -> ListIPSetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        ListIPSetsError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        ListIPSetsError::WAFInvalidAccount(String::from(error_message))
                    }
                    "ValidationException" => ListIPSetsError::Validation(error_message.to_string()),
                    _ => ListIPSetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListIPSetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListIPSetsError {
    fn from(err: serde_json::error::Error) -> ListIPSetsError {
        ListIPSetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIPSetsError {
    fn from(err: CredentialsError) -> ListIPSetsError {
        ListIPSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIPSetsError {
    fn from(err: HttpDispatchError) -> ListIPSetsError {
        ListIPSetsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListIPSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIPSetsError {
    fn description(&self) -> &str {
        match *self {
            ListIPSetsError::WAFInternalError(ref cause) => cause,
            ListIPSetsError::WAFInvalidAccount(ref cause) => cause,
            ListIPSetsError::Validation(ref cause) => cause,
            ListIPSetsError::Credentials(ref err) => err.description(),
            ListIPSetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListIPSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRateBasedRules
#[derive(Debug, PartialEq)]
pub enum ListRateBasedRulesError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListRateBasedRulesError {
    pub fn from_body(body: &str) -> ListRateBasedRulesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        ListRateBasedRulesError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        ListRateBasedRulesError::WAFInvalidAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRateBasedRulesError::Validation(error_message.to_string())
                    }
                    _ => ListRateBasedRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRateBasedRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRateBasedRulesError {
    fn from(err: serde_json::error::Error) -> ListRateBasedRulesError {
        ListRateBasedRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRateBasedRulesError {
    fn from(err: CredentialsError) -> ListRateBasedRulesError {
        ListRateBasedRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRateBasedRulesError {
    fn from(err: HttpDispatchError) -> ListRateBasedRulesError {
        ListRateBasedRulesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListRateBasedRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRateBasedRulesError {
    fn description(&self) -> &str {
        match *self {
            ListRateBasedRulesError::WAFInternalError(ref cause) => cause,
            ListRateBasedRulesError::WAFInvalidAccount(ref cause) => cause,
            ListRateBasedRulesError::Validation(ref cause) => cause,
            ListRateBasedRulesError::Credentials(ref err) => err.description(),
            ListRateBasedRulesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListRateBasedRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourcesForWebACL
#[derive(Debug, PartialEq)]
pub enum ListResourcesForWebACLError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListResourcesForWebACLError {
    pub fn from_body(body: &str) -> ListResourcesForWebACLError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        ListResourcesForWebACLError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        ListResourcesForWebACLError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        ListResourcesForWebACLError::WAFNonexistentItem(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListResourcesForWebACLError::Validation(error_message.to_string())
                    }
                    _ => ListResourcesForWebACLError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListResourcesForWebACLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListResourcesForWebACLError {
    fn from(err: serde_json::error::Error) -> ListResourcesForWebACLError {
        ListResourcesForWebACLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListResourcesForWebACLError {
    fn from(err: CredentialsError) -> ListResourcesForWebACLError {
        ListResourcesForWebACLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListResourcesForWebACLError {
    fn from(err: HttpDispatchError) -> ListResourcesForWebACLError {
        ListResourcesForWebACLError::HttpDispatch(err)
    }
}
impl fmt::Display for ListResourcesForWebACLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourcesForWebACLError {
    fn description(&self) -> &str {
        match *self {
            ListResourcesForWebACLError::WAFInternalError(ref cause) => cause,
            ListResourcesForWebACLError::WAFInvalidAccount(ref cause) => cause,
            ListResourcesForWebACLError::WAFNonexistentItem(ref cause) => cause,
            ListResourcesForWebACLError::Validation(ref cause) => cause,
            ListResourcesForWebACLError::Credentials(ref err) => err.description(),
            ListResourcesForWebACLError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListResourcesForWebACLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRules
#[derive(Debug, PartialEq)]
pub enum ListRulesError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListRulesError {
    pub fn from_body(body: &str) -> ListRulesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        ListRulesError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        ListRulesError::WAFInvalidAccount(String::from(error_message))
                    }
                    "ValidationException" => ListRulesError::Validation(error_message.to_string()),
                    _ => ListRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRulesError::Unknown(String::from(body)),
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
            ListRulesError::WAFInternalError(ref cause) => cause,
            ListRulesError::WAFInvalidAccount(ref cause) => cause,
            ListRulesError::Validation(ref cause) => cause,
            ListRulesError::Credentials(ref err) => err.description(),
            ListRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSizeConstraintSets
#[derive(Debug, PartialEq)]
pub enum ListSizeConstraintSetsError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListSizeConstraintSetsError {
    pub fn from_body(body: &str) -> ListSizeConstraintSetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        ListSizeConstraintSetsError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        ListSizeConstraintSetsError::WAFInvalidAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListSizeConstraintSetsError::Validation(error_message.to_string())
                    }
                    _ => ListSizeConstraintSetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSizeConstraintSetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSizeConstraintSetsError {
    fn from(err: serde_json::error::Error) -> ListSizeConstraintSetsError {
        ListSizeConstraintSetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSizeConstraintSetsError {
    fn from(err: CredentialsError) -> ListSizeConstraintSetsError {
        ListSizeConstraintSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSizeConstraintSetsError {
    fn from(err: HttpDispatchError) -> ListSizeConstraintSetsError {
        ListSizeConstraintSetsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListSizeConstraintSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSizeConstraintSetsError {
    fn description(&self) -> &str {
        match *self {
            ListSizeConstraintSetsError::WAFInternalError(ref cause) => cause,
            ListSizeConstraintSetsError::WAFInvalidAccount(ref cause) => cause,
            ListSizeConstraintSetsError::Validation(ref cause) => cause,
            ListSizeConstraintSetsError::Credentials(ref err) => err.description(),
            ListSizeConstraintSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSizeConstraintSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSqlInjectionMatchSets
#[derive(Debug, PartialEq)]
pub enum ListSqlInjectionMatchSetsError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListSqlInjectionMatchSetsError {
    pub fn from_body(body: &str) -> ListSqlInjectionMatchSetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => ListSqlInjectionMatchSetsError::WAFInternalError(String::from(error_message)),
                    "WAFInvalidAccountException" => ListSqlInjectionMatchSetsError::WAFInvalidAccount(String::from(error_message)),
                    "ValidationException" => {
                        ListSqlInjectionMatchSetsError::Validation(error_message.to_string())
                    }
                    _ => ListSqlInjectionMatchSetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSqlInjectionMatchSetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSqlInjectionMatchSetsError {
    fn from(err: serde_json::error::Error) -> ListSqlInjectionMatchSetsError {
        ListSqlInjectionMatchSetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSqlInjectionMatchSetsError {
    fn from(err: CredentialsError) -> ListSqlInjectionMatchSetsError {
        ListSqlInjectionMatchSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSqlInjectionMatchSetsError {
    fn from(err: HttpDispatchError) -> ListSqlInjectionMatchSetsError {
        ListSqlInjectionMatchSetsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListSqlInjectionMatchSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSqlInjectionMatchSetsError {
    fn description(&self) -> &str {
        match *self {
            ListSqlInjectionMatchSetsError::WAFInternalError(ref cause) => cause,
            ListSqlInjectionMatchSetsError::WAFInvalidAccount(ref cause) => cause,
            ListSqlInjectionMatchSetsError::Validation(ref cause) => cause,
            ListSqlInjectionMatchSetsError::Credentials(ref err) => err.description(),
            ListSqlInjectionMatchSetsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListSqlInjectionMatchSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListWebACLs
#[derive(Debug, PartialEq)]
pub enum ListWebACLsError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListWebACLsError {
    pub fn from_body(body: &str) -> ListWebACLsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        ListWebACLsError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        ListWebACLsError::WAFInvalidAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListWebACLsError::Validation(error_message.to_string())
                    }
                    _ => ListWebACLsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListWebACLsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListWebACLsError {
    fn from(err: serde_json::error::Error) -> ListWebACLsError {
        ListWebACLsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListWebACLsError {
    fn from(err: CredentialsError) -> ListWebACLsError {
        ListWebACLsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListWebACLsError {
    fn from(err: HttpDispatchError) -> ListWebACLsError {
        ListWebACLsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListWebACLsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListWebACLsError {
    fn description(&self) -> &str {
        match *self {
            ListWebACLsError::WAFInternalError(ref cause) => cause,
            ListWebACLsError::WAFInvalidAccount(ref cause) => cause,
            ListWebACLsError::Validation(ref cause) => cause,
            ListWebACLsError::Credentials(ref err) => err.description(),
            ListWebACLsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListWebACLsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListXssMatchSets
#[derive(Debug, PartialEq)]
pub enum ListXssMatchSetsError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListXssMatchSetsError {
    pub fn from_body(body: &str) -> ListXssMatchSetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        ListXssMatchSetsError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        ListXssMatchSetsError::WAFInvalidAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListXssMatchSetsError::Validation(error_message.to_string())
                    }
                    _ => ListXssMatchSetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListXssMatchSetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListXssMatchSetsError {
    fn from(err: serde_json::error::Error) -> ListXssMatchSetsError {
        ListXssMatchSetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListXssMatchSetsError {
    fn from(err: CredentialsError) -> ListXssMatchSetsError {
        ListXssMatchSetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListXssMatchSetsError {
    fn from(err: HttpDispatchError) -> ListXssMatchSetsError {
        ListXssMatchSetsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListXssMatchSetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListXssMatchSetsError {
    fn description(&self) -> &str {
        match *self {
            ListXssMatchSetsError::WAFInternalError(ref cause) => cause,
            ListXssMatchSetsError::WAFInvalidAccount(ref cause) => cause,
            ListXssMatchSetsError::Validation(ref cause) => cause,
            ListXssMatchSetsError::Credentials(ref err) => err.description(),
            ListXssMatchSetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListXssMatchSetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateByteMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateByteMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add an IP address to an <code>IPSet</code>, but the IP address already exists in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul>
    WAFInvalidOperation(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li> </ul>
    WAFNonexistentContainer(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateByteMatchSetError {
    pub fn from_body(body: &str) -> UpdateByteMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        UpdateByteMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        UpdateByteMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidOperationException" => {
                        UpdateByteMatchSetError::WAFInvalidOperation(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        UpdateByteMatchSetError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        UpdateByteMatchSetError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFNonexistentContainerException" => UpdateByteMatchSetError::WAFNonexistentContainer(String::from(error_message)),
                    "WAFNonexistentItemException" => {
                        UpdateByteMatchSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        UpdateByteMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateByteMatchSetError::Validation(error_message.to_string())
                    }
                    _ => UpdateByteMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateByteMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateByteMatchSetError {
    fn from(err: serde_json::error::Error) -> UpdateByteMatchSetError {
        UpdateByteMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateByteMatchSetError {
    fn from(err: CredentialsError) -> UpdateByteMatchSetError {
        UpdateByteMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateByteMatchSetError {
    fn from(err: HttpDispatchError) -> UpdateByteMatchSetError {
        UpdateByteMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateByteMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateByteMatchSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateByteMatchSetError::WAFInternalError(ref cause) => cause,
            UpdateByteMatchSetError::WAFInvalidAccount(ref cause) => cause,
            UpdateByteMatchSetError::WAFInvalidOperation(ref cause) => cause,
            UpdateByteMatchSetError::WAFInvalidParameter(ref cause) => cause,
            UpdateByteMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            UpdateByteMatchSetError::WAFNonexistentContainer(ref cause) => cause,
            UpdateByteMatchSetError::WAFNonexistentItem(ref cause) => cause,
            UpdateByteMatchSetError::WAFStaleData(ref cause) => cause,
            UpdateByteMatchSetError::Validation(ref cause) => cause,
            UpdateByteMatchSetError::Credentials(ref err) => err.description(),
            UpdateByteMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateByteMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIPSet
#[derive(Debug, PartialEq)]
pub enum UpdateIPSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add an IP address to an <code>IPSet</code>, but the IP address already exists in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul>
    WAFInvalidOperation(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li> </ul>
    WAFNonexistentContainer(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateIPSetError {
    pub fn from_body(body: &str) -> UpdateIPSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        UpdateIPSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        UpdateIPSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidOperationException" => {
                        UpdateIPSetError::WAFInvalidOperation(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        UpdateIPSetError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        UpdateIPSetError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFNonexistentContainerException" => {
                        UpdateIPSetError::WAFNonexistentContainer(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        UpdateIPSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        UpdateIPSetError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        UpdateIPSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateIPSetError::Validation(error_message.to_string())
                    }
                    _ => UpdateIPSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateIPSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateIPSetError {
    fn from(err: serde_json::error::Error) -> UpdateIPSetError {
        UpdateIPSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateIPSetError {
    fn from(err: CredentialsError) -> UpdateIPSetError {
        UpdateIPSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateIPSetError {
    fn from(err: HttpDispatchError) -> UpdateIPSetError {
        UpdateIPSetError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateIPSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIPSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateIPSetError::WAFInternalError(ref cause) => cause,
            UpdateIPSetError::WAFInvalidAccount(ref cause) => cause,
            UpdateIPSetError::WAFInvalidOperation(ref cause) => cause,
            UpdateIPSetError::WAFInvalidParameter(ref cause) => cause,
            UpdateIPSetError::WAFLimitsExceeded(ref cause) => cause,
            UpdateIPSetError::WAFNonexistentContainer(ref cause) => cause,
            UpdateIPSetError::WAFNonexistentItem(ref cause) => cause,
            UpdateIPSetError::WAFReferencedItem(ref cause) => cause,
            UpdateIPSetError::WAFStaleData(ref cause) => cause,
            UpdateIPSetError::Validation(ref cause) => cause,
            UpdateIPSetError::Credentials(ref err) => err.description(),
            UpdateIPSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateIPSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRateBasedRule
#[derive(Debug, PartialEq)]
pub enum UpdateRateBasedRuleError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add an IP address to an <code>IPSet</code>, but the IP address already exists in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul>
    WAFInvalidOperation(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li> </ul>
    WAFNonexistentContainer(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateRateBasedRuleError {
    pub fn from_body(body: &str) -> UpdateRateBasedRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        UpdateRateBasedRuleError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        UpdateRateBasedRuleError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidOperationException" => {
                        UpdateRateBasedRuleError::WAFInvalidOperation(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        UpdateRateBasedRuleError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        UpdateRateBasedRuleError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFNonexistentContainerException" => UpdateRateBasedRuleError::WAFNonexistentContainer(String::from(error_message)),
                    "WAFNonexistentItemException" => {
                        UpdateRateBasedRuleError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        UpdateRateBasedRuleError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        UpdateRateBasedRuleError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateRateBasedRuleError::Validation(error_message.to_string())
                    }
                    _ => UpdateRateBasedRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRateBasedRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRateBasedRuleError {
    fn from(err: serde_json::error::Error) -> UpdateRateBasedRuleError {
        UpdateRateBasedRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRateBasedRuleError {
    fn from(err: CredentialsError) -> UpdateRateBasedRuleError {
        UpdateRateBasedRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRateBasedRuleError {
    fn from(err: HttpDispatchError) -> UpdateRateBasedRuleError {
        UpdateRateBasedRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateRateBasedRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRateBasedRuleError {
    fn description(&self) -> &str {
        match *self {
            UpdateRateBasedRuleError::WAFInternalError(ref cause) => cause,
            UpdateRateBasedRuleError::WAFInvalidAccount(ref cause) => cause,
            UpdateRateBasedRuleError::WAFInvalidOperation(ref cause) => cause,
            UpdateRateBasedRuleError::WAFInvalidParameter(ref cause) => cause,
            UpdateRateBasedRuleError::WAFLimitsExceeded(ref cause) => cause,
            UpdateRateBasedRuleError::WAFNonexistentContainer(ref cause) => cause,
            UpdateRateBasedRuleError::WAFNonexistentItem(ref cause) => cause,
            UpdateRateBasedRuleError::WAFReferencedItem(ref cause) => cause,
            UpdateRateBasedRuleError::WAFStaleData(ref cause) => cause,
            UpdateRateBasedRuleError::Validation(ref cause) => cause,
            UpdateRateBasedRuleError::Credentials(ref err) => err.description(),
            UpdateRateBasedRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateRateBasedRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRule
#[derive(Debug, PartialEq)]
pub enum UpdateRuleError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add an IP address to an <code>IPSet</code>, but the IP address already exists in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul>
    WAFInvalidOperation(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li> </ul>
    WAFNonexistentContainer(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateRuleError {
    pub fn from_body(body: &str) -> UpdateRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        UpdateRuleError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        UpdateRuleError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidOperationException" => {
                        UpdateRuleError::WAFInvalidOperation(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        UpdateRuleError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        UpdateRuleError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFNonexistentContainerException" => {
                        UpdateRuleError::WAFNonexistentContainer(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        UpdateRuleError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        UpdateRuleError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        UpdateRuleError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => UpdateRuleError::Validation(error_message.to_string()),
                    _ => UpdateRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRuleError {
    fn from(err: serde_json::error::Error) -> UpdateRuleError {
        UpdateRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRuleError {
    fn from(err: CredentialsError) -> UpdateRuleError {
        UpdateRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRuleError {
    fn from(err: HttpDispatchError) -> UpdateRuleError {
        UpdateRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRuleError {
    fn description(&self) -> &str {
        match *self {
            UpdateRuleError::WAFInternalError(ref cause) => cause,
            UpdateRuleError::WAFInvalidAccount(ref cause) => cause,
            UpdateRuleError::WAFInvalidOperation(ref cause) => cause,
            UpdateRuleError::WAFInvalidParameter(ref cause) => cause,
            UpdateRuleError::WAFLimitsExceeded(ref cause) => cause,
            UpdateRuleError::WAFNonexistentContainer(ref cause) => cause,
            UpdateRuleError::WAFNonexistentItem(ref cause) => cause,
            UpdateRuleError::WAFReferencedItem(ref cause) => cause,
            UpdateRuleError::WAFStaleData(ref cause) => cause,
            UpdateRuleError::Validation(ref cause) => cause,
            UpdateRuleError::Credentials(ref err) => err.description(),
            UpdateRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum UpdateSizeConstraintSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add an IP address to an <code>IPSet</code>, but the IP address already exists in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul>
    WAFInvalidOperation(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li> </ul>
    WAFNonexistentContainer(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateSizeConstraintSetError {
    pub fn from_body(body: &str) -> UpdateSizeConstraintSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        UpdateSizeConstraintSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        UpdateSizeConstraintSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidOperationException" => UpdateSizeConstraintSetError::WAFInvalidOperation(String::from(error_message)),
                    "WAFInvalidParameterException" => UpdateSizeConstraintSetError::WAFInvalidParameter(String::from(error_message)),
                    "WAFLimitsExceededException" => {
                        UpdateSizeConstraintSetError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFNonexistentContainerException" => UpdateSizeConstraintSetError::WAFNonexistentContainer(String::from(error_message)),
                    "WAFNonexistentItemException" => UpdateSizeConstraintSetError::WAFNonexistentItem(String::from(error_message)),
                    "WAFReferencedItemException" => {
                        UpdateSizeConstraintSetError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        UpdateSizeConstraintSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSizeConstraintSetError::Validation(error_message.to_string())
                    }
                    _ => UpdateSizeConstraintSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSizeConstraintSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSizeConstraintSetError {
    fn from(err: serde_json::error::Error) -> UpdateSizeConstraintSetError {
        UpdateSizeConstraintSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSizeConstraintSetError {
    fn from(err: CredentialsError) -> UpdateSizeConstraintSetError {
        UpdateSizeConstraintSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSizeConstraintSetError {
    fn from(err: HttpDispatchError) -> UpdateSizeConstraintSetError {
        UpdateSizeConstraintSetError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateSizeConstraintSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSizeConstraintSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateSizeConstraintSetError::WAFInternalError(ref cause) => cause,
            UpdateSizeConstraintSetError::WAFInvalidAccount(ref cause) => cause,
            UpdateSizeConstraintSetError::WAFInvalidOperation(ref cause) => cause,
            UpdateSizeConstraintSetError::WAFInvalidParameter(ref cause) => cause,
            UpdateSizeConstraintSetError::WAFLimitsExceeded(ref cause) => cause,
            UpdateSizeConstraintSetError::WAFNonexistentContainer(ref cause) => cause,
            UpdateSizeConstraintSetError::WAFNonexistentItem(ref cause) => cause,
            UpdateSizeConstraintSetError::WAFReferencedItem(ref cause) => cause,
            UpdateSizeConstraintSetError::WAFStaleData(ref cause) => cause,
            UpdateSizeConstraintSetError::Validation(ref cause) => cause,
            UpdateSizeConstraintSetError::Credentials(ref err) => err.description(),
            UpdateSizeConstraintSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateSizeConstraintSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateSqlInjectionMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add an IP address to an <code>IPSet</code>, but the IP address already exists in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul>
    WAFInvalidOperation(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li> </ul>
    WAFNonexistentContainer(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateSqlInjectionMatchSetError {
    pub fn from_body(body: &str) -> UpdateSqlInjectionMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => UpdateSqlInjectionMatchSetError::WAFInternalError(String::from(error_message)),
                    "WAFInvalidAccountException" => UpdateSqlInjectionMatchSetError::WAFInvalidAccount(String::from(error_message)),
                    "WAFInvalidOperationException" => UpdateSqlInjectionMatchSetError::WAFInvalidOperation(String::from(error_message)),
                    "WAFInvalidParameterException" => UpdateSqlInjectionMatchSetError::WAFInvalidParameter(String::from(error_message)),
                    "WAFLimitsExceededException" => UpdateSqlInjectionMatchSetError::WAFLimitsExceeded(String::from(error_message)),
                    "WAFNonexistentContainerException" => UpdateSqlInjectionMatchSetError::WAFNonexistentContainer(String::from(error_message)),
                    "WAFNonexistentItemException" => UpdateSqlInjectionMatchSetError::WAFNonexistentItem(String::from(error_message)),
                    "WAFStaleDataException" => {
                        UpdateSqlInjectionMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateSqlInjectionMatchSetError::Validation(error_message.to_string())
                    }
                    _ => UpdateSqlInjectionMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateSqlInjectionMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateSqlInjectionMatchSetError {
    fn from(err: serde_json::error::Error) -> UpdateSqlInjectionMatchSetError {
        UpdateSqlInjectionMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateSqlInjectionMatchSetError {
    fn from(err: CredentialsError) -> UpdateSqlInjectionMatchSetError {
        UpdateSqlInjectionMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateSqlInjectionMatchSetError {
    fn from(err: HttpDispatchError) -> UpdateSqlInjectionMatchSetError {
        UpdateSqlInjectionMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateSqlInjectionMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateSqlInjectionMatchSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateSqlInjectionMatchSetError::WAFInternalError(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::WAFInvalidAccount(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::WAFInvalidOperation(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::WAFInvalidParameter(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::WAFNonexistentContainer(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::WAFNonexistentItem(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::WAFStaleData(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::Validation(ref cause) => cause,
            UpdateSqlInjectionMatchSetError::Credentials(ref err) => err.description(),
            UpdateSqlInjectionMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateSqlInjectionMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateWebACL
#[derive(Debug, PartialEq)]
pub enum UpdateWebACLError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add an IP address to an <code>IPSet</code>, but the IP address already exists in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul>
    WAFInvalidOperation(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li> </ul>
    WAFNonexistentContainer(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul>
    WAFReferencedItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateWebACLError {
    pub fn from_body(body: &str) -> UpdateWebACLError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        UpdateWebACLError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        UpdateWebACLError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidOperationException" => {
                        UpdateWebACLError::WAFInvalidOperation(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        UpdateWebACLError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        UpdateWebACLError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFNonexistentContainerException" => {
                        UpdateWebACLError::WAFNonexistentContainer(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        UpdateWebACLError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFReferencedItemException" => {
                        UpdateWebACLError::WAFReferencedItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        UpdateWebACLError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateWebACLError::Validation(error_message.to_string())
                    }
                    _ => UpdateWebACLError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateWebACLError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateWebACLError {
    fn from(err: serde_json::error::Error) -> UpdateWebACLError {
        UpdateWebACLError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateWebACLError {
    fn from(err: CredentialsError) -> UpdateWebACLError {
        UpdateWebACLError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateWebACLError {
    fn from(err: HttpDispatchError) -> UpdateWebACLError {
        UpdateWebACLError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateWebACLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateWebACLError {
    fn description(&self) -> &str {
        match *self {
            UpdateWebACLError::WAFInternalError(ref cause) => cause,
            UpdateWebACLError::WAFInvalidAccount(ref cause) => cause,
            UpdateWebACLError::WAFInvalidOperation(ref cause) => cause,
            UpdateWebACLError::WAFInvalidParameter(ref cause) => cause,
            UpdateWebACLError::WAFLimitsExceeded(ref cause) => cause,
            UpdateWebACLError::WAFNonexistentContainer(ref cause) => cause,
            UpdateWebACLError::WAFNonexistentItem(ref cause) => cause,
            UpdateWebACLError::WAFReferencedItem(ref cause) => cause,
            UpdateWebACLError::WAFStaleData(ref cause) => cause,
            UpdateWebACLError::Validation(ref cause) => cause,
            UpdateWebACLError::Credentials(ref err) => err.description(),
            UpdateWebACLError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateWebACLError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateXssMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateXssMatchSetError {
    ///<p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    ///<p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn't in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn't in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add an IP address to an <code>IPSet</code>, but the IP address already exists in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul>
    WAFInvalidOperation(String),
    ///<p>The operation failed because AWS WAF didn't recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, QUERY_STRING, or URI.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul>
    WAFInvalidParameter(String),
    ///<p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="http://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    ///<p>The operation failed because you tried to add an object to or delete an object from another object that doesn't exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn't exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn't exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn't exist.</p> </li> </ul>
    WAFNonexistentContainer(String),
    ///<p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    ///<p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateXssMatchSetError {
    pub fn from_body(body: &str) -> UpdateXssMatchSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "WAFInternalErrorException" => {
                        UpdateXssMatchSetError::WAFInternalError(String::from(error_message))
                    }
                    "WAFInvalidAccountException" => {
                        UpdateXssMatchSetError::WAFInvalidAccount(String::from(error_message))
                    }
                    "WAFInvalidOperationException" => {
                        UpdateXssMatchSetError::WAFInvalidOperation(String::from(error_message))
                    }
                    "WAFInvalidParameterException" => {
                        UpdateXssMatchSetError::WAFInvalidParameter(String::from(error_message))
                    }
                    "WAFLimitsExceededException" => {
                        UpdateXssMatchSetError::WAFLimitsExceeded(String::from(error_message))
                    }
                    "WAFNonexistentContainerException" => {
                        UpdateXssMatchSetError::WAFNonexistentContainer(String::from(error_message))
                    }
                    "WAFNonexistentItemException" => {
                        UpdateXssMatchSetError::WAFNonexistentItem(String::from(error_message))
                    }
                    "WAFStaleDataException" => {
                        UpdateXssMatchSetError::WAFStaleData(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateXssMatchSetError::Validation(error_message.to_string())
                    }
                    _ => UpdateXssMatchSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateXssMatchSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateXssMatchSetError {
    fn from(err: serde_json::error::Error) -> UpdateXssMatchSetError {
        UpdateXssMatchSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateXssMatchSetError {
    fn from(err: CredentialsError) -> UpdateXssMatchSetError {
        UpdateXssMatchSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateXssMatchSetError {
    fn from(err: HttpDispatchError) -> UpdateXssMatchSetError {
        UpdateXssMatchSetError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateXssMatchSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateXssMatchSetError {
    fn description(&self) -> &str {
        match *self {
            UpdateXssMatchSetError::WAFInternalError(ref cause) => cause,
            UpdateXssMatchSetError::WAFInvalidAccount(ref cause) => cause,
            UpdateXssMatchSetError::WAFInvalidOperation(ref cause) => cause,
            UpdateXssMatchSetError::WAFInvalidParameter(ref cause) => cause,
            UpdateXssMatchSetError::WAFLimitsExceeded(ref cause) => cause,
            UpdateXssMatchSetError::WAFNonexistentContainer(ref cause) => cause,
            UpdateXssMatchSetError::WAFNonexistentItem(ref cause) => cause,
            UpdateXssMatchSetError::WAFStaleData(ref cause) => cause,
            UpdateXssMatchSetError::Validation(ref cause) => cause,
            UpdateXssMatchSetError::Credentials(ref err) => err.description(),
            UpdateXssMatchSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateXssMatchSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the WAF Regional API. WAF Regional clients implement this trait.
pub trait WAFRegional {
    #[doc="<p>Associates a web ACL with a resource.</p>"]
    fn associate_web_acl(&self,
                         input: &AssociateWebACLRequest)
                         -> Result<AssociateWebACLResponse, AssociateWebACLError>;


    #[doc="<p>Creates a <code>ByteMatchSet</code>. You then use <a>UpdateByteMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>ByteMatchSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>BadBot</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateByteMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_byte_match_set(&self,
                             input: &CreateByteMatchSetRequest)
                             -> Result<CreateByteMatchSetResponse, CreateByteMatchSetError>;


    #[doc="<p>Creates an <a>IPSet</a>, which you use to specify which web requests you want to allow or block based on the IP addresses that the requests originate from. For example, if you're receiving a lot of requests from one or more individual IP addresses or one or more ranges of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that contains those IP addresses and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateIPSet</code> request.</p> </li> <li> <p>Submit a <code>CreateIPSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_ip_set(&self,
                     input: &CreateIPSetRequest)
                     -> Result<CreateIPSetResponse, CreateIPSetError>;


    #[doc="<p>Creates a <a>RateBasedRule</a>. The <code>RateBasedRule</code> contains a <code>RateLimit</code>, which specifies the maximum number of requests that AWS WAF allows from a specified IP address in a five-minute period. The <code>RateBasedRule</code> also contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to count or block if these requests exceed the <code>RateLimit</code>.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request not only must exceed the <code>RateLimit</code>, but it also must match all the specifications to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that meet the conditions in the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions must be received at a rate of more than 15,000 requests every five minutes. If both conditions are met and the rate is exceeded, AWS WAF blocks the requests. If the rate drops below 15,000 for a five-minute period, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p> <p>To create and configure a <code>RateBasedRule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the rule. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRateBasedRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRateBasedRule</code> request to specify the predicates that you want to include in the rule.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RateBasedRule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_rate_based_rule(&self,
                              input: &CreateRateBasedRuleRequest)
                              -> Result<CreateRateBasedRuleResponse, CreateRateBasedRuleError>;


    #[doc="<p>Creates a <code>Rule</code>, which contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to block. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed or blocked. For example, suppose you add the following to a <code>Rule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to blocks requests that satisfy the <code>Rule</code>. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to specify the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_rule(&self,
                   input: &CreateRuleRequest)
                   -> Result<CreateRuleResponse, CreateRuleError>;


    #[doc="<p>Creates a <code>SizeConstraintSet</code>. You then use <a>UpdateSizeConstraintSet</a> to identify the part of a web request that you want AWS WAF to check for length, such as the length of the <code>User-Agent</code> header or the length of the query string. For example, you can create a <code>SizeConstraintSet</code> that matches any requests that have a query string that is longer than 100 bytes. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateSizeConstraintSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_size_constraint_set
        (&self,
         input: &CreateSizeConstraintSetRequest)
         -> Result<CreateSizeConstraintSetResponse, CreateSizeConstraintSetError>;


    #[doc="<p>Creates a <a>SqlInjectionMatchSet</a>, which you use to allow, block, or count requests that contain snippets of SQL code in a specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateSqlInjectionMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count malicious SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_sql_injection_match_set
        (&self,
         input: &CreateSqlInjectionMatchSetRequest)
         -> Result<CreateSqlInjectionMatchSetResponse, CreateSqlInjectionMatchSetError>;


    #[doc="<p>Creates a <code>WebACL</code>, which contains the <code>Rules</code> that identify the CloudFront web requests that you want to allow, block, or count. AWS WAF evaluates <code>Rules</code> in order based on the value of <code>Priority</code> for each <code>Rule</code>.</p> <p>You also specify a default action, either <code>ALLOW</code> or <code>BLOCK</code>. If a web request doesn't match any of the <code>Rules</code> in a <code>WebACL</code>, AWS WAF responds to the request with the default action. </p> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>ByteMatchSet</code> objects and other predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateWebACL</code> request.</p> </li> <li> <p>Submit a <code>CreateWebACL</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <a>UpdateWebACL</a> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution.</p> </li> </ol> <p>For more information about how to use the AWS WAF API, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_web_acl(&self,
                      input: &CreateWebACLRequest)
                      -> Result<CreateWebACLResponse, CreateWebACLError>;


    #[doc="<p>Creates an <a>XssMatchSet</a>, which you use to allow, block, or count requests that contain cross-site scripting attacks in the specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateXssMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateXssMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_xss_match_set(&self,
                            input: &CreateXssMatchSetRequest)
                            -> Result<CreateXssMatchSetResponse, CreateXssMatchSetError>;


    #[doc="<p>Permanently deletes a <a>ByteMatchSet</a>. You can't delete a <code>ByteMatchSet</code> if it's still used in any <code>Rules</code> or if it still includes any <a>ByteMatchTuple</a> objects (any filters).</p> <p>If you just want to remove a <code>ByteMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>ByteMatchSet</code> to remove filters, if any. For more information, see <a>UpdateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteByteMatchSet</code> request.</p> </li> </ol>"]
    fn delete_byte_match_set(&self,
                             input: &DeleteByteMatchSetRequest)
                             -> Result<DeleteByteMatchSetResponse, DeleteByteMatchSetError>;


    #[doc="<p>Permanently deletes an <a>IPSet</a>. You can't delete an <code>IPSet</code> if it's still used in any <code>Rules</code> or if it still includes any IP addresses.</p> <p>If you just want to remove an <code>IPSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>IPSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>IPSet</code> to remove IP address ranges, if any. For more information, see <a>UpdateIPSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteIPSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteIPSet</code> request.</p> </li> </ol>"]
    fn delete_ip_set(&self,
                     input: &DeleteIPSetRequest)
                     -> Result<DeleteIPSetResponse, DeleteIPSetError>;


    #[doc="<p>Permanently deletes a <a>RateBasedRule</a>. You can't delete a rule if it's still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a rule from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RateBasedRule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RateBasedRule</code> to remove predicates, if any. For more information, see <a>UpdateRateBasedRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRateBasedRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRateBasedRule</code> request.</p> </li> </ol>"]
    fn delete_rate_based_rule(&self,
                              input: &DeleteRateBasedRuleRequest)
                              -> Result<DeleteRateBasedRuleResponse, DeleteRateBasedRuleError>;


    #[doc="<p>Permanently deletes a <a>Rule</a>. You can't delete a <code>Rule</code> if it's still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a <code>Rule</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>Rule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>Rule</code> to remove predicates, if any. For more information, see <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRule</code> request.</p> </li> </ol>"]
    fn delete_rule(&self,
                   input: &DeleteRuleRequest)
                   -> Result<DeleteRuleResponse, DeleteRuleError>;


    #[doc="<p>Permanently deletes a <a>SizeConstraintSet</a>. You can't delete a <code>SizeConstraintSet</code> if it's still used in any <code>Rules</code> or if it still includes any <a>SizeConstraint</a> objects (any filters).</p> <p>If you just want to remove a <code>SizeConstraintSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>SizeConstraintSet</code> to remove filters, if any. For more information, see <a>UpdateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSizeConstraintSet</code> request.</p> </li> </ol>"]
    fn delete_size_constraint_set
        (&self,
         input: &DeleteSizeConstraintSetRequest)
         -> Result<DeleteSizeConstraintSetResponse, DeleteSizeConstraintSetError>;


    #[doc="<p>Permanently deletes a <a>SqlInjectionMatchSet</a>. You can't delete a <code>SqlInjectionMatchSet</code> if it's still used in any <code>Rules</code> or if it still contains any <a>SqlInjectionMatchTuple</a> objects.</p> <p>If you just want to remove a <code>SqlInjectionMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SqlInjectionMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>SqlInjectionMatchSet</code> to remove filters, if any. For more information, see <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> </ol>"]
    fn delete_sql_injection_match_set
        (&self,
         input: &DeleteSqlInjectionMatchSetRequest)
         -> Result<DeleteSqlInjectionMatchSetResponse, DeleteSqlInjectionMatchSetError>;


    #[doc="<p>Permanently deletes a <a>WebACL</a>. You can't delete a <code>WebACL</code> if it still contains any <code>Rules</code>.</p> <p>To delete a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>WebACL</code> to remove <code>Rules</code>, if any. For more information, see <a>UpdateWebACL</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteWebACL</code> request.</p> </li> <li> <p>Submit a <code>DeleteWebACL</code> request.</p> </li> </ol>"]
    fn delete_web_acl(&self,
                      input: &DeleteWebACLRequest)
                      -> Result<DeleteWebACLResponse, DeleteWebACLError>;


    #[doc="<p>Permanently deletes an <a>XssMatchSet</a>. You can't delete an <code>XssMatchSet</code> if it's still used in any <code>Rules</code> or if it still contains any <a>XssMatchTuple</a> objects.</p> <p>If you just want to remove an <code>XssMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>XssMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>XssMatchSet</code> to remove filters, if any. For more information, see <a>UpdateXssMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteXssMatchSet</code> request.</p> </li> </ol>"]
    fn delete_xss_match_set(&self,
                            input: &DeleteXssMatchSetRequest)
                            -> Result<DeleteXssMatchSetResponse, DeleteXssMatchSetError>;


    #[doc="<p>Removes a web ACL from the specified resource.</p>"]
    fn disassociate_web_acl(&self,
                            input: &DisassociateWebACLRequest)
                            -> Result<DisassociateWebACLResponse, DisassociateWebACLError>;


    #[doc="<p>Returns the <a>ByteMatchSet</a> specified by <code>ByteMatchSetId</code>.</p>"]
    fn get_byte_match_set(&self,
                          input: &GetByteMatchSetRequest)
                          -> Result<GetByteMatchSetResponse, GetByteMatchSetError>;


    #[doc="<p>When you want to create, update, or delete AWS WAF objects, get a change token and include the change token in the create, update, or delete request. Change tokens ensure that your application doesn't submit conflicting requests to AWS WAF.</p> <p>Each create, update, or delete request must use a unique change token. If your application submits a <code>GetChangeToken</code> request and then submits a second <code>GetChangeToken</code> request before submitting a create, update, or delete request, the second <code>GetChangeToken</code> request returns the same value as the first <code>GetChangeToken</code> request.</p> <p>When you use a change token in a create, update, or delete request, the status of the change token changes to <code>PENDING</code>, which indicates that AWS WAF is propagating the change to all AWS WAF servers. Use <code>GetChangeTokenStatus</code> to determine the status of your change token.</p>"]
    fn get_change_token(&self) -> Result<GetChangeTokenResponse, GetChangeTokenError>;


    #[doc="<p>Returns the status of a <code>ChangeToken</code> that you got by calling <a>GetChangeToken</a>. <code>ChangeTokenStatus</code> is one of the following values:</p> <ul> <li> <p> <code>PROVISIONED</code>: You requested the change token by calling <code>GetChangeToken</code>, but you haven't used it yet in a call to create, update, or delete an AWS WAF object.</p> </li> <li> <p> <code>PENDING</code>: AWS WAF is propagating the create, update, or delete request to all AWS WAF servers.</p> </li> <li> <p> <code>IN_SYNC</code>: Propagation is complete.</p> </li> </ul>"]
    fn get_change_token_status
        (&self,
         input: &GetChangeTokenStatusRequest)
         -> Result<GetChangeTokenStatusResponse, GetChangeTokenStatusError>;


    #[doc="<p>Returns the <a>IPSet</a> that is specified by <code>IPSetId</code>.</p>"]
    fn get_ip_set(&self, input: &GetIPSetRequest) -> Result<GetIPSetResponse, GetIPSetError>;


    #[doc="<p>Returns the <a>RateBasedRule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRateBasedRule</code> request.</p>"]
    fn get_rate_based_rule(&self,
                           input: &GetRateBasedRuleRequest)
                           -> Result<GetRateBasedRuleResponse, GetRateBasedRuleError>;


    #[doc="<p>Returns an array of IP addresses currently being blocked by the <a>RateBasedRule</a> that is specified by the <code>RuleId</code>. The maximum number of managed keys that will be blocked is 10,000. If more than 10,000 addresses exceed the rate limit, the 10,000 addresses with the highest rates will be blocked.</p>"]
    fn get_rate_based_rule_managed_keys
        (&self,
         input: &GetRateBasedRuleManagedKeysRequest)
         -> Result<GetRateBasedRuleManagedKeysResponse, GetRateBasedRuleManagedKeysError>;


    #[doc="<p>Returns the <a>Rule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRule</code> request.</p>"]
    fn get_rule(&self, input: &GetRuleRequest) -> Result<GetRuleResponse, GetRuleError>;


    #[doc="<p>Gets detailed information about a specified number of requests--a sample--that AWS WAF randomly selects from among the first 5,000 requests that your AWS resource received during a time range that you choose. You can specify a sample size of up to 500 requests, and you can specify any time range in the previous three hours.</p> <p> <code>GetSampledRequests</code> returns a time range, which is usually the time range that you specified. However, if your resource (such as a CloudFront distribution) received 5,000 requests before the specified time range elapsed, <code>GetSampledRequests</code> returns an updated time range. This new time range indicates the actual period during which AWS WAF selected the requests in the sample.</p>"]
    fn get_sampled_requests(&self,
                            input: &GetSampledRequestsRequest)
                            -> Result<GetSampledRequestsResponse, GetSampledRequestsError>;


    #[doc="<p>Returns the <a>SizeConstraintSet</a> specified by <code>SizeConstraintSetId</code>.</p>"]
    fn get_size_constraint_set
        (&self,
         input: &GetSizeConstraintSetRequest)
         -> Result<GetSizeConstraintSetResponse, GetSizeConstraintSetError>;


    #[doc="<p>Returns the <a>SqlInjectionMatchSet</a> that is specified by <code>SqlInjectionMatchSetId</code>.</p>"]
    fn get_sql_injection_match_set
        (&self,
         input: &GetSqlInjectionMatchSetRequest)
         -> Result<GetSqlInjectionMatchSetResponse, GetSqlInjectionMatchSetError>;


    #[doc="<p>Returns the <a>WebACL</a> that is specified by <code>WebACLId</code>.</p>"]
    fn get_web_acl(&self, input: &GetWebACLRequest) -> Result<GetWebACLResponse, GetWebACLError>;


    #[doc="<p>Returns the web ACL for the specified resource.</p>"]
    fn get_web_acl_for_resource
        (&self,
         input: &GetWebACLForResourceRequest)
         -> Result<GetWebACLForResourceResponse, GetWebACLForResourceError>;


    #[doc="<p>Returns the <a>XssMatchSet</a> that is specified by <code>XssMatchSetId</code>.</p>"]
    fn get_xss_match_set(&self,
                         input: &GetXssMatchSetRequest)
                         -> Result<GetXssMatchSetResponse, GetXssMatchSetError>;


    #[doc="<p>Returns an array of <a>ByteMatchSetSummary</a> objects.</p>"]
    fn list_byte_match_sets(&self,
                            input: &ListByteMatchSetsRequest)
                            -> Result<ListByteMatchSetsResponse, ListByteMatchSetsError>;


    #[doc="<p>Returns an array of <a>IPSetSummary</a> objects in the response.</p>"]
    fn list_ip_sets(&self,
                    input: &ListIPSetsRequest)
                    -> Result<ListIPSetsResponse, ListIPSetsError>;


    #[doc="<p>Returns an array of <a>RuleSummary</a> objects.</p>"]
    fn list_rate_based_rules(&self,
                             input: &ListRateBasedRulesRequest)
                             -> Result<ListRateBasedRulesResponse, ListRateBasedRulesError>;


    #[doc="<p>Returns an array of resources associated with the specified web ACL.</p>"]
    fn list_resources_for_web_acl
        (&self,
         input: &ListResourcesForWebACLRequest)
         -> Result<ListResourcesForWebACLResponse, ListResourcesForWebACLError>;


    #[doc="<p>Returns an array of <a>RuleSummary</a> objects.</p>"]
    fn list_rules(&self, input: &ListRulesRequest) -> Result<ListRulesResponse, ListRulesError>;


    #[doc="<p>Returns an array of <a>SizeConstraintSetSummary</a> objects.</p>"]
    fn list_size_constraint_sets
        (&self,
         input: &ListSizeConstraintSetsRequest)
         -> Result<ListSizeConstraintSetsResponse, ListSizeConstraintSetsError>;


    #[doc="<p>Returns an array of <a>SqlInjectionMatchSet</a> objects.</p>"]
    fn list_sql_injection_match_sets
        (&self,
         input: &ListSqlInjectionMatchSetsRequest)
         -> Result<ListSqlInjectionMatchSetsResponse, ListSqlInjectionMatchSetsError>;


    #[doc="<p>Returns an array of <a>WebACLSummary</a> objects in the response.</p>"]
    fn list_web_ac_ls(&self,
                      input: &ListWebACLsRequest)
                      -> Result<ListWebACLsResponse, ListWebACLsError>;


    #[doc="<p>Returns an array of <a>XssMatchSet</a> objects.</p>"]
    fn list_xss_match_sets(&self,
                           input: &ListXssMatchSetsRequest)
                           -> Result<ListXssMatchSetsResponse, ListXssMatchSetsError>;


    #[doc="<p>Inserts or deletes <a>ByteMatchTuple</a> objects (filters) in a <a>ByteMatchSet</a>. For each <code>ByteMatchTuple</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>ByteMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to look for. For more information, including how you specify the values for the AWS WAF API and the AWS CLI or SDKs, see <code>TargetString</code> in the <a>ByteMatchTuple</a> data type. </p> </li> <li> <p>Where to look, such as at the beginning or the end of a query string.</p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p>For example, you can add a <code>ByteMatchSetUpdate</code> object that matches web requests in which <code>User-Agent</code> headers contain the string <code>BadBot</code>. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>ByteMatchSet.</code> For more information, see <a>CreateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateByteMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_byte_match_set(&self,
                             input: &UpdateByteMatchSetRequest)
                             -> Result<UpdateByteMatchSetResponse, UpdateByteMatchSetError>;


    #[doc="<p>Inserts or deletes <a>IPSetDescriptor</a> objects in an <code>IPSet</code>. For each <code>IPSetDescriptor</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>IPSetDescriptor</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The IP address version, <code>IPv4</code> or <code>IPv6</code>. </p> </li> <li> <p>The IP address in CIDR notation, for example, <code>192.0.2.0/24</code> (for the range of IP addresses from <code>192.0.2.0</code> to <code>192.0.2.255</code>) or <code>192.0.2.44/32</code> (for the individual IP address <code>192.0.2.44</code>). </p> </li> </ul> <p>AWS WAF supports /8, /16, /24, and /32 IP address ranges for IPv4, and /24, /32, /48, /56, /64 and /128 for IPv6. For more information about CIDR notation, see the Wikipedia entry <a href=\"https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing\">Classless Inter-Domain Routing</a>.</p> <p>IPv6 addresses can be represented using any of the following formats:</p> <ul> <li> <p>1111:0000:0000:0000:0000:0000:0000:0111/128</p> </li> <li> <p>1111:0:0:0:0:0:0:0111/128</p> </li> <li> <p>1111::0111/128</p> </li> <li> <p>1111::111/128</p> </li> </ul> <p>You use an <code>IPSet</code> to specify which web requests you want to allow or block based on the IP addresses that the requests originated from. For example, if you're receiving a lot of requests from one or a small number of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that specifies those IP addresses, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateIPSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>IPSet</code>, you specify the IP addresses that you want to add and/or the IP addresses that you want to delete. If you want to change an IP address, you delete the existing IP address and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_ip_set(&self,
                     input: &UpdateIPSetRequest)
                     -> Result<UpdateIPSetResponse, UpdateIPSetError>;


    #[doc="<p>Inserts or deletes <a>Predicate</a> objects in a rule and updates the <code>RateLimit</code> in the rule. </p> <p>Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to block or count. The <code>RateLimit</code> specifies the number of requests every five minutes that triggers the rule.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request must match all the predicates and exceed the <code>RateLimit</code> to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions much be received at a rate of more than 15,000 every five minutes. If the rate drops below this limit, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p>"]
    fn update_rate_based_rule(&self,
                              input: &UpdateRateBasedRuleRequest)
                              -> Result<UpdateRateBasedRuleResponse, UpdateRateBasedRuleError>;


    #[doc="<p>Inserts or deletes <a>Predicate</a> objects in a <code>Rule</code>. Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to allow, block, or count. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed, blocked, or counted. For example, suppose you add the following to a <code>Rule</code>: </p> <ul> <li> <p>A <code>ByteMatchSet</code> that matches the value <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44</code> </p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the <code>Rule</code>. For a request to be blocked, the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code> <i>and</i> the request must originate from the IP address 192.0.2.44.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create the <code>Rule</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to add predicates to the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>ByteMatchSet</code> or <code>IPSet</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_rule(&self,
                   input: &UpdateRuleRequest)
                   -> Result<UpdateRuleResponse, UpdateRuleError>;


    #[doc="<p>Inserts or deletes <a>SizeConstraint</a> objects (filters) in a <a>SizeConstraintSet</a>. For each <code>SizeConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>SizeConstraintSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to evaluate, such as the length of a query string or the length of the <code>User-Agent</code> header.</p> </li> <li> <p>Whether to perform any transformations on the request, such as converting it to lowercase, before checking its length. Note that transformations of the request body are not supported because the AWS resource forwards only the first <code>8192</code> bytes of your request to AWS WAF.</p> </li> <li> <p>A <code>ComparisonOperator</code> used for evaluating the selected part of the request against the specified <code>Size</code>, such as equals, greater than, less than, and so on.</p> </li> <li> <p>The length, in bytes, that you want AWS WAF to watch for in selected part of the request. The length is computed after applying the transformation.</p> </li> </ul> <p>For example, you can add a <code>SizeConstraintSetUpdate</code> object that matches web requests in which the length of the <code>User-Agent</code> header is greater than 100 bytes. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>SizeConstraintSet.</code> For more information, see <a>CreateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateSizeConstraintSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_size_constraint_set
        (&self,
         input: &UpdateSizeConstraintSetRequest)
         -> Result<UpdateSizeConstraintSetResponse, UpdateSizeConstraintSetError>;


    #[doc="<p>Inserts or deletes <a>SqlInjectionMatchTuple</a> objects (filters) in a <a>SqlInjectionMatchSet</a>. For each <code>SqlInjectionMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change a <code>SqlInjectionMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header, the name of the header.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for snippets of malicious SQL code.</p> </li> </ul> <p>You use <code>SqlInjectionMatchSet</code> objects to specify which CloudFront requests you want to allow, block, or count. For example, if you're receiving requests that contain snippets of SQL code in the query string and you want to block the requests, you can create a <code>SqlInjectionMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateSqlInjectionMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for snippets of SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_sql_injection_match_set
        (&self,
         input: &UpdateSqlInjectionMatchSetRequest)
         -> Result<UpdateSqlInjectionMatchSetResponse, UpdateSqlInjectionMatchSetError>;


    #[doc="<p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>WebACL</code>. Each <code>Rule</code> identifies web requests that you want to allow, block, or count. When you update a <code>WebACL</code>, you specify the following values:</p> <ul> <li> <p>A default action for the <code>WebACL</code>, either <code>ALLOW</code> or <code>BLOCK</code>. AWS WAF performs the default action if a request doesn't match the criteria in any of the <code>Rules</code> in a <code>WebACL</code>.</p> </li> <li> <p>The <code>Rules</code> that you want to add and/or delete. If you want to replace one <code>Rule</code> with another, you delete the existing <code>Rule</code> and add the new one.</p> </li> <li> <p>For each <code>Rule</code>, whether you want AWS WAF to allow requests, block requests, or count requests that match the conditions in the <code>Rule</code>.</p> </li> <li> <p>The order in which you want AWS WAF to evaluate the <code>Rules</code> in a <code>WebACL</code>. If you add more than one <code>Rule</code> to a <code>WebACL</code>, AWS WAF evaluates each request against the <code>Rules</code> in order based on the value of <code>Priority</code>. (The <code>Rule</code> that has the lowest value for <code>Priority</code> is evaluated first.) When a web request matches all of the predicates (such as <code>ByteMatchSets</code> and <code>IPSets</code>) in a <code>Rule</code>, AWS WAF immediately takes the corresponding action, allow or block, and doesn't evaluate the request against the remaining <code>Rules</code> in the <code>WebACL</code>, if any. </p> </li> </ul> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Create a <code>WebACL</code>. See <a>CreateWebACL</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <code>UpdateWebACL</code> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution. </p> </li> </ol> <p>Be aware that if you try to add a RATE_BASED rule to a web ACL without setting the rule type when first creating the rule, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule (the default rule type) with the specified ID, which does not exist. </p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_web_acl(&self,
                      input: &UpdateWebACLRequest)
                      -> Result<UpdateWebACLResponse, UpdateWebACLError>;


    #[doc="<p>Inserts or deletes <a>XssMatchTuple</a> objects (filters) in an <a>XssMatchSet</a>. For each <code>XssMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change a <code>XssMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header, the name of the header.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for cross-site scripting attacks.</p> </li> </ul> <p>You use <code>XssMatchSet</code> objects to specify which CloudFront requests you want to allow, block, or count. For example, if you're receiving requests that contain cross-site scripting attacks in the request body and you want to block the requests, you can create an <code>XssMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateXssMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateXssMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_xss_match_set(&self,
                            input: &UpdateXssMatchSetRequest)
                            -> Result<UpdateXssMatchSetResponse, UpdateXssMatchSetError>;
}
/// A client for the WAF Regional API.
pub struct WAFRegionalClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> WAFRegionalClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        WAFRegionalClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> WAFRegional for WAFRegionalClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Associates a web ACL with a resource.</p>"]
    fn associate_web_acl(&self,
                         input: &AssociateWebACLRequest)
                         -> Result<AssociateWebACLResponse, AssociateWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.AssociateWebACL");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<AssociateWebACLResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AssociateWebACLError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a <code>ByteMatchSet</code>. You then use <a>UpdateByteMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>ByteMatchSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>BadBot</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateByteMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_byte_match_set(&self,
                             input: &CreateByteMatchSetRequest)
                             -> Result<CreateByteMatchSetResponse, CreateByteMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.CreateByteMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateByteMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateByteMatchSetError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Creates an <a>IPSet</a>, which you use to specify which web requests you want to allow or block based on the IP addresses that the requests originate from. For example, if you're receiving a lot of requests from one or more individual IP addresses or one or more ranges of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that contains those IP addresses and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateIPSet</code> request.</p> </li> <li> <p>Submit a <code>CreateIPSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_ip_set(&self,
                     input: &CreateIPSetRequest)
                     -> Result<CreateIPSetResponse, CreateIPSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateIPSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateIPSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateIPSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a <a>RateBasedRule</a>. The <code>RateBasedRule</code> contains a <code>RateLimit</code>, which specifies the maximum number of requests that AWS WAF allows from a specified IP address in a five-minute period. The <code>RateBasedRule</code> also contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to count or block if these requests exceed the <code>RateLimit</code>.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request not only must exceed the <code>RateLimit</code>, but it also must match all the specifications to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that meet the conditions in the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions must be received at a rate of more than 15,000 requests every five minutes. If both conditions are met and the rate is exceeded, AWS WAF blocks the requests. If the rate drops below 15,000 for a five-minute period, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p> <p>To create and configure a <code>RateBasedRule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the rule. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRateBasedRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRateBasedRule</code> request to specify the predicates that you want to include in the rule.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RateBasedRule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_rate_based_rule(&self,
                              input: &CreateRateBasedRuleRequest)
                              -> Result<CreateRateBasedRuleResponse, CreateRateBasedRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.CreateRateBasedRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateRateBasedRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateRateBasedRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a <code>Rule</code>, which contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to block. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed or blocked. For example, suppose you add the following to a <code>Rule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to blocks requests that satisfy the <code>Rule</code>. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to specify the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_rule(&self,
                   input: &CreateRuleRequest)
                   -> Result<CreateRuleResponse, CreateRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a <code>SizeConstraintSet</code>. You then use <a>UpdateSizeConstraintSet</a> to identify the part of a web request that you want AWS WAF to check for length, such as the length of the <code>User-Agent</code> header or the length of the query string. For example, you can create a <code>SizeConstraintSet</code> that matches any requests that have a query string that is longer than 100 bytes. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateSizeConstraintSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_size_constraint_set
        (&self,
         input: &CreateSizeConstraintSetRequest)
         -> Result<CreateSizeConstraintSetResponse, CreateSizeConstraintSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.CreateSizeConstraintSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateSizeConstraintSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateSizeConstraintSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a <a>SqlInjectionMatchSet</a>, which you use to allow, block, or count requests that contain snippets of SQL code in a specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateSqlInjectionMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count malicious SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_sql_injection_match_set
        (&self,
         input: &CreateSqlInjectionMatchSetRequest)
         -> Result<CreateSqlInjectionMatchSetResponse, CreateSqlInjectionMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.CreateSqlInjectionMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateSqlInjectionMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateSqlInjectionMatchSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a <code>WebACL</code>, which contains the <code>Rules</code> that identify the CloudFront web requests that you want to allow, block, or count. AWS WAF evaluates <code>Rules</code> in order based on the value of <code>Priority</code> for each <code>Rule</code>.</p> <p>You also specify a default action, either <code>ALLOW</code> or <code>BLOCK</code>. If a web request doesn't match any of the <code>Rules</code> in a <code>WebACL</code>, AWS WAF responds to the request with the default action. </p> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>ByteMatchSet</code> objects and other predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateWebACL</code> request.</p> </li> <li> <p>Submit a <code>CreateWebACL</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <a>UpdateWebACL</a> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution.</p> </li> </ol> <p>For more information about how to use the AWS WAF API, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_web_acl(&self,
                      input: &CreateWebACLRequest)
                      -> Result<CreateWebACLResponse, CreateWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateWebACL");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateWebACLResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateWebACLError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates an <a>XssMatchSet</a>, which you use to allow, block, or count requests that contain cross-site scripting attacks in the specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateXssMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateXssMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn create_xss_match_set(&self,
                            input: &CreateXssMatchSetRequest)
                            -> Result<CreateXssMatchSetResponse, CreateXssMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.CreateXssMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateXssMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateXssMatchSetError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Permanently deletes a <a>ByteMatchSet</a>. You can't delete a <code>ByteMatchSet</code> if it's still used in any <code>Rules</code> or if it still includes any <a>ByteMatchTuple</a> objects (any filters).</p> <p>If you just want to remove a <code>ByteMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>ByteMatchSet</code> to remove filters, if any. For more information, see <a>UpdateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteByteMatchSet</code> request.</p> </li> </ol>"]
    fn delete_byte_match_set(&self,
                             input: &DeleteByteMatchSetRequest)
                             -> Result<DeleteByteMatchSetResponse, DeleteByteMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.DeleteByteMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteByteMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteByteMatchSetError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Permanently deletes an <a>IPSet</a>. You can't delete an <code>IPSet</code> if it's still used in any <code>Rules</code> or if it still includes any IP addresses.</p> <p>If you just want to remove an <code>IPSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>IPSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>IPSet</code> to remove IP address ranges, if any. For more information, see <a>UpdateIPSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteIPSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteIPSet</code> request.</p> </li> </ol>"]
    fn delete_ip_set(&self,
                     input: &DeleteIPSetRequest)
                     -> Result<DeleteIPSetResponse, DeleteIPSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteIPSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteIPSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteIPSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Permanently deletes a <a>RateBasedRule</a>. You can't delete a rule if it's still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a rule from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RateBasedRule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RateBasedRule</code> to remove predicates, if any. For more information, see <a>UpdateRateBasedRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRateBasedRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRateBasedRule</code> request.</p> </li> </ol>"]
    fn delete_rate_based_rule(&self,
                              input: &DeleteRateBasedRuleRequest)
                              -> Result<DeleteRateBasedRuleResponse, DeleteRateBasedRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.DeleteRateBasedRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteRateBasedRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteRateBasedRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Permanently deletes a <a>Rule</a>. You can't delete a <code>Rule</code> if it's still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a <code>Rule</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>Rule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>Rule</code> to remove predicates, if any. For more information, see <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRule</code> request.</p> </li> </ol>"]
    fn delete_rule(&self,
                   input: &DeleteRuleRequest)
                   -> Result<DeleteRuleResponse, DeleteRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Permanently deletes a <a>SizeConstraintSet</a>. You can't delete a <code>SizeConstraintSet</code> if it's still used in any <code>Rules</code> or if it still includes any <a>SizeConstraint</a> objects (any filters).</p> <p>If you just want to remove a <code>SizeConstraintSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>SizeConstraintSet</code> to remove filters, if any. For more information, see <a>UpdateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSizeConstraintSet</code> request.</p> </li> </ol>"]
    fn delete_size_constraint_set
        (&self,
         input: &DeleteSizeConstraintSetRequest)
         -> Result<DeleteSizeConstraintSetResponse, DeleteSizeConstraintSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.DeleteSizeConstraintSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteSizeConstraintSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteSizeConstraintSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Permanently deletes a <a>SqlInjectionMatchSet</a>. You can't delete a <code>SqlInjectionMatchSet</code> if it's still used in any <code>Rules</code> or if it still contains any <a>SqlInjectionMatchTuple</a> objects.</p> <p>If you just want to remove a <code>SqlInjectionMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SqlInjectionMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>SqlInjectionMatchSet</code> to remove filters, if any. For more information, see <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> </ol>"]
    fn delete_sql_injection_match_set
        (&self,
         input: &DeleteSqlInjectionMatchSetRequest)
         -> Result<DeleteSqlInjectionMatchSetResponse, DeleteSqlInjectionMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.DeleteSqlInjectionMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteSqlInjectionMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteSqlInjectionMatchSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Permanently deletes a <a>WebACL</a>. You can't delete a <code>WebACL</code> if it still contains any <code>Rules</code>.</p> <p>To delete a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>WebACL</code> to remove <code>Rules</code>, if any. For more information, see <a>UpdateWebACL</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteWebACL</code> request.</p> </li> <li> <p>Submit a <code>DeleteWebACL</code> request.</p> </li> </ol>"]
    fn delete_web_acl(&self,
                      input: &DeleteWebACLRequest)
                      -> Result<DeleteWebACLResponse, DeleteWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteWebACL");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteWebACLResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteWebACLError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Permanently deletes an <a>XssMatchSet</a>. You can't delete an <code>XssMatchSet</code> if it's still used in any <code>Rules</code> or if it still contains any <a>XssMatchTuple</a> objects.</p> <p>If you just want to remove an <code>XssMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>XssMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>XssMatchSet</code> to remove filters, if any. For more information, see <a>UpdateXssMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteXssMatchSet</code> request.</p> </li> </ol>"]
    fn delete_xss_match_set(&self,
                            input: &DeleteXssMatchSetRequest)
                            -> Result<DeleteXssMatchSetResponse, DeleteXssMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.DeleteXssMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteXssMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteXssMatchSetError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Removes a web ACL from the specified resource.</p>"]
    fn disassociate_web_acl(&self,
                            input: &DisassociateWebACLRequest)
                            -> Result<DisassociateWebACLResponse, DisassociateWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.DisassociateWebACL");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DisassociateWebACLResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DisassociateWebACLError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the <a>ByteMatchSet</a> specified by <code>ByteMatchSetId</code>.</p>"]
    fn get_byte_match_set(&self,
                          input: &GetByteMatchSetRequest)
                          -> Result<GetByteMatchSetResponse, GetByteMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetByteMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetByteMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetByteMatchSetError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>When you want to create, update, or delete AWS WAF objects, get a change token and include the change token in the create, update, or delete request. Change tokens ensure that your application doesn't submit conflicting requests to AWS WAF.</p> <p>Each create, update, or delete request must use a unique change token. If your application submits a <code>GetChangeToken</code> request and then submits a second <code>GetChangeToken</code> request before submitting a create, update, or delete request, the second <code>GetChangeToken</code> request returns the same value as the first <code>GetChangeToken</code> request.</p> <p>When you use a change token in a create, update, or delete request, the status of the change token changes to <code>PENDING</code>, which indicates that AWS WAF is propagating the change to all AWS WAF servers. Use <code>GetChangeTokenStatus</code> to determine the status of your change token.</p>"]
    fn get_change_token(&self) -> Result<GetChangeTokenResponse, GetChangeTokenError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetChangeToken");
        request.set_payload(Some(b"{}".to_vec()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetChangeTokenResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetChangeTokenError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the status of a <code>ChangeToken</code> that you got by calling <a>GetChangeToken</a>. <code>ChangeTokenStatus</code> is one of the following values:</p> <ul> <li> <p> <code>PROVISIONED</code>: You requested the change token by calling <code>GetChangeToken</code>, but you haven't used it yet in a call to create, update, or delete an AWS WAF object.</p> </li> <li> <p> <code>PENDING</code>: AWS WAF is propagating the create, update, or delete request to all AWS WAF servers.</p> </li> <li> <p> <code>IN_SYNC</code>: Propagation is complete.</p> </li> </ul>"]
    fn get_change_token_status
        (&self,
         input: &GetChangeTokenStatusRequest)
         -> Result<GetChangeTokenStatusResponse, GetChangeTokenStatusError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.GetChangeTokenStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetChangeTokenStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetChangeTokenStatusError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the <a>IPSet</a> that is specified by <code>IPSetId</code>.</p>"]
    fn get_ip_set(&self, input: &GetIPSetRequest) -> Result<GetIPSetResponse, GetIPSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetIPSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetIPSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetIPSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the <a>RateBasedRule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRateBasedRule</code> request.</p>"]
    fn get_rate_based_rule(&self,
                           input: &GetRateBasedRuleRequest)
                           -> Result<GetRateBasedRuleResponse, GetRateBasedRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetRateBasedRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetRateBasedRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetRateBasedRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Returns an array of IP addresses currently being blocked by the <a>RateBasedRule</a> that is specified by the <code>RuleId</code>. The maximum number of managed keys that will be blocked is 10,000. If more than 10,000 addresses exceed the rate limit, the 10,000 addresses with the highest rates will be blocked.</p>"]
    fn get_rate_based_rule_managed_keys
        (&self,
         input: &GetRateBasedRuleManagedKeysRequest)
         -> Result<GetRateBasedRuleManagedKeysResponse, GetRateBasedRuleManagedKeysError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.GetRateBasedRuleManagedKeys");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetRateBasedRuleManagedKeysResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetRateBasedRuleManagedKeysError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the <a>Rule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRule</code> request.</p>"]
    fn get_rule(&self, input: &GetRuleRequest) -> Result<GetRuleResponse, GetRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                Ok(serde_json::from_str::<GetRuleResponse>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(GetRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets detailed information about a specified number of requests--a sample--that AWS WAF randomly selects from among the first 5,000 requests that your AWS resource received during a time range that you choose. You can specify a sample size of up to 500 requests, and you can specify any time range in the previous three hours.</p> <p> <code>GetSampledRequests</code> returns a time range, which is usually the time range that you specified. However, if your resource (such as a CloudFront distribution) received 5,000 requests before the specified time range elapsed, <code>GetSampledRequests</code> returns an updated time range. This new time range indicates the actual period during which AWS WAF selected the requests in the sample.</p>"]
    fn get_sampled_requests(&self,
                            input: &GetSampledRequestsRequest)
                            -> Result<GetSampledRequestsResponse, GetSampledRequestsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.GetSampledRequests");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetSampledRequestsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetSampledRequestsError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the <a>SizeConstraintSet</a> specified by <code>SizeConstraintSetId</code>.</p>"]
    fn get_size_constraint_set
        (&self,
         input: &GetSizeConstraintSetRequest)
         -> Result<GetSizeConstraintSetResponse, GetSizeConstraintSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.GetSizeConstraintSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetSizeConstraintSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetSizeConstraintSetError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the <a>SqlInjectionMatchSet</a> that is specified by <code>SqlInjectionMatchSetId</code>.</p>"]
    fn get_sql_injection_match_set
        (&self,
         input: &GetSqlInjectionMatchSetRequest)
         -> Result<GetSqlInjectionMatchSetResponse, GetSqlInjectionMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.GetSqlInjectionMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetSqlInjectionMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetSqlInjectionMatchSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the <a>WebACL</a> that is specified by <code>WebACLId</code>.</p>"]
    fn get_web_acl(&self, input: &GetWebACLRequest) -> Result<GetWebACLResponse, GetWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetWebACL");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetWebACLResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(GetWebACLError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the web ACL for the specified resource.</p>"]
    fn get_web_acl_for_resource
        (&self,
         input: &GetWebACLForResourceRequest)
         -> Result<GetWebACLForResourceResponse, GetWebACLForResourceError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.GetWebACLForResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetWebACLForResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetWebACLForResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Returns the <a>XssMatchSet</a> that is specified by <code>XssMatchSetId</code>.</p>"]
    fn get_xss_match_set(&self,
                         input: &GetXssMatchSetRequest)
                         -> Result<GetXssMatchSetResponse, GetXssMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.GetXssMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetXssMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(GetXssMatchSetError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Returns an array of <a>ByteMatchSetSummary</a> objects.</p>"]
    fn list_byte_match_sets(&self,
                            input: &ListByteMatchSetsRequest)
                            -> Result<ListByteMatchSetsResponse, ListByteMatchSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListByteMatchSets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListByteMatchSetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListByteMatchSetsError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Returns an array of <a>IPSetSummary</a> objects in the response.</p>"]
    fn list_ip_sets(&self,
                    input: &ListIPSetsRequest)
                    -> Result<ListIPSetsResponse, ListIPSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListIPSets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListIPSetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListIPSetsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns an array of <a>RuleSummary</a> objects.</p>"]
    fn list_rate_based_rules(&self,
                             input: &ListRateBasedRulesRequest)
                             -> Result<ListRateBasedRulesResponse, ListRateBasedRulesError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.ListRateBasedRules");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListRateBasedRulesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListRateBasedRulesError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Returns an array of resources associated with the specified web ACL.</p>"]
    fn list_resources_for_web_acl
        (&self,
         input: &ListResourcesForWebACLRequest)
         -> Result<ListResourcesForWebACLResponse, ListResourcesForWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.ListResourcesForWebACL");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListResourcesForWebACLResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListResourcesForWebACLError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Returns an array of <a>RuleSummary</a> objects.</p>"]
    fn list_rules(&self, input: &ListRulesRequest) -> Result<ListRulesResponse, ListRulesError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListRules");
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


    #[doc="<p>Returns an array of <a>SizeConstraintSetSummary</a> objects.</p>"]
    fn list_size_constraint_sets
        (&self,
         input: &ListSizeConstraintSetsRequest)
         -> Result<ListSizeConstraintSetsResponse, ListSizeConstraintSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.ListSizeConstraintSets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListSizeConstraintSetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListSizeConstraintSetsError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Returns an array of <a>SqlInjectionMatchSet</a> objects.</p>"]
    fn list_sql_injection_match_sets
        (&self,
         input: &ListSqlInjectionMatchSetsRequest)
         -> Result<ListSqlInjectionMatchSetsResponse, ListSqlInjectionMatchSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.ListSqlInjectionMatchSets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListSqlInjectionMatchSetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListSqlInjectionMatchSetsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns an array of <a>WebACLSummary</a> objects in the response.</p>"]
    fn list_web_ac_ls(&self,
                      input: &ListWebACLsRequest)
                      -> Result<ListWebACLsResponse, ListWebACLsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListWebACLs");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListWebACLsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListWebACLsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns an array of <a>XssMatchSet</a> objects.</p>"]
    fn list_xss_match_sets(&self,
                           input: &ListXssMatchSetsRequest)
                           -> Result<ListXssMatchSetsResponse, ListXssMatchSetsError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.ListXssMatchSets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListXssMatchSetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListXssMatchSetsError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Inserts or deletes <a>ByteMatchTuple</a> objects (filters) in a <a>ByteMatchSet</a>. For each <code>ByteMatchTuple</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>ByteMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to look for. For more information, including how you specify the values for the AWS WAF API and the AWS CLI or SDKs, see <code>TargetString</code> in the <a>ByteMatchTuple</a> data type. </p> </li> <li> <p>Where to look, such as at the beginning or the end of a query string.</p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p>For example, you can add a <code>ByteMatchSetUpdate</code> object that matches web requests in which <code>User-Agent</code> headers contain the string <code>BadBot</code>. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>ByteMatchSet.</code> For more information, see <a>CreateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateByteMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_byte_match_set(&self,
                             input: &UpdateByteMatchSetRequest)
                             -> Result<UpdateByteMatchSetResponse, UpdateByteMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.UpdateByteMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateByteMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateByteMatchSetError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Inserts or deletes <a>IPSetDescriptor</a> objects in an <code>IPSet</code>. For each <code>IPSetDescriptor</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>IPSetDescriptor</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The IP address version, <code>IPv4</code> or <code>IPv6</code>. </p> </li> <li> <p>The IP address in CIDR notation, for example, <code>192.0.2.0/24</code> (for the range of IP addresses from <code>192.0.2.0</code> to <code>192.0.2.255</code>) or <code>192.0.2.44/32</code> (for the individual IP address <code>192.0.2.44</code>). </p> </li> </ul> <p>AWS WAF supports /8, /16, /24, and /32 IP address ranges for IPv4, and /24, /32, /48, /56, /64 and /128 for IPv6. For more information about CIDR notation, see the Wikipedia entry <a href=\"https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing\">Classless Inter-Domain Routing</a>.</p> <p>IPv6 addresses can be represented using any of the following formats:</p> <ul> <li> <p>1111:0000:0000:0000:0000:0000:0000:0111/128</p> </li> <li> <p>1111:0:0:0:0:0:0:0111/128</p> </li> <li> <p>1111::0111/128</p> </li> <li> <p>1111::111/128</p> </li> </ul> <p>You use an <code>IPSet</code> to specify which web requests you want to allow or block based on the IP addresses that the requests originated from. For example, if you're receiving a lot of requests from one or a small number of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that specifies those IP addresses, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateIPSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>IPSet</code>, you specify the IP addresses that you want to add and/or the IP addresses that you want to delete. If you want to change an IP address, you delete the existing IP address and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_ip_set(&self,
                     input: &UpdateIPSetRequest)
                     -> Result<UpdateIPSetResponse, UpdateIPSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateIPSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateIPSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateIPSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Inserts or deletes <a>Predicate</a> objects in a rule and updates the <code>RateLimit</code> in the rule. </p> <p>Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to block or count. The <code>RateLimit</code> specifies the number of requests every five minutes that triggers the rule.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request must match all the predicates and exceed the <code>RateLimit</code> to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions much be received at a rate of more than 15,000 every five minutes. If the rate drops below this limit, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 15,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p>"]
    fn update_rate_based_rule(&self,
                              input: &UpdateRateBasedRuleRequest)
                              -> Result<UpdateRateBasedRuleResponse, UpdateRateBasedRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.UpdateRateBasedRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateRateBasedRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateRateBasedRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Inserts or deletes <a>Predicate</a> objects in a <code>Rule</code>. Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to allow, block, or count. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed, blocked, or counted. For example, suppose you add the following to a <code>Rule</code>: </p> <ul> <li> <p>A <code>ByteMatchSet</code> that matches the value <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44</code> </p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the <code>Rule</code>. For a request to be blocked, the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code> <i>and</i> the request must originate from the IP address 192.0.2.44.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create the <code>Rule</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to add predicates to the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>ByteMatchSet</code> or <code>IPSet</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_rule(&self,
                   input: &UpdateRuleRequest)
                   -> Result<UpdateRuleResponse, UpdateRuleError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Inserts or deletes <a>SizeConstraint</a> objects (filters) in a <a>SizeConstraintSet</a>. For each <code>SizeConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>SizeConstraintSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to evaluate, such as the length of a query string or the length of the <code>User-Agent</code> header.</p> </li> <li> <p>Whether to perform any transformations on the request, such as converting it to lowercase, before checking its length. Note that transformations of the request body are not supported because the AWS resource forwards only the first <code>8192</code> bytes of your request to AWS WAF.</p> </li> <li> <p>A <code>ComparisonOperator</code> used for evaluating the selected part of the request against the specified <code>Size</code>, such as equals, greater than, less than, and so on.</p> </li> <li> <p>The length, in bytes, that you want AWS WAF to watch for in selected part of the request. The length is computed after applying the transformation.</p> </li> </ul> <p>For example, you can add a <code>SizeConstraintSetUpdate</code> object that matches web requests in which the length of the <code>User-Agent</code> header is greater than 100 bytes. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>SizeConstraintSet.</code> For more information, see <a>CreateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateSizeConstraintSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_size_constraint_set
        (&self,
         input: &UpdateSizeConstraintSetRequest)
         -> Result<UpdateSizeConstraintSetResponse, UpdateSizeConstraintSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.UpdateSizeConstraintSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateSizeConstraintSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateSizeConstraintSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Inserts or deletes <a>SqlInjectionMatchTuple</a> objects (filters) in a <a>SqlInjectionMatchSet</a>. For each <code>SqlInjectionMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change a <code>SqlInjectionMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header, the name of the header.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for snippets of malicious SQL code.</p> </li> </ul> <p>You use <code>SqlInjectionMatchSet</code> objects to specify which CloudFront requests you want to allow, block, or count. For example, if you're receiving requests that contain snippets of SQL code in the query string and you want to block the requests, you can create a <code>SqlInjectionMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateSqlInjectionMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for snippets of SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_sql_injection_match_set
        (&self,
         input: &UpdateSqlInjectionMatchSetRequest)
         -> Result<UpdateSqlInjectionMatchSetResponse, UpdateSqlInjectionMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AWSWAF_Regional_20161128.UpdateSqlInjectionMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateSqlInjectionMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(UpdateSqlInjectionMatchSetError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>WebACL</code>. Each <code>Rule</code> identifies web requests that you want to allow, block, or count. When you update a <code>WebACL</code>, you specify the following values:</p> <ul> <li> <p>A default action for the <code>WebACL</code>, either <code>ALLOW</code> or <code>BLOCK</code>. AWS WAF performs the default action if a request doesn't match the criteria in any of the <code>Rules</code> in a <code>WebACL</code>.</p> </li> <li> <p>The <code>Rules</code> that you want to add and/or delete. If you want to replace one <code>Rule</code> with another, you delete the existing <code>Rule</code> and add the new one.</p> </li> <li> <p>For each <code>Rule</code>, whether you want AWS WAF to allow requests, block requests, or count requests that match the conditions in the <code>Rule</code>.</p> </li> <li> <p>The order in which you want AWS WAF to evaluate the <code>Rules</code> in a <code>WebACL</code>. If you add more than one <code>Rule</code> to a <code>WebACL</code>, AWS WAF evaluates each request against the <code>Rules</code> in order based on the value of <code>Priority</code>. (The <code>Rule</code> that has the lowest value for <code>Priority</code> is evaluated first.) When a web request matches all of the predicates (such as <code>ByteMatchSets</code> and <code>IPSets</code>) in a <code>Rule</code>, AWS WAF immediately takes the corresponding action, allow or block, and doesn't evaluate the request against the remaining <code>Rules</code> in the <code>WebACL</code>, if any. </p> </li> </ul> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Create a <code>WebACL</code>. See <a>CreateWebACL</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <code>UpdateWebACL</code> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution. </p> </li> </ol> <p>Be aware that if you try to add a RATE_BASED rule to a web ACL without setting the rule type when first creating the rule, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule (the default rule type) with the specified ID, which does not exist. </p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_web_acl(&self,
                      input: &UpdateWebACLRequest)
                      -> Result<UpdateWebACLResponse, UpdateWebACLError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateWebACL");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateWebACLResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateWebACLError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Inserts or deletes <a>XssMatchTuple</a> objects (filters) in an <a>XssMatchSet</a>. For each <code>XssMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change a <code>XssMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header, the name of the header.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for cross-site scripting attacks.</p> </li> </ul> <p>You use <code>XssMatchSet</code> objects to specify which CloudFront requests you want to allow, block, or count. For example, if you're receiving requests that contain cross-site scripting attacks in the request body and you want to block the requests, you can create an <code>XssMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateXssMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateXssMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href=\"http://docs.aws.amazon.com/waf/latest/developerguide/\">AWS WAF Developer Guide</a>.</p>"]
    fn update_xss_match_set(&self,
                            input: &UpdateXssMatchSetRequest)
                            -> Result<UpdateXssMatchSetResponse, UpdateXssMatchSetError> {
        let mut request = SignedRequest::new("POST", "waf-regional", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSWAF_Regional_20161128.UpdateXssMatchSet");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateXssMatchSetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateXssMatchSetError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
